
#include "../types.h"

struct GroupHeader {
    typeid_t label;
    int32_t type;
    uint16_t timestamp;
    uint16_t version;
    uint32_t unknown;
};
