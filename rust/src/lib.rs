mod config;
mod discover;
mod integrations;
mod utils;

pub type CuteResult<T> = std::result::Result<T, Box<dyn std::error::Error>>; // :3

pub use discover::discover_lights;
pub use integrations::Light;
pub use utils::future::FutureBatch;
