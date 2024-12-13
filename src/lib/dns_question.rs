use crate::lib::types::{Class, Type};
#[allow(dead_code, unused)]

#[derive(Debug, Clone, PartialEq)]
pub struct DnsQuestion {
    pub(crate) name: Vec<u8>, // Domain name in labels
    qtype: Type,   // Question Type 2-bytes integer
    qclass: Class,  // Question Class 2-bytes integer
}

impl DnsQuestion {

    pub fn new() -> Self {
        DnsQuestion{
            name: b"\x0ccodecrafters\x02io\x00".to_vec(),
            qtype: Type::A,
            qclass: Class::IN,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for byte in self.name.iter() {
            bytes.push(*byte);
        }

        bytes.push((self.qtype as u16 >> 8) as u8);
        bytes.push((self.qtype as u16 & 0xFF) as u8);
        bytes.push((self.qclass as u16  >> 8) as u8);
        bytes.push((self.qclass as u16 & 0xFF) as u8);
        bytes
    }

    pub fn from_bytes(p0: &[u8]) -> Self {
        let mut i = 0;
        let mut name = Vec::new();
        while p0[i] != 0 {
            name.push(p0[i]);
            i += 1;
        }
        name.push(p0[i]);
        i += 1;

        let qtype = Type::try_from((p0[i + 1] as u16) | (p0[i] as u16) << 8).unwrap();
        let qclass = Class::try_from((p0[i + 3] as u16) | (p0[i + 2] as u16) << 8).unwrap();


        DnsQuestion {
            name,
            qtype,
            qclass,
        }
    }
}


#[test]
fn test_question(){
    let question = DnsQuestion::new();
    let bytes = question.to_bytes();
    let question2 = DnsQuestion::from_bytes(&bytes);
    assert_eq!(question, question2);
}

#[test]
fn test_serde_question(){
    let question = DnsQuestion::new();
    let bytes = question.to_bytes();
    let question2 = DnsQuestion::from_bytes(&bytes);
    assert_eq!(question, question2);
}