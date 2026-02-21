On this page

Submit transactions directly to the blockchain with optional confirmation waiting.

## Endpoint​


    POST /agent/submit  


## Overview​

The submit endpoint allows you to submit raw EVM transactions directly to the blockchain. Unlike the [Prompt Endpoint](/agent-api/prompt-endpoint) which uses natural language, this endpoint accepts explicit transaction parameters.

Key features:

* **Direct transaction submission** — Bypass the AI agent for pre-built transactions
* **Synchronous response** — Returns transaction hash immediately
* **Optional confirmation** — Wait for on-chain confirmation or return immediately
* **Full control** — Specify exact gas, nonce, and calldata



## Request​

### Headers​

Header| Value| Required  
---|---|---  
`Content-Type`| `application/json`| Yes  
`X-API-Key`| Your API key| Yes  

### Body​


    {  
      "transaction": {  
        "to": "0x...",  
        "chainId": 8453,  
        "value": "1000000000000000000",  
        "data": "0x..."  
      },  
      "description": "Transfer 1 ETH",  
      "waitForConfirmation": true  
    }  


### Transaction Object​

Field| Type| Description| Required  
---|---|---|---  
`to`| string| Destination address| Yes  
`chainId`| number| Chain ID (8453 for Base, 1 for Ethereum, etc.)| Yes  
`value`| string| Value in wei (as string)| No  
`data`| string| Calldata (hex string starting with `0x`)| No  
`gas`| string| Gas limit| No  
`gasPrice`| string| Legacy gas price in wei| No  
`maxFeePerGas`| string| EIP-1559 max fee per gas| No  
`maxPriorityFeePerGas`| string| EIP-1559 priority fee| No  
`nonce`| number| Transaction nonce (auto-filled if omitted)| No  

### Additional Fields​

Field| Type| Description| Default  
---|---|---|---  
`description`| string| Human-readable description for logging| -  
`waitForConfirmation`| boolean| Wait for transaction confirmation| `true`  

## Response​

### Success with Confirmation (200 OK)​

When `waitForConfirmation: true` (default):

    {  
      "success": true,  
      "transactionHash": "0x...",  
      "status": "success",  
      "blockNumber": "12345678",  
      "gasUsed": "21000",  
      "signer": "0x...",  
      "chainId": 8453  
    }  


### Success without Confirmation (200 OK)​

When `waitForConfirmation: false`:

    {  
      "success": true,  
      "transactionHash": "0x...",  
      "status": "pending",  
      "signer": "0x...",  
      "chainId": 8453  
    }  


### Response Fields​

Field| Type| Description  
---|---|---  
`success`| boolean| `true` if submission succeeded  
`transactionHash`| string| The transaction hash  
`status`| string| `"success"`, `"reverted"`, or `"pending"`  
`blockNumber`| string| Block number (if confirmed)  
`gasUsed`| string| Gas used (if confirmed)  
`signer`| string| Address that signed the transaction  
`chainId`| number| Chain ID the transaction was submitted to  

### Error Responses​

**Invalid Request (400)**

    {  
      "error": "Invalid request",  
      "message": "transaction object is required"  
    }  

**Submission Failed (400)**

    {  
      "success": false,  
      "error": "Insufficient funds for gas",  
      "transactionHash": "0x...",  
      "status": "failed",  
      "signer": "0x...",  
      "chainId": 8453  
    }  

**Authentication Required (401)**

    {  
      "error": "Authentication required",  
      "message": "Valid API key with associated wallet required"  
    }  

**Agent Access Not Enabled (403)**

    {  
      "error": "Agent API access not enabled",  
      "message": "Enable agent access for your API key at bankr.bot/api"  
    }  

**Read-Only API Key (403)**

    {  
      "error": "Read-only API key",  
      "message": "This API key has read-only access and cannot submit transactions. Update your API key permissions at https://bankr.bot/api"  
    }  

note

