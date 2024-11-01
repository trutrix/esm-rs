/// Fixed size types used in ESM struct definitions.
/**
 * When defining custom types, a simple sized definition is all bindgen needs:
 *
 *   good: typedef char timestamp_t[2];
 *   bad: typedef uint16_t timestamp_t;
 *
 * Overspecifying causes bindgen to remove useful traits like Debug, even if the
 * equivalent type in Rust includes a Debug impl.
 */

#pragma once

#include <stdint.h>

typedef float float32_t;
typedef double float64_t;

/**
 * <div rustbindgen replaces="FourCC"></div>
 */
typedef char typeid_t[4];


/**
 * <div rustbindgen replaces="RecordFlags"></div>
 */
typedef char flags32_t[4];



typedef char timestamp16_t[2];
typedef char users16_t[2];

typedef uint8_t bool8_t;
typedef uint64_t hash_t;

typedef uint32_t formid_t;
typedef uint8_t rgba8_t[4];
typedef int16_t cell_coord_t[2];
