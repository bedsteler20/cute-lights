import typing

if typing.TYPE_CHECKING:

    class Light:
        is_on: bool
        supports_color: bool
        red: int
        green: int
        blue: int
        brightness: int
        name: str
        id: int

        def set_on(self, on: bool):
            pass

        def set_brightness(self, brightness: int):
            pass

        def set_color(self, r: int, g: int, b: int):
            pass

    class Frame:
        def set_on(self, light: Light, on: bool):
            pass

        def set_on_all(self, lights: typing.List[Light], on: bool):
            pass

        def set_brightness(self, light: Light, brightness: int):
            pass

        def set_brightness_all(self, lights: typing.List[Light], brightness: int):
            pass

        def set_color(self, light: Light, r: int, g: int, b: int):
            pass

        def set_color_all(self, lights: typing.List[Light], r: int, g: int, b: int):
            pass

        def run(self):
            pass

        def clear(self):
            pass

    def discover_lights() -> typing.List[Light]:
        pass

else:
    from .cute_light import *
    import asyncio

    __doc__ = cute_light.__doc__
    __all__ = cute_light.__all__

    def __set_on_all(self, lights, on):
        for l in lights:
            self.set_on(l, on)

    def __set_brightness_all(self, lights, brightness):
        for l in lights:
            self.set_brightness(l, brightness)

    def __set_color_all(self, lights, r, g, b):
        for l in lights:
            self.set_color(l, r, g, b)

    Frame.set_on_all = __set_on_all
