# CLI Full Commands — Implementation Tasks

| Metadata | Details |
| :--- | :--- |
| **Design Doc** | specs/2026-02-21-02-cli-full-commands/design.md |
| **Owner** | — |
| **Start Date** | 2026-02-21 |
| **Target Date** | 2026-02-23 |
| **Status** | Completed |

## Summary & Phasing

Extend `bankr-cli` to achieve feature parity with the `@bankr/cli` npm package for all commands that the `bankr-agent-api` crate can support. The work is divided into four phases: foundation (config module), core commands (login/logout/whoami/config/status), integration (skills, output formatting), and polish (tests, docs).

- **Phase 1: Foundation & Scaffolding** — Config module, directory restructure, new dependencies
- **Phase 2: Core Logic** — login, logout, config, whoami pretty-print, status rename
- **Phase 3: Integration & Features** — skills command, wire config fallback into all commands, display module
- **Phase 4: Polish, QA & Docs** — Unit tests, error message refinement, help text, documentation

---

## Phase 1: Foundation & Scaffolding

### Task 1.1: Create config module

> **Context:** The npm CLI reads credentials from `$HOME/.bankr/config.json`. We need an equivalent config module that can load, save, and resolve configuration. This is the foundation for `login`, `logout`, `config`, and `whoami` commands.
> **Verification:** Config round-trip test passes — write then read back gives identical values.

- **Priority:** P0
- **Scope:** Config file I/O module
- **Status:** 🟢 DONE

- [x] **Step 1:** Add `dirs` or `home` crate to workspace dependencies for portable home directory resolution. Add to `bankr-cli/Cargo.toml` with `workspace = true`.
- [x] **Step 2:** Create `bin/bankr-cli/src/config.rs` with:
  - `BankrConfig` struct (`api_key: Option<String>`, `api_url: Option<String>`) with `Serialize`/`Deserialize`, `#[serde(rename_all = "camelCase")]`.
  - `default_config_path() -> PathBuf` — returns `$HOME/.bankr/config.json`.
  - `load(path: &Path) -> BankrConfig` — reads file, deserializes; returns `Default` on missing/malformed.
  - `save(path: &Path, config: &BankrConfig) -> Result<()>` — creates parent dirs, writes JSON, sets `0600` permissions on Unix.
  - `mask_api_key(key: &str) -> String` — e.g., `bk_WKW...46ZE` (first 6 + "..." + last 4).
  - `resolve_api_key(cli_flag: Option<&str>, env_var: Option<&str>, config: &BankrConfig) -> Option<String>` — priority: flag > env > config file.
- [x] **Step 3:** Register `mod config;` in `main.rs`.
- [x] **Verification:** `cargo check -p bankr-cli` compiles. Write a `#[cfg(test)] mod tests` in `config.rs` testing load/save round-trip and mask function.

### Task 1.2: Restructure CLI into command modules

> **Context:** The current `main.rs` is 371 lines with all command handlers inline. Splitting into a `commands/` module tree will keep the codebase maintainable as we add 4+ new subcommands. Reuse existing handler functions — move them without changing logic.
> **Verification:** `bankr-cli --help` output unchanged. `cargo test` passes.

- **Priority:** P0
- **Scope:** File restructure
- **Status:** 🟢 DONE

- [x] **Step 1:** Create `bin/bankr-cli/src/commands/mod.rs` re-exporting all submodules.
- [x] **Step 2:** Move `cmd_whoami` → `commands/whoami.rs`, `cmd_prompt` → `commands/prompt.rs`, `cmd_job`/`cmd_cancel` → `commands/job.rs`, `cmd_sign` → `commands/sign.rs`, `cmd_submit` → `commands/submit.rs`.
- [x] **Step 3:** Create empty stubs for `commands/auth.rs` (login/logout), `commands/config_cmd.rs` (config show), `commands/skills.rs`.
- [x] **Step 4:** Update `main.rs` to import from `commands::*` and keep only clap definitions + dispatch logic.
- [x] **Verification:** `cargo build -p bankr-cli` succeeds. Running `./target/debug/bankr-cli --help` shows same subcommands as before.

---

## Phase 2: Core Logic

### Task 2.1: Implement `login` subcommand

> **Context:** The npm CLI `bankr login` prompts for an API key, verifies it by calling `GET /agent/me`, and saves it to `config.json`. Reuse `BankrAgentClient::get_me()` for verification and `config::save()` for persistence.
> **Verification:** Running `bankr-cli login --api-key <key>` creates `$HOME/.bankr/config.json` with the key and prints a success message.

