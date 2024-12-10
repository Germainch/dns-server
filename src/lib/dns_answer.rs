
pub struct DnsAnswer {
    name: Vec<u8>, // Domain name in labels
    atype: u16,   // Answer Type 2-bytes integer
    aclass: u16,  // Answer Class 2-bytes integer
    ttl: u32,     // Time to Live 4-bytes integer
    rdlength: u16, // Resource Data Length 2-bytes integer
    rdata: Vec<u8>, // Resource Data
}



impl DnsAnswer {
    fn new() -> Self {
        DnsAnswer {
            name: b"\x0ccodecrafters\x02io\x00".to_vec(),
            atype: 1,
            aclass: 1,
            ttl: 60,
            rdlength: 4,
            rdata: vec![127, 0, 0, 1],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let name_bytes = self.name.as_slice();
        name_bytes.iter().for_each(|byte| bytes.push(*byte));
        bytes.push((self.atype >> 8) as u8);
        bytes.push((self.atype & 0xFF) as u8);
        bytes.push((self.aclass >> 8) as u8);
        bytes.push((self.aclass & 0xFF) as u8);
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
    fn from_buf(p0: &[u8; 512]) -> Self {
        let mut i = 12;
        while p0[i] != 0 {
            let len = p0[i] as usize;
            i += len + 1;
        }
        i += 5;
        let mut name = Vec::new();
        while p0[i] != 0 {
            let len = p0[i] as usize;
            for j in 0..len {
                name.push(p0[i + j + 1]);
            }
            name.push(b'.');
            i += len + 1;
        }
        name.pop();
        DnsAnswer {
            name,
            atype: 1,
            aclass: 1,
            ttl: 60,
            rdlength: 4,
            rdata: vec![8, 8, 8, 8],
        }
    }
}