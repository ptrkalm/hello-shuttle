mod hello;
use axum::{extract::Json, extract::State, routing::get, Router, response::IntoResponse};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::MongoDb] database: Database) -> shuttle_axum::ShuttleAxum {
    let collection = database.collection::<Todo>("todos");

    let router = Router::new()
        .route("/", get(hello::hello_world))
        .route("/v2", get(hello::hello_world_2))
        .route("/todos", get(list_todos).post(add_todo))
        .with_state(Arc::new(collection));

    Ok(router.into())
}

async fn add_todo(State(collection): State<Arc<Collection<Todo>>>, Json(todo): Json<Todo>) -> impl IntoResponse {
    let todo_id = collection
    .insert_one(todo, None)
    .await
    .unwrap();

    Json(serde_json::json!(todo_id))
}

async fn list_todos(State(collection): State<Arc<Collection<Todo>>>) -> Json<Vec<Todo>> {
    let todos = collection.find(None, None)
    .await
    .unwrap();

    let todos = todos.deserialize_current().into_iter().collect::<Vec<Todo>>();

    Json(todos)
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    pub note: String,
}
