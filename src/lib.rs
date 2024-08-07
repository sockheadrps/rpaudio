use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum Command {
    Load(String),
    Pause,
    Resume,
    Stop,
}

#[pyclass]
pub struct AudioHandler {
    command_sender: Arc<Mutex<mpsc::Sender<Command>>>,
}

#[pymethods]
impl AudioHandler {
    #[new]
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));

        // Start the audio player thread
        let sender_clone = Arc::clone(&sender);
        thread::spawn(move || {
            let mut stream = None;
            let mut sink = None;

            while let Ok(command) = receiver.recv() {
                match command {
                    Command::Load(file_path) => {
                        let (new_stream, stream_handle) = OutputStream::try_default().unwrap();
                        let new_sink = Sink::try_new(&stream_handle).unwrap();

                        let file = File::open(file_path).unwrap();
                        let source = Decoder::new(BufReader::new(file)).unwrap();
                        new_sink.append(source);

                        stream = Some(new_stream);
                        sink = Some(new_sink);
                    }
                    Command::Pause => {
                        if let Some(sink) = &sink {
                            sink.pause();
                        }
                    }
                    Command::Resume => {
                        if let Some(sink) = &sink {
                            sink.play();
                        }
                    }
                    Command::Stop => {
                        if let Some(sink) = &sink {
                            sink.stop();
                        }
                        break;
                    }
                }
            }
        });

        AudioHandler {
            command_sender: sender,
        }
    }

    fn load_audio(&self, file_path: &str) -> PyResult<()> {
        let command = Command::Load(file_path.to_string());
        self.command_sender.lock().unwrap().send(command)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn pause(&self) -> PyResult<()> {
        let command = Command::Pause;
        self.command_sender.lock().unwrap().send(command)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn resume(&self) -> PyResult<()> {
        let command = Command::Resume;
        self.command_sender.lock().unwrap().send(command)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }

    fn stop(&self) -> PyResult<()> {
        let command = Command::Stop;
        self.command_sender.lock().unwrap().send(command)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(())
    }
}


#[pymodule]
fn rpaudio(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AudioHandler>()?;
    Ok(())
}