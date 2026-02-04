//! VPN plugin configuration.

use crate::types::{EncryptionAlgorithm, KeyExchangeProtocol};

/// Configuration for the VPN plugin.
#[derive(Debug, Clone)]
pub struct VpnConfig {
    /// Enable kill switch.
    pub kill_switch:            bool,
    /// Enable DNS leak protection.
    pub dns_leak_protection:    bool,
    /// Preferred encryption algorithm.
    pub encryption:             EncryptionAlgorithm,
    /// Preferred key exchange protocol.
    pub key_exchange:           KeyExchangeProtocol,
    /// Auto-reconnect on disconnect.
    pub auto_reconnect:         bool,
    /// Maximum reconnect attempts.
    pub max_reconnect_attempts: u32,
    /// Reconnect delay (seconds).
    pub reconnect_delay_secs:   u64,
    /// Enable split tunneling.
    pub split_tunneling:        bool,
}

impl Default for VpnConfig {
    fn default() -> Self {
        Self {
            kill_switch:            true,
            dns_leak_protection:    true,
            encryption:             EncryptionAlgorithm::Aes256GcmPqc,
            key_exchange:           KeyExchangeProtocol::HybridMlKem,
            auto_reconnect:         true,
            max_reconnect_attempts: 5,
            reconnect_delay_secs:   5,
            split_tunneling:        false,
        }
    }
}
