use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct AudioTimer {
    start_time: Option<Instant>,
    elapsed_time: Duration,
    running: bool, 
}

impl AudioTimer {
    pub fn new() -> Self {
        AudioTimer {
            start_time: None,
            elapsed_time: Duration::from_secs(0),
            running: false, 
        }
    }

    pub fn start(&mut self) {
        if !self.running {
            self.start_time = Some(Instant::now());
            self.running = true; 
        }
    }

    pub fn stop(&mut self) {
        if let Some(start) = self.start_time {
            self.elapsed_time += start.elapsed();
            self.start_time = None;
            self.running = false; 
        }
    }

    pub fn pause(&mut self) {
        if self.running {
            self.stop(); 
        }
    }

    pub fn resume(&mut self) {
        if !self.running {
            self.start(); 
        }
    }

    pub fn get_elapsed(&self) -> f64 {
        let total_elapsed = self.elapsed_time
            + self
                .start_time
                .map_or(Duration::new(0, 0), |start| start.elapsed());
        total_elapsed.as_secs_f64()
    }

    pub fn adjust_elapsed(&mut self, playback_speed: f64) {
        if let Some(start) = self.start_time {
            let delta_time = start.elapsed();
            let adjusted_delta = delta_time.as_secs_f64() * playback_speed;

            self.elapsed_time += Duration::from_secs_f64(adjusted_delta);

            self.start_time = Some(Instant::now());
        }
    }

    pub fn force_elapsed(&mut self, new_elapsed: Duration) {
        self.elapsed_time = new_elapsed;

        if self.running {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed_time = Duration::new(0, 0);
        self.running = false; 
    }
}
