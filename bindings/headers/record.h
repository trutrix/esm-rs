
#include "../types.h"
#include "./version.h"


/**
 * <div rustbindgen nodebug></div>
 */
struct RecordHeader {
    typeid_t type_id;
    uint32_t size;
    flags32_t flags;
    uint32_t id;
    struct VersionControl version_control;
};

