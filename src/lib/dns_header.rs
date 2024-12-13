use crate::lib::record::RR;
use crate::lib::dns_question::DnsQuestion;
use crate::lib::serde::DNSSerialization;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::ops::Shl;

// --------------------- FLAGS STRUCTS ---------------------
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QR {
    QUERY = 0,
    RESPONSE = 1,
}

/// Operation Code:
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OPCODE {
    QUERY = 0,
    IQUERY = 1,
    STATUS = 2,
    UNASSIGNED = 3,
    NOTIFY = 4,
    UPDATE = 5,
    DSO = 6, // DNS Stateful Operation
}

/// Response Code :
///
///
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RCODE {
    NOERROR = 0,  // No Error
    FORMERR = 1,  // Format Error
    SERVFAIL = 2, // Server Failure
    NXDOMAIN = 3, // Non-Existent Domain
    NOTIMP = 4,   // Not Implemented
    REFUSED = 5,  // Query Refused
    YXDOMAIN = 6, // Name exists when it should not
    YXRRSET = 7,  // RR set exists when it should not
    NXRRSET = 8,  // RR set sould exists but does not
    NOTAUTH = 9,  // Not Authorized / Not authoritative
    NOTZONE = 10, // Name not contained in zone
    DSOTYPENI = 11, // DSO-TYPE not implemented

                  // ...
}

// --------------------- TRY FROM IMPLEMENTATIONS ---------------------
impl TryFrom<u8> for QR {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(QR::QUERY),
            1 => Ok(QR::RESPONSE),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for OPCODE {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OPCODE::QUERY),
            1 => Ok(OPCODE::IQUERY),
            2 => Ok(OPCODE::STATUS),
            3 => Ok(OPCODE::UNASSIGNED),
            4 => Ok(OPCODE::NOTIFY),
            5 => Ok(OPCODE::UPDATE),
            6 => Ok(OPCODE::DSO),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for RCODE {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RCODE::NOERROR),
            1 => Ok(RCODE::FORMERR),
            2 => Ok(RCODE::SERVFAIL),
            3 => Ok(RCODE::NXDOMAIN),
            4 => Ok(RCODE::NOTIMP),
            5 => Ok(RCODE::REFUSED),
            6 => Ok(RCODE::YXDOMAIN),
            7 => Ok(RCODE::YXRRSET),
            8 => Ok(RCODE::NXRRSET),
            9 => Ok(RCODE::NOTAUTH),
            10 => Ok(RCODE::NOTZONE),
            11 => Ok(RCODE::DSOTYPENI),
            _ => Err(()),
        }
    }
}

// --------------------- DNS HEADER ---------------------
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DnsHeader {
    pub(crate) id: u16,
    pub(crate) qr: QR,
    pub(crate) opcode: OPCODE,
    pub(crate) aa: u8,
    pub(crate) tc: u8,
    pub(crate) rd: u8,
    pub(crate) ra: u8,
    pub(crate) z: u8,
    pub(crate) rcode: RCODE,
    pub(crate) qdcount: u16,
    pub(crate) ancount: u16,
    pub(crate) nscount: u16,
    pub(crate) arcount: u16,
}

impl DnsHeader {
    pub(crate) fn new() -> Self {
        DnsHeader {
            id: 1234,              // Packet Identifier
            qr: QR::RESPONSE,      // Query/Response Indicator (1 for response)
            opcode: OPCODE::QUERY, // Operation Code
            aa: 0,                 // Authoritative Answer
            tc: 0,                 // Truncation
            rd: 0,                 // Recursion Desired
            ra: 0,                 // Recursion Available
            z: 0,                  // Reserved
            rcode: RCODE::NOERROR, // Response Code
            qdcount: 0,            // Question Count
            ancount: 1,            // Answer Record Count
            nscount: 0,            // Authority Record Count
            arcount: 0,            // Additional Record Count
        }
    }

    pub fn set_qr(&mut self, qr: QR) {
        self.qr = qr;
    }
}

impl DNSSerialization for DnsHeader {
    fn serialize(&self) -> Bytes {
        let mut bytes = BytesMut::from(vec![0u8; 12].as_slice());
        bytes.put_u16(self.id);
        bytes.put_u8(
            ((self.qr as u8) << 7)
                | ((self.opcode as u8) << 3)
                | (self.aa << 2)
                | (self.tc << 1)
                | (self.rd << 0),
        );

        bytes.put_u16(self.qdcount);
        bytes.put_u16(self.ancount);
        bytes.put_u16(self.nscount);
        bytes.put_u16(self.arcount);

        Bytes::from(bytes)
    }

    fn deserialize(mut s: Bytes) -> Self {

        if s.remaining() < 12 {
            return Self::new();
        }

        let id = s.get_u16();
        let a = s.get_u8();
        let b = s.get_u8();

        let mut qr = QR::RESPONSE;
        if let Ok(res) = QR::try_from(a >> 7){
            qr = res;
        }
        else {
            return Self::new();
        }

        let mut opcode = OPCODE::QUERY;
        if let Ok(res) = OPCODE::try_from((a >> 3) & 0x0F){
            opcode = res;
        }
        else {
            return Self::new();
        }

        let aa = (a >> 3) & 0x01;
        let tc = (a >> 2) & 0x02;
        let rd = (a >> 1) & 0x01;
        let ra = b >> 7;
        let z = b >> 4 & 0x07;
        let rcode = RCODE::try_from((b >> 2) & 0xF).unwrap();

        let qdcount = s.get_u16();
        let ancount = s.get_u16();
        let nscount = s.get_u16();
        let arcount = s.get_u16();

        DnsHeader {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }
}

// --------------------- TESTS --------------------
