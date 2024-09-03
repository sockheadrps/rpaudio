use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use timesync::{ActionType, EffectActionData, TimeSyncEffect, TimeSyncEffects};
mod audioqueue;
mod exmetadata;
mod mixer;
pub use exmetadata::MetaData;
unsafe impl Send for AudioSink {}
use pyo3::types::PyModule;
mod timesync;

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
    speed: Arc<Mutex<f32>>,
    position: Arc<Mutex<Duration>>,
    time_remaining: Arc<Mutex<Option<f32>>>,
    time_sync_effects: Arc<Mutex<TimeSyncEffects>>, 
}

#[pymethods]
impl AudioSink {
    #[new]
    #[pyo3(signature = (callback=None))]
    pub fn new(callback: Option<Py<PyAny>>) -> Self {
        AudioSink {
            is_playing: Arc::new(Mutex::new(false)),
            callback: Arc::new(Mutex::new(callback)),
            cancel_callback: Arc::new(Mutex::new(false)),
            sink: None,
            stream: None,
            metadata: MetaData::default(),
            volume: Arc::new(Mutex::new(1.0)),
            start_time: Arc::new(Mutex::new(None)),
            speed: Arc::new(Mutex::new(1.0)),
            position: Mutex::new(Duration::from_secs(0)).into(),
            time_remaining: Mutex::new(None).into(),
            time_sync_effects: Arc::new(Mutex::new(TimeSyncEffects::new())), // Initialize TimeSyncEffects
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
            self.metadata.duration.map(|duration| duration.to_string()),
        );

        let py_dict = PyDict::new_bound(py);

        // Insert items into the Python dictionary
        for (key, value) in dict {
            py_dict.set_item(key, value)?;
        }

        Ok(py_dict.into())
    }

    #[getter]
    pub fn is_playing(&self) -> bool {
        *self.is_playing.lock().unwrap()
    }

    pub fn load_audio(&mut self, file_path: String) -> PyResult<Self> {
        if self.sink.is_some() {
            return Ok(self.clone());
        }
        // let mut time_sync_effects = timesync::TimeSyncEffects::new();

        let metadata = match exmetadata::extract_metadata(file_path.as_ref()) {
            Ok(meta) => meta,
            Err(_e) => {
                return Err(PyRuntimeError::new_err("Failed to extract metadata"));
            }
        };
        self.metadata = metadata;

        // let mut effects = TimeSyncEffects::new();

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
        let total_duration = match source.total_duration() {
            Some(duration) => duration.as_secs_f32(),
            None => 0.0,
        };
        sink.lock().unwrap().append(source);

        self.stream = Some(Arc::new(Mutex::new(new_stream)));
        self.sink = Some(sink.clone());

        if let Some(sink) = &self.sink {
            (*sink.lock().unwrap()).pause();
        }

        let effects = Arc::clone(&self.time_sync_effects);
        {
            let mut effects_lock = effects.lock().unwrap();
            effects_lock.add_effect(TimeSyncEffect::new(
                Duration::from_secs(10),
                |audio_sink| {
                    audio_sink.set_fade(5.0, 1.0, 0.1).unwrap();
                },
            ));
        }
        let is_playing = self.is_playing.clone();
        let callback = self.callback.clone();
        let cancel_callback = self.cancel_callback.clone();
        let sink_clone = sink.clone();
        let speed = self.speed.clone();
        let mut time_remaining = *self.time_remaining.lock().unwrap();
        let time_sync_effects_clone = Arc::clone(&self.time_sync_effects);
        let self_lock = Arc::new(Mutex::new(self.clone()));

        thread::spawn(move || {
                let mut last_check_time = Instant::now();

            loop {
                {
                    let mut is_playing_guard = is_playing.lock().unwrap();
                    let sink = sink_clone.lock().unwrap();

                    if sink.empty() {
                        *is_playing_guard = false;
                        drop(is_playing_guard);
                        if !*cancel_callback.lock().unwrap() {
                            Self::invoke_callback(&*callback.lock().unwrap());
                        }
                        break;
                    }

                    if sink.is_paused() {
                        *is_playing_guard = false;
                    } else {
                        *is_playing_guard = true;
                        time_remaining = Some(total_duration - sink.get_pos().as_secs_f32());
                    }
                    let speed = speed.lock().unwrap();
                    sink.set_speed(*speed);
                    let current_time = Instant::now() - last_check_time;
                    time_sync_effects_clone.lock().unwrap().apply_due_effects(&self_lock.lock().unwrap(), sink.get_pos());
                    last_check_time = Instant::now();

                }

                thread::sleep(Duration::from_millis(100));
            }

            let mut is_playing_guard = is_playing.lock().unwrap();
            *is_playing_guard = false;
        });

        Ok(self.clone())
    }

    pub fn play(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().play();
            *self.is_playing.lock().unwrap() = true;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to play. Load audio first.",
            ))
        }
    }

    pub fn pause(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().pause();
            *self.is_playing.lock().unwrap() = false;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to pause. Load audio first.",
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
        *self.cancel_callback.lock().unwrap() = true;
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
            let sink = Arc::clone(sink);
            let volume = sink.lock().unwrap().volume();
            Ok(volume)
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available. Load audio first.",
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

    pub fn set_speed(&mut self, speed: f32) -> PyResult<()> {
        if speed <= 0.0 {
            return Err(PyValueError::new_err("Speed must be greater than 0."));
        }
        *self.speed.lock().unwrap() = speed;
        Ok(())
    }

    pub fn get_speed(&self) -> f32 {
        *self.speed.lock().unwrap()
    }

    pub fn set_fade(&self, duration: f32, start_vol: f32, end_vol: f32) -> PyResult<()> {
        eprint!("Setting fade");
        if duration < 0.0 {
            return Err(PyValueError::new_err("Duration must be non-negative."));
        }
        if start_vol < 0.0 || start_vol > 1.0 || end_vol < 0.0 || end_vol > 1.0 {
            return Err(PyValueError::new_err("Volume must be between 0.0 and 1.0."));
        }

        let fade_duration = Duration::from_secs_f32(duration);
        let volume_step = (end_vol - start_vol) / fade_duration.as_secs_f32();

        if let Some(sink) = &self.sink {
            let sink = Arc::clone(sink);
            let is_playing = Arc::clone(&self.is_playing);

            thread::spawn(move || {
                let start_time = Instant::now();

                loop {
                    if let Ok(is_playing_guard) = is_playing.lock() {
                        if !*is_playing_guard {
                            break;
                        }
                    }

                    let elapsed = Instant::now() - start_time;

                    if elapsed >= fade_duration {
                        break;
                    }

                    let current_volume = start_vol + volume_step * elapsed.as_secs_f32();
                    let clamped_volume = current_volume.clamp(0.0, 1.0);

                    if let Ok(sink_lock) = sink.lock() {
                        sink_lock.set_volume(clamped_volume);
                    }

                    thread::sleep(Duration::from_millis(100));
                }

                if let Ok(sink_lock) = sink.lock() {
                    sink_lock.set_volume(end_vol.clamp(0.0, 1.0));
                }
            });

            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to set fade. Load audio first.",
            ))
        }
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
    m.add_class::<MetaData>()?;
    m.add_class::<mixer::ChannelManager>()?;
    m.add_class::<audioqueue::AudioChannel>()?;
    Ok(())
}
// fn rpaudio(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_class::<AudioSink>()?;
//     m.add_class::<MetaData>()?;
//     m.add_class::<mixer::ChannelManager>()?;
//     m.add_class::<audioqueue::AudioChannel>()?;
//     Ok(())
// }
