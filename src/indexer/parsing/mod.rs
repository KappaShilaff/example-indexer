use crate::indexer::abi::example_abi;
use crate::indexer::utils::run_function;
use crate::models::abi::example_abi::{ExampleEvent, ExampleFunctionInput, ExampleFunctionOutput};
use crate::models::caches::Caches;
use anyhow::Result;
use futures::channel::mpsc::{Receiver, Sender};
use futures::{SinkExt, StreamExt};
use nekoton_abi::transaction_parser::ExtractedOwned;
use nekoton_abi::{PackAbiPlain, UnpackAbiPlain};
use ton_abi::Token;
use ton_block::{MsgAddressInt, Transaction};
use transaction_buffer::models::RawTransaction;
use transaction_buffer::split_extracted_owned;

pub async fn parsing(
    caches: Caches,
    mut rx_parsed_events: Receiver<Vec<(Vec<ExtractedOwned>, RawTransaction)>>,
    mut tx_commit: Sender<()>,
) {
    while let Some(produced_transaction) = rx_parsed_events.next().await {
        for (extracted, raw_transaction) in produced_transaction {
            handle_parsed_events(extracted, &caches, raw_transaction.data).await
        }

        tx_commit.send(()).await.expect("dead tx_commit db");
    }
    panic!("rip kafka");
}

async fn handle_parsed_events(
    extracted: Vec<ExtractedOwned>,
    caches: &Caches,
    transaction: Transaction,
) {
    let (functions, events) = split_extracted_owned(extracted);

    for event in events {
        match event.name.as_str() {
            "ExampleEvent" => {
                if let Err(e) = handle_example_event(caches, &transaction, event).await {
                    log::error!("handle_example_event error {}", e);
                }
            }
            _ => {
                unreachable!("unknown event: {}", event.name)
            }
        }
    }

    for function in functions {
        match function.name.as_str() {
            "exampleFunction" => {}
            _ => {
                unreachable!("unknown function: {}", function.name)
            }
        }
    }
}

async fn handle_example_event(
    caches: &Caches,
    transaction: &Transaction,
    event: ExtractedOwned,
) -> Result<()> {
    let contract_address = MsgAddressInt::with_standart(None, 0, transaction.account_addr.clone())
        .expect("invalid contract address");

    let example_event: ExampleEvent = event.tokens.unpack()?;

    // make your logic here

    let function_input: Vec<Token> = ExampleFunctionInput {
        value1: 0,
        value2: Default::default(),
    }
    .pack();

    let function_output: ExampleFunctionOutput = run_function(
        &caches.transaction_consumer,
        &contract_address,
        transaction.now,
        example_abi()
            .function("exampleFunction")
            .expect("invalid function"),
        &function_input,
        None,
    )
    .await?;

    caches
        .sqlx_client
        .new_user(&example_event.value0.to_string())
        .await?;

    let _ = caches
        .sqlx_client
        .get_user(&function_output.value0.to_string())
        .await?;

    Ok(())
}
