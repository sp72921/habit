use axum::{
    body::Body,
    http::Request,
    routing::{delete, get, patch, post},
    Router,
};
use handlers::{form, habit, habit_delete, habit_edit, index};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

mod handlers;
mod init;
mod models;
mod templates;
use crate::models::{Priority, Recur, Status};

#[derive(Debug, Serialize)]
// TODO
struct Data {
    date: String,
    value: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct Habit {
    pattern: Recur,
    datetime: String,
    status: Status,
    priority: Priority,
    habit: String,
}

#[derive(Clone, Debug, Deserialize)]
struct UpdateTodo {
    status: Option<Status>,
    habit: Option<String>,
}

type Db = Arc<RwLock<HashMap<Uuid, Habit>>>;

#[tokio::main]
async fn main() {
    init::logging();

    let db = Db::default();

    let app = Router::new()
        .route("/", get(index))
        .route("/form", get(form).post(habit))
        .route("/habit", post(habit))
        .route("/form/edit/:id", patch(habit_edit))
        .route("/delete/:id", delete(habit_delete))
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(|_: &Request<Body>| tracing::info_span!("R"))
                .on_request(init::on_request)
                .on_response(init::on_response)
                .on_failure(init::on_failure),
        )
        .with_state(db);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Server is starting...");
    tracing::info!("Listening at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
