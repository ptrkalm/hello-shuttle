mod hello;
use axum::{routing::get, Router};

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello::hello_world))
        .route("/v2", get(hello::hello_world_2))
        .route("/v3", get(hello::hello_world_3));

    Ok(router.into())
}
