pub struct DnsQuestion {
    name: Vec<u8>, // Domain name in labels
    qtype: u16,   // Question Type 2-bytes integer
    qclass: u16,  // Question Class 2-bytes integer
}

impl DnsQuestion {

    pub fn new() -> Self {
        DnsQuestion{
            name: b"\x0ccodecrafters\x02io\x00".to_vec(),
            qtype: 1,
            qclass: 1,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let name_bytes = self.name.as_slice();
        name_bytes.iter().for_each(|byte| bytes.push(*byte));
        bytes.push((self.qtype >> 8) as u8);
        bytes.push((self.qtype & 0xFF) as u8);
        bytes.push((self.qclass >> 8) as u8);
        bytes.push((self.qclass & 0xFF) as u8);
        bytes
    }

    pub fn from_buf(p0: &[u8; 512]) -> Self {
        let mut i = 12;
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
        DnsQuestion {
            name,
            qtype: 1,
            qclass: 1,
        }
    }
}