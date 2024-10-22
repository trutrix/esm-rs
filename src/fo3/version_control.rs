


#[derive(Eq, PartialEq, PartialOrd, Hash, Clone, Copy)]
pub struct VersionControl {
    pub timestamp: Timestamp,
    pub users: VCUsers,
    pub form: u16,
    pub revision: u16
}

// 16-bit timestamp
#[derive(Eq, PartialEq, PartialOrd, Hash, Clone, Copy)]
pub struct Timestamp(pub [i8;2]);

// print format
impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let year = ((self.0[0] - 1) / 12 + 3) % 10;
        let month = ((self.0[0] - 1) % 12) + 1;
        write!(f, "{:02}/{:02}/200{}", self.0[1] >> 1, month, year)
    }
}

// debug print format
impl std::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let year = ((self.0[0] - 1) / 12 + 3) % 10;
        let month = ((self.0[0] - 1) % 12) + 1;
        write!(f, "{:02}/{:02}/200{}", self.0[1] >> 1, month, year)
    }
}


// 16-bit version control information
#[derive(Eq, PartialEq, PartialOrd, Hash, Clone, Copy, Default)]
pub struct VCUsers(pub [u8;2]);

// print format
impl std::fmt::Display for VCUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
        { write!(f, "Last: {:03} Current: {:03}", self.0[0], self.0[1]) }
}

// debug print format
impl std::fmt::Debug for VCUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
        { write!(f, "Last: {:03} Current: {:03}", self.0[0], self.0[1]) }
}