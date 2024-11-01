

#[derive(Clone, Copy)]
pub struct RecordFlags(pub u32);

impl std::fmt::Debug for RecordFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            write!(f, "None")
        } else {
            if self.is_compressed() {
                write!(f, "Compressed ")?;
            }
            write!(f, "Unknown")
        }
        
    }
}

impl RecordFlags {
    pub fn is_compressed(&self) -> bool {
        (self.0 & 0x00040000) != 0
    }
}