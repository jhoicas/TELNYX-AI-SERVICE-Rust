use axum::{
    middleware::Next,
    http::Request,
};
use tracing::info;

pub async fn logging_middleware<B>(
    req: Request<B>,
    next: Next,
) -> impl axum::response::IntoResponse {
    let start = std::time::Instant::now();
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    let response = next.run(req).await;

    let duration = start.elapsed();
    info!(
        "📡 Request completado",
        method = method.to_string(),
        path = path,
        status = response.status().as_u16(),
        duration_ms = duration.as_millis()
    );

    response
}
