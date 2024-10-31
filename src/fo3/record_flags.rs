

#[derive(Debug)]
pub struct RecordFlags(pub u32);

impl RecordFlags {
    pub fn is_compressed(&self) -> bool {
        (self.0 & 0x00040000) != 0
    }
}