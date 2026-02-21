use bankr_agent_api::{
    BankrAgentClient,
    types::{EvmTransaction, SubmitRequest},
};
use eyre::{Result, eyre};

use crate::print_json;

#[expect(clippy::too_many_arguments)]
pub(crate) async fn cmd_submit(
    client: &BankrAgentClient,
    to: &str,
    chain_id: u64,
    value: Option<String>,
    data: Option<String>,
    gas: Option<String>,
    max_fee_per_gas: Option<String>,
    max_priority_fee_per_gas: Option<String>,
    nonce: Option<u64>,
    description: Option<String>,
    no_wait: bool,
    raw: bool,
) -> Result<()> {
    let req = SubmitRequest {
        transaction: EvmTransaction {
            to: to.to_owned(),
            chain_id,
            value,
            data,
            gas,
            gas_price: None,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            nonce,
        },
        description,
        wait_for_confirmation: Some(!no_wait),
    };

    let resp = client.submit_transaction(&req).await.map_err(|e| eyre!("{e}"))?;
    print_json(&resp, raw)
}
