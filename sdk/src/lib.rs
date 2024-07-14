mod config;
mod discover;
mod integrations;
mod utils;
mod frame;

#[cfg(feature = "c_api")]
mod c_abi;
pub type CuteResult<T> = std::result::Result<T, Box<dyn std::error::Error>>; // :3


pub use integrations::Light;
pub use discover::discover_lights;
pub use frame::Frame;