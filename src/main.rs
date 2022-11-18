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
    let header = ip::header::HeaderV6{
        dscp: 0x0,
        ecn: 0x2,
        flow_label: 0x12345,
        payload_length: 1234,
        next_header: ip::types::TransportProtocolsNumbers::Sctp,
        hop_limit: 16,
        source_address: AddressV6::from_string("feed:1234:abcd:5345:6576:1234:abcd:abcd").unwrap(),
        destination_address: AddressV6::from_string("12ed:6666:4543:2344:beef:1234:1234:abcd").unwrap()
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
