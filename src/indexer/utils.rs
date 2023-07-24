use anyhow::{anyhow, Result};
use nekoton_abi::FunctionExt;
use nekoton_utils::SimpleClock;
use ton_abi::{Function, Token};
use ton_block::{AccountStuff, MsgAddressInt};
use transaction_consumer::{Client, TransactionConsumer};

pub async fn run_function<T>(
    transaction_consumer: &TransactionConsumer,
    contract_address: &MsgAddressInt,
    timestamp_block: u32,
    function: &Function,
    input: &[Token],
    state: Option<AccountStuff>, // if u have state, u can use it
) -> Result<T, anyhow::Error>
where
    std::vec::Vec<ton_abi::Token>: nekoton_abi::UnpackAbiPlain<T>,
{
    let function_output = match state {
        Some(state) => function.run_local(&SimpleClock, state, input)?,
        None => transaction_consumer
            .get_client()
            .clone()
            .ok_or_else(|| anyhow!("transaction_consumer.get_client() is None"))?
            .run_local_with_time_check(contract_address, function, input, timestamp_block)
            .await?
            .ok_or_else(|| anyhow!("run_local_with_time_check is None"))?,
    };

    Ok(nekoton_abi::UnpackAbiPlain::<T>::unpack(
        function_output
            .tokens
            .ok_or_else(|| anyhow!("empty tokens"))?,
    )?)
}
