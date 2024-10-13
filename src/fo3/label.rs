use std::io::Read;

use fourcc::FourCC;

use super::{cell_coord_t, group_label_t, GroupLabelType_ExteriorCellBlock};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupLabel {
    Top(FourCC),
    WorldChildren(u32),
    InteriorCellBlock(i32),
    InteriorCellSubBlock(i32),
    ExteriorCellBlock([i16;2]),
    ExteriorCellSubBlock([i16;2]),
    CellChildren(u32),
    TopicChildren(u32),
    CellPersistentChildren(u32),
    CellTemporaryChildren(u32),
    CellVisibleDistantChildren(u32)
}

impl From<group_label_t> for GroupLabel {
    fn from(value: group_label_t) -> Self {
        match value.group_type {
            0 => {
                GroupLabel::Top(FourCC::from(&value.group_value))
            }
            1 => {
                GroupLabel::WorldChildren(u32::from_le_bytes(value.group_value))
            }
            2 => {
                GroupLabel::InteriorCellBlock(i32::from_le_bytes(value.group_value))
            }
            3 => {
                GroupLabel::InteriorCellSubBlock(i32::from_le_bytes(value.group_value))
            }
            4 => {
                GroupLabel::ExteriorCellBlock([i16::from_le_bytes(value.group_value[0..2].try_into().unwrap()), i16::from_le_bytes(value.group_value[2..4].try_into().unwrap())])
            }
            5 => {
                GroupLabel::ExteriorCellSubBlock([i16::from_le_bytes(value.group_value[0..2].try_into().unwrap()), i16::from_le_bytes(value.group_value[2..4].try_into().unwrap())])
            }
            6 => {
                GroupLabel::CellChildren(u32::from_le_bytes(value.group_value))
            }
            7 => {
                GroupLabel::TopicChildren(u32::from_le_bytes(value.group_value))
            }
            8 => {
                GroupLabel::CellPersistentChildren(u32::from_le_bytes(value.group_value))
            }
            9 => {
                GroupLabel::CellTemporaryChildren(u32::from_le_bytes(value.group_value))
            }
            10 => {
                GroupLabel::CellVisibleDistantChildren(u32::from_le_bytes(value.group_value))
            }


            _ => {
                panic!("Unknown group type: {:?}", value);
            }
        }
    }
}