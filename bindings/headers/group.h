
#include "../types.h"

struct GroupHeader {
    typeid_t type_id;
    uint32_t size;
    typeid_t label;
    int32_t type;
    timestamp16_t timestamp;
    version16_t version;
    uint32_t unknown;
};
