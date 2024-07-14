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
