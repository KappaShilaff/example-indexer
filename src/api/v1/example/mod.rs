use crate::api::Context;
use aide::axum::routing::post;
use aide::axum::ApiRouter;
use aide::transform::TransformPathItem;
use axum::extract::State;
use axum::Json;

pub mod requests;
pub mod responses;

use requests::*;
use responses::*;

type Result<T> = super::super::error::Result<Json<T>>;

pub(crate) fn router() -> ApiRouter<Context> {
    ApiRouter::new().api_route_with("/hello", post(hello), tag)
}

pub async fn hello(
    State(ctx): State<Context>,
    Json(input): Json<HelloRequest>,
) -> Result<HelloResponse> {
    let res = ctx.services.hello(input.user_address).await?;

    Ok(Json(HelloResponse { user: res }))
}

fn tag(op: TransformPathItem) -> TransformPathItem {
    op.tag("example tag")
}
