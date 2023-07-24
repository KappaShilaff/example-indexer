use crate::services::Services;
use anyhow::Result;

impl Services {
    pub async fn hello(&self, user: String) -> Result<String> {
        self.caches.sqlx_client.get_user(&user).await
    }
}
