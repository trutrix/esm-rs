// Packed struct bindings for Bethesda Softworks Archive.

#include "./types.h";

#pragma pack(push, 1)

struct BSAHeader {
    typeid_t type_id;
    uint32_t version;
    uint32_t offset;
    uint32_t archive_flags;
    uint32_t folder_count;
    uint32_t file_count;
    uint32_t total_folder_name_length;
    uint32_t total_file_name_length;
    uint16_t file_flags;
    uint16_t padding;
};

struct BSAFolderRecord {
    hash_t name_hash;
    uint32_t count;
    //uint32_t padding_1;
    uint32_t offset;
    //uint32_t padding_2;
};

struct BSAFileRecord {
    hash_t name_hash;
    uint32_t size;
    uint32_t offset;
};

#pragma pack(pop)
