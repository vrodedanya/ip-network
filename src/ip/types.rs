use num_derive::FromPrimitive;    

#[derive(Debug, PartialEq)]
pub enum Version {
    IpV4 = 4,
    IpV6 = 6,
}

#[derive(FromPrimitive)]
#[derive(Debug)]
pub enum TransportProtocolsNumbers {
    Tcp = 6,
    Udp = 17,
    Sctp = 132
}