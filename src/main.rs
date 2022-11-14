pub mod ip;

fn main() {
    let ip_network = ip::Network::new(
        ip::address::Address::V6(ip::address::AddressV6::from_string("FF45:AC32:1223:1243:AC12:DE21:41EF:54FD").expect("Will be parsed")),
        ip::bitmask::Bitmask::V6(ip::bitmask::BitmaskV6::new(125).expect("Number matched"))
    );
    println!("       Network name: {}", ip_network.name.to_string());
    println!("Binary network name: {}", ip_network.name.to_bitstring());
    println!("        Binary mask: {}", ip_network.mask.to_bitstring());
    println!("   Addresses number: {}", ip_network.addresses_number());
    println!("Available addresses:");
    for address in ip_network.get_available_addresses() {
        println!("   {}", address.to_string());
    }
/*
    let header = ip::header::V4{
        header_length: 5,
        dscp: 0x0,
        ecn: 0x2,
        packet_length: 56,
        id: 0x000f,
        dont_fragment: true,
        has_fragments: false,
        fragment_offset: 0,
        ttl: 62,
        protocol: ip::types::TransportProtocolsNumbers::Sctp,
        checksum: 0xf109,
        src_ip: ip::address::V4::from_string("23.41.23.41"),
        dst_ip: ip::address::V4::from_string("117.123.43.12")
    };
    let bytes = header.encode();
    for byte in bytes {
        print!("{:0>2x} ", byte);
    }
    println!();

    let decoded = ip::header::V4::decode(bytes);

    println!("{:?}", header);
    println!("{:?}", decoded);

    let address = ip::address::V6::from_string("2000:ABCD:1243:64CD:EFD1:AA11:5642:1111");
    println!("          IpV6: {}", address.to_string());
    println!("IpV6 as binary: {}", address.to_bitstring());
*/

}
