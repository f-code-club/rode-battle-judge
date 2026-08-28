use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    judge::repository::{model::Language, problem, submission},
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
        todo!()
    } else {
        todo!()
    }
}
