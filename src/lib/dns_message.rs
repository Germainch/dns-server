use crate::lib::dns_header::{DnsHeader, QR};
use crate::lib::dns_question::DnsQuestion;
use crate::lib::record::RR;
use crate::lib::serde::DNSSerialization;
use bytes::{Buf, Bytes};
use std::net::{IpAddr, Ipv4Addr};
use std::vec;

#[derive(Debug)]
pub struct DnsMessage {
    pub(crate) header: DnsHeader,
    pub(crate) question: Vec<DnsQuestion>, // usually 0 or 1
    pub(crate) answer: Vec<RR>,            // usually 0 or 1
    pub(crate) authority: IpAddr,
    pub(crate) additionnal_space: usize,
}

impl DnsMessage {
    fn new() -> Self {
        DnsMessage {
            header: DnsHeader::new(),
            question: vec![DnsQuestion::new()],
            answer: vec![RR::new()],
            authority: IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)),
            additionnal_space: 0,
        }
    }
}

impl DNSSerialization for DnsMessage {
    fn serialize(&self) -> Bytes {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.header.serialize());

        if self.question.len() > 0 {
            bytes.extend_from_slice(&self.question[0].serialize());
        }

        if self.answer.len() > 0 {
            bytes.extend_from_slice(&self.answer[0].serialize());
        }

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
    fn deserialize(s: &mut Bytes) -> Option<Self> {

        let header = match DnsHeader::deserialize(s){
            Some(h) => h,
            None => return None,
        };

        let question = match DnsQuestion::deserialize(s){
            Some(q) => vec![q],
            None => vec![],
        };

        let answer = match RR::deserialize(s){
            Some(a) => vec![a],
            None => vec![],
        };

        let authority = IpAddr::from(Ipv4Addr::new(
            s.get_u8(),
            s.get_u8(),
            s.get_u8(),
            s.get_u8(),
        ));

        let additionnal_space = s.remaining();

        Some(DnsMessage {
            header,
            question,
            answer,
            authority,
            additionnal_space,
        })
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