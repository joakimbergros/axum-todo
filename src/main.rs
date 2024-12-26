mod endpoints;
mod todo;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use endpoints::*;
use std::sync::Arc;
use todo::Todo;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let todos = Arc::new(Mutex::new(vec![Todo::new(
        1,
        "First".into(),
        "Test".into(),
    )]));

    let app = Router::new()
        .route("/", get(all))
        .route("/", post(add))
        .route("/:id", get(single))
        .route("/:id", post(toggle))
        .route("/:id", patch(edit))
        .route("/:id", delete(remove))
        .with_state(todos);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
