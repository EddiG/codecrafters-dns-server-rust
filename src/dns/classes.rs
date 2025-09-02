/// CLASS fields appear in resource records.
/// Note that these classes are a subset of QCLASSes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RClass {
    IN = 1,
}

impl RClass {
    pub fn to_be_bytes(&self) -> [u8; 2] {
        (*self as u16).to_be_bytes()
    }
}

/// QCLASS fields appear in the question section of a query.
/// QCLASS values are a superset of CLASS values; every CLASS is a valid QCLASS.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QClass {
    IN = 1,
    Any = 255,
}

impl QClass {
    pub fn to_be_bytes(&self) -> [u8; 2] {
        (*self as u16).to_be_bytes()
    }
}
