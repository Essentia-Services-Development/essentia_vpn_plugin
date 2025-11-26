//! Neural network-optimized routing.

use crate::types::VpnServer;

/// Neural router for optimal server selection.
pub struct NeuralRouter {
    servers: Vec<VpnServer>,
}

impl NeuralRouter {
    /// Create a new neural router.
    pub fn new() -> Self {
        Self { servers: Vec::new() }
    }

    /// Add a server to the routing pool.
    pub fn add_server(&mut self, server: VpnServer) {
        self.servers.push(server);
    }

    /// Get all available servers.
    pub fn servers(&self) -> &[VpnServer] {
        &self.servers
    }

    /// Find best server for a given country.
    pub fn find_best_server(&self, country: &str) -> Option<&VpnServer> {
        self.servers
            .iter()
            .filter(|s| s.country == country && s.pqc_enabled)
            .min_by(|a, b| a.load.partial_cmp(&b.load).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Find best server overall (lowest load, PQC enabled).
    pub fn find_optimal_server(&self) -> Option<&VpnServer> {
        self.servers
            .iter()
            .filter(|s| s.pqc_enabled)
            .min_by(|a, b| a.load.partial_cmp(&b.load).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Update server load information.
    pub fn update_server_load(&mut self, server_id: &str, load: f32) {
        if let Some(server) = self.servers.iter_mut().find(|s| s.id == server_id) {
            server.load = load.clamp(0.0, 1.0);
        }
    }
}

impl Default for NeuralRouter {
    fn default() -> Self {
        Self::new()
    }
}
