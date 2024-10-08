use pyo3::{Bound, IntoPy, PyAny, Python, ToPyObject};
use pyo3::types::{PyAnyMethods, PyDict, PyList};

pub fn json_to_py<'py>(py: Python<'py>, value: &serde_json::Value) -> Bound<'py, PyAny> {
    match value {
        serde_json::Value::Object(map) => {
            let dict = PyDict::new_bound(py);
            for (k, v) in map {
                dict.set_item(k, json_to_py(py, v)).unwrap();
            }
            dict.to_object(py)
        }
        serde_json::Value::Array(arr) => {
            let list = PyList::new_bound(py, arr.iter().map(|v| json_to_py(py, v)));
            list.to_object(py)
        }
        serde_json::Value::String(s) => s.into_py(py),
        serde_json::Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                i.to_object(py)
            } else if let Some(f) = num.as_f64() {
                f.to_object(py)
            } else {
                py.None()
            }
        }
        serde_json::Value::Bool(b) => b.into_py(py),
        serde_json::Value::Null => py.None()
    }.into_bound(py)
}
