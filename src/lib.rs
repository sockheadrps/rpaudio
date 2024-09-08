use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use timesync::{ActionType, ChangeSpeed, FadeIn, FadeOut};
mod audioqueue;
mod exmetadata;
mod mixer;
mod timesync;
pub use exmetadata::MetaData;
unsafe impl Send for AudioSink {}
use pyo3::types::PyModule;

#[derive(Clone)]
#[pyclass]
pub struct AudioSink {
    is_playing: Arc<Mutex<bool>>,
    callback: Arc<Mutex<Option<Py<PyAny>>>>,
    cancel_callback: Arc<Mutex<bool>>,
    sink: Option<Arc<Mutex<Sink>>>,
    stream: Option<Arc<Mutex<OutputStream>>>,
    pub metadata: MetaData,
    volume: Arc<Mutex<f32>>,
    start_time: Arc<Mutex<Option<Instant>>>,
    speed: Arc<Mutex<f32>>,
    position: Arc<Mutex<Duration>>,
    time_remaining: Arc<Mutex<Option<f32>>>,
}

#[pymethods]
impl AudioSink {
    #[new]
    #[pyo3(signature = (callback=None))]
    pub fn new(callback: Option<Py<PyAny>>) -> Self {
        AudioSink {
            is_playing: Arc::new(Mutex::new(false)),
            callback: Arc::new(Mutex::new(callback)),
            cancel_callback: Arc::new(Mutex::new(false)),
            sink: None,
            stream: None,
            metadata: MetaData::default(),
            volume: Arc::new(Mutex::new(1.0)),
            start_time: Arc::new(Mutex::new(None)),
            speed: Arc::new(Mutex::new(1.0)),
            position: Mutex::new(Duration::from_secs(0)).into(),
            time_remaining: Mutex::new(None).into(),
        }
    }

    #[getter]
    pub fn metadata(&self, py: Python) -> PyResult<Py<PyDict>> {
        let mut dict = HashMap::new();
        dict.insert("title", self.metadata.title.clone());
        dict.insert("artist", self.metadata.artist.clone());
        dict.insert("date", self.metadata.date.clone());
        dict.insert("year", self.metadata.year.clone());
        dict.insert("album_title", self.metadata.album_title.clone());
        dict.insert("album_artist", self.metadata.album_artist.clone());
        dict.insert("track_number", self.metadata.track_number.clone());
        dict.insert("total_tracks", self.metadata.total_tracks.clone());
        dict.insert("disc_number", self.metadata.disc_number.clone());
        dict.insert("total_discs", self.metadata.total_discs.clone());
        dict.insert("genre", self.metadata.genre.clone());
        dict.insert("composer", self.metadata.composer.clone());
        dict.insert("comment", self.metadata.comment.clone());
        dict.insert(
            "sample_rate",
            self.metadata.sample_rate.map(|rate| rate.to_string()),
        );
        dict.insert("channels", self.metadata.channels.clone());
        dict.insert(
            "duration",
            self.metadata.duration.map(|duration| duration.to_string()),
        );

        let py_dict = PyDict::new_bound(py);

        for (key, value) in dict {
            py_dict.set_item(key, value)?;
        }

        Ok(py_dict.into())
    }

    #[getter]
    pub fn is_playing(&self) -> bool {
        *self.is_playing.lock().unwrap()
    }

