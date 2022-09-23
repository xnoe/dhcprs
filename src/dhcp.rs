use std::net::Ipv4Addr;

/// Enum representing the parameter for DHCP Option 53 "Message Type".
#[derive(Debug)]
pub enum DHCPMessageType {
    // RFC1533
    DHCPDiscover,
    DHCPOffer,
    DHCPRequest,
    DHCPDecline,
    DHCPACK,
    DHCPNAK,
    DHCPRelease,

    // RFC2132
    DHCPInform,
}

impl TryFrom<u8> for DHCPMessageType {
    type Error = ();
    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            1 => Ok(DHCPMessageType::DHCPDiscover),
            2 => Ok(DHCPMessageType::DHCPOffer),
            3 => Ok(DHCPMessageType::DHCPRequest),
            4 => Ok(DHCPMessageType::DHCPDecline),
            5 => Ok(DHCPMessageType::DHCPACK),
            6 => Ok(DHCPMessageType::DHCPNAK),
            7 => Ok(DHCPMessageType::DHCPRelease),
            8 => Ok(DHCPMessageType::DHCPInform),
            _ => Err(()),
        }
    }
}

impl From<DHCPMessageType> for u8 {
    fn from(item: DHCPMessageType) -> Self {
        match item {
            DHCPMessageType::DHCPDiscover => 1,
            DHCPMessageType::DHCPOffer => 2,
            DHCPMessageType::DHCPRequest => 3,
            DHCPMessageType::DHCPDecline => 4,
            DHCPMessageType::DHCPACK => 5,
            DHCPMessageType::DHCPNAK => 6,
            DHCPMessageType::DHCPRelease => 7,
            DHCPMessageType::DHCPInform => 8,
        }
    }
}

/// Enum representing the parameter for DHCP Option 46 "NetBIOS over TCP/IP Node Type".
#[derive(Debug)]
pub enum NetBIOSoverTCPIPNodeType {
    Bnode,
    Pnode,
    Mnode,
    Hnode,
}

impl TryFrom<u8> for NetBIOSoverTCPIPNodeType {
    type Error = ();
    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            1 => Ok(NetBIOSoverTCPIPNodeType::Bnode),
            2 => Ok(NetBIOSoverTCPIPNodeType::Pnode),
            4 => Ok(NetBIOSoverTCPIPNodeType::Mnode),
            8 => Ok(NetBIOSoverTCPIPNodeType::Hnode),
            _ => Err(()),
        }
    }
}

impl From<NetBIOSoverTCPIPNodeType> for u8 {
    fn from(item: NetBIOSoverTCPIPNodeType) -> Self {
        match item {
            NetBIOSoverTCPIPNodeType::Bnode => 1,
            NetBIOSoverTCPIPNodeType::Pnode => 2,
            NetBIOSoverTCPIPNodeType::Mnode => 4,
            NetBIOSoverTCPIPNodeType::Hnode => 8,
        }
    }
}

