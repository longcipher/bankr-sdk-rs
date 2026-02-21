//! Request and response types for the Bankr Agent API.
//!
//! All types are derived from the official API documentation at
//! <https://docs.bankr.bot/agent-api/overview>.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// User Info — GET /agent/me
// ---------------------------------------------------------------------------

/// Wallet entry returned by the `/agent/me` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    /// Chain identifier (`"evm"` or `"solana"`).
    pub chain: String,
    /// Wallet address.
    pub address: String,
}

/// Social account linked to the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialAccount {
    /// Platform name (e.g. `farcaster`, `twitter`, `telegram`).
    pub platform: String,
    /// Username on that platform.
    pub username: Option<String>,
}

/// Bankr Club subscription info.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankrClub {
    /// Whether the subscription is active.
    pub active: bool,
    /// `"monthly"` or `"yearly"`.
    pub subscription_type: Option<String>,
    /// Unix timestamp (ms) of next renewal or cancellation.
    pub renew_or_cancel_on: Option<u64>,
}

/// Leaderboard entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leaderboard {
    /// User score.
    pub score: u64,
    /// Leaderboard rank.
    pub rank: Option<u64>,
}

/// Successful response from `GET /agent/me`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoResponse {
    /// Always `true` on success.
    pub success: bool,
    /// Wallet addresses.
    pub wallets: Vec<Wallet>,
    /// Connected social accounts.
    pub social_accounts: Vec<SocialAccount>,
    /// Referral code.
    pub ref_code: Option<String>,
    /// Bankr Club subscription info.
    pub bankr_club: Option<BankrClub>,
    /// Leaderboard info.
    pub leaderboard: Option<Leaderboard>,
}

// ---------------------------------------------------------------------------
// Prompt — POST /agent/prompt
// ---------------------------------------------------------------------------

/// Request body for `POST /agent/prompt`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptRequest {
    /// Natural language command (max 10 000 characters).
    pub prompt: String,
    /// Optional thread ID to continue a conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

/// Success response (202 Accepted) from `POST /agent/prompt`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptResponse {
    /// Always `true` on success.
    pub success: bool,
    /// Unique job identifier.
    pub job_id: String,
    /// Conversation thread ID.
    pub thread_id: String,
    /// Current status (always `"pending"` on creation).
    pub status: String,
    /// Human-readable message.
    pub message: String,
}

// ---------------------------------------------------------------------------
// Job Management — GET /agent/job/{jobId}, POST /agent/job/{jobId}/cancel
// ---------------------------------------------------------------------------

/// Job status values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JobStatus {
    /// Job is queued for processing.
    Pending,
    /// Job is currently being processed.
    Processing,
    /// Job finished successfully.
    Completed,
    /// Job encountered an error.
    Failed,
    /// Job was cancelled by the user.
    Cancelled,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Processing => write!(f, "processing"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// A single status-update entry within a job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusUpdate {
    /// Status update message.
    pub message: Option<String>,
    /// Timestamp of the update.
    pub timestamp: Option<String>,
}

/// Rich data item returned with completed jobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichDataItem {
    /// Type discriminator (e.g. `"token_info"`, `"chart"`).
    #[serde(rename = "type")]
    pub kind: String,
    /// Remaining fields vary by type.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Response from `GET /agent/job/{jobId}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobResponse {
    /// Whether the request succeeded.
    pub success: bool,
    /// Job identifier.
    pub job_id: String,
    /// Conversation thread ID.
    pub thread_id: Option<String>,
    /// Current job status.
    pub status: JobStatus,
    /// Original prompt submitted.
    pub prompt: String,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
    /// Whether the job can still be cancelled.
    pub cancellable: Option<bool>,
    /// Progress messages during processing.
    pub status_updates: Option<Vec<StatusUpdate>>,
    /// When processing started.
    pub started_at: Option<String>,
    /// Agent response text (when completed).
    pub response: Option<String>,
    /// Additional structured data (when completed).
    pub rich_data: Option<Vec<RichDataItem>>,
    /// When the job finished (completed or failed).
    pub completed_at: Option<String>,
    /// Processing duration in milliseconds (when completed).
    pub processing_time: Option<u64>,
    /// Error message (when failed).
    pub error: Option<String>,
    /// When the job was cancelled.
    pub cancelled_at: Option<String>,
}

