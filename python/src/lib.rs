use pyo3::prelude::*;

mod discover;
mod frame;
mod light;
mod utils;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn cute_light(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(discover::discover_lights, m)?)?;
    m.add_class::<frame::Frame>()?;
    m.add_class::<light::Light>()?;

    Ok(())
}
