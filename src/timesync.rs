use pyo3::prelude::*;

// Define the FadeIn struct with optional parameters
#[derive(Clone, Debug, Copy, PartialEq)]
#[pyclass]
pub struct FadeIn {
    #[pyo3(get, set)]
    pub duration: f32,
    #[pyo3(get, set)]
    pub start_val: Option<f32>,
    #[pyo3(get, set)]
    pub end_val: f32,
    #[pyo3(get, set)]
    pub apply_after: Option<f32>,
}

// Default values for FadeIn
const DEFAULT_FADE_IN_DURATION: f32 = 5.0;
const DEFAULT_FADE_IN_START_VOL: Option<f32> = None;
const DEFAULT_FADE_IN_END_VOL: f32 = 1.0;

#[pymethods]
impl FadeIn {
    #[new]
    #[pyo3(signature = (
        duration=DEFAULT_FADE_IN_DURATION,
        start_val=DEFAULT_FADE_IN_START_VOL,
        end_val=DEFAULT_FADE_IN_END_VOL,
        apply_after=None
    ))]
    pub fn new(
        duration: Option<f32>,
        start_val: Option<f32>,
        end_val: Option<f32>,
        apply_after: Option<f32>,
    ) -> PyResult<Self> {
        Ok(FadeIn {
            duration: duration.unwrap_or(DEFAULT_FADE_IN_DURATION),
            start_val,
            end_val: end_val.unwrap_or(DEFAULT_FADE_IN_END_VOL),
            apply_after,
        })
    }
}

// Define the FadeOut struct with optional parameters
const DEFAULT_FADE_OUT_DURATION: f32 = 5.0;
const DEFAULT_FADE_OUT_START_VOL: f32 = 1.0;
const DEFAULT_FADE_OUT_END_VOL: Option<f32> = None;

#[derive(Clone, Debug, Copy, PartialEq)]
#[pyclass]
pub struct FadeOut {
    #[pyo3(get, set)]
    pub duration: f32,
    #[pyo3(get, set)]
    pub start_val: f32,
    #[pyo3(get, set)]
    pub end_val: Option<f32>,
    #[pyo3(get, set)]
    pub apply_after: Option<f32>,
}

#[pymethods]
impl FadeOut {
    #[new]
    #[pyo3(signature = (
        duration=DEFAULT_FADE_OUT_DURATION,
        start_val=DEFAULT_FADE_OUT_START_VOL,
        end_val=DEFAULT_FADE_OUT_END_VOL,
        apply_after=None
    ))]
    pub fn new(
        duration: Option<f32>,
        start_val: Option<f32>,
        end_val: Option<f32>,
        apply_after: Option<f32>,
    ) -> PyResult<Self> {
        Ok(FadeOut {
            duration: duration.unwrap_or(DEFAULT_FADE_OUT_DURATION),
            start_val: start_val.unwrap_or(DEFAULT_FADE_OUT_START_VOL),
            end_val,
            apply_after,
        })
    }
}

// Define the ChangeSpeed struct with optional parameters
pub const DEFAULT_SPEED_CHANGE_DURATION: f32 = 5.0;
pub const DEFAULT_START_SPEED: f32 = 1.0;
pub const DEFAULT_END_SPEED: f32 = 1.5;

#[derive(Clone, Debug, Copy, PartialEq)]
#[pyclass]
pub struct ChangeSpeed {
    #[pyo3(get, set)]
    pub duration: f32,
    #[pyo3(get, set)]
    pub start_val: Option<f32>,
    #[pyo3(get, set)]
    pub end_val: f32,
    #[pyo3(get, set)]
    pub apply_after: Option<f32>,
}

#[pymethods]
impl ChangeSpeed {
    #[new]
    #[pyo3(signature = (
        duration=DEFAULT_SPEED_CHANGE_DURATION,
        start_val=DEFAULT_START_SPEED,
        end_val=DEFAULT_END_SPEED,
        apply_after=None
    ))]
    pub fn new(
        duration: Option<f32>,
        start_val: Option<f32>,
        end_val: Option<f32>,
        apply_after: Option<f32>,
    ) -> PyResult<Self> {
        Ok(ChangeSpeed {
            duration: duration.unwrap_or(DEFAULT_SPEED_CHANGE_DURATION),
            start_val: Some(start_val.unwrap_or(DEFAULT_START_SPEED)),
            end_val: end_val.unwrap_or(DEFAULT_END_SPEED),
            apply_after,
        })
    }
}

// Define the ActionType enum
#[derive(Clone, Debug, PartialEq)]
#[pyclass]
pub enum ActionType {
    FadeIn(FadeIn),
    FadeOut(FadeOut),
    ChangeSpeed(ChangeSpeed),
}

// Define the EffectSync struct
#[derive(PartialEq, Clone, Debug)]
pub struct EffectSync {
    start_position: f32,
    duration: f32,
    start_val: f32,
    end_val: f32,
    completion_pos: f32,
    current_position: f32,
    apply_after: Option<f32>,
    pub action: ActionType,
}

pub enum EffectResult {
    Value(f32),
    Completed,
}
impl EffectSync {
    pub fn new(action: ActionType, current_position: f32, sink_duration: Option<f32>) -> Self {
        let (start_position, duration, start_val, end_val, apply_after) = match action {
            ActionType::FadeIn(fade_in) => (
                current_position,
                fade_in.duration,
                fade_in.start_val.unwrap_or(0.0),
                fade_in.end_val,
                fade_in.apply_after,
            ),
            ActionType::FadeOut(fade_out) => (
                if fade_out.apply_after.is_none() {
                    sink_duration.unwrap_or(current_position) - fade_out.duration
                } else {
                    current_position
                },
                fade_out.duration,
                fade_out.start_val,
                fade_out.end_val.unwrap_or(0.0),
                fade_out.apply_after,
            ),
            ActionType::ChangeSpeed(change_speed) => (
                current_position,
                change_speed.duration,
                change_speed.start_val.unwrap_or(1.0),
                change_speed.end_val,
                change_speed.apply_after,
            ),
        };

        let completion_pos = start_position + duration;

        Self {
            start_position,
            duration,
            start_val,
            end_val,
            completion_pos,
            current_position,
            apply_after,
            action,
        }
    }

    pub fn update(&self, current_position: f32) -> EffectResult {
        println!("Current position: {}", current_position);
        println!("Start position: {}", self.start_position);
        println!("Completion position: {}", self.completion_pos);
        println!("Apply after: {:?}", self.apply_after);

        if let Some(apply_after) = self.apply_after {
            if current_position < apply_after {
                println!("Effect not yet applied, waiting until: {}", apply_after);
                return EffectResult::Value(self.start_val);
            }
        }

        if current_position >= self.completion_pos {
            return EffectResult::Completed;
        }

        let progress = (current_position - self.start_position) / self.duration;
        let progress = progress.clamp(0.0, 1.0);

        let set_val = self.start_val + (self.end_val - self.start_val) * progress;
        EffectResult::Value(set_val)
    }
}
