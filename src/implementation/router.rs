//! Neural network-optimized routing implementation.

use std::{cell::RefCell, rc::Rc};

use crate::types::VpnServer;

/// Neural router for optimal server selection.
pub struct NeuralRouter {
    servers: Vec<Rc<RefCell<VpnServer>>>,
}

impl NeuralRouter {
    /// Create a new neural router.
    #[must_use]
    pub fn new() -> Self {
        Self { servers: Vec::new() }
    }

    /// Add a server to the routing pool.
    pub fn add_server(&mut self, server: Rc<RefCell<VpnServer>>) {
        self.servers.push(server);
    }

    /// Get all available servers.
    #[must_use]
    pub fn servers(&self) -> &[Rc<RefCell<VpnServer>>] {
        &self.servers
    }

    /// Find best server for a given country.
    #[must_use]
    pub fn find_best_server(&self, country: &str) -> Option<&Rc<RefCell<VpnServer>>> {
        self.servers
            .iter()
            .filter(|s| s.borrow().country == country && s.borrow().pqc_enabled)
            .min_by(|a, b| {
                a.borrow()
                    .load
                    .partial_cmp(&b.borrow().load)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Find best server overall (lowest load, PQC enabled).
    #[must_use]
    pub fn find_optimal_server(&self) -> Option<&Rc<RefCell<VpnServer>>> {
        self.servers.iter().filter(|s| s.borrow().pqc_enabled).min_by(|a, b| {
            a.borrow()
                .load
                .partial_cmp(&b.borrow().load)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Update server load information.
    pub fn update_server_load(&mut self, server_id: &str, load: f32) {
        if let Some(server) = self.servers.iter().find(|s| s.borrow().id == server_id) {
            server.borrow_mut().load = load.clamp(0.0, 1.0);
        }
    }
}

impl Default for NeuralRouter {
    fn default() -> Self {
        Self::new()
    }
}
