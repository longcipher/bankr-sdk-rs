On this page

Retrieve profile information for the authenticated API key holder, including wallet addresses, social accounts, and Bankr Club status.

## Endpoint​


    GET /agent/me  


## Request​

### Headers​

Header| Value| Required  
---|---|---  
`X-API-Key`| Your API key| Yes  

No request body is needed.

## Response​

### Success (200 OK)​


    {  
      "success": true,  
      "wallets": [  
        { "chain": "evm", "address": "0x1234567890abcdef1234567890abcdef12345678" },  
        { "chain": "solana", "address": "5FHwkrdxkAoGQ..." }  
      ],  
      "socialAccounts": [  
        { "platform": "farcaster", "username": "alice" },  
        { "platform": "twitter", "username": "alice_web3" }  
      ],  
      "refCode": "A1B2C3D4-BNKR",  
      "bankrClub": {  
        "active": true,  
        "subscriptionType": "monthly",  
        "renewOrCancelOn": 1720000000000  
      },  
      "leaderboard": {  
        "score": 1250,  
        "rank": 42  
      }  
    }  


### Response Fields​

Field| Type| Description  
---|---|---  
`success`| boolean| Whether the request was successful  
`wallets`| array| User's wallet addresses (EVM is always present, Solana if set)  
`wallets[].chain`| string| `"evm"` or `"solana"`  
`wallets[].address`| string| Wallet address  
`socialAccounts`| array| Connected social accounts (non-archived)  
`socialAccounts[].platform`| string| Platform name (`farcaster`, `twitter`, `telegram`, etc.)  
`socialAccounts[].username`| string?| Username on that platform  
`refCode`| string?| Referral code  
`bankrClub.active`| boolean| Whether Bankr Club subscription is active  
`bankrClub.subscriptionType`| string?| `"monthly"` or `"yearly"`  
`bankrClub.renewOrCancelOn`| number?| Unix timestamp of next renewal or cancellation  
`leaderboard.score`| number| User score  
`leaderboard.rank`| number?| Leaderboard rank  

### Error Responses​

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


## Examples​

### curl​


    curl https://api.bankr.bot/agent/me \  
      -H "X-API-Key: your_api_key_here"  


### TypeScript​


    const response = await fetch("https://api.bankr.bot/agent/me", {  
      headers: { "X-API-Key": process.env.BANKR_API_KEY! },  
    });  

    const data = await response.json();  
    console.log("Wallets:", data.wallets);  
    console.log("Score:", data.leaderboard.score);  


### Bankr CLI​


    bankr whoami  

The CLI `whoami` command displays your full profile info including wallet addresses, social accounts, Bankr Club status, and score.
