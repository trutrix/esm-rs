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
 * <div rustbindgen replaces="Timestamp"></div>
 */
typedef char timestamp_t[2];
