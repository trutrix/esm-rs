
#include "../types.h"

struct DNAM {
    int32_t animation_group;
    float32_t animation_timescale;
    float32_t unknown_1;
    uint8_t fire_trigger;
    uint8_t unknown_2[2];
    uint8_t reload_animation;
    float32_t minimum_spread;
    float32_t maximum_spread;
    float32_t unknown_3;
    float32_t zoom_fov;
    float32_t unused_1;
    formid_t projectile_form_id;
    uint8_t unknown_4;
    uint8_t firing_animation;
    uint8_t damage_multiplier;
    uint8_t unknown_5;
    float32_t projectile_velocity;
    float32_t max_range;
    int32_t projectile_behaviour;
    uint8_t unknown_6[4];
    float32_t reload_time;
    float32_t ammo_used_per_shot;
    float32_t ap_cost_per_shot;
    float32_t trigger_delay_1;
    float32_t trigger_delay_2;
    float32_t rapid_fire_timing;
    float32_t unused_2;
    float32_t critical_death_effect_chance;
    float32_t unknown_7;
    float32_t damage_vs_defense_modifier;
    int32_t unused_3;
    int32_t weapon_skill_type;
    int32_t unknown_8;
    float32_t unknown_9;
    float32_t unknown_10;
    int32_t resist_type;
    float32_t burst_mode_fire_rate;
    float32_t unknown_11;
    float32_t unknown_12;
};
