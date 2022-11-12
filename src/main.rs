pub mod ip;

fn main() {
    let ip_network = ip::Network::new(
        ip::Address::from_string("192.168.110.0"),
        ip::Bitmask::new(28)
    );
    println!("       Network name: {}", ip_network.name.to_string());
    println!("Binary network name: {}", ip_network.name.to_bitstring());
    println!("        Binary mask: {}", ip_network.mask.to_bitstring());
    println!("   Addresses number: {}", ip_network.addresses_number());
    println!("Available addresses:");
    for address in ip_network.get_available_addresses() {
        println!("   {}", address.to_string());
    }

    let header = ip::header::Header{
        version: ip::header::Version::IpV4,
        header_length: 5,
        dscp: 0x0,
        ecn: 0x2,
        packet_length: 56,
        id: 0x000f,
        dont_fragment: true,
        has_fragments: false,
        fragment_offset: 0,
        ttl: 62,
        protocol: ip::header::TransportProtocolsNumbers::Sctp,
        checksum: 0xf109,
        src_ip: ip::Address::from_string("23.41.23.41"),
        dst_ip: ip::Address::from_string("117.123.43.12")
    };
    let bytes = header.encode();
    for byte in bytes {
        print!("{:0>2x} ", byte);
    }
    println!();

    let mut decoded = ip::header::Header{version: ip::header::Version::IpV4,
        header_length: 0,
        dscp: 0x0,
        ecn: 0x0,
        packet_length: 0,
        id: 0x0000,
        dont_fragment: true,
        has_fragments: false,
        fragment_offset: 0,
        ttl: 0,
        protocol: ip::header::TransportProtocolsNumbers::Tcp,
        checksum: 0x0000,
        src_ip: ip::Address::from_string("123.12.32.21"),
        dst_ip: ip::Address::from_string("12.32.43.12")};
    decoded.decode(bytes);

    println!("{:?}", header);
    println!("{:?}", decoded);
}
