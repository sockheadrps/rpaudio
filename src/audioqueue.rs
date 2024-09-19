use crate::timesync::{self, ActionType};
use crate::AudioSink;
use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::PyList;
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
        write!(
            f,
            "AudioSink {{ is_playing: {:?} }}",
            *self.is_playing.lock().unwrap()
        )
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
                    thread::sleep(Duration::from_millis(500));
                    continue;
                }

                if let (Ok(mut playing_guard), Ok(mut queue_guard)) =
                    (channel.currently_playing.lock(), channel.queue.lock())
                {
                    if playing_guard.is_none() && !queue_guard.is_empty() {
                        let mut next_sink = queue_guard.remove(0);
                        *playing_guard = Some(next_sink.clone());
                        println!("Playing next sink: {:?}", next_sink);

                        let effects_guard = match channel.effects_chain.lock() {
                            Ok(guard) => guard,
                            Err(_) => {
                                eprintln!("Failed to acquire lock on effects_chain");
                                continue;
                            }
                        };

                        if let Some(sender) = next_sink.action_sender.take() {
                            for effect in effects_guard.iter() {
                                sender.send(effect.clone()).unwrap();
                            }
                        } else {
                            eprintln!("Action sender is None");
                        }

                        if let Err(e) = next_sink.play() {
                            eprintln!("Failed to play sink: {}", e);
                            *playing_guard = None;
                        }
                    }
                } else {
                    eprintln!("Failed to acquire locks on currently_playing or queue");
                }

                if let Ok(mut playing_guard) = channel.currently_playing.lock() {
                    if let Some(ref mut sink) = *playing_guard {
                        if !sink.is_playing() {
                            println!("Sink is not playing, stopping it");
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
            println!("Successfully set auto_consume to {}", value);
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
        let playing_guard = self.currently_playing.lock().unwrap();
        playing_guard.clone()
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

    pub fn set_queue_contents(&mut self, new_queue: Vec<AudioSink>) {
        if let Ok(mut queue_guard) = self.queue.lock() {
            *queue_guard = new_queue;
        } else {
            eprintln!("Failed to acquire lock on queue in set_queue_contents()");
        }
    }

    #[getter]
    pub fn is_playing(&self) -> bool {
        if let Ok(currently_playing_guard) = self.currently_playing.lock() {
            if let Some(ref sink) = *currently_playing_guard {
                sink.is_playing()
            } else {
                false
            }
        } else {
            eprintln!("Failed to acquire lock on currently_playing in is_playing()");
            false
        }
    }

    #[getter]
    pub fn effects_chain(&self) -> Vec<ActionType> {
        if let Ok(effects_guard) = self.effects_chain.lock() {
            effects_guard.clone()
        } else {
            eprintln!("Failed to acquire lock on effects_chain in effects_chain()");
            Vec::new()
        }
    }

    pub fn set_effects_chain(&mut self, effect_list: Py<PyList>) -> PyResult<()> {
        Python::with_gil(|py| {
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
        })
    }
}
