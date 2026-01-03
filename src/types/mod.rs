//! VPN Plugin type definitions.
//!
//! This module contains all type definitions for the VPN plugin:
//! - VPN server and tunnel types
//! - Connection state and statistics
//! - Encryption and key exchange protocols

mod core;

pub use core::{
    ConnectionStats, EncryptionAlgorithm, KeyExchangeProtocol, TunnelState, VpnServer, VpnTunnel,
};
