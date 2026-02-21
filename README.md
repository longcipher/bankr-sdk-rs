# bankr-sdk-rs

[![crates.io](https://img.shields.io/crates/v/bankr-agent-api.svg)](https://crates.io/crates/bankr-agent-api)
[![docs.rs](https://docs.rs/bankr-agent-api/badge.svg)](https://docs.rs/bankr-agent-api)

Rust SDK and CLI for the [Bankr Agent API](https://docs.bankr.bot/agent-api/overview).

## Crates

| Crate | Description |
|---|---|
| [`bankr-agent-api`](crates/bankr-agent-api) | Async Rust client library for the Bankr Agent API |
| [`bankr-cli`](bin/bankr-cli) | Command-line tool for API exploration and debugging |

## Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
bankr-agent-api = "0.1"
```

### Quick Start

```rust,no_run
use bankr_agent_api::{BankrAgentClient, types::PromptRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BankrAgentClient::new("your_api_key")?;

    // Get your Bankr profile
    let me = client.get_me().await?;
    println!("Wallets: {:?}", me.wallets);

    // Submit a prompt and wait for completion
    let req = PromptRequest {
        prompt: "what is the price of ETH?".to_owned(),
        thread_id: None,
    };
    let job = client.prompt_and_wait(&req).await?;
    println!("Response: {:?}", job.response);

    Ok(())
}
```

## CLI Usage

### Installation

```bash
cargo install bankr-cli
```

### Authentication

Set your Bankr API key via environment variable (recommended):

```bash
export BANKR_API_KEY=your_api_key_here
```

Or pass it directly with `--api-key <KEY>`.

### Commands

```bash
# Show your authenticated user profile
bankr-cli whoami

# Submit a prompt (fire-and-forget, prints job ID)
bankr-cli prompt "what is the price of ETH?"

# Submit a prompt and wait for the result
bankr-cli prompt --wait "swap 0.01 ETH for USDC"

# Submit a prompt inside an existing thread
bankr-cli prompt --thread-id <THREAD_ID> "follow-up question"

# Poll a job by ID
bankr-cli job <JOB_ID>

# Cancel a pending or processing job
bankr-cli cancel <JOB_ID>

# Sign a personal message
bankr-cli sign personal "hello world"

# Sign EIP-712 typed data (pass raw JSON)
bankr-cli sign typed-data '{"domain":{...},"message":{...}}'

# Sign a transaction without broadcasting
bankr-cli sign transaction --to 0xRecipient --chain-id 1 --value 1000000

# Submit a raw EVM transaction
bankr-cli submit --to 0xRecipient --chain-id 1 --value 1000000
```

Run `bankr-cli --help` or `bankr-cli <COMMAND> --help` for the full list of options.

## Authentication

Obtain your Bankr API key from [bankr.bot](https://bankr.bot). Pass it via:

1. `BANKR_API_KEY` environment variable *(recommended)*
2. `--api-key <KEY>` global flag

## API Coverage

| Endpoint | Method | Description |
|---|---|---|
| `/agent/me` | GET | Authenticated user profile |
| `/agent/prompt` | POST | Submit a natural language prompt |
| `/agent/job/{jobId}` | GET | Get job status and result |
| `/agent/job/{jobId}/cancel` | POST | Cancel a pending job |
| `/agent/sign` | POST | Sign a message, typed data, or transaction |
| `/agent/submit` | POST | Submit a raw EVM transaction |

See the [Bankr API docs](https://docs.bankr.bot/agent-api/overview) for the full reference.

## Development

```bash
# Run all checks (lint + test + build)
just ci

# Format code
just format

# Run lints
just lint

# Run tests
just test

# Build
just build

# Generate and open documentation
just docs
```
