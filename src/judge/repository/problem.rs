#![allow(unused)]

use sqlx::PgExecutor;
use uuid::Uuid;

use crate::judge::repository::model::TestCase;

use super::model::Problem;

pub async fn get(executer: impl PgExecutor<'_>, id: Uuid) -> sqlx::Result<Option<Problem>> {
    sqlx::query_as!(
        Problem,
        r#"
            SELECT
                COALESCE(
                    (
                        SELECT ARRAY_AGG(pl.language)
                        FROM problem_languages pl
                        WHERE pl.problem_id = $1
                    ),
                    ARRAY[]::language[]
                ) as "languages!:_",
                content,
                checker_language as "checker_language:_",
                checker_path,
                time_limit,
                memory_limit
            FROM problems
            WHERE id = $1
        "#,
        id
    )
    .fetch_optional(executer)
    .await
}

pub async fn get_test_cases(
    executer: impl PgExecutor<'_>,
    id: Uuid,
) -> sqlx::Result<Vec<TestCase>> {
    sqlx::query_as!(
        TestCase,
        r#"
            SELECT id, input
            FROM test_cases
            WHERE problem_id = $1
        "#,
        id
    )
    .fetch_all(executer)
    .await
}