/// This enum represents all the DHCP options supported by dhcprs
#[derive(Debug)]
pub enum DHCPOption {
    // RFC1533
    Pad,                                                       // 0
    End,                                                       // 255
    SubnetMask(Ipv4Addr),                                      // 1 4 m1 m2 m3 m4
    TimeOffset(u32),                                           // 2 4 n1 n2 n3 n4
    Router(Vec<Ipv4Addr>),                                     // 3 n a1 a2 a3 a4 a1 a2 ...
    TimeServer(Vec<Ipv4Addr>),                                 // 4 n a1 a2 a3 a4 a1 a2 ...
    NameServer(Vec<Ipv4Addr>),                                 // 5 n a1 a2 a3 a4 a1 a2 ...
    DomainNameServer(Vec<Ipv4Addr>),                           // 6 n a1 a2 a3 a4 a1 a2 ...
    LogServer(Vec<Ipv4Addr>),                                  // 7 n a1 a2 a3 a4 a1 a2 ...
    CookieServer(Vec<Ipv4Addr>),                               // 8 n a1 a2 a3 a4 a1 a2 ...
    LPRServer(Vec<Ipv4Addr>),                                  // 9 n a1 a2 a3 a4 a1 a2 ...
    ImpressServer(Vec<Ipv4Addr>),                              // 10 n a1 a2 a3 a4 a1 a2 ...
    ResourceLocationServer(Vec<Ipv4Addr>),                     // 11 n a1 a2 a3 a4 a1 a2 ...
    HostName(String),                                          // 12 n h1 h2 h3 h4 h5 h6 ...
    BootfileSize(u16),                                         // 13 2 l1 l2
    MeritDumpFile(String),                                     // 14 n n1 n2 n3 n4 ...
    DomainName(String),                                        // 15 n d1 d2 d3 d4 ...
    SwapServer(Ipv4Addr),                                      // 16 4 a1 a2 a3 a4
    RootPath(String),                                          // 17 n n1 n2 n3 n4 ...
    ExtensionsPath(String),                                    // 18 n n1 n2 n3 n4 ...
    IPForwarding(bool),                                        // 19 1 0/1
    NonLocalSourceRouting(bool),                               // 20 1 0/1
    PolicyFilter(Vec<(Ipv4Addr, Ipv4Addr)>), // 21 n a1 a2 a3 a4 m1 m2 m3 m4 a1 a2 a3 a4 m1 m2 m3 m4 ...
    MaximumDatagramReassemblySize(u16),      // 22 2 s1 s2
    DefaultIPTTL(u8),                        // 23 1 ttl
    PathMTUAgingTimeout(u32),                // 24 4 t1 t2 t3 t4
    PathMTUPlateaus(Vec<u16>),               // 25 n s1 s2 s1 s2 ...
    InterfaceMTU(u16),                       // 26 2 m1 m2
    AllSubnetsLocal(bool),                   // 27 1 0/1
    BroadcastAddress(Ipv4Addr),              // 28 4 b1 b2 b3 b4
    PerformMaskDiscovery(bool),              // 29 1 0/1
    MaskSupplier(bool),                      // 30 1 0/1
    PerformRouterDiscovery(bool),            // 31 1 0/1
    RouterSolicitationAddress(Ipv4Addr),     // 32 4 a1 a2 a3 a4
    StaticRoutes(Vec<(Ipv4Addr, Ipv4Addr)>), // 33 n d1 d2 d3 d4 r1 r2 r3 r4 d1 d2 d3 d4 r1 r2 r3 r4 ...
    TrailerEncapsultion(bool),               // 34 1 0/1
    ARPCacheTimeout(u32),                    // 35 4 t1 t2 t3 t4
    EthernetEncapsulation(bool),             // 36 1 0/1
    TCPDefaultTTL(u8),                       // 37 1 n
    TCPKeepaliveInterval(u32),               // 38 4 t1 t2 t3 t4
    TCPKeepaliveGarbage(bool),               // 39 1 0/1
    NISDomain(String),                       // 40 n n1 n2 n3 n4 ...
    NetworkInformationServers(Vec<Ipv4Addr>), // 41 n a1 a2 a3 a4 a1 a2 a3 a4 ...
    NTPServers(Vec<Ipv4Addr>),               // 42 n a1 a2 a3 a4 a1 a2 a3 a4 ...
    VendorSpecificInformation(Vec<u8>),      // 43 n i1 i2 ...
    NetBIOSoverTCPIPNameServer(Vec<Ipv4Addr>), // 44 n a1 a2 a3 a4 b1 b2 b3 b4 ...
    NetBIOSoverTCPIPDatagramDistributionServer(Vec<Ipv4Addr>), // 45 n a1 a2 a3 a4 b1 b2 b3 b4 ...
    NetBIOSoverTCPIPNodeType(NetBIOSoverTCPIPNodeType), // 46 1 n
    NetBIOSoverTCPIPScope(Vec<u8>),          // 47 n s1 s2 s3 s4 ...
    XWindowSystemFontServer(Vec<Ipv4Addr>),  // 48 n a1 a2 a3 a4 a1 a2 ...
    XWindowSystemDisplayManager(Vec<Ipv4Addr>), // 49 n a2 a3 a4 a4 a1 a2 ...
    RequestIPAddress(Ipv4Addr),              // 50 4 a1 a2 a3 a4
    IPAddressLeaseTime(u32),                 // 51 4 t1 t2 t3 t4
    OptionOverload((bool, bool)), // 52 1 1/2/3 (1 -> file overloaded, 2 -> sname overloaded, 3 -> both overloaded)
    DHCPMessageType(DHCPMessageType), // 53 1 n
    ServerIdentifier(Ipv4Addr),   // 54 4 a1 a2 a3 a4
    ParameterRequest(Vec<u8>),    // 55 n c1 c2 ...
    Message(String),              // 56 n c1 c2 ...
    MaximumDHCPMessageSize(u16),  // 57 2 l1 l2
    RenewalTime(u32),             // 58 4 t1 t2 t3 t4
    RebindingTime(u32),           // 59 4 t1 t2 t3 t4
    ClassIdentifier(Vec<u8>),     // 60 n i1 i2 ...
    ClientIdentifier(Vec<u8>),    // 61 n t1 i1 i2 ...

