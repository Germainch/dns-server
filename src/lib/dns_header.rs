use crate::lib::dns_answer::RR;
use crate::lib::dns_question::DnsQuestion;
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
    id: u16,
    pub(crate) qr: QR,
    opcode: OPCODE,
    aa: u8,
    tc: u8,
    rd: u8,
    ra: u8,
    z: u8,
    rcode: RCODE,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
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

    pub fn to_bytes(&self) -> [u8; 12] {
        let mut bytes = [0u8; 12];

        // Packet Identifier (ID)
        bytes[0] = (self.id >> 8) as u8;
        bytes[1] = (self.id & 0xFF) as u8;

        // Flags (QR, OPCODE, AA, TC, RD)
        bytes[2] = ((self.qr as u8) << 7)
            | ((self.opcode as u8 & 0xF) << 3)
            | ((self.aa & 0x1) << 2)
            | ((self.tc & 0x1) << 1)
            | (self.rd & 0x1);

        // Flags (RA, Z, RCODE)
        bytes[3] = (self.ra << 7) | ((self.z & 0x7) << 4) | (self.rcode as u8 & 0xF);

        // Question Count (QDCOUNT)
        bytes[4] = (self.qdcount >> 8) as u8;
        bytes[5] = (self.qdcount & 0xFF) as u8;

        // Answer Record Count (ANCOUNT)
        bytes[6] = (self.ancount >> 8) as u8;
        bytes[7] = (self.ancount & 0xFF) as u8;

        // Authority Record Count (NSCOUNT)
        bytes[8] = (self.nscount >> 8) as u8;
        bytes[9] = (self.nscount & 0xFF) as u8;

        // Additional Record Count (ARCOUNT)
        bytes[10] = (self.arcount >> 8) as u8;
        bytes[11] = (self.arcount & 0xFF) as u8;

        bytes
    }
    pub fn from_bytes(data: &[u8; 12]) -> Self {
        let qr = match QR::try_from(data[2] >> 7) {
            Ok(qr) => qr,
            Err(e) => panic!("Invalid QR"),
        };

        let opcode = OPCODE::try_from((data[2] >> 3) & 0xF).unwrap_or_else(|e| OPCODE::UNASSIGNED);

        let rcode = RCODE::try_from((data[3] >> 2) & 0xF).unwrap_or_else(|e| RCODE::FORMERR);

        DnsHeader {
            id: (((data[0] as u16) << 8) | (data[1] as u16 & 0xFF)) as u16,
            qr,
            opcode,
            aa: (data[2] >> 3) & 0x01,
            tc: (data[2] >> 2) & 0x02,
            rd: (data[2] >> 1) & 0x01,
            ra: data[3] >> 7,
            z: data[3] >> 4 & 0x07,
            rcode,
            qdcount: (data[4] as u16) << 8 | data[5] as u16,
            ancount: (data[6] as u16) << 8 | data[7] as u16,
            nscount: (data[8] as u16) << 8 | data[9] as u16,
            arcount: (data[10] as u16) << 8 | data[11] as u16,
        }
    }
}



// --------------------- TESTS ---------------------
#[test]
fn test_header() {
    let header = DnsHeader::new();
    println!("{:X?}", header);
    let bytes = header.to_bytes();
    println!("{:X?}", bytes);
}

#[test]
fn test_header_serde() {
    let header = DnsHeader::new();
    println!("header: {:X?}", header);

    let copy = header.clone();

    let bytes = header.to_bytes();
    let reconstructed: DnsHeader = DnsHeader::from_bytes(&bytes);
    println!("bytes : {:X?}", bytes);
    println!("reconstructed : {:X?}", reconstructed);

    assert_eq!(header, reconstructed);
}
