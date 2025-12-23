//! Tunnel management.

use crate::{
    errors::{VpnError, VpnResult},
    types::{ConnectionStats, TunnelState, VpnServer, VpnTunnel},
};

/// Tunnel manager for VPN connections.
pub struct TunnelManager {
    active_tunnel:  Option<VpnTunnel>,
    next_tunnel_id: u64,
}

impl TunnelManager {
    /// Create a new tunnel manager.
    pub fn new() -> Self {
        Self { active_tunnel: None, next_tunnel_id: 1 }
    }

    /// Create a tunnel to server.
    pub fn create_tunnel(&mut self, server: VpnServer) -> VpnResult<u64> {
        if self.active_tunnel.is_some() {
            return Err(VpnError::Tunnel("Tunnel already active";
        }

        let id = self.next_tunnel_id;
        self.next_tunnel_id += 1;

        self.active_tunnel = Some(VpnTunnel {
            id,
            server,
            state: TunnelState::Connecting,
            encryption: crate::types::EncryptionAlgorithm::Aes256GcmPqc,
            key_exchange: crate::types::KeyExchangeProtocol::HybridMlKem,
            stats: ConnectionStats::default(),
        });

        Ok(id)
    }

    /// Get active tunnel.
    pub fn active_tunnel(&self) -> Option<&VpnTunnel> {
        self.active_tunnel.as_ref()
    }

    /// Update tunnel state.
    pub fn update_state(&mut self, state: TunnelState) {
        if let Some(ref mut tunnel) = self.active_tunnel {
            tunnel.state = state;
        }
    }

    /// Close active tunnel.
    pub fn close_tunnel(&mut self) {
        if let Some(ref mut tunnel) = self.active_tunnel {
            tunnel.state = TunnelState::Disconnecting;
        }
        self.active_tunnel = None;
    }

    /// Check if tunnel is connected.
    pub fn is_connected(&self) -> bool {
        self.active_tunnel
            .as_ref()
            .map(|t| t.state == TunnelState::Connected)
            .unwrap_or(false)
    }
}

impl Default for TunnelManager {
    fn default() -> Self {
        Self::new()
    }
}

