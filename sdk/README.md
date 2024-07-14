# Cute Lights

Cute Lights is a simple library for controlling various types of smart lights threw a unified api. It is designed to be simple to use and easy to extend. It can be used as a rust crate or a c shared library. With bindings for dotnet and python.

## Supported Lights

-   [x] Philips Hue
-   [x] Tp-Link Kasa
-   [x] Govee (Must have lan control enabled)
-   [ ] OpenRgb

## Usage

### Rust

```rust
use cute_lights::{
    Light,
    discover_lights,
    Frame,
    CuteResult
};
use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() -> CuteResult<()> {
    let lights = discover_lights().await?;

    let frame = Frame::new();

    loop {
        for light in &lights {
            frame.set_on(&light, true);
            frame.set_brightness(&light, 100);
            frame.set_color(&light, 0, 255, 0);
        }
        frame.run().await;
        frame.clear();
        sleep(Duration::from_secs(1));
    }
}
```
