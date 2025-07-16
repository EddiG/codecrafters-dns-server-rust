/// A DNS header as defined in [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1).
#[derive(Debug, Clone, Copy)]
pub struct DNSHeader {
    pub id: u16,
    pub flags: Flags,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DNSHeader {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    /// Creates a new DNS header with specified ID and flags, zero counts.
    pub fn new(id: u16, flags: Flags) -> Self {
        Self {
            id,
            flags,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    /// Creates a query header with recursion desired flag set.
    pub fn query(id: u16) -> Self {
        let mut flags = Flags::new();
        flags.set_rd(true); // Recursion desired
        Self::new(id, flags)
    }

    /// Creates a response header with QR flag set and response code.
    pub fn response(id: u16, rcode: u8) -> Self {
        let mut flags = Flags::new();
        flags.set_qr(true);
        flags.set_rcode(rcode);
        Self::new(id, flags)
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

/// The DNS header's flags
#[derive(Debug, Clone, Copy)]
pub struct Flags(u16);

impl Flags {
    // RFC 1035 bit positions (MSB first)
    const QR: u16 = 1 << 15; // Query/Response flag
    const OPCODE: u16 = 0x7800; // Operation code (4 bits)
    const AA: u16 = 1 << 10; // Authoritative Answer
    const TC: u16 = 1 << 9; // Truncation
    const RD: u16 = 1 << 8; // Recursion Desired
    const RA: u16 = 1 << 7; // Recursion Available
    const Z: u16 = 0x70; // Reserved (3 bits)
    const RCODE: u16 = 0x0F; // Response code (4 bits)

    /// Creates flags with all bits set to zero.
    pub fn new() -> Self {
        Self(0)
    }

    /// Sets the Query/Response flag (0=query, 1=response).
    pub fn set_qr(&mut self, is_response: bool) {
        if is_response {
            self.0 |= Self::QR;
        } else {
            self.0 &= !Self::QR;
        }
    }

    /// Sets the operation code (0=standard query, 1=inverse query, 2=status).
    pub fn set_opcode(&mut self, opcode: u8) {
        self.0 = (self.0 & !Self::OPCODE) | ((opcode as u16 & 0xF) << 11);
    }

    /// Sets the Authoritative Answer flag.
    pub fn set_aa(&mut self, authoritative: bool) {
        if authoritative {
            self.0 |= Self::AA;
        } else {
            self.0 &= !Self::AA;
        }
    }

    /// Sets the Recursion Desired flag.
    pub fn set_rd(&mut self, recursion_desired: bool) {
        if recursion_desired {
            self.0 |= Self::RD;
        } else {
            self.0 &= !Self::RD;
        }
    }

    /// Sets the Truncation flag (message was truncated).
    pub fn set_tc(&mut self, truncated: bool) {
        if truncated {
            self.0 |= Self::TC;
        } else {
            self.0 &= !Self::TC;
        }
    }

    /// Sets the Recursion Available flag.
    pub fn set_ra(&mut self, recursion_available: bool) {
        if recursion_available {
            self.0 |= Self::RA;
        } else {
            self.0 &= !Self::RA;
        }
    }

    /// Sets reserved bits (should be zero per RFC 1035).
    pub fn set_z(&mut self, reserved: u8) {
        self.0 = (self.0 & !Self::Z) | ((reserved as u16 & 0x7) << 4);
    }

    /// Sets the response code (0=no error, 1=format error, 2=server failure, etc.).
    pub fn set_rcode(&mut self, rcode: u8) {
        self.0 = (self.0 & !Self::RCODE) | (rcode as u16 & 0xF);
    }

    /// Converts flags to big-endian byte array for network transmission.
    pub fn to_be_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Flags> for u16 {
    fn from(flags: Flags) -> u16 {
        flags.0
    }
}
