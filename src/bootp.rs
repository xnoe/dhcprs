use eui48::MacAddress;

// Raw BOOTP Packet.
// Total size: 546 bytes
#[allow(dead_code)]
#[repr(packed)]
pub struct RawBOOTPPacket {
    // RFC951
    op: u8,           // OpCode
    htype: u8,        // Host Type
    hlen: u8,         // Host Length
    hops: u8,         // Hops
    xid: u32,         // Transaction ID
    secs: u16,        // Seconds since boot
    flags: u16,       // Unused in BOOTP, flags in DHCP
    ciaddr: u32,      // Client Address (provided by client)
    yiaddr: u32,      // Client Address (provided by server)
    siaddr: u32,      // Server Address
    giaddr: u32,      // Gateway Address
    chaddr: [u8; 16], // Client Hardware (MAC) Address
    sname: [u8; 64],  // Server hostname, null terminated string
    file: [u8; 128],  // Boot file name, null terminated string
    vend: [u8; 312], // Vendor specific data. In RFC2131, this is required to be 312 octets for DHCP.
}

impl RawBOOTPPacket {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, 546) }
    }
}

pub enum OpCode {
    BOOTREQUEST,
    BOOTREPLY,
}

impl TryFrom<u8> for OpCode {
    type Error = ();
    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            1 => Ok(Self::BOOTREQUEST),
            2 => Ok(Self::BOOTREPLY),
            _ => Err(()),
        }
    }
}

impl TryFrom<OpCode> for u8 {
    type Error = ();
    fn try_from(item: OpCode) -> Result<Self, Self::Error> {
        match item {
            OpCode::BOOTREQUEST => Ok(1),
            OpCode::BOOTREPLY => Ok(2),
        }
    }
}

// Higher level BOOTP Packet representation
pub struct BOOTPPacket {
    pub op: OpCode,
    pub hops: u8,
    pub xid: u32,
    pub secs: u16,
    pub flags: u16,
    pub ciaddr: Option<std::net::Ipv4Addr>,
    pub yiaddr: Option<std::net::Ipv4Addr>,
    pub siaddr: Option<std::net::Ipv4Addr>,
    pub giaddr: Option<std::net::Ipv4Addr>,
    pub chaddr: MacAddress,
    pub sname: [u8; 64],
    pub file: [u8; 128],
    vend: [u8; 312],
}

impl From<BOOTPPacket> for RawBOOTPPacket {
    fn from(item: BOOTPPacket) -> Self {
        let mut chaddr: [u8; 16] = [0; 16];
        chaddr[..6].copy_from_slice(item.chaddr.as_bytes());
        Self {
            op: item.op.try_into().unwrap(),
            htype: 1,
            hlen: 6,
            hops: item.hops,
            xid: item.xid,
            secs: item.secs,
            flags: item.flags,
            ciaddr: if item.ciaddr.is_some() {
                u32::to_be(item.ciaddr.unwrap().into())
            } else {
                0
            },
            yiaddr: if item.yiaddr.is_some() {
                u32::to_be(item.yiaddr.unwrap().into())
            } else {
                0
            },
            siaddr: if item.siaddr.is_some() {
                u32::to_be(item.siaddr.unwrap().into())
            } else {
                0
            },
            giaddr: if item.giaddr.is_some() {
                u32::to_be(item.giaddr.unwrap().into())
            } else {
                0
            },
            chaddr: chaddr,
            sname: item.sname,
            file: item.file,
            vend: item.vend,
        }
    }
}

impl From<RawBOOTPPacket> for BOOTPPacket {
    fn from(item: RawBOOTPPacket) -> Self {
        Self {
            op: item.op.try_into().unwrap(),
            hops: item.hops,
            xid: item.xid,
            secs: item.secs,
            flags: item.flags,
            ciaddr: if item.ciaddr == 0 {
                None
            } else {
                Some(std::net::Ipv4Addr::from(u32::from_be(item.ciaddr)))
            },
            yiaddr: if item.yiaddr == 0 {
                None
            } else {
                Some(std::net::Ipv4Addr::from(u32::from_be(item.yiaddr)))
            },
            siaddr: if item.siaddr == 0 {
                None
            } else {
                Some(std::net::Ipv4Addr::from(u32::from_be(item.siaddr)))
            },
            giaddr: if item.giaddr == 0 {
                None
            } else {
                Some(std::net::Ipv4Addr::from(u32::from_be(item.giaddr)))
            },
            chaddr: MacAddress::from_bytes(&item.chaddr[0..6]).unwrap(),
            sname: item.sname,
            file: item.file,
            vend: item.vend,
        }
    }
}

impl BOOTPPacket {
    /// Create a new BOOTP packet.
    ///
    /// dhcprs encourages the use of higher level interfaces which are then
    /// converted to lower level (packed) structures that can be serialised
    /// in to bytes and then sent down a socket.
    pub fn new(
        op: OpCode,
        hops: u8,
        xid: u32,
        secs: u16,
        flags: u16,
        ciaddr: Option<std::net::Ipv4Addr>,
        yiaddr: Option<std::net::Ipv4Addr>,
        siaddr: Option<std::net::Ipv4Addr>,
        giaddr: Option<std::net::Ipv4Addr>,
        chaddr: MacAddress,
        sname: [u8; 64],
        file: [u8; 128],
        vend: [u8; 312],
    ) -> Self {
        Self {
            op: op,
            hops: hops,
            xid: xid,
            secs: secs,
            flags: flags,
            ciaddr: ciaddr,
            yiaddr: yiaddr,
            siaddr: siaddr,
            giaddr: giaddr,
            chaddr: chaddr,
            sname: sname,
            file: file,
            vend: vend,
        }
    }

    pub fn get_vend(&self) -> &[u8] {
        return &self.vend;
    }
}
