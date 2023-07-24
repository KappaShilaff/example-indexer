use sqlx::PgPool;

mod users;

#[derive(Clone)]
pub struct SqlxClient {
    pub pool: PgPool,
}

impl SqlxClient {
    pub fn new(pool: PgPool) -> SqlxClient {
        SqlxClient { pool }
    }
}