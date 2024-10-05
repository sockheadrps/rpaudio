use audiosink::AudioSink;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use timesync::{ActionType, ChangeSpeed, FadeIn, FadeOut};
mod audioqueue;
mod exceptions;
mod exmetadata;
mod audiosink;
mod mixer;
mod timesync;
use crate::exceptions::EffectConflictException;
pub use exmetadata::MetaData;


#[pymodule]
#[pyo3(name="effects")]
fn effects(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FadeIn>()?;
    m.add_class::<FadeOut>()?;
    m.add_class::<ChangeSpeed>()?;
    Ok(())
}

#[pymodule]
#[pyo3(name="rpaudio_exceptions")]
fn rpaudio_exceptions(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("EffectConflictException", py.get_type_bound::<EffectConflictException>())?;
    Ok(())
}


#[pymodule]
fn rpaudio(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AudioSink>()?;
    m.add_class::<mixer::ChannelManager>()?;
    m.add_class::<audioqueue::AudioChannel>()?;
    // Creating the effects submodule
    let effects_module = PyModule::new_bound(py, "effects")?;
    effects(effects_module.py(), &effects_module)?; // Add classes to effects module
    m.add_submodule(&effects_module)?; // Add effects module to rpaudio

    // Exception Submodule
    let rpaudio_exceptions_module = PyModule::new_bound(py, "rpaudio_exceptions")?;
    rpaudio_exceptions(py, &rpaudio_exceptions_module)?;
    m.add_submodule(&rpaudio_exceptions_module)?;

    // m.add_wrapped(wrap_pymodule!(rpaudio_exceptions))?;
    m.add_class::<ActionType>()?;
    // m.add_function(wrap_pyfunction!(get_audio_info, m)?)?;
    Ok(())
}
