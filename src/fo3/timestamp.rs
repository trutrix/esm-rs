
// 16-bit timestamp
#[derive(Eq, PartialEq, PartialOrd, Hash, Clone, Copy, Default)]
pub struct Timestamp(pub [i8;2]);

// print format
impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let year = ((self.0[1] - 1) / 12 + 3) % 10;
        let month = ((self.0[1] - 1) % 12) + 1;
        write!(f, "{:02}/{:02}/200{}", self.0[0], month, year)
    }
}

// debug print format
impl std::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let year = ((self.0[1] - 1) / 12 + 3) % 10;
        let month = ((self.0[1] - 1) % 12) + 1;
        write!(f, "{:02}/{:02}/200{}", self.0[0], month, year)
    }
}
