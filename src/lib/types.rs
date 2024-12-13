#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    MX = 15,
    TXT = 16,
    AAAA = 28,
    SRV = 33,
    NAPTR = 35,
    DS = 43,
    RRSIG = 46,
    DNSKEY = 48,
    NSEC3 = 50,
    NSEC3PARAM = 51,
    TLSA = 52,
    CAA = 257,
    ANY = 255,
}

impl TryFrom<u16> for Type {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Type::A),
            2 => Ok(Type::NS),
            5 => Ok(Type::CNAME),
            6 => Ok(Type::SOA),
            12 => Ok(Type::PTR),
            15 => Ok(Type::MX),
            16 => Ok(Type::TXT),
            28 => Ok(Type::AAAA),
            33 => Ok(Type::SRV),
            35 => Ok(Type::NAPTR),
            43 => Ok(Type::DS),
            46 => Ok(Type::RRSIG),
            48 => Ok(Type::DNSKEY),
            50 => Ok(Type::NSEC3),
            51 => Ok(Type::NSEC3PARAM),
            52 => Ok(Type::TLSA),
            255 => Ok(Type::ANY),
            _ => Err(()),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Class {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
    ANY = 255,
}

impl TryFrom<u16> for Class {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Class::IN),
            2 => Ok(Class::CS),
            3 => Ok(Class::CH),
            4 => Ok(Class::HS),
            255 => Ok(Class::ANY),
            _ => Err(()),
        }
    }
}
