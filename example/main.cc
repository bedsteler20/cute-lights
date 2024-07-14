#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include <vector>
#include <unistd.h>
#include "cute_lights.h"


int main() {
    auto discoverer = light_discoverer_new();
    auto lights = std::vector<light *>();

    // Loop through the lights
    for (light *l = light_discoverer_next(discoverer); l != nullptr; l = light_discoverer_next(discoverer)) {
        lights.push_back(l);
        printf("Found light: %s\n", light_get_name(l));
        // light_set_on(l, false);
    }

    light_discoverer_free(discoverer);

    // int64_t hue = 0;

    // frame *f = frame_new();

    // while (true) {
    //     printf("Setting hue to %ld\n", hue);
    //     for (auto l : lights) {
    //         frame_set_color(f, l, hue, 100, 100);
    //     }

    //     hue += 1;
    //     if (hue > 360) {
    //         hue = 0;
    //     }

    //     frame_run(f);
    //     frame_clear(f);
    //     usleep(100000);
    // }

    return 0;
}