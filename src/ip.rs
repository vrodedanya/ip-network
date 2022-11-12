pub struct Address{
    bits: u32
}

pub struct Bitmask {
    mask: u32,
    bits_number: u8
}

pub struct Network {
    pub name: Address,
    pub mask: Bitmask
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
        return bytes
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

impl Network {
    pub fn new(name: Address, mask: Bitmask) -> Network {
        Network { 
            name: Network::address_to_name(name, mask.get()), 
            mask
        }
    }
    pub fn address_to_name(address: Address, bitmask: u32) -> Address {
        return Address::from_u32(bitmask & address.bits)
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