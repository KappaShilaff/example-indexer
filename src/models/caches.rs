use std::sync::Arc;
use transaction_consumer::TransactionConsumer;
use crate::settings::Config;
use crate::sqlx_client::SqlxClient;

#[derive(Clone)]
pub struct Caches {
    pub sqlx_client: SqlxClient,
    pub transaction_consumer: Arc<TransactionConsumer>,
    pub constants: Arc<Config>
}

impl Caches {
    pub fn new(sqlx_client: SqlxClient, transaction_consumer: Arc<TransactionConsumer>, config: Arc<Config>) -> Self {
        Self {
            sqlx_client,
            transaction_consumer,
            constants: config
        }
    }
}