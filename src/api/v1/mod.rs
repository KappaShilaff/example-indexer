use super::Context;
use aide::axum::routing::get;
use aide::axum::ApiRouter;
use axum::{Extension, Router};
use std::sync::Arc;
use crate::api::get_open_api;

pub mod docs;
pub mod example;

pub fn router(ctx: Context) -> Router {
    let mut api = get_open_api(
        "v1",
        &ctx.caches.constants.indexer_prod_url,
        &ctx.caches.constants.indexer_test_url,
    );

    ApiRouter::new()
        .api_route_with("/swagger.yaml", get(docs::yaml_docs), |op| {
            op.tag("swagger")
        })
        .nest("/example", example::router())
        .nest("/docs", docs::docs_routes())
        .with_state(ctx)
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)))
}
