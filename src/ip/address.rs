use std::{num::ParseIntError};

#[derive(Clone, Copy)]
pub enum Address {
    V4(AddressV4),
    V6(AddressV6),
}

impl Address {
    pub fn to_string(&self) -> String {
        match self {
            Address::V4(address) => address.to_string(),
            Address::V6(address) => address.to_string(),
        }
    }
    pub fn next(&self) -> Address {
        match self {
            Address::V4(address) => Address::V4(address.next()),
            Address::V6(address) => Address::V6(address.next()),
        }
    }
    
    pub fn to_bitstring(&self) -> String {
        match self {
            Address::V4(address) => address.to_bitstring(),
            Address::V6(address) => address.to_bitstring(),
        }
    }
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
            splitted[3].parse()?]))
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

    pub fn to_string(&self) -> String {
        self.as_bytes().map(|x|x.to_string()).join(".")
    }

    pub fn to_bitstring(&self) -> String {
        self.as_bytes().map(|x|format!("{:0>8b}", x)).join(".")
    }

    pub fn next(&self) -> AddressV4 {
        AddressV4::from_u32(self.bits + 1)
    }
}

impl AddressV6 {
    pub fn from_bytes(bytes: [u8; 16]) -> AddressV6 {
        AddressV6{bits: u128::from_be_bytes(bytes)}
    }
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

    pub fn to_string(&self) -> String {
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

    pub fn to_bitstring(&self) -> String {
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

    pub fn next(&self) -> AddressV6 {
        AddressV6::from_u128(self.bits + 1)
    }
}