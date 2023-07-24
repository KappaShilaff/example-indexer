use crate::sqlx_client::SqlxClient;
use anyhow::Result;

impl SqlxClient {
    pub async fn new_user(&self, _user_address: &str) -> Result<()> {
        // sqlx::query!(
        //     r#"
        //     INSERT INTO example_table (user_id)
        //     VALUES ($1)
        //     ON CONFLICT DO NOTHING
        //     "#,
        //     user_address
        // )
        // .execute(&self.pool)
        // .await?;
        Ok(())
    }

    pub async fn get_user(&self, _user_address: &str) -> Result<String> {
        // let res = sqlx::query!(
        //     r#"
        //     SELECT user_address
        //     FROM example_table
        //     WHERE user_address = $1
        //     "#,
        //     user_id
        // )
        // .fetch_one(&self.pool)
        // .await?;
        // Ok(res.user_id)
        Ok("example user address".to_string())
    }
}