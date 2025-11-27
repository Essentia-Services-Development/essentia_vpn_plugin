//! # Essentia VPN Plugin
//!
//! Quantum-safe neural tunnels with PQC encryption for the Essentia platform.
//!
//! ## Features
//!
//! - Post-quantum cryptographic key exchange (ML-KEM)
//! - Neural network-optimized routing
//! - Consciousness-integrated traffic patterns
//! - Multi-hop tunnel support
//! - Kill switch and leak protection
//!
//! ## FlexForge Integration
//!
//! The VPN plugin integrates with FlexForge via `VpnPluginFlexForge`:
//!
//! ```ignore
//! use essentia_vpn_plugin::flexforge::VpnPluginFlexForge;
//!
//! let plugin = VpnPluginFlexForge::new();
//! let panel_info = plugin.panel_info();
//! ```
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                       VPN Plugin                             │
//! ├─────────────────────────────────────────────────────────────┤
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
//! │  │    PQC      │  │   Tunnel    │  │   Neural Router     │  │
//! │  │  Key Mgmt   │  │   Manager   │  │                     │  │
//! │  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
//! │         │                │                     │             │
//! │         ▼                ▼                     ▼             │
//! │  ┌─────────────────────────────────────────────────────┐    │
//! │  │              Secure Transport Layer                  │    │
//! │  │        (essentia_pqc + essentia_net_plugin)          │    │
//! │  └─────────────────────────────────────────────────────┘    │
//! └─────────────────────────────────────────────────────────────┘
//! ```

mod config;
mod errors;
pub mod flexforge;
mod key_exchange;
mod plugin;
mod router;
mod tunnel;
mod types;

pub use config::VpnConfig;
pub use errors::{VpnError, VpnResult};
pub use flexforge::{ConnectionState, VpnPluginFlexForge, VpnUiConfig};
pub use key_exchange::PqcKeyExchange;
pub use plugin::VpnPlugin;
pub use router::NeuralRouter;
pub use tunnel::TunnelManager;
pub use types::{
    ConnectionStats, EncryptionAlgorithm, KeyExchangeProtocol, TunnelState, VpnServer, VpnTunnel,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let config = VpnConfig::default();
        assert!(config.kill_switch);
    }
}
