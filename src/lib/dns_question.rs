use crate::lib::serde::DNSSerialization;
use crate::lib::types::{Class, Type};
use bytes::{Buf, Bytes};
#[allow(dead_code, unused)]
#[derive(Debug, Clone, PartialEq)]
pub struct DnsQuestion {
    pub(crate) name: Vec<u8>, // Domain name in labels
    pub(crate) qtype: Type,   // Question Type 2-bytes integer
    pub(crate) qclass: Class, // Question Class 2-bytes integer
}

impl DnsQuestion {
    pub fn new() -> Self {
        DnsQuestion {
            name: b"\x0ccodecrafters\x02io\x00".to_vec(),
            qtype: Type::A,
            qclass: Class::IN,
        }
    }
}

impl DNSSerialization for DnsQuestion {
    fn serialize(&self) -> Bytes {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.name);
        bytes.push(0);
        bytes.push((self.qtype as u16 >> 8) as u8);
        bytes.push((self.qtype as u16 & 0xFF) as u8);
        bytes.push((self.qclass as u16 >> 8) as u8);
        bytes.push((self.qclass as u16 & 0xFF) as u8);

        Bytes::from(bytes)
    }
    fn deserialize(mut s: Bytes) -> Self {
        if !s.has_remaining() {
            return Self::new();
        }

        let mut name: String = String::new();
        let mut b = s.get_u8();
        while b != 0 {
            name.push(b as char);
        }

        let mut qtype = Type::A;
        if let Ok(res) = Type::try_from(s.get_u16()){
            qtype = res;
        }
        else {
            return Self::new();
        };

        let mut qclass = Class::IN;
        if let Ok(res) = Class::try_from(s.get_u16()){
            qclass = res;
        }
        else {
            return Self::new();
        }

        DnsQuestion {
            name: name.into_bytes(),
            qtype,
            qclass,
        }
    }
}
