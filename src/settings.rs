use std::net::SocketAddr;
use std::sync::Arc;

use config::{ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub healthcheck_addr: SocketAddr,

    pub database_url: String,
    pub db_pool_size: u32,
    pub drop_base_index: i32,

    pub states_rpc_endpoint: String,
    pub brokers: String,
    pub kafka_topic: String,
    pub kafka_group_id: String,
    pub kafka_client_id: String,

    pub indexer_prod_url: String,
    pub indexer_test_url: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(Environment::default())
            .add_source(File::with_name("Settings.toml").required(false))
            .build()?
            .try_deserialize()
    }
}

pub fn get_config() -> Arc<Config> {
    Arc::new(Config::new().unwrap_or_else(|e| panic!("Error parsing config: {}", e)))
}