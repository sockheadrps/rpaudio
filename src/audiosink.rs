use crate::exceptions::EffectConflictException;
use crate::timesync::ExtractableEffect;
use crate::timesync::{ActionType, EffectResult, EffectSync};
use crate::{exmetadata, MetaData};
use ::std::sync::mpsc::{Receiver, Sender};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};
use rodio::{Decoder, OutputStream, Sink};
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

unsafe impl Send for AudioSink {}

#[pyclass]
#[derive(Serialize)]
struct AudioInfo {
    #[pyo3(get)]
    position: f32,
    #[pyo3(get)]
    speed: f32,
    #[pyo3(get)]
    effects: Vec<ActionType>,
    #[pyo3(get)]
    volume: f32,
}

impl AudioSink {
    pub fn handle_action_and_effects(&mut self, sink: Arc<Mutex<Sink>>) {
        if let Some(receiver) = &self.action_receiver {
            if let Ok(action) = receiver.try_recv() {
                let mut effects_guard = self.effects.lock().unwrap();
                let effect_sync = Arc::new(EffectSync::new(
                    action.clone(),
                    sink.lock().unwrap().get_pos().as_secs_f32(),
                    Some(self.metadata.duration.unwrap() as f32),
                ));
                match action {
                    ActionType::FadeIn(fade_in) => {
                        if self.initial_play {
                            if let Some(start_val) = fade_in.start_val {
                                sink.lock().unwrap().set_volume(start_val);
                            }
                            self.initial_play = false;
                        }

                        effects_guard.push(effect_sync);
                    }
                    ActionType::FadeOut(_fade_out) => {
                        effects_guard.push(effect_sync);
                    }
                    ActionType::ChangeSpeed(_) => {
                        effects_guard.push(effect_sync);
                    }
                }
            }

            let mut effects_guard = self.effects.lock().unwrap();
            let current_position = self.get_pos().unwrap() as f32;
            effects_guard.retain(|effect| {
                let keep_effect = match effect.action {
                    ActionType::FadeIn(_fade_in) => match effect.update(current_position) {
                        EffectResult::Value(val) => {
                            let mut lock = self.vol_manipulation_lock.write().unwrap();
                            sink.lock().unwrap().set_volume(val);
                            *lock = true;
                            true
                        }
                        EffectResult::Ignored => true,
                        EffectResult::Completed(val) => {
                            let mut lock = self.vol_manipulation_lock.write().unwrap();
                            sink.lock().unwrap().set_volume(val);
                            *lock = false;
                            false
                        }
                    },
                    ActionType::FadeOut(_fade_out) => match effect.update(current_position) {
                        EffectResult::Value(val) => {
                            let mut lock = self.vol_manipulation_lock.write().unwrap();
                            sink.lock().unwrap().set_volume(val);
                            *lock = true;
                            true
                        }
                        EffectResult::Ignored => true,
                        EffectResult::Completed(val) => {
                            let mut lock = self.vol_manipulation_lock.write().unwrap();
                            sink.lock().unwrap().set_volume(val);
                            *lock = false;
                            false
                        }
                    },
                    ActionType::ChangeSpeed(_change_speed) => {
                        match effect.update(current_position) {
                            EffectResult::Value(val) => {
                                let mut lock = self.speed_manipulation_lock.write().unwrap();
                                sink.lock().unwrap().set_speed(val);
                                *lock = true;
                                true
                            }
                            EffectResult::Ignored => true,
                            EffectResult::Completed(val) => {
                                sink.lock().unwrap().set_speed(val);
                                let mut lock = self.speed_manipulation_lock.write().unwrap();
                                *lock = false;
                                false
                            }
                        }
                    }
                };

                if sink.lock().unwrap().is_paused() && self.resume {
                    sink.lock().unwrap().play();
                    self.resume = false;
                }
                keep_effect
            });
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct AudioSink {
    pub is_playing: Arc<RwLock<bool>>,
    callback: Arc<Option<Py<PyAny>>>,
    cancel_callback: Arc<RwLock<bool>>,
    pub sink: Option<Arc<Mutex<Sink>>>,
    stream: Option<Arc<OutputStream>>,
    pub metadata: MetaData,
    volume: f32,
    start_time: Option<Instant>,
    position: Duration,
    pub action_sender: Option<Sender<ActionType>>,
    pub action_receiver: Option<Arc<Receiver<ActionType>>>,
    initial_play: bool,
    effects: Arc<Mutex<Vec<Arc<EffectSync>>>>,
    effects_chain: Vec<ActionType>,
    resume: bool,
    vol_manipulation_lock: Arc<RwLock<bool>>,
    speed_manipulation_lock: Arc<RwLock<bool>>,
}

impl AudioSink {
    fn invoke_callback(callback: &Option<Py<PyAny>>) {
        Python::with_gil(|py| {
            if let Some(callback) = callback {
                if let Err(e) = callback.call0(py) {
                    eprintln!("Failed to invoke callback: {}", e);
                }
            }
        });
    }
}

#[pymethods]
impl AudioSink {
    #[new]
    #[pyo3(signature = (callback=None))]
    pub fn new(callback: Option<Py<PyAny>>) -> Self {
        let (action_sender, action_receiver) = mpsc::channel();
        AudioSink {
            is_playing: Arc::new(RwLock::new(false)),
            callback: Arc::new(callback),
            cancel_callback: Arc::new(RwLock::new(false)),
            sink: None,
            stream: None,
            metadata: MetaData::default(),
            volume: 1.0,
            start_time: None,
            position: Duration::from_secs(0),
            action_sender: Some(action_sender),
            action_receiver: Some(Arc::new(action_receiver)),
            initial_play: true,
            effects: Arc::new(Mutex::new(Vec::new())),
            effects_chain: Vec::new(),
            resume: false,
            vol_manipulation_lock: Arc::new(RwLock::new(false)),
            speed_manipulation_lock: Arc::new(RwLock::new(false)),
        }
    }

    #[getter]
    pub fn metadata(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(self.metadata.clone().into_py(py))
    }

    #[getter]
    pub fn is_playing(&self) -> bool {
        *self.is_playing.read().unwrap()
    }

    #[getter]
    pub fn callback(&self) -> Option<Py<PyAny>> {
        (*self.callback).clone()
    }

    pub fn load_audio(&mut self, file_path: String) -> PyResult<Self> {
        if self.sink.is_some() {
            return Err(PyRuntimeError::new_err(
                "Audio is already loaded. Please stop the current audio before loading a new one.",
            ));
        }

        let (new_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).map_err(|e| {
            PyRuntimeError::new_err(format!("Failed to create sink: {}", e))
        })?));

        let file_path_clone = file_path.clone();
        let file = File::open(file_path_clone).unwrap();
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| format!("Failed to decode audio file: {}", e))
            .unwrap();

        self.metadata = exmetadata::extract_metadata(std::path::Path::new(&file_path))
            .map_err(|_| PyRuntimeError::new_err("Failed to extract metadata"))?;

        {
            sink.lock().unwrap().append(source);
            sink.lock().unwrap().pause();
        }

        self.stream = Some(Arc::new(new_stream));
        sink.lock().unwrap().set_volume(0.0);
        self.sink = Some(sink.clone());

        if let Some(sink) = &self.sink {
            sink.lock().unwrap().pause();
        }
        let self_clone = self.clone();

        let is_playing_clone = Arc::clone(&self.is_playing);
        let callback = self.callback.clone();
        let cancel_callback_clone = self.cancel_callback.clone();

        thread::spawn({
            let sink = Arc::clone(&sink);

            move || {
                let mut self_clone = self_clone;

                loop {
                    let sink_guard = sink.lock().unwrap();

                    if sink_guard.empty() {
                        let mut is_playing_guard = is_playing_clone.write().unwrap();
                        *is_playing_guard = false;

                        if !*cancel_callback_clone.read().unwrap() {
                            Self::invoke_callback(&*callback);
                        }
                        drop(self_clone);
                        break;
                    }

                    drop(sink_guard);
                    self_clone.handle_action_and_effects(Arc::clone(&sink));

                    thread::sleep(Duration::from_millis(100));
                }
            }
        });

        Ok(self.clone())
    }

