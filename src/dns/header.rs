use super::{flags::Flags, Response};

/// A DNS header as defined in [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1).
#[derive(Debug, Clone, Copy)]
pub(super) struct Header<Dir> {
    pub id: u16,
    pub flags: Flags<Dir>,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl Header<Response> {
    /// Creates a response header with QR flag set.
    pub fn response(id: u16) -> Self {
        let flags = Flags::<Response>::new();
        Self {
            id,
            flags,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }
}

impl<Dir> Header<Dir> {
    /// Serialize into a 12-byte array (big-endian/u16 network order)
    pub fn to_be_bytes(&self) -> [u8; 12] {
        let mut buf = [0u8; 12];
        buf[0..2].copy_from_slice(&self.id.to_be_bytes());
        buf[2..4].copy_from_slice(&self.flags.to_be_bytes());
        buf[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        buf[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        buf[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        buf[10..12].copy_from_slice(&self.arcount.to_be_bytes());
        buf
    }
}
