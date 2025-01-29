use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Form,
};
use uuid::Uuid;

use crate::{
    templates::{FormEditTemplate, FormTemplate, HabitTemplate, HtmlTemplate, Indextemplate},
    Db, Habit, UpdateTodo,
};

pub async fn index() -> impl IntoResponse {
    HtmlTemplate(Indextemplate {})
}

pub async fn form() -> impl IntoResponse {
    HtmlTemplate(FormTemplate {})
}

pub async fn habit(State(db): State<Db>, Form(payload): Form<Habit>) -> impl IntoResponse {
    let local_time =
        chrono::NaiveDateTime::parse_from_str(&payload.datetime, "%Y-%m-%dT%H:%M").unwrap();
    let local_format = local_time.format("%d/%m/%Y %H:%M");

    let h = Habit {
        pattern: payload.pattern,
        datetime: payload.datetime,
        status: payload.status,
        priority: payload.priority,
        habit: payload.habit,
    };

    let id = Uuid::new_v4();

    db.write().unwrap().insert(id, h.clone());

    HtmlTemplate(HabitTemplate {
        id: id.to_string(),
        status: h.status,
        priority: Some(h.priority),
        pattern: h.pattern,
        habit: h.habit,
        timestamp: local_format,
    })
}

pub async fn habit_edit(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
    Form(input): Form<UpdateTodo>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let mut h = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    if let Some(changed_status) = input.status {
        h.status = changed_status;
    }

    if let Some(changed_habit) = input.habit {
        h.habit = changed_habit;
    }

    db.write().unwrap().insert(id, h.clone());

    let html = FormEditTemplate {
        habit_uuid: id,
        status: h.status,
        habit: h.habit,
    };

    Ok(HtmlTemplate(html))
}

pub async fn habit_delete() {}
