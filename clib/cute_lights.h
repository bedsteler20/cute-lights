#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct FramePtr;

struct LightDiscovererPtr;

struct LightPtr;

extern "C" {

bool light_set_on(LightPtr *l, bool on);

bool light_set_color(LightPtr *l, uint8_t red, uint8_t green, uint8_t blue);

bool light_set_brightness(LightPtr *l, uint8_t brightness);

uint8_t light_get_brightness(LightPtr *l);

uint8_t light_get_red(LightPtr *l);

uint8_t light_get_green(LightPtr *l);

uint8_t light_get_blue(LightPtr *l);

bool light_get_is_on(LightPtr *l);

char *light_get_name(LightPtr *l);

char *light_get_id(LightPtr *l);

bool light_get_supports_color(LightPtr *l);

void light_free(LightPtr *l);

LightDiscovererPtr *light_discoverer_new();

LightPtr *light_discoverer_next(LightDiscovererPtr *ld);

void light_discoverer_free(LightDiscovererPtr *ld);

FramePtr *frame_new();

void frame_clear(FramePtr *f);

void frame_free(FramePtr *f);

void frame_set_on(FramePtr *f, LightPtr *l, bool on);

void frame_set_color(FramePtr *f, LightPtr *l, uint8_t red, uint8_t green, uint8_t blue);

void frame_set_brightness(FramePtr *f, LightPtr *l, uint8_t brightness);

void frame_run(FramePtr *f);

} // extern "C"
