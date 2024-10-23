use super::RecordHeader;




impl std::fmt::Debug for RecordHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "Record {{ Type: {:?}, Size, {:?}, Flags: {:?}, Id: {:?} }}", {self.type_id}, {self.size}, {self.flags}, {self.id})
    }
}