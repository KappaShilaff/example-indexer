use crate::services::Services;

use aide::openapi::{Info, OpenApi, Server};
use axum::routing::get;
use axum::{Json, Router};
use axum_prometheus::metrics_exporter_prometheus::PrometheusHandle;
use axum_prometheus::{PrometheusMetricLayer, PrometheusMetricLayerBuilder};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use crate::models::caches::Caches;

type Result<T> = error::Result<Json<T>>;
mod error;
pub mod v1;

#[derive(Clone)]
pub struct Context {
    pub services: Arc<Services>,
    pub caches: Caches,
}

fn router(ctx: Context) -> Router {
    Router::new()
        .route("/healthcheck", get(health_check))
        .nest("/v1", v1::router(ctx))
}

async fn health_check() -> Result<()> {
    Ok(Json(()))
}

pub async fn http_service(services: Arc<Services>, caches: Caches) {
    let ctx = Context { services, caches };
    aide::gen::extract_schemas(true);

    let (prometheus_layer, metric_handle) = get_metrics_layer_and_handle();

    let router_metrics =
        Router::new().route("/metrics", get(|| async move { metric_handle.render() }));

    tokio::spawn(async move {
        axum::Server::bind(&"0.0.0.0:10000".parse().unwrap())
            .serve(router_metrics.into_make_service())
            .await
            .unwrap()
    });

    let router = router(ctx.clone()).layer(
        ServiceBuilder::new()
            .layer(prometheus_layer)
            .layer(CorsLayer::permissive()),
    );

    axum::Server::bind(&ctx.caches.constants.server_addr.to_string().parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn get_open_api(version: &str, prod_url: &str, test_url: &str) -> OpenApi {
    OpenApi {
        info: Info {
            description: Some("an example API".to_string()),
            ..Info::default()
        },
        servers: vec![
            Server {
                url: format!("{}/{}", prod_url, version),
                description: Some("production".to_string()),
                variables: Default::default(),
                extensions: Default::default(),
            },
            Server {
                url: format!("{}/{}", test_url, version),
                description: Some("test".to_string()),
                variables: Default::default(),
                extensions: Default::default(),
            },
        ],
        ..OpenApi::default()
    }
}

fn get_metrics_layer_and_handle() -> (PrometheusMetricLayer<'static>, PrometheusHandle) {
    PrometheusMetricLayerBuilder::new()
        .with_group_patterns_as("/v2/pools/address/address", &["/v2/pools/address/:address"])
        .with_group_patterns_as("/v1/pairs/address/address", &["/v1/pairs/address/:address"])
        .with_group_patterns_as(
            "/v1/pairs/left/left_address/right/right_address",
            &["/v1/pairs/left/:left_address/right/:right_address"],
        )
        .with_group_patterns_as(
            "/v1/pairs/left/left_address/right/right_address/ohlcv",
            &["/v1/pairs/left/:left_address/right/:right_address/ohlcv"],
        )
        .with_group_patterns_as(
            "/v1/pairs/address/address/ohlcv",
            &["/v1/pairs/address/:address/ohlcv"],
        )
        .with_group_patterns_as(
            "/v1/pairs/address/address/volume",
            &["/v1/pairs/address/:address/volume"],
        )
        .with_group_patterns_as(
            "/v1/pairs/address/address/tvl",
            &["/v1/pairs/address/:address/tvl"],
        )
        .with_group_patterns_as(
            "/v1/pairs/address/address/fee",
            &["/v1/pairs/address/:address/fee"],
        )
        .with_group_patterns_as(
            "/v1/pairs/pool/pool_address/ts/timestamp",
            &["/v1/pairs/pool/:pool_address/ts/:timestamp"],
        )
        .with_group_patterns_as(
            "/v1/currencies/currency_address",
            &["/v1/currencies/:currency_address"],
        )
        .with_group_patterns_as(
            "/v1/currencies/currency_address/prices",
            &["/v1/currencies/:currency_address/prices"],
        )
        .with_group_patterns_as(
            "/v1/currencies/currency_address/volume",
            &["/v1/currencies/:currency_address/volume"],
        )
        .with_group_patterns_as(
            "/v1/currencies/currency_address/tvl",
            &["/v1/currencies/:currency_address/tvl"],
        )
        .with_group_patterns_as(
            "/v1/currencies/currency_address/fee",
            &["/v1/currencies/:currency_address/fee"],
        )
        .with_default_metrics()
        .build_pair()
}