    pub fn play(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            *self.is_playing.write().unwrap() = true;
            if self.initial_play {
                sink.lock().unwrap().play();

                self.handle_action_and_effects(sink.clone());
            } else {
                self.resume = true;
                self.handle_action_and_effects(sink.clone());
            }
            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to play. Load audio first.",
            ))
        }
    }

    pub fn pause(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            *self.is_playing.write().unwrap() = false;
            sink.lock().unwrap().pause();
            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available. Load audio first.",
            ))
        }
    }

    pub fn stop(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().stop();
            *self.is_playing.write().unwrap() = false;

            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to stop. Load audio first.",
            ))
        }
    }

    pub fn get_volume(&self) -> PyResult<f32> {
        if let Some(sink) = &self.sink {
            Ok(sink.lock().unwrap().volume())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available. Load audio first.",
            ))
        }
    }

    pub fn set_volume(&mut self, volume: f32) -> PyResult<()> {
        if volume < 0.0 || volume > 1.0 {
            return Err(PyValueError::new_err("Volume must be between 0.0 and 1.0."));
        }

        if let Some(sink) = &self.sink {
            let lock = self.vol_manipulation_lock.read().unwrap();

            if *lock {
                return Err(EffectConflictException::with_context("Volume"));
            } else {
                sink.lock().unwrap().set_volume(volume);
                self.volume = volume;
                Ok(())
            }
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to set volume. Load audio first.",
            ))
        }
    }

    pub fn get_pos(&self) -> PyResult<f64> {
        if let Some(sink) = &self.sink {
            let duration = sink.lock().unwrap().get_pos();
            let position_seconds = duration.as_secs_f64();
            Ok((position_seconds * 100.0).round() / 100.0)
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available. Load audio first.",
            ))
        }
    }

    pub fn set_duration(&mut self, duration: f32) -> PyResult<()> {
        let duration = Duration::from_secs_f32(duration);
        self.metadata.duration = Some(duration.as_secs_f64());
        Ok(())
    }

    pub fn get_remaining_time(&self) -> PyResult<f64> {
        if let Some(sink) = &self.sink {
            let sink_lock = sink.lock().unwrap();
            let position = sink_lock.get_pos();

            if let Some(duration) = self.metadata.duration {
                let remaining = duration - position.as_secs_f64();
                Ok((remaining * 100.0).round() / 100.0)
            } else {
                Err(PyRuntimeError::new_err("Audio duration is not available."))
            }
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available. Load audio first.",
            ))
        }
    }

    pub fn get_speed(&self) -> f32 {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().speed()
        } else {
            1.0
        }
    }

    pub fn set_speed(&mut self, speed: f32) -> PyResult<()> {
        if speed <= 0.0 {
            return Err(PyValueError::new_err("Speed must be greater than 0."));
        }

        if let Some(sink) = &self.sink {
            let lock = self.speed_manipulation_lock.read().unwrap();

            if *lock {
                return Err(EffectConflictException::with_context("Speed"));
            } else {
                sink.lock().unwrap().set_speed(speed);
                Ok(())
            }
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available. Load audio first.",
            ))
        }
    }

    pub fn try_seek(&mut self, position: f32) -> PyResult<()> {
        if position <= 0.0 {
            return Err(PyValueError::new_err("Position must be non-negative."));
        }

        if let Some(sink) = &self.sink {
            let duration = Duration::from_secs_f32(position);

            let result = sink.lock().unwrap().try_seek(duration);
            match result {
                Ok(_) => {
                    self.position = Duration::from_secs_f64(self.get_pos().unwrap());
                    self.start_time = Some(Instant::now());
                    Ok(())
                }
                Err(e) => Err(PyRuntimeError::new_err(format!("Seek failed: {:?}", e))),
            }
        } else {
            Err(PyRuntimeError::new_err(
                "No audio sink available. Load audio first.",
            ))
        }
    }

    pub fn cancel_callback(&mut self) {
        let mut cancel_guard = self.cancel_callback.write().unwrap();
        *cancel_guard = true;
    }

    pub fn empty(&self) -> bool {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().empty()
        } else {
            false
        }
    }

    pub fn apply_effects(&mut self, effect_list: Py<PyList>) -> PyResult<()> {
        Python::with_gil(|py| -> PyResult<()> {
            let effects_guard = &mut self.effects_chain;

            let _effect_list: Vec<Py<PyAny>> = effect_list.extract(py)?;

            let rust_effect_list: Result<Vec<ActionType>, PyErr> = _effect_list
                .into_iter()
                .map(|effect| {
                    let effect = effect.downcast_bound::<PyAny>(py).unwrap();
                    effect.extract_action()
                })
                .collect();

            *effects_guard = rust_effect_list?;

            Ok(())
        })?;

        if let Some(sender) = self.action_sender.take() {
            let effects_guard = &self.effects_chain;

            for effect in effects_guard.iter() {
                sender
                    .send(effect.clone())
                    .map_err(|_| {
                        eprintln!("Failed to send effect");
                    })
                    .ok();
            }
        }

        Ok(())
    }

    pub fn playback_data(&self) -> PyResult<PyObject> {
        let self_clone = self.clone();
        Python::with_gil(|py| {
            let metadata = self_clone.metadata.clone();

            let dict = metadata.into_py_dict_bound(py);

            let effects_list = PyList::new_bound(py, &Vec::<PyObject>::new());

            for effect in self.effects.lock().unwrap().iter() {
                let effect_dict = match &effect.action {
                    ActionType::FadeIn(fi) => fi.into_py_dict_bound(py),
                    ActionType::FadeOut(fo) => fo.into_py_dict_bound(py),
                    ActionType::ChangeSpeed(cs) => cs.into_py_dict_bound(py),
                };
                effects_list.append(effect_dict)?;
            }
            dict.set_item("effects", effects_list)?;
            return Ok(dict.into());
        })
    }
}
