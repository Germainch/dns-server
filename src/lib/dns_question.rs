use crate::lib::serde::DNSSerialization;
use crate::lib::types::{Class, Type};
use bytes::{Buf, Bytes};
#[allow(dead_code, unused)]
#[derive(Debug, Clone, PartialEq)]
pub struct DnsQuestion {
    pub(crate) name: String, // Domain name in labels
    pub(crate) qtype: Type,   // Question Type 2-bytes integer
    pub(crate) qclass: Class, // Question Class 2-bytes integer
}

impl DnsQuestion {
    pub fn new() -> Self {
        DnsQuestion {
            name: "codecrafters.io".parse().unwrap(),
            qtype: Type::A,
            qclass: Class::IN,
        }
    }
    fn len(&self) -> usize {
        self.name.len()
    }
}

impl DNSSerialization for DnsQuestion {
    fn serialize(&self) -> Bytes {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.name.as_bytes());
        bytes.push(0);
        bytes.push((self.qtype as u16 >> 8) as u8);
        bytes.push((self.qtype as u16 & 0xFF) as u8);
        bytes.push((self.qclass as u16 >> 8) as u8);
        bytes.push((self.qclass as u16 & 0xFF) as u8);

        Bytes::from(bytes)
    }
    fn deserialize(s: &mut Bytes) -> Option<Self> {
        if !s.has_remaining() || s.iter().peekable().peek().is_some_and(|b| **b == 0) {
            return None;
        }

        let mut name: String = String::new();
        let mut b = s.get_u8();
        while b != 0 {
            name.push(b as char);
            b = s.get_u8();
        }

        let qtype = Type::try_from(((s.get_u8() as u16) << 8) | s.get_u8() as u16);
        let qclass = Class::try_from(((s.get_u8() as u16) << 8) | s.get_u8() as u16);

        Some(DnsQuestion {
            name,
            qtype:  Type::A,
            qclass: Class::IN,
        })
    }
}
