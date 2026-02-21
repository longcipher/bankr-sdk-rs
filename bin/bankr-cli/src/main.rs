//! # bankr-cli
//!
//! Command-line tool for interacting with the Bankr Agent API.
//!
//! ## Usage
//!
//! ```text
//! export BANKR_API_KEY=your_api_key_here
//! bankr-cli whoami
//! bankr-cli prompt "what is the price of ETH?"
//! bankr-cli job <job_id>
//! bankr-cli cancel <job_id>
//! bankr-cli sign personal "Hello, Bankr!"
//! bankr-cli submit --chain-id 8453 --to 0x... --value "1000000000000000000"
//! ```

// CLI binary — allow print macros and expect/unwrap for user-facing output.
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::unwrap_used, clippy::expect_used)]

use std::time::Duration;

use bankr_agent_api::{
    BankrAgentClient,
    types::{EvmTransaction, PromptRequest, SignRequest, SignatureType, SubmitRequest},
};
use clap::{Parser, Subcommand};
use eyre::{Result, WrapErr, eyre};

/// Bankr Agent API command-line interface.
#[derive(Debug, Parser)]
#[command(name = "bankr-cli", version, about = "CLI for the Bankr Agent API")]
struct Cli {
    /// Bankr API key (overrides BANKR_API_KEY env var).
    #[arg(long, env = "BANKR_API_KEY", global = true, hide_env_values = true)]
    api_key: Option<String>,

    /// Base URL override (default: https://api.bankr.bot).
    #[arg(long, env = "BANKR_BASE_URL", global = true)]
    base_url: Option<String>,

    /// Output raw JSON instead of pretty-printed.
    #[arg(long, global = true, default_value_t = false)]
    raw: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Retrieve your account profile (wallets, socials, Bankr Club status).
    Whoami,

    /// Submit a natural language prompt to the Bankr AI agent.
    Prompt {
        /// The prompt text.
        prompt: String,

        /// Continue an existing conversation thread.
        #[arg(long)]
        thread_id: Option<String>,

        /// Wait for the job to complete (poll).
        #[arg(long, default_value_t = true)]
        wait: bool,

        /// Polling interval in seconds.
        #[arg(long, default_value_t = 2)]
        poll_interval: u64,

        /// Maximum number of poll attempts.
        #[arg(long, default_value_t = 60)]
        max_attempts: u32,
    },

    /// Get the status of a submitted job.
    Job {
        /// Job ID to query.
        job_id: String,
    },

    /// Cancel a pending or processing job.
    Cancel {
        /// Job ID to cancel.
        job_id: String,
    },

    /// Sign a message, typed data, or transaction.
    Sign {
        #[command(subcommand)]
        kind: SignCommands,
    },

    /// Submit a raw EVM transaction to the blockchain.
    Submit {
        /// Destination address.
        #[arg(long)]
        to: String,

        /// Chain ID (e.g. 8453 for Base, 1 for Ethereum).
        #[arg(long)]
        chain_id: u64,

        /// Value in wei.
        #[arg(long)]
        value: Option<String>,

        /// Calldata (hex string starting with 0x).
        #[arg(long)]
        data: Option<String>,

        /// Gas limit.
        #[arg(long)]
        gas: Option<String>,

        /// EIP-1559 max fee per gas.
        #[arg(long)]
        max_fee_per_gas: Option<String>,

        /// EIP-1559 priority fee.
        #[arg(long)]
        max_priority_fee_per_gas: Option<String>,

        /// Transaction nonce.
        #[arg(long)]
        nonce: Option<u64>,

        /// Human-readable description.
        #[arg(long)]
        description: Option<String>,

        /// Do not wait for on-chain confirmation.
        #[arg(long, default_value_t = false)]
        no_wait: bool,
    },
}

#[derive(Debug, Subcommand)]
enum SignCommands {
    /// Sign a plain text message (personal_sign).
    Personal {
        /// The message to sign.
        message: String,
    },

    /// Sign EIP-712 typed data (pass JSON string).
    TypedData {
        /// JSON string of the typed data object.
        typed_data_json: String,
    },

