use bankr_agent_api::{BankrAgentClient, types::PromptRequest};
use eyre::{Result, eyre};

use crate::print_json;

pub(crate) async fn cmd_skills(client: &BankrAgentClient, raw: bool) -> Result<()> {
    let req = PromptRequest {
        prompt: "List all available skills and capabilities with brief descriptions and examples"
            .to_owned(),
        thread_id: None,
    };

    let job = client.prompt_and_wait(&req).await.map_err(|e| eyre!("{e}"))?;

    if raw {
        print_json(&job, raw)
    } else {
        if let Some(response) = &job.response {
            println!("{response}");
        } else {
            println!("No response received (job status: {})", job.status);
        }
        Ok(())
    }
}
