import asyncio
import colorsys
import kasa.modulemapping
from lightkit.core import Light, LightDiscoverer
import kasa


class KasaLight(Light):
    _inner: kasa.Light

    def __init__(self, inner: kasa.Light):
        self._inner = inner

    async def set_color(
        self, h: int, s: int, l: int, transmission: int | None = None
    ) -> None:
        await super().set_color(h, s, l, transmission)
        await self._inner.set_hsv(h, s, l, transition=transmission)

    async def set_brightness(
        self, brightness: int, transmission: int | None = None
    ) -> None:
        await self._inner.set_brightness(brightness, transition=transmission)

    async def on(self) -> None:
        await self._inner.set_state(kasa.LightState(True))
    
    async def off(self) -> None:
        await self._inner.set_state(kasa.LightState(False))


class KasaDiscoverer(LightDiscoverer):
    _ips: list[str]

    def __init__(self, ips: list[str]):
        self._ips = ips

    async def _get_device(self, ip: str) -> KasaLight:
        dev = await kasa.Discover.discover_single(ip)
        await dev.update()
        light = dev.modules[kasa.Module.Light]
        return KasaLight(light)

    async def discover(self) -> list[KasaLight]:
        return await asyncio.gather(*[self._get_device(ip) for ip in self._ips])
