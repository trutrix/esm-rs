use fourcc::FourCC;

// Official name for the unqiue identifier for a record
pub type FormId = u32;

// IMPORTANT: WRLD, CELL, and QUST records can be followed by a child group

pub struct Record<T> {
    pub header: RecordHeader,
    // Size is header.size
    pub fields: Vec<T>
}

// Some records have a fixed amount of fields
// Some records have compounded fields
// Some records have oversized fields with iden 'XXXX'


// Record header is 24 bytes long
pub struct RecordHeader {
    pub iden: FourCC, // Content of record changes based on this
    pub size: u32, // Size NOT including 24-byte header
    pub flags: RecordFlags, 
    pub form_id: FormId, // Unique identifier for record
    pub version_control: VersionControl, // Version control for perforce
    pub unknown: u16 // Sometimes values appear here, no one seems to know what it is
}

pub struct RecordFlags(pub u32);

impl RecordFlags {

    // Not sure if this is the correct way to implement this
    // It works but might be wrong later on when more flags are introduced
    // If the record is compressed, there is a u32 (actual size) before the data, which is compressed with zlib
    pub fn is_compressed(&self) -> bool {
        self.0 & 0x00040000 != 0
    }

    // There are more flags, but we don't need them for now
}


pub struct VersionControl {
    pub timestamp: u16,
    pub last_user: u8,
    pub current_user: u8,
    pub version: u16,
}