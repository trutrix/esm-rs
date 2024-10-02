// Fixed size types used in ESM struct definitions.

#pragma once

#include <stdint.h>

typedef float float32_t;
typedef double float64_t;

/**
 * <div rustbindgen replaces="FourCC"></div>
 */
typedef char typeid_t[4];

/**
 * <div rustbindgen hide></div>
 */
typedef uint16_t timestamp_t;
