//! Environment variable helpers with legacy `XION_*` fallback.

use std::env;

/// Read `primary` env var, falling back to `legacy` with a deprecation warning.
pub fn env_var_with_legacy(primary: &str, legacy: &str) -> Option<String> {
    if let Ok(value) = env::var(primary) {
        return Some(value);
    }
    if let Ok(value) = env::var(legacy) {
        tracing::warn!("{legacy} is deprecated; use {primary} instead");
        return Some(value);
    }
    None
}

/// Set `primary` from CLI/runtime, preserving legacy override reads elsewhere.
pub fn set_runtime_env(primary: &str, value: &str) {
    env::set_var(primary, value);
}
