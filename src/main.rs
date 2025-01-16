use axum::{
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

mod models;
mod templates;
use crate::models::{Priority, Recur, Status};

}

}

}

        }
    }
}

#[derive(Debug, Deserialize)]
struct CreateHabit {
    pattern: Recur,
    datetime: String,
    status: Status,
    priority: Priority,
    habit: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/form", get(form))
        .route("/habit", post(habit))
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    templates::HtmlTemplate(templates::Indextemplate {})
}

async fn form() -> impl IntoResponse {
    templates::HtmlTemplate(templates::FormTemplate {})
}

}

async fn habit(Form(payload): Form<CreateHabit>) -> impl IntoResponse {
    // let local_time = DateTime::parse_from_rfc3339(&payload.datetime).unwrap();
    // let local_time = DateTime::from_timestamp_millis(payload.datetime).unwrap();
    let local_time =
        chrono::NaiveDateTime::parse_from_str(&payload.datetime, "%Y-%m-%dT%H:%M").unwrap();
    let local_format = local_time.format("%d/%m/%Y %H:%M");

    templates::HtmlTemplate(templates::HabitTemplate {
        status: payload.status,
        priority: Some(payload.priority),
        pattern: payload.pattern,
        habit: payload.habit,
        timestamp: local_format,
    })
}
