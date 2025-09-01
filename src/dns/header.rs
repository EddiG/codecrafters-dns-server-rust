use super::flags::Flags;

/// A DNS header as defined in [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1).
#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub id: u16,
    pub flags: Flags,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl Header {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    /// Creates a response header with QR flag set and response code.
    pub fn response(id: u16) -> Self {
        let mut flags = Flags::new();
        flags.set_qr(true);
        Self {
            id,
            flags,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    /// Serialize into a 12-byte array (big-endian/u16 network order)
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..2].copy_from_slice(&self.id.to_be_bytes());
        buf[2..4].copy_from_slice(&self.flags.to_be_bytes());
        buf[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        buf[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        buf[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        buf[10..12].copy_from_slice(&self.arcount.to_be_bytes());
        buf
    }
}
