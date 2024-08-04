use pyo3::prelude::*;
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;

#[pyfunction]
fn play_audio(file_path: &str, duration: u64) -> PyResult<()> {
    // Create an output stream
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Load the sound file
    let file = File::open(file_path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    // Play the sound file
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // Keep the program running while the sound is playing
    std::thread::sleep(std::time::Duration::from_secs(duration));

    Ok(())
}

#[pymodule (name = "rpaudio")]
fn audio_player(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(play_audio, m)?)?;
    Ok(())
}





















// use pyo3::prelude::*;
// use rodio::{Decoder, OutputStream, Sink};
// use std::fs::File;
// use std::io::BufReader;
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::sync::mpsc::{self, Receiver, Sender};

// #[pyclass]
// struct AudioPlayer {
//     sink: Arc<Mutex<Sink>>,
//     control_tx: Arc<Mutex<Sender<String>>>,
// }

// #[pymethods]
// impl AudioPlayer {
//     #[new]
//     fn new(file_path: String) -> PyResult<Self> {
//         // Create an output stream
//         let (_stream, stream_handle) = OutputStream::try_default().unwrap();

//         // Load the sound file
//         let file = File::open(file_path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
//         let source = Decoder::new(BufReader::new(file)).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

//         // Create a sink to control the playback
//         let sink = Sink::try_new(&stream_handle).unwrap();
//         sink.append(source);

//         let sink = Arc::new(Mutex::new(sink));

//         // Channel to communicate between the main thread and the control thread
//         let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
//         let control_tx = Arc::new(Mutex::new(tx));

//         let sink_clone = Arc::clone(&sink);

//         // Spawn a thread to handle control messages
//         thread::spawn(move || {
//             loop {
//                 match rx.recv().unwrap().as_str() {
//                     "pause" => {
//                         let sink = sink_clone.lock().unwrap();
//                         sink.pause();
//                     }
//                     "resume" => {
//                         let sink = sink_clone.lock().unwrap();
//                         sink.play();
//                     }
//                     "quit" => break,
//                     _ => {}
//                 }
//             }
//         });

//         Ok(AudioPlayer { sink, control_tx })
//     }

//     fn pause(&self) {
//         let tx = self.control_tx.lock().unwrap();
//         tx.send("pause".to_string()).unwrap();
//     }

//     fn resume(&self) {
//         let tx = self.control_tx.lock().unwrap();
//         tx.send("resume".to_string()).unwrap();
//     }
// }

// #[pymodule]
// fn rpaudio(py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_class::<AudioPlayer>()?;
//     Ok(())
// }
