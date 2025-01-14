use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use chrono::format::{DelayedFormat, StrftimeItems};
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

impl std::fmt::Display for Recur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Recur::Daily => write!(f, "|Daily|"),
            Recur::Weekly => write!(f, "|Weekly|"),
            Recur::Monthly => write!(f, "|Monthly|"),
        }
    }
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
    pattern: Option<Recur>,
    datetime: String,
    status: Option<Status>,
    habit: String,
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

#[derive(Template)]
#[template(path = "habit.html")]
struct HabitTemplate<'a> {
    pattern: Recur,
    timestamp: DelayedFormat<StrftimeItems<'a>>,
    label: Status,
    habit: String,
}

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
    // let local_time = DateTime::parse_from_rfc3339(&payload.datetime).unwrap();
    // let local_time = DateTime::from_timestamp_millis(payload.datetime).unwrap();
    let local_time =
        chrono::NaiveDateTime::parse_from_str(&payload.datetime, "%Y-%m-%dT%H:%M").unwrap();
    let local_format = local_time.format("%d/%m/%Y %H:%M");

    let p = match payload.pattern {
        Some(val) => val,
        None => Recur::Daily,
    };

    let l = match payload.status {
        Some(val) => val,
        None => Status::Todo,
    };

    HtmlTemplate(HabitTemplate {
        pattern: p,
        timestamp: local_format,
        label: l,
        habit: payload.habit,
    })
}
