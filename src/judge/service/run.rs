use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    judge::{
        repository::{model::Language, problem, submission},
        service::run_algorithm::run_algorithm,
        service::run_frontend::run_frontend,
    },
    shared::Storage,
};

pub async fn run(storage: &Storage, pool: &PgPool, id: Uuid) -> color_eyre::Result<()> {
    let sub = submission::get(pool, id)
        .await?
        .ok_or(color_eyre::eyre::anyhow!("invalid submission id"))?;
    let prob = problem::get(pool, sub.problem_id)
        .await?
        .ok_or(color_eyre::eyre::anyhow!("invalid problem id"))?;

    if prob.languages.contains(&Language::Html) {
        let score = run_frontend(storage, sub, prob).await?;
        submission::update(pool, id, None, Some(score)).await?;
    } else {
        let verdict = run_algorithm(storage, sub, prob).await?;
        submission::update(pool, id, Some(verdict.into()), None).await?;
    }

    Ok(())
}
