use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use rodio::{Decoder, OutputStream, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use timesync::{ActionType, ChangeSpeed, EffectResult, EffectSync, FadeIn, FadeOut};
mod audioqueue;
mod exmetadata;
mod mixer;
mod timesync;
pub use exmetadata::MetaData;
unsafe impl Send for AudioSink {}
use pyo3::types::PyModule;
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Clone)]
#[pyclass]
pub struct AudioSink {
    is_playing: Arc<Mutex<bool>>,
    callback: Arc<Mutex<Option<Py<PyAny>>>>,
    cancel_callback: Arc<Mutex<bool>>,
    sink: Option<Arc<Mutex<Sink>>>,
    stream: Option<Arc<Mutex<OutputStream>>>,
    pub metadata: MetaData,
    volume: Arc<Mutex<f32>>,
    start_time: Arc<Mutex<Option<Instant>>>,
    position: Arc<Mutex<Duration>>,
    pub action_sender: Option<Sender<ActionType>>,
    pub action_receiver: Option<Arc<Mutex<Receiver<ActionType>>>>,
    initial_play: bool,
    effects: Arc<Mutex<Vec<Arc<EffectSync>>>>,
    effects_chain: Arc<Mutex<Vec<ActionType>>>,
}

impl AudioSink {
    pub fn handle_action_and_effects(&mut self, sink: Arc<Mutex<Sink>>) {
        if let Some(receiver) = &self.action_receiver {
            if let Ok(action) = receiver.lock().unwrap().try_recv() {
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

            effects_guard.retain(|effect| {
                let current_position = sink.lock().unwrap().get_pos().as_secs_f32();
                let keep_effect = match effect.action {
                    ActionType::FadeIn(_fade_in) => match effect.update(current_position) {
                        EffectResult::Value(val) => {
                            sink.lock().unwrap().set_volume(val);
                            true
                        }
                        EffectResult::Ignored => true,
                        EffectResult::Completed(val) => {
                            sink.lock().unwrap().set_volume(val); 
                            false 
                        }
                    },
                    ActionType::FadeOut(_fade_out) => match effect.update(current_position) {
                        EffectResult::Value(val) => {
                            sink.lock().unwrap().set_volume(val);
                            true
                        }
                        EffectResult::Ignored => true,
                        EffectResult::Completed(val) => {
                            sink.lock().unwrap().set_volume(val);
                            false
                        }
                    },
                    ActionType::ChangeSpeed(_change_speed) => {
                        match effect.update(current_position) {
                            EffectResult::Value(val) => {
                                sink.lock().unwrap().set_speed(val);
                                true
                            }
                            EffectResult::Ignored => true,
                            EffectResult::Completed(val) => {
                                sink.lock().unwrap().set_speed(val);
                                false
                            }
                        }
                    }
                };

                keep_effect
            });
        }
    }
}

#[pymethods]
impl AudioSink {
    #[new]
    #[pyo3(signature = (callback=None))]
    pub fn new(callback: Option<Py<PyAny>>) -> Self {
        let (action_sender, action_receiver) = mpsc::channel();
        AudioSink {
            is_playing: Arc::new(Mutex::new(false)),
            callback: Arc::new(Mutex::new(callback)),
            cancel_callback: Arc::new(Mutex::new(false)),
            sink: None,
            stream: None,
            metadata: MetaData::default(),
            volume: Arc::new(Mutex::new(1.0)),
            start_time: Arc::new(Mutex::new(None)),
            position: Mutex::new(Duration::from_secs(0)).into(),
            action_sender: Some(action_sender),
            action_receiver: Some(Arc::new(Mutex::new(action_receiver))),
            initial_play: true,
            effects: Arc::new(Mutex::new(Vec::new())),
            effects_chain: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[getter]
    pub fn metadata(&self, py: Python) -> PyResult<Py<PyDict>> {
        let mut dict = HashMap::new();
        dict.insert("title", self.metadata.title.clone());
        dict.insert("artist", self.metadata.artist.clone());
        dict.insert("date", self.metadata.date.clone());
        dict.insert("year", self.metadata.year.clone());
        dict.insert("album_title", self.metadata.album_title.clone());
        dict.insert("album_artist", self.metadata.album_artist.clone());
        dict.insert("track_number", self.metadata.track_number.clone());
        dict.insert("total_tracks", self.metadata.total_tracks.clone());
        dict.insert("disc_number", self.metadata.disc_number.clone());
        dict.insert("total_discs", self.metadata.total_discs.clone());
        dict.insert("genre", self.metadata.genre.clone());
        dict.insert("composer", self.metadata.composer.clone());
        dict.insert("comment", self.metadata.comment.clone());
        dict.insert(
            "sample_rate",
            self.metadata.sample_rate.map(|rate| rate.to_string()),
        );
        dict.insert("channels", self.metadata.channels.clone());
        dict.insert(
            "duration",
            self.metadata
                .duration
                .map(|duration| format!("{:.1}", duration)),
        );

        let py_dict = PyDict::new_bound(py);

        for (key, value) in dict {
            py_dict.set_item(key, value)?;
        }

        Ok(py_dict.into())
    }

    #[getter]
    pub fn is_playing(&self) -> bool {
        *self.is_playing.lock().unwrap()
    }

    #[getter]
    pub fn callback(&self) -> Option<Py<PyAny>> {
        self.callback.lock().unwrap().clone()
    }

    pub fn load_audio(&mut self, file_path: String) -> PyResult<Self> {
        if self.sink.is_some() {
            return Err(PyRuntimeError::new_err(
                "Audio is already loaded. Please stop the current audio before loading a new one.",
            ));
        }

        let (new_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink_result = Sink::try_new(&stream_handle);
        let sink = match sink_result {
            Ok(s) => Arc::new(Mutex::new(s)),
            Err(e) => {
                return Err(PyRuntimeError::new_err(format!(
                    "Failed to create sink: {}",
                    e
                )))
            }
        };

        let file_path_clone = file_path.clone();
        let file = File::open(file_path_clone).unwrap();
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| format!("Failed to decode audio file: {}", e))
            .unwrap();

        let metadata = match exmetadata::extract_metadata(file_path.as_ref()) {
            Ok(meta) => meta,
            Err(_) => return Err(PyRuntimeError::new_err("Failed to extract metadata")),
        };
        self.metadata = metadata;

        sink.lock().unwrap().append(source);

        self.stream = Some(Arc::new(Mutex::new(new_stream)));
        self.sink = Some(sink.clone());

        if let Some(sink) = &self.sink {
            (*sink.lock().unwrap()).pause();
        }
        let self_clone = self.clone();

        let is_playing = self.is_playing.clone();
        let callback = self.callback.clone();
        let cancel_callback = self.cancel_callback.clone();

        thread::spawn(move || {
            let mut self_clone = self_clone;

            loop {
                {
                    if self_clone.empty() {
                        let mut is_playing_guard = is_playing.lock().unwrap();
                        *is_playing_guard = false;
                        drop(self_clone);

                        let cancel_callback_guard = cancel_callback.lock().unwrap();

                        if !*cancel_callback_guard {
                            let callback_guard = callback.lock().unwrap();
                            Self::invoke_callback(&*callback_guard);
                        }
                        break;
                    }
                }

                self_clone.handle_action_and_effects(sink.clone());
                thread::sleep(Duration::from_millis(20));
            }
        });

        Ok(self.clone())
    }

    pub fn play(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            *self.is_playing.lock().unwrap() = true;
            if self.initial_play {
                sink.lock().unwrap().play();

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
            match sink.try_lock() {
                Ok(sink) => {
                    *self.is_playing.lock().unwrap() = false;
                    sink.pause();
                    Ok(())
                }
                Err(_) => Err(PyRuntimeError::new_err("Failed to acquire lock")),
            }
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available. Load audio first.",
            ))
        }
    }

