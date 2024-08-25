use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::audioqueue::AudioChannel;

#[derive(Debug)]
#[pyclass]
pub struct ChannelManager {
    channels: Arc<Mutex<HashMap<String, AudioChannel>>>,
}

#[pymethods]
impl ChannelManager {
    #[new]
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_channel(&self, name: String, channel: &AudioChannel) {
        self.channels.lock().unwrap().insert(name, channel.clone());
    }

    pub fn drop_channel(&self, name: String) -> PyResult<()> {
        let mut channels = self.channels.lock().unwrap();

        if let Some(channel) = channels.remove(&name) {
            if let Some(mut current_audio) = channel.current_audio() {
                let _ = current_audio.pause();
            }

            channel.queue.lock().unwrap().clear();

            if let Some(mut current_audio) = channel.current_audio() {
                let _ = current_audio.stop();
            }

            Ok(())
        } else {
            Err(PyRuntimeError::new_err("Channel not found"))
        }
    }

    pub fn channel(&self, name: String) -> PyResult<Option<AudioChannel>> {
        let channels = self.channels.lock().unwrap();
        match channels.get(&name) {
            Some(channel) => Ok(Some(channel.clone())),
            None => Ok(None),
        }
    }


    pub fn start_all(&self) {
        let mut channels = self.channels.lock().unwrap();
        for (_, channel) in channels.iter_mut() {
            channel.set_auto_consume(true);
        }
    }

    pub fn stop_all(&self) {
        let mut channels = self.channels.lock().unwrap();
        for (_, channel) in channels.iter_mut() {
            channel.set_auto_consume(false);
        }
    }
}

#[pymodule]
pub fn channelmanager(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ChannelManager>()?;
    Ok(())
}
