use std::net::UdpSocket;

use codecrafters_dns_server::dns::{Message, Response};

fn main() {
    println!("DNS server starting on 127.0.0.1:2053");

    let socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind UDP socket");
    let mut buf = [0u8; 512];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                println!("Received {size} bytes from {src}");

                let response_message = Message::<Response>::new(1234);

                let response_bytes = response_message.to_bytes();
                socket
                    .send_to(&response_bytes, src)
                    .expect("Failed to send response");
            }
            Err(e) => eprintln!("Socket error: {e}"),
        }
    }
}
