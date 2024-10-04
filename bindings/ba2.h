
#include "../types.h";

struct BA2Header {
    typeid_t type_id;
    uint32_t version;
    typeid_t archive_type;
    uint32_t file_count;
    uint64_t name_table_offset;
};