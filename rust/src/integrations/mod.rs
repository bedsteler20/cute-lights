use crate::config::CuteLightsConfig;

pub mod govee;
pub mod hue;
pub mod kasa;

// ANCHOR - ImplementationDiscoverer
#[async_trait::async_trait]
pub trait Integration
where
    Self: std::marker::Send + std::marker::Sync,
    Self: Sized,
{
    fn name() -> String;
    async fn discover(config: &'static CuteLightsConfig) -> anyhow::Result<Vec<Box<dyn Light>>>;

    fn preflight(config: &CuteLightsConfig) -> bool;
}

// ANCHOR - ImplementationLight
#[async_trait::async_trait]
pub trait Light
where
    Self: std::marker::Send + std::marker::Sync,
{
    async fn set_on(&mut self, on: bool) -> anyhow::Result<()>;
    async fn set_color(&mut self, r: u8, g: u8, b: u8) -> anyhow::Result<()>;
    async fn set_brightness(&mut self, brightness: u8) -> anyhow::Result<()>;
    fn is_on(&self) -> bool;
    fn name(&self) -> String;
    fn supports_color(&self) -> bool;

    fn red(&self) -> u8;
    fn green(&self) -> u8;
    fn blue(&self) -> u8;

    fn brightness(&self) -> u8;
    fn id(&self) -> String;
}

impl std::fmt::Debug for dyn Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Light: {} ({})", self.name(), self.id())
    }
}

impl std::fmt::Display for dyn Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Light: {} ({})", self.name(), self.id())
    }
}

impl std::hash::Hash for dyn Light {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for dyn Light {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
