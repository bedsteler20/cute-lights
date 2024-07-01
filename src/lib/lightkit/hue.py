import colorsys
from lightkit.core import Light, LightDiscoverer
import phue


class HueLight(Light):
    _inner: phue.Light
    def __init__(self, inner: phue.Light):
        self._inner = inner
    

    async def set_color(self, h: int, s: int, l: int, transmission: int | None = None) -> None:
        await super().set_color(h, s, l, transmission)
        self._inner.transitiontime = transmission
        self._inner.hue = int(h / 360 * 65535)
        self._inner.saturation = int(s / 100 * 254)
        self._inner.brightness = int(l / 100 * 254)

    async def set_brightness(self, brightness: int, transmission: int | None = None) -> None:
        self._inner.transitiontime = transmission
        self._inner.brightness = brightness

    async def on(self) -> None:
        self._inner.on = True

    async def off(self) -> None:
        self._inner.on = False


class HueDiscoverer(LightDiscoverer):
    _bridge_ip: str

    def __init__(self, bridge_ip: str):
        self._bridge_ip = bridge_ip

    async def discover(self) -> list[HueLight]:
        bridge = phue.Bridge(ip=self._bridge_ip)
        bridge.connect()
        data = bridge.get_light_objects("id")
        lights = []
        for light in data.values():
            lights.append(HueLight(light))
        return lights
