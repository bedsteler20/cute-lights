use std::{future::Future, pin::Pin};

use pyo3::prelude::*;
use tokio::task::JoinSet;

use crate::{light::Light, utils::synchronize};

#[pyclass(unsendable)]
pub struct Frame {
    set: Vec<Pin<Box<Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>>>>,
}

#[pymethods]
impl Frame {
    #[new]
    pub fn new() -> Self {
        Frame { set: vec![] }
    }

    pub fn clear(&mut self) {
        self.set.clear();
    }

    pub fn set_on(&mut self, l: &mut Light, on: bool) {
        let ptr = l as *mut Light;
        self.set.push(Box::pin(unsafe { (*ptr).inner.set_on(on) }));
    }

    pub fn set_color(&mut self, l: &mut Light, red: u8, green: u8, blue: u8) {
        let ptr = l as *mut Light;
        self.set.push(Box::pin(unsafe { (*ptr).inner.set_color(red, green, blue) }));
    }

    pub fn set_brightness(&mut self, l: &mut Light, brightness: u8) {
        let ptr = l as *mut Light;
        self.set.push(Box::pin(unsafe { (*ptr).inner.set_brightness(brightness) }));
    }


    pub fn run(&mut self) -> PyResult<()> {
        let set = std::mem::take(&mut self.set);
        let fut = async {
            let mut join_set = JoinSet::new();
            for f in set {
                join_set.spawn(f);
            }
            while let Some(Ok(_)) = join_set.join_next().await {}
        };
        synchronize(fut);
        Ok(())
    }
}
