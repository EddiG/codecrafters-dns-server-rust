use std::{mem, net::UdpSocket};

/// A DNS header as defined in RFC 1035.
#[derive(Debug, Clone)]
pub struct DNSHeader {
    id: u16,
    flags: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl DNSHeader {
    pub const SIZE: usize = mem::size_of::<Self>();

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

fn main() {
    println!("DNS server starting on 127.0.0.1:2053");

    let socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind UDP socket");
    let mut buf = [0u8; 512];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                println!("Received {} bytes from {}", size, src);

                let response_header = DNSHeader {
                    id: 1234,
                    flags: 0b1000000000000000,
                    qdcount: 0,
                    ancount: 0,
                    nscount: 0,
                    arcount: 0,
                };

                let response = response_header.to_bytes();
                socket
                    .send_to(&response, src)
                    .expect("Failed to send response");
            }
            Err(e) => eprintln!("Socket error: {}", e),
        }
    }
}
