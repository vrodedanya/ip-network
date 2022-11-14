#[derive(Clone)]
pub enum Bitmask {
    V4(BitmaskV4),
    V6(BitmaskV6),
}

impl Bitmask { // Probably traits are the better way
    pub fn bits_number(&self) -> u8 {
        match self {
            Bitmask::V4(mask) => mask.bits_number,
            Bitmask::V6(mask) => mask.bits_number,
        }
    }

    pub fn to_bitstring(&self) -> String {
        match self {
            Bitmask::V4(mask) => mask.to_bitstring(),
            Bitmask::V6(mask) => mask.to_bitstring(),
        }
    }

    pub fn addresses_number(&self) -> u128 {
        match self {
            Bitmask::V4(mask) => mask.addresses_number(),
            Bitmask::V6(mask) => mask.addresses_number(),
        }
    }
}

#[derive(Clone)]
pub struct BitmaskV4 {
    mask: u32,
    bits_number: u8
}

#[derive(Clone)]
pub struct BitmaskV6 {
    mask: u128,
    bits_number: u8
}

#[derive(Debug)]
pub enum MaskError {
    WrongBitsNumber(String),
}

impl BitmaskV4 {
    pub fn new(bits_number: u8) -> Result<BitmaskV4, MaskError> {
        if bits_number >= 32 {
            return Err(MaskError::WrongBitsNumber(String::from("Mask can't be greater than 31")));
        }
        return Ok(BitmaskV4{
            mask: BitmaskV4::fill_n_bits(bits_number),
            bits_number
        })
    }

    pub fn get(&self) -> u32 {
        self.mask
    }

    pub fn bits_number(&self) -> u8 {
        self.bits_number
    }

    pub fn addresses_number(&self) -> u128 {
        let available_bits = 32 - self.bits_number;
        (2_u32.pow(available_bits.into()) - 2).into()
    }

    pub fn to_bitstring(&self) -> String {
        self.mask.to_be_bytes().map(|x|format!("{:0>8b}", x)).join(".")
    }

    fn fill_n_bits(n: u8) -> u32 {
        let mut bitmask = 0_u32;
        for i in 0..n {
            bitmask |= 0x1 << i;
        }
        return bitmask.reverse_bits();
    }
}

impl BitmaskV6 {
    pub fn new(bits_number: u8) -> Result<BitmaskV6, MaskError> {
        if bits_number >= 127 {
            return Err(MaskError::WrongBitsNumber(String::from("Mask can't be greater than 127")));
        }
        return Ok(BitmaskV6{
            mask: BitmaskV6::fill_n_bits(bits_number),
            bits_number
        })
    }

    pub fn get(&self) -> u128 {
        self.mask
    }

    pub fn bits_number(&self) -> u8 {
        self.bits_number
    }

    pub fn addresses_number(&self) -> u128 {
        let available_bits = 128 - self.bits_number;
        (2_u128.pow(available_bits.into()) - 2).into()
    }

    pub fn to_bitstring(&self) -> String {
        let bytes = self.mask.to_be_bytes();
        format!("{:0>8b}", bytes[0]) + &format!("{:0>8b}", bytes[1]) + ":" +
        &format!("{:0>8b}", bytes[2]) + &format!("{:0>8b}", bytes[3]) + ":" +
        &format!("{:0>8b}", bytes[4]) + &format!("{:0>8b}", bytes[5]) + ":" +
        &format!("{:0>8b}", bytes[6]) + &format!("{:0>8b}", bytes[7]) + ":" +
        &format!("{:0>8b}", bytes[8]) + &format!("{:0>8b}", bytes[9]) + ":" +
        &format!("{:0>8b}", bytes[10]) + &format!("{:0>8b}", bytes[11]) + ":" +
        &format!("{:0>8b}", bytes[12]) + &format!("{:0>8b}", bytes[13]) + ":" +
        &format!("{:0>8b}", bytes[14]) + &format!("{:0>8b}", bytes[15])
    }

    fn fill_n_bits(n: u8) -> u128 {
        let mut bitmask: u128 = 0;
        for i in 0..n {
            bitmask |= 0x1 << i;
        }
        return bitmask.reverse_bits();
    }
}