# Essentia VPN Plugin

Quantum-safe neural tunnels with PQC encryption for the Essentia platform.

## Features

- Post-quantum cryptographic key exchange (ML-KEM)
- Neural network-optimized routing
- Consciousness-integrated traffic patterns
- Multi-hop tunnel support
- Kill switch and leak protection

## FlexForge Integration

The VPN plugin integrates with FlexForge via `VpnPluginFlexForge`:

```ignore
use essentia_vpn_plugin::flexforge::VpnPluginFlexForge;

let plugin = VpnPluginFlexForge::new();
let panel_info = plugin.panel_info();
```

## Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│                       VPN Plugin                             │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │    PQC      │  │   Tunnel    │  │   Neural Router     │  │
│  │  Key Mgmt   │  │   Manager   │  │                     │  │
│  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
│         │                │                     │             │
│         ▼                ▼                     ▼             │
│  ┌─────────────────────────────────────────────────────┐    │
│  │              Secure Transport Layer                  │    │
│  │        (essentia_pqc + essentia_net_plugin)          │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

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
