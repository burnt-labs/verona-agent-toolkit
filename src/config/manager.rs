use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use super::credentials::CredentialsManager;
use super::schema::Config;

pub struct ConfigManager {
    config_dir: PathBuf,
    config: Config,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        // Use unified ~/.xion-toolkit/ directory for all platforms
        let home_dir = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .context("Failed to determine home directory")?;

        let config_dir = PathBuf::from(home_dir).join(".xion-toolkit");

        // Ensure config directory exists
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;

        let config = Self::load_or_create_config(&config_dir)?;

        Ok(Self { config_dir, config })
    }

    fn config_file_path(&self) -> PathBuf {
        self.config_dir.join("config.json")
    }

    fn load_or_create_config(config_dir: &Path) -> Result<Config> {
        let config_path = config_dir.join("config.json");

        if config_path.exists() {
            let config_str =
                fs::read_to_string(&config_path).context("Failed to read config file")?;
            let config: Config =
                serde_json::from_str(&config_str).context("Failed to parse config file")?;
            Ok(config)
        } else {
            let config = Config::default();
            let config_str = serde_json::to_string_pretty(&config)
                .context("Failed to serialize default config")?;
            fs::write(&config_path, config_str).context("Failed to write default config file")?;
            Ok(config)
        }
    }

    pub fn load_config(&self) -> Result<&Config> {
        Ok(&self.config)
    }

    pub fn save_config(&mut self) -> Result<()> {
        let config_path = self.config_file_path();
        let config_str =
            serde_json::to_string_pretty(&self.config).context("Failed to serialize config")?;
        fs::write(&config_path, config_str).context("Failed to write config file")?;
        Ok(())
    }

    pub fn get_current_network(&self) -> &str {
        // Check for CLI override via environment variable
        if let Ok(network_override) = std::env::var("XION_NETWORK_OVERRIDE") {
            // Return the override value (leak to get 'static lifetime)
            // This is safe because the environment variable lives for the process lifetime
            Box::leak(network_override.into_boxed_str())
        } else {
            &self.config.network
        }
    }

    pub fn set_network(&mut self, network: &str) -> Result<()> {
        if !["testnet", "mainnet"].contains(&network) {
            anyhow::bail!("Invalid network: {}. Must be testnet or mainnet", network);
        }
        if network == "mainnet" {
            mainnet_network_config()?;
        }
        self.config.network = network.to_string();
        self.save_config()
    }

    pub fn get_status(&self) -> Result<serde_json::Value> {
        let current_network = self.get_current_network();
        let network_config = self.get_network_config()?;

        // Check if user has credentials for this network
        let credentials_manager = CredentialsManager::new(current_network)?;
        let has_credentials = credentials_manager.has_credentials()?;

        Ok(serde_json::json!({
            "network": current_network,
            "chain_id": network_config.chain_id,
            "oauth_api_url": network_config.oauth_api_url,
            "authenticated": has_credentials,
            "callback_port": network_config.callback_port
        }))
    }

    pub fn get_network_config(&self) -> Result<super::constants::NetworkConfig> {
        use super::constants::get_testnet_config;

        let current_network = self.get_current_network();
        match current_network {
            "testnet" => Ok(get_testnet_config()),
            "mainnet" => mainnet_network_config(),
            _ => anyhow::bail!("Unknown network: {}", current_network),
        }
    }

    pub fn reset_config(&mut self) -> Result<()> {
        self.config = Config::default();
        self.save_config()
    }

    /// Create a config manager rooted at a custom directory (for tests).
    #[cfg(test)]
    pub fn with_config_dir(config_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        let config = Self::load_or_create_config(&config_dir)?;
        Ok(Self { config_dir, config })
    }
}

fn mainnet_network_config() -> Result<super::constants::NetworkConfig> {
    use super::constants::get_mainnet_config;

    let config = get_mainnet_config();
    validate_mainnet_oauth_client_id(&config.oauth_client_id)?;
    Ok(config)
}

fn is_unconfigured_mainnet_oauth_client_id(client_id: &str) -> bool {
    client_id.is_empty()
        || client_id.contains("PLACEHOLDER")
        || client_id.starts_with("your-mainnet")
}

fn validate_mainnet_oauth_client_id(client_id: &str) -> Result<()> {
    if is_unconfigured_mainnet_oauth_client_id(client_id) {
        anyhow::bail!(
            "Mainnet OAuth client ID is not configured in this binary. \
             Rebuild with XION_MAINNET_OAUTH_CLIENT_ID set in the environment or in a .env file \
             loaded at build time, or use a release binary built with the GitHub Actions variable. \
             See CONTRIBUTING.md and docs/release.md."
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unconfigured_mainnet_oauth_client_id() {
        assert!(is_unconfigured_mainnet_oauth_client_id(""));
        assert!(is_unconfigured_mainnet_oauth_client_id(
            "PLACEHOLDER_MAINNET_CLIENT_ID"
        ));
        assert!(is_unconfigured_mainnet_oauth_client_id(
            "your-mainnet-client-id-here"
        ));
        assert!(!is_unconfigured_mainnet_oauth_client_id(
            "GhA--realClientId"
        ));
    }

    #[test]
    fn test_validate_mainnet_oauth_client_id_rejects_placeholder() {
        let err = validate_mainnet_oauth_client_id("your-mainnet-client-id-here").unwrap_err();
        assert!(
            err.to_string()
                .contains("Rebuild with XION_MAINNET_OAUTH_CLIENT_ID"),
            "unexpected error: {err}"
        );
    }

    /// Codex P1: `set_network mainnet` must validate before persisting (see PR #75).
    #[test]
    fn test_set_network_mainnet_fails_without_persisting_when_unconfigured() {
        use crate::config::constants::get_mainnet_config;
        use tempfile::tempdir;

        let mainnet_id = get_mainnet_config().oauth_client_id;
        if !is_unconfigured_mainnet_oauth_client_id(&mainnet_id) {
            // Built with a real mainnet client ID; persistence regression not applicable.
            return;
        }

        let temp = tempdir().expect("temp dir");
        let mut manager =
            ConfigManager::with_config_dir(temp.path().to_path_buf()).expect("config manager");
        assert_eq!(manager.load_config().expect("config").network, "testnet");

        let err = manager.set_network("mainnet").unwrap_err();
        assert!(
            err.to_string()
                .contains("Rebuild with XION_MAINNET_OAUTH_CLIENT_ID"),
            "unexpected error: {err}"
        );

        let reloaded =
            ConfigManager::with_config_dir(temp.path().to_path_buf()).expect("reload config");
        assert_eq!(
            reloaded.load_config().expect("config").network,
            "testnet",
            "config.json must not be updated when mainnet validation fails"
        );
    }
}
