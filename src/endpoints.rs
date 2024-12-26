use crate::todo::{SetTodo, Todo};
use axum::extract::{Path, State};
use axum::http::header::LOCATION;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn all(State(state): State<Arc<Mutex<Vec<Todo>>>>) -> Response {
    let values = state.lock().await;

    match values.len() {
        1.. => (StatusCode::OK, Json(values.clone())).into_response(),
        _ => (StatusCode::NO_CONTENT, ()).into_response(),
    }
}

pub async fn single(
    State(state): State<Arc<Mutex<Vec<Todo>>>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let values = state.lock().await;

    let item = values.iter().find(|x| x.id == id);

    match item {
        Some(i) => Json(i.clone()).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn add(
    State(state): State<Arc<Mutex<Vec<Todo>>>>,
    Json(todo): Json<SetTodo>,
) -> impl IntoResponse {
    let mut values = state.lock().await;

    let last_id = values.iter().last().map_or(0, |todo| todo.id);

    let id = last_id + 1;
    values.push(Todo::new(id, todo.title, todo.content));

    let mut header_map = HeaderMap::new();

    let location = format!("/{}", id);
    header_map.insert(LOCATION, HeaderValue::from_str(location.as_str()).unwrap());
    (StatusCode::CREATED, header_map, ()).into_response()
}

pub async fn remove(State(state): State<Arc<Mutex<Vec<Todo>>>>, Path(id): Path<u64>) -> Response {
    let mut values = state.lock().await;

    let pos = values.iter().position(|x| x.id == id);

    match pos {
        Some(pos) => {
            values.remove(pos);
            StatusCode::NO_CONTENT.into_response()
        }
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn edit(
    State(state): State<Arc<Mutex<Vec<Todo>>>>,
    Path(id): Path<u64>,
    Json(todo): Json<SetTodo>,
) -> Response {
    let mut values = state.lock().await;

    let pos = values.iter_mut().find(|t| t.id == id);

    match pos {
        Some(t) => {
            t.update(todo.title, todo.content);

            let mut header_map = HeaderMap::new();

            let location = format!("/{}", t.id);
            header_map.insert(LOCATION, HeaderValue::from_str(location.as_str()).unwrap());
            (StatusCode::NO_CONTENT, header_map, ()).into_response()
        }
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn toggle(
    State(state): State<Arc<Mutex<Vec<Todo>>>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let mut values = state.lock().await;

    let todo = values.iter_mut().find(|t| t.id == id);

    match todo {
        Some(t) => {
            t.toggle_state();

            StatusCode::NO_CONTENT.into_response()
        }
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}
