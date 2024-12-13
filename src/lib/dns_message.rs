use std::net::{IpAddr, Ipv4Addr};
use crate::lib::dns_answer::{build_answer, DnsAnswer, RR};
use crate::lib::dns_header::{DnsHeader, QR};
use crate::lib::dns_question::DnsQuestion;

#[derive(Debug)]
pub struct DnsMessage {
    header: DnsHeader,
    question: DnsQuestion, // usually 0 or 1
    answer: DnsAnswer,     // usually 0 or 1
    authority : IpAddr,
    additionnal_space: usize,
}

impl DnsMessage {
    pub fn from_bytes(buf: &[u8; 512]) -> Self {

        let header = DnsHeader::from_bytes(buf[0..12].try_into().unwrap());
        let question = DnsQuestion::from_bytes(&buf[12..]);
        let question_len = question.name.len() + 4; // 4 octets for qtype and qclass
        let answer = build_answer(&buf[(12 + question_len)..]);
        let answer_len = answer.to_bytes().len(); // 10 octets for atype, aclass, ttl, rdlength

        let auth_buf = &buf[(12 + question_len + answer_len)..];
        let authority = IpAddr::from(Ipv4Addr::new(auth_buf[0], auth_buf[1], auth_buf[2], auth_buf[3]));
        let authority_len = 4; // 4 octets for authority
        let additionnal_space = buf.len() - (12 + question_len + answer_len + authority_len);

        DnsMessage {
            header,
            question,
            answer,
            authority,
            additionnal_space,
        }
    }

    pub fn build_response(buf: &[u8;512]) -> Self{
        let mut header = DnsHeader::from_bytes(buf[0..12].try_into().unwrap());
        header.set_qr(QR::RESPONSE);
        let question = DnsQuestion::new();
        let answer = build_answer(&buf[(12+question.to_bytes().len())..]);
        let authority = IpAddr::from(Ipv4Addr::new(0, 0, 0, 0));
        let additionnal_space = 0;

        DnsMessage {
            header,
            question,
            answer,
            authority,
            additionnal_space,
        }
    }

    pub fn new() -> Self {
        DnsMessage {
            header: DnsHeader::new(),
            question: DnsQuestion::new(),
            answer: DnsAnswer::new(),
            authority: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            additionnal_space: 0,
        }
    }

    pub fn to_bytes(&self) -> [u8;512] {
        let mut buffer: Vec<u8> = vec![0; 512];

        let mut bytes = self.header.to_bytes();
        let q = self.question.to_bytes();
        let a = self.answer.to_bytes();
        let authority = self.authority.to_string()
                                    .trim()
                                    .split(".")
                                    .map(|x| x.parse::<u8>().unwrap())
                                    .collect::<Vec<u8>>();

        let mut concat = [bytes.as_slice(), q.as_slice(), a.as_slice(), authority.as_slice()].concat();
        let len = concat.len();
        println!("{:?}", len);

        // fill the rest of the buffer with 0s
        for i in len..512{
            concat.push(0);
        }
        <[u8; 512]>::try_from(concat).unwrap()
    }
}

#[test]
fn test_message(){
    let message = DnsMessage::new();
    let bytes = message.to_bytes();
    assert_eq!(bytes.len(), 512);
}

#[test]
fn test(){
    let ip = IpAddr::from(Ipv4Addr::new(127, 0, 0, 1));
    let ip_str = ip.to_string();
    println!("{}", ip_str);
    let aaa: Vec<_> = ip_str.trim().split(".").map(|x| x.parse::<u8>().unwrap()).collect();
    println!("{:X?}", aaa);
}