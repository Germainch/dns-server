use std::net::IpAddr;
use crate::lib::dns_answer::DnsAnswer;
use crate::lib::dns_header::DnsHeader;
use crate::lib::dns_question::DnsQuestion;

pub struct DnsMessage {
    header: DnsHeader,
    question: Vec<DnsQuestion>, // usually 0 or 1
    answer: Vec<DnsAnswer>, // usually 0 or 1
    authority : IpAddr,
    additionnal_space: usize,
}

impl DnsMessage {

}