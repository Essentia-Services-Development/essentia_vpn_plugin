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

mod types;
mod errors;
mod config;
mod key_exchange;
mod tunnel;
mod router;
mod plugin;

pub use types::{
    VpnServer, VpnTunnel, TunnelState, ConnectionStats,
    EncryptionAlgorithm, KeyExchangeProtocol,
};
pub use errors::{VpnError, VpnResult};
pub use config::VpnConfig;
pub use key_exchange::PqcKeyExchange;
pub use tunnel::TunnelManager;
pub use router::NeuralRouter;
pub use plugin::VpnPlugin;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let config = VpnConfig::default();
        assert!(config.kill_switch);
    }
}
