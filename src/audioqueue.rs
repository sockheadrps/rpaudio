use crate::timesync::{self, ActionType};
use crate::AudioSink;
use pyo3::exceptions::PyTypeError;
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
        channel._channel_loop();
        channel
    }

    pub fn push(&mut self, sink: AudioSink) {
        self.queue.lock().unwrap().push(sink);
    }

    pub fn pop(&mut self) -> Option<AudioSink> {
        self.queue.lock().unwrap().pop()
    }

    pub fn consume(&mut self) {
        if let Some(mut sink) = self.pop() {
            let _ = sink.play();
        }
    }

    #[setter]
    pub fn set_auto_consume(&mut self, value: bool) {
        *self.auto_consume.lock().unwrap() = value;
    }

    #[getter]
    pub fn auto_consume(&self) -> bool {
        *self.auto_consume.lock().unwrap()
    }

    #[getter]
    pub fn current_audio(&self) -> Option<AudioSink> {
        let playing_guard = self.currently_playing.lock().unwrap();
        playing_guard.clone()
    }

    pub fn drop_current_audio(&mut self) {
        let mut currently_playing_guard = self.currently_playing.lock().unwrap();
        if let Some(mut sink) = currently_playing_guard.take() {
            let _ = sink.stop();
        }
    }

    #[getter]
    pub fn queue_contents(&self) -> Vec<AudioSink> {
        let queue_guard = self.queue.lock().unwrap();
        queue_guard.clone()
    }

    pub fn set_queue_contents(&mut self, new_queue: Vec<AudioSink>) {
        let mut queue_guard = self.queue.lock().unwrap();
        *queue_guard = new_queue;
    }

    #[getter]
    pub fn is_playing(&self) -> bool {
        let currently_playing_guard = self.currently_playing.lock().unwrap();
        if let Some(ref sink) = *currently_playing_guard {
            sink.is_playing()
        } else {
            false
        }
    }

    #[getter]
    pub fn effects_chain(&self) -> Vec<ActionType> {
        let effects_guard = self.effects_chain.lock().unwrap();
        effects_guard.clone()
    }

    pub fn set_effects_chain(&mut self, effect_list: Py<PyList>) -> PyResult<()> {
        let mut effects_guard = self.effects_chain.lock().unwrap();
        Python::with_gil(|py| {
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

            let _ = py;

            *effects_guard = rust_effect_list;


            Ok(())
        })
    }

    fn apply_effects(&self, sink: &mut AudioSink) {
        println!("Applying effects to sink");
        let effects_guard = self.effects_chain.lock().unwrap();
        for effect in effects_guard.iter() {
            match effect {
                ActionType::FadeIn(fade_in) => {
                    println!(
                        "Executing FadeIn: start_vol={:?}, end_vol={}, duration={}, apply_after={:?}",
                        fade_in.start_vol, fade_in.end_vol, fade_in.duration, fade_in.apply_after
                    );
                    sink.set_fade(
                        fade_in.apply_after,
                        fade_in.duration,
                        fade_in.start_vol,
                        fade_in.end_vol,
                    )
                    .unwrap();
                }
                ActionType::FadeOut(fade_out) => {
                    sink.set_fade(
                        fade_out.apply_after,
                        fade_out.duration,
                        Some(fade_out.start_vol),
                        fade_out.end_vol,
                    )
                    .unwrap();
                    println!("FadeOut effect: {:?}", fade_out);
                }
                ActionType::ChangeSpeed(set_speed) => {
                    sink.set_speed(
                        set_speed.apply_after,
                        set_speed.duration,
                        Some(set_speed.start_speed.unwrap() as f32),
                        set_speed.end_speed,
                    )
                    .unwrap();
                    println!("ChangeSpeed effect: {:?}", set_speed);
                }
            }
        }
    }

    fn _channel_loop(&self) {
        let queue = Arc::clone(&self.queue);
        let auto_consume = Arc::clone(&self.auto_consume);
        let currently_playing = Arc::clone(&self.currently_playing);
        let effects_chain = Arc::clone(&self.effects_chain);
    
        thread::spawn(move || {
            loop {
                {
                    let should_consume = *auto_consume.lock().unwrap();
                    if !should_consume {
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    }
                }
    
                {
                    let mut playing_guard = currently_playing.lock().unwrap();
                    let mut queue_guard = queue.lock().unwrap();

                    if playing_guard.is_none() && !queue_guard.is_empty() {
                        let mut next_sink = queue_guard.remove(0);
                        *playing_guard = Some(next_sink.clone());

                        let effects_chain = Arc::clone(&effects_chain);
    
                        {
                            let effects_guard = effects_chain.lock().unwrap();
                            next_sink.execute_scheduled_effects(effects_guard.to_vec());

                        }

                        if let Err(e) = next_sink.play() {
                            eprintln!("Failed to play sink: {}", e);
                            *playing_guard = None;
                        }
                    }
                }
    
                thread::sleep(Duration::from_millis(100));
    
                {
                    let mut playing_guard = currently_playing.lock().unwrap();

                    if let Some(ref sink) = *playing_guard {
                        let is_playing = sink.is_playing();

                        if !is_playing && sink.empty() {
                            // println!("Finished playing current sink");
                            *playing_guard = None;
                        }
                    }
                }

                {
                    let queue_empty = queue.lock().unwrap().is_empty();
                    let playing_empty = currently_playing.lock().unwrap().is_none();

                    if playing_empty && queue_empty {
                        // println!("Queue is empty, stopping channel loop after audio has finished");
                        break;
                    }
                }
            }
        });
    }
    
}
