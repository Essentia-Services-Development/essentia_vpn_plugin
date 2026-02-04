//! Core VPN traits.

use crate::{
    errors::VpnResult,
    types::{ConnectionStats, TunnelState, VpnServer, VpnTunnel},
};

/// Trait for VPN tunnel providers.
pub trait TunnelProvider: Send + Sync {
    /// Creates a new tunnel to the specified server.
    fn create_tunnel(&self, server: &VpnServer) -> VpnResult<VpnTunnel>;

    /// Gets the current state of a tunnel.
    fn tunnel_state(&self, tunnel_id: u64) -> VpnResult<TunnelState>;

    /// Gets connection statistics for a tunnel.
    fn tunnel_stats(&self, tunnel_id: u64) -> VpnResult<ConnectionStats>;

    /// Destroys a tunnel.
    fn destroy_tunnel(&self, tunnel_id: u64) -> VpnResult<()>;
}

/// Trait for VPN connections.
pub trait VpnConnection: Send + Sync {
    /// Connects to a VPN server.
    fn connect(&mut self, server: &VpnServer) -> VpnResult<()>;

    /// Disconnects from the current server.
    fn disconnect(&mut self) -> VpnResult<()>;

    /// Checks if currently connected.
    fn is_connected(&self) -> bool;

    /// Gets the current connection state.
    fn connection_state(&self) -> TunnelState;
}