    /// Sign an EVM transaction without broadcasting.
    Transaction {
        /// Destination address.
        #[arg(long)]
        to: String,

        /// Chain ID.
        #[arg(long)]
        chain_id: u64,

        /// Value in wei.
        #[arg(long)]
        value: Option<String>,

        /// Calldata (hex).
        #[arg(long)]
        data: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise tracing from RUST_LOG env var (default: warn).
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let cli = Cli::parse();

    let api_key =
        cli.api_key.ok_or_else(|| eyre!("BANKR_API_KEY env var or --api-key flag is required"))?;

    let client = match &cli.base_url {
        Some(url) => BankrAgentClient::with_base_url(&api_key, url).map_err(|e| eyre!("{e}"))?,
        None => BankrAgentClient::new(&api_key).map_err(|e| eyre!("{e}"))?,
    };

    match cli.command {
        Commands::Whoami => cmd_whoami(&client, cli.raw).await,
        Commands::Prompt { prompt, thread_id, wait, poll_interval, max_attempts } => {
            cmd_prompt(
                &client,
                &prompt,
                thread_id.as_deref(),
                wait,
                poll_interval,
                max_attempts,
                cli.raw,
            )
            .await
        }
        Commands::Job { job_id } => cmd_job(&client, &job_id, cli.raw).await,
        Commands::Cancel { job_id } => cmd_cancel(&client, &job_id, cli.raw).await,
        Commands::Sign { kind } => cmd_sign(&client, kind, cli.raw).await,
        Commands::Submit {
            to,
            chain_id,
            value,
            data,
            gas,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            nonce,
            description,
            no_wait,
        } => {
            cmd_submit(
                &client,
                &to,
                chain_id,
                value,
                data,
                gas,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                nonce,
                description,
                no_wait,
                cli.raw,
            )
            .await
        }
    }
}

// ---------------------------------------------------------------------------
// Subcommand handlers
// ---------------------------------------------------------------------------

async fn cmd_whoami(client: &BankrAgentClient, raw: bool) -> Result<()> {
    let resp = client.get_me().await.map_err(|e| eyre!("{e}"))?;
    print_json(&resp, raw)
}

async fn cmd_prompt(
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

async fn cmd_job(client: &BankrAgentClient, job_id: &str, raw: bool) -> Result<()> {
    let resp = client.get_job(job_id).await.map_err(|e| eyre!("{e}"))?;
    print_json(&resp, raw)
}

async fn cmd_cancel(client: &BankrAgentClient, job_id: &str, raw: bool) -> Result<()> {
    let resp = client.cancel_job(job_id).await.map_err(|e| eyre!("{e}"))?;
    print_json(&resp, raw)
}

async fn cmd_sign(client: &BankrAgentClient, kind: SignCommands, raw: bool) -> Result<()> {
    let req = match kind {
        SignCommands::Personal { message } => SignRequest {
            signature_type: SignatureType::PersonalSign,
            message: Some(message),
            typed_data: None,
            transaction: None,
        },
        SignCommands::TypedData { typed_data_json } => {
            let typed_data: serde_json::Value =
                serde_json::from_str(&typed_data_json).wrap_err("Invalid typed-data JSON")?;
            SignRequest {
                signature_type: SignatureType::EthSignTypedDataV4,
                message: None,
                typed_data: Some(typed_data),
                transaction: None,
            }
        }
        SignCommands::Transaction { to, chain_id, value, data } => SignRequest {
            signature_type: SignatureType::EthSignTransaction,
            message: None,
            typed_data: None,
            transaction: Some(EvmTransaction {
                to,
                chain_id,
                value,
                data,
                gas: None,
                gas_price: None,
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
                nonce: None,
            }),
        },
    };

    let resp = client.sign(&req).await.map_err(|e| eyre!("{e}"))?;
    print_json(&resp, raw)
}

#[expect(clippy::too_many_arguments)]
async fn cmd_submit(
    client: &BankrAgentClient,
    to: &str,
    chain_id: u64,
    value: Option<String>,
    data: Option<String>,
    gas: Option<String>,
    max_fee_per_gas: Option<String>,
    max_priority_fee_per_gas: Option<String>,
    nonce: Option<u64>,
    description: Option<String>,
    no_wait: bool,
    raw: bool,
) -> Result<()> {
    let req = SubmitRequest {
        transaction: EvmTransaction {
            to: to.to_owned(),
            chain_id,
            value,
            data,
            gas,
            gas_price: None,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            nonce,
        },
        description,
        wait_for_confirmation: Some(!no_wait),
    };

    let resp = client.submit_transaction(&req).await.map_err(|e| eyre!("{e}"))?;
    print_json(&resp, raw)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn print_json<T: serde::Serialize>(value: &T, raw: bool) -> Result<()> {
    let output = if raw {
        serde_json::to_string(value).wrap_err("JSON serialization failed")?
    } else {
        serde_json::to_string_pretty(value).wrap_err("JSON serialization failed")?
    };
    println!("{output}");
    Ok(())
}
