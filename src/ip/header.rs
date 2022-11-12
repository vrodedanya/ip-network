use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Header {
    pub version: Version,
    pub header_length: u8,
    pub dscp: u8, // Differentiated Services Code Point
    pub ecn: u8, // Explicit Congestion Notification
    pub packet_length: u16,
    pub id: u16,
    pub dont_fragment: bool,
    pub has_fragments: bool,
    pub fragment_offset: u16,
    pub ttl: u8, // Time to live
    pub protocol: TransportProtocolsNumbers,
    pub checksum: u16,
    pub src_ip: super::Address,
    pub dst_ip: super::Address
}


impl Header {
    pub fn encode(&self) -> [u8; 20] {
        let mut bytes = [0_u8; 20];
        bytes[0] = (self.version as u8) << 4 & 0xf0;
        bytes[0] |= 5;
        bytes[1] = self.dscp << 2 & 0xf6;
        bytes[1] = self.ecn & 0x03;
        let length_bytes = self.packet_length.to_be_bytes();
        bytes[2] = length_bytes[0];
        bytes[3] = length_bytes[1];
        let id_bytes = self.id.to_be_bytes();
        bytes[4] = id_bytes[0];
        bytes[5] = id_bytes[1];
        bytes[6] |= if self.dont_fragment {0x40} else {0x0} | if self.has_fragments {0x20} else {0x0};
        let offset_bytes = self.fragment_offset.to_be_bytes();
        bytes[6] |= (offset_bytes[0] >> 3)& 0x1f;
        bytes[7] = (offset_bytes[0] << 3) | offset_bytes[1] >> 3;
        bytes[8] = self.ttl;
        bytes[9] = (self.protocol as u8);
        let checksum_bytes = self.checksum.to_be_bytes();
        bytes[10] = checksum_bytes[0];
        bytes[11] = checksum_bytes[1];
        let src_ip_bytes = self.src_ip.as_bytes();
        let dst_ip_bytes = self.dst_ip.as_bytes();
        bytes[12] = src_ip_bytes[0];
        bytes[13] = src_ip_bytes[1];
        bytes[14] = src_ip_bytes[2];
        bytes[15] = src_ip_bytes[3];
        bytes[16] = dst_ip_bytes[0];
        bytes[17] = dst_ip_bytes[1];
        bytes[18] = dst_ip_bytes[2];
        bytes[19] = dst_ip_bytes[3];
        return bytes;
    }
    
    pub fn decode(&mut self, bytes: [u8; 20]) {
        self.version = if (bytes[0] >> 4 & 0x0f) == 4 {Version::IpV4} else {Version::IpV6};
        self.header_length = bytes[0] & 0x0f;
        self.dscp = bytes[1] >> 2;
        self.ecn = bytes[1] & 0x03;
        self.packet_length = u16::from_be_bytes([bytes[2], bytes[3]]);
        self.id = u16::from_be_bytes([bytes[4], bytes[5]]);
        self.dont_fragment = bytes[6] & 0x40 == 0x40;
        self.has_fragments = bytes[6] & 0x20 == 0x20;
        let offset_bytes = self.fragment_offset.to_be_bytes();
        self.fragment_offset = u16::from_be_bytes([bytes[6] << 3 | bytes[7] >> 3, offset_bytes[1] << 7]);
        self.ttl = bytes[8];
        self.protocol = FromPrimitive::from_u8(bytes[9]).unwrap();
        self.checksum = u16::from_be_bytes([bytes[10], bytes[11]]);
        self.src_ip = super::Address::from_bytes(bytes[12], bytes[13], bytes[14], bytes[15]);
        self.dst_ip = super::Address::from_bytes(bytes[16], bytes[17], bytes[18], bytes[19]);
    }
}