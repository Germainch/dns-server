use std::io::{BufReader, Read};
#[allow(unused_imports)]
use std::net::UdpSocket;
use serde::{Deserialize, Serialize};
use serde::de::IntoDeserializer;
#[derive(Debug)]
struct DnsHeader {
    id: u16,
    qr: u8,
    opcode: u8,
    aa: u8,
    tc: u8,
    rd: u8,
    ra: u8,
    z: u8,
    rcode: u8,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}
impl DnsHeader {
    fn new() -> Self {
        DnsHeader {
            id: 1234,   // Packet Identifier
            qr: 1,      // Query/Response Indicator (1 for response)
            opcode: 0,  // Operation Code
            aa: 0,      // Authoritative Answer
            tc: 0,      // Truncation
            rd: 0,      // Recursion Desired
            ra: 0,      // Recursion Available
            z: 0,       // Reserved
            rcode: 0,   // Response Code
            qdcount: 0, // Question Count
            ancount: 0, // Answer Record Count
            nscount: 0, // Authority Record Count
            arcount: 0, // Additional Record Count
        }
    }
    fn to_bytes(&self) -> [u8; 12] {
        let mut bytes = [0u8; 12];

        // Packet Identifier (ID)
        bytes[0] = (self.id >> 8) as u8;
        bytes[1] = (self.id & 0xFF) as u8;

        // Flags (QR, OPCODE, AA, TC, RD)
        bytes[2] = (self.qr << 7)
            | ((self.opcode & 0xF) << 3)
            | ((self.aa & 0x1) << 2)
            | ((self.tc & 0x1) << 1)
            | (self.rd & 0x1);

        // Flags (RA, Z, RCODE)
        bytes[3] = (self.ra << 7) | ((self.z & 0x7) << 4) | (self.rcode & 0xF);

        // Question Count (QDCOUNT)
        bytes[4] = (self.qdcount >> 8) as u8;
        bytes[5] = (self.qdcount & 0xFF) as u8;

        // Answer Record Count (ANCOUNT)
        bytes[6] = (self.ancount >> 8) as u8;
        bytes[7] = (self.ancount & 0xFF) as u8;

        // Authority Record Count (NSCOUNT)
        bytes[8] = (self.nscount >> 8) as u8;
        bytes[9] = (self.nscount & 0xFF) as u8;

        // Additional Record Count (ARCOUNT)
        bytes[10] = (self.arcount >> 8) as u8;
        bytes[11] = (self.arcount & 0xFF) as u8;

        bytes
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response = DnsHeader::new().to_bytes();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
