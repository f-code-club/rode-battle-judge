#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = StatusCode::OK, description = "Return ok", body = &'static str)
    )
)]
pub async fn health() -> &'static str {
    "ok"
}
