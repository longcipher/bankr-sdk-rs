# Bankr Agent API SDK — Implementation Tasks

| Metadata | Details |
| :--- | :--- |
| **Design Doc** | specs/2026-02-21-01-bankr-agent-api-sdk/design.md |
| **Owner** | pb-plan agent |
| **Start Date** | 2026-02-21 |
| **Target Date** | 2026-02-21 |
| **Status** | In Progress |

## Summary & Phasing

- **Phase 1: Foundation & Scaffolding** — Add workspace deps, create crate skeletons
- **Phase 2: Core Logic** — Types, error handling, client implementation
- **Phase 3: Integration** — CLI binary with all subcommands
- **Phase 4: Polish** — Lint, format, test

---

## Phase 1: Foundation & Scaffolding

### Task 1.1: Update Workspace Dependencies

> **Context:** Add `hpx`, `hpx-transport`, `serde_json`, `tokio`, `clap` to workspace deps.
> **Verification:** `cargo check` succeeds.

- [x] Add dependencies to root `Cargo.toml`
- [x] Verification: `cargo check` succeeds

### Task 1.2: Create bankr-agent-api Crate Skeleton

> **Context:** Create `crates/bankr-agent-api/` with Cargo.toml and src/lib.rs.
> **Verification:** `cargo check -p bankr-agent-api` succeeds.

- [x] Create `Cargo.toml` with workspace deps
- [x] Create `src/lib.rs` with module declarations

### Task 1.3: Create bankr-cli Crate Skeleton

> **Context:** Create `bin/bankr-cli/` with Cargo.toml and src/main.rs.
> **Verification:** `cargo check -p bankr-cli` succeeds.

- [x] Create `Cargo.toml` with workspace deps
- [x] Create `src/main.rs` with basic clap setup

---

## Phase 2: Core Logic

### Task 2.1: Implement API Types

> **Context:** Define all request/response structs based on API documentation.
> **Verification:** Types compile and are well-documented.

- [x] Implement prompt request/response types
- [x] Implement job status/response types
- [x] Implement sign request/response types
- [x] Implement submit request/response types
- [x] Implement user info response types

### Task 2.2: Implement Error Types

> **Context:** Create `BankrError` enum using `thiserror`.
> **Verification:** Error types compile and provide good messages.

- [x] Define `BankrError` with HTTP, API, serde variants
- [x] Implement conversions

### Task 2.3: Implement BankrAgentClient

> **Context:** REST client using `hpx-transport::RestClient` with `ApiKeyAuth`.
> **Verification:** All methods compile; types match API docs.

- [x] Constructor with API key and optional base URL
- [x] `get_me()` method
- [x] `submit_prompt()` method
- [x] `get_job()` / `cancel_job()` methods
- [x] `sign()` method
- [x] `submit_transaction()` method
- [x] `poll_job()` helper

---

## Phase 3: Integration

### Task 3.1: Implement CLI Subcommands

> **Context:** Wire up clap subcommands to BankrAgentClient methods.
> **Verification:** `cargo run -p bankr-cli -- --help` shows all commands.

- [x] `whoami` command
- [x] `prompt` command with optional polling
- [x] `job` command
- [x] `cancel` command
- [x] `sign` command (personal_sign, typed data, transaction)
- [x] `submit` command

---

## Phase 4: Polish

### Task 4.1: Lint and Build Verification

> **Context:** Ensure everything compiles clean.
> **Verification:** `cargo build --workspace` and `cargo clippy --all` pass.

- [ ] Fix all compilation errors
- [ ] Fix all clippy warnings
- [ ] Verify `--help` output

---

## Summary & Timeline

| Phase | Tasks | Target Date |
| :--- | :---: | :--- |
| **1. Foundation** | 3 | 02-21 |
| **2. Core Logic** | 3 | 02-21 |
| **3. Integration** | 1 | 02-21 |
| **4. Polish** | 1 | 02-21 |
| **Total** | **8** | |
