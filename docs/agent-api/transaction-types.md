On this page

The Agent API returns various transaction types depending on the operation. These are returned in the `transactions` array of completed jobs.

## Common Structure​

All transactions follow this pattern:

    {  
      "type": "transaction_type",  
      "metadata": {  
        "chainId": 8453,  
        "to": "0x...",  
        "data": "0x...",  
        "value": "0",  
        "gas": "150000"  
      }  
    }  


## Swap Transactions​

### `swap`​

Token-to-token or token-to-native swaps.

    {  
      "type": "swap",  
      "metadata": {  
        "__ORIGINAL_TX_DATA__": {  
          "chain": "base",  
          "humanReadableMessage": "Swap 100 USDC for ETH",  
          "inputTokenAddress": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",  
          "inputTokenAmount": "100000000",  
          "inputTokenTicker": "USDC",  
          "outputTokenAddress": "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE",  
          "outputTokenTicker": "ETH",  
          "receiver": "0x..."  
        },  
        "transaction": {  
          "chainId": 8453,  
          "to": "0x...",  
          "data": "0x...",  
          "gas": "200000",  
          "gasPrice": "1000000",  
          "value": "0"  
        }  
      }  
    }  


### `swapCrossChain`​

Cross-chain swaps using bridge aggregators.

    {  
      "type": "swapCrossChain",  
      "metadata": {  
        "chainId": 137,  
        "description": "Bridge 50 USDC from Polygon to Base",  
        "to": "0x...",  
        "data": "0x...",  
        "value": "0"  
      }  
    }  


## Transfer Transactions​

### `transfer_erc20`​

ERC-20 token transfers.

    {  
      "type": "transfer_erc20",  
      "metadata": {  
        "__ORIGINAL_TX_DATA__": {  
          "chain": "base",  
          "humanReadableMessage": "Send 50 USDC to 0x...",  
          "inputTokenAddress": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",  
          "inputTokenAmount": "50000000",  
          "inputTokenTicker": "USDC",  
          "receiver": "0x..."  
        },  
        "transaction": {  
          "chainId": 8453,  
          "to": "0x...",  
          "data": "0x...",  
          "gas": "65000",  
          "gasPrice": "1000000",  
          "value": "0"  
        }  
      }  
    }  


### `transfer_eth`​

Native ETH transfers.

    {  
      "type": "transfer_eth",  
      "metadata": {  
        "__ORIGINAL_TX_DATA__": {  
          "chain": "base",  
          "humanReadableMessage": "Send 0.1 ETH to 0x...",  
          "inputTokenAmount": "100000000000000000",  
          "inputTokenTicker": "ETH",  
          "receiver": "0x..."  
        },  
        "transaction": {  
          "chainId": 8453,  
          "to": "0x...",  
          "data": "0x",  
          "gas": "21000",  
          "gasPrice": "1000000",  
          "value": "100000000000000000"  
        }  
      }  
    }  


## Approval Transactions​

### `approval`​

Token approval for DEX or contract interaction.

    {  
      "type": "approval",  
      "metadata": {  
        "__ORIGINAL_TX_DATA__": {  
          "chain": "base",  
          "humanReadableMessage": "Approve USDC for swap",  
          "inputTokenAddress": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",  
          "inputTokenTicker": "USDC"  
        },  
        "transaction": {  
          "chainId": 8453,  
          "to": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",  
          "data": "0x...",  
          "gas": "50000",  
          "gasPrice": "1000000",  
          "value": "0"  
        }  
      }  
    }  


## Wrapper Transactions​

### `convert_eth_to_weth`​

Wrap ETH to WETH.

### `convert_weth_to_eth`​

Unwrap WETH to ETH.

## NFT Transactions​

### `buy_nft`​

Purchase an NFT from marketplace.

### `transfer_nft`​

Transfer NFT to another address.

### `mint_manifold_nft` / `mint_seadrop_nft`​

Mint NFTs from specific platforms.

## Trading Transactions​

### `avantisTrade`​

Leveraged trading position on Avantis.

    {  
      "type": "avantisTrade",  
      "metadata": {  
        "chainId": 8453,  
        "description": "Open $50 long on BTC/USD with 10x leverage",  
        "to": "0x...",  
        "data": "0x...",  
        "value": "0"  
      }  
    }  


## Staking Transactions​

### `manage_bankr_staking`​

Stake or unstake BANKR tokens.

    {  
      "type": "manage_bankr_staking",  
      "metadata": {  
        "chainId": 8453,  
        "description": "Stake 1000 BANKR",  
        "to": "0x...",  
        "data": "0x...",  
        "value": "0"  
      }  
    }  


## Executing Transactions​

When you receive transactions in the response, you can:

1. **Execute with your own wallet** — Sign and broadcast using viem/ethers
2. **Use Bankr's hosted wallet** — Transactions execute automatically if using terminal

For self-custody execution:

    import { createWalletClient, http } from 'viem';  
    import { base } from 'viem/chains';  

    const client = createWalletClient({  
      chain: base,  
      transport: http(),  
    });  

    const hash = await client.sendTransaction({  
      to: transaction.metadata.transaction.to,  
      data: transaction.metadata.transaction.data,  
      value: BigInt(transaction.metadata.transaction.value),  
      gas: BigInt(transaction.metadata.transaction.gas),  
    });  
