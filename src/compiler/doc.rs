use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::compiler::{error::ApiError, handler};

#[derive(OpenApi)]
#[openapi(
    paths(handler::health, handler::compile),
    components(schemas(ApiError,))
)]
struct ApiDoc;

pub fn build() -> Router {
    SwaggerUi::new("/swagger")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
        .into()
}
