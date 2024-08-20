use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::{thread, time::Duration};
use rodio::{Decoder, OutputStream, Sink};
use std::io::BufReader;
mod exmetadata;
mod audioqueue;
pub use exmetadata::{MetaData, metadata};
unsafe impl Send for AudioSink {}

#[derive(Clone)]
#[pyclass]
pub struct AudioSink {
    is_playing: Arc<Mutex<bool>>,
    callback: Arc<Mutex<Option<Py<PyAny>>>>,
    sink: Option<Arc<Mutex<Sink>>>,
    stream: Option<Arc<Mutex<OutputStream>>>,
    metadata: MetaData,
}

#[pymethods]
impl AudioSink {
    #[new]
    pub fn new(callback: Option<Py<PyAny>>) -> Self {
        AudioSink {
            is_playing: Arc::new(Mutex::new(false)),
            callback: Arc::new(Mutex::new(callback)),
            sink: None,
            stream: None,
            metadata: MetaData::default(),
        }
    }

    #[getter]
    pub fn metadata(&self) -> MetaData {
        self.metadata.clone()
    }

    #[getter]
    pub fn is_playing(&self) -> bool {
        *self.is_playing.lock().unwrap()
    }

    pub fn load_audio(&mut self, file_path: String) -> PyResult<()> {
        if self.sink.is_some() {
            println!("Sink already exists, unload it first");
            return Ok(());
        }

        let metadata = metadata(&file_path)?;
        self.metadata = metadata;

        let (new_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink_result = Sink::try_new(&stream_handle);
        let sink = match sink_result {
            Ok(s) => Arc::new(Mutex::new(s)),
            Err(e) => return Err(PyRuntimeError::new_err(format!("Failed to create sink: {}", e))),
        };

        let file_path_clone = file_path.clone();
        let file = File::open(file_path_clone).unwrap();
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| format!("Failed to decode audio file: {}", e))
            .unwrap();
        sink.lock().unwrap().append(source);

        self.stream = Some(Arc::new(Mutex::new(new_stream)));
        self.sink = Some(sink.clone());

        if let Some(sink) = &self.sink {
            (*sink.lock().unwrap()).pause();
        }

        let is_playing = self.is_playing.clone();
        let callback = self.callback.clone();
        let sink_clone = sink.clone();

        let file_path_clone = file_path.clone();
        thread::spawn(move || {
            loop {
                {
                    let mut is_playing_guard = is_playing.lock().unwrap();
                    let sink = sink_clone.lock().unwrap();

                    if sink.empty() {
                        println!("Sink is empty, stopping playback of {}", file_path_clone);
                        *is_playing_guard = false;
                        drop(is_playing_guard);
                        Self::invoke_callback(&*callback.lock().unwrap());
                        break;
                    }

                    if sink.is_paused() {
                        *is_playing_guard = false;
                    } else {
                        *is_playing_guard = true;
                    }
                }

                thread::sleep(Duration::from_millis(100));
            }

            let mut is_playing_guard = is_playing.lock().unwrap();
            *is_playing_guard = false;
        });

        Ok(())
    }

    pub fn play(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().play();
            *self.is_playing.lock().unwrap() = true;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err("No sink available to play. Load audio first."))
        }
    }

    pub fn pause(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().pause();
            *self.is_playing.lock().unwrap() = false;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err("No sink available to pause. Load audio first."))
        }
    }

    pub fn stop(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().stop();
            *self.is_playing.lock().unwrap() = false;
            Self::invoke_callback(&*self.callback.lock().unwrap());
            Ok(())
        } else {
            Err(PyRuntimeError::new_err("No sink available to stop. Load audio first."))
        }
    }

    pub fn empty(&self) -> bool {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().empty()
        } else {
            false
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
fn rpaudio(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AudioSink>()?;
    m.add_class::<MetaData>()?;
    audioqueue::audioqueue(py, m)?;
    Ok(())
}
