use crate::lib::serde::DNSSerialization;
use crate::lib::types::{Class, Type};
use bytes::{Buf, Bytes};

#[derive(Debug, Clone, PartialEq)]
pub struct RR {
    pub(crate) name: String,  // Domain name in labels
    pub(crate) atype: Type,   // Answer Type 2-bytes integer
    pub(crate) aclass: Class, // Answer Class 2-bytes integer
    pub(crate) ttl: u32,      // Time to Live 4-bytes integer
    pub(crate) rdlength: u16, // Resource Data Length 2-bytes integer
    pub(crate) rdata: u32,    // Resource Data
}

impl RR {
    pub(crate) fn new() -> Self {
        RR {
            name: "codecrafters.io".parse().unwrap(),
            atype: Type::A,
            aclass: Class::IN,
            ttl: 60,
            rdlength: 4,
            rdata: 0x7F000001,
        }
    }
}

impl DNSSerialization for RR {
    fn serialize(&self) -> Bytes {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.name.as_bytes());
        bytes.push(0);
        bytes.push((self.atype as u16 >> 8) as u8);
        bytes.push((self.atype as u16 & 0xFF) as u8);
        bytes.push((self.aclass as u16 >> 8) as u8);
        bytes.push((self.aclass as u16 & 0xFF) as u8);
        bytes.push((self.ttl >> 24) as u8);
        bytes.push((self.ttl >> 16) as u8);
        bytes.push((self.ttl >> 8) as u8);
        bytes.push((self.ttl & 0xFF) as u8);
        bytes.push((self.rdlength >> 8) as u8);
        bytes.push((self.rdlength & 0xFF) as u8);
        bytes.extend_from_slice(&self.rdata.to_be_bytes());

        Bytes::from(bytes)
    }

    fn deserialize(mut s: &mut Bytes) -> Option<Self> {
        if !s.has_remaining() || s.remaining() < 12 {
            return None;
        }

        let mut name: String = String::new();
        let mut b = s.get_u8();
        while b != 0 {
            name.push(b as char);
            b = s.get_u8();
        }
        let atype = match Type::try_from(s.get_u16()) {
            Ok(t) => t,
            Err(_) => return None,
        };
        let aclass = match Class::try_from(s.get_u16()) {
            Ok(c) => c,
            Err(_) => return None,
        };
        let ttl = s.get_u32();
        let rdlength = s.get_u16();
        let rdata = s.get_u32();

        Some(RR {
            name,
            atype,
            aclass,
            ttl,
            rdlength,
            rdata,
        })
    }
}
