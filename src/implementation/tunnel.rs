//! Tunnel management implementation.

use std::rc::Rc;

use crate::{
    errors::{VpnError, VpnResult},
    types::{
        ConnectionStats, EncryptionAlgorithm, KeyExchangeProtocol, TunnelState, VpnServer,
        VpnTunnel,
    },
};

/// Tunnel manager for VPN connections.
pub struct TunnelManager {
    active_tunnel:  Option<VpnTunnel>,
    next_tunnel_id: u64,
}

impl TunnelManager {
    /// Create a new tunnel manager.
    #[must_use]
    pub fn new() -> Self {
        Self { active_tunnel: None, next_tunnel_id: 1 }
    }

    /// Create a tunnel to server.
    ///
    /// # Errors
    ///
    /// Returns `VpnError::Tunnel` if a tunnel is already active.
    pub fn create_tunnel(&mut self, server: Rc<VpnServer>) -> VpnResult<u64> {
        if self.active_tunnel.is_some() {
            return Err(VpnError::Tunnel("Tunnel already active".to_string()));
        }

        let id = self.next_tunnel_id;
        self.next_tunnel_id += 1;

        self.active_tunnel = Some(VpnTunnel {
            id,
            server: (*server).clone(),
            state: TunnelState::Connecting,
            encryption: EncryptionAlgorithm::Aes256GcmPqc,
            key_exchange: KeyExchangeProtocol::HybridMlKem,
            stats: ConnectionStats::default(),
        });

        Ok(id)
    }

    /// Get active tunnel.
    #[must_use]
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
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.active_tunnel.as_ref().is_some_and(|t| t.state == TunnelState::Connected)
    }
}

impl Default for TunnelManager {
    fn default() -> Self {
        Self::new()
    }
}
