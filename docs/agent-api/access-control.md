On this page

API keys support granular access controls that let you restrict what an agent can do and where it can connect from.

All access controls are managed at [bankr.bot/api](https://bankr.bot/api).

## API Key Permissions​

Each API key has independent capability flags:

Flag| Default| Description  
---|---|---  
`agentApiEnabled`| Disabled| Access to `/agent/*` endpoints (prompt, sign, submit, jobs)  
`llmGatewayEnabled`| Disabled| Access to the [LLM Gateway](/llm-gateway/overview) at `llm.bankr.bot`  
`readOnly`| Disabled| Restricts the key to read-only operations (no transactions)  
`allowedIps`| Empty (all IPs)| IP allowlist — when empty, all IPs are accepted  

All keys share the `bk_...` format. Each flag is configured independently.

## Read-Only Mode​

When `readOnly` is enabled on an API key, the agent can only retrieve information — it cannot execute transactions, swaps, transfers, or any state-changing operations.

### Behavior by Endpoint​

Endpoint| Behavior  
---|---  
`POST /agent/prompt`| Works, but only read tools are available (prices, balances, analytics, research)  
`POST /agent/sign`| Blocked — returns 403  
`POST /agent/submit`| Blocked — returns 403  
`GET /agent/job/:jobId`| Works normally  
`POST /agent/cancel/:jobId`| Works normally  

### Error Responses​

**Sign endpoint (403):**

    {  
      "error": "Read-only API key",  
      "message": "This API key has read-only access and cannot sign messages or transactions. Update your API key permissions at https://bankr.bot/api"  
    }  

**Submit endpoint (403):**

    {  
      "error": "Read-only API key",  
      "message": "This API key has read-only access and cannot submit transactions. Update your API key permissions at https://bankr.bot/api"  
    }  


### How It Works​

When a read-only key calls `/agent/prompt`, the agent session receives a system directive that removes all write tools. The following tool categories are filtered out:

* Token swaps
* Token and ETH transfers
* NFT purchases and trades
* Staking and unstaking
* Limit, stop, DCA, and TWAP orders
* Token launches and deployments
* Leveraged trading positions
* Polymarket bets
* Fee claims

The agent is aware of the restriction and will explain it to users who request write operations.

## IP Allowlist​

The `allowedIps` array restricts which IP addresses can use the key. Validation runs in the auth middleware before any endpoint logic.

* **Empty array** (default) — all IPs are accepted
* **One or more IPs** — only requests from listed IPs are accepted

**Error response (403):**

    {  
      "error": "IP address not allowed",  
      "message": "IP address not allowed for this API key"  
    }  


## Rate Limits​

### Daily Message Limits​

The `/agent/prompt` endpoint enforces a per-account daily message limit:

Tier| Daily Limit  
---|---  
Standard| 100 messages  
Bankr Club| 1,000 messages  
Custom (per key)| Set at [bankr.bot/api](https://bankr.bot/api)  

Custom limits override both the standard and Bankr Club defaults.

The limit uses a **rolling 24-hour window** from the time of first usage — it does not reset at midnight.

**Error response (429):**

    {  
      "error": "Daily limit exceeded",  
      "message": "You have reached your daily API limit of 100 messages. Upgrade to Bankr Club for 1000 messages/day. Resets at 2025-01-15T12:00:00.000Z",  
      "resetAt": 1736942400000,  
      "limit": 100,  
      "used": 100  
    }  

The `resetAt` field is a Unix timestamp (milliseconds) indicating when the counter resets. The `limit` and `used` fields show the current quota and consumption.

note

The "Upgrade to Bankr Club" portion of the message only appears for standard accounts. Bankr Club members and accounts with a custom daily limit see a shorter message without the upgrade prompt.

### General API Rate Limits​

These apply to all API consumers by IP or API key:

Scope| Window| Limit  
---|---|---  
Public endpoints (`/public/*`)| 15 minutes| 100 requests per IP  
General endpoints| 1 minute| 120 requests per IP  
External orders (`/trading/order`)| 1 second| 10 requests per API key  

## Dedicated Agent Wallet​

For production agent deployments, use a **separate Bankr account** with its own API key and wallet. This provides:

* **Blast radius isolation** — a compromised key only affects the agent wallet, not your main holdings
* **Independent controls** — configure read-only mode, IP allowlist, and rate limits specifically for the agent
* **Easy revocation** — revoke and regenerate the agent key without disrupting your main account



### Setup​

1. Create a separate Bankr account at [bankr.bot](https://bankr.bot)
2. Generate an API key at [bankr.bot/api](https://bankr.bot/api)
3. Configure access controls (read-only mode, IP allowlist)
4. Fund the wallet with only the amounts the agent needs



### Recommended Configurations​

Use Case| Read-Only| IP Allowlist| Notes  
---|---|---|---  
Monitoring bot| Yes| Yes| Price alerts, portfolio tracking — no transaction risk  
Trading bot| No| Yes| Needs write access for swaps; lock down to server IPs  
Dev / testing| No| No| Flexible access; use small balances  

## API Key vs LLM Gateway Key​

A single API key can serve both the Agent API and the LLM Gateway when both flags are enabled. You can also use separate keys:

Config| Agent API| LLM Gateway  
---|---|---  
Single key| `BANKR_API_KEY`| Same key  
Separate keys| `BANKR_API_KEY`| `BANKR_LLM_KEY`  

In the CLI:

* `bankr login --api-key KEY` sets the Agent API key
* `bankr login --llm-key KEY` sets the LLM Gateway key
* `bankr config set llmKey KEY` updates the LLM key independently

**When to use separate keys:**

* Different permission requirements (e.g., agent key is read-only, LLM key only needs gateway access)
* Independent revocation — rotate one without affecting the other
* Different rate limit tracking
