use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use crate::indexer::abi::all_for_parse;
use crate::indexer::parsing::parsing;
use crate::models::caches::Caches;
use anyhow::Result;
use itertools::Itertools;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use transaction_buffer::drop_base::check_base;
use transaction_buffer::models::{BufferedConsumerChannels, BufferedConsumerConfig};
use transaction_buffer::start_parsing_and_get_channels;
use transaction_consumer::TransactionConsumer;
use url::Url;

use crate::settings::{get_config, Config};
use crate::sqlx_client::SqlxClient;

pub mod api;
pub mod indexer;
pub mod models;
pub mod services;
pub mod settings;
pub mod sqlx_client;

pub async fn start_server() -> Result<()> {
    init_utils();
    log::info!("start service");
    let config = get_config();

    let pool = get_pool(&config).await;
    check_base(&pool, config.drop_base_index).await;
    sqlx::migrate!().run(&pool).await?;

    let transaction_consumer = get_transaction_consumer(&config).await;

    let buffered_consumer_config = BufferedConsumerConfig::new(
        transaction_consumer.clone(),
        pool.clone(),
        all_for_parse(),
        100_000,
        60,
        2,
    );

    let caches = Caches::new(SqlxClient::new(pool), transaction_consumer, config);

    let BufferedConsumerChannels {
        rx_parsed_events,
        tx_commit,
        notify_for_services,
    } = start_parsing_and_get_channels(buffered_consumer_config);

    {
        let caches = caches.clone();
        tokio::spawn(parsing(caches, rx_parsed_events, tx_commit));
    }

    notify_for_services.notified().await;

    let services = services::Services::new(caches.clone());

    tokio::spawn(api::http_service(services, caches));

    Ok(())
}

fn init_utils() {
    stackdriver_logger::init_with_cargo!();

    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        log::error!("{:?}", info);
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            log::error!("panic payload: {s:?}");
        }
        prev(info);
        std::process::exit(1);
    }));
}

async fn get_pool(config: &Config) -> PgPool {
    PgPoolOptions::new()
        .max_connections(config.db_pool_size)
        .connect(&config.database_url)
        .await
        .expect("fail pg pool")
}

async fn get_transaction_consumer(config: &Config) -> Arc<TransactionConsumer> {
    let mut kafka_settings: HashMap<String, String> = Default::default();
    kafka_settings.insert("bootstrap.servers".into(), config.brokers.clone());
    kafka_settings.insert("client.id".into(), config.kafka_client_id.clone());

    let (group_id, topic, states_rpc_endpoint, options) = (
        config.kafka_group_id.clone(),
        config.kafka_topic.clone(),
        from_string_to_vec(config.states_rpc_endpoint.clone())
            .into_iter()
            .map(|x| Url::from_str(&x).unwrap())
            .collect_vec(),
        kafka_settings,
    );

    TransactionConsumer::new(
        &group_id,
        &topic,
        states_rpc_endpoint,
        None,
        transaction_consumer::ConsumerOptions {
            kafka_options: options
                .iter()
                .map(|(x, y)| (x.as_str(), y.as_str()))
                .collect::<HashMap<_, _>>(),
            skip_0_partition: true,
        },
    )
    .await
    .expect("fail get transaction producer")
}

pub fn from_string_to_vec(input: String) -> Vec<String> {
    input.split(',').map(|x| x.to_string()).collect_vec()
}
