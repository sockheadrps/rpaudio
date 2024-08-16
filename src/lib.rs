use pyo3::prelude::*;
use pyo3::types::PyAny;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use pyo3::exceptions::PyRuntimeError;
mod exmetadata;
pub use exmetadata::{extract_metadata, MetaData};



#[pyclass]
pub struct AudioSink {
    is_playing: Arc<Mutex<bool>>,
    callback: Arc<Mutex<Option<Py<PyAny>>>>,
    sink: Option<Arc<Mutex<Sink>>>,
    stream: Option<OutputStream>,
    metadata: MetaData,
}

unsafe impl Send for AudioSink {}

#[pymethods]
impl AudioSink {
    #[new]
    fn new(callback: Option<Py<PyAny>>) -> Self {
        AudioSink {
            is_playing: Arc::new(Mutex::new(false)),
            callback: Arc::new(Mutex::new(callback)),
            sink: None,
            stream: None,
            metadata: MetaData::default(),
        }
    }

    #[getter]
    fn is_playing(&self) -> PyResult<bool> {
        return Ok(*self.is_playing.lock().unwrap());
    }

    fn load_audio(&mut self, file_path: &str) -> PyResult<()> {
        if let Some(_) = self.sink {
            return Ok(());
        }

        self.metadata = exmetadata::extract_metadata(file_path).unwrap();
        

        let (new_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));

        let file = File::open(file_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        sink.lock().unwrap().append(source);

        self.stream = Some(new_stream);
        self.sink = Some(sink.clone());

        let is_playing = self.is_playing.clone();
        let callback = self.callback.clone();

        thread::spawn(move || {
            loop {
                if !*is_playing.lock().unwrap() && sink.lock().unwrap().empty() {
                    break;
                }

                if sink.lock().unwrap().empty() {
                    *is_playing.lock().unwrap() = false;
                    Self::invoke_callback(&*callback.lock().unwrap());
                    break;
                }
                thread::sleep(Duration::from_millis(100));
            }
        });

        Ok(())
    }

    fn metadata(&self) -> MetaData {
        self.metadata.clone()
    }
    

    fn play(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            (*sink.lock().unwrap()).play();
            *self.is_playing.lock().unwrap() = true;
            println!("PLAY");
            Ok(())
        } else {
            let message = "No sink available to play. Load audio first.";
            Err(PyRuntimeError::new_err(message))
        }
    }

    fn pause(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            (*sink.lock().unwrap()).pause();
            *self.is_playing.lock().unwrap() = false;
            println!("PAUSE");
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("No sink available to pause. Load audio first."))
        }
    }

    fn stop(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            (*sink.lock().unwrap()).stop();
            *self.is_playing.lock().unwrap() = false;
            Self::invoke_callback(&*self.callback.lock().unwrap());
            println!("STOP");
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("No sink available to stop. Load audio first."))
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
fn rpaudio(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AudioSink>()?;
    Ok(())
}

