use sqlx::PgExecutor;
use uuid::Uuid;

pub async fn get_by_problem(
    executor: impl PgExecutor<'_>,
    problem_id: Uuid,
) -> sqlx::Result<Vec<String>> {
    sqlx::query_scalar!(
        r#"
            SELECT input_path
            FROM test_cases
            WHERE problem_id = $1
        "#,
        problem_id
    )
    .fetch_all(executor)
    .await
}
