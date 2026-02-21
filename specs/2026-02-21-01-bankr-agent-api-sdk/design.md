# Design Document: Bankr Agent API SDK

| Metadata | Details |
| :--- | :--- |
| **Author** | pb-plan agent |
| **Status** | Draft |
| **Created** | 2026-02-21 |
| **Reviewers** | — |
| **Related Issues** | N/A |

## 1. Executive Summary

**Problem:** There is no Rust SDK for interacting with the Bankr Agent API. Developers need to manually construct HTTP requests to use the API.

**Solution:** Create a `bankr-agent-api` library crate with typed request/response models and an `hpx-transport`-based REST client, plus a `bankr-cli` binary for convenient API invocation and debugging.

---

## 2. Requirements & Goals

### 2.1 Problem Statement

The Bankr Agent API (<https://api.bankr.bot>) provides endpoints for submitting prompts, managing jobs, signing messages/transactions, and submitting transactions on-chain. Currently there is no Rust client library, requiring users to manually construct HTTP requests.

### 2.2 Functional Goals

1. **Typed API client:** Provide `BankrAgentClient` with methods for all Agent API endpoints.
2. **Request/response models:** Strongly-typed Rust structs for all API payloads.
3. **Job polling:** Built-in polling helper that waits for job completion.
4. **CLI tool:** `bankr-cli` binary with subcommands for each API endpoint.
5. **Submit endpoint support:** Construct and submit EVM transactions via the API.

### 2.3 Non-Functional Goals

- **Performance:** Use `hpx-transport` with `rustls` for TLS — no OpenSSL dependency.
- **Reliability:** Proper error handling with `thiserror` in the library crate.
- **Security:** API key loaded from environment variable, never hardcoded.

### 2.4 Out of Scope

- WebSocket/streaming support
- LLM Gateway endpoints
- Full on-chain transaction building (alloy/solana SDK integration is minimal for testing)

### 2.5 Assumptions

- `hpx` 2.1.0 and `hpx-transport` 2.1.0 are the target versions.
- The base URL is `https://api.bankr.bot`.
- Authentication is via `X-API-Key` header.

---

## 3. Architecture Overview

### 3.1 System Context

```text
bankr-cli (binary)
  └── bankr-agent-api (library)
        └── hpx-transport (RestClient + ApiKeyAuth)
              └── hpx (HTTP engine with rustls)
```

### 3.2 Key Design Principles

- Use `hpx-transport::RestClient` with `ApiKeyAuth` for all HTTP communication.
- All API types derive `Serialize`/`Deserialize` and `Debug`.
- Library errors use `thiserror`; CLI uses `eyre` for top-level error handling.
- CLI reads `BANKR_API_KEY` from env and passes it to the client.

### 3.3 Existing Components to Reuse

| Component | Location | How to Reuse |
| :--- | :--- | :--- |
| Workspace lint config | `Cargo.toml` | Inherit via `lints.workspace = true` |
| Workspace package metadata | `Cargo.toml` | Inherit `version`, `edition`, `license` |

---

## 4. Detailed Design

### 4.1 Module Structure

```text
crates/bankr-agent-api/
├── Cargo.toml
└── src/
    ├── lib.rs          # Re-exports
    ├── client.rs       # BankrAgentClient
    ├── error.rs        # Error types (thiserror)
    └── types.rs        # Request/response structs

bin/bankr-cli/
├── Cargo.toml
└── src/
    └── main.rs         # CLI entry point (clap)
```

### 4.2 Data Structures & Types

See `types.rs` — covers all API request/response models from the documentation.

### 4.3 Interface Design

```rust
impl BankrAgentClient {
    pub fn new(api_key: &str) -> Result<Self, BankrError>;
    pub fn with_base_url(api_key: &str, base_url: &str) -> Result<Self, BankrError>;
    pub async fn get_me(&self) -> Result<UserInfoResponse, BankrError>;
    pub async fn submit_prompt(&self, req: &PromptRequest) -> Result<PromptResponse, BankrError>;
    pub async fn get_job(&self, job_id: &str) -> Result<JobResponse, BankrError>;
    pub async fn cancel_job(&self, job_id: &str) -> Result<CancelJobResponse, BankrError>;
    pub async fn sign(&self, req: &SignRequest) -> Result<SignResponse, BankrError>;
    pub async fn submit_transaction(&self, req: &SubmitRequest) -> Result<SubmitResponse, BankrError>;
    pub async fn poll_job(&self, job_id: &str, interval: Duration, max_attempts: u32) -> Result<JobResponse, BankrError>;
}
```

### 4.4 Logic Flow

1. CLI parses args → constructs `BankrAgentClient` from env `BANKR_API_KEY`.
2. Dispatches to appropriate client method.
3. Client sends HTTP request via `hpx-transport::RestClient`.
4. Response deserialized into typed struct.
5. CLI prints JSON output.

### 4.5 Configuration

| Config | Source | Description |
| :--- | :--- | :--- |
| `BANKR_API_KEY` | Environment variable | API key for authentication |
| `BANKR_BASE_URL` | Environment variable (optional) | Override base URL (default: `https://api.bankr.bot`) |

### 4.6 Error Handling

- `BankrError` enum with variants for HTTP errors, API errors, deserialization errors.
- API error responses are parsed into structured `ApiErrorResponse`.

---

## 5. Verification & Testing Strategy

### 5.1 Critical Path Verification

| Verification Step | Command | Success Criteria |
| :--- | :--- | :--- |
| **VP-01** | `cargo build --workspace` | All crates compile without errors |
| **VP-02** | `cargo clippy --all -- -D warnings` | No clippy warnings |
| **VP-03** | `cargo run -p bankr-cli -- --help` | Help text displayed |

---

## 6. Implementation Plan

- [x] **Phase 1: Foundation** — Workspace deps, crate scaffolding, core types
- [x] **Phase 2: Core Logic** — Client implementation, error handling
- [x] **Phase 3: Integration** — CLI binary with all subcommands
- [ ] **Phase 4: Polish** — Tests, docs, lint fixes