- **Priority:** P0
- **Scope:** Auth command — login
- **Status:** 🟢 DONE

- [x] **Step 1:** Add `Login` variant to `Commands` enum: `Login { #[arg(long)] api_key: Option<String> }`.
- [x] **Step 2:** Implement `cmd_login` in `commands/auth.rs`:
  - If `--api-key` provided, use it. Otherwise read from stdin (print `Enter your Bankr API key:` prompt).
  - Create `BankrAgentClient` with the key.
  - Call `get_me()`. On success → save to config, print `✔ Logged in successfully`. On failure → print `✖ Invalid API key` with error.
- [x] **Step 3:** Wire `Commands::Login` in `main.rs` dispatch (note: login does NOT require an existing API key, so must be handled before the `resolve_api_key` check).
- [x] **Verification:** `bankr-cli login --api-key bk_test_key` → creates config file or shows API error.

### Task 2.2: Implement `logout` subcommand

> **Context:** The npm CLI `bankr logout` clears stored credentials. We remove the `apiKey` field from the config file (or delete the file entirely).
> **Verification:** After logout, config file no longer contains `apiKey`.

- **Priority:** P0
- **Scope:** Auth command — logout
- **Status:** 🟢 DONE

- [x] **Step 1:** Add `Logout` variant to `Commands` enum.
- [x] **Step 2:** Implement `cmd_logout` in `commands/auth.rs`:
  - Load existing config.
  - Set `api_key = None`.
  - Save back (preserving `api_url` if set).
  - Print `✔ Logged out. Credentials removed from <path>`.
- [x] **Step 3:** Wire in `main.rs` dispatch (logout also does not require an existing API key).
- [x] **Verification:** Run login then logout → config file exists but has no `apiKey`.

### Task 2.3: Implement `config` subcommand

> **Context:** The npm CLI `bankr config` shows current configuration. Display the resolved API key (masked), base URL, and config file path.
> **Verification:** `bankr-cli config` prints config info without making any API calls.

- **Priority:** P1
- **Scope:** Config display command
- **Status:** 🟢 DONE

- [x] **Step 1:** Add `Config` variant to `Commands` enum.
- [x] **Step 2:** Implement `cmd_config` in `commands/config_cmd.rs`:
  - Load config from file.
  - Display: config file path, masked API key (or "Not set"), API URL (or default).
  - Format:

    ```text
    Config file:  /Users/user/.bankr/config.json
    API Key:      bk_WKW...46ZE
    API URL:      https://api.bankr.bot
    ```

- [x] **Step 3:** Wire in `main.rs` — does not require API key.
- [x] **Verification:** `bankr-cli config` displays config info. Works with no config file (shows "Not set").

### Task 2.4: Enhance `whoami` with pretty-print output

> **Context:** Currently `cmd_whoami` outputs raw JSON. The npm CLI shows a human-friendly format with checkmarks. Reuse `BankrAgentClient::get_me()` and the `UserInfoResponse` type. Add a `display.rs` module for formatting.
> **Verification:** `bankr-cli whoami` shows npm CLI-style output. `bankr-cli whoami --raw` still shows JSON.

- **Priority:** P0
- **Scope:** Whoami display formatting
- **Status:** 🟢 DONE

- [x] **Step 1:** Create `bin/bankr-cli/src/display.rs` with helper functions:
  - `print_whoami(config_path: &Path, api_key: &str, api_url: &str, resp: &UserInfoResponse)` — formats the npm CLI-style output.
  - `format_wallets(wallets: &[Wallet])` — uppercase chain name + address.
  - `format_social_accounts(accounts: &[SocialAccount])` — platform + username.
  - `format_bankr_club(club: &Option<BankrClub>)` — "Active (monthly)" / "Inactive".
- [x] **Step 2:** Update `cmd_whoami` in `commands/whoami.rs`:
  - Accept config path, API key, and base URL as additional parameters.
  - If `--raw`: call `print_json` as before.
  - Otherwise: print `✔ Bankr API connection OK`, `✔ Account info loaded`, then call `print_whoami`.
- [x] **Step 3:** Display output format:

  ```text
  ✔ Bankr API connection OK
  ✔ Account info loaded

  Bankr API Key:  bk_WKW...46ZE
  Bankr API URL:  https://api.bankr.bot
  Source:  /Users/user/.bankr/config.json
  Config:  /Users/user/.bankr/config.json

  Wallets:
    EVM      0x3582...8e83
    SOLANA   BdHTB...mctJ

  Social Accounts:
    email        user@example.com

  Bankr Club:  Inactive
  Referral Code:  NT3LZYXW-BNKR
  Score:  0
  ```

