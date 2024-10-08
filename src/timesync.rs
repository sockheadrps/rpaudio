use std::fmt;

use pyo3::{prelude::*, types::IntoPyDict};
use serde::Serialize;

use crate::utils::json_to_py;

#[derive(Clone, Debug, Copy, PartialEq, Serialize)]
#[pyclass]
pub struct FadeIn {
    #[pyo3(get, set)]
    pub duration: Option<f32>,
    #[pyo3(get, set)]
    pub start_val: Option<f32>,
    #[pyo3(get, set)]
    pub end_val: Option<f32>,
    #[pyo3(get, set)]
    pub apply_after: Option<f32>,
}

pub trait ExtractableEffect {
    fn extract_action<'py>(&'py self) -> PyResult<ActionType>;
}

impl ExtractableEffect for Bound<'_, PyAny> {
    fn extract_action<'py>(&'py self) -> PyResult<ActionType> {
        self.extract::<FadeIn>().map(ActionType::FadeIn)
            .or_else(|_| self.extract::<FadeOut>().map(ActionType::FadeOut))
            .or_else(|_| self.extract::<ChangeSpeed>().map(ActionType::ChangeSpeed))
    }
}

#[pymethods]
impl FadeIn {
    #[new]
    #[pyo3(signature = (duration=None, start_val=None, end_val=None, apply_after=None))]
    pub fn new(
        duration: Option<f32>,
        start_val: Option<f32>,
        end_val: Option<f32>,
        apply_after: Option<f32>,
    ) -> PyResult<Self> {
        Ok(FadeIn {
            duration,
            start_val,
            end_val,
            apply_after,
        })
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Serialize)]
#[pyclass]
pub struct FadeOut {
    #[pyo3(get, set)]
    pub duration: Option<f32>,
    #[pyo3(get, set)]
    pub start_val: Option<f32>,
    #[pyo3(get, set)]
    pub end_val: Option<f32>,
    #[pyo3(get, set)]
    pub apply_after: Option<f32>,
}

#[pymethods]
impl FadeOut {
    #[new]
    #[pyo3(signature = (duration=None, start_val=None, end_val=None, apply_after=None))]
    pub fn new(
        duration: Option<f32>,
        start_val: Option<f32>,
        end_val: Option<f32>,
        apply_after: Option<f32>,
    ) -> PyResult<Self> {
        Ok(FadeOut {
            duration,
            start_val,
            end_val,
            apply_after,
        })
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Serialize)]
#[pyclass]
pub struct ChangeSpeed {
    #[pyo3(get, set)]
    pub duration: Option<f32>,
    #[pyo3(get, set)]
    pub start_val: Option<f32>,
    #[pyo3(get, set)]
    pub end_val: Option<f32>,
    #[pyo3(get, set)]
    pub apply_after: Option<f32>,
}

#[pymethods]
impl ChangeSpeed {
    #[new]
    #[pyo3(signature = (duration=None, start_val=None, end_val=None, apply_after=None))]
    pub fn new(
        duration: Option<f32>,
        start_val: Option<f32>,
        end_val: Option<f32>,
        apply_after: Option<f32>,
    ) -> PyResult<Self> {
        Ok(ChangeSpeed {
            duration,
            start_val,
            end_val,
            apply_after,
        })
    }
}

impl fmt::Display for FadeIn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FadeIn {{ duration: {:?}, start_val: {:?}, end_val: {:?} apply_after: {:?} }}", 
            self.duration, self.start_val, self.end_val, self.apply_after)
    }
}

impl IntoPyDict for FadeIn {
    fn into_py_dict_bound(self, py: Python<'_>) -> Bound<'_, pyo3::types::PyDict> {
        let value = serde_json::to_value(self).unwrap();
        json_to_py(py, &value).extract().unwrap()
    }
}

impl fmt::Display for FadeOut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FadeOut {{ duration: {:?}, start_val: {:?}, end_val: {:?} apply_after: {:?} }}", 
            self.duration, self.start_val, self.end_val, self.apply_after)
    }
}

impl IntoPyDict for FadeOut {
    fn into_py_dict_bound(self, py: Python<'_>) -> Bound<'_, pyo3::types::PyDict> {
        let value = serde_json::to_value(self).unwrap();
        json_to_py(py, &value).extract().unwrap()
    }
}

impl fmt::Display for ChangeSpeed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChangeSpeed {{ duration: {:?}, start_val: {:?}, end_val: {:?} apply_after: {:?} }}", 
            self.duration, self.start_val, self.end_val, self.apply_after)
    }
}

