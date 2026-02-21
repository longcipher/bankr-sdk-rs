# Design Document: CLI Full Commands

| Metadata | Details |
| :--- | :--- |
| **Author** | pb-plan agent |
| **Status** | Draft |
| **Created** | 2026-02-21 |
| **Reviewers** | — |
| **Related Issues** | N/A |

## 1. Executive Summary

**Problem:** The current `bankr-cli` only supports a subset of the commands available in the official `@bankr/cli` npm package. It reads the API key exclusively from an environment variable or `--api-key` flag, whereas the npm CLI reads from `$HOME/.bankr/config.json`. The `whoami` output is raw JSON instead of a human-friendly display.

**Solution:** Extend `bankr-cli` to read configuration from `$HOME/.bankr/config.json`, add `login`/`logout`/`config` subcommands for credential management, rename `job` to `status` for npm CLI parity, improve `whoami` output to match the npm CLI's human-readable format, and add a `skills` subcommand — covering all commands that the `bankr-agent-api` crate can support.

---

## 2. Requirements & Goals

### 2.1 Problem Statement

The official Bankr npm CLI (`@bankr/cli`) supports `login`, `logout`, `whoami`, `prompt`, `status`, `cancel`, `config`, `skills`, `launch`, `sign`, `submit`, `llm`, and `help`. The Rust `bankr-cli` currently only implements `whoami`, `prompt`, `job`, `cancel`, `sign`, and `submit` — and lacks config-file support, human-friendly output, and several subcommands.

### 2.2 Functional Goals

1. **Config file support:** Read API key and base URL from `$HOME/.bankr/config.json` (fallback chain: `--api-key` flag → `BANKR_API_KEY` env → config file).
2. **`--config <path>` flag:** Allow overriding the config file path, matching `bankr --config <path>`.
3. **`login` subcommand:** Prompt for API key interactively, write to config file, verify with `GET /agent/me`.
4. **`logout` subcommand:** Remove stored credentials from the config file.
5. **`whoami` subcommand:** Pretty-print account info (wallets, socials, club status, referral code, score) matching the npm CLI format.
6. **`config` subcommand:** Show current config values (API key masked, base URL, config path).
7. **`status <jobId>` subcommand:** Rename current `job` → `status` to match npm CLI. Keep `job` as a hidden alias.
8. **`skills` subcommand:** Submit a "show my skills" prompt and display the agent's skill list.
9. **Retain existing commands:** `prompt`, `cancel`, `sign`, `submit` remain unchanged.

### 2.3 Non-Functional Goals

- **Performance:** Config file I/O is synchronous and negligible. No new network calls except where explicitly needed.
- **Reliability:** Graceful degradation when config file is missing or malformed.
- **Security:** API key is masked when displayed (show first 6 + last 4 characters). Config file created with `0600` permissions on Unix.
- **UX:** Colored terminal output using ANSI codes or a minimal crate (`console` or inline ANSI). Checkmarks (✔) and crosses (✖) for visual feedback matching the npm CLI.

### 2.4 Out of Scope

- **`launch` subcommand:** Requires an interactive wizard with multi-step prompts — complex UI beyond current scope.
- **`llm` subcommand:** Requires the separate LLM Gateway API which `bankr-agent-api` does not implement.
- **WebSocket/streaming support.**
- **Interactive prompt REPL mode.**

### 2.5 Assumptions

- The npm CLI's config file format is `{ "apiKey": "bk_...", "apiUrl": "https://api.bankr.bot" }`.
- The default config path is `$HOME/.bankr/config.json` on all platforms.
- The `login` flow accepts the API key via stdin (interactive) or `--api-key` flag (non-interactive).
- The `skills` subcommand can be implemented by sending a known prompt to the agent and displaying the response.

---

## 3. Architecture Overview

### 3.1 System Context

```text
┌─────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  bankr-cli  │────▶│ bankr-agent-api  │────▶│ api.bankr.bot   │
│  (binary)   │     │   (library)      │     │  (Bankr API)    │
└──────┬──────┘     └──────────────────┘     └─────────────────┘
       │
       ▼
 ~/.bankr/config.json
```

The CLI binary adds a thin config/UX layer on top of the existing `bankr-agent-api` client library. All API calls go through `BankrAgentClient`. Config management is purely local file I/O.

### 3.2 Key Design Principles

