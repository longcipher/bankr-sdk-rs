On this page

Sign messages and transactions without broadcasting them to the network.

## Endpoint​


    POST /agent/sign  


## Overview​

The sign endpoint allows you to cryptographically sign:

* **Plain text messages** — Standard Ethereum message signing (`personal_sign`)
* **Typed data** — EIP-712 structured data signing (`eth_signTypedData_v4`)
* **Transactions** — Sign a transaction without submitting it (`eth_signTransaction`)

This is a **synchronous** endpoint that returns immediately with the signature.

## Request​

### Headers​

Header| Value| Required  
---|---|---  
`Content-Type`| `application/json`| Yes  
`X-API-Key`| Your API key| Yes  

### Body​

The request body varies based on the signature type.

#### personal_sign​

Sign a plain text message:

    {  
      "signatureType": "personal_sign",  
      "message": "Hello, Bankr!"  
    }  

Field| Type| Description| Required  
---|---|---|---  
`signatureType`| string| Must be `"personal_sign"`| Yes  
`message`| string| The message to sign| Yes  

#### eth_signTypedData_v4​

Sign EIP-712 structured data:

    {  
      "signatureType": "eth_signTypedData_v4",  
      "typedData": {  
        "domain": {  
          "name": "MyApp",  
          "version": "1",  
          "chainId": 8453,  
          "verifyingContract": "0x..."  
        },  
        "types": {  
          "Permit": [  
            { "name": "owner", "type": "address" },  
            { "name": "spender", "type": "address" },  
            { "name": "value", "type": "uint256" },  
            { "name": "nonce", "type": "uint256" },  
            { "name": "deadline", "type": "uint256" }  
          ]  
        },  
        "primaryType": "Permit",  
        "message": {  
          "owner": "0x...",  
          "spender": "0x...",  
          "value": "1000000",  
          "nonce": "0",  
          "deadline": "1707158800"  
        }  
      }  
    }  

Field| Type| Description| Required  
---|---|---|---  
`signatureType`| string| Must be `"eth_signTypedData_v4"`| Yes  
`typedData`| object| EIP-712 typed data structure| Yes  
`typedData.domain`| object| Domain separator (name, version, chainId, verifyingContract)| Yes  
`typedData.types`| object| Type definitions for the message| Yes  
`typedData.primaryType`| string| The primary type being signed| Yes  
`typedData.message`| object| The actual data to sign| Yes  

#### eth_signTransaction​

Sign a transaction without broadcasting:

    {  
      "signatureType": "eth_signTransaction",  
      "transaction": {  
        "to": "0x...",  
        "chainId": 8453,  
        "value": "1000000000000000000",  
        "data": "0x..."  
      }  
    }  

Field| Type| Description| Required  
---|---|---|---  
`signatureType`| string| Must be `"eth_signTransaction"`| Yes  
`transaction.to`| string| Destination address| Yes  
`transaction.chainId`| number| Chain ID| Yes  
`transaction.value`| string| Value in wei| No  
`transaction.data`| string| Calldata (hex)| No  
`transaction.gas`| string| Gas limit| No  
`transaction.gasPrice`| string| Legacy gas price| No  
`transaction.maxFeePerGas`| string| EIP-1559 max fee| No  
`transaction.maxPriorityFeePerGas`| string| EIP-1559 priority fee| No  
`transaction.nonce`| number| Transaction nonce| No  

## Response​

### Success (200 OK)​


    {  
      "success": true,  
      "signature": "0x...",  
      "signer": "0x...",  
      "signatureType": "personal_sign"  
    }  

Field| Type| Description  
---|---|---  
`success`| boolean| `true` if signing succeeded  
`signature`| string| The hex-encoded signature  
`signer`| string| Address that produced the signature  
`signatureType`| string| The signature type used  

### Error Responses​

**Invalid Request (400)**

    {  
      "error": "Invalid request",  
      "message": "signatureType is required"  
    }  

**Missing Field (400)**

    {  
      "error": "Invalid request",  
      "message": "message is required for personal_sign"  
    }  

**Signing Failed (400)**

    {  
      "success": false,  
      "error": "Signing failed",  
      "signer": "0x...",  
      "signatureType": "personal_sign"  
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
      "message": "This API key has read-only access and cannot sign messages or transactions. Update your API key permissions at https://bankr.bot/api"  
    }  

note

This is a write operation. If your key has read-only mode enabled, disable it at [bankr.bot/api](https://bankr.bot/api) or use a different key. See [Access Control](/agent-api/access-control) for details.

## Examples​

### Sign a Message​


    curl -X POST https://api.bankr.bot/agent/sign \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{  
        "signatureType": "personal_sign",  
        "message": "Sign in to MyApp at 2025-01-26T10:00:00Z"  
      }'  


### Sign a Permit (EIP-2612)​


    curl -X POST https://api.bankr.bot/agent/sign \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{  
        "signatureType": "eth_signTypedData_v4",  
        "typedData": {  
          "domain": {  
            "name": "USD Coin",  
            "version": "2",  
            "chainId": 8453,  
            "verifyingContract": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"  
          },  
          "types": {  
            "Permit": [  
              { "name": "owner", "type": "address" },  
              { "name": "spender", "type": "address" },  
              { "name": "value", "type": "uint256" },  
              { "name": "nonce", "type": "uint256" },  
              { "name": "deadline", "type": "uint256" }  
            ]  
          },  
          "primaryType": "Permit",  
          "message": {  
            "owner": "0xYourAddress",  
            "spender": "0xSpenderAddress",  
            "value": "1000000",  
            "nonce": "0",  
            "deadline": "1735689600"  
          }  
        }  
      }'  


### Sign a Transaction​


    curl -X POST https://api.bankr.bot/agent/sign \  
      -H "Content-Type: application/json" \  
      -H "X-API-Key: your_api_key_here" \  
      -d '{  
        "signatureType": "eth_signTransaction",  
        "transaction": {  
          "to": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",  
          "chainId": 8453,  
          "value": "0",  
          "data": "0xa9059cbb000000000000000000000000recipient0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f4240"  
        }  
      }'  


## Use Cases​

### Authentication​

Sign messages to prove wallet ownership for login flows:

    {  
      "signatureType": "personal_sign",  
      "message": "Sign in to MyApp\n\nNonce: abc123\nTimestamp: 2025-01-26T10:00:00Z"  
    }  


### Gasless Approvals (Permits)​

Sign EIP-2612 permits to approve token spending without gas:

    {  
      "signatureType": "eth_signTypedData_v4",  
      "typedData": { ... }  
    }  


### Offline Transaction Signing​

Sign transactions for later broadcast or multi-sig workflows:

    {  
      "signatureType": "eth_signTransaction",  
      "transaction": { ... }  
    }  


## Verifying Signatures​

### personal_sign​

Use `viem` or `ethers` to recover the signer:

    import { recoverMessageAddress } from 'viem';  

    const signer = await recoverMessageAddress({  
      message: "Hello, Bankr!",  
      signature: "0x..."  
    });  


### eth_signTypedData_v4​

Verify typed data signatures:

    import { verifyTypedData } from 'viem';  

    const isValid = await verifyTypedData({  
      address: "0x...",  
      domain: { ... },  
      types: { ... },  
      primaryType: "Permit",  
      message: { ... },  
      signature: "0x..."  
    });  


## Next Steps​

* [Submit Endpoint](/agent-api/submit-endpoint) — Submit signed transactions
* [Transaction Types](/agent-api/transaction-types) — Supported transaction formats
