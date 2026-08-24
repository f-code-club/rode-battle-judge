use sqlx::PgExecutor;
use uuid::Uuid;

use super::model::{Submission, Verdict};

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

pub async fn update(
    executor: impl PgExecutor<'_>,
    id: Uuid,
    verdict: Option<Verdict>,
    score: Option<f32>,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            UPDATE submissions
            SET
                verdict = $2,
                score = $3,
                updated_at = now()
            WHERE id = $1
        "#,
        id,
        verdict as Option<Verdict>,
        score,
    )
    .execute(executor)
    .await?;

    Ok(())
}
