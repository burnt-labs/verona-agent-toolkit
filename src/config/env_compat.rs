//! Environment variable helpers with legacy `XION_*` fallback.

use std::env;

/// Read `primary` env var, falling back to `legacy` with a deprecation warning.
/// Empty or whitespace-only values are treated as unset.
pub fn env_var_with_legacy(primary: &str, legacy: &str) -> Option<String> {
    if let Some(value) = read_nonempty_env(primary) {
        return Some(value);
    }
    if let Some(value) = read_nonempty_env(legacy) {
        tracing::warn!("{legacy} is deprecated; use {primary} instead");
        return Some(value);
    }
    None
}

fn read_nonempty_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

/// Set `primary` from CLI/runtime, preserving legacy override reads elsewhere.
pub fn set_runtime_env(primary: &str, value: &str) {
    env::set_var(primary, value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial(env_compat)]
    fn test_env_var_with_legacy_prefers_primary() {
        env::set_var("VERONA_TEST_PRIMARY", "primary-value");
        env::set_var("XION_TEST_LEGACY", "legacy-value");

        let value = env_var_with_legacy("VERONA_TEST_PRIMARY", "XION_TEST_LEGACY");
        assert_eq!(value.as_deref(), Some("primary-value"));

        env::remove_var("VERONA_TEST_PRIMARY");
        env::remove_var("XION_TEST_LEGACY");
    }

    #[test]
    #[serial(env_compat)]
    fn test_env_var_with_legacy_falls_back_to_legacy() {
        env::remove_var("VERONA_TEST_PRIMARY");
        env::set_var("XION_TEST_LEGACY", "legacy-only");

        let value = env_var_with_legacy("VERONA_TEST_PRIMARY", "XION_TEST_LEGACY");
        assert_eq!(value.as_deref(), Some("legacy-only"));

        env::remove_var("XION_TEST_LEGACY");
    }

    #[test]
    #[serial(env_compat)]
    fn test_env_var_with_legacy_returns_none_when_missing() {
        env::remove_var("VERONA_TEST_PRIMARY");
        env::remove_var("XION_TEST_LEGACY");

        let value = env_var_with_legacy("VERONA_TEST_PRIMARY", "XION_TEST_LEGACY");
        assert!(value.is_none());
    }

    #[test]
    #[serial(env_compat)]
    fn test_env_var_with_legacy_treats_empty_primary_as_unset() {
        env::set_var("VERONA_TEST_PRIMARY", "   ");
        env::set_var("XION_TEST_LEGACY", "legacy-fallback");

        let value = env_var_with_legacy("VERONA_TEST_PRIMARY", "XION_TEST_LEGACY");
        assert_eq!(value.as_deref(), Some("legacy-fallback"));

        env::remove_var("VERONA_TEST_PRIMARY");
        env::remove_var("XION_TEST_LEGACY");
    }
}
