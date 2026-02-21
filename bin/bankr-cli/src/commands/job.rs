use bankr_agent_api::BankrAgentClient;
use eyre::{Result, eyre};

use crate::{display, print_json};

pub(crate) async fn cmd_job(client: &BankrAgentClient, job_id: &str, raw: bool) -> Result<()> {
    let resp = client.get_job(job_id).await.map_err(|e| eyre!("{e}"))?;
    if raw {
        print_json(&resp, raw)
    } else {
        display::print_job_status(&resp);
        Ok(())
    }
}

pub(crate) async fn cmd_cancel(client: &BankrAgentClient, job_id: &str, raw: bool) -> Result<()> {
    let resp = client.cancel_job(job_id).await.map_err(|e| eyre!("{e}"))?;
    print_json(&resp, raw)
}
