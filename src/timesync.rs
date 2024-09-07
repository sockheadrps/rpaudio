use pyo3::prelude::*;


#[derive(Clone, Debug)]
#[pyclass]
pub struct FadeIn {
    #[pyo3(get, set)]
    pub duration: f32,
    #[pyo3(get, set)]
    pub start_vol: f32,
    #[pyo3(get, set)]
    pub end_vol: f32,
    #[pyo3(get, set)]
    pub apply_after: Option<f32>, 
}

const DEFAULT_FADE_IN_DURATION: f32 = 5.0;
const DEFAULT_FADE_IN_START_VOL: f32 = 0.1;
const DEFAULT_FADE_IN_END_VOL: f32 = 1.0;

#[pymethods]
impl FadeIn {
    #[new]
    #[pyo3(signature = (
        duration=DEFAULT_FADE_IN_DURATION,
        start_vol=DEFAULT_FADE_IN_START_VOL,
        end_vol=DEFAULT_FADE_IN_END_VOL,
        apply_after=None
    ))]
    pub fn new(
        duration: Option<f32>,
        start_vol: Option<f32>,
        end_vol: Option<f32>,
        apply_after: Option<f32>
    ) -> PyResult<Self> {
        Ok(FadeIn {
            duration: duration.unwrap_or(DEFAULT_FADE_IN_DURATION),
            start_vol: start_vol.unwrap_or(DEFAULT_FADE_IN_START_VOL),
            end_vol: end_vol.unwrap_or(DEFAULT_FADE_IN_END_VOL),
            apply_after,
        })
    }
}

// Constants for default values
const DEFAULT_FADE_OUT_DURATION: f32 = 5.0;
const DEFAULT_FADE_OUT_START_VOL: f32 = 0.1;
const DEFAULT_FADE_OUT_END_VOL: f32 = 1.0;

#[derive(Clone, Debug)]
#[pyclass]
pub struct FadeOut {
    #[pyo3(get, set)]
    pub duration: f32,
    #[pyo3(get, set)]
    pub start_vol: f32,
    #[pyo3(get, set)]
    pub end_vol: f32,
    #[pyo3(get, set)]
    pub apply_after: Option<f32>,
}

#[pymethods]
impl FadeOut {
    #[new]
    #[pyo3(signature = (
        duration=DEFAULT_FADE_OUT_DURATION,
        start_vol=DEFAULT_FADE_OUT_START_VOL,
        end_vol=DEFAULT_FADE_OUT_END_VOL,
        apply_after=None
    ))]
    pub fn new(
        duration: Option<f32>,
        start_vol: Option<f32>,
        end_vol: Option<f32>,
        apply_after: Option<f32>
    ) -> PyResult<Self> {
        Ok(FadeOut {
            duration: duration.unwrap_or(DEFAULT_FADE_OUT_DURATION),
            start_vol: start_vol.unwrap_or(DEFAULT_FADE_OUT_START_VOL),
            end_vol: end_vol.unwrap_or(DEFAULT_FADE_OUT_END_VOL),
            apply_after,
        })
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