    // RFC2132
    NISPlusDomain(String),              // 64 n n1 n2 n3 n4 ...
    NISPlusServers(Vec<Ipv4Addr>),      // 65 n a1 a2 a3 a4 a1 a2 ...
    TFTPServerName(String),             // 66 n c1 c2 c3 ...
    BootfileName(String),               // 67 n c1 c2 c3 ...
    MobileIPHomeAgent(Vec<Ipv4Addr>),   // 68 n a1 a2 a3 a4 a1 a2 ...
    SMTPServer(Vec<Ipv4Addr>),          // 69 n a1 a2 a3 a4 a1 a2 ...
    POP3Server(Vec<Ipv4Addr>),          // 70 n a1 a2 a3 a4 a1 a2 ...
    NNTPServer(Vec<Ipv4Addr>),          // 71 n a1 a2 a3 a4 a1 a2 ...
    DefaultWWWServer(Vec<Ipv4Addr>),    // 72 n a1 a2 a3 a4 a1 a2 ...
    DefaultFingerServer(Vec<Ipv4Addr>), // 73 n a1 a2 a3 a4 a1 a2 ...
    DefaultIRCServer(Vec<Ipv4Addr>),    // 74 n a1 a2 a3 a4 a1 a2 ...
    StreetTalkServer(Vec<Ipv4Addr>),    // 75 n a1 a2 a3 a4 a1 a2 ...
    STDAServer(Vec<Ipv4Addr>),          // 76 n a1 a2 a3 a4 a1 a2 ...

    /*// RFC4833
    TimezonePOSIX(String), // 100 N IEEE 1003.1 String
    TimezoneDB(String),    // 101 N Reference to TZ Database

    // RFC3442
    ClasslessStaticRoute(Vec<(Ipv4Addr, u32, Ipv4Addr)>), // 121 n d1 ... dN r1 r2 r3 r4 d1 ... dN r1 r2 r3 r4*/

    // Catchall
    Option(u8, Vec<u8>),
}

macro_rules! break_unwrap {
    ($e:expr) => {
        match $e {
            Some(t) => *t,
            _ => break,
        }
    };
}

