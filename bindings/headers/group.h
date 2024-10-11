
#include "../types.h"

enum group_label_t {
    Top = 0,
    WorldChildren = 1,
    InteriorCellBlock = 2,
    InteriorCellSubBlock = 3,
    ExteriorCellBlock = 4,
    ExteriorCellSubBlock = 5,
    CellChildren = 6,
    TopicChildren = 7,
    CellPersistantChildren = 8,
    CellTemporaryChildren = 9,
    CellVisibleDistantChildren = 10
};

struct GroupHeader {
    typeid_t type_id;
    uint32_t size;
    typeid_t label;
    enum group_label_t type;
    timestamp16_t timestamp;
    version16_t version;
    uint32_t unknown;
};


