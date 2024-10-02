
#include "../types.h"

struct RecordHeader {
    uint32_t flags;
    uint32_t type;
    uint16_t timestamp;
    uint16_t version;
    uint16_t internal_version;
    uint16_t unknown;
};
