
#include "../types.h"

struct GroupHeader {
    typeid_t type_id;
    uint32_t size;
    typeid_t label;
    uint32_t type;
    timestamp16_t timestamp;
    version16_t version;
    uint32_t unknown;
};

const uint32_t TOP_GROUP = 0;
const uint32_t WORLD_CHILDREN = 1;
const uint32_t EXTERIOR_CELL_BLOCK = 2;
const uint32_t EXTERIOR_CELL_SUB_BLOCK = 3;
const uint32_t INTERIOR_CELL_BLOCK = 4;
const uint32_t INTERIOR_CELL_SUB_BLOCK = 5;
const uint32_t CELL_CHILDREN = 6;
const uint32_t TOPIC_CHILDREN = 7;
const uint32_t CELL_PERSISTANT_CHILDREN = 8;
const uint32_t CELL_TEMPORARY_CHILDREN = 9;
const uint32_t CELL_VISIBLE_DISTANT_CHILDREN = 10;