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
}
