#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct FramePtr FramePtr;

typedef struct LightDiscovererPtr LightDiscovererPtr;

typedef struct LightPtr LightPtr;

bool light_set_on(struct LightPtr *l, bool on);

bool light_set_color(struct LightPtr *l, uint8_t red, uint8_t green, uint8_t blue);

bool light_set_brightness(struct LightPtr *l, uint8_t brightness);

uint8_t light_get_brightness(struct LightPtr *l);

uint8_t light_get_red(struct LightPtr *l);

uint8_t light_get_green(struct LightPtr *l);

uint8_t light_get_blue(struct LightPtr *l);

bool light_get_is_on(struct LightPtr *l);

char *light_get_name(struct LightPtr *l);

char *light_get_id(struct LightPtr *l);

bool light_get_supports_color(struct LightPtr *l);

void light_free(struct LightPtr *l);

struct LightDiscovererPtr *light_discoverer_new(void);

struct LightPtr *light_discoverer_next(struct LightDiscovererPtr *ld);

void light_discoverer_free(struct LightDiscovererPtr *ld);

struct FramePtr *frame_new(void);

void frame_clear(struct FramePtr *f);

void frame_free(struct FramePtr *f);

void frame_set_on(struct FramePtr *f, struct LightPtr *l, bool on);

void frame_set_color(struct FramePtr *f,
                     struct LightPtr *l,
                     uint8_t red,
                     uint8_t green,
                     uint8_t blue);

void frame_set_brightness(struct FramePtr *f, struct LightPtr *l, uint8_t brightness);

void frame_run(struct FramePtr *f);
