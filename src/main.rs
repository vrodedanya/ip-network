pub mod ip;

fn main() {
    use ip::{address::*, bitmask::*};
    let ip_network = ip::Network::new(
        Box::new(AddressV4::from_string("192.168.0.0").unwrap()),
        Bitmask::V4(BitmaskV4::new(28).unwrap())
    );
    println!("       Network name: {}", ip_network.name());
    println!("Binary network name: {}", ip_network.name_as_bits());
    println!("        Binary mask: {}", ip_network.mask());
    println!("   Addresses number: {}", ip_network.addresses_number());
    println!("Available addresses:");
    for address in ip_network.get_available_addresses() {
        println!("   {}", address.to_string());
    }

    let header = ip::header::HeaderV4{
        header_length: 5,
        dscp: 0x0,
        ecn: 0x2,
        packet_length: 56,
        id: 0x000f,
        dont_fragment: true,
        has_fragments: false,
        fragment_offset: 10,
        ttl: 62,
        protocol: ip::types::TransportProtocolsNumbers::Sctp,
        checksum: 0xf109,
        src_ip: AddressV4::from_string("23.41.23.41").unwrap(),
        dst_ip: AddressV4::from_string("117.123.43.12").unwrap()
    };
    let bytes = header.encode();
    for byte in &bytes {
        print!("{:0>2x} ", byte);
    }
    println!();

    let decoded = ip::header::decode(&bytes).unwrap();

    println!("{:?}", header);
    println!("{:?}", decoded);
}
