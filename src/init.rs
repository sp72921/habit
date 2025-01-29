use axum::{
    body::Body,
    http::{Request, Response},
};
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tracing::Span;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up logging");
}

pub fn on_request(request: &Request<Body>, _: &Span) {
    tracing::info!(
        "-> STARTED: method {} path {}",
        request.method(),
        request.uri().path()
    )
}

pub fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!(
        "<- GENERATED: status {} in {:?}",
        response.status(),
        latency
    )
}

pub fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("!! FAILED: {:?} after {:?}", error, latency)
}
