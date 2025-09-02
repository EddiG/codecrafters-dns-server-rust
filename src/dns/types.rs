/// TYPE fields are used in resource records.
/// Note that these types are a subset of QTYPEs.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RType {
    A = 1,
    CNAME = 5,
}

impl RType {
    pub fn to_be_bytes(&self) -> [u8; 2] {
        (*self as u16).to_be_bytes()
    }
}

/// QTYPE fields appear in the question part of a query.
/// QTYPES are a superset of TYPEs, hence all TYPEs are valid QTYPEs.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QType {
    A = 1,
    CNAME = 5,
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
    ANY = 255,
}

impl QType {
    pub fn to_be_bytes(&self) -> [u8; 2] {
        (*self as u16).to_be_bytes()
    }
}
