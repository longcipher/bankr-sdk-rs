//! Error types for the `bankr-agent-api` crate.

use crate::types::ApiErrorBody;

/// Errors that can occur when interacting with the Bankr Agent API.
#[derive(Debug, thiserror::Error)]
pub enum BankrError {
    /// HTTP transport error from hpx-transport.
    #[error("HTTP transport error: {0}")]
    Transport(String),

    /// The API returned a non-success HTTP status code.
    #[error("API error (HTTP {status}): {body}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Parsed error body (if available).
        body: ApiErrorBody,
    },

    /// Failed to deserialize the API response.
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// Job polling timed out.
    #[error("Job polling timed out after {attempts} attempts")]
    PollTimeout {
        /// Number of poll attempts made.
        attempts: u32,
    },

    /// Job failed on the server side.
    #[error("Job failed: {message}")]
    JobFailed {
        /// Error message from the API.
        message: String,
    },

    /// Job was cancelled.
    #[error("Job was cancelled")]
    JobCancelled,

    /// Client configuration error.
    #[error("Configuration error: {0}")]
    Config(String),
}

impl std::fmt::Display for ApiErrorBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref msg) = self.message {
            write!(f, "{msg}")
        } else if let Some(ref err) = self.error {
            write!(f, "{err}")
        } else {
            write!(f, "(no details)")
        }
    }
}
