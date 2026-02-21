use std::time::Duration;

use bankr_agent_api::{BankrAgentClient, types::PromptRequest};
use eyre::{Result, eyre};

use crate::print_json;

pub(crate) async fn cmd_prompt(
    client: &BankrAgentClient,
    prompt: &str,
    thread_id: Option<&str>,
    wait: bool,
    poll_interval: u64,
    max_attempts: u32,
    raw: bool,
) -> Result<()> {
    let req =
        PromptRequest { prompt: prompt.to_owned(), thread_id: thread_id.map(ToOwned::to_owned) };

    if wait {
        let job = client
            .prompt_and_wait_with(&req, Duration::from_secs(poll_interval), max_attempts)
            .await
            .map_err(|e| eyre!("{e}"))?;
        print_json(&job, raw)
    } else {
        let resp = client.submit_prompt(&req).await.map_err(|e| eyre!("{e}"))?;
        print_json(&resp, raw)
    }
}