    pub fn load_audio(&mut self, file_path: String) -> PyResult<Self> {
        if self.sink.is_some() {
            return Ok(self.clone());
        }

        let metadata = match exmetadata::extract_metadata(file_path.as_ref()) {
            Ok(meta) => meta,
            Err(_e) => {
                return Err(PyRuntimeError::new_err("Failed to extract metadata"));
            }
        };
        self.metadata = metadata;

        let (new_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink_result = Sink::try_new(&stream_handle);
        let sink = match sink_result {
            Ok(s) => Arc::new(Mutex::new(s)),
            Err(e) => {
                return Err(PyRuntimeError::new_err(format!(
                    "Failed to create sink: {}",
                    e
                )))
            }
        };

        let file_path_clone = file_path.clone();
        let file = File::open(file_path_clone).unwrap();
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| format!("Failed to decode audio file: {}", e))
            .unwrap();

        let total_duration = match source.total_duration() {
            Some(duration) => duration.as_secs_f32(),
            None => 0.0,
        };
        sink.lock().unwrap().append(source);

        self.stream = Some(Arc::new(Mutex::new(new_stream)));
        self.sink = Some(sink.clone());

        if let Some(sink) = &self.sink {
            (*sink.lock().unwrap()).pause();
        }

        let is_playing = self.is_playing.clone();
        let callback = self.callback.clone();
        let cancel_callback = self.cancel_callback.clone();
        let sink_clone = sink.clone();
        let speed = self.speed.clone();

        let mut time_remaining_guard = self.time_remaining.lock().unwrap();
        *time_remaining_guard = Some(total_duration - sink.lock().unwrap().get_pos().as_secs_f32());

        thread::spawn(move || {
            loop {
                {
                    let mut is_playing_guard = is_playing.lock().unwrap();
                    let sink = sink_clone.lock().unwrap();

                    if sink.empty() {
                        *is_playing_guard = false;
                        drop(is_playing_guard);
                        if !*cancel_callback.lock().unwrap() {
                            Self::invoke_callback(&*callback.lock().unwrap());
                        }
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

        Ok(self.clone())
    }

    pub fn play(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().play();
            *self.is_playing.lock().unwrap() = true;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to play. Load audio first.",
            ))
        }
    }

    pub fn pause(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().pause();
            *self.is_playing.lock().unwrap() = false;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to pause. Load audio first.",
            ))
        }
    }

    pub fn stop(&mut self) -> PyResult<()> {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().stop();
            *self.is_playing.lock().unwrap() = false;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to stop. Load audio first.",
            ))
        }
    }

    pub fn empty(&self) -> bool {
        if let Some(sink) = &self.sink {
            sink.lock().unwrap().empty()
        } else {
            false
        }
    }

    pub fn cancel_callback(&mut self) {
        *self.cancel_callback.lock().unwrap() = true;
    }

    pub fn set_volume(&mut self, volume: f32) -> PyResult<()> {
        if volume < 0.0 || volume > 1.0 {
            return Err(PyValueError::new_err("Volume must be between 0.0 and 1.0."));
        }

        if let Some(sink) = &self.sink {
            sink.lock().unwrap().set_volume(volume);
            *self.volume.lock().unwrap() = volume;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to set volume. Load audio first.",
            ))
        }
    }

    pub fn get_volume(&self) -> PyResult<f32> {
        if let Some(sink) = &self.sink {
            let sink = Arc::clone(sink);
            let volume = sink.lock().unwrap().volume();
            Ok(volume)
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available. Load audio first.",
            ))
        }
    }

    pub fn get_pos(&self) -> PyResult<f64> {
        if let Some(sink) = &self.sink {
            let duration = sink.lock().unwrap().get_pos();
            let position_seconds = duration.as_secs_f64();
            Ok((position_seconds * 100.0).round() / 100.0)
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available. Load audio first.",
            ))
        }
    }

    pub fn try_seek(&mut self, position: f32) -> PyResult<()> {
        if position < 0.0 {
            return Err(PyValueError::new_err("Position must be non-negative."));
        }

        if let Some(sink) = &self.sink {
            let duration = Duration::from_secs_f32(position);

            let result = sink.lock().unwrap().try_seek(duration);
            match result {
                Ok(_) => {
                    *self.position.lock().unwrap() =
                        Duration::from_secs_f64(self.get_pos().unwrap());
                    *self.start_time.lock().unwrap() = Some(Instant::now());
                    Ok(())
                }
                Err(e) => Err(PyRuntimeError::new_err(format!("Seek failed: {:?}", e))),
            }
        } else {
            Err(PyRuntimeError::new_err(
                "No audio sink available. Load audio first.",
            ))
        }
    }

    pub fn get_speed(&self) -> f32 {
        *self.speed.lock().unwrap()
    }
    #[pyo3(signature = (
        apply_after=None,
        duration=0.0,
        start_vol=0.0,
        end_vol=1.0
    ))]
    pub fn set_fade(
        &self,
        apply_after: Option<f32>,
        duration: f32,
        mut start_vol: Option<f32>,
        end_vol: f32,
    ) -> PyResult<()> {
        if start_vol == None {
            start_vol = Some(self.get_volume().unwrap());
            println!("start_vol is: {:?}", start_vol);
        }
        let start_vol = start_vol.unwrap();
        if duration < 0.0 {
            return Err(PyValueError::new_err("Duration must be non-negative."));
        }
        if start_vol < 0.0 || start_vol > 1.0 || end_vol < 0.0 || end_vol > 1.0 {
            return Err(PyValueError::new_err("Volume must be between 0.0 and 1.0."));
        }

        let fade_duration = Duration::from_secs_f32(duration);
        let volume_step = (end_vol - start_vol) / duration;

        if let Some(sink) = &self.sink {
            let sink = Arc::clone(sink);
            let remaining_time = self.get_remaining_time().unwrap_or(0.0);
            let sink_dur = self.metadata.duration.unwrap_or(0.0);

            let scheduled = Arc::new(Mutex::new(false));

            // Compute start time for fade
            let scheduled_start_time =
                remaining_time as f32 - sink_dur as f32 + apply_after.unwrap_or(0.0);

            thread::spawn({
                let sink = sink.clone();
                let scheduled = scheduled.clone();
                move || {
                    let mut scheduled_guard = scheduled.lock().unwrap();
                    if apply_after.is_some() {
                        *scheduled_guard = true;
                        let wait_until =
                            Instant::now() + Duration::from_secs_f64(scheduled_start_time as f64);
                        while Instant::now() < wait_until {
                            if sink.lock().unwrap().empty() {
                                println!("sink is empty");
                                return;
                            }
                            thread::sleep(Duration::from_millis(100));
                        }
                    }

                    let start_instant = Instant::now();
                    while start_instant.elapsed() < fade_duration {
                        let elapsed = start_instant.elapsed().as_secs_f32();
                        let current_volume = start_vol + volume_step * elapsed;
                        let clamped_volume = current_volume.clamp(0.0, 1.0);

                        {
                            let sink_lock = sink.lock().unwrap();
                            if sink_lock.empty() {
                                return;
                            }
                            sink_lock.set_volume(clamped_volume);
                        }

                        println!("volume is: {:?}", clamped_volume);
                        thread::sleep(Duration::from_millis(100));
                    }

                    {
                        let sink_lock = sink.lock().unwrap();
                        if !sink_lock.empty() {
                            sink_lock.set_volume(end_vol);
                        }
                    }
                    println!("Fade complete");
                }
            });
            println!("thread complete");

            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to set fade. Load audio first.",
            ))
        }
    }

    #[pyo3(signature = (
        apply_after=None,
        duration=0.0,
        start_speed=None,
        end_speed=1.0
    ))]
    pub fn set_speed(
        &self,
        apply_after: Option<f32>,
        duration: f32,
        mut start_speed: Option<f32>,
        end_speed: f32,
    ) -> PyResult<()> {
        // print current speed
        println!("current speed is: {:?}", self.get_speed());
        // Use the current speed if start_speed is not provided
        if start_speed == None {
            start_speed = Some(self.get_speed());
            println!("start_speed is: {:?}", self.get_speed());
        }
        let start_speed = start_speed.unwrap();

        // Validate duration and speed values
        if duration < 0.0 {
            return Err(PyValueError::new_err("Duration must be non-negative."));
        }
        if start_speed < 0.1 || start_speed > 4.0 || end_speed < 0.1 || end_speed > 4.0 {
            return Err(PyValueError::new_err("Speed must be between 0.1 and 4.0."));
        }

        let speed_duration = Duration::from_secs_f32(duration);
        let speed_step = (end_speed - start_speed) / duration;

        if let Some(sink) = &self.sink {
            let sink = Arc::clone(sink);
            let remaining_time = self.get_remaining_time().unwrap_or(0.0);
            let sink_dur = self.metadata.duration.unwrap_or(0.0);

            let scheduled = Arc::new(Mutex::new(false));

            // Compute start time for speed change
            let scheduled_start_time =
                remaining_time as f32 - sink_dur as f32 + apply_after.unwrap_or(0.0);

            // Print debug information
            println!("remaining_time is: {:?}", remaining_time);
            println!("sink_dur is: {:?}", sink_dur);
            println!("apply_after is: {:?}", apply_after.unwrap_or(0.0));
            println!("scheduled_start_time is: {:?}", scheduled_start_time);
            println!("start_speed is: {:?}", start_speed);

            thread::spawn({
                let sink = sink.clone();
                let scheduled = scheduled.clone();
                move || {
                    let mut scheduled_guard = scheduled.lock().unwrap();
                    if apply_after.is_some() {
                        *scheduled_guard = true;
                        let wait_until =
                            Instant::now() + Duration::from_secs_f64(scheduled_start_time as f64);
                        while Instant::now() < wait_until {
                            if sink.lock().unwrap().empty() {
                                return;
                            }
                            thread::sleep(Duration::from_millis(100));
                        }
                    }

                    let start_instant = Instant::now();
                    while start_instant.elapsed() < speed_duration {
                        let elapsed = start_instant.elapsed().as_secs_f32();
                        let current_speed = start_speed + speed_step * elapsed;
                        let clamped_speed = current_speed.clamp(0.1, 4.0); // Ensure speed stays in bounds

                        {
                            let mut sink_lock = sink.lock().unwrap();
                            if sink_lock.empty() {
                                return;
                            }
                            sink_lock.set_speed(clamped_speed);
                        }

                        println!("speed is: {:?}", clamped_speed);
                        thread::sleep(Duration::from_millis(100));
                    }

                    {
                        let mut sink_lock = sink.lock().unwrap();
                        if !sink_lock.empty() {
                            sink_lock.set_speed(end_speed);
                        }
                    }
                    println!("Speed change complete");
                }
            });

            Ok(())
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available to set speed. Load audio first.",
            ))
        }
    }

    pub fn get_remaining_time(&self) -> PyResult<f64> {
        if let Some(sink) = &self.sink {
            let sink_lock = sink.lock().unwrap();
            let position = sink_lock.get_pos();

            if let Some(duration) = self.metadata.duration {
                let remaining = duration - position.as_secs_f64();
                print!("remaining: {:?}", remaining);
                Ok((remaining * 100.0).round() / 100.0)
            } else {
                Err(PyRuntimeError::new_err("Audio duration is not available."))
            }
        } else {
            Err(PyRuntimeError::new_err(
                "No sink available. Load audio first.",
            ))
        }
    }

    pub fn apply_effects(&self, effect_list: Py<PyList>) -> PyResult<()> {
        let mut sched_vec = <Vec<ActionType>>::new();

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

            sched_vec.extend(rust_effect_list);

            self.execute_scheduled_effects(sched_vec);

            Ok(())
        })
    }

    pub fn execute_scheduled_effects(&self, effect_list: Vec<ActionType>) {
        for effect in effect_list.iter() {
            match effect {
                ActionType::FadeIn(fade_in) => {
                    println!(
                        "Executing FadeIn: start_vol={:?}, end_vol={}, duration={}, apply_after={:?}",
                        fade_in.start_vol, fade_in.end_vol, fade_in.duration, fade_in.apply_after
                    );
                    self.set_fade(
                        fade_in.apply_after,
                        fade_in.duration,
                        fade_in.start_vol,
                        fade_in.end_vol,
                    )
                    .unwrap();
                }
                ActionType::FadeOut(fade_out) => {
                    let total_duration = match self.time_remaining.lock().unwrap().as_ref() {
                        Some(duration) => *duration,
                        None => {
                            eprintln!("Error: Audio duration is not available.");
                            return;
                        }
                    };
                    let apply_after = match fade_out.apply_after {
                        Some(time) => time,
                        None => total_duration - fade_out.duration,
                    };
                    println!("total_duration is: {:?}", total_duration);
                    println!("apply_after is: {:?}", apply_after);

                    // Ensure apply_after is within valid range
                    if apply_after < 0.0 {
                        eprintln!(
                            "Error: Calculated apply_after is negative. Skipping FadeOut effect."
                        );
                        continue;
                    }

                    println!(
                        "Executing FadeOut: start_vol={:?}, end_vol={}, duration={}, apply_after={}",
                        fade_out.start_vol, fade_out.end_vol, fade_out.duration, apply_after
                    );

                    self.set_fade(
                        Some(apply_after),
                        fade_out.duration,
                        Some(fade_out.start_vol),
                        fade_out.end_vol,
                    )
                    .unwrap();
                }
                ActionType::ChangeSpeed(set_speed) => {
                    self.set_speed(
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
fn rpaudio(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AudioSink>()?;
    m.add_class::<MetaData>()?;
    m.add_class::<mixer::ChannelManager>()?;
    m.add_class::<audioqueue::AudioChannel>()?;
    m.add_class::<ActionType>()?;
    m.add_class::<FadeIn>()?;
    m.add_class::<FadeOut>()?;
    m.add_class::<ChangeSpeed>()?;

    Ok(())
}
