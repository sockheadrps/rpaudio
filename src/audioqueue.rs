use crate::timesync::{self, ActionType};
use crate::AudioSink;
use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fmt, thread};
use timesync::{ChangeSpeed, FadeIn, FadeOut};

#[derive(Debug, Clone)]
#[pyclass]
pub struct AudioChannel {
    pub queue: Arc<Mutex<Vec<AudioSink>>>,
    auto_consume: Arc<Mutex<bool>>,
    currently_playing: Arc<Mutex<Option<AudioSink>>>,
    effects_chain: Arc<Mutex<Vec<ActionType>>>,
}

impl fmt::Debug for AudioSink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AudioSink {{ is_playing: {:?} }}", self.is_playing)
    }
}

#[pymethods]
impl AudioChannel {
    #[new]
    pub fn new() -> Self {
        let channel = Self {
            queue: Arc::new(Mutex::new(Vec::new())),
            auto_consume: Arc::new(Mutex::new(false)),
            currently_playing: Arc::new(Mutex::new(None)),
            effects_chain: Arc::new(Mutex::new(Vec::new())),
        };

        let channel_arc = Arc::new(Mutex::new(channel));
        let channel_clone = Arc::clone(&channel_arc);

        thread::spawn(move || {
            let mut backoff = 10;

            loop {
                let channel = channel_clone.lock().unwrap();

                let should_consume = {
                    let auto_consume_guard = match channel.auto_consume.lock() {
                        Ok(guard) => *guard,
                        Err(_) => {
                            println!(
                                "Failed to acquire lock on auto_consume, retrying after backoff"
                            );
                            thread::sleep(Duration::from_millis(backoff));
                            backoff = std::cmp::min(backoff * 2, 1000);
                            continue;
                        }
                    };
                    auto_consume_guard
                };

                if !should_consume {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }

                if let (Ok(mut playing_guard), Ok(mut queue_guard)) =
                    (channel.currently_playing.lock(), channel.queue.lock())
                {
                    if playing_guard.is_none() && !queue_guard.is_empty() {
                        let mut next_sink = queue_guard.remove(0);

                        drop(queue_guard); 

                        if let Err(e) = next_sink.play() {
                            eprintln!("Failed to play sink: {}", e);
                            continue;
                        }

                        *playing_guard = Some(next_sink);

                        let effects_guard = match channel.effects_chain.lock() {
                            Ok(guard) => guard,
                            Err(_) => {
                                eprintln!("Failed to acquire lock on effects_chain");
                                continue;
                            }
                        };

                        if let Some(sender) = playing_guard.as_mut().unwrap().action_sender.take() {
                            for effect in effects_guard.iter() {
                                if let Err(e) = sender.send(effect.clone()) {
                                    eprintln!("Failed to send effect: {}", e);
                                }
                            }
                        } else {
                            eprintln!("Action sender is None");
                        }
                    }
                } else {
                    eprintln!("Failed to acquire locks on currently_playing or queue");
                }

                if let Ok(mut playing_guard) = channel.currently_playing.lock() {
                    if let Some(ref mut sink) = *playing_guard {
                        let force_stop = {
                            let force_stop_guard = sink.force_stop.read().unwrap();
                            *force_stop_guard
                        };

                        let is_sink_empty = {
                            let sink_guard = sink.sink.as_ref().unwrap().lock().unwrap();
                            sink_guard.empty()
                        };

                        if (!sink.is_playing() && is_sink_empty) || force_stop {
                            if let Err(e) = sink.stop() {
                                eprintln!("Failed to stop sink: {}", e);
                            }
                            *playing_guard = None;
                        }
                    }
                } else {
                    eprintln!("Failed to acquire lock on currently_playing in _channel_loop()");
                }

                thread::sleep(Duration::from_millis(100)); 
            }
        });

        let x = channel_arc.lock().unwrap().clone();
        x
    }

    pub fn push(&mut self, sink: AudioSink) {
        if let Ok(mut queue_guard) = self.queue.lock() {
            queue_guard.push(sink);
        } else {
            eprintln!("Failed to acquire lock on queue in push()");
        }
    }

    pub fn pop(&mut self) -> Option<AudioSink> {
        if let Ok(mut queue_guard) = self.queue.lock() {
            queue_guard.pop()
        } else {
            eprintln!("Failed to acquire lock on queue in pop()");
            None
        }
    }

    pub fn consume(&mut self) {
        if let Some(mut sink) = self.pop() {
            let _ = sink.play();
        }
    }

    #[setter]
    pub fn set_auto_consume(&mut self, value: bool) {
        if let Ok(mut auto_consume_guard) = self.auto_consume.lock() {
            *auto_consume_guard = value;
        } else {
            eprintln!("Failed to acquire lock on auto_consume in set_auto_consume()");
        }
    }

    #[getter]
    pub fn auto_consume(&self) -> bool {
        if let Ok(auto_consume_guard) = self.auto_consume.lock() {
            *auto_consume_guard
        } else {
            eprintln!("Failed to acquire lock on auto_consume in auto_consume()");
            false
        }
    }

    #[getter]
    pub fn current_audio(&self) -> Option<AudioSink> {
        if let Ok(playing_guard) = self.currently_playing.lock() {
            playing_guard.clone()
        } else {
            println!("Failed to acquire lock on currently_playing in current_audio()");
            None
        }
    }