- [x] **Verification:** `bankr-cli whoami` with a valid key shows formatted output. `--raw` shows JSON.

### Task 2.5: Rename `job` to `status` with alias

> **Context:** The npm CLI uses `bankr status <jobId>` not `bankr job <jobId>`. Rename to `status` and keep `job` as a hidden alias for backward compatibility.
> **Verification:** Both `bankr-cli status <id>` and `bankr-cli job <id>` work identically.

- **Priority:** P1
- **Scope:** Command rename
- **Status:** 🟢 DONE

- [x] **Step 1:** Rename `Commands::Job` to `Commands::Status` in the clap enum. Update `#[command(name = "status")]`.
- [x] **Step 2:** Add a hidden alias: `#[command(alias = "job", hide = true)]` on `Status` variant or add a separate hidden `Job` variant that delegates to `Status`.
- [x] **Step 3:** Update dispatch in `main.rs`.
- [x] **Verification:** `bankr-cli status <id>` works. `bankr-cli job <id>` still works. `bankr-cli --help` shows `status` not `job`.

---

## Phase 3: Integration & Features

### Task 3.1: Wire config file into main CLI flow

> **Context:** All existing commands currently require `BANKR_API_KEY` env or `--api-key`. Integrate the config fallback chain so config file credentials work seamlessly. Commands that don't need an API key (`login`, `logout`, `config`) must be handled before the key resolution.
> **Verification:** `bankr-cli whoami` works with API key only in config file (no env var, no flag).

- **Priority:** P0
- **Scope:** Config integration into main dispatch
- **Status:** 🟢 DONE

- [x] **Step 1:** Add `--config <path>` global flag to `Cli` struct (default: `~/.bankr/config.json`).
- [x] **Step 2:** In `main()`:
  - Load config from the config path.
  - For `login`, `logout`, `config` → dispatch immediately (no API key needed).
  - For all other commands → resolve API key via `resolve_api_key(cli.api_key, env, config)`. Error if none found.
- [x] **Step 3:** Pass resolved base URL through config fallback: `cli.base_url` > `config.api_url` > default.
- [x] **Step 4:** Update error message when no API key found to list all three sources:

  ```text
  Error: No API key found. Provide one via:
    --api-key <key>
    BANKR_API_KEY environment variable
    bankr-cli login
  ```

- [x] **Verification:** Set API key only in config file → `bankr-cli whoami` succeeds. Remove from config → clear error message.

### Task 3.2: Implement `skills` subcommand

> **Context:** The npm CLI `bankr skills` shows all agent skills with examples. This can be implemented by sending a known prompt ("list all your skills and capabilities with examples") to the agent via `BankrAgentClient::prompt_and_wait()` and displaying the response.
> **Verification:** `bankr-cli skills` displays agent skills.

- **Priority:** P2
- **Scope:** Skills command
- **Status:** 🟢 DONE

- [x] **Step 1:** Add `Skills` variant to `Commands` enum.
- [x] **Step 2:** Implement `cmd_skills` in `commands/skills.rs`:
  - Submit prompt: `"List all available skills and capabilities with brief descriptions and examples"`.
  - Use `prompt_and_wait` with default poll settings.
  - Display the `response` field from the completed job.
  - If `--raw`: print full job JSON.
- [x] **Step 3:** Wire in `main.rs` dispatch (requires API key).
- [x] **Verification:** `bankr-cli skills` returns a skill listing from the agent.

### Task 3.3: Add display module for consistent formatting

> **Context:** Multiple commands need consistent colored/formatted output. Centralize formatting helpers in `display.rs`. Use ANSI escape codes directly to avoid adding a large dependency.
> **Verification:** All commands produce consistent visual output.

- **Priority:** P1
- **Scope:** Display helpers
- **Status:** 🟢 DONE

- [x] **Step 1:** In `display.rs`, add helpers:
  - `success(msg: &str)` — prints `✔ {msg}` in green.
  - `error(msg: &str)` — prints `✖ {msg}` in red.
  - `label_value(label: &str, value: &str)` — prints `{label}  {value}` with aligned columns.
  - `section_header(title: &str)` — prints blank line + `{title}:`.
- [x] **Step 2:** Refactor `whoami`, `login`, `logout` handlers to use these helpers.
- [x] **Step 3:** Add `print_job_status(job: &JobResponse)` — pretty-print job info for `status` command (when not `--raw`).
- [x] **Verification:** Visual inspection of output for `whoami`, `login`, `logout`, `status` commands.

