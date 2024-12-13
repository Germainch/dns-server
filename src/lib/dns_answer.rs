use crate::lib::types::{Class, Type};

#[allow(dead_code, unused)]
#[derive(Debug)]
pub struct DnsAnswer{
    rrs:Vec<RR>
}

impl DnsAnswer{
    pub fn new() -> Self{
        DnsAnswer{
            rrs: vec![RR::new()]
        }
    }

    pub fn to_bytes(&self) -> Vec<u8>{
        self.rrs.first().unwrap().to_bytes()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RR {
    pub(crate) name: String,  // Domain name in labels
    atype: Type,   // Answer Type 2-bytes integer
    aclass: Class, // Answer Class 2-bytes integer
    ttl: u32,       // Time to Live 4-bytes integer
    rdlength: u16,  // Resource Data Length 2-bytes integer
    rdata: Vec<u8>, // Resource Data
}

impl RR {
    pub(crate) fn new() -> Self {
        RR {
            name: "codecrafters.io".parse().unwrap(),
            atype: Type::A,
            aclass: Class::IN,
            ttl: 60,
            rdlength: 4,
            rdata: vec![127, 0, 0, 1],
        }
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for byte in  self.name.as_bytes() {
            bytes.push(*byte);
        }

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
        let rdata_bytes = self.rdata.as_slice();
        rdata_bytes.iter().for_each(|byte| bytes.push(*byte));
        bytes
    }

    pub(crate) fn from_bytes(p0: &[u8]) -> Self {

        if(p0.len() < 10 || p0[0] == 0){
            return Self::new();
        }

        let mut i = 0;
        let mut name = Vec::new();

        while p0[i] != 0 {
            name.push(p0[i]);
            i += 1;
        }

        // we push the last 0 byte and increment i
        name.push(p0[i]);
        i += 1;

        let name = String::from_utf8(name).unwrap();

        let atype = Type::try_from((p0[i] as u16) << 8 | p0[i + 1] as u16).unwrap();
        let aclass = Class::try_from((p0[i + 2] as u16) << 8 | p0[i + 3] as u16).unwrap();
        let ttl = (p0[i + 4] as u32) << 24 | (p0[i + 5] as u32) << 16 | (p0[i + 6] as u32) << 8 | p0[i + 7] as u32;
        let rdlength = (p0[i + 8] as u16) << 8 | p0[i + 9] as u16;
        let mut rdata = Vec::new();
        for j in 0..rdlength {
            rdata.push(p0[i + 10 + j as usize]);
        }

        RR {
            name,
            atype,
            aclass,
            ttl,
            rdlength,
            rdata,
        }
    }



    pub fn set_name(&mut self, buf: &[u8] ){
        let mut i = 0;
        let mut name = Vec::new();
        while buf[i] != 0 {
            name.push(buf[i]);
            i += 1;
        }
        name.push(buf[i]);
        i += 1;
        self.name = String::from_utf8(name).unwrap();
    }

    pub fn set_type(&mut self, atype: Type){
        self.atype = atype;
    }

    pub fn set_class(&mut self, aclass: Class){
        self.aclass = aclass;
    }

    pub fn set_ttl(&mut self, ttl: u32){
        self.ttl = ttl;
    }

    pub fn set_rdlength(&mut self, rdlength: u16){
        self.rdlength = rdlength;
    }

    pub fn set_rdata(&mut self, rdata: Vec<u8>){
        self.rdata = rdata;
    }
}

pub fn build_answer(buf: &[u8]) -> DnsAnswer {
    let mut rr = RR::from_bytes(&buf);
    rr.set_type(Type::A);
    rr.set_class(Class::IN);
    rr.set_ttl(60);
    rr.set_rdlength(4);
    rr.set_rdata(vec![127, 0, 0, 1]);
    DnsAnswer {
        rrs: vec![rr],
    }
}

// --------------------- TESTS ---------------------

#[test]
fn test_answer() {
    let answer = RR::new();
    println!("{:X?}", answer);
    let bytes = answer.to_bytes();
    println!("{:X?}", bytes);
}

#[test]
fn test_serde_answer(){
    let answer = RR::new();
    println!("answer: {:X?}", answer.name);

    let copy = answer.clone();

    let bytes = answer.to_bytes();
    println!("bytes : {:X?}", bytes);
    let reconstructed: RR = RR::from_bytes(&bytes);
    println!("reconstructed : {:X?}", reconstructed.name);

    assert_eq!(copy, reconstructed);
}