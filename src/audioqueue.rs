use crate::timesync::{self, ActionType, ExtractableEffect};
use crate::{AudioSink, MetaData};
use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList};
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
                        Ok(guard) => guard,
                        Err(_) => {
                            println!(
                                "Failed to acquire lock on auto_consume, retrying after backoff"
                            );
                            thread::sleep(Duration::from_millis(backoff));
                            backoff = std::cmp::min(backoff * 2, 1000);
                            continue;
                        }
                    };
                    *auto_consume_guard
                };

                if !should_consume {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }

                {
                    if let (Ok(mut playing_guard), Ok(mut queue_guard)) =
                        (channel.currently_playing.lock(), channel.queue.lock())
                    {
                        if playing_guard.is_none() && !queue_guard.is_empty() {
                            let mut next_sink = queue_guard.remove(0);

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

                            if let Some(sender) =
                                playing_guard.as_mut().unwrap().action_sender.take()
                            {
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
                            if !sink.is_playing()
                                && sink.sink.as_ref().unwrap().lock().unwrap().empty()
                            {
                                if let Err(e) = sink.stop() {
                                    eprintln!("Failed to stop sink: {}", e);
                                }
                                *playing_guard = None;
                            }
                        }
                    } else {
                        eprintln!("Failed to acquire lock on currently_playing in _channel_loop()");
                    }
                }

                thread::sleep(Duration::from_millis(100));
            }
        });

        let x = channel_arc.lock().unwrap().clone();
        x
    }

    pub fn push(&mut self, sink: AudioSink) {
        if let Ok(mut queue_guard) = self.queue.try_lock() {
            queue_guard.push(sink);
        } else {
            eprintln!("Failed to acquire lock on queue in push()");
        }
    }

    pub fn pop(&mut self) -> Option<AudioSink> {
        if let Ok(mut queue_guard) = self.queue.try_lock() {
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
        if let Ok(mut auto_consume_guard) = self.auto_consume.try_lock() {
            *auto_consume_guard = value;
        } else {
            eprintln!("Failed to acquire lock on auto_consume in set_auto_consume()");
        }
    }

    #[getter]
    pub fn auto_consume(&self) -> bool {
        if let Ok(auto_consume_guard) = self.auto_consume.try_lock() {
            *auto_consume_guard
        } else {
            eprintln!("Failed to acquire lock on auto_consume in auto_consume()");
            false
        }
    }

    #[getter]
    pub fn current_audio(&self) -> Option<AudioSink> {
        if let Ok(playing_guard) = self.currently_playing.try_lock() {
            playing_guard.clone()
        } else {
            println!("Failed to acquire lock on currently_playing in current_audio()");
            None
        }
    }

    pub fn drop_current_audio(&mut self) {
        if let Ok(mut currently_playing_guard) = self.currently_playing.try_lock() {
            if let Some(mut sink) = currently_playing_guard.take() {
                let _ = sink.stop();
            }
        } else {
            eprintln!("Failed to acquire lock on currently_playing in drop_current_audio()");
        }
    }

    #[getter]
    pub fn queue_contents(&self) -> Vec<AudioSink> {
        if let Ok(queue_guard) = self.queue.try_lock() {
            queue_guard.clone()
        } else {
            eprintln!("Failed to acquire lock on queue in queue_contents()");
            Vec::new()
        }
    }

    #[getter]
    pub fn is_playing(&self) -> bool {
        if let Ok(currently_playing_guard) = self.currently_playing.try_lock() {
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
        let effects_guard = self.effects_chain.try_lock().unwrap();
        println!("lock acquired for effects_chain");

        let effects_list: Vec<PyObject> = effects_guard
            .iter()
            .map(|effect| match effect {
                ActionType::FadeIn(fade_in) => Py::new(py, fade_in.clone()).unwrap().into_py(py),
                ActionType::FadeOut(fade_out) => Py::new(py, fade_out.clone()).unwrap().into_py(py),
                ActionType::ChangeSpeed(change_speed) => Py::new(py, change_speed.clone()).unwrap().into_py(py)
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
            if let Ok(playing_guard) = self.currently_playing.try_lock() {
                if let Some(ref sink) = *playing_guard {
                    let metadata = sink.metadata.clone();

                    let dict = metadata.into_py_dict_bound(py);

                    // let effects_list = self.effects(py).unwrap();
                    let effects_list = PyList::new_bound(py, &Vec::<PyObject>::new());

                    for effect in self.effects_chain.try_lock().unwrap().iter() {
                        let effect_dict = match effect {
                            ActionType::FadeIn(fi)=> fi.into_py_dict_bound(py),
                            ActionType::FadeOut(fo) => fo.into_py_dict_bound(py),
                            ActionType::ChangeSpeed(cs) => cs.into_py_dict_bound(py)
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
