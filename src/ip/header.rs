use num_traits::FromPrimitive;

#[derive(Debug)]
pub enum Header {
    V4(HeaderV4),
    V6(HeaderV6)
}

#[derive(Debug)]
pub enum HeaderError {
    EmptyPacker,
    InvalidPacket
}

pub fn get_version(bytes: &Vec<u8>) -> Result<super::Version, HeaderError> {
    if bytes.len() == 0 {
        return Err(HeaderError::EmptyPacker);
    }
    if bytes[0] & 0xf0 == 0x40 {
        return Ok(super::Version::IpV4);
    }
    else {
        return Ok(super::Version::IpV6);
    }
}

pub fn decode(bytes: &Vec<u8>) -> Result<Header, HeaderError> {
    if get_version(&bytes)? == super::Version::IpV4 {
        Ok(Header::V4(HeaderV4::decode(&bytes)?))
    }
    else {
        Ok(Header::V6(HeaderV6::decode(&bytes)?))
    }
}

#[derive(Debug)]
pub struct HeaderV4 {
    pub header_length: u8,
    pub dscp: u8, // Differentiated Services Code Point
    pub ecn: u8, // Explicit Congestion Notification
    pub packet_length: u16,
    pub id: u16,
    pub dont_fragment: bool,
    pub has_fragments: bool,
    pub fragment_offset: u16,
    pub ttl: u8, // Time to live
    pub protocol: super::TransportProtocolsNumbers,
    pub checksum: u16,
    pub src_ip: super::address::AddressV4,
    pub dst_ip: super::address::AddressV4
}

#[derive(Debug)]
pub struct HeaderV6 {
    pub dscp: u8, // Differentiated Services Code Point
    pub ecn: u8, // Explicit Congestion Notification
    pub flow_label: u32,
    pub payload_length: u16,
    pub next_header: super::TransportProtocolsNumbers,
    pub hop_limit: u8,
    pub source_address: super::address::AddressV6,
    pub destination_address: super::address::AddressV6,
}


impl HeaderV4 {
    pub fn empty() -> HeaderV4 {
        HeaderV4 {
            header_length: 0,
            dscp: 0x0,
            ecn: 0x0,
            packet_length: 0,
            id: 0x0000,
            dont_fragment: false,
            has_fragments: false,
            fragment_offset: 0,
            ttl: 0,
            protocol: super::TransportProtocolsNumbers::Tcp,
            checksum: 0x0000,
            src_ip: super::address::AddressV4::from_u32(0),
            dst_ip: super::address::AddressV4::from_u32(0)
        }
    }
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![0; 20];
        bytes.resize(20, 0);
        bytes[0] = 0x40;
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
        bytes[6] |= offset_bytes[0]& 0x1f;
        bytes[7] = offset_bytes[1];
        bytes[8] = self.ttl;
        bytes[9] = self.protocol as u8;
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
    
    pub fn decode(bytes: &Vec<u8>) -> Result<HeaderV4, HeaderError> {
        if bytes.len() != 20 {
            return Err(HeaderError::InvalidPacket);
        }
        let mut header = HeaderV4::empty();
        header.header_length = bytes[0] & 0x0f;
        header.dscp = bytes[1] >> 2;
        header.ecn = bytes[1] & 0x03;
        header.packet_length = u16::from_be_bytes([bytes[2], bytes[3]]);
        header.id = u16::from_be_bytes([bytes[4], bytes[5]]);
        header.dont_fragment = bytes[6] & 0x40 == 0x40;
        header.has_fragments = bytes[6] & 0x20 == 0x20;
        header.fragment_offset = u16::from_be_bytes([bytes[6] & 0x1f, bytes[7]]);
        header.ttl = bytes[8];
        header.protocol = FromPrimitive::from_u8(bytes[9]).unwrap();
        header.checksum = u16::from_be_bytes([bytes[10], bytes[11]]);
        header.src_ip = super::address::AddressV4::from_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
        header.dst_ip = super::address::AddressV4::from_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        return Ok(header);
    }
}

impl HeaderV6 {
    pub fn empty() -> HeaderV6 {
        return HeaderV6 {
            dscp: 0,
            ecn: 0,
            flow_label: 0,
            payload_length: 0,
            next_header: super::TransportProtocolsNumbers::Tcp,
            hop_limit: 0,
            source_address: super::address::AddressV6::from_u128(0),
            destination_address: super::address::AddressV6::from_u128(0)
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(0x60 | self.dscp >> 2);
        let flow_label_bytes = self.flow_label.to_be_bytes();
        bytes.push((self.dscp << 6 | self.ecn << 4) & 0xf0 | flow_label_bytes[1] & 0x0f);
        bytes.push(flow_label_bytes[2]);
        bytes.push(flow_label_bytes[3]);
        let payload_length = self.payload_length.to_be_bytes();
        bytes.push(payload_length[0]);
        bytes.push(payload_length[1]);
        bytes.push(self.next_header as u8);
        bytes.push(self.hop_limit);
        let src_bytes = self.source_address.as_bytes();
        bytes.append(&mut src_bytes.to_vec());
        let dst_bytes = self.destination_address.as_bytes();
        bytes.append(&mut dst_bytes.to_vec());
        return bytes;
    }

    pub fn decode(bytes: &Vec<u8>) -> Result<HeaderV6, HeaderError> {
        if bytes.len() != 40 {
            return Err(HeaderError::InvalidPacket);
        }
        let mut header = HeaderV6::empty();
        header.dscp = (bytes[0] & 0x0f) << 2 | bytes[1] >> 6;
        header.ecn = bytes[1] >> 4 & 0x03;
        header.flow_label = u32::from_be_bytes([0, bytes[1] & 0x0f, bytes[2], bytes[3]]);
        header.payload_length = u16::from_be_bytes([bytes[4], bytes[5]]);
        header.next_header = FromPrimitive::from_u8(bytes[6]).unwrap() ;
        header.hop_limit = bytes[7];
        let mut arr = [0u8; 16];
        arr.copy_from_slice(&bytes[8..24]);
        header.source_address.set_u128(u128::from_be_bytes(arr));
        arr.copy_from_slice(&bytes[24..]);
        header.destination_address.set_u128(u128::from_be_bytes(arr));
        return Ok(header)
    }
}