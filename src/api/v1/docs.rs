use std::sync::Arc;

use crate::api::Context;
use aide::{
    axum::{
        routing::{get},
        ApiRouter, IntoApiResponse,
    },
    openapi::OpenApi,
    redoc::Redoc,
};
use axum::{response::IntoResponse, Extension, Json};




pub fn docs_routes() -> ApiRouter<Context> {
    // We infer the return types for these routes
    // as an example.
    //
    // As a result, the `serve_redoc` route will
    // have the `text/html` content-type correctly set
    // with a 200 status.
    aide::gen::infer_responses(true);

    let router = ApiRouter::new()
        .route(
            "/",
            get(Redoc::new("/v1/docs/private/api.json").with_title("Aide Axum").axum_handler()),
        )
        .route("/private/api.json", get(serve_docs));

    // Afterwards we disable response inference because
    // it might be incorrect for other routes.
    aide::gen::infer_responses(false);

    router
}

pub async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}

pub async fn yaml_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    serde_yaml::to_string(&api).unwrap().into_response()
}