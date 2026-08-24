use sqlx::PgExecutor;
use uuid::Uuid;

use super::model::Problem;

pub async fn get(executer: impl PgExecutor<'_>, id: Uuid) -> sqlx::Result<Option<Problem>> {
    sqlx::query_as!(
        Problem,
        r#"
            SELECT
                content,
                checker_language as "checker_language:_",
                checker_path,
                time_limit,
                memory_limit,
                (
                    SELECT ARRAY_AGG(t.input_path)
                    FROM test_cases t
                    WHERE t.problem_id = $1
                ) AS test_cases
            FROM problems
            WHERE id = $1
        "#,
        id
    )
    .fetch_optional(executer)
    .await
}
