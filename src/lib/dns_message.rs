use crate::lib::dns_header::{DnsHeader, QR};
use crate::lib::dns_question::DnsQuestion;
use crate::lib::record::RR;
use crate::lib::serde::DNSSerialization;
use bytes::{Buf, Bytes};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
pub struct DnsMessage {
    pub(crate) header: DnsHeader,
    pub(crate) question: DnsQuestion, // usually 0 or 1
    pub(crate) answer: RR,            // usually 0 or 1
    pub(crate) authority: IpAddr,
    pub(crate) additionnal_space: usize,
}

impl DnsMessage {
    fn new() -> Self {
        DnsMessage {
            header: DnsHeader::new(),
            question: DnsQuestion::new(),
            answer: RR::new(),
            authority: IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)),
            additionnal_space: 0,
        }
    }
}

impl DNSSerialization for DnsMessage {
    fn serialize(&self) -> Bytes {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.header.serialize());
        bytes.extend_from_slice(&self.question.serialize());
        bytes.extend_from_slice(&self.answer.serialize());
        bytes.extend_from_slice(
            &self
                .authority
                .to_string()
                .split(".")
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>(),
        );

        for i in bytes.len()..512 {
            bytes.push(0);
        }

        Bytes::from(bytes)
    }
    fn deserialize(s: &mut Bytes) -> Self {
        let header = DnsHeader::deserialize(s);
        let question = DnsQuestion::deserialize(s);
        let answer = RR::deserialize(s);
        let authority = IpAddr::from(Ipv4Addr::new(
            s.get_u8(),
            s.get_u8(),
            s.get_u8(),
            s.get_u8(),
        ));
        let additionnal_space = s.remaining();

        DnsMessage {
            header,
            question,
            answer,
            authority,
            additionnal_space,
        }
    }
}

#[test]
fn test_message() {
    let message = DnsMessage::new();
    let bytes = message.serialize();
    assert_eq!(bytes.len(), 512);
}

#[test]
fn test() {
    let ip = IpAddr::from(Ipv4Addr::new(127, 0, 0, 1));
    let ip_str = ip.to_string();
    println!("{}", ip_str);
    let aaa: Vec<_> = ip_str
        .trim()
        .split(".")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    println!("{:X?}", aaa);
}