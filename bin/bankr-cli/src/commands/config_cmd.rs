use std::path::Path;

use eyre::Result;

use crate::config;

/// `bankr-cli config` — display current configuration.
pub(crate) fn cmd_config(config_path: &Path) -> Result<()> {
    let cfg = config::load(config_path);

    let masked_key =
        cfg.api_key.as_deref().map_or_else(|| "Not set".to_owned(), config::mask_api_key);

    let api_url = cfg.api_url.as_deref().unwrap_or("https://api.bankr.bot");

    println!("Config file:  {}", config_path.display());
    println!("API Key:      {masked_key}");
    println!("API URL:      {api_url}");

    Ok(())
}
