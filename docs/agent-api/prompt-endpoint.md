On this page

Submit natural language commands to the Bankr AI agent for processing.

## Endpoint​


    POST /agent/prompt  


## Request​

### Headers​

Header| Value| Required  
---|---|---  
`Content-Type`| `application/json`| Yes  
`X-API-Key`| Your API key| Yes  

### Body​


    {  
      "prompt": "swap $10 of ETH to USDC on base"  
    }  

Field| Type| Description| Required  
---|---|---|---  
`prompt`| string| Natural language command (max 10,000 characters)| Yes  
`threadId`| string| Continue an existing conversation thread| No  

When `threadId` is provided, the agent loads prior messages from that thread so it has conversation context. If omitted, a new thread is created automatically.

## Response​

### Success (202 Accepted)​


    {  
      "success": true,  
      "jobId": "abc123def456",  
      "threadId": "thr_XYZ789",  
      "status": "pending",  
      "message": "Job created successfully"  
    }  

Field| Type| Description  
---|---|---  
`success`| boolean| Whether the request was successful  
`jobId`| string| Unique identifier for tracking this job  
`threadId`| string| Conversation thread ID (reuse to continue the conversation)  
`status`| string| Current status (`"pending"`)  
`message`| string| Human-readable message  

### Error Responses​

**Invalid Request (400)**

    {  
      "error": "Invalid request",  
      "message": "Request body must include a prompt"  
    }  

**Prompt Too Long (400)**

    {  
      "error": "Prompt too long",  
      "message": "Prompt must be 10,000 characters or less"  
    }  

**Authentication Required (401)**

    {  
      "error": "Authentication required",  
      "message": "Please provide a valid API key"  
    }  

**Agent Access Not Enabled (403)**

    {  
      "error": "Agent API access not enabled",  
      "message": "Enable agent access for your API key at bankr.bot/api"  
    }  

**Rate Limit Exceeded (429)**

    {  
      "error": "Daily limit exceeded",  
      "message": "You have reached your daily API limit of 100 messages. Upgrade to Bankr Club for 1000 messages/day. Resets at 2025-01-15T12:00:00.000Z",  
      "resetAt": 1736942400000,  
      "limit": 100,  
      "used": 100  
    }  

The daily limit is 100 messages for standard accounts and 1,000 for Bankr Club members. Custom per-key limits can also be configured. The window is a rolling 24 hours, not a midnight reset.

**IP Not Allowed (403)**

    {  
      "error": "IP address not allowed",  
      "message": "IP address not allowed for this API key"  
    }  

See [Access Control](/agent-api/access-control) for full details on rate limits, IP allowlisting, and other key permissions.

## Examples​

### Price Query​


    curl -X POST https://api.bankr.bot/agent/prompt \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{"prompt": "what is the price of BTC?"}'  


### Token Swap​


    curl -X POST https://api.bankr.bot/agent/prompt \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{"prompt": "swap $50 of ETH to USDC on base"}'  


### Balance Check​


    curl -X POST https://api.bankr.bot/agent/prompt \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{"prompt": "what are my token balances?"}'  


### Token Launch​


    curl -X POST https://api.bankr.bot/agent/prompt \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{"prompt": "deploy a token called MyAgent with symbol AGENT on base"}'  


### Continue a Conversation​

Use the `threadId` from a previous response to continue the conversation with context:

    curl -X POST https://api.bankr.bot/agent/prompt \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{"prompt": "and what about SOL?", "threadId": "thr_XYZ789"}'  


## Prompt Best Practices​

### Be Specific​


    # Good  
    "swap $10 of ETH to USDC on base"  

    # Less clear  
    "swap some eth"  


### Specify Chains When Needed​


    # Clear chain context  
    "buy $5 of BONK on solana"  
    "swap 100 USDC to ETH on polygon"  


### Include Amounts​


    # Good  
    "buy $10 of BNKR"  

    # Ambiguous  
    "buy some BNKR"  


## Next Steps​

After submitting a prompt, poll for results using the [Job Management](/agent-api/job-management) endpoints.
