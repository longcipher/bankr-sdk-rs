//! # bankr-cli
//!
//! Bankr AI agent CLI — interact with the Bankr Agent API from the terminal.
//!
//! ## Commands
//!
//! | Command  | Description                                  |
//! |----------|----------------------------------------------|
//! | login    | Authenticate with the Bankr API              |
//! | logout   | Clear stored credentials                     |
//! | config   | Manage CLI configuration                     |
//! | whoami   | Show current authentication info             |
//! | prompt   | Send a prompt to the Bankr AI agent          |
//! | status   | Check the status of a job                    |
//! | cancel   | Cancel a running job                         |
//! | sign     | Sign messages, typed data, or transactions   |
//! | skills   | Show all Bankr AI agent skills with examples |
//! | submit   | Submit a transaction to the blockchain       |
//!
//! ## Usage
//!
//! ```text
//! bankr-cli login
//! bankr-cli whoami
//! bankr-cli prompt "what is the price of ETH?"
//! bankr-cli status <job_id>
//! bankr-cli cancel <job_id>
//! bankr-cli sign personal "Hello, Bankr!"
//! bankr-cli submit --chain-id 8453 --to 0x... --value "1000000000000000000"
//! ```

// CLI binary — allow print macros and expect/unwrap for user-facing output.
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::unwrap_used, clippy::expect_used)]

mod commands;
pub mod config;
pub mod display;

use std::path::PathBuf;

use bankr_agent_api::BankrAgentClient;
use clap::{Parser, Subcommand};
use commands::sign::SignCommands;
use eyre::{Result, WrapErr, eyre};

/// Bankr AI agent CLI.
#[derive(Debug, Parser)]
#[command(name = "bankr-cli", version, about = "Bankr AI agent CLI")]
struct Cli {
    /// Bankr API key (overrides BANKR_API_KEY env var and config file).
    #[arg(long, env = "BANKR_API_KEY", global = true, hide_env_values = true)]
    api_key: Option<String>,

    /// Base URL override (default: https://api.bankr.bot).
    #[arg(long, env = "BANKR_BASE_URL", global = true)]
    base_url: Option<String>,

    /// Path to the configuration file.
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    /// Output raw JSON instead of pretty-printed.
    #[arg(long, global = true, default_value_t = false)]
    raw: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Authenticate with the Bankr API.
    Login {
        /// API key to store. If omitted you will be prompted interactively.
        #[arg(long)]
        api_key: Option<String>,
    },

    /// Clear stored credentials.
    Logout,

    /// Manage CLI configuration.
    Config,

    /// Show current authentication info.
    Whoami,

    /// Send a prompt to the Bankr AI agent.
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

    /// Check the status of a job.
    #[command(alias = "job")]
    Status {
        /// Job ID to query.
        job_id: String,
    },

    /// Cancel a running job.
    Cancel {
        /// Job ID to cancel.
        job_id: String,
    },

    /// Sign messages, typed data, or transactions.
    Sign {
        #[command(subcommand)]
        kind: SignCommands,
    },

    /// Show all Bankr AI agent skills with examples.
    Skills,

    /// Submit a transaction to the blockchain.
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

    let config_path = cli
        .config
        .clone()
        .or_else(config::default_config_path)
        .ok_or_else(|| eyre!("cannot determine config file path (HOME not set?)"))?;

    // -----------------------------------------------------------------
    // Commands that do NOT require an API key
    // -----------------------------------------------------------------
    match &cli.command {
        Commands::Login { api_key } => {
            return commands::auth::cmd_login(
                api_key.as_deref(),
                cli.base_url.as_deref(),
                &config_path,
            )
            .await;
        }
        Commands::Logout => {
            return commands::auth::cmd_logout(&config_path);
        }
        Commands::Config => {
            return commands::config_cmd::cmd_config(&config_path);
        }
        _ => {} // fall through to API-key-requiring commands
    }

    // -----------------------------------------------------------------
    // Resolve API key: flag > env > config
    // -----------------------------------------------------------------
    let cfg = config::load(&config_path);
    let api_key = config::resolve_api_key(cli.api_key.as_deref(), None, &cfg).ok_or_else(|| {
        eyre!(
            "API key required. Set via --api-key, BANKR_API_KEY env var, or run `bankr-cli login`."
        )
    })?;

    let base_url =
        cli.base_url.as_deref().or(cfg.api_url.as_deref()).unwrap_or("https://api.bankr.bot");

    let client = BankrAgentClient::with_base_url(&api_key, base_url).map_err(|e| eyre!("{e}"))?;

    match cli.command {
        Commands::Whoami => {
            commands::whoami::cmd_whoami(&client, cli.raw, &config_path, &api_key, base_url).await
        }
        Commands::Skills => commands::skills::cmd_skills(&client, cli.raw).await,
        Commands::Prompt { prompt, thread_id, wait, poll_interval, max_attempts } => {
            commands::prompt::cmd_prompt(
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
        Commands::Status { job_id } => commands::job::cmd_job(&client, &job_id, cli.raw).await,
        Commands::Cancel { job_id } => commands::job::cmd_cancel(&client, &job_id, cli.raw).await,
        Commands::Sign { kind } => commands::sign::cmd_sign(&client, kind, cli.raw).await,
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
            commands::submit::cmd_submit(
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
        // Login / Logout / Config already handled above.
        Commands::Login { .. } | Commands::Logout | Commands::Config => unreachable!(),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

pub(crate) fn print_json<T: serde::Serialize>(value: &T, raw: bool) -> Result<()> {
    let output = if raw {
        serde_json::to_string(value).wrap_err("JSON serialization failed")?
    } else {
        serde_json::to_string_pretty(value).wrap_err("JSON serialization failed")?
    };
    println!("{output}");
    Ok(())
}
