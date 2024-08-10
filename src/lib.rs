use pyo3::prelude::*;
use pyo3::types::PyAny;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


#[pyclass]
pub struct AudioHandler {
    is_playing: Arc<Mutex<bool>>,
    callback: Arc<Mutex<Option<Py<PyAny>>>>,
    sink: Option<Arc<Mutex<Sink>>>
}

#[pymethods]
impl AudioHandler {
    #[new]
    fn new(callback: Option<Py<PyAny>>) -> Self {
        AudioHandler {
            // command_sender: sender,
            is_playing: Arc::new(Mutex::new(false)),
            callback: Arc::new(Mutex::new(callback)),
            sink: None
        }
    }

    fn load_audio(&mut self, file_path: &str) -> PyResult<()> {
        if let Some(_) = self.sink {
            return Ok(());
        }

        let (new_stream, stream_handle) = OutputStream::try_default().unwrap();
        self.sink = Some(Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap())));

        let file = File::open(file_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        (*self.sink.as_ref().unwrap().lock().unwrap()).pause();
        (*self.sink.as_ref().unwrap().lock().unwrap()).append(source);

        let _stream = Some(new_stream);
        let is_playing = self.is_playing.clone();
        let callback = self.callback.clone();

        let sink = self.sink.as_ref().unwrap().clone();

        thread::spawn(move || {
            loop {
                let tmp = &*sink.lock().unwrap();
                if tmp.empty() {
                    *is_playing.lock().unwrap() = false;
                    Self::invoke_callback(&*callback.lock().unwrap());
                    break;
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
        Ok(())
    }

    fn play(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            let sink = &*sink.lock().unwrap();
            sink.play();
            *self.is_playing.lock().unwrap() = true;
            sink.sleep_until_end()
        }
        Ok(())
    }

    fn pause(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            (*sink.lock().unwrap()).pause();
            *self.is_playing.lock().unwrap() = false;
        }
        Ok(())
    }

    fn stop(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            (*sink.lock().unwrap()).stop();
            *self.is_playing.lock().unwrap() = false;
        }
        Self::invoke_callback(&*self.callback.lock().unwrap());
        Ok(())
    }

    fn is_playing(&self) -> bool {
        *self.is_playing.lock().unwrap()
    }
}

impl AudioHandler {
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
    m.add_class::<AudioHandler>()?;
    Ok(())
}