impl IntoPyDict for ChangeSpeed {
    fn into_py_dict_bound(self, py: Python<'_>) -> Bound<'_, pyo3::types::PyDict> {
        let value = serde_json::to_value(self).unwrap();
        json_to_py(py, &value).extract().unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[allow(non_upper_case_globals)]
#[pyclass]
pub enum ActionType {
    FadeIn(FadeIn),
    FadeOut(FadeOut),
    ChangeSpeed(ChangeSpeed),
}

#[derive(PartialEq, Clone, Debug)]
pub struct EffectSync {
    pub start_position: f32,
    duration: f32,
    start_val: f32,
    end_val: f32,
    completion_pos: f32,
    current_position: f32,
    apply_after: Option<f32>,
    pub action: ActionType,
}

impl fmt::Display for EffectSync {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EffectSync {{ start_position: {}, duration: {}, start_val: {}, end_val: {}, completion_pos: {}, current_position: {}, apply_after: {:?}, action: {} }}", 
               self.start_position, 
               self.duration, 
               self.start_val, 
               self.end_val, 
               self.completion_pos, 
               self.current_position, 
               self.apply_after, 
               self.action)
    }
}

pub enum EffectResult {
    Value(f32),
    Ignored,
    Completed(f32),
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::FadeIn(_) => write!(f, "FadeIn"),

            ActionType::FadeOut(_) => write!(f, "FadeOut"),

            ActionType::ChangeSpeed(_) => write!(f, "ChangeSpeed"),
        }
    }
}

impl EffectSync {
    pub fn new(action: ActionType, current_position: f32, sink_duration: Option<f32>) -> Self {
        let (start_position, duration, start_val, end_val, apply_after) = match action {
            ActionType::FadeIn(fade_in) => {
                let duration = fade_in.duration.unwrap_or(2.0);
                let start_val = fade_in.start_val.unwrap_or(0.0);
                let end_val = fade_in.end_val.unwrap_or(1.0);
                let start_position = if fade_in.apply_after.is_none() {
                    current_position
                } else {
                    current_position + fade_in.apply_after.unwrap()
                };

                (
                    start_position,
                    duration,
                    start_val,
                    end_val,
                    fade_in.apply_after,
                )
            }
            ActionType::FadeOut(fade_out) => {
                let duration = fade_out.duration.unwrap_or(2.0);
                let start_val = fade_out.start_val.unwrap_or(1.0);
                let end_val = fade_out.end_val.unwrap_or(0.0);
                let start_position = if fade_out.apply_after.is_none() {
                    sink_duration.unwrap_or(current_position) - duration
                } else {
                    current_position + fade_out.apply_after.unwrap()
                };

                (
                    start_position,
                    duration,
                    start_val,
                    end_val,
                    fade_out.apply_after,
                )
            }
            ActionType::ChangeSpeed(change_speed) => {
                let duration = change_speed.duration.unwrap_or(0.0);
                let start_val = change_speed.start_val.unwrap_or(1.0);
                let end_val = change_speed.end_val.unwrap_or(1.5);
                let start_position = if change_speed.apply_after.is_none() {
                    current_position
                } else {
                    current_position + change_speed.apply_after.unwrap()
                };

                (
                    start_position,
                    duration,
                    start_val,
                    end_val,
                    change_speed.apply_after,
                )
            }
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
        if current_position <= self.start_position {
            return EffectResult::Ignored;
        } else {
            if current_position >= self.completion_pos {
                let rounded_end_val = format!("{:.2}", self.end_val).parse::<f32>().unwrap_or(self.end_val);
                return EffectResult::Completed(rounded_end_val);
            } else {
                let progress = (current_position - self.start_position)
                    / (self.completion_pos - self.start_position);
                let rounded_progresss = format!("{:.2}", progress).parse::<f32>().unwrap_or(progress);
                let progress = rounded_progresss.clamp(0.0, 1.0);
                let set_val = self.start_val + (self.end_val - self.start_val) * progress;
                return EffectResult::Value(set_val);
            }
        }
    }
}
