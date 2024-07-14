import ctypes

def load_lib():
    lib = ctypes.CDLL('./target/debug/libcutelights.so')

    # ============ LightDiscoverer ============

    # LightDiscoverer* LightDiscoverer::new();
    lib.light_discoverer_new.restype = ctypes.c_void_p
    lib.light_discoverer_new.argtypes = []

    # Light* LightDiscoverer::next(LightDiscoverer* discoverer);
    lib.light_discoverer_next.restype = ctypes.c_void_p
    lib.light_discoverer_next.argtypes = [ctypes.c_void_p]

    # void LightDiscoverer::free(LightDiscoverer* discoverer);
    lib.light_discoverer_free.argtypes = [ctypes.c_void_p]
    lib.light_discoverer_free.restype = None

    # ============ Light ============

    # bool Light::set_on(Light* light, bool on);
    lib.light_set_on.argtypes = [ctypes.c_void_p, ctypes.c_bool]
    lib.light_set_on.restype = ctypes.c_bool

    # bool Light::set_color(Light* light, int64 h, int64 s, int64 b);
    lib.light_set_color.argtypes = [ctypes.c_void_p, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64]
    lib.light_set_color.restype = ctypes.c_bool

    # bool Light::set_brightness(Light* light, int64 brightness);
    lib.light_set_brightness.argtypes = [ctypes.c_void_p, ctypes.c_int64]
    lib.light_set_brightness.restype = ctypes.c_bool

    # int64 Light::get_brightness(Light* light);
    lib.light_get_brightness.argtypes = [ctypes.c_void_p]
    lib.light_get_brightness.restype = ctypes.c_int64

    # int64 Light::get_hue(Light* light);
    lib.light_get_hue.argtypes = [ctypes.c_void_p]
    lib.light_get_hue.restype = ctypes.c_int64

    # int64 Light::get_saturation(Light* light);
    lib.light_get_saturation.argtypes = [ctypes.c_void_p]
    lib.light_get_saturation.restype = ctypes.c_int64

    # bool Light::get_is_on(Light* light);
    lib.light_get_is_on.argtypes = [ctypes.c_void_p]
    lib.light_get_is_on.restype = ctypes.c_bool

    # String* Light::get_name()
    lib.light_get_name.restype = ctypes.c_char_p
    lib.light_get_name.argtypes = [ctypes.c_void_p]

    # String* Light::get_id()
    lib.light_get_id.restype = ctypes.c_char_p
    lib.light_get_id.argtypes = [ctypes.c_void_p]

    # bool Light::get_supports_color(Light* light);
    lib.light_get_supports_color.argtypes = [ctypes.c_void_p]
    lib.light_get_supports_color.restype = ctypes.c_bool

    # void Light::free(Light* light);
    lib.light_free.argtypes = [ctypes.c_void_p]
    lib.light_free.restype = None

    # ============ Frame ============
    
    # Frame* Frame::new();
    lib.frame_new.restype = ctypes.c_void_p
    lib.frame_new.argtypes = []

    # void Frame::clear(Frame* frame);
    lib.frame_clear.argtypes = [ctypes.c_void_p]
    lib.frame_clear.restype = None

    # void Frame::free(Frame* frame);
    lib.frame_free.argtypes = [ctypes.c_void_p]
    lib.frame_free.restype = None

    # void Frame::set_on(Frame* frame, Light* light, bool on);
    lib.frame_set_on.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_bool]
    lib.frame_set_on.restype = None

    # void Frame::set_color(Frame* frame, Light* light, int64 h, int64 s, int64 b);
    lib.frame_set_color.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64]
    lib.frame_set_color.restype = None

    # void Frame::set_brightness(Frame* frame, Light* light, int64 brightness);
    lib.frame_set_brightness.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_int64]
    lib.frame_set_brightness.restype = None

    # void Frame::run(Frame* frame);
    lib.frame_run.argtypes = [ctypes.c_void_p]
    lib.frame_run.restype = None

    return lib

lib = load_lib()

class Light:
    def __init__(self, pointer):
        self.pointer = pointer

        # These are string func's that dont change so we can cache them and 
        # not have to cross the FFI boundary every time and decode utf-8
        self.name = lib.light_get_name(self.pointer).decode('utf-8')
        self.id = lib.light_get_id(self.pointer).decode('utf-8')

    def set_on(self, on:bool) -> bool:
        return lib.light_set_on(self.pointer, on)
    
    def set_color(self, h:int, s:int, b:int) -> bool:
        if h < 0 or h > 360:
            raise ValueError('Hue must be between 0 and 360')
        if s < 0 or s > 100:
            raise ValueError('Saturation must be between 0 and 100')
        if b < 0 or b > 100:
            raise ValueError('Brightness must be between 0 and 100')
        
        return lib.light_set_color(self.pointer, h, s, b)
    
    def set_brightness(self, brightness: int) -> bool:
        if brightness < 0 or brightness > 100:
            raise ValueError('Brightness must be between 0 and 100')
        return lib.light_set_brightness(self.pointer, brightness)
    
    @property
    def brightness(self) -> int:
        return lib.light_get_brightness(self.pointer)
    
    @property
    def hue(self) -> int:
        return lib.light_get_hue(self.pointer)
    
    @property
    def saturation(self) -> int:
        return lib.light_get_saturation(self.pointer)
    
    @property
    def is_on(self) -> bool:
        return lib.light_get_is_on(self.pointer)
    
    @property
    def supports_color(self) -> bool:
        return lib.light_get_supports_color(self.pointer)
    
    def __del__(self):
        lib.light_free(self.pointer)

class Frame:
    def __init__(self) -> None:
        self.pointer = lib.frame_new()

    def clear(self) -> None:
        lib.frame_clear(self.pointer)
    
    def set_on(self, light: Light, on: bool) -> None:
        lib.frame_set_on(self.pointer, light.pointer, on)

    def set_color(self, light: Light, h: int, s: int, b: int) -> None:
        lib.frame_set_color(self.pointer, light.pointer, h, s, b)
    
    def set_brightness(self, light: Light, brightness: int) -> None:
        lib.frame_set_brightness(self.pointer, light.pointer, brightness)

    def set_on_all(self, lights: list[Light], on: bool) -> None:
        for light in lights:
            lib.frame_set_on(self.pointer, light.pointer, on)

    def set_color_all(self, lights: list[Light], h: int, s: int, b: int) -> None:
        for light in lights:
            lib.frame_set_color(self.pointer, light.pointer, h, s, b)

    def set_brightness_all(self, lights: list[Light], brightness: int) -> None:
        for light in lights:
            lib.frame_set_brightness(self.pointer, light.pointer, brightness)

    def run(self) -> None:
        lib.frame_run(self.pointer)

    def __del__(self):
        lib.frame_free(self.pointer)

def discover():
    discoverer = lib.light_discoverer_new()
    lights = []
    while True:
        light = lib.light_discoverer_next(discoverer)
        if light == None:
            break
        lights.append(Light(light))
    lib.light_discoverer_free(discoverer)
    return lights

from time import sleep

__all__ = ['Light', 'Frame', 'discover', 'sleep']

if __name__ == '__main__':
    lights = discover()
    for light in lights:
        print(light.name)