    pub fn drop_current_audio(&mut self) {
        if let Ok(mut currently_playing_guard) = self.currently_playing.lock() {
            if let Some(mut sink) = currently_playing_guard.take() {
                let _ = sink.stop();
            }
        } else {
            eprintln!("Failed to acquire lock on currently_playing in drop_current_audio()");
        }
    }

    #[getter]
    pub fn queue_contents(&self) -> Vec<AudioSink> {
        if let Ok(queue_guard) = self.queue.lock() {
            queue_guard.clone()
        } else {
            eprintln!("Failed to acquire lock on queue in queue_contents()");
            Vec::new()
        }
    }

    #[getter]
    pub fn is_playing(&self) -> bool {
        if let Ok(currently_playing_guard) = self.currently_playing.lock() {
            if let Some(ref sink) = *currently_playing_guard {
                let playing_state = sink.is_playing();
                playing_state
            } else {
                false
            }
        } else {
            eprintln!("Failed to acquire lock on currently_playing in is_playing()");
            false
        }
    }

    #[getter]
    pub fn effects(&self, py: Python) -> PyResult<Py<PyList>> {
        let effects_guard = self.effects_chain.lock().unwrap();
        println!("lock acquired for effects_chain");

        let effects_list: Vec<PyObject> = effects_guard
            .iter()
            .map(|effect| match effect {
                ActionType::FadeIn(fade_in) => Py::new(py, fade_in.clone()).unwrap().into_py(py),
                ActionType::FadeOut(fade_out) => Py::new(py, fade_out.clone()).unwrap().into_py(py),
                ActionType::ChangeSpeed(change_speed) => {
                    Py::new(py, change_speed.clone()).unwrap().into_py(py)
                }
            })
            .collect();
        let py_list = PyList::new_bound(py, effects_list);

        Ok(py_list.into())
    }

    pub fn set_effects_chain(&mut self, effect_list: Py<PyList>) -> PyResult<()> {
        Python::with_gil(|py| {
            let mut effects_guard = match self.effects_chain.lock() {
                Ok(guard) => guard,
                Err(_) => {
                    return Err(PyRuntimeError::new_err(
                        "Failed to acquire lock on effects_chain",
                    ));
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
        })
    }

    pub fn current_audio_data(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(playing_guard) = self.currently_playing.lock() {
                if let Some(ref sink) = *playing_guard {
                    let metadata = sink.metadata.clone();

                    let dict = PyDict::new_bound(py);
                    dict.set_item("album_artist", metadata.album_artist)?;
                    dict.set_item("album_title", metadata.album_title)?;
                    dict.set_item("artist", metadata.artist)?;
                    dict.set_item("channels", metadata.channels)?;
                    dict.set_item("comment", metadata.comment)?;
                    dict.set_item("composer", metadata.composer)?;
                    dict.set_item("date", metadata.date)?;
                    dict.set_item("disc_number", metadata.disc_number)?;
                    dict.set_item("duration", metadata.duration)?;
                    dict.set_item("genre", metadata.genre)?;
                    dict.set_item("sample_rate", metadata.sample_rate)?;
                    dict.set_item("title", metadata.title)?;
                    dict.set_item("total_discs", metadata.total_discs)?;
                    dict.set_item("total_tracks", metadata.total_tracks)?;
                    dict.set_item("track_number", metadata.track_number)?;
                    dict.set_item("year", metadata.year)?;
                    dict.set_item("speed", sink.get_speed())?;
                    dict.set_item("position", sink.get_pos()?)?;
                    dict.set_item("volume", sink.get_volume()?)?;

                    let effects_list: Vec<PyObject> = Vec::new();
                    let effects_list = PyList::new_bound(py, &effects_list);

                    for effect in self.effects_chain.lock().unwrap().iter() {
                        let effect_dict = PyDict::new_bound(py);
                        match effect {
                            ActionType::FadeIn(FadeIn {
                                duration,
                                apply_after,
                                start_val,
                                end_val,
                            }) => {
                                let fadein_dict = PyDict::new_bound(py);
                                fadein_dict.set_item("duration", duration)?;
                                fadein_dict.set_item("apply_after", apply_after)?;
                                fadein_dict.set_item("start_val", start_val)?;
                                fadein_dict.set_item("end_val", end_val)?;
                                effect_dict.set_item("FadeIn", fadein_dict)?;
                            }
                            ActionType::FadeOut(FadeOut {
                                duration,
                                apply_after,
                                ..
                            }) => {
                                let fadeout_dict = PyDict::new_bound(py);
                                fadeout_dict.set_item("duration", duration)?;
                                fadeout_dict.set_item("apply_after", apply_after)?;
                                effect_dict.set_item("FadeOut", fadeout_dict)?;
                            }
                            ActionType::ChangeSpeed(ChangeSpeed {
                                duration,
                                end_val,
                                apply_after,
                                ..
                            }) => {
                                let changespeed_dict = PyDict::new_bound(py);
                                changespeed_dict.set_item("duration", duration)?;
                                changespeed_dict.set_item("end_val", end_val)?;
                                changespeed_dict.set_item("apply_after", apply_after)?;
                                effect_dict.set_item("ChangeSpeed", changespeed_dict)?;
                            }
                            _ => {}
                        }
                        effects_list.append(effect_dict)?;
                    }
                    dict.set_item("effects", effects_list)?;
                    return Ok(dict.into());
                } else {
                    return Ok(py.None());
                }
            }
            Err(PyRuntimeError::new_err(
                "Failed to acquire lock on currently_playing",
            ))
        })
    }
}
