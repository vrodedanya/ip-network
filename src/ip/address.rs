use std::num::ParseIntError;

#[derive(Clone, Copy)]
pub enum SomeAddress {
    V4(AddressV4),
    V6(AddressV6),
}

pub trait Address {
    fn to_string(&self) -> String;
    fn next(&self) -> Box<dyn Address>;
    fn to_bitstring(&self) -> String;
    fn apply_bitmask(&self, bitmask: &super::bitmask::Bitmask) -> Box<dyn Address>;
}

#[derive(Debug, Clone, Copy)]
pub struct AddressV4 {
    bits: u32
}

#[derive(Debug, Clone, Copy)]
pub struct AddressV6 {
    bits: u128
}

impl AddressV4 {
    pub fn from_bytes(bytes: [u8; 4]) -> AddressV4 {
        AddressV4{bits: u32::from_be_bytes(bytes)}
    }
    pub fn from_string(ip_address: &str) -> Result<AddressV4, ParseIntError> {
        let splitted: Vec<&str> = ip_address.split('.').collect();
        Ok(AddressV4::from_bytes(
            [splitted[0].parse()?, 
            splitted[1].parse()?, 
            splitted[2].parse()?, 
            splitted[3].parse()?]
        ))
    }
    pub fn from_u32(ip_address: u32) -> AddressV4 {
        AddressV4 { bits: ip_address }
    }

    pub fn as_bytes(&self) -> [u8; 4] {
        let bytes = self.bits.to_be_bytes();
        return bytes;
    }

    pub fn as_u32(&self) -> u32 {
        return self.bits;
    }

    pub fn set_u32(&mut self, new_value: u32) {
        self.bits = new_value;
    }
}

impl Address for AddressV4 {
    fn to_string(&self) -> String {
        self.as_bytes().map(|x| x.to_string()).join(".")
    }

    fn to_bitstring(&self) -> String {
        self.as_bytes().map(|x| format!("{:0>8b}", x)).join(".")
    }

    // ! May overflow
    // ! Better approach to return result
    fn next(&self) -> Box<dyn Address> {
        Box::new(AddressV4::from_u32(self.bits + 1))
    }
    fn apply_bitmask(&self, bitmask: &super::bitmask::Bitmask) -> Box<dyn Address>
    {
        match bitmask {
            super::bitmask::Bitmask::V4(mask) => Box::new(AddressV4::from_u32(self.as_u32() & mask.get())),
            super::bitmask::Bitmask::V6(_) => panic!("Can't apply V6 bitmask to V4 address"),
        }
    }
}

impl AddressV6 {
    pub fn from_bytes(bytes: [u8; 16]) -> AddressV6 {
        AddressV6{bits: u128::from_be_bytes(bytes)}
    }
    
    // TODO: Add support of :: and 
    pub fn from_string(ip_address: &str) -> Result<AddressV6, ParseIntError> { // Only full format
        let splitted: Vec<&str> = ip_address.split(':').collect();
        let mut bytes = [0_u8; 16];
        for i in (0..16).step_by(2) {
            bytes[i] = u8::from_str_radix(&splitted[i / 2][0..2], 16)?;
            bytes[i + 1] = u8::from_str_radix(&splitted[i / 2][2..4], 16)?;
        }
        Ok(AddressV6::from_bytes(bytes))
    }
    pub fn from_u128(ip_address: u128) -> AddressV6 {
        AddressV6 { bits: ip_address }
    }

    pub fn as_bytes(&self) -> [u8; 16] {
        let bytes = self.bits.to_be_bytes();
        return bytes;
    }

    pub fn as_u128(&self) -> u128 {
        return self.bits;
    }

    pub fn set_u128(& mut self, new_value: u128) {
        self.bits = new_value;
    }
}

impl Address for AddressV6 {
    // TODO: Refactor
    fn to_string(&self) -> String {
        let bytes = self.as_bytes();
        format!("{:0<2x}", bytes[0]) + &format!("{:0<2x}", bytes[1]) + ":" +
        &format!("{:0<2x}", bytes[2]) + &format!("{:0<2x}", bytes[3]) + ":" +
        &format!("{:0<2x}", bytes[4]) + &format!("{:0<2x}", bytes[5]) + ":" +
        &format!("{:0<2x}", bytes[6]) + &format!("{:0<2x}", bytes[7]) + ":" +
        &format!("{:0<2x}", bytes[8]) + &format!("{:0<2x}", bytes[9]) + ":" +
        &format!("{:0<2x}", bytes[10]) + &format!("{:0<2x}", bytes[11]) + ":" +
        &format!("{:0<2x}", bytes[12]) + &format!("{:0<2x}", bytes[13]) + ":" +
        &format!("{:0<2x}", bytes[14]) + &format!("{:0<2x}", bytes[15])
    }

