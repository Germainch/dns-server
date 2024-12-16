mod lib;

use crate::lib::record::RR;
use crate::lib::dns_header::{DnsHeader, QR};
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
                let mut bytes = Bytes::from(buf.to_vec());

                let mut message = DnsMessage::deserialize(&mut bytes);


                let response = message.serialize();
                let msg = response.iter().as_slice();


                udp_socket
                    .send_to(msg, source)
                    .expect("Failed to send a response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
