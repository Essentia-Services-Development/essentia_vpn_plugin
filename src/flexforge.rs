//! FlexForge Integration for Essentia VPN Plugin
//!
//! Provides configuration panels and status streaming for VPN connections.
//!
//! ## Features
//!
//! - Server selection and connection management
//! - Kill switch configuration
//! - Real-time connection status streaming
//! - Bandwidth metrics display

use essentia_traits::plugin_contracts::{
    ConfigField, ConfigSchema, FlexForgeCapability, FlexForgeIntegration, FlexForgePanelCategory,
    FlexForgePanelInfo, StreamingCapable, UiConfigurable,
};

use crate::types::KeyExchangeProtocol;

/// VPN Plugin FlexForge integration.
#[derive(Debug)]
pub struct VpnPluginFlexForge {
    config:           VpnUiConfig,
    stream_active:    bool,
    stream_id:        Option<u64>,
    next_id:          u64,
    /// Connection state for UI display
    connection_state: ConnectionState,
}

/// Configuration exposed through FlexForge UI.
#[derive(Debug, Clone)]
pub struct VpnUiConfig {
    /// Enable kill switch
    pub kill_switch:    bool,
    /// Auto-connect on startup
    pub auto_connect:   bool,
    /// Preferred server region
    pub server_region:  String,
    /// Key exchange protocol
    pub key_exchange:   KeyExchangeProtocol,
    /// DNS leak protection
    pub dns_protection: bool,
    /// Split tunneling enabled
    pub split_tunnel:   bool,
}

/// Connection state for streaming updates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected
    Disconnected,
    /// Connecting to server
    Connecting,
    /// Connected and active
    Connected,
    /// Reconnecting after disconnect
    Reconnecting,
    /// Error state
    Error,
}

impl ConnectionState {
    /// Convert state to string representation
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Disconnected => "disconnected",
            Self::Connecting => "connecting",
            Self::Connected => "connected",
            Self::Reconnecting => "reconnecting",
            Self::Error => "error",
        }
    }
}

impl Default for VpnUiConfig {
    fn default() -> Self {
        Self {
            kill_switch:    true,
            auto_connect:   false,
            server_region:  String::from("auto"),
            key_exchange:   KeyExchangeProtocol::MlKem,
            dns_protection: true,
            split_tunnel:   false,
        }
    }
}

impl VpnPluginFlexForge {
    /// Creates a new FlexForge integration wrapper.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config:           VpnUiConfig::default(),
            stream_active:    false,
            stream_id:        None,
            next_id:          1,
            connection_state: ConnectionState::Disconnected,
        }
    }

    /// Returns panel info with capabilities.
    #[must_use]
    pub fn panel_info(&self) -> FlexForgePanelInfo {
        FlexForgePanelInfo {
            id:           self.panel_id().to_string(),
            name:         self.display_name().to_string(),
            category:     self.category(),
            icon:         self.icon_glyph().map(String::from),
            priority:     self.priority(),
            capabilities: vec![
                FlexForgeCapability::Configuration,
                FlexForgeCapability::Streaming,
                FlexForgeCapability::Dashboard,
            ],
        }
    }

    /// Gets the current connection state.
    #[must_use]
    pub fn connection_state(&self) -> ConnectionState {
        self.connection_state
    }

    /// Sets the connection state (called by VPN core).
    pub fn set_connection_state(&mut self, state: ConnectionState) {
        self.connection_state = state;
    }

    fn next_stream_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        id
    }
}

impl Default for VpnPluginFlexForge {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// FlexForge Integration
// ============================================================================

impl FlexForgeIntegration for VpnPluginFlexForge {
    fn panel_id(&self) -> &str {
        "essentia_vpn_plugin"
    }

    fn category(&self) -> FlexForgePanelCategory {
        FlexForgePanelCategory::Security
    }

    fn display_name(&self) -> &str {
        "VPN & Tunnels"
    }

    fn icon_glyph(&self) -> Option<&str> {
        Some("\u{E839}") // Shield with lock icon
    }

    fn priority(&self) -> u32 {
        10 // Primary in Security category
    }

    fn on_panel_activate(&mut self) {
        // Start status streaming when panel is viewed
        if !self.stream_active {
            let _ = self.start_stream();
        }
    }

    fn on_panel_deactivate(&mut self) {
        if self.stream_active
            && let Some(id) = self.stream_id
        {
            let _ = self.stop_stream(id);
        }
    }

    fn on_refresh(&mut self) -> bool {
        // Always refresh when streaming connection status
        self.stream_active
    }
}

// ============================================================================
// UI Configurable
// ============================================================================

impl UiConfigurable for VpnPluginFlexForge {
    fn config_schema(&self) -> ConfigSchema {
        ConfigSchema::new()
            .with_field(
                ConfigField::toggle("kill_switch", "Kill Switch", true)
                    .with_description("Block all traffic if VPN disconnects")
                    .with_group("Security"),
            )
            .with_field(
                ConfigField::toggle("dns_protection", "DNS Leak Protection", true)
                    .with_description("Prevent DNS queries outside VPN tunnel")
                    .with_group("Security"),
            )
            .with_field(
                ConfigField::select("key_exchange", "Key Exchange Protocol", vec![
                    String::from("ml_kem"),
                    String::from("hybrid_ml_kem"),
                    String::from("x25519"),
                ])
                .with_description("Post-quantum key exchange algorithm")
                .with_group("Security"),
            )
            .with_field(
                ConfigField::toggle("auto_connect", "Auto-Connect", false)
                    .with_description("Connect automatically on application start")
                    .with_group("Connection"),
            )
            .with_field(
                ConfigField::select("server_region", "Server Region", vec![
                    String::from("auto"),
                    String::from("us-east"),
                    String::from("us-west"),
                    String::from("eu-west"),
                    String::from("eu-central"),
                    String::from("asia-pacific"),
                ])
                .with_description("Preferred server region for connection")
                .with_group("Connection"),
            )
            .with_field(
                ConfigField::toggle("split_tunnel", "Split Tunneling", false)
                    .with_description("Allow some apps to bypass VPN")
                    .with_group("Advanced"),
            )
    }