1. **Config fallback chain:** `--api-key` flag > `BANKR_API_KEY` env > config file. This preserves backward compatibility while adding config file support.
2. **Module separation:** Config handling and pretty-printing live in separate modules, not in `main.rs`.
3. **npm CLI parity:** Match the output format of `@bankr/cli` wherever reasonable.
4. **Minimal new dependencies:** Prefer `serde_json` (already in tree) for config. Use ANSI escape codes directly or add `console` crate for colored output.

### 3.3 Existing Components to Reuse

| Component | Location | How to Reuse |
| :--- | :--- | :--- |
| `BankrAgentClient` | `crates/bankr-agent-api/src/client.rs` | Use for all API calls (`get_me`, `submit_prompt`, etc.) |
| `UserInfoResponse` / all types | `crates/bankr-agent-api/src/types.rs` | Deserialize API responses; format for display |
| `BankrError` | `crates/bankr-agent-api/src/error.rs` | Propagate API errors to CLI error messages |
| CLI structure (clap) | `bin/bankr-cli/src/main.rs` | Extend existing `Commands` enum with new subcommands |
| `print_json` helper | `bin/bankr-cli/src/main.rs` | Reuse for `--raw` JSON output mode |

---

## 4. Detailed Design

### 4.1 Module Structure

```text
bin/bankr-cli/src/
├── main.rs          # CLI entry point, clap definitions, command dispatch
├── config.rs        # Config file reading/writing, BankrConfig struct
├── display.rs       # Pretty-print helpers (whoami, job status, etc.)
├── commands/
│   ├── mod.rs       # Re-export all command handlers
│   ├── auth.rs      # login, logout
│   ├── whoami.rs    # whoami command handler
│   ├── prompt.rs    # prompt command handler
│   ├── job.rs       # status/cancel command handlers
│   ├── sign.rs      # sign command handler
│   ├── submit.rs    # submit command handler
│   ├── config.rs    # config show subcommand handler
│   └── skills.rs    # skills command handler
```

### 4.2 Data Structures & Types

```rust
/// Config file structure matching $HOME/.bankr/config.json
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BankrConfig {
    /// API key (e.g., "bk_WKW...46ZE").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// API base URL (default: "https://api.bankr.bot").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
}
```

### 4.3 Interface Design

**Config resolution:**

```rust
/// Resolve the API key from (in priority order):
/// 1. --api-key CLI flag
/// 2. BANKR_API_KEY environment variable
/// 3. Config file (--config path or default)
fn resolve_api_key(cli_flag: Option<&str>, config: &BankrConfig) -> Option<String>;
```

**CLI commands (updated clap enum):**

```rust
#[derive(Debug, Subcommand)]
enum Commands {
    /// Authenticate with the Bankr API.
    Login {
        /// Provide API key non-interactively.
        #[arg(long)]
        api_key: Option<String>,
    },
    /// Clear stored credentials.
    Logout,
    /// Show current authentication info.
    Whoami,
    /// Send a prompt to the Bankr AI agent.
    Prompt { /* existing fields */ },
    /// Check the status of a job.
    Status { job_id: String },
    /// Cancel a running job.
    Cancel { job_id: String },
    /// Sign messages, typed data, or transactions.
    Sign { /* existing fields */ },
    /// Submit a transaction to the blockchain.
    Submit { /* existing fields */ },
    /// Manage CLI configuration.
    Config,
    /// Show all Bankr AI agent skills with examples.
    Skills,
}
```

### 4.4 Logic Flow

**Login flow:**

1. If `--api-key` provided, use it; otherwise prompt interactively (`Enter your Bankr API key:`).
2. Create `BankrAgentClient` with the key.
3. Call `get_me()` to verify the key works.
4. On success: write key to config file, print success message with account summary.
5. On failure: print error, do not write config.

**Whoami pretty-print format:**

```text
✔ Bankr API connection OK
✔ Account info loaded

Bankr API Key:  bk_WKW...46ZE
Bankr API URL:  https://api.bankr.bot
Source:  /Users/user/.bankr/config.json
Config:  /Users/user/.bankr/config.json

Wallets:
  EVM      0x3582670fcdca408af4143712694bae724a128e83
  SOLANA   BdHTBcwq4jxo2y6tqW2pe7FktoZMJU8T3gnxCGzWmctJ

Social Accounts:
  email        user@example.com

Bankr Club:  Active (monthly)
Referral Code:  NT3LZYXW-BNKR
Score:  1250
```

