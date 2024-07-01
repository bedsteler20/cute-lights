import openrgb
import openrgb.orgb
from lightkit.core import Light, LightDiscoverer


class OpenRgbLight(Light):
    _inner: openrgb.orgb.Device

    def __init__(self, inner: openrgb.orgb.Device):
        self._inner = inner

    async def set_color(
        self, h: int, s: int, l: int, transmission: int | None = None
    ) -> None:
        await super().set_color(h, s, l, transmission)
        self._inner.set_color(openrgb.utils.RGBColor.fromHSV(h, s, l))

    async def set_brightness(
        self, brightness: int, transmission: int | None = None
    ) -> None:
        pass


class OpenRgbDiscoverer(LightDiscoverer):
    def __init__(self):
        super().__init__()

    async def discover(self):
        client = openrgb.OpenRGBClient()
        client.clear()

        return [OpenRgbLight(device) for device in client.devices]
