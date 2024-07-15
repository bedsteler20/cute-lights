use std::hash::{DefaultHasher, Hash, Hasher};

use pyo3::prelude::*;

use crate::utils::synchronize;

#[pyclass]
pub struct Light {
    pub inner: Box<dyn cute_lights::Light>,
}

impl Light {
    pub fn new(inner: Box<dyn cute_lights::Light>) -> Self {
        Light { inner }
    }
}

#[pymethods]
impl Light {
    fn set_on(&mut self, on: bool) -> PyResult<()> {
        match synchronize(self.inner.set_on(on)) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    fn set_color(&mut self, r: u8, g: u8, b: u8) -> PyResult<()> {
        match synchronize(self.inner.set_color(r, g, b)) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    fn set_brightness(&mut self, brightness: u8) -> PyResult<()> {
        match synchronize(self.inner.set_brightness(brightness)) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                e.to_string(),
            )),
        }
    }

    #[getter]
    fn is_on(&self) -> bool {
        self.inner.is_on()
    }

    #[getter]
    fn supports_color(&self) -> bool {
        self.inner.supports_color()
    }

    #[getter]
    fn red(&self) -> u8 {
        self.inner.red()
    }

    #[getter]
    fn green(&self) -> u8 {
        self.inner.green()
    }

    #[getter]
    fn blue(&self) -> u8 {
        self.inner.blue()
    }

    #[getter]
    fn brightness(&self) -> u8 {
        self.inner.brightness()
    }

    #[getter]
    pub fn id(&self) -> String {
        self.inner.id()
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name()
    }

    pub fn __repr__(&self) -> String {
        format!("Light: {} ({})", self.inner.name(), self.inner.id())
    }

    pub fn __eq__(&self, other: &Self) -> bool {
        self.inner.id() == other.inner.id()
    }

    pub fn __ne__(&self, other: &Self) -> bool {
        self.inner.id() != other.inner.id()
    }

    pub fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.inner.id().hash(&mut hasher);
        hasher.finish()
    }
}
