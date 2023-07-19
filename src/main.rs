use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, Shuttle!"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}

#[tokio::test]
async fn hello_world_test_get() {
    let result = hello_world().await;
    assert_eq!(result, "Hello, Shuttle!");
}