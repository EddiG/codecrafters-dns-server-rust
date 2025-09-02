use std::net::UdpSocket;

use codecrafters_dns_server::dns::{Message, Question};

fn main() {
    println!("DNS server starting on 127.0.0.1:2053");

    let socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind UDP socket");
    let mut buf = [0u8; 512];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                println!("Received {size} bytes from {src}");

                let question = vec![Question::new("codecrafters.io", 1, 1)];
                let response_message = Message::response(1234, question);

                let response_bytes = response_message.to_be_bytes();
                socket
                    .send_to(&response_bytes, src)
                    .expect("Failed to send response");
            }
            Err(e) => eprintln!("Socket error: {e}"),
        }
    }
}
