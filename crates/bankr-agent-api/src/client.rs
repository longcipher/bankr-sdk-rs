//! Bankr Agent API client built on `hpx-transport`.

use std::time::Duration;

use hpx_transport::{
    ExchangeClient, TypedResponse,
    auth::ApiKeyAuth,
    exchange::{RestClient, RestConfig},
};
use tracing::{debug, info, warn};

use crate::{
    error::BankrError,
    types::{
        CancelJobResponse, JobResponse, JobStatus, PromptRequest, PromptResponse, SignRequest,
        SignResponse, SubmitRequest, SubmitResponse, UserInfoResponse,
    },
};

/// Default base URL for the Bankr Agent API.
const DEFAULT_BASE_URL: &str = "https://api.bankr.bot";

/// Default request timeout.
const DEFAULT_TIMEOUT: Duration = Duration::from_mins(1);

/// Default polling interval when waiting for a job to complete.
const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(2);

/// Default maximum number of poll attempts.
const DEFAULT_MAX_POLL_ATTEMPTS: u32 = 60;

/// Client for the Bankr Agent API.
///
/// Uses `hpx-transport`'s [`RestClient`] with [`ApiKeyAuth`] to communicate
/// with `https://api.bankr.bot`.
#[derive(Debug)]
pub struct BankrAgentClient {
    rest: RestClient<ApiKeyAuth>,
}

impl BankrAgentClient {
    /// Create a new client with the given API key and the default base URL.
    ///
    /// # Errors
    ///
    /// Returns [`BankrError::Config`] if the underlying HTTP client cannot be
    /// created.
    pub fn new(api_key: &str) -> Result<Self, BankrError> {
        Self::with_base_url(api_key, DEFAULT_BASE_URL)
    }

    /// Create a new client with a custom base URL.
    ///
    /// # Errors
    ///
    /// Returns [`BankrError::Config`] if the underlying HTTP client cannot be
    /// created.
    pub fn with_base_url(api_key: &str, base_url: &str) -> Result<Self, BankrError> {
        let config =
            RestConfig::new(base_url).timeout(DEFAULT_TIMEOUT).user_agent("bankr-sdk-rs/0.1.0");

        let auth = ApiKeyAuth::header("X-API-Key", api_key);

        let rest = RestClient::new(config, auth).map_err(|e| BankrError::Config(e.to_string()))?;

        Ok(Self { rest })
    }

    // -----------------------------------------------------------------------
    // User Info
    // -----------------------------------------------------------------------

    /// Retrieve the authenticated user's profile.
    ///
    /// `GET /agent/me`
    pub async fn get_me(&self) -> Result<UserInfoResponse, BankrError> {
        debug!("GET /agent/me");
        let resp: TypedResponse<UserInfoResponse> =
            self.rest.get("/agent/me").await.map_err(transport_err)?;
        Ok(resp.data)
    }

    // -----------------------------------------------------------------------
    // Prompt
    // -----------------------------------------------------------------------

    /// Submit a natural language prompt to the Bankr AI agent.
    ///
    /// `POST /agent/prompt`
    pub async fn submit_prompt(&self, req: &PromptRequest) -> Result<PromptResponse, BankrError> {
        debug!(prompt = %req.prompt, "POST /agent/prompt");
        let resp: TypedResponse<PromptResponse> =
            self.rest.post("/agent/prompt", req).await.map_err(transport_err)?;
        Ok(resp.data)
    }

    // -----------------------------------------------------------------------
    // Job Management
    // -----------------------------------------------------------------------

    /// Get the status of a previously submitted job.
    ///
    /// `GET /agent/job/{jobId}`
    pub async fn get_job(&self, job_id: &str) -> Result<JobResponse, BankrError> {
        debug!(job_id, "GET /agent/job/{job_id}");
        let path = format!("/agent/job/{job_id}");
        let resp: TypedResponse<JobResponse> = self.rest.get(&path).await.map_err(transport_err)?;
        Ok(resp.data)
    }

