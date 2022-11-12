#[derive(Debug)]
pub struct Address{
    bits: u32
}

impl Address {
    pub fn from_bytes(byte1: u8, byte2: u8, byte3: u8, byte4: u8) -> Address {
        Address{bits: u32::from_be_bytes([byte1, byte2, byte3, byte4])}
    }
    pub fn from_string(ip_address: &str) -> Address {
        let splitted: Vec<&str> = ip_address.split('.').collect();
        Address::from_bytes(
            splitted[0].parse().unwrap(), 
            splitted[1].parse().unwrap(), 
            splitted[2].parse().unwrap(), 
            splitted[3].parse().unwrap())
    }
    pub fn from_u32(ip_address: u32) -> Address {
        Address { bits: ip_address }
    }

    pub fn as_bytes(&self) -> [u8; 4] {
        let bytes = self.bits.to_be_bytes();
        return bytes;
    }

    pub fn as_u32(&self) -> u32 {
        return self.bits;
    }

    pub fn to_string(&self) -> String {
        let bytes = self.as_bytes();
        bytes[0].to_string() + "." + &bytes[1].to_string() + "." + &bytes[2].to_string() + "." + &bytes[3].to_string()
    }

    pub fn to_bitstring(&self) -> String {
        let bytes = self.as_bytes();
        format!("{:0>8b}", bytes[0]) + "." + &format!("{:0>8b}", bytes[1]) + "." +  &format!("{:0>8b}", bytes[2]) + "." +  &format!("{:0>8b}", bytes[3])
    }

    pub fn next(&self) -> Address {
        Address::from_u32(self.bits + 1)
    }
}