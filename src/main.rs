use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Json, Router,
};
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use rinja::Template;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
enum Recur {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Todo,
    Done,
    Postpone,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Todo => write!(f, "TODO"),
            Status::Done => write!(f, "DONE"),
            Status::Postpone => write!(f, "POSTPONE"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CreateHabit {
    habit: String,
    // pattern: Recur,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/track", get(track))
        .route("/habit", post(habit));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct Indextemplate;

#[derive(Template)]
#[template(path = "track.html")]
struct TrackTemplate;

// #[derive(Template)]
// #[template(path = "habit.html")]
// struct HabitTemplate {
//     timestamp: DateTime<Local>,
//     label: Status,
//     habit: String,
//     // pattern: Recur,
// }

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

async fn index() -> impl IntoResponse {
    HtmlTemplate(Indextemplate {})
}

async fn track() -> impl IntoResponse {
    HtmlTemplate(TrackTemplate {})
}

async fn habit(Form(payload): Form<CreateHabit>) -> impl IntoResponse {
    let local_time = Local::now();
    let status = Status::Todo;
    let habit = payload.habit;
    // HtmlTemplate(HabitTemplate {
    //     timestamp: local_time,
    //     label: Status::Todo,
    //     habit: payload.habit,
    // })

    Html(format!(
        "<b>{local_time:?}</b> <i>{status:?}</i> {habit:?}</div>"
    ))
    .into_response();

    // println!("{habit:#?}")
    // (StatusCode::CREATED, Json(habit))
}
