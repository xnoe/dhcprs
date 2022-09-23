//! DHCP/BOOTP packet encode/decode library.
//!
//! The purpose of this library is to provide an easy to use interface over
//! DHCP, BOOTP and UDP packets to enable the creation of programs that
//! make use of DHCP.
//!
//! BOOTP specific functionality is provided by the `dhcprs::bootp` module
//! DHCP specific functionality is provided by the `dhcprs::dhcp` module
//!
//! There is additionally simple UDP packet encode/decode provided by the
//! `dhcprs::udpbuilder`.

pub mod bootp;
pub mod dhcp;
pub mod udpbuilder;
