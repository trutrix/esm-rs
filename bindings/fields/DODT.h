
#include "../types.h"

struct DODT {
    float32_t min_width;
    float32_t max_width;
    float32_t min_height;
    float32_t max_height;
    float32_t depth;
    float32_t shininess;
    float32_t parallax_scale;
    uint8_t parallax_passes;
    uint8_t flags;
    uint8_t unknown[2];
    rgba8_t color;
};