**Config file operations:**

- `load()` → Read and deserialize, return `Default` if missing/malformed.
- `save()` → Serialize and write, create `~/.bankr/` directory if needed, set file permissions to `0600`.

### 4.5 Configuration

| Config Key | Default | Description |
| :--- | :--- | :--- |
| `apiKey` | (none) | Bankr API key |
| `apiUrl` | `https://api.bankr.bot` | API base URL |

Config path: `$HOME/.bankr/config.json` (override with `--config <path>`).

### 4.6 Error Handling

- Missing API key across all sources → clear error message listing all three sources.
- Config file parse error → warn and continue with empty config (do not crash).
- API connection failure during `login`/`whoami` → display `✖ Bankr API connection failed` with error details.
- File permission errors when writing config → report and suggest manual fix.

---

## 5. Verification & Testing Strategy

### 5.1 Unit Testing

- `config.rs`: Test `load`/`save` round-trip, missing file handling, malformed JSON handling, API key masking.
- `display.rs`: Test formatting functions with known `UserInfoResponse` fixtures.
- Config resolution: Test priority chain (flag > env > file).

### 5.2 Integration Testing

- Build the CLI binary and run subcommands against a mock or the real API.
- `bankr-cli config` should work without an API connection.
- `bankr-cli login --api-key <key>` should create the config file.

### 5.3 Critical Path Verification (The "Harness")

| Verification Step | Command | Success Criteria |
| :--- | :--- | :--- |
| **VP-01** | `cargo build --workspace` | Compiles without errors |
| **VP-02** | `cargo test --all-features` | All tests pass |
| **VP-03** | `cargo +nightly clippy --all -- -D warnings` | No warnings |
| **VP-04** | `./target/debug/bankr-cli --help` | Shows all subcommands including `login`, `logout`, `config`, `status`, `skills` |
| **VP-05** | `./target/debug/bankr-cli config` | Shows config path and values (no API key needed) |
| **VP-06** | `BANKR_API_KEY=bk_test ./target/debug/bankr-cli whoami` | Pretty-printed account info or clear API error |

### 5.4 Validation Rules

| Test Case ID | Action | Expected Outcome | Verification Method |
| :--- | :--- | :--- | :--- |
| **TC-01** | Run `bankr-cli whoami` with valid key in config | Pretty-printed wallets, socials, club, score | Visual inspection |
| **TC-02** | Run `bankr-cli whoami --raw` with valid key | Raw JSON output | `jq` parses successfully |
| **TC-03** | Run `bankr-cli login --api-key bk_test` | Config file written, key verified | Check `~/.bankr/config.json` exists |
| **TC-04** | Run `bankr-cli logout` | Config file cleared | Check config no longer has `apiKey` |
| **TC-05** | Run `bankr-cli status <id>` | Same as old `bankr-cli job <id>` | Compare JSON output |
| **TC-06** | Run with `--api-key` flag | Flag takes priority over config file | Verify correct key used |
| **TC-07** | Run `bankr-cli config` | Shows masked key, URL, path | Visual inspection |
| **TC-08** | Run `bankr-cli skills` | Shows agent skills | Visual inspection |

---

## 6. Implementation Plan

- [ ] **Phase 1: Foundation** — Config module, directory structure, `BankrConfig` type
- [ ] **Phase 2: Core Logic** — `login`, `logout`, `config`, `whoami` (pretty), `status` rename
- [ ] **Phase 3: Integration** — `skills` command, wire config into all existing commands, output formatting
- [ ] **Phase 4: Polish** — Tests, error messages, documentation, `--help` text

---

## 7. Cross-Functional Concerns

- **Backward compatibility:** The `BANKR_API_KEY` env var and `--api-key` flag continue to work. The `job` subcommand is kept as a hidden alias for `status`.
- **Security:** Config file is created with `0600` permissions. API key is always masked in display (first 6 + last 4 chars, e.g., `bk_WKW...46ZE`).
- **Cross-platform:** Config path uses `$HOME` on Unix/macOS. On Windows, `USERPROFILE` is used. The `dirs` or `home` crate can provide `home_dir()` portably, but `std::env::var("HOME")` is acceptable for Unix-only initial support.
