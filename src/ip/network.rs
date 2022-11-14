pub struct Network {
    pub name: super::address::Address,
    pub mask: super::bitmask::Bitmask
}

impl Network {
    pub fn new(name: super::address::Address, mask: super::bitmask::Bitmask) -> Network {
        Network { 
            name: Network::address_to_name(name, mask.clone()), 
            mask
        }
    }
    pub fn address_to_name(addr: super::address::Address, bitmask: super::bitmask::Bitmask) -> super::address::Address {
        use super::address::*;
        use super::bitmask::*;

        match addr {
            Address::V4(address) => {
                let bitmask_v4 = match bitmask {
                    Bitmask::V4(bitmask) => bitmask,
                    Bitmask::V6(_) => panic!("Can't use V6 bitmask for AddressV4"),
                };
                Address::V4(super::address::AddressV4::from_u32(address.as_u32() & bitmask_v4.get()))
            }
            Address::V6(address) => 
            {
                let bitmask_v6 = match bitmask {
                    Bitmask::V6(bitmask) => bitmask,
                    Bitmask::V4(_) => panic!("Can't use V4 bitmask for AddressV6"),
                };
                Address::V6(super::address::AddressV6::from_u128(address.as_u128() & bitmask_v6.get()))
            },
        }
    }

    pub fn addresses_number(&self) -> u128 {
        return self.mask.addresses_number();
    }

    pub fn to_string(&self) -> String {
        self.name.to_string() + "/" + &self.mask.bits_number().to_string()
    }

    pub fn get_available_addresses(&self) -> Vec<super::address::Address> {
        let mut available_addresses: Vec<super::address::Address> = vec![];
        let number = self.addresses_number();
        let mut current_address = self.name.next();
        for _ in 0..number {
            available_addresses.push(current_address);
            current_address = current_address.next().clone();
        }
        return available_addresses;
    }
}