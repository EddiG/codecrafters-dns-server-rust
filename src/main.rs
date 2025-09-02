use std::net::UdpSocket;

use codecrafters_dns_server::dns::{
    DomainName, Message, QClass, QType, Question, RClass, RType, ResourceRecord,
};

fn main() {
    println!("DNS server starting on 127.0.0.1:2053");

    let socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind UDP socket");
    let mut buf = [0u8; 512];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                println!("Received {size} bytes from {src}");

                let domain = "codecrafters.io".parse::<DomainName>().unwrap();
                let question = vec![Question::new(domain.clone(), QType::A, QClass::IN)];
                let answer = vec![ResourceRecord::new(
                    domain,
                    RType::A,
                    RClass::IN,
                    3600,
                    vec![127, 0, 0, 1],
                )];
                let response_message = Message::response(1234, question, answer);

                let response_bytes = response_message.to_be_bytes();
                socket
                    .send_to(&response_bytes, src)
                    .expect("Failed to send response");
            }
            Err(e) => eprintln!("Socket error: {e}"),
        }
    }
}
