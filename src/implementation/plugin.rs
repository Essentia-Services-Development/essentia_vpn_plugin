//! VPN plugin implementation.

use std::rc::Rc;

use crate::{
    errors::{VpnError, VpnResult},
    implementation::{NeuralRouter, PqcKeyExchange, TunnelManager, VpnConfig},
    types::{TunnelState, VpnServer},
};

/// Main VPN plugin interface.
pub struct VpnPlugin {
    config:             VpnConfig,
    tunnel_manager:     TunnelManager,
    key_exchange:       Option<PqcKeyExchange>,
    router:             NeuralRouter,
    kill_switch_active: bool,
}

impl VpnPlugin {
    /// Create a new VPN plugin.
    #[must_use]
    pub fn new(config: VpnConfig) -> Self {
        Self {
            config,
            tunnel_manager: TunnelManager::new(),
            key_exchange: None,
            router: NeuralRouter::new(),
            kill_switch_active: false,
        }
    }

    /// Get configuration.
    #[must_use]
    pub fn config(&self) -> &VpnConfig {
        &self.config
    }

    /// Get router.
    #[must_use]
    pub fn router(&self) -> &NeuralRouter {
        &self.router
    }

    /// Get mutable router.
    pub fn router_mut(&mut self) -> &mut NeuralRouter {
        &mut self.router
    }

    /// Connect to a specific server.
    ///
    /// # Errors
    ///
    /// Returns `VpnError::Connection` if already connected or connection fails.
    pub fn connect(&mut self, server: Rc<VpnServer>) -> VpnResult<()> {
        if self.is_connected() {
            return Err(VpnError::Connection("Already connected".to_string()));
        }

        // Enable kill switch if configured
        if self.config.kill_switch {
            self.activate_kill_switch();
        }

        // Create tunnel
        self.tunnel_manager.create_tunnel(server)?;

        // Perform key exchange
        let mut key_exchange = PqcKeyExchange::new(self.config.key_exchange);
        let _public_key = key_exchange.generate_keypair()?;

        // In production, would send public key to server and complete exchange
        self.key_exchange = Some(key_exchange);

        // Update state
        self.tunnel_manager.update_state(TunnelState::Connected);

        Ok(())
    }

    /// Connect to optimal server.
    ///
    /// # Errors
    ///
    /// Returns `VpnError::Connection` if no servers available or connection
    /// fails.
    pub fn connect_optimal(&mut self) -> VpnResult<()> {
        let server_rc = Rc::clone(
            self.router
                .find_optimal_server()
                .ok_or_else(|| VpnError::Connection("No servers available".into()))?,
        );

        let server = Rc::new((*server_rc).borrow().clone());
        self.connect(server)
    }

    /// Disconnect from current server.
    pub fn disconnect(&mut self) {
        self.tunnel_manager.close_tunnel();

        // Clear key exchange
        if let Some(ref mut ke) = self.key_exchange {
            ke.clear();
        }
        self.key_exchange = None;

        // Deactivate kill switch
        if self.config.kill_switch {
            self.deactivate_kill_switch();
        }
    }

    /// Check if connected.
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.tunnel_manager.is_connected()
    }

    /// Get connection state.
    #[must_use]
    pub fn state(&self) -> TunnelState {
        self.tunnel_manager
            .active_tunnel()
            .map(|t| t.state)
            .unwrap_or(TunnelState::Disconnected)
    }

    /// Activate kill switch.
    fn activate_kill_switch(&mut self) {
        // In production, would configure system firewall
        self.kill_switch_active = true;
    }

    /// Deactivate kill switch.
    fn deactivate_kill_switch(&mut self) {
        // In production, would restore firewall rules
        self.kill_switch_active = false;
    }

    /// Check if kill switch is active.
    #[must_use]
    pub fn is_kill_switch_active(&self) -> bool {
        self.kill_switch_active
    }
}

impl Default for VpnPlugin {
    fn default() -> Self {
        Self::new(VpnConfig::default())
    }
}

impl Drop for VpnPlugin {
    fn drop(&mut self) {
        self.disconnect();
    }
}

#[cfg(all(test, feature = "full-tests"))]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let plugin = VpnPlugin::default();
        assert!(plugin.config().kill_switch);
    }

    #[test]
    fn test_not_connected_initially() {
        let plugin = VpnPlugin::default();
        assert!(!plugin.is_connected());
    }

    #[test]
    fn test_initial_state() {
        let plugin = VpnPlugin::default();
        assert_eq!(plugin.state(), TunnelState::Disconnected);
    }

    #[test]
    fn test_connect_no_servers() {
        let mut plugin = VpnPlugin::default();
        let result = plugin.connect_optimal();
        assert!(result.is_err());
    }
}