impl DHCPOption {
    pub fn from_bytes(bytes: &[u8]) -> Vec<DHCPOption> {
        let mut options: Vec<DHCPOption> = Vec::new();

        let mut iterator = bytes.iter();
        loop {
            match break_unwrap!(iterator.next()) {
                0 => continue,
                255 => break,
                1 => {
                    break_unwrap!(iterator.next());

                    let (m1, m2, m3, m4) = (
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                    );
                    options.push(DHCPOption::SubnetMask(Ipv4Addr::new(m1, m2, m3, m4)));
                }

                2 => {
                    break_unwrap!(iterator.next());
                    let mut total: u32 = 0;
                    for _ in 0..4 {
                        total <<= 8;
                        total += break_unwrap!(iterator.next()) as u32;
                    }

                    options.push(DHCPOption::TimeOffset(total));
                }

                3 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::Router(addresses));
                }

                4 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::TimeServer(addresses));
                }

                5 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::NameServer(addresses));
                }

                6 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::DomainNameServer(addresses));
                }

                7 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::LogServer(addresses));
                }

                8 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::CookieServer(addresses));
                }

                9 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::LPRServer(addresses));
                }

                10 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::ImpressServer(addresses));
                }

                11 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::ResourceLocationServer(addresses));
                }

                12 => {
                    let count = break_unwrap!(iterator.next());
                    let mut chars: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        chars.push(break_unwrap!(iterator.next()));
                    }

                    // If data is invalid just silently fail and act as if the option didn't exist.
                    match std::str::from_utf8(&chars) {
                        Ok(s) => options.push(DHCPOption::HostName(s.to_owned())),
                        Err(_) => (),
                    }
                }

                13 => {
                    break_unwrap!(iterator.next());
                    let mut total: u16 = 0;
                    for _ in 0..2 {
                        total <<= 8;
                        total += break_unwrap!(iterator.next()) as u16;
                    }

                    options.push(DHCPOption::BootfileSize(total));
                }

                14 => {
                    let count = break_unwrap!(iterator.next());
                    let mut chars: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        chars.push(break_unwrap!(iterator.next()));
                    }

                    // If data is invalid just silently fail and act as if the option didn't exist.
                    match std::str::from_utf8(&chars) {
                        Ok(s) => options.push(DHCPOption::MeritDumpFile(s.to_owned())),
                        Err(_) => (),
                    }
                }

                15 => {
                    let count = break_unwrap!(iterator.next());
                    let mut chars: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        chars.push(break_unwrap!(iterator.next()));
                    }

                    // If data is invalid just silently fail and act as if the option didn't exist.
                    match std::str::from_utf8(&chars) {
                        Ok(s) => options.push(DHCPOption::DomainName(s.to_owned())),
                        Err(_) => (),
                    }
                }

                16 => {
                    break_unwrap!(iterator.next());

                    let (a1, a2, a3, a4) = (
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                    );
                    options.push(DHCPOption::SwapServer(Ipv4Addr::new(a1, a2, a3, a4)));
                }

                17 => {
                    let count = break_unwrap!(iterator.next());
                    let mut chars: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        chars.push(break_unwrap!(iterator.next()));
                    }

                    // If data is invalid just silently fail and act as if the option didn't exist.
                    match std::str::from_utf8(&chars) {
                        Ok(s) => options.push(DHCPOption::RootPath(s.to_owned())),
                        Err(_) => (),
                    }
                }

                18 => {
                    let count = break_unwrap!(iterator.next());
                    let mut chars: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        chars.push(break_unwrap!(iterator.next()));
                    }

                    // If data is invalid just silently fail and act as if the option didn't exist.
                    match std::str::from_utf8(&chars) {
                        Ok(s) => options.push(DHCPOption::ExtensionsPath(s.to_owned())),
                        Err(_) => (),
                    }
                }

                19 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::IPForwarding(
                        if break_unwrap!(iterator.next()) == 0 {
                            false
                        } else {
                            true
                        },
                    ));
                }

                20 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::NonLocalSourceRouting(
                        if break_unwrap!(iterator.next()) == 0 {
                            false
                        } else {
                            true
                        },
                    ));
                }

                21 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<(Ipv4Addr, Ipv4Addr)> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        let (m1, m2, m3, m4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses
                            .push((Ipv4Addr::new(a1, a2, a3, a4), Ipv4Addr::new(m1, m2, m3, m4)));
                    }

                    options.push(DHCPOption::PolicyFilter(addresses));
                }

                22 => {
                    break_unwrap!(iterator.next());
                    let mut total: u16 = 0;
                    for _ in 0..2 {
                        total <<= 8;
                        total += break_unwrap!(iterator.next()) as u16;
                    }

                    options.push(DHCPOption::MaximumDatagramReassemblySize(total));
                }

                23 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::DefaultIPTTL(break_unwrap!(iterator.next())));
                }

                24 => {
                    break_unwrap!(iterator.next());
                    let mut total: u32 = 0;
                    for _ in 0..4 {
                        total <<= 8;
                        total += break_unwrap!(iterator.next()) as u32;
                    }

                    options.push(DHCPOption::PathMTUAgingTimeout(total));
                }

                25 => {
                    let count = break_unwrap!(iterator.next()) / 2;
                    let mut plateaus: Vec<u16> = Vec::new();

                    for _ in 0..count {
                        let mut total: u16 = 0;
                        for _ in 0..2 {
                            total <<= 8;
                            total += break_unwrap!(iterator.next()) as u16;
                        }
                        plateaus.push(total);
                    }

                    options.push(DHCPOption::PathMTUPlateaus(plateaus));
                }

                26 => {
                    break_unwrap!(iterator.next());
                    let mut total: u16 = 0;
                    for _ in 0..2 {
                        total <<= 8;
                        total += break_unwrap!(iterator.next()) as u16;
                    }

                    options.push(DHCPOption::InterfaceMTU(total));
                }

                27 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::AllSubnetsLocal(
                        if break_unwrap!(iterator.next()) == 0 {
                            false
                        } else {
                            true
                        },
                    ));
                }

                28 => {
                    break_unwrap!(iterator.next());

                    let (a1, a2, a3, a4) = (
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                    );
                    options.push(DHCPOption::BroadcastAddress(Ipv4Addr::new(a1, a2, a3, a4)));
                }

                29 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::PerformMaskDiscovery(
                        if break_unwrap!(iterator.next()) == 0 {
                            false
                        } else {
                            true
                        },
                    ));
                }

                30 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::MaskSupplier(
                        if break_unwrap!(iterator.next()) == 0 {
                            false
                        } else {
                            true
                        },
                    ));
                }

                31 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::PerformRouterDiscovery(
                        if break_unwrap!(iterator.next()) == 0 {
                            false
                        } else {
                            true
                        },
                    ));
                }

                32 => {
                    break_unwrap!(iterator.next());

                    let (a1, a2, a3, a4) = (
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                    );
                    options.push(DHCPOption::RouterSolicitationAddress(Ipv4Addr::new(
                        a1, a2, a3, a4,
                    )));
                }

                33 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<(Ipv4Addr, Ipv4Addr)> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        let (r1, r2, r3, r4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses
                            .push((Ipv4Addr::new(a1, a2, a3, a4), Ipv4Addr::new(r1, r2, r3, r4)));
                    }

                    options.push(DHCPOption::StaticRoutes(addresses));
                }

                34 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::TrailerEncapsultion(
                        if break_unwrap!(iterator.next()) == 0 {
                            false
                        } else {
                            true
                        },
                    ));
                }

                35 => {
                    break_unwrap!(iterator.next());
                    let mut total: u32 = 0;
                    for _ in 0..4 {
                        total <<= 8;
                        total += break_unwrap!(iterator.next()) as u32;
                    }

                    options.push(DHCPOption::ARPCacheTimeout(total));
                }

                37 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::EthernetEncapsulation(
                        if break_unwrap!(iterator.next()) == 0 {
                            false
                        } else {
                            true
                        },
                    ));
                }

                38 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::DefaultIPTTL(break_unwrap!(iterator.next())));
                }

                39 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::TCPKeepaliveGarbage(
                        if break_unwrap!(iterator.next()) == 0 {
                            false
                        } else {
                            true
                        },
                    ));
                }

                40 => {
                    let count = break_unwrap!(iterator.next());
                    let mut chars: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        chars.push(break_unwrap!(iterator.next()));
                    }

                    // If data is invalid just silently fail and act as if the option didn't exist.
                    match std::str::from_utf8(&chars) {
                        Ok(s) => options.push(DHCPOption::NISDomain(s.to_owned())),
                        Err(_) => (),
                    }
                }

                41 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::NetworkInformationServers(addresses));
                }

                42 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::NTPServers(addresses));
                }

                43 => {
                    let count = break_unwrap!(iterator.next());
                    let mut vendor_information: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        vendor_information.push(break_unwrap!(iterator.next()));
                    }

                    options.push(DHCPOption::VendorSpecificInformation(vendor_information));
                }

                44 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::NetBIOSoverTCPIPNameServer(addresses));
                }

                45 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::NetBIOSoverTCPIPDatagramDistributionServer(
                        addresses,
                    ));
                }

                46 => {
                    break_unwrap!(iterator.next());

                    let node_type: NetBIOSoverTCPIPNodeType =
                        match break_unwrap!(iterator.next()).try_into() {
                            Ok(n) => n,
                            _ => continue,
                        };

                    options.push(DHCPOption::NetBIOSoverTCPIPNodeType(node_type));
                }

                47 => {
                    let count = break_unwrap!(iterator.next());
                    let mut bytes: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        bytes.push(break_unwrap!(iterator.next()));
                    }

                    options.push(DHCPOption::NetBIOSoverTCPIPScope(bytes));
                }

                48 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::XWindowSystemFontServer(addresses));
                }

                49 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::XWindowSystemDisplayManager(addresses));
                }

                50 => {
                    break_unwrap!(iterator.next());

                    let (a1, a2, a3, a4) = (
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                    );
                    options.push(DHCPOption::RequestIPAddress(Ipv4Addr::new(a1, a2, a3, a4)));
                }

                51 => {
                    break_unwrap!(iterator.next());
                    let mut total: u32 = 0;
                    for _ in 0..4 {
                        total <<= 8;
                        total += break_unwrap!(iterator.next()) as u32;
                    }

                    options.push(DHCPOption::IPAddressLeaseTime(total));
                }

                52 => {
                    break_unwrap!(iterator.next());

                    options.push(DHCPOption::OptionOverload(
                        match break_unwrap!(iterator.next()) {
                            1 => (true, false),
                            2 => (false, true),
                            3 => (true, true),
                            _ => continue,
                        },
                    ));
                }

                53 => {
                    break_unwrap!(iterator.next());

                    let node_type: DHCPMessageType = match break_unwrap!(iterator.next()).try_into()
                    {
                        Ok(n) => n,
                        _ => continue,
                    };

                    options.push(DHCPOption::DHCPMessageType(node_type));
                }

                54 => {
                    break_unwrap!(iterator.next());

                    let (a1, a2, a3, a4) = (
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                        break_unwrap!(iterator.next()),
                    );
                    options.push(DHCPOption::ServerIdentifier(Ipv4Addr::new(a1, a2, a3, a4)));
                }

                55 => {
                    let count = break_unwrap!(iterator.next());
                    let mut bytes: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        bytes.push(break_unwrap!(iterator.next()));
                    }

                    options.push(DHCPOption::ParameterRequest(bytes));
                }

                56 => {
                    let count = break_unwrap!(iterator.next());
                    let mut chars: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        chars.push(break_unwrap!(iterator.next()));
                    }

                    // If data is invalid just silently fail and act as if the option didn't exist.
                    match std::str::from_utf8(&chars) {
                        Ok(s) => options.push(DHCPOption::Message(s.to_owned())),
                        Err(_) => (),
                    }
                }

                57 => {
                    break_unwrap!(iterator.next());
                    let mut total: u16 = 0;
                    for _ in 0..2 {
                        total <<= 8;
                        total += break_unwrap!(iterator.next()) as u16;
                    }

                    options.push(DHCPOption::MaximumDHCPMessageSize(total));
                }

                58 => {
                    break_unwrap!(iterator.next());
                    let mut total: u32 = 0;
                    for _ in 0..4 {
                        total <<= 8;
                        total += break_unwrap!(iterator.next()) as u32;
                    }

                    options.push(DHCPOption::RenewalTime(total));
                }

                59 => {
                    break_unwrap!(iterator.next());
                    let mut total: u32 = 0;
                    for _ in 0..4 {
                        total <<= 8;
                        total += break_unwrap!(iterator.next()) as u32;
                    }

                    options.push(DHCPOption::RebindingTime(total));
                }

                60 => {
                    let count = break_unwrap!(iterator.next());
                    let mut bytes: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        bytes.push(break_unwrap!(iterator.next()));
                    }

                    options.push(DHCPOption::ClassIdentifier(bytes));
                }

                61 => {
                    let count = break_unwrap!(iterator.next());
                    let mut bytes: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        bytes.push(break_unwrap!(iterator.next()));
                    }

                    options.push(DHCPOption::ClientIdentifier(bytes));
                }

                64 => {
                    let count = break_unwrap!(iterator.next());
                    let mut chars: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        chars.push(break_unwrap!(iterator.next()));
                    }

                    // If data is invalid just silently fail and act as if the option didn't exist.
                    match std::str::from_utf8(&chars) {
                        Ok(s) => options.push(DHCPOption::NISPlusDomain(s.to_owned())),
                        Err(_) => (),
                    }
                }

                65 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::NISPlusServers(addresses));
                }

                66 => {
                    let count = break_unwrap!(iterator.next());
                    let mut chars: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        chars.push(break_unwrap!(iterator.next()));
                    }

                    // If data is invalid just silently fail and act as if the option didn't exist.
                    match std::str::from_utf8(&chars) {
                        Ok(s) => options.push(DHCPOption::TFTPServerName(s.to_owned())),
                        Err(_) => (),
                    }
                }

                67 => {
                    let count = break_unwrap!(iterator.next());
                    let mut chars: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        chars.push(break_unwrap!(iterator.next()));
                    }

                    // If data is invalid just silently fail and act as if the option didn't exist.
                    match std::str::from_utf8(&chars) {
                        Ok(s) => options.push(DHCPOption::BootfileName(s.to_owned())),
                        Err(_) => (),
                    }
                }

                68 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::MobileIPHomeAgent(addresses));
                }

                69 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::SMTPServer(addresses));
                }

                70 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::POP3Server(addresses));
                }

                71 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::NNTPServer(addresses));
                }

                72 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::DefaultWWWServer(addresses));
                }

                73 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::DefaultFingerServer(addresses));
                }

                74 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::DefaultIRCServer(addresses));
                }

                75 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::StreetTalkServer(addresses));
                }

                76 => {
                    let count = break_unwrap!(iterator.next()) / 4;
                    let mut addresses: Vec<Ipv4Addr> = Vec::new();

                    for _ in 0..count {
                        let (a1, a2, a3, a4) = (
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                            break_unwrap!(iterator.next()),
                        );
                        addresses.push(Ipv4Addr::new(a1, a2, a3, a4));
                    }

                    options.push(DHCPOption::STDAServer(addresses));
                }

                // Catchall for if we cannot decode the option to a specific enum variant.
                n => {
                    let count = break_unwrap!(iterator.next());

                    let mut bytes: Vec<u8> = Vec::new();

                    for _ in 0..count {
                        bytes.push(break_unwrap!(iterator.next()));
                    }

                    options.push(DHCPOption::Option(n, bytes));
                }
            }
        }

        return options;
    }

    pub fn to_bytes(options: Vec<DHCPOption>) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(0x63);
        bytes.push(0x82);
        bytes.push(0x53);
        bytes.push(0x63);

        for option in options {
            match option {
                DHCPOption::Pad => bytes.push(0),
                DHCPOption::End => bytes.push(255),

                DHCPOption::SubnetMask(addr) => {
                    bytes.push(1);
                    bytes.push(4);
                    bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                }

                DHCPOption::TimeOffset(x) => {
                    bytes.push(2);
                    bytes.push(4);
                    bytes.extend_from_slice(&x.to_be_bytes());
                }

                DHCPOption::Router(addrs) => {
                    bytes.push(3);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::TimeServer(addrs) => {
                    bytes.push(4);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::NameServer(addrs) => {
                    bytes.push(5);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::DomainNameServer(addrs) => {
                    bytes.push(6);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::LogServer(addrs) => {
                    bytes.push(7);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::CookieServer(addrs) => {
                    bytes.push(8);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::LPRServer(addrs) => {
                    bytes.push(9);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::ImpressServer(addrs) => {
                    bytes.push(10);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::ResourceLocationServer(addrs) => {
                    bytes.push(11);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::HostName(s) => {
                    bytes.push(12);
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }

                DHCPOption::BootfileSize(l) => {
                    bytes.push(13);
                    bytes.push(2);
                    bytes.extend_from_slice(&(l as u16).to_be_bytes());
                }

                DHCPOption::MeritDumpFile(s) => {
                    bytes.push(14);
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }

                DHCPOption::DomainName(s) => {
                    bytes.push(15);
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }

                DHCPOption::SwapServer(addr) => {
                    bytes.push(16);
                    bytes.push(4);
                    bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                }

                DHCPOption::RootPath(s) => {
                    bytes.push(17);
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }

                DHCPOption::ExtensionsPath(s) => {
                    bytes.push(18);
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }

                DHCPOption::IPForwarding(b) => {
                    bytes.push(19);
                    bytes.push(1);
                    bytes.push(b as u8);
                }

                DHCPOption::NonLocalSourceRouting(b) => {
                    bytes.push(20);
                    bytes.push(1);
                    bytes.push(b as u8);
                }

                DHCPOption::PolicyFilter(addrs) => {
                    bytes.push(21);
                    let count = addrs.len() * 8;
                    bytes.push(count as u8);
                    for (addr, mask) in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                        bytes.extend_from_slice(&(u32::from(mask)).to_be_bytes());
                    }
                }

                DHCPOption::MaximumDatagramReassemblySize(l) => {
                    bytes.push(22);
                    bytes.push(2);
                    bytes.extend_from_slice(&l.to_be_bytes());
                }

                DHCPOption::DefaultIPTTL(l) => {
                    bytes.push(23);
                    bytes.push(1);
                    bytes.push(l);
                }

                DHCPOption::PathMTUAgingTimeout(l) => {
                    bytes.push(24);
                    bytes.push(2);
                    bytes.extend_from_slice(&l.to_be_bytes());
                }

                DHCPOption::PathMTUPlateaus(plateaus) => {
                    bytes.push(25);
                    bytes.push((plateaus.len() * 2) as u8);
                    for plateau in plateaus {
                        bytes.extend_from_slice(&plateau.to_be_bytes());
                    }
                }

                DHCPOption::InterfaceMTU(l) => {
                    bytes.push(26);
                    bytes.push(2);
                    bytes.extend_from_slice(&l.to_be_bytes());
                }

                DHCPOption::AllSubnetsLocal(b) => {
                    bytes.push(27);
                    bytes.push(1);
                    bytes.push(b as u8);
                }

                DHCPOption::BroadcastAddress(addr) => {
                    bytes.push(28);
                    bytes.push(4);
                    bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                }

                DHCPOption::PerformMaskDiscovery(b) => {
                    bytes.push(29);
                    bytes.push(1);
                    bytes.push(b as u8);
                }

                DHCPOption::MaskSupplier(b) => {
                    bytes.push(30);
                    bytes.push(1);
                    bytes.push(b as u8);
                }

                DHCPOption::PerformRouterDiscovery(b) => {
                    bytes.push(31);
                    bytes.push(1);
                    bytes.push(b as u8);
                }

                DHCPOption::RouterSolicitationAddress(addr) => {
                    bytes.push(32);
                    bytes.push(4);
                    bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                }

                DHCPOption::StaticRoutes(addrs) => {
                    bytes.push(33);
                    let count = addrs.len() * 8;
                    bytes.push(count as u8);
                    for (addr, router) in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                        bytes.extend_from_slice(&(u32::from(router)).to_be_bytes());
                    }
                }

                DHCPOption::TrailerEncapsultion(b) => {
                    bytes.push(34);
                    bytes.push(1);
                    bytes.push(b as u8);
                }

                DHCPOption::ARPCacheTimeout(l) => {
                    bytes.push(35);
                    bytes.push(4);
                    bytes.extend_from_slice(&l.to_be_bytes());
                }

                DHCPOption::EthernetEncapsulation(b) => {
                    bytes.push(36);
                    bytes.push(1);
                    bytes.push(b as u8);
                }

                DHCPOption::TCPDefaultTTL(l) => {
                    bytes.push(37);
                    bytes.push(1);
                    bytes.push(l);
                }

                DHCPOption::TCPKeepaliveInterval(l) => {
                    bytes.push(38);
                    bytes.push(4);
                    bytes.extend_from_slice(&l.to_be_bytes());
                }

                DHCPOption::TCPKeepaliveGarbage(b) => {
                    bytes.push(39);
                    bytes.push(1);
                    bytes.push(b as u8);
                }

                DHCPOption::NISDomain(s) => {
                    bytes.push(40);
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }

                DHCPOption::NetworkInformationServers(addrs) => {
                    bytes.push(41);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::NTPServers(addrs) => {
                    bytes.push(42);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::VendorSpecificInformation(b) => {
                    bytes.push(43);
                    bytes.push(b.len() as u8);
                    bytes.extend_from_slice(&b);
                }

                DHCPOption::NetBIOSoverTCPIPNameServer(addrs) => {
                    bytes.push(44);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::NetBIOSoverTCPIPDatagramDistributionServer(addrs) => {
                    bytes.push(45);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::NetBIOSoverTCPIPNodeType(n) => {
                    bytes.push(46);
                    bytes.push(1);
                    bytes.push(n.into());
                }

                DHCPOption::NetBIOSoverTCPIPScope(b) => {
                    bytes.push(47);
                    bytes.push(b.len() as u8);
                    bytes.extend_from_slice(&b);
                }

                DHCPOption::XWindowSystemFontServer(addrs) => {
                    bytes.push(48);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::XWindowSystemDisplayManager(addrs) => {
                    bytes.push(49);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::RequestIPAddress(addr) => {
                    bytes.push(50);
                    bytes.push(4);
                    bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                }

                DHCPOption::IPAddressLeaseTime(l) => {
                    bytes.push(51);
                    bytes.push(4);
                    bytes.extend_from_slice(&l.to_be_bytes());
                }

                DHCPOption::OptionOverload(b) => {
                    let v = match b {
                        (true, false) => 1,
                        (false, true) => 2,
                        (true, true) => 3,
                        _ => continue,
                    };

                    bytes.push(52);
                    bytes.push(1);
                    bytes.push(v);
                }

                DHCPOption::DHCPMessageType(t) => {
                    bytes.push(53);
                    bytes.push(1);
                    bytes.push(t.into());
                }

                DHCPOption::ServerIdentifier(addr) => {
                    bytes.push(54);
                    bytes.push(4);
                    bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                }

                DHCPOption::ParameterRequest(b) => {
                    bytes.push(55);
                    bytes.push(b.len() as u8);
                    bytes.extend_from_slice(&b);
                }

                DHCPOption::Message(s) => {
                    bytes.push(56);
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }

                DHCPOption::MaximumDHCPMessageSize(l) => {
                    bytes.push(57);
                    bytes.push(2);
                    bytes.extend_from_slice(&l.to_be_bytes());
                }

                DHCPOption::RenewalTime(l) => {
                    bytes.push(58);
                    bytes.push(4);
                    bytes.extend_from_slice(&l.to_be_bytes());
                }

                DHCPOption::RebindingTime(l) => {
                    bytes.push(59);
                    bytes.push(4);
                    bytes.extend_from_slice(&l.to_be_bytes());
                }

                DHCPOption::ClassIdentifier(b) => {
                    bytes.push(60);
                    bytes.push(b.len() as u8);
                    bytes.extend_from_slice(&b);
                }

                DHCPOption::ClientIdentifier(b) => {
                    bytes.push(61);
                    bytes.push(b.len() as u8);
                    bytes.extend_from_slice(&b);
                }

                DHCPOption::NISPlusDomain(s) => {
                    bytes.push(64);
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }

                DHCPOption::NISPlusServers(addrs) => {
                    bytes.push(65);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::TFTPServerName(s) => {
                    bytes.push(66);
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }

                DHCPOption::BootfileName(s) => {
                    bytes.push(67);
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }

                DHCPOption::MobileIPHomeAgent(addrs) => {
                    bytes.push(68);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::SMTPServer(addrs) => {
                    bytes.push(69);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::POP3Server(addrs) => {
                    bytes.push(70);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::NNTPServer(addrs) => {
                    bytes.push(71);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::DefaultWWWServer(addrs) => {
                    bytes.push(72);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::DefaultFingerServer(addrs) => {
                    bytes.push(73);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::DefaultIRCServer(addrs) => {
                    bytes.push(74);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::StreetTalkServer(addrs) => {
                    bytes.push(75);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::STDAServer(addrs) => {
                    bytes.push(76);
                    let count = addrs.len() * 4;
                    bytes.push(count as u8);
                    for addr in addrs {
                        bytes.extend_from_slice(&(u32::from(addr)).to_be_bytes());
                    }
                }

                DHCPOption::Option(n, b) => {
                    bytes.push(n);
                    bytes.push(b.len() as u8);
                    bytes.extend_from_slice(&b);
                }
            }
        }

        return bytes;
    }
}
