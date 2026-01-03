//! VPN error type definitions.

use core::fmt;

/// VPN operation errors.
#[derive(Debug)]
pub enum VpnError {
    /// Connection error.
    Connection(String),
    /// Key exchange error.
    KeyExchange(String),
    /// Tunnel error.
    Tunnel(String),
    /// Authentication error.
    Authentication(String),
    /// Configuration error.
    Configuration(String),
    /// Network error.
    Network(String),
}

impl fmt::Display for VpnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connection(msg) => write!(f, "Connection error: {msg}"),
            Self::KeyExchange(msg) => write!(f, "Key exchange error: {msg}"),
            Self::Tunnel(msg) => write!(f, "Tunnel error: {msg}"),
            Self::Authentication(msg) => write!(f, "Authentication error: {msg}"),
            Self::Configuration(msg) => write!(f, "Configuration error: {msg}"),
            Self::Network(msg) => write!(f, "Network error: {msg}"),
        }
    }
}

impl std::error::Error for VpnError {}

/// Result type for VPN operations.
pub type VpnResult<T> = Result<T, VpnError>;