---

## Phase 4: Polish, QA & Docs

### Task 4.1: Add unit tests for config module

> **Context:** Config loading/saving is critical path logic. Test edge cases: missing file, empty file, malformed JSON, round-trip, mask function.
> **Verification:** `cargo test -p bankr-cli` passes with >80% coverage of `config.rs`.

- **Priority:** P1
- **Scope:** Unit tests
- **Status:** 🟢 DONE

- [x] **Step 1:** In `config.rs` `#[cfg(test)] mod tests`:
  - `test_default_config()` — `BankrConfig::default()` has `None` fields.
  - `test_load_missing_file()` — returns default config.
  - `test_load_malformed_json()` — returns default config without panic.
  - `test_save_and_load_roundtrip()` — write config, read back, fields match.
  - `test_mask_api_key()` — verify masking for various key lengths.
  - `test_resolve_api_key_priority()` — flag > env > config.
- [x] **Verification:** `cargo test -p bankr-cli` — all tests pass.

### Task 4.2: Add unit tests for display module

> **Context:** Display formatting should produce expected output for known inputs.
> **Verification:** `cargo test -p bankr-cli` passes.

- **Priority:** P2
- **Scope:** Unit tests
- **Status:** 🟢 DONE

- [x] **Step 1:** In `display.rs` `#[cfg(test)] mod tests`:
  - `test_format_wallets()` — verify wallet display for EVM and Solana.
  - `test_format_bankr_club_active()` / `_inactive()` — verify club status text.
  - `test_format_social_accounts()` — verify platform + username display.
- [x] **Verification:** `cargo test -p bankr-cli` — all tests pass.

### Task 4.3: Update CLI help text and documentation

> **Context:** Ensure `--help` output is clear and matches the npm CLI's command descriptions. Update the crate-level doc comment in `main.rs`.
> **Verification:** `bankr-cli --help` matches expected output.

- **Priority:** P2
- **Scope:** Documentation
- **Status:** 🟢 DONE

- [x] **Step 1:** Update `#[command(about = ...)]` on `Cli` struct to: `"Bankr AI agent CLI"`.
- [x] **Step 2:** Ensure each subcommand's doc comment matches the npm CLI's help descriptions:
  - `login` → "Authenticate with the Bankr API"
  - `logout` → "Clear stored credentials"
  - `whoami` → "Show current authentication info"
  - `prompt` → "Send a prompt to the Bankr AI agent"
  - `status` → "Check the status of a job"
  - `cancel` → "Cancel a running job"
  - `config` → "Manage CLI configuration"
  - `skills` → "Show all Bankr AI agent skills with examples"
  - `sign` → "Sign messages, typed data, or transactions"
  - `submit` → "Submit a transaction to the blockchain"
- [x] **Step 3:** Update the crate-level doc comment in `main.rs` to reflect all supported commands.
- [x] **Verification:** `bankr-cli --help` shows all commands with correct descriptions.

### Task 4.4: Final lint, format, and test pass

> **Context:** Ensure the complete implementation passes all quality checks.
> **Verification:** `just format && just lint && just test` passes with zero errors and zero warnings.

- **Priority:** P0
- **Scope:** Quality assurance
- **Status:** 🟢 DONE

- [x] **Step 1:** Run `just format` — fix any formatting issues.
- [x] **Step 2:** Run `just lint` — fix all clippy warnings and lint errors.
- [x] **Step 3:** Run `just test` — ensure all tests pass.
- [x] **Step 4:** Run `cargo build --workspace` — clean build.
- [x] **Verification:** All three commands pass with zero errors.

---

## Summary & Timeline

| Phase | Tasks | Target Date |
| :--- | :---: | :--- |
| **1. Foundation** | 2 | 02-21 |
| **2. Core Logic** | 5 | 02-22 |
| **3. Integration** | 3 | 02-22 |
| **4. Polish** | 4 | 02-23 |
| **Total** | **14** | |

## Definition of Done

1. [x] **Linted:** `just lint` passes with zero warnings.
2. [x] **Tested:** Unit tests for `config.rs` and `display.rs` covering core logic.
3. [x] **Formatted:** `just format` produces no changes.
4. [x] **Verified:** Each task's specific Verification criterion met.
5. [x] **Parity:** `bankr-cli --help` shows commands matching the npm CLI (excluding `launch` and `llm`).
6. [x] **Config:** `bankr-cli whoami` works with API key from `$HOME/.bankr/config.json`.
