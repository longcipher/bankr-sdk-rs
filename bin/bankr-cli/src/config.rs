//! Configuration management for bankr-cli.
//!
//! Loads, saves, and resolves configuration from `$HOME/.bankr/config.json`,
//! matching the format used by the npm CLI (camelCase JSON keys).

use std::path::{Path, PathBuf};

use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};

/// Bankr CLI configuration, persisted as JSON in `$HOME/.bankr/config.json`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankrConfig {
    /// API key for authenticating with the Bankr Agent API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// Base URL for the Bankr Agent API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
}

/// Returns the default config file path: `$HOME/.bankr/config.json`.
///
/// # Panics
///
/// Returns `None` if the home directory cannot be determined.
pub fn default_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".bankr").join("config.json"))
}

/// Load configuration from `path`.
///
/// Returns [`BankrConfig::default()`] if the file does not exist or cannot be
/// parsed.
pub fn load(path: &Path) -> BankrConfig {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return BankrConfig::default();
    };

    serde_json::from_str(&contents).unwrap_or_default()
}

/// Persist `config` as pretty-printed JSON to `path`.
///
/// Creates parent directories as needed and sets `0600` permissions on Unix.
pub fn save(path: &Path, config: &BankrConfig) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .wrap_err_with(|| format!("failed to create config directory {}", parent.display()))?;
    }

    let json =
        serde_json::to_string_pretty(config).wrap_err("failed to serialize configuration")?;

    std::fs::write(path, &json)
        .wrap_err_with(|| format!("failed to write config file {}", path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(path, perms)
            .wrap_err_with(|| format!("failed to set permissions on {}", path.display()))?;
    }

    Ok(())
}

/// Mask an API key for display, e.g. `bk_WKW...46ZE`.
///
/// Keys shorter than 10 characters are fully masked as `***`.
pub fn mask_api_key(key: &str) -> String {
    if key.len() < 10 {
        return "***".to_owned();
    }
    format!("{}...{}", &key[..6], &key[key.len() - 4..])
}

/// Resolve the API key with priority: CLI flag > environment variable > config file.
pub fn resolve_api_key(
    cli_flag: Option<&str>,
    env_var: Option<&str>,
    config: &BankrConfig,
) -> Option<String> {
    cli_flag.or(env_var).map(String::from).or_else(|| config.api_key.clone())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn round_trip_write_then_read() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");

        let original = BankrConfig {
            api_key: Some("bk_test_key_12345".to_owned()),
            api_url: Some("https://api.example.com".to_owned()),
        };

        save(&path, &original).unwrap();
        let loaded = load(&path);

        assert_eq!(original, loaded);
    }

    #[test]
    fn load_missing_file_returns_default() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");

        let config = load(&path);
        assert_eq!(config, BankrConfig::default());
    }

    #[test]
    fn load_malformed_json_returns_default() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.json");
        fs::write(&path, "not json at all").unwrap();

        let config = load(&path);
        assert_eq!(config, BankrConfig::default());
    }

    #[test]
    fn save_creates_parent_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a").join("b").join("config.json");

        save(&path, &BankrConfig::default()).unwrap();
        assert!(path.exists());
    }

    #[cfg(unix)]
    #[test]
    fn save_sets_unix_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");

        save(&path, &BankrConfig::default()).unwrap();

        let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }

    #[test]
    fn camel_case_json_keys() {
        let config =
            BankrConfig { api_key: Some("key".to_owned()), api_url: Some("url".to_owned()) };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("apiKey"));
        assert!(json.contains("apiUrl"));
        assert!(!json.contains("api_key"));
        assert!(!json.contains("api_url"));
    }

    #[test]
    fn mask_api_key_normal() {
        assert_eq!(mask_api_key("bk_WKW12346ZE"), "bk_WKW...46ZE");
    }

    #[test]
    fn mask_api_key_long() {
        assert_eq!(mask_api_key("bk_this_is_a_very_long_key_value_1234"), "bk_thi...1234");
    }

    #[test]
    fn mask_api_key_short() {
        assert_eq!(mask_api_key("short"), "***");
    }

    #[test]
    fn resolve_api_key_priority() {
        let config = BankrConfig { api_key: Some("from_config".to_owned()), api_url: None };

        // CLI flag wins over everything.
        assert_eq!(
            resolve_api_key(Some("from_flag"), Some("from_env"), &config),
            Some("from_flag".to_owned()),
        );

        // Env var wins over config.
        assert_eq!(resolve_api_key(None, Some("from_env"), &config), Some("from_env".to_owned()),);

        // Config is the fallback.
        assert_eq!(resolve_api_key(None, None, &config), Some("from_config".to_owned()),);

        // None if nothing is set.
        let empty = BankrConfig::default();
        assert_eq!(resolve_api_key(None, None, &empty), None);
    }

    #[test]
    fn load_empty_file_returns_default() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("empty.json");
        fs::write(&path, "").unwrap();

        let config = load(&path);
        assert_eq!(config, BankrConfig::default());
    }

    #[test]
    fn default_config_path_is_under_home() {
        if let Some(path) = default_config_path() {
            assert!(path.ends_with(".bankr/config.json"));
        }
    }
}
