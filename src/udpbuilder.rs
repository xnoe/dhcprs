#[allow(dead_code)]
#[repr(packed)]
pub struct RawUDPPacket {
    // IP Header
    version_ihl: u8, // First 4 bits: Version (always 4), next 4 bits: Internet Header Length (Header size in u32s)
    dscp_ecn: u8, // Differentiated Services Code Point / Explicit Congestion Notification, zero for our purposes
    total_length: u16, // Total length of the IP header + UDP header + data
    identification: u16, // Identification (for fragmentation purposes)
    flags_fragment_offset: u16, // Flags and the fragment offset (0x4000 for DF)
    ttl: u8,      // Time to live
    protocol: u8, // Proto (UDP is 0x11 / dec 17)
    ip_checksum: u16, // Ones' complement of the ones' complement sum of all 16 bit words of the header
    source_addr: u32, // Source address
    dest_addr: u32,   // Destination address

    // UDP Header
    source_port: u16,  // Source port
    dest_port: u16,    // Destination port
    length: u16,       // Length of UDP Header + data
    udp_checksum: u16, // Ones' complement of the ones' complement sum of all 16 bit words of the pseudoheader + udp header + data

    // UDP Data
    data: [u8; 546], // 546 byte array to store a RawBOOTPPacket
}

impl RawUDPPacket {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, 574) }
    }
}

impl From<UDPPacket> for RawUDPPacket {
    fn from(item: UDPPacket) -> Self {
        let ph: PseudoHeader = item.clone().into();
        let udp_checksum = ph.checksum();

        let source_addr: u32 = item.source_addr.into();
        let dest_addr: u32 = item.dest_addr.into();

        let mut packet = Self {
            version_ihl: 0x45,             // Version 4, IHL of 5 (20 bytes)
            dscp_ecn: 0,                   // No need for either DSCP or ECN
            total_length: 574_u16.to_be(), // Will always be 576 bytes
            identification: 0x1337,        // No need (no fragmentation)
            flags_fragment_offset: 0,
            ttl: 64,        // Set TTL to 255 because why not
            protocol: 17,   // UDP
            ip_checksum: 0, // Zero for now, will be set later
            source_addr: source_addr.to_be(),
            dest_addr: dest_addr.to_be(),
            source_port: item.source_port.to_be(),
            dest_port: item.dest_port.to_be(),
            length: ((item.data.len() + 8) as u16).to_be(),
            udp_checksum: udp_checksum,
            data: item.data,
        };

        let header_u16 =
            unsafe { std::slice::from_raw_parts(&packet as *const Self as *const u16, 10) };
        let mut total: u16 = 0;
        for &word in header_u16 {
            let (value, carry) = total.overflowing_add(word);
            if carry {
                total = value + 1;
            } else {
                total = value;
            }
        }

        packet.ip_checksum = !total;
        return packet;
    }
}

#[allow(dead_code)]
#[repr(packed)]
struct PseudoHeader {
    source_addr: u32,
    dest_addr: u32,
    zero: u8,
    protocol: u8,
    udp_length: u16,
    source_port: u16,
    dest_port: u16,
    udp_length2: u16,
    udp_checksum: u16,
    data: [u8; 546],
}

impl From<UDPPacket> for PseudoHeader {
    fn from(item: UDPPacket) -> Self {
        let source_addr: u32 = item.source_addr.into();
        let dest_addr: u32 = item.dest_addr.into();
        Self {
            source_addr: source_addr.to_be(),
            dest_addr: dest_addr.to_be(),
            zero: 0,
            protocol: 17,
            udp_length: ((item.data.len() + 8) as u16).to_be(),
            source_port: item.source_port.to_be(),
            dest_port: item.dest_port.to_be(),
            udp_length2: ((item.data.len() + 8) as u16).to_be(),
            udp_checksum: 0,
            data: item.data,
        }
    }
}

impl PseudoHeader {
    pub fn checksum(&self) -> u16 {
        let as_u16 = unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u16,
                (self.data.len() + 20) / 2,
            )
        };
        let mut total: u16 = 0;
        for &word in as_u16 {
            let (value, carry) = total.overflowing_add(word);
            if carry {
                total = value + 1;
            } else {
                total = value;
            }
        }
        !total
    }
}

#[derive(Clone)]
pub struct UDPPacket {
    pub source_addr: std::net::Ipv4Addr,
    pub dest_addr: std::net::Ipv4Addr,
    pub source_port: u16,
    pub dest_port: u16,
    data: [u8; 546],
}

impl From<RawUDPPacket> for UDPPacket {
    fn from(item: RawUDPPacket) -> Self {
        Self {
            source_addr: std::net::Ipv4Addr::from(item.source_addr.to_be_bytes()),
            dest_addr: std::net::Ipv4Addr::from(item.source_addr.to_be_bytes()),
            source_port: u16::from_be(item.source_port),
            dest_port: u16::from_be(item.dest_port),
            data: item.data,
        }
    }
}

impl UDPPacket {
    /// Creates a new UDPPacket
    ///
    /// Should be converted to a `dhcprs::udpbuilder::RawUDPPacket` to send.
    pub fn new(
        source_addr: std::net::Ipv4Addr,
        dest_addr: std::net::Ipv4Addr,
        source_port: u16,
        dest_port: u16,
        data: [u8; 546],
    ) -> Self {
        Self {
            source_addr: source_addr,
            dest_addr: dest_addr,
            source_port: source_port,
            dest_port: dest_port,
            data: data,
        }
    }

    /// Returns a reference to the payload of the UDP packet.
    pub fn get_data(&self) -> &[u8] {
        return &self.data;
    }
}
