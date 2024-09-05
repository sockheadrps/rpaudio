use crate::AudioSink;
use pyo3::prelude::*;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct TimeSyncEffect {
    pub action: Arc<dyn Fn(&AudioSink) + Send + Sync>,
    pub applied: bool,
}

impl TimeSyncEffect {
    pub fn new<F>(action: F) -> Self
    where
        F: Fn(&AudioSink) + Send + Sync + 'static,
    {
        Self {
            action: Arc::new(action),
            applied: false,
        }
    }

    pub fn apply(&mut self, audiosink: &AudioSink) {
        if !self.applied {
            (self.action)(audiosink);
            self.applied = true;
        }
    }
}
#[derive(Clone)]
#[derive(Debug)]
pub struct EffectActionData {
    pub action_type: ActionType,
}

impl<'a> FromPyObject<'a> for EffectActionData {
    fn extract_bound(ob: &Bound<pyo3::PyAny>) -> PyResult<Self> {
        let _ = ob;
        Ok(EffectActionData {
            action_type: ActionType::FadeIn(FadeIn { duration: 0.0, start_vol: 0.0, end_vol: 0.0 }), 
        })
    }

    fn type_input() -> pyo3::inspect::types::TypeInfo {
        pyo3::inspect::types::TypeInfo::Any
    }
}


#[derive(Clone)]
#[derive(Debug)]
#[pyclass]
pub struct FadeIn {
    #[pyo3(get, set)]
    pub duration: f32,
    #[pyo3(get, set)]
    pub start_vol: f32,
    #[pyo3(get, set)]
    pub end_vol: f32,
}

const DEFAULT_DURATION: f32 = 5.0;
const DEFAULT_START_VOL: f32 = 0.1;
const DEFAULT_END_VOL: f32 = 1.0;

#[pymethods]
impl FadeIn {
    #[new]
    #[pyo3(signature = (duration=DEFAULT_DURATION, start_vol=DEFAULT_START_VOL, end_vol=DEFAULT_END_VOL))]
    pub fn new(duration: Option<f32>, start_vol: Option<f32>, end_vol: Option<f32>) -> PyResult<Self> {
        Ok(FadeIn {
            duration: duration.unwrap_or(DEFAULT_DURATION),
            start_vol: start_vol.unwrap_or(DEFAULT_START_VOL),
            end_vol: end_vol.unwrap_or(DEFAULT_END_VOL),
        })
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[pyclass]
pub struct FadeOut {
    #[pyo3(get, set)]
    pub duration: f32,
}


#[pymethods]
impl FadeOut {
    #[new]
    pub fn new(duration: f32) -> Self {
        FadeOut { duration }
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[pyclass]
pub struct ChangeSpeed {
    #[pyo3(get, set)]
    pub speed: f32,
}

#[pymethods]
impl ChangeSpeed {
    #[new]
    pub fn new(speed: f32) -> Self {
        ChangeSpeed { speed }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[pyclass]
pub enum ActionType {
    FadeIn(FadeIn),
    FadeOut(FadeOut),
    ChangeSpeed(ChangeSpeed),
}

#[pymethods]
impl ActionType {
    #[staticmethod]
    pub fn fade_in() -> Self {
        ActionType::FadeIn(FadeIn {
            duration: 12.0,
            start_vol: 0.0,
            end_vol: 1.0,
        })
    }

    #[staticmethod]
    pub fn fade_out(duration: f32) -> Self {
        ActionType::FadeOut(FadeOut { duration })
    }

    #[staticmethod]
    pub fn nchange_speed(speed: f32) -> Self {
        ActionType::ChangeSpeed(ChangeSpeed { speed })
    }
}

#[derive(Clone)]
pub struct TimeSyncEffects {
    pub effects: Vec<TimeSyncEffect>,
}

impl TimeSyncEffects {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn add_effect(&mut self, effect: TimeSyncEffect) {
        self.effects.push(effect);
    }

    pub fn apply_due_effects(
        &mut self,
        audiosink: &AudioSink,
        current_time: Duration,
        start_time: Duration,
        _remaining_time: Duration,
    ) {
        for effect in &mut self.effects {
            if current_time >= start_time && !effect.applied {
                effect.apply(audiosink);
            }
        }
    }

    pub fn into_iter(self) -> std::vec::IntoIter<TimeSyncEffect> {
        self.effects.into_iter()
    }
}


