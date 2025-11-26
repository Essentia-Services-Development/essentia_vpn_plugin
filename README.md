# Essentia VPN Plugin

Quantum-safe VPN with neural routing for the Essentia ecosystem.

## Features

- **PQC Key Exchange**: ML-KEM/Kyber post-quantum key exchange
- **Tunnel Management**: Secure VPN tunnel creation and management
- **Neural Router**: AI-optimized routing decisions
- **Traffic Encryption**: Full traffic encryption with PQC

## Usage

```rust
use essentia_vpn_plugin::{VpnPlugin, VpnConfig, TunnelConfig};

let plugin = VpnPlugin::default();
let config = TunnelConfig::new("vpn.example.com", 443);
let tunnel = plugin.create_tunnel(config)?;
plugin.connect(&tunnel.id)?;
```

## SSOP Compliance

This plugin is fully SSOP-compliant (std-only, zero third-party dependencies).

## License

MIT
