import asyncio
import colorsys
from lightkit.core import Light, LightDiscoverer
import govee_led_wez as govee


class GoveeLight(Light):
    _inner: govee.GoveeDevice
    _controller: govee.GoveeController

    def __init__(self, inner: govee.GoveeDevice, controller: govee.GoveeController):
        self._inner = inner
        self._controller = controller

    async def set_color(
        self, h: int, s: int, l: int, transmission: int | None = None
    ) -> None:
        await super().set_color(h, s, l, transmission)
        await self._controller.set_power_state(self._inner, True)
        r, g, b = colorsys.hls_to_rgb(h / 360, l / 100, l / 100)
        await self._controller.set_color(
            self._inner, govee.GoveeColor(r * 255, g * 255, b * 255)
        )

    async def set_brightness(
        self, brightness: int, transmission: int | None = None
    ) -> None:
        await self._controller.set_brightness(self._inner, brightness)

    async def on(self) -> None:
        await self._controller.set_power_state(self._inner, True)

    async def off(self) -> None:
        await self._controller.set_power_state(self._inner, False)


class GoveeDiscoverer(LightDiscoverer):
    _ips: list[str]

    def __init__(self, ips: list[str]):
        self._ips = ips

    async def discover(self) -> list[GoveeLight]:
        controller = govee.GoveeController()
        found_ips: dict[str, govee.GoveeDevice] = {}

        def device_changed(device: govee.GoveeDevice):
            found_ips[device.lan_definition.ip_addr] = device

        controller.set_device_change_callback(device_changed)
        controller.start_lan_poller()
        await asyncio.sleep(2)
        return [GoveeLight(device, controller) for device in found_ips.values()]
