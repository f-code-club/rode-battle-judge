use sqlx::PgExecutor;
use uuid::Uuid;

use super::model::Submission;

pub async fn get(executor: impl PgExecutor<'_>, id: Uuid) -> sqlx::Result<Option<Submission>> {
    sqlx::query_as!(
        Submission,
        r#"
            SELECT problem_id, account_id, language as "language: _", code
            FROM submissions
            WHERE id = $1
        "#,
        id
    )
    .fetch_optional(executor)
    .await
}
