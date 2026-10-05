use std::time::Duration;

use byte_unit::Byte;
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

        submission::update()
            .executor(pool)
            .id(id)
            .score(score)
            .call()
            .await?;
    } else {
        let metrics_list = run_algorithm(storage, pool, sub, prob).await?;
        let n = metrics_list.len();

        let verdict = metrics_list[n - 1].verdict;
        let run_time: Duration = metrics_list
            .iter()
            .map(|x| x.run_time)
            .max()
            .expect("There must be at least 1 run");
        let memory_usage: Byte = metrics_list
            .iter()
            .map(|x| x.memory_usage)
            .max()
            .expect("There must be at least 1 run");

        submission::update()
            .executor(pool)
            .id(id)
            .verdict(verdict.into())
            .run_time(run_time.as_millis() as i32)
            .memory_usage(memory_usage.as_u64() as i32)
            .call()
            .await?;
    }

    Ok(())
}
