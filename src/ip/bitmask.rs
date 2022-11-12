pub struct Bitmask {
    mask: u32,
    bits_number: u8
}

impl Bitmask {
    pub fn new(bits_number: u8) -> Bitmask {
        if bits_number >= 32 {
            panic!("Mask can't be greater than 31");
        }
        Bitmask{
            mask: Bitmask::fill_n_bits(bits_number),
            bits_number: bits_number
        }
    }

    pub fn get(&self) -> u32 {
        self.mask
    }

    pub fn bits_number(&self) -> u8 {
        self.bits_number
    } 

    pub fn to_bitstring(&self) -> String {
        let bytes = self.mask.to_be_bytes();
        format!("{:0>8b}", bytes[0]) + "." + &format!("{:0>8b}", bytes[1]) + "." +  &format!("{:0>8b}", bytes[2]) + "." +  &format!("{:0>8b}", bytes[3])
    }

    fn fill_n_bits(n: u8) -> u32 {
        let mut bitmask = 0;
        for i in 0..n {
            bitmask |= 0x80000000 >> i;
        }
        return bitmask;
    }
}