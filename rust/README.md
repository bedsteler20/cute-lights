# Cute Lights

Cute Lights is a simple library for controlling various types of smart lights threw a unified api. It is designed to be simple to use and easy to extend. It can be used as a rust crate or a c shared library. With bindings for dotnet and python.

## Supported Lights

-   [x] Philips Hue
-   [x] Tp-Link Kasa
-   [x] Govee (Must have lan control enabled)
-   [ ] OpenRgb

## Usage

```rust
use cute_lights::{discover_lights, CuteResult};
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> CuteResult<()> {
    let mut lights = discover_lights().await;

    loop {
       for light in lights.iter_mut() {
            light.set_on(true).await?;
            light.set_color(255, 0, 0).await?;
            light.set_brightness(100).await?;
            sleep(Duration::from_secs(1));
        }
    }
}

```

## Configuration

The configuration file is located at `~/.config/cute_lights/lights.toml`. It is used to store the ip addresses and api keys for lights. The file should look like this:

```toml
[kasa]
enabled = true
addresses = [
    "192.168.86.xx",
    "192.168.86.xx",
]

[govee]
enabled = true
addresses = ["192.168.86.xx"]

[hue]
enabled = true
bridge_ip = "192.168.86.xx"
username = "<Your Hue Api Key>"
```

## Language Bindings

-   [x] Rust
