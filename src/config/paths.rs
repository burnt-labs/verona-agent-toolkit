//! Config directory paths and legacy migration.

use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_DIR_NAME: &str = ".verona-toolkit";
const LEGACY_CONFIG_DIR_NAME: &str = ".xion-toolkit";

/// Resolve the home directory for config storage.
fn home_dir() -> Result<PathBuf> {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .context("Failed to determine home directory")
        .map(PathBuf::from)
}

/// Copy a directory tree recursively (fallback when rename fails).
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).with_context(|| format!("Failed to create {}", dst.display()))?;
    for entry in fs::read_dir(src).with_context(|| format!("Failed to read {}", src.display()))? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).with_context(|| {
                format!(
                    "Failed to copy {} to {}",
                    src_path.display(),
                    dst_path.display()
                )
            })?;
        }
    }
    Ok(())
}

/// Migrate legacy `~/.xion-toolkit/` to `~/.verona-toolkit/` when needed.
fn migrate_legacy_config_dir(_home: &Path, new_dir: &Path, legacy_dir: &Path) -> Result<()> {
    if new_dir.exists() || !legacy_dir.exists() {
        return Ok(());
    }

    tracing::warn!(
        "Migrating config from {} to {}",
        legacy_dir.display(),
        new_dir.display()
    );

    if fs::rename(legacy_dir, new_dir).is_err() {
        copy_dir_all(legacy_dir, new_dir).context("Failed to copy legacy config directory")?;
        fs::remove_dir_all(legacy_dir).context("Failed to remove legacy config directory")?;
    }

    Ok(())
}

/// Return the SSOT config directory (`~/.verona-toolkit/`), migrating legacy data if present.
pub fn config_dir() -> Result<PathBuf> {
    let home = home_dir()?;
    let new_dir = home.join(CONFIG_DIR_NAME);
    let legacy_dir = home.join(LEGACY_CONFIG_DIR_NAME);

    migrate_legacy_config_dir(&home, &new_dir, &legacy_dir)?;

    fs::create_dir_all(&new_dir).context("Failed to create config directory")?;
    Ok(new_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_migrate_legacy_config_dir_renames() {
        let temp = tempdir().expect("temp dir");
        let legacy = temp.path().join(LEGACY_CONFIG_DIR_NAME);
        let new_dir = temp.path().join(CONFIG_DIR_NAME);
        fs::create_dir_all(&legacy).expect("legacy dir");
        fs::write(
            legacy.join("config.json"),
            r#"{"network":"testnet","version":"1.0"}"#,
        )
        .expect("write config");

        migrate_legacy_config_dir(temp.path(), &new_dir, &legacy).expect("migrate");

        assert!(new_dir.exists());
        assert!(!legacy.exists());
        assert!(new_dir.join("config.json").exists());
    }

    #[test]
    fn test_migrate_skips_when_new_dir_exists() {
        let temp = tempdir().expect("temp dir");
        let legacy = temp.path().join(LEGACY_CONFIG_DIR_NAME);
        let new_dir = temp.path().join(CONFIG_DIR_NAME);
        fs::create_dir_all(&legacy).expect("legacy dir");
        fs::create_dir_all(&new_dir).expect("new dir");
        fs::write(legacy.join("old.json"), "legacy").expect("write legacy");
        fs::write(new_dir.join("new.json"), "current").expect("write new");

        migrate_legacy_config_dir(temp.path(), &new_dir, &legacy).expect("migrate");

        assert!(legacy.exists());
        assert!(new_dir.join("new.json").exists());
        assert!(!new_dir.join("old.json").exists());
    }
}
