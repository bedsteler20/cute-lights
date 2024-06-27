import asyncio


class Light:
    async def set_color(
        self, h: int, s: int, l: int, transmission: int | None = None
    ) -> None:
        if h < 0 or h > 360:
            raise ValueError(f"Invalid hue value {h} (expected 0-360)")
        if s < 0 or s > 100:
            raise ValueError(f"Invalid saturation value {s} (expected 0-100)")
        if l < 0 or l > 100:
            raise ValueError(f"Invalid lightness value {l} (expected 0-100)")

    async def set_brightness(
        self, brightness: int, transmission: int | None = None
    ) -> None:
        if brightness < 0 or brightness > 100:
            raise ValueError(f"Invalid brightness value {brightness} (expected 0-100)")
        
    async def on(self) -> None:
        pass

    async def off(self) -> None:
        pass


class LightDiscoverer:
    async def discover(self) -> list[Light]:
        raise NotImplementedError


class LightKit:
    _discoverers: list[LightDiscoverer]

    def __init__(self):
        self._discoverers = []

    def add_discoverer(self, discoverer: LightDiscoverer):
        self._discoverers.append(discoverer)

    async def discover(self) -> list[Light]:
        print("Discovering lights")
        all_lights = await asyncio.gather(
            *[discoverer.discover() for discoverer in self._discoverers]
        )
        return [light for lights in all_lights for light in lights]
