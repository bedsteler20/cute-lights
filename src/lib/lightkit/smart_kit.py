from typing import Coroutine
import aiohttp
from lightkit.core import Light, LightDiscoverer
import pysmartthings
class SmartDiscover ( LightDiscoverer):
    def __init__(self, token: str) -> None:
        super().__init__()
        self.api_key = token

    async def discover(self) -> list[Light]:
        http = aiohttp.ClientSession() 
        self.api= pysmartthings.SmartThings(http,self.api_key)
        devices = await self.api.devices()
        new_divices = []
        for dev in devices :
            print(dev.capabilities)
            if 'switch' in dev.capabilities:
                new_divices.append(SmartLight(dev))
        return new_divices

class SmartLight (Light):
    _inner: pysmartthings.DeviceEntity
    def __init__(self, dev:pysmartthings.DeviceEntity) -> None:
        super().__init__()
        self._inner = dev
    async def on(self):
        await self._inner.command('main','switch','on')
    async def off(self) -> None:
        await self._inner.command('main','switch','off')
    async def set_color(self, h: int, s: int, l: int, transmission: int | None = None) -> None:
        pass