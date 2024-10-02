use pyo3::{create_exception, PyErr};
use pyo3::exceptions::PyException;
use std::fmt;

create_exception!(rpaudio, EffectConflictException, PyException);

impl fmt::Display for EffectConflictException {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Value cannot be changed while effects are actively being applied.")
  }
}

impl EffectConflictException {
  pub fn with_context(context: &str) -> PyErr {
      let message = format!("Cannot change property {} on AudioSink while scheduled effects are actively being applied.", context);
      EffectConflictException::new_err(message)
  }
}