mod lib;

#[allow(unused_imports)]
use std::net::UdpSocket;
use crate::lib::dns_answer::DnsAnswer;
use crate::lib::dns_header::DnsHeader;
use crate::lib::dns_question::DnsQuestion;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];


    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                // udp_socket.send_to( &response, source ).expect("Failed to send a response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
