
// 16-bit version control information
#[derive(Eq, PartialEq, PartialOrd, Hash, Clone, Copy, Default)]
pub struct VersionControlInfo(pub [u8;2]);

// print format
impl std::fmt::Display for VersionControlInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
        { write!(f, "{:04}:{:04}", self.0[0], self.0[1]) }
}

// debug print format
impl std::fmt::Debug for VersionControlInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
        { write!(f, "{:04}:{:04}", self.0[0], self.0[1]) }
}
