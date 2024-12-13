use std::net::{IpAddr, Ipv4Addr};
use crate::lib::dns_answer::DnsAnswer;
use crate::lib::dns_header::DnsHeader;
use crate::lib::dns_question::DnsQuestion;

pub struct DnsMessage {
    header: DnsHeader,
    question: Vec<DnsQuestion>, // usually 0 or 1
    answer: Vec<DnsAnswer>,     // usually 0 or 1
    authority : IpAddr,
    additionnal_space: usize,
}

impl DnsMessage {
    pub fn from_bytes(buf: &[u8; 512]) -> Self {

        let header = DnsHeader::from_bytes(buf[0..12].try_into().unwrap());
        let question = DnsQuestion::from_bytes(&buf[12..]);
        let question_len = question.name.len() + 4; // 4 octets for qtype and qclass
        let answer = DnsAnswer::from_bytes(&buf[(12 + question_len)..]);
        let answer_len = answer.name.len() + 10; // 10 octets for atype, aclass, ttl, rdlength

        let question_vec = vec![question];
        let answer_vec = vec![answer.clone()];
        let auth_buf = &buf[(12 + question_len + answer.name.len() + 10)..];
        let authority = IpAddr::from(Ipv4Addr::new(auth_buf[0], auth_buf[1], auth_buf[2], auth_buf[3]));
        let authority_len = 4; // 4 octets for authority
        let additionnal_space = buf.len() - (12 + question_len + answer_len + authority_len);

        DnsMessage {
            header,
            question: question_vec,
            answer: answer_vec,
            authority,
            additionnal_space,
        }
    }

    pub fn new() -> Self {
        DnsMessage {
            header: DnsHeader::new(),
            question: vec![DnsQuestion::new()],
            answer: vec![DnsAnswer::new()],
            authority: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            additionnal_space: 0,
        }
    }

    pub fn to_bytes(&self) -> [u8;512] {
        let mut buffer: Vec<u8> = vec![0; 512];

        let mut bytes = self.header.to_bytes();

        for i in 0..bytes.len() {
            buffer[i] = (bytes[i] as u8);
        }

        let q = self.question[0].to_bytes();
        let a = self.answer[0].to_bytes();

        // question section
        buffer[12..(12 + q.len())].copy_from_slice(&q);

        // answer section
        buffer[(12 + q.len())..(12 + q.len() + a.len())].copy_from_slice(&a);

        // authority section
        buffer[(12 + q.len() + a.len())..(12 + q.len() + a.len() + 4)]
            .copy_from_slice(&self.authority.to_string()
                                            .trim()
                                            .split(".")
                                            .map(|x| x.parse::<u8>().unwrap())
                                            .collect::<Vec<u8>>());
        // additionnal space
        buffer[(12 + q.len() + a.len() + 4)..512].fill(0);
        <[u8; 512]>::try_from(buffer).unwrap()
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