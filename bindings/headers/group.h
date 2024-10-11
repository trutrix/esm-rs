
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

const uint8_t TOP_GROUP = 0;
const uint8_t WORLD_CHILDREN = 1;
const uint8_t EXTERIOR_CELL_BLOCK = 2;
const uint8_t EXTERIOR_CELL_SUB_BLOCK = 3;
const uint8_t INTERIOR_CELL_BLOCK = 4;
const uint8_t INTERIOR_CELL_SUB_BLOCK = 5;
const uint8_t CELL_CHILDREN = 6;
const uint8_t TOPIC_CHILDREN = 7;
const uint8_t CELL_PERSISTANT_CHILDREN = 8;
const uint8_t CELL_TEMPORARY_CHILDREN = 9;
const uint8_t CELL_VISIBLE_DISTANT_CHILDREN = 10;