
#include "../types.h"

struct MGEF {
    uint8_t flags[4];
    float32_t cost;
    formid_t script_id;
    int32_t magic_school;
    int32_t resistance_av;
    uint8_t unknown[2];
    uint8_t unused[2];
    formid_t light_id;
    float32_t projectile_speed;
    formid_t hit_shader;
    formid_t enchantment_shader;
    formid_t casting_sound;
    formid_t bolt_sound;
    formid_t hit_sound_effect;
    formid_t area_sound;
    float32_t ce_enchantment_factor;
    float32_t ce_barter_factor;
    int32_t is_scripted;
    int32_t actor_value;
};
