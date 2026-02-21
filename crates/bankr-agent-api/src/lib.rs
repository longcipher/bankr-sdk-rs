//! # bankr-agent-api
//!
//! Rust client library for the [Bankr Agent API](https://docs.bankr.bot/agent-api/overview).
//!
//! ## Quick start
//!
//! ```rust,no_run
//! use bankr_agent_api::{BankrAgentClient, types::PromptRequest};
//!
//! # async fn example() -> Result<(), bankr_agent_api::error::BankrError> {
//! let client = BankrAgentClient::new("your_api_key")?;
//!
//! // Get user profile
//! let me = client.get_me().await?;
//! println!("Wallets: {:?}", me.wallets);
//!
//! // Submit a prompt and wait for the result
//! let req = PromptRequest { prompt: "what is the price of ETH?".to_owned(), thread_id: None };
//! let job = client.prompt_and_wait(&req).await?;
//! println!("Response: {:?}", job.response);
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod error;
pub mod types;

// Re-export the main client type at crate root for convenience.
pub use client::BankrAgentClient;
