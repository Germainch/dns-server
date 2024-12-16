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
    fn len(&self) -> usize {
        self.name.len()
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
    fn deserialize(s: &mut Bytes) -> Self {
        if !s.has_remaining() {
            return Self::new();
        }

        let mut name: String = String::new();
        let mut b = s.get_u8();
        while b != 0 {
            name.push(b as char);
            b = s.get_u8();
        }

        let qtype = Type::try_from(((s.get_u8() as u16) << 8) | s.get_u8() as u16);
        let qclass = Class::try_from(((s.get_u8() as u16) << 8) | s.get_u8() as u16);

        DnsQuestion {
            name:   name.into_bytes(),
            qtype:  Type::A,
            qclass: Class::IN,
        }
    }
}
