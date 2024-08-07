use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::exceptions::PyRuntimeError;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self};
use std::time::Duration;

enum Command {
    Load(String),
    Pause,
    Stop,
    Play,
}

#[pyclass]
pub struct AudioHandler {
    command_sender: Arc<Mutex<mpsc::Sender<Command>>>,
    is_playing: Arc<Mutex<bool>>,
    callback: Arc<Mutex<Option<Py<PyAny>>>>,
}

#[pymethods]
impl AudioHandler {
    #[new]
    fn new(callback: Option<Py<PyAny>>) -> Self {
        pyo3::prepare_freethreaded_python();

        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let is_playing = Arc::new(Mutex::new(false));
        let callback = Arc::new(Mutex::new(callback));

        let _command_sender = Arc::clone(&sender);
        let is_playing_clone = Arc::clone(&is_playing);
        let callback_clone = Arc::clone(&callback);

        thread::spawn(move || {
            let mut _stream = None;
            let mut sink = None;

            while let Ok(command) = receiver.recv() {
                match command {
                    Command::Load(file_path) => {
                        let (new_stream, stream_handle) = OutputStream::try_default().unwrap();
                        let new_sink = Sink::try_new(&stream_handle).unwrap();

                        let file = File::open(file_path).unwrap();
                        let source = Decoder::new(BufReader::new(file)).unwrap();
                        new_sink.append(source);

                        _stream = Some(new_stream);
                        sink = Some(new_sink);

                        let sink = sink.take().unwrap();
                        let is_playing_clone = Arc::clone(&is_playing_clone);
                        let callback_clone = Arc::clone(&callback_clone);

                        thread::spawn(move || {
                            loop {
                                if sink.empty() {
                                    *is_playing_clone.lock().unwrap() = false;
                                    AudioHandler::invoke_callback(&callback_clone);
                                    break;
                                }
                                thread::sleep(Duration::from_millis(100));
                            }
                        });
                    }
                    Command::Play => {
                        if let Some(sink) = &sink {
                            sink.play();
                            *is_playing_clone.lock().unwrap() = true;
                        }
                    }
                    Command::Pause => {
                        if let Some(sink) = &sink {
                            sink.pause();
                            *is_playing_clone.lock().unwrap() = false;
                        }
                    }
                    Command::Stop => {
                        if let Some(sink) = &sink {
                            sink.stop();
                            *is_playing_clone.lock().unwrap() = false;
                        }
                        AudioHandler::invoke_callback(&callback_clone);
                        break;
                    }
                }
            }
        });

        AudioHandler {
            command_sender: sender,
            is_playing,
            callback,
        }
    }

    fn load_audio(&self, file_path: &str) -> PyResult<()> {
        let command = Command::Load(file_path.to_string());
        self.command_sender.lock().unwrap().send(command)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn play(&self) -> PyResult<()> {
        *self.is_playing.lock().unwrap() = true;
        let command = Command::Play;
        self.command_sender.lock().unwrap().send(command)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn pause(&self) -> PyResult<()> {
        *self.is_playing.lock().unwrap() = false;
        let command = Command::Pause;
        self.command_sender.lock().unwrap().send(command)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn stop(&self) -> PyResult<()> {
        *self.is_playing.lock().unwrap() = false;
        let command = Command::Stop;
        self.command_sender.lock().unwrap().send(command)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn is_playing(&self) -> bool {
        *self.is_playing.lock().unwrap()
    }
}

impl AudioHandler {
    fn invoke_callback(callback: &Arc<Mutex<Option<Py<PyAny>>>>) {
        Python::with_gil(|py| {
            if let Some(callback) = &*callback.lock().unwrap() {
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