    // TODO: Refactor
    fn to_bitstring(&self) -> String {
        let bytes = self.as_bytes();
        format!("{:0>8b}", bytes[0]) + &format!("{:0>8b}", bytes[1]) + ":" +
        &format!("{:0>8b}", bytes[2]) + &format!("{:0>8b}", bytes[3]) + ":" +
        &format!("{:0>8b}", bytes[4]) + &format!("{:0>8b}", bytes[5]) + ":" +
        &format!("{:0>8b}", bytes[6]) + &format!("{:0>8b}", bytes[7]) + ":" +
        &format!("{:0>8b}", bytes[8]) + &format!("{:0>8b}", bytes[9]) + ":" +
        &format!("{:0>8b}", bytes[10]) + &format!("{:0>8b}", bytes[11]) + ":" +
        &format!("{:0>8b}", bytes[12]) + &format!("{:0>8b}", bytes[13]) + ":" +
        &format!("{:0>8b}", bytes[14]) + &format!("{:0>8b}", bytes[15])
    }

    // ! May overflow. Return Result<>
    fn next(&self) -> Box<dyn Address> {
        Box::new(AddressV6::from_u128(self.bits + 1))
    }

    fn apply_bitmask(&self, bitmask: &super::bitmask::Bitmask) -> Box<dyn Address> {
        match bitmask {
            super::bitmask::Bitmask::V4(_) => panic!("Can't apply V4 bitmask to V6 address"),
            super::bitmask::Bitmask::V6(mask) => Box::new(AddressV6::from_u128(self.as_u128() & mask.get())),
        }
    }
}


// TODO: Add tests for next method
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn address_v4_correct_from_u32() {
        let address = AddressV4::from_u32(0xbcdaf000);
        assert_eq!(address.bits, 0xbcdaf000);
        assert_eq!(address.to_string(), "188.218.240.0");
        assert_eq!(address.to_bitstring(), "10111100.11011010.11110000.00000000");
    }

    #[test]
    fn address_v4_correct_from_string() {
        let address = AddressV4::from_string("188.218.240.0");
        assert!(address.is_ok());
        let address = address.unwrap();
        assert_eq!(address.bits, 0xbcdaf000);
        assert_eq!(address.to_string(), "188.218.240.0");
        assert_eq!(address.to_bitstring(), "10111100.11011010.11110000.00000000");
    }

    #[test]
    fn address_v4_correct_from_bytes() {
        let address = AddressV4::from_bytes([0xbc, 0xda, 0xf0, 0x00]);
        assert_eq!(address.bits, 0xbcdaf000);
        assert_eq!(address.to_string(), "188.218.240.0");
        assert_eq!(address.to_bitstring(), "10111100.11011010.11110000.00000000");
    }

    #[test]
    fn address_v4_incorrect_from_string() {
        let address = AddressV4::from_string("123|123.423.432.23");
        assert!(address.is_err());
    }

    #[test]
    fn address_v6_correct_from_u128() {
        let address = AddressV6::from_u128(0xFABC1234BEEF45640000EEFD11124123);
        assert_eq!(address.bits, 0xFABC1234BEEF45640000EEFD11124123);
        assert_eq!(address.to_string(), "fabc:1234:beef:4564:0000:eefd:1112:4123");
        assert_eq!(address.to_bitstring(), "1111101010111100:0001001000110100:1011111011101111:0100010101100100:0000000000000000:1110111011111101:0001000100010010:0100000100100011");
    }

    #[test]
    fn address_v6_correct_from_string() {
        let address = AddressV6::from_string("fabc:1234:beef:4564:0000:eefd:1112:4123");
        assert!(address.is_ok());
        let address = address.unwrap();
        assert_eq!(address.bits, 0xFABC1234BEEF45640000EEFD11124123);
        assert_eq!(address.to_string(), "fabc:1234:beef:4564:0000:eefd:1112:4123");
        assert_eq!(address.to_bitstring(), "1111101010111100:0001001000110100:1011111011101111:0100010101100100:0000000000000000:1110111011111101:0001000100010010:0100000100100011");
    }

    #[test]
    fn address_v6_correct_from_bytes() {
        let address = AddressV6::from_bytes([0xFA, 0xBC, 0x12, 0x34, 0xBE, 0xEF, 0x45, 0x64, 0x00, 0x00, 0xEE, 0xFD, 0x11, 0x12, 0x41, 0x23]);
        assert_eq!(address.bits, 0xFABC1234BEEF45640000EEFD11124123);
        assert_eq!(address.to_string(), "fabc:1234:beef:4564:0000:eefd:1112:4123");
        assert_eq!(address.to_bitstring(), "1111101010111100:0001001000110100:1011111011101111:0100010101100100:0000000000000000:1110111011111101:0001000100010010:0100000100100011");
    }

    #[test]
    fn address_v6_incorrect_from_string() {
        let address = AddressV6::from_string("123|123.423.432.23");
        assert!(address.is_err());
    }
    
}