//! Post-quantum cryptographic key exchange.

use crate::{
    errors::{VpnError, VpnResult},
    types::KeyExchangeProtocol,
};

/// PQC key exchange handler.
pub struct PqcKeyExchange {
    protocol:      KeyExchangeProtocol,
    public_key:    Option<Vec<u8>>,
    shared_secret: Option<Vec<u8>>,
}

impl PqcKeyExchange {
    /// Create a new key exchange handler.
    #[must_use]
    pub fn new(protocol: KeyExchangeProtocol) -> Self {
        Self { protocol, public_key: None, shared_secret: None }
    }

    /// Get the key exchange protocol.
    #[must_use]
    pub fn protocol(&self) -> KeyExchangeProtocol {
        self.protocol
    }

    /// Generate key pair.
    ///
    /// # Errors
    ///
    /// Returns `VpnError::KeyExchange` if key generation fails.
    pub fn generate_keypair(&mut self) -> VpnResult<Vec<u8>> {
        // In production, this would use essentia_pqc ML-KEM
        // Placeholder key generation
        let public_key = vec![0u8; 1184]; // ML-KEM-768 public key size
        self.public_key = Some(public_key.clone());
        Ok(public_key)
    }

    /// Perform key encapsulation (client side).
    ///
    /// # Errors
    ///
    /// Returns `VpnError::KeyExchange` if encapsulation fails.
    pub fn encapsulate(&mut self, server_public_key: &[u8]) -> VpnResult<(Vec<u8>, Vec<u8>)> {
        if server_public_key.is_empty() {
            return Err(VpnError::KeyExchange("Empty server public key".to_string()));
        }

        // In production, uses ML-KEM encapsulation
        let ciphertext = vec![0u8; 1088]; // ML-KEM-768 ciphertext size
        let shared_secret = vec![0u8; 32]; // 256-bit shared secret

        self.shared_secret = Some(shared_secret.clone());
        Ok((ciphertext, shared_secret))
    }

    /// Perform key decapsulation (server side).
    ///
    /// # Errors
    ///
    /// Returns `VpnError::KeyExchange` if decapsulation fails.
    pub fn decapsulate(&mut self, ciphertext: &[u8]) -> VpnResult<Vec<u8>> {
        if ciphertext.is_empty() {
            return Err(VpnError::KeyExchange("Empty ciphertext".to_string()));
        }

        // In production, uses ML-KEM decapsulation
        let shared_secret = vec![0u8; 32];
        self.shared_secret = Some(shared_secret.clone());
        Ok(shared_secret)
    }

    /// Get shared secret.
    #[must_use]
    pub fn shared_secret(&self) -> Option<&[u8]> {
        self.shared_secret.as_deref()
    }

    /// Clear sensitive data.
    pub fn clear(&mut self) {
        if let Some(ref mut key) = self.public_key {
            key.fill(0);
        }
        if let Some(ref mut secret) = self.shared_secret {
            secret.fill(0);
        }
        self.public_key = None;
        self.shared_secret = None;
    }
}

impl Drop for PqcKeyExchange {
    fn drop(&mut self) {
        self.clear();
    }
}
