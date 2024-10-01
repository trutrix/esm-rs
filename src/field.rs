use fourcc::FourCC;


// If a field header is 'XXXX', the value is a u32, denoting the actual size of the next field (which should read zero),
// I think I have only seen this behavior in WRLD records

// The iden denotes the type of field, but changes depending on the record it is in
// Some idens are identical across records such as EDID - EditorId, which is a ZeroString

pub struct Field<T> {
    pub header: FieldHeader,
    pub data: T
}

pub struct FieldHeader {
    pub iden: FourCC,
    pub size: u16
}