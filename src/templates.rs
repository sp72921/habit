use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use chrono::format::{DelayedFormat, StrftimeItems};

use crate::models::{Priority, Recur, Status};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Indextemplate;

#[derive(Template)]
#[template(path = "form.html")]
pub struct FormTemplate;

#[derive(Template)]
#[template(path = "form-edit.html")]
pub struct FormEditTemplate {
    pub habit_uuid: uuid::Uuid,
    pub status: Status,
    pub habit: String,
}

#[derive(Template)]
#[template(path = "habit.html")]
pub struct HabitTemplate<'a> {
    pub id: String,
    pub status: Status,
    pub priority: Option<Priority>,
    pub pattern: Recur,
    pub timestamp: DelayedFormat<StrftimeItems<'a>>,
    pub habit: String,
}

pub struct HtmlTemplate<T>(pub T);

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
