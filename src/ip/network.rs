pub struct Network {
    pub name: super::Address,
    pub mask: super::Bitmask
}

impl Network {
    pub fn new(name: super::Address, mask: super::Bitmask) -> Network {
        Network { 
            name: Network::address_to_name(name, mask.get()), 
            mask
        }
    }
    pub fn address_to_name(address: super::Address, bitmask: u32) -> super::Address {
        return super::Address::from_u32(bitmask & address.as_u32())
    }

    pub fn addresses_number(&self) -> u32 {
        let available_bits = 32 - self.mask.bits_number();
        (2_u32.pow(available_bits.into()) - 2).into()
    }

    pub fn to_string(&self) -> String {
        self.name.to_string() + "/" + &self.mask.bits_number().to_string()
    }

    pub fn get_available_addresses(&self) -> Vec<super::Address> {
        let mut available_addresses: Vec<super::Address> = vec![];
        let number = self.addresses_number();
        let mut current_address = self.name.next();
        for _ in 0..number {
            available_addresses.push(super::Address::from_u32(current_address.as_u32()));
            current_address = current_address.next();
        }
        return available_addresses;
    }
}