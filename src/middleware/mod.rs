use axum::{
    extract::Request, // USAR ESTO en lugar de http::Request para Axum 0.7
    middleware::Next,
    response::IntoResponse,
};
use tracing::info;

// Quitamos el genérico <B>. Ahora recibimos 'Request' directamente.
pub async fn logging_middleware(
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let start = std::time::Instant::now();
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    // Ahora next.run(req) funcionará porque los tipos coinciden
    let response = next.run(req).await;

    let duration = start.elapsed();

    // CORRECCIÓN DE LOGS:
    // Opción 1 (Texto plano): Agregamos los corchetes {} dentro de las comillas
    info!(
        "📡 Request completado | Method: {} | Path: {} | Status: {} | Duration: {}ms",
        method,
        path,
        response.status().as_u16(),
        duration.as_millis()
    );

    /* // Opción 2 (Estructurado - Más profesional para Datadog/CloudWatch):
    // Si prefieres este estilo, descomenta esto y comenta el de arriba:
    info!(
        method = ?method,
        path = %path,
        status = %response.status().as_u16(),
        duration_ms = %duration.as_millis(),
        "📡 Request completado"
    ); 
    */

    response
}