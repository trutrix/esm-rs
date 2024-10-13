
#include "../types.h"


enum GroupLabelType: uint32_t {
    Top = 0,
    WorldChildren = 1,
    ExteriorCellBlock = 2,
    ExteriorCellSubBlock = 3,
    InteriorCellBlock = 4,
    InteriorCellSubBlock = 5,
    CellChildren = 6,
    TopicChildren = 7,
    CellPersistantChildren = 8,
    CellTemporaryChildren = 9,
    CellVisibleDistantChildren = 10
};

struct group_label_t {
    uint8_t group_value[4];
    enum GroupLabelType group_type;
};

struct GroupHeader {
    typeid_t type_id;
    uint32_t size;
    struct group_label_t label;
    timestamp16_t timestamp;
    version16_t version;
    uint32_t unknown;
};



