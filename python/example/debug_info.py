import cute_light


lights = cute_light.discover_lights()
for light in lights:
    print(f"Light {light.name}")
    print(f"  Id: {light.id}")
    print(f"  Supports Color: {light.supports_color}")
    print(f"  Is On: {light.is_on}")
    print(f"  Red: {light.red}")
    print(f"  Green: {light.green}")
    print(f"  Blue: {light.blue}")
    print(f"  Brightness: {light.brightness}")