This is a write operation. If your key has read-only mode enabled, disable it at [bankr.bot/api](https://bankr.bot/api) or use a different key. See [Access Control](/agent-api/access-control) for details.

## Examples​

### Simple ETH Transfer​


    curl -X POST https://api.bankr.bot/agent/submit \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{  
        "transaction": {  
          "to": "0xRecipientAddress",  
          "chainId": 8453,  
          "value": "1000000000000000000"  
        },  
        "description": "Send 1 ETH to friend"  
      }'  


### ERC20 Transfer​


    curl -X POST https://api.bankr.bot/agent/submit \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{  
        "transaction": {  
          "to": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",  
          "chainId": 8453,  
          "value": "0",  
          "data": "0xa9059cbb000000000000000000000000recipient000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f4240"  
        },  
        "description": "Transfer 1 USDC"  
      }'  


### Contract Interaction​


    curl -X POST https://api.bankr.bot/agent/submit \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{  
        "transaction": {  
          "to": "0xContractAddress",  
          "chainId": 8453,  
          "value": "0",  
          "data": "0x..."  
        },  
        "description": "Call myFunction on contract",  
        "waitForConfirmation": true  
      }'  


### Fire-and-Forget​

Submit without waiting for confirmation:

    curl -X POST https://api.bankr.bot/agent/submit \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{  
        "transaction": {  
          "to": "0xRecipientAddress",  
          "chainId": 8453,  
          "value": "100000000000000000"  
        },  
        "waitForConfirmation": false  
      }'  


### With Custom Gas​


    curl -X POST https://api.bankr.bot/agent/submit \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{  
        "transaction": {  
          "to": "0xRecipientAddress",  
          "chainId": 8453,  
          "value": "1000000000000000000",  
          "maxFeePerGas": "50000000000",  
          "maxPriorityFeePerGas": "2000000000",  
          "gas": "21000"  
        }  
      }'  


## Supported Chains​

Chain| Chain ID| Native Token  
---|---|---  
Ethereum| 1| ETH  
Base| 8453| ETH  
Polygon| 137| MATIC  
Unichain| 130| ETH  

## Use Cases​

### Pre-Built Transactions​

Submit transactions built by external tools or SDKs:

    // Build transaction with viem/ethers  
    const tx = {  
      to: contractAddress,  
      data: encodeFunctionData({ ... }),  
      chainId: 8453  
    };  

    // Submit via Bankr  
    await fetch('https://api.bankr.bot/agent/submit', {  
      method: 'POST',  
      headers: {  
        'Content-Type': 'application/json',  
        'X-API-Key': apiKey  
      },  
      body: JSON.stringify({ transaction: tx })  
    });  


### Multi-Step Workflows​

Execute multiple transactions in sequence:

    // Step 1: Approve  
    const approveResult = await submitTx({  
      to: tokenAddress,  
      data: encodeApprove(spender, amount),  
      chainId: 8453  
    });  

    // Step 2: Swap (after approval confirms)  
    if (approveResult.status === 'success') {  
      await submitTx({  
        to: routerAddress,  
        data: encodeSwap(...),  
        chainId: 8453  
      });  
    }  


### Automation Backends​

Submit transactions from automated systems:

    // Cron job or event-triggered  
    if (priceConditionMet) {  
      await submitTx({  
        transaction: preBuiltSwapTx,  
        description: 'Automated swap triggered by price condition'  
      });  
    }  


## Sign vs Submit​

Feature| Sign Endpoint| Submit Endpoint  
---|---|---  
Broadcasts to chain| No| Yes  
Returns signature| Yes| No (returns tx hash)  
Waits for confirmation| N/A| Optional  
Use case| Permits, auth, offline signing| Direct execution  

## Next Steps​

* [Sign Endpoint](/agent-api/sign-endpoint) — Sign without broadcasting
* [Transaction Types](/agent-api/transaction-types) — Supported transaction formats
* [Prompt Endpoint](/agent-api/prompt-endpoint) — Natural language transactions
