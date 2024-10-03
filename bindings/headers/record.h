
#include "../types.h"

struct RecordHeader {
    typeid_t type_id;
    uint32_t size;
    flags32_t flags;
    uint32_t id;
    timestamp16_t timestamp;
    version16_t version;
    uint16_t internal_version;
    uint16_t unknown;
};
