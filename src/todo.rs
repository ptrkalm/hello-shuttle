use axum::{extract::State, response::IntoResponse, Json};
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub async fn add_todo(
    State(collection): State<Arc<Collection<Todo>>>,
    Json(todo): Json<Todo>,
) -> impl IntoResponse {
    let todo_id = collection.insert_one(todo, None).await.unwrap();

    Json(serde_json::json!(todo_id))
}

pub async fn list_todos(State(collection): State<Arc<Collection<Todo>>>) -> Json<Vec<Todo>> {
    let todos = collection.find(None, None).await.unwrap();

    let todos = todos
        .deserialize_current()
        .into_iter()
        .collect::<Vec<Todo>>();

    Json(todos)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub note: String,
}
