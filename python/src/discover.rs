use pyo3::prelude::*;

use crate::{light::Light, utils::synchronize};

#[pyfunction]
pub fn discover_lights() -> PyResult<Vec<Light>> {
    let lights = synchronize(cute_lights::discover_lights());
    let mut py_lights = Vec::new();

    for light in lights {
        py_lights.push(Light::new(light));
    }

    Ok(py_lights)
}
