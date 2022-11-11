pub struct Address(
    pub u8, 
    pub u8, 
    pub u8, 
    pub u8
);

pub struct Bitmask {
    mask: u32,
    bits_number: u8
}

pub struct Network {
    pub name: Address,
    pub mask: Bitmask
}

impl Address {
    pub fn new(byte1: u8, byte2: u8, byte3: u8, byte4: u8) -> Address {
        Address(byte1, byte2, byte3, byte4)
    }
    pub fn from_string(ip_address: &str) -> Address {
        let splitted: Vec<&str> = ip_address.split('.').collect();
        Address(splitted[0].parse().unwrap(), splitted[1].parse().unwrap(), splitted[2].parse().unwrap(), splitted[3].parse().unwrap())
    }
    pub fn from_u32(ip_address: u32) -> Address {
        let bytes = ip_address.to_be_bytes();
        Address(bytes[0], bytes[1], bytes[2], bytes[3])
    }

    pub fn to_string(&self) -> String {
        self.0.to_string() + "." + &self.1.to_string() + "." + &self.2.to_string() + "." + &self.3.to_string()
    }

    pub fn to_bitstring(&self) -> String {
        format!("{:0>8b}", self.0) + "." + &format!("{:0>8b}", self.1) + "." +  &format!("{:0>8b}", self.2) + "." +  &format!("{:0>8b}", self.3)
    }

    pub fn next(&self) -> Address {
        let asu32 = u32::from_be_bytes([self.0, self.1, self.2, self.3]);
        Address::from_u32(asu32 + 1)
    }
}

impl Bitmask {
    pub fn new(mask: u8) -> Bitmask {
        if mask >= 32 {
            panic!("Mask can't be greater than 31");
        }
        Bitmask{
            mask: Bitmask::fill_n_bits(mask),
            bits_number: mask
        }
    }

    pub fn get(&self) -> u32 {
        self.mask
    }

    pub fn bits_number(&self) -> u8 {
        self.bits_number
    } 

    pub fn to_bitstring(&self) -> String {
        let bytes = self.mask.reverse_bits().to_be_bytes();
        format!("{:0>8b}", bytes[0]) + "." + &format!("{:0>8b}", bytes[1]) + "." +  &format!("{:0>8b}", bytes[2]) + "." +  &format!("{:0>8b}", bytes[3])
    }

    fn fill_n_bits(n: u8) -> u32 {
        let mut bitmask = 0;
        for i in 0..n {
            bitmask |= 1 << i;
        }
        return bitmask;
    }
}

impl Network {
    pub fn new(name: Address, mask: Bitmask) -> Network {
        Network { 
            name: Network::address_to_name(name, mask.get()), 
            mask
        }
    }
    pub fn address_to_name(address: Address, bitmask: u32) -> Address {
        let bytes = bitmask.reverse_bits().to_be_bytes();
        return Address(
            address.0 & bytes[0], 
            address.1 & bytes[1], 
            address.2 & bytes[2], 
            address.3 & bytes[3]
        )
    }

    pub fn addresses_number(&self) -> u32 {
        let available_bits = 32 - self.mask.bits_number;
        (2_u32.pow(available_bits.into()) - 2).into()
    }

    pub fn to_string(&self) -> String {
        self.name.to_string() + "/" + &self.mask.bits_number().to_string()
    }

    pub fn get_available_addresses(&self) -> Vec<Address> {
        let mut available_addresses: Vec<Address> = vec![];
        let number = self.addresses_number();
        let mut current_address = self.name.next();
        for _ in 0..number {
            available_addresses.push(Address{..current_address});
            current_address = current_address.next();
        }
        return available_addresses;
    }
}