/// Response from `POST /agent/job/{jobId}/cancel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelJobResponse {
    /// Whether the request succeeded.
    pub success: bool,
    /// Job identifier.
    pub job_id: String,
    /// Status after cancellation.
    pub status: String,
    /// Original prompt.
    pub prompt: Option<String>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Cancellation timestamp.
    pub cancelled_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Sign — POST /agent/sign
// ---------------------------------------------------------------------------

/// Signature type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignatureType {
    /// Standard Ethereum personal-sign.
    #[serde(rename = "personal_sign")]
    PersonalSign,
    /// EIP-712 structured data signing.
    #[serde(rename = "eth_signTypedData_v4")]
    EthSignTypedDataV4,
    /// Sign a transaction without broadcasting.
    #[serde(rename = "eth_signTransaction")]
    EthSignTransaction,
}

impl std::fmt::Display for SignatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PersonalSign => write!(f, "personal_sign"),
            Self::EthSignTypedDataV4 => write!(f, "eth_signTypedData_v4"),
            Self::EthSignTransaction => write!(f, "eth_signTransaction"),
        }
    }
}

/// EVM transaction parameters used by both sign and submit endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmTransaction {
    /// Destination address.
    pub to: String,
    /// Chain ID.
    pub chain_id: u64,
    /// Value in wei (as string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Calldata (hex string starting with `0x`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Gas limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<String>,
    /// Legacy gas price in wei.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    /// EIP-1559 max fee per gas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee_per_gas: Option<String>,
    /// EIP-1559 priority fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_priority_fee_per_gas: Option<String>,
    /// Transaction nonce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
}

/// Request body for `POST /agent/sign`.
///
/// The body shape depends on `signature_type`:
/// - `personal_sign` → `message` is required
/// - `eth_signTypedData_v4` → `typed_data` is required
/// - `eth_signTransaction` → `transaction` is required
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignRequest {
    /// The type of signature to produce.
    pub signature_type: SignatureType,
    /// Plain text message (for `personal_sign`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// EIP-712 typed data (for `eth_signTypedData_v4`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_data: Option<serde_json::Value>,
    /// Transaction to sign (for `eth_signTransaction`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<EvmTransaction>,
}

/// Success response from `POST /agent/sign`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignResponse {
    /// `true` if signing succeeded.
    pub success: bool,
    /// The hex-encoded signature.
    pub signature: Option<String>,
    /// Address that produced the signature.
    pub signer: Option<String>,
    /// The signature type used.
    pub signature_type: Option<SignatureType>,
    /// Error message if signing failed.
    pub error: Option<String>,
}

// ---------------------------------------------------------------------------
// Submit — POST /agent/submit
// ---------------------------------------------------------------------------

/// Request body for `POST /agent/submit`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitRequest {
    /// The transaction to submit.
    pub transaction: EvmTransaction,
    /// Human-readable description for logging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Wait for on-chain confirmation (default: `true`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_confirmation: Option<bool>,
}

/// Response from `POST /agent/submit`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResponse {
    /// `true` if submission succeeded.
    pub success: bool,
    /// The transaction hash.
    pub transaction_hash: Option<String>,
    /// `"success"`, `"reverted"`, `"pending"`, or `"failed"`.
    pub status: Option<String>,
    /// Block number (if confirmed).
    pub block_number: Option<String>,
    /// Gas used (if confirmed).
    pub gas_used: Option<String>,
    /// Address that signed the transaction.
    pub signer: Option<String>,
    /// Chain ID.
    pub chain_id: Option<u64>,
    /// Error message if submission failed.
    pub error: Option<String>,
}

// ---------------------------------------------------------------------------
// Common API Error Response
// ---------------------------------------------------------------------------

/// Standard error envelope returned by the Bankr API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorBody {
    /// Error type/title.
    pub error: Option<String>,
    /// Human-readable error message.
    pub message: Option<String>,
    /// When the rate-limit counter resets (Unix ms) — for 429 errors.
    pub reset_at: Option<u64>,
    /// Rate-limit quota.
    pub limit: Option<u64>,
    /// Number of messages used in the current window.
    pub used: Option<u64>,
}
