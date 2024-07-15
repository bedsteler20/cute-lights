# Cute Lights

Cute Lights is a simple library for controlling various types of smart lights threw a unified api. It is designed to be simple to use and easy to extend. It can be used as a rust crate or a c shared library. With bindings for dotnet and python.

## Supported Lights

-   [x] Philips Hue
-   [x] Tp-Link Kasa
-   [x] Govee (Must have lan control enabled)
-   [ ] OpenRgb

## Usage

```cs
using CuteLights.Sdk;

var lights = LightDiscoverer.Discover();

Console.WriteLine("Discovered lights:");
foreach (var light in lights) {
    Console.WriteLine($"  {light.Name} ({light.Id})");
}

var frame = new Frame();
var on = true;
while (true) {
    frame.SetOnAll(lights, on);
    on = !on;
    await frame.Run();
    frame.Clear();
    await Task.Delay(1000);
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

- [Dotnet](https://www.nuget.org/packages/CuteLight.Sdk/)
- [Rust](https://crates.io/crates/cute_lights)
