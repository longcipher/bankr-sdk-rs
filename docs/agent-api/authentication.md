On this page

All Agent API endpoints require authentication via API key.

## Getting an API Key​

1. Visit [bankr.bot/api](https://bankr.bot/api)
2. Sign in to your Bankr account
3. Generate a new API key
4. **Enable Agent API access** on the key

Important

Your API key must have Agent API access explicitly enabled. A standard API key without agent access will receive a 403 error.

## Using Your API Key​

Include your API key in the `X-API-Key` header with every request:

    curl -X POST https://api.bankr.bot/agent/prompt \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{"prompt": "what is the price of ETH?"}'  


## Requirements​

* API key must be **active** (not revoked)
* API key must have **Agent API access enabled**



## Error Responses​

### Missing or Invalid API Key (401)​


    {  
      "error": "Authentication required",  
      "message": "Please provide a valid API key"  
    }  


### Agent Access Not Enabled (403)​


    {  
      "error": "Agent API access not enabled",  
      "message": "Enable agent access for your API key at bankr.bot/api"  
    }  


## Access Control​

API keys support read-only mode and IP allowlisting to restrict what operations the key can perform and where it can be used from.

**Read-only mode** — when enabled, the agent can only retrieve information (prices, balances, analytics). Write endpoints (`/agent/sign`, `/agent/submit`) return 403:

    {  
      "error": "Read-only API key",  
      "message": "This API key has read-only access and cannot sign messages or transactions. Update your API key permissions at https://bankr.bot/api"  
    }  

**IP allowlist** — restrict the key to specific IP addresses. Requests from unlisted IPs return 403:

    {  
      "error": "IP address not allowed",  
      "message": "IP address not allowed for this API key"  
    }  

See [Access Control](/agent-api/access-control) for full details on permissions, rate limits, and recommended configurations.

## Security Best Practices​

### Keep Your Key Secret​

* **Never commit** API keys to version control
* **Never share** your key publicly or in client-side code
* Use **environment variables** to store keys

    // Good - use environment variables  
    const API_KEY = process.env.BANKR_API_KEY;  

    // Bad - never hardcode keys  
    const API_KEY = "sk_live_abc123...";  


### Use a Dedicated Account​

Consider creating a separate Bankr account specifically for API access:

* Limits exposure if the key is compromised
* Allows you to control exactly what assets are at risk
* Makes it easier to revoke access without affecting your main account
* Enable **read-only mode** for monitoring-only agents
* Use the **IP allowlist** to lock the key to your server IPs



### Revoke Compromised Keys​

If you suspect your API key has been leaked:

1. Go to [bankr.bot/api](https://bankr.bot/api) immediately
2. Revoke the compromised key
3. Generate a new key
4. Update your applications



### Monitor Usage​

Regularly check your account for unexpected activity:

* Unusual transactions
* Unknown token swaps
* Unexpected balance changes



## Environment Setup​

### Node.js / TypeScript​


    // Load from environment  
    const API_KEY = process.env.BANKR_API_KEY;  

    if (!API_KEY) {  
      throw new Error('BANKR_API_KEY environment variable is required');  
    }  

    // Use in requests  
    const response = await fetch('https://api.bankr.bot/agent/prompt', {  
      method: 'POST',  
      headers: {  
        'Content-Type': 'application/json',  
        'X-API-Key': API_KEY,  
      },  
      body: JSON.stringify({ prompt: 'what is the price of ETH?' }),  
    });  


### .env File​


    # .env  
    BANKR_API_KEY=your_api_key_here  

Add `.env` to your `.gitignore`:

    # .gitignore  
    .env  
    .env.local  
    .env.*.local  
