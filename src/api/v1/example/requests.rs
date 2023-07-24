use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HelloRequest {
    pub user_address: String,
}
