use std::path::Path;

use bankr_agent_api::BankrAgentClient;
use eyre::{Result, eyre};

use crate::{display, print_json};

pub(crate) async fn cmd_whoami(
    client: &BankrAgentClient,
    raw: bool,
    config_path: &Path,
    api_key: &str,
    api_url: &str,
) -> Result<()> {
    let resp = client.get_me().await.map_err(|e| eyre!("{e}"))?;
    if raw {
        print_json(&resp, raw)
    } else {
        display::print_whoami(config_path, api_key, api_url, &resp);
        Ok(())
    }
}
