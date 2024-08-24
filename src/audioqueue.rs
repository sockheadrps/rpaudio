use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use std::{fmt, thread};
use std::time::Duration;
use crate::AudioSink;

#[derive(Debug, Clone)]
#[pyclass]
pub struct AudioChannel {
    pub queue: Arc<Mutex<Vec<AudioSink>>>,
    auto_consume: Arc<Mutex<bool>>,
    currently_playing: Arc<Mutex<Option<AudioSink>>>,
}

impl fmt::Debug for AudioSink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AudioSink {{ is_playing: {:?} }}", *self.is_playing.lock().unwrap())
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
    pub fn current_audio(&self) -> Option<AudioSink> {
        let playing_guard = self.currently_playing.lock().unwrap();
        playing_guard.clone() // Return the currently playing sink if it exists
    }

    #[getter]
    pub fn queue_contents(&self) -> Vec<AudioSink> {
        let queue_guard = self.queue.lock().unwrap();
        queue_guard.clone()  // Ensure AudioSink implements Clone
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

    fn _channel_loop(&self) {
        let queue = Arc::clone(&self.queue);
        let auto_consume = Arc::clone(&self.auto_consume);
        let currently_playing = Arc::clone(&self.currently_playing);
    
        thread::spawn(move || {
            loop {
                {
                    let should_consume = *auto_consume.lock().unwrap();
                    if !should_consume {
                        // println!("Auto consume is turned off, sleeping and waiting for it to turn on");
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
    
                        // println!("Playing new sink");
    
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
    
            // println!("Channel loop finished");
        });
    
        // println!("Channel loop started");
    }
}    

