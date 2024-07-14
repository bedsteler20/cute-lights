use crate::{config::CuteLightsConfig, CuteResult};

pub mod govee;
pub mod hue;
pub mod kasa;

// ANCHOR - ImplementationDiscoverer
#[async_trait::async_trait]
pub trait Integration
where
    Self: std::marker::Send + std::marker::Sync,
    Self: Sized
{

    fn name() -> String;
    async fn discover(config: &'static CuteLightsConfig) -> CuteResult<Vec<Box<dyn Light>>>;

    fn preflight(config: &CuteLightsConfig) -> bool;
}

// ANCHOR - ImplementationLight
#[async_trait::async_trait]
pub trait Light
where
    Self: std::marker::Send + std::marker::Sync,
{

    async fn set_on(&mut self, on: bool) -> CuteResult<()>;
    async fn set_color(&mut self, h: i64, s: i64, b: i64) -> CuteResult<()>;
    async fn set_brightness(&mut self, brightness: i64) -> CuteResult<()>;
    fn is_on(&self) -> bool;
    fn name(&self) -> String;
    fn supports_color(&self) -> bool;

    fn hue(&self) -> i64;
    fn saturation(&self) -> i64;
    fn brightness(&self) -> i64;
    fn id(&self) -> String;
}
