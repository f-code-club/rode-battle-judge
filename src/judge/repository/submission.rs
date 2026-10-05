#![allow(unused)]

use bon::builder;
use moka::ops::compute::Op;
use sqlx::{PgExecutor, PgPool};
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

#[builder]
pub async fn update(
    executor: &PgPool,
    id: Uuid,
    verdict: Option<Verdict>,
    run_time: Option<i32>,
    memory_usage: Option<i32>,
    score: Option<f32>,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            UPDATE submissions
            SET
                verdict = $2,
                run_time = $3,
                memory_usage = $4,
                score = $5,
                updated_at = now()
            WHERE id = $1
        "#,
        id,
        verdict as Option<Verdict>,
        run_time,
        memory_usage,
        score,
    )
    .execute(executor)
    .await?;

    Ok(())
}
