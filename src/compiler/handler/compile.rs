use axum::{Json, http::StatusCode};
use code_executor::{Code, Judge};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::compiler::error::{self, ApiError, ApiResult, ResultExt};

#[derive(Debug, ToSchema, Deserialize)]
#[schema(description = "Programming language supported by the compiler")]
pub enum Language {
    Rust,
    Cpp,
    Python,
    Java,
}

#[derive(Debug, ToSchema, Deserialize)]
#[schema(
    as = compile::Request,
    description = "Source code compilation request"
)]
pub struct Request {
    #[schema(example = "fn main() { println!(\"Hello, world!\"); }")]
    pub code: String,

    #[schema(example = "Rust")]
    pub language: Language,
}

#[utoipa::path(
    post,
    path = "/compile",
    tag = "Compiler",
    operation_id = "compiler::compile",
    request_body(
        content = Request,
        description = "Source code to compile"
    ),
    responses(
        (
            status = 200,
            description = "Compilation succeeded",
            content_type = "application/octet-stream",
            body = Vec<u8>
        ),
        (
            status = 400,
            description = "Invalid request or compilation failed",
            body = ApiError
        ),
        (
            status = 500,
            description = "Internal compiler error",
            body = ApiError
        )
    )
)]
#[tracing::instrument(err(Debug))]
pub async fn compile(Json(request): Json<Request>) -> ApiResult<Vec<u8>> {
    let compiler = Judge::builder()
        .main(Code {
            content: request.code.as_bytes(),
            language: request.language.into(),
        })
        .build()
        .await
        .with_context(StatusCode::BAD_REQUEST, "failed to save code")?;
    let compiler = compiler
        .compile()
        .await
        .with_context(
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to run compile command",
        )?
        .map_err(|_| ApiError {
            context: error::Context {
                status: StatusCode::BAD_REQUEST,
                message: "invalid code".to_string(),
                ..Default::default()
            },
            ..Default::default()
        })?;

    compiler
        .read_executable()
        .await
        .with_context(StatusCode::INTERNAL_SERVER_ERROR, "failed to read file")
}

impl From<Language> for code_executor::Language {
    fn from(val: Language) -> Self {
        match val {
            Language::Rust => code_executor::language::RUST,
            Language::Cpp => code_executor::language::CPP,
            Language::Python => code_executor::language::PYTHON,
            Language::Java => code_executor::language::JAVA,
        }
    }
}
