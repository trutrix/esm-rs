
use fourcc::FourCC;

use super::GroupHeader;


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

impl GroupHeader {
    pub fn try_get_label(&self) -> Result<GroupLabel, String> {
        match self.group_type {
            0 => {
                Ok(GroupLabel::Top(FourCC::from(&self.group_value)))
            }
            1 => {
                Ok(GroupLabel::WorldChildren(u32::from_le_bytes(self.group_value)))
            }
            2 => {
                Ok(GroupLabel::InteriorCellBlock(i32::from_le_bytes(self.group_value)))
            }
            3 => {
                Ok(GroupLabel::InteriorCellSubBlock(i32::from_le_bytes(self.group_value)))
            }
            4 => {
                Ok(GroupLabel::ExteriorCellBlock([i16::from_le_bytes(self.group_value[0..2].try_into().unwrap()), i16::from_le_bytes(self.group_value[2..4].try_into().unwrap())]))
            }
            5 => {
                Ok(GroupLabel::ExteriorCellSubBlock([i16::from_le_bytes(self.group_value[0..2].try_into().unwrap()), i16::from_le_bytes(self.group_value[2..4].try_into().unwrap())]))
            }
            6 => {
                Ok(GroupLabel::CellChildren(u32::from_le_bytes(self.group_value)))
            }
            7 => {
                Ok(GroupLabel::TopicChildren(u32::from_le_bytes(self.group_value)))
            }
            8 => {
                Ok(GroupLabel::CellPersistentChildren(u32::from_le_bytes(self.group_value)))
            }
            9 => {
                Ok(GroupLabel::CellTemporaryChildren(u32::from_le_bytes(self.group_value)))
            }
            10 => {
                Ok(GroupLabel::CellVisibleDistantChildren(u32::from_le_bytes(self.group_value)))
            }
            _ => {
                Err(format!("Unknown group type: {:?}", self))
            }
        }
    }
}