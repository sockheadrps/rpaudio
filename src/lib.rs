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

