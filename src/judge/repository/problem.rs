#![allow(unused)]

use sqlx::PgExecutor;
use uuid::Uuid;

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
                memory_limit,
                (
                    SELECT ARRAY_AGG(t.input_path)
                    FROM test_cases t
                    WHERE t.problem_id = $1
                ) as test_cases
            FROM problems
            WHERE id = $1
        "#,
        id
    )
    .fetch_optional(executer)
    .await
}
