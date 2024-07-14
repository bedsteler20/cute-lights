use crate::{utils::future::FutureBatch, Light};



pub struct Frame {
    inner: FutureBatch<()>
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            inner: FutureBatch::new()
        }
    }

    pub fn set_color(&mut self, light: &'static mut impl Light, h: u8, s: u8, b: u8) {
        self.inner.push(async move {
            light.set_color(h, s, b).await.unwrap();
            ()
        });
    }

    pub fn set_brightness(&mut self, light: &'static mut impl Light, brightness: u8) {
        self.inner.push(async move {
            light.set_brightness(brightness).await.unwrap();
            ()
        });
    }

    pub fn set_on(&mut self, light: &'static mut impl Light, on: bool) {
        self.inner.push(async move {
            light.set_on(on).await.unwrap();
            ()
        });
    }

    pub fn clear(&mut self) {
        self.inner = FutureBatch::new();
    }

    pub async fn run(self) {
        self.inner.run().await;
    }
}