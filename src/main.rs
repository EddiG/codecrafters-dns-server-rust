use std::net::UdpSocket;

use codecrafters_dns_server::DNSHeader;

fn main() {
    println!("DNS server starting on 127.0.0.1:2053");

    let socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind UDP socket");
    let mut buf = [0u8; 512];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                println!("Received {size} bytes from {src}");

                let response_header = DNSHeader::response(1234, 0);

                let response = response_header.to_bytes();
                socket
                    .send_to(&response, src)
                    .expect("Failed to send response");
            }
            Err(e) => eprintln!("Socket error: {e}"),
        }
    }
}
