//! Core VPN type definitions.

/// VPN server representation.
#[derive(Debug, Clone)]
pub struct VpnServer {
    /// Server identifier.
    pub id:          String,
    /// Server hostname or IP.
    pub hostname:    String,
    /// Server port.
    pub port:        u16,
    /// Server country code.
    pub country:     String,
    /// Server city.
    pub city:        String,
    /// Server load (0.0 - 1.0).
    pub load:        f32,
    /// Supports PQC.
    pub pqc_enabled: bool,
}

/// VPN tunnel representation.
#[derive(Debug, Clone)]
pub struct VpnTunnel {
    /// Tunnel identifier.
    pub id:           u64,
    /// Connected server.
    pub server:       VpnServer,
    /// Tunnel state.
    pub state:        TunnelState,
    /// Encryption algorithm.
    pub encryption:   EncryptionAlgorithm,
    /// Key exchange protocol.
    pub key_exchange: KeyExchangeProtocol,
    /// Connection statistics.
    pub stats:        ConnectionStats,
}

/// Tunnel state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TunnelState {
    /// Tunnel disconnected.
    #[default]
    Disconnected,
    /// Tunnel connecting.
    Connecting,
    /// Key exchange in progress.
    KeyExchange,
    /// Tunnel connected.
    Connected,
    /// Tunnel reconnecting.
    Reconnecting,
    /// Tunnel disconnecting.
    Disconnecting,
    /// Tunnel error.
    Error,
}

/// Connection statistics.
#[derive(Debug, Clone, Default)]
pub struct ConnectionStats {
    /// Bytes sent.
    pub bytes_sent:     u64,
    /// Bytes received.
    pub bytes_received: u64,
    /// Connection uptime (seconds).
    pub uptime_secs:    u64,
    /// Current latency (ms).
    pub latency_ms:     u32,
    /// Packets lost (percentage).
    pub packet_loss:    f32,
}

/// Encryption algorithm.
#[derive(Debug, Clone, Copy, Default)]
pub enum EncryptionAlgorithm {
    /// AES-256-GCM.
    #[default]
    Aes256Gcm,
    /// ChaCha20-Poly1305.
    ChaCha20Poly1305,
    /// AES-256-GCM with PQC hybrid.
    Aes256GcmPqc,
}

/// Key exchange protocol.
#[derive(Debug, Clone, Copy, Default)]
pub enum KeyExchangeProtocol {
    /// X25519.
    X25519,
    /// ML-KEM (post-quantum).
    #[default]
    MlKem,
    /// Hybrid X25519 + ML-KEM.
    HybridMlKem,
}
