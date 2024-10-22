#include "../types.h"

/**
 * <div rustbindgen replaces="VersionControl"></div>
 */
struct VersionControl {
    timestamp16_t timestamp;
    users16_t users;
    uint16_t form;
    uint16_t revision;
};