    /// Cancel a pending or processing job.
    ///
    /// `POST /agent/job/{jobId}/cancel`
    pub async fn cancel_job(&self, job_id: &str) -> Result<CancelJobResponse, BankrError> {
        debug!(job_id, "POST /agent/job/{job_id}/cancel");
        let path = format!("/agent/job/{job_id}/cancel");
        // The cancel endpoint expects an empty POST body.
        let empty = serde_json::json!({});
        let resp: TypedResponse<CancelJobResponse> =
            self.rest.post(&path, &empty).await.map_err(transport_err)?;
        Ok(resp.data)
    }

    // -----------------------------------------------------------------------
    // Sign
    // -----------------------------------------------------------------------

    /// Sign a message, typed data, or transaction without broadcasting.
    ///
    /// `POST /agent/sign`
    pub async fn sign(&self, req: &SignRequest) -> Result<SignResponse, BankrError> {
        debug!(sig_type = %req.signature_type, "POST /agent/sign");
        let resp: TypedResponse<SignResponse> =
            self.rest.post("/agent/sign", req).await.map_err(transport_err)?;
        Ok(resp.data)
    }

    // -----------------------------------------------------------------------
    // Submit
    // -----------------------------------------------------------------------

    /// Submit a raw EVM transaction to the blockchain.
    ///
    /// `POST /agent/submit`
    pub async fn submit_transaction(
        &self,
        req: &SubmitRequest,
    ) -> Result<SubmitResponse, BankrError> {
        debug!(chain_id = req.transaction.chain_id, "POST /agent/submit");
        let resp: TypedResponse<SubmitResponse> =
            self.rest.post("/agent/submit", req).await.map_err(transport_err)?;
        Ok(resp.data)
    }

    // -----------------------------------------------------------------------
    // Polling helper
    // -----------------------------------------------------------------------

    /// Submit a prompt and poll until the job completes (or fails / is
    /// cancelled).
    ///
    /// Uses the default polling interval (2 s) and max attempts (60).
    pub async fn prompt_and_wait(&self, req: &PromptRequest) -> Result<JobResponse, BankrError> {
        self.prompt_and_wait_with(req, DEFAULT_POLL_INTERVAL, DEFAULT_MAX_POLL_ATTEMPTS).await
    }

    /// Submit a prompt and poll with custom interval and attempt count.
    pub async fn prompt_and_wait_with(
        &self,
        req: &PromptRequest,
        interval: Duration,
        max_attempts: u32,
    ) -> Result<JobResponse, BankrError> {
        let prompt_resp = self.submit_prompt(req).await?;
        info!(job_id = %prompt_resp.job_id, "Job submitted, polling…");
        self.poll_job(&prompt_resp.job_id, interval, max_attempts).await
    }

    /// Poll a job until it reaches a terminal state.
    pub async fn poll_job(
        &self,
        job_id: &str,
        interval: Duration,
        max_attempts: u32,
    ) -> Result<JobResponse, BankrError> {
        for attempt in 1..=max_attempts {
            let job = self.get_job(job_id).await?;
            debug!(attempt, status = %job.status, "Poll attempt");

            match job.status {
                JobStatus::Completed => return Ok(job),
                JobStatus::Failed => {
                    return Err(BankrError::JobFailed {
                        message: job.error.unwrap_or_else(|| "unknown error".to_owned()),
                    });
                }
                JobStatus::Cancelled => return Err(BankrError::JobCancelled),
                JobStatus::Pending | JobStatus::Processing => {
                    if attempt < max_attempts {
                        tokio::time::sleep(interval).await;
                    }
                }
            }
        }

        warn!(job_id, "Poll timeout reached");
        Err(BankrError::PollTimeout { attempts: max_attempts })
    }
}

/// Convert an `hpx_transport` error into a [`BankrError`].
fn transport_err(err: impl std::fmt::Display) -> BankrError {
    BankrError::Transport(err.to_string())
}