    fn on_config_changed(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "kill_switch" => {
                self.config.kill_switch = value == "true";
                Ok(())
            },
            "dns_protection" => {
                self.config.dns_protection = value == "true";
                Ok(())
            },
            "key_exchange" => {
                self.config.key_exchange = match value {
                    "ml_kem" => KeyExchangeProtocol::MlKem,
                    "hybrid_ml_kem" => KeyExchangeProtocol::HybridMlKem,
                    "x25519" => KeyExchangeProtocol::X25519,
                    _ => return Err(format!("Unknown key exchange: {value}")),
                };
                Ok(())
            },
            "auto_connect" => {
                self.config.auto_connect = value == "true";
                Ok(())
            },
            "server_region" => {
                self.config.server_region = value.to_string();
                Ok(())
            },
            "split_tunnel" => {
                self.config.split_tunnel = value == "true";
                Ok(())
            },
            _ => Err(format!("Unknown configuration key: {key}")),
        }
    }

    fn apply_config(&mut self, config: &[(String, String)]) -> Result<(), String> {
        for (key, value) in config {
            self.on_config_changed(key, value)?;
        }
        Ok(())
    }

    fn get_current_config(&self) -> Vec<(String, String)> {
        let key_exchange_str = match self.config.key_exchange {
            KeyExchangeProtocol::MlKem => "ml_kem",
            KeyExchangeProtocol::HybridMlKem => "hybrid_ml_kem",
            KeyExchangeProtocol::X25519 => "x25519",
        };

        vec![
            (
                String::from("kill_switch"),
                self.config.kill_switch.to_string(),
            ),
            (
                String::from("dns_protection"),
                self.config.dns_protection.to_string(),
            ),
            (String::from("key_exchange"), key_exchange_str.to_string()),
            (
                String::from("auto_connect"),
                self.config.auto_connect.to_string(),
            ),
            (
                String::from("server_region"),
                self.config.server_region.clone(),
            ),
            (
                String::from("split_tunnel"),
                self.config.split_tunnel.to_string(),
            ),
        ]
    }

    fn reset_to_defaults(&mut self) {
        self.config = VpnUiConfig::default();
    }
}

// ============================================================================
// Streaming Capable
// ============================================================================

impl StreamingCapable for VpnPluginFlexForge {
    fn is_streaming(&self) -> bool {
        self.stream_active
    }

    fn start_stream(&mut self) -> Result<u64, String> {
        if self.stream_active {
            return Err("Stream already active".to_string());
        }

        let stream_id = self.next_stream_id();
        self.stream_id = Some(stream_id);
        self.stream_active = true;

        Ok(stream_id)
    }

    fn stop_stream(&mut self, stream_id: u64) -> Result<(), String> {
        if !self.stream_active {
            return Err("No active stream".to_string());
        }

        if self.stream_id != Some(stream_id) {
            return Err("Invalid stream ID".to_string());
        }

        self.stream_active = false;
        self.stream_id = None;

        Ok(())
    }

    fn target_fps(&self) -> u32 {
        // Low rate status updates
        5
    }

    fn render_frame(&mut self, stream_id: u64, _delta_ms: f64) -> bool {
        if !self.stream_active || self.stream_id != Some(stream_id) {
            return false;
        }

        // Emit status frame with connection state, bandwidth, etc.
        // In production, this would serialize to ERSP status frame
        true
    }
}

#[cfg(all(test, feature = "full-tests"))]
mod tests {
    use super::*;

    #[test]
    fn test_panel_id() {
        let plugin = VpnPluginFlexForge::new();
        assert_eq!(plugin.panel_id(), "essentia_vpn_plugin");
        assert_eq!(plugin.category(), FlexForgePanelCategory::Security);
    }

    #[test]
    fn test_default_config() {
        let plugin = VpnPluginFlexForge::new();
        assert!(plugin.config.kill_switch);
        assert!(plugin.config.dns_protection);
        assert!(!plugin.config.auto_connect);
    }

    #[test]
    fn test_connection_state() {
        let mut plugin = VpnPluginFlexForge::new();
        assert_eq!(plugin.connection_state(), ConnectionState::Disconnected);

        plugin.set_connection_state(ConnectionState::Connected);
        assert_eq!(plugin.connection_state(), ConnectionState::Connected);
    }

    #[test]
    fn test_streaming() {
        let mut plugin = VpnPluginFlexForge::new();

        let stream_id = plugin.start_stream().expect("Should start streaming");
        assert!(plugin.is_streaming());
        assert_eq!(plugin.target_fps(), 5);

        plugin.stop_stream(stream_id).expect("Should stop streaming");
        assert!(!plugin.is_streaming());
    }
}
