
#include "../types.h"

struct GroupHeader {
    typeid_t label;
    int32_t type;
    timestamp_t timestamp;
    uint16_t version;
    uint32_t unknown;
};
