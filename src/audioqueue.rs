use crate::timesync::{ActionType, ExtractableEffect};
use crate::AudioSink;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fmt, thread};

#[derive(Debug, Clone)]
#[pyclass]
pub struct AudioChannel {
    pub queue: Arc<Mutex<Vec<AudioSink>>>,
    pub auto_consume: Arc<Mutex<bool>>,
    currently_playing: Arc<Mutex<Option<AudioSink>>>,
    effects_chain: Arc<Mutex<Vec<ActionType>>>,
    channel_volume: Arc<Mutex<f32>>,
}

impl fmt::Debug for AudioSink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AudioSink {{ is_playing: {:?} }}", self.is_playing)
    }
}

impl AudioChannel {
    pub fn pop(&mut self) -> Option<AudioSink> {
        if let Ok(mut queue_guard) = self.queue.lock() {
            queue_guard.pop()
        } else {
            None
        }
    }

    pub fn consume(&mut self) {
        if let Some(mut sink) = self.pop() {
            let volume = self.channel_volume.lock().unwrap();
            let _ = sink.set_volume(*volume);
            let _ = sink.play();
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        if let Ok(mut volume_guard) = self.channel_volume.lock() {
            *volume_guard = volume;
        }
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
            channel_volume: Arc::new(Mutex::new(1.0)),
        };

        let channel_arc = Arc::new(Mutex::new(channel));
        let channel_clone = Arc::clone(&channel_arc);

        thread::spawn(move || {

            loop {
                let channel = channel_clone.lock().unwrap();

                let should_consume = match channel.auto_consume.lock() {
                    Ok(guard) => *guard,
                    Err(_) => continue,
                };

                if !should_consume {
                    thread::sleep(Duration::from_millis(3));
                    continue;
                }

                if let (Ok(mut playing_guard), Ok(mut queue_guard)) =
                    (channel.currently_playing.lock(), channel.queue.lock())
                {
                    if playing_guard.is_none() && !queue_guard.is_empty() {
                        let mut next_sink = queue_guard.remove(0);
                        let volume = channel.channel_volume.lock().unwrap();
                        let _ = next_sink.set_volume(*volume);

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
                        }
                    }
                }

                if let Ok(mut playing_guard) = channel.currently_playing.lock() {
                    if let Some(ref mut sink) = *playing_guard {
                        let is_sink_empty = {
                            let sink_guard = sink.sink.as_ref().unwrap().lock().unwrap();
                            sink_guard.empty()
                        };

                        if !sink.is_playing() && is_sink_empty {
                            if let Err(e) = sink.stop() {
                                eprintln!("Failed to stop sink: {}", e);
                            }
                            *playing_guard = None;
                        }
                    }
                }

                thread::sleep(Duration::from_millis(3));
            }
        });

        let x = channel_arc.lock().unwrap().clone();
        x
    }

    pub fn push(&mut self, sink: AudioSink) {
        if let Ok(mut queue_guard) = self.queue.lock() {
            queue_guard.push(sink);
        }
    }

    #[setter]
    pub fn set_auto_consume(&mut self, value: bool) {
        if let Ok(mut auto_consume_guard) = self.auto_consume.lock() {
            *auto_consume_guard = value;
        }
    }

    #[setter]
    pub fn  channel_volume(&mut self, volume: f32) {
        if let Ok(mut volume_guard) = self.channel_volume.lock() {
            *volume_guard = volume;
            if let Ok(mut currently_playing) = self.currently_playing.lock() {
                if let Some(ref mut sink) = *currently_playing {
                    let _ = sink.set_volume(volume);
                }
            }
        }
    }

    #[getter]
    pub fn auto_consume(&self) -> bool {
        if let Ok(auto_consume_guard) = self.auto_consume.lock() {
            *auto_consume_guard
        } else {
            false
        }
    }

    #[getter]
    pub fn current_audio(&self) -> Option<AudioSink> {
        if let Ok(playing_guard) = self.currently_playing.lock() {
            playing_guard.clone()
        } else {
            None
        }
    }

    pub fn drop_current_audio(&mut self) {
        if let Ok(mut currently_playing_guard) = self.currently_playing.lock() {
            if let Some(mut sink) = currently_playing_guard.take() {
                let _ = sink.stop();
            }
        }
    }

    #[getter]
    pub fn queue_contents(&self) -> Vec<AudioSink> {
        if let Ok(queue_guard) = self.queue.lock() {
            queue_guard.clone()
        } else {
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
                .map(|effect| effect.downcast_bound(py).unwrap().extract_action())
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

                    let dict = metadata.into_py_dict_bound(py);

                    let effects_list = PyList::new_bound(py, &Vec::<PyObject>::new());

                    for effect in self.effects_chain.try_lock().unwrap().iter() {
                        let effect_dict = match effect {
                            ActionType::FadeIn(fi) => fi.into_py_dict_bound(py),
                            ActionType::FadeOut(fo) => fo.into_py_dict_bound(py),
                            ActionType::ChangeSpeed(cs) => cs.into_py_dict_bound(py),
                        };
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
