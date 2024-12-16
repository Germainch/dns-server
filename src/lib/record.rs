
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
    fn deserialize(mut s: &mut Bytes) -> Self {
        let mut rr = RR::new();

        if !s.has_remaining() {
            return Self::new();
        }

        let mut name: String = String::new();
        let mut b = s.get_u8();
        while b != 0 {
            name.push(b as char);
            b = s.get_u8();
        }

        rr.name = name;

        rr.atype = match Type::try_from(s.get_u16()){
            Ok(t) => {t}
            Err(_) => {return Self::new();}
        };
        rr.aclass = Class::try_from(s.get_u16()).unwrap();
        rr.ttl = s.get_u32();
        rr.rdlength = s.get_u16();
        rr.rdata = s.get_u32();

        rr
    }
}
