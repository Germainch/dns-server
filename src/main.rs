mod lib;

use crate::lib::record::RR;
use crate::lib::dns_header::DnsHeader;
use crate::lib::dns_message::DnsMessage;
use crate::lib::dns_question::DnsQuestion;
#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_variables)]
use std::net::UdpSocket;
use bytes::Bytes;
use crate::lib::serde::DNSSerialization;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let bytes = Bytes::from(buf.to_vec());
                println!("Received bytes: {:X?}", bytes);
                let message = DnsMessage::deserialize(bytes);
                let response = message.serialize();

                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send a response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
