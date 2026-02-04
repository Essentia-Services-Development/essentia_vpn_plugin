//! VPN Plugin implementations.
//!
//! This module contains all implementations for the VPN plugin:
//! - Tunnel manager implementation
//! - Key exchange implementation
//! - Neural router implementation
//! - Plugin core implementation

mod config;
mod key_exchange;
mod plugin;
mod router;
mod tunnel;

pub use config::VpnConfig;
pub use key_exchange::PqcKeyExchange;
pub use plugin::VpnPlugin;
pub use router::NeuralRouter;
pub use tunnel::TunnelManager;
