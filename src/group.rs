use fourcc::FourCC;

use crate::record::{FormId, VersionControl};



pub struct Group<T> {
    pub header: GroupHeader,
    // Size is (header.size - 24 bytes)
    pub data: T
}

pub struct GroupHeader {
    pub iden: FourCC,
    pub size: u32, // Size INCLUDING 24-byte header
    pub label: GroupLabel,
    pub version_control: VersionControl, // Doesnt seem like its used for Groups
    pub unknown: u16
}

// Size: 8 bytes
// First u32 is the content
// Second u32 is the type of group label
#[repr(u32)] // Just put this here to shut up the compiler
pub enum GroupLabel {
    Top(FourCC) = 0,
    WorldChildren(FormId) = 1,
    InteriorCellBlock(i32) = 2,
    InteriorCellSubBlock(i32) = 3,
    ExteriorCellBlock([i16;2]) = 4,
    ExteriorCellSubBlock([i16;2]) = 5,
    CellChildren(FormId) = 6,
    TopicChildren(FormId) = 7,
    CellPersistentChildren(FormId) = 8,
    CellTemporaryChildren(FormId) = 9,
    CellVisibleDistantChildren(FormId) = 10
}