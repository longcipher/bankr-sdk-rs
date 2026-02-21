use std::{io::Write, path::Path};

use bankr_agent_api::BankrAgentClient;
use eyre::{Result, eyre};

use crate::{config, display};

/// Prompt the user for an API key on stdin (interactive).
fn read_api_key_interactive() -> Result<String> {
    eprint!("Enter your Bankr API key: ");
    std::io::stderr().flush().map_err(|e| eyre!("{e}"))?;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).map_err(|e| eyre!("{e}"))?;
    let trimmed = buf.trim().to_owned();
    if trimmed.is_empty() {
        return Err(eyre!("API key cannot be empty"));
    }
    Ok(trimmed)
}

/// `bankr-cli login [--api-key KEY]`
pub(crate) async fn cmd_login(
    api_key_flag: Option<&str>,
    base_url: Option<&str>,
    config_path: &Path,
) -> Result<()> {
    let api_key = match api_key_flag {
        Some(k) => k.to_owned(),
        None => read_api_key_interactive()?,
    };

    // Verify the key by calling get_me().
    let client = match base_url {
        Some(url) => BankrAgentClient::with_base_url(&api_key, url).map_err(|e| eyre!("{e}"))?,
        None => BankrAgentClient::new(&api_key).map_err(|e| eyre!("{e}"))?,
    };

    client.get_me().await.map_err(|e| eyre!("login failed: {e}"))?;

    // Persist to config.
    let mut cfg = config::load(config_path);
    cfg.api_key = Some(api_key);
    if let Some(url) = base_url {
        cfg.api_url = Some(url.to_owned());
    }
    config::save(config_path, &cfg)?;

    display::success("Logged in successfully");
    Ok(())
}

/// `bankr-cli logout`
pub(crate) fn cmd_logout(config_path: &Path) -> Result<()> {
    let mut cfg = config::load(config_path);
    cfg.api_key = None;
    config::save(config_path, &cfg)?;
    display::success(&format!("Logged out. Credentials removed from {}", config_path.display()));
    Ok(())
}
