use sqlx::PgPool;
use uuid::Uuid;

use crate::judge::repository::{model::Language, problem, submission};

use super::model::Metrics;

pub async fn run(pool: &PgPool, id: Uuid) -> color_eyre::Result<Metrics> {
    let sub = submission::get(pool, id)
        .await?
        .ok_or(color_eyre::eyre::anyhow!("invalid submission id"))?;
    let prob = problem::get(pool, sub.problem_id)
        .await?
        .ok_or(color_eyre::eyre::anyhow!("invalid problem id"))?;

    if prob.languages.contains(&Language::Html) {
        todo!()
    } else {
        todo!()
    }
}