    pub fn stop(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().stop();
            *self.is_playing.lock().unwrap() = false;

            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to stop. Load audio first.",
            ))
        }
    }

    pub fn empty(&self) -> bool {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().empty()
        } else {
            false
        }
    }

    pub fn cancel_callback(&mut self) {
        let mut cancel_guard = self.cancel_callback.lock().unwrap();
        *cancel_guard = true;
    }

    pub fn set_volume(&mut self, volume: f32) -> PyResult<()> {
        if volume < 0.0 || volume > 1.0 {
            return Err(PyValueError::new_err("Volume must be between 0.0 and 1.0."));
        }

        if let Some(sink) = &self.sink {
            sink.lock().unwrap().set_volume(volume);
            *self.volume.lock().unwrap() = volume;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to set volume. Load audio first.",
            ))
        }
    }

    pub fn get_volume(&self) -> PyResult<f32> {
        if let Some(sink) = &self.sink {
            match sink.try_lock() {
                Ok(sink) => Ok(sink.volume()),
                Err(_) => Err(PyRuntimeError::new_err("Failed to acquire lock")),
            }
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

    pub fn try_seek(&mut self, position: f32) -> PyResult<()> {
        if position < 0.0 {
            return Err(PyValueError::new_err("Position must be non-negative."));
        }

        if let Some(sink) = &self.sink {
            let duration = Duration::from_secs_f32(position);

            let result = sink.lock().unwrap().try_seek(duration);
            match result {
                Ok(_) => {
                    *self.position.lock().unwrap() =
                        Duration::from_secs_f64(self.get_pos().unwrap());
                    *self.start_time.lock().unwrap() = Some(Instant::now());
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

    pub fn get_speed(&self) -> f32 {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().speed()
        } else {
            1.0
        }
    }

    pub fn get_remaining_time(&self) -> PyResult<f64> {
        if let Some(sink) = &self.sink {
            let sink_lock = sink.lock().unwrap();
            let position = sink_lock.get_pos();

            if let Some(duration) = self.metadata.duration {
                let remaining = duration - position.as_secs_f64();
                print!("remaining: {:?}", remaining);
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

    pub fn apply_effects(&mut self, effect_list: Py<PyList>) -> PyResult<()> {
        let _ = Python::with_gil(|py| {
            let mut effects_guard = match self.effects_chain.lock() {
                Ok(guard) => guard,
                Err(_) => {
                    return Err(PyRuntimeError::new_err(
                        "Failed to acquire lock on effects_chain",
                    ))
                }
            };

            let _effect_list: Vec<Py<PyAny>> = effect_list.extract(py)?;

            let rust_effect_list: Vec<ActionType> = _effect_list
                .into_iter()
                .map(|effect| {
                    let effect = effect.downcast_bound(py).unwrap();
                    if let Ok(fade_in) = effect.extract::<FadeIn>() {
                        Ok(ActionType::FadeIn(fade_in))
                    } else if let Ok(fade_out) = effect.extract::<FadeOut>() {
                        Ok(ActionType::FadeOut(fade_out))
                    } else if let Ok(change_speed) = effect.extract::<ChangeSpeed>() {
                        Ok(ActionType::ChangeSpeed(change_speed))
                    } else {
                        Err(PyTypeError::new_err("Unknown effect type"))
                    }
                })
                .collect::<Result<Vec<ActionType>, PyErr>>()?;

            *effects_guard = rust_effect_list;

            Ok(())
        });
        let effects_guard = match self.effects_chain.lock() {
            Ok(guard) => guard,
            Err(_) => {
                eprintln!("Failed to acquire lock on effects_chain");
                return Err(PyRuntimeError::new_err(
                    "Failed to acquire lock on effects_chain",
                ));
            }
        };
        if let Some(sender) = self.action_sender.take() {
            for effect in effects_guard.iter() {
                sender.send(effect.clone()).unwrap();
            }
        } else {
            eprintln!("Action sender is None");
        }
        Ok(())
    }
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

#[pymodule]
fn rpaudio(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AudioSink>()?;
    m.add_class::<mixer::ChannelManager>()?;
    m.add_class::<audioqueue::AudioChannel>()?;
    m.add_class::<ActionType>()?;
    m.add_class::<FadeIn>()?;
    m.add_class::<FadeOut>()?;
    m.add_class::<ChangeSpeed>()?;

    Ok(())
}
