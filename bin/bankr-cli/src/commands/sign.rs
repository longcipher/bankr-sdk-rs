use bankr_agent_api::{
    BankrAgentClient,
    types::{EvmTransaction, SignRequest, SignatureType},
};
use clap::Subcommand;
use eyre::{Result, WrapErr, eyre};

use crate::print_json;

#[derive(Debug, Subcommand)]
pub(crate) enum SignCommands {
    /// Sign a plain text message (personal_sign).
    Personal {
        /// The message to sign.
        message: String,
    },

    /// Sign EIP-712 typed data (pass JSON string).
    TypedData {
        /// JSON string of the typed data object.
        typed_data_json: String,
    },

    /// Sign an EVM transaction without broadcasting.
    Transaction {
        /// Destination address.
        #[arg(long)]
        to: String,

        /// Chain ID.
        #[arg(long)]
        chain_id: u64,

        /// Value in wei.
        #[arg(long)]
        value: Option<String>,

        /// Calldata (hex).
        #[arg(long)]
        data: Option<String>,
    },
}

pub(crate) async fn cmd_sign(
    client: &BankrAgentClient,
    kind: SignCommands,
    raw: bool,
) -> Result<()> {
    let req = match kind {
        SignCommands::Personal { message } => SignRequest {
            signature_type: SignatureType::PersonalSign,
            message: Some(message),
            typed_data: None,
            transaction: None,
        },
        SignCommands::TypedData { typed_data_json } => {
            let typed_data: serde_json::Value =
                serde_json::from_str(&typed_data_json).wrap_err("Invalid typed-data JSON")?;
            SignRequest {
                signature_type: SignatureType::EthSignTypedDataV4,
                message: None,
                typed_data: Some(typed_data),
                transaction: None,
            }
        }
        SignCommands::Transaction { to, chain_id, value, data } => SignRequest {
            signature_type: SignatureType::EthSignTransaction,
            message: None,
            typed_data: None,
            transaction: Some(EvmTransaction {
                to,
                chain_id,
                value,
                data,
                gas: None,
                gas_price: None,
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
                nonce: None,
            }),
        },
    };

    let resp = client.sign(&req).await.map_err(|e| eyre!("{e}"))?;
    print_json(&resp, raw)
}
