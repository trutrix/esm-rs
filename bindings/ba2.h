
#include "../types.h";

struct BA2Header {
    typeid_t type_id;
    uint32_t version;
    typeid_t archive_type;
    uint32_t file_count;
    uint64_t name_table_offset;
};

struct FileEntry {
    uint32_t unknown0;
    typeid_t ext;
    uint32_t unknown1;
    uint32_t unknown2;
    uint64_t offset;
    uint32_t packed_size;
    uint32_t unpacked_size;
    uint32_t unknown3;
};

struct TextureEntry {
    uint32_t filename_hash;
    uint32_t file_extension;
    uint32_t directory_hash;
    uint8_t unknown;
    uint8_t num_chunks;
    uint16_t chunk_header_size;
    uint16_t texture_height;
    uint16_t texture_width;
    uint8_t mip_levels;
    uint8_t format;
    ubool8_t is_cubemap;
    uint8_t tile_mode;
};

struct TextureChunk {
    uint64_t offset;
    uint32_t packed_size;
    uint32_t unpacked_size;
    uint16_t start_mip;
    uint16_t end_mip;
    uint32_t align;
};