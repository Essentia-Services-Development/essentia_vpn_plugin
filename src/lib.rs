// VPN plugin pedantic lint allowances (VPN-LINT-STAGING-01)
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::similar_names)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::unused_self)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::implicit_clone)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::assigning_clones)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::if_not_else)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::float_cmp)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::unnested_or_patterns)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::match_bool)]
#![allow(clippy::unnecessary_literal_bound)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::single_char_pattern)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::range_plus_one)]

mod config;
mod errors;
pub mod flexforge;
mod key_exchange;
mod plugin;
mod router;
mod tunnel;
mod types;

pub use config::VpnConfig;
pub use errors::{VpnError, VpnResult};
pub use flexforge::{ConnectionState, VpnPluginFlexForge, VpnUiConfig};
pub use key_exchange::PqcKeyExchange;
pub use plugin::VpnPlugin;
pub use router::NeuralRouter;
pub use tunnel::TunnelManager;
pub use types::{
    ConnectionStats, EncryptionAlgorithm, KeyExchangeProtocol, TunnelState, VpnServer, VpnTunnel,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let config = VpnConfig::default();
        assert!(config.kill_switch);
    }
}
