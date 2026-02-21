//! Pretty-print helpers for CLI output.
//!
//! These functions format API responses for human-readable terminal display,
//! matching the output style of the npm `bankr-cli`.

use std::path::Path;

use bankr_agent_api::types::{JobResponse, JobStatus, UserInfoResponse};

use crate::config::mask_api_key;

/// Print a success message in green: `✔ {msg}`.
pub(crate) fn success(msg: &str) {
    println!("\x1b[32m\u{2714} {msg}\x1b[0m");
}

/// Print an error message in red: `✖ {msg}`.
pub(crate) fn error(msg: &str) {
    println!("\x1b[31m\u{2716} {msg}\x1b[0m");
}

/// Pretty-print a `JobResponse` for the `status` command.
pub(crate) fn print_job_status(job: &JobResponse) {
    println!("Job ID:      {}", job.job_id);
    println!("Status:      {}", job.status);
    println!("Prompt:      {}", job.prompt);
    println!("Created:     {}", job.created_at);

    if let Some(started) = &job.started_at {
        println!("Started:     {started}");
    }
    if let Some(completed) = &job.completed_at {
        println!("Completed:   {completed}");
    }
    if let Some(ms) = job.processing_time {
        println!("Duration:    {ms}ms");
    }
    if let Some(thread) = &job.thread_id {
        println!("Thread:      {thread}");
    }

    match job.status {
        JobStatus::Completed => {
            if let Some(resp) = &job.response {
                println!();
                println!("Response:");
                println!("{resp}");
            }
        }
        JobStatus::Failed => {
            if let Some(err) = &job.error {
                println!();
                error(&format!("Error: {err}"));
            }
        }
        _ => {}
    }
}

/// Pretty-print the `whoami` response with config metadata.
pub(crate) fn print_whoami(
    config_path: &Path,
    api_key: &str,
    api_url: &str,
    resp: &UserInfoResponse,
) {
    success("Bankr API connection OK");
    success("Account info loaded");
    println!();
    println!("Bankr API Key:  {}", mask_api_key(api_key));
    println!("Bankr API URL:  {api_url}");
    println!("Source:  {}", config_path.display());
    println!("Config:  {}", config_path.display());
    println!();

    print_wallets(resp);
    print_social_accounts(resp);
    print_bankr_club(resp);
    print_referral_code(resp);
    print_score(resp);
}

fn print_wallets(resp: &UserInfoResponse) {
    println!("{}", format_wallets(resp));
    println!();
}

fn print_social_accounts(resp: &UserInfoResponse) {
    println!("{}", format_social_accounts(resp));
    println!();
}

fn print_bankr_club(resp: &UserInfoResponse) {
    println!("Bankr Club:  {}", format_bankr_club_status(resp));
}

fn print_referral_code(resp: &UserInfoResponse) {
    let code = resp.ref_code.as_deref().unwrap_or("(none)");
    println!("Referral Code:  {code}");
}

fn print_score(resp: &UserInfoResponse) {
    let score = resp.leaderboard.as_ref().map_or(0, |lb| lb.score);
    println!("Score:  {score}");
}

// ---------------------------------------------------------------------------
// Testable formatting helpers
// ---------------------------------------------------------------------------

/// Format the wallets section as a string.
fn format_wallets(resp: &UserInfoResponse) -> String {
    if resp.wallets.is_empty() {
        "Wallets:  (none)".to_owned()
    } else {
        let mut lines = vec!["Wallets:".to_owned()];
        for w in &resp.wallets {
            lines.push(format!("  {:<8} {}", w.chain.to_uppercase(), w.address));
        }
        lines.join("\n")
    }
}

/// Format the social accounts section as a string.
fn format_social_accounts(resp: &UserInfoResponse) -> String {
    if resp.social_accounts.is_empty() {
        "Social Accounts:  (none)".to_owned()
    } else {
        let mut lines = vec!["Social Accounts:".to_owned()];
        for sa in &resp.social_accounts {
            let username = sa.username.as_deref().unwrap_or("(not set)");
            lines.push(format!("  {:<12} {username}", sa.platform));
        }
        lines.join("\n")
    }
}

/// Format the Bankr Club status label.
fn format_bankr_club_status(resp: &UserInfoResponse) -> String {
    resp.bankr_club.as_ref().map_or_else(
        || "Inactive".to_owned(),
        |club| {
            if club.active {
                club.subscription_type.as_deref().unwrap_or("Active").to_owned()
            } else {
                "Inactive".to_owned()
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use bankr_agent_api::types::{BankrClub, SocialAccount, Wallet};

    use super::*;

    /// Build a minimal `UserInfoResponse` for tests.
    fn empty_response() -> UserInfoResponse {
        UserInfoResponse {
            success: true,
            wallets: vec![],
            social_accounts: vec![],
            ref_code: None,
            bankr_club: None,
            leaderboard: None,
        }
    }

    #[test]
    fn test_format_wallets_empty() {
        let resp = empty_response();
        assert_eq!(format_wallets(&resp), "Wallets:  (none)");
    }

    #[test]
    fn test_format_wallets() {
        let mut resp = empty_response();
        resp.wallets = vec![
            Wallet { chain: "evm".to_owned(), address: "0xAbC123".to_owned() },
            Wallet { chain: "solana".to_owned(), address: "So1anaAddr".to_owned() },
        ];

        let output = format_wallets(&resp);
        assert!(output.starts_with("Wallets:"));
        assert!(output.contains("EVM"));
        assert!(output.contains("0xAbC123"));
        assert!(output.contains("SOLANA"));
        assert!(output.contains("So1anaAddr"));
    }

    #[test]
    fn test_format_bankr_club_active() {
        let mut resp = empty_response();
        resp.bankr_club = Some(BankrClub {
            active: true,
            subscription_type: Some("monthly".to_owned()),
            renew_or_cancel_on: None,
        });
        assert_eq!(format_bankr_club_status(&resp), "monthly");
    }

    #[test]
    fn test_format_bankr_club_active_no_type() {
        let mut resp = empty_response();
        resp.bankr_club =
            Some(BankrClub { active: true, subscription_type: None, renew_or_cancel_on: None });
        assert_eq!(format_bankr_club_status(&resp), "Active");
    }

    #[test]
    fn test_format_bankr_club_inactive() {
        let mut resp = empty_response();
        resp.bankr_club = Some(BankrClub {
            active: false,
            subscription_type: Some("yearly".to_owned()),
            renew_or_cancel_on: None,
        });
        assert_eq!(format_bankr_club_status(&resp), "Inactive");
    }

    #[test]
    fn test_format_bankr_club_none() {
        let resp = empty_response();
        assert_eq!(format_bankr_club_status(&resp), "Inactive");
    }

    #[test]
    fn test_format_social_accounts_empty() {
        let resp = empty_response();
        assert_eq!(format_social_accounts(&resp), "Social Accounts:  (none)");
    }

    #[test]
    fn test_format_social_accounts() {
        let mut resp = empty_response();
        resp.social_accounts = vec![
            SocialAccount {
                platform: "twitter".to_owned(),
                username: Some("@bankr_user".to_owned()),
            },
            SocialAccount { platform: "telegram".to_owned(), username: None },
        ];

        let output = format_social_accounts(&resp);
        assert!(output.starts_with("Social Accounts:"));
        assert!(output.contains("twitter"));
        assert!(output.contains("@bankr_user"));
        assert!(output.contains("telegram"));
        assert!(output.contains("(not set)"));
    }
}
