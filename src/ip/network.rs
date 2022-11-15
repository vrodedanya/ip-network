use super::{address::Address, bitmask::Bitmask};

pub struct Network {
    name: Box<dyn Address>,
    mask: Bitmask
}

impl Network {
    pub fn new(name: Box<dyn Address>, mask: Bitmask) -> Network {
        Network { 
            name: Network::address_to_name(name, &mask), 
            mask
        }
    }
    pub fn address_to_name(addr: Box<dyn Address>, bitmask: &Bitmask) -> Box<dyn Address> {
        return addr.apply_bitmask(bitmask);
    }

    pub fn name(&self) -> String {
        return self.name.to_string();
    }

    pub fn name_as_bits(&self) -> String {
        return self.name.to_bitstring();
    }

    pub fn mask(&self) -> String {
        return self.mask.to_bitstring();
    }

    pub fn addresses_number(&self) -> u128 {
        return self.mask.addresses_number();
    }

    pub fn to_string(&self) -> String {
        self.name.to_string() + "/" + &self.mask.bits_number().to_string()
    }

    pub fn get_available_addresses(&self) -> Vec<Box<dyn Address>> {
        let mut available_addresses: Vec<Box<dyn Address>> = Vec::new();
        let number = self.addresses_number();
        let mut current_address = self.name.next();
        for _ in 0..number {
            let next = current_address.next();
            available_addresses.push(current_address);
            current_address = next;
        }
        return available_addresses;
    }
}