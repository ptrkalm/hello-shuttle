mod hello;
mod todo;
use axum::{extract::Json, extract::State, response::IntoResponse, routing::get, Router};
use mongodb::Database;
use std::sync::Arc;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::MongoDb] database: Database) -> shuttle_axum::ShuttleAxum {
    let collection = database.collection::<todo::Todo>("todos");

    let router = Router::new()
        .route("/", get(hello::hello_world))
        .route("/v2", get(hello::hello_world_2))
        .route("/todos", get(todo::list_todos).post(todo::add_todo))
        .with_state(Arc::new(collection));

    Ok(router.into())
}
