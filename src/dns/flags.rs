use super::Response;
use std::marker::PhantomData;

/// The DNS header's flags
#[derive(Debug, Clone, Copy)]
pub struct Flags<Dir>(u16, PhantomData<Dir>);

impl Flags<Response> {
    pub fn new() -> Self {
        Self(Self::QR, PhantomData)
    }
}

impl<Dir> Flags<Dir> {
    // RFC 1035 bit positions (MSB first)
    const QR: u16 = 1 << 15; // Query/Response flag
                             //
                             // const OPCODE: u16 = 0x7800; // Operation code (4 bits)
                             // const AA: u16 = 1 << 10; // Authoritative Answer
                             // const TC: u16 = 1 << 9; // Truncation
                             // const RD: u16 = 1 << 8; // Recursion Desired
                             // const RA: u16 = 1 << 7; // Recursion Available
                             // const RCODE: u16 = 0x0F; // Response code (4 bits)

    /// Converts flags to big-endian byte array for network transmission.
    pub fn to_be_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }

    // /// Sets the Query/Response flag (0=query, 1=response).
    // pub fn set_qr(&mut self, is_response: bool) {
    //     if is_response {
    //         self.0 |= Self::QR;
    //     } else {
    //         self.0 &= !Self::QR;
    //     }
    // }

    // /// Sets the operation code (0=standard query, 1=inverse query, 2=status).
    // pub fn set_opcode(&mut self, opcode: u8) {
    //     self.0 = (self.0 & !Self::OPCODE) | (((opcode as u16) << 11) & Self::OPCODE);
    // }

    // /// Sets the Authoritative Answer flag.
    // pub fn set_aa(&mut self, authoritative: bool) {
    //     if authoritative {
    //         self.0 |= Self::AA;
    //     } else {
    //         self.0 &= !Self::AA;
    //     }
    // }

    // /// Sets the Recursion Desired flag.
    // pub fn set_rd(&mut self, recursion_desired: bool) {
    //     if recursion_desired {
    //         self.0 |= Self::RD;
    //     } else {
    //         self.0 &= !Self::RD;
    //     }
    // }

    // /// Sets the Truncation flag (message was truncated).
    // pub fn set_tc(&mut self, truncated: bool) {
    //     if truncated {
    //         self.0 |= Self::TC;
    //     } else {
    //         self.0 &= !Self::TC;
    //     }
    // }

    // /// Sets the Recursion Available flag.
    // pub fn set_ra(&mut self, recursion_available: bool) {
    //     if recursion_available {
    //         self.0 |= Self::RA;
    //     } else {
    //         self.0 &= !Self::RA;
    //     }
    // }

    // /// Sets the response code (0=no error, 1=format error, 2=server failure, etc.).
    // pub fn set_rcode(&mut self, rcode: u8) {
    //     self.0 = (self.0 & !Self::RCODE) | (rcode as u16 & Self::RCODE);
    // }
}

// impl From<Flags<Response>> for u16 {
//     fn from(flags: Flags<Response>) -> u16 {
//         flags.0
//     }
// }
