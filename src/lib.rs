use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use url::Url;

use crate::settings::{Config, get_config};

mod settings;

pub async fn start_server() -> Result<()> {
    init_utils();
    log::info!("start service");
    let config = get_config();
    let pool = get_pool(&config).await;
    let (group_id, topic, states_rpc_endpoint, options) = get_kafka_settings(&config);


    todo!()
}

fn init_utils() {
    std::panic::set_hook(Box::new(handle_panic));
    stackdriver_logger::init_with_cargo!();
}

async fn get_pool(config: &Config) -> PgPool {
    PgPoolOptions::new()
        .max_connections(config.db_pool_size)
        .connect(&config.database_url)
        .await
        .expect("fail pg pool")
}

fn handle_panic(panic_info: &std::panic::PanicInfo<'_>) {
    log::error!("{:?}", panic_info);
    std::process::exit(1);
}

fn get_kafka_settings(config: &Config) -> (String, String, Vec<Url>, HashMap<String, String>) {
    let mut kafka_settings: HashMap<String, String> = Default::default();
    kafka_settings.insert("bootstrap.servers".into(), config.brokers.clone());
    kafka_settings.insert("client.id".into(), config.kafka_client_id.clone());

    (
        config.kafka_group_id.clone(),
        config.kafka_topic.clone(),
        from_string_to_vec(config.states_rpc_endpoint.clone())
            .into_iter()
            .map(|x| Url::from_str(&x).unwrap())
            .collect_vec(),
        kafka_settings,
    )
}

pub fn from_string_to_vec(input: String) -> Vec<String> {
    input.split(',').map(|x| x.to_string()).collect_vec()
}
