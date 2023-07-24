use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HelloResponse {
    pub user: String,
}
