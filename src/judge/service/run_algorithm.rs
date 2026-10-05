use std::time::Duration;

use byte_unit::Byte;
use code_executor::{Code, Judge, Resource, Verdict};
use futures_lite::StreamExt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    judge::repository::{
        self,
        model::{Problem, Submission},
        problem, submission,
    },
    shared::Storage,
};

pub async fn run_algorithm(
    storage: &Storage,
    pool: &PgPool,
    id: Uuid,
    sub: Submission,
    prob: Problem,
) -> color_eyre::Result<(Verdict, Duration, Byte)> {
    let checker_path = prob
        .checker_path
        .ok_or_else(|| color_eyre::eyre::anyhow!("problem missing checker"))?;
    let checker = storage.download(checker_path).await?;
    let checker_language = prob
        .checker_language
        .expect("checker language must be set if checker is not null");

    let time_limit = prob
        .time_limit
        .expect("algorithm problem must have time limit");
    let memory_limit = prob
        .memory_limit
        .expect("algorithm problem must have memory limit");

    let judge = Judge::builder()
        .checker(Code {
            content: &checker,
            language: checker_language.into(),
        })
        .main(Code {
            content: sub.code.as_bytes(),
            language: sub.language.into(),
        })
        .time_limit(Duration::from_millis(time_limit as u64))
        .resource(Resource {
            memory: Byte::MEGABYTE
                .multiply(memory_limit as usize)
                .expect("memory limit must be valid"),
            ..Default::default()
        })
        .build()
        .await?;
    let judge = match judge.compile().await? {
        Ok(judge) => judge,
        Err(verdict) => {
            return Ok((verdict, Duration::ZERO, Byte::from_u64(0)));
        }
    };

    let mut test_cases = problem::get_test_cases(pool, sub.problem_id);

    let mut transaction = pool.begin().await?;
    let mut verdict = Verdict::Accepted;
    let mut run_time = Duration::ZERO;
    let mut memory_usage = Byte::from_u64(0);
    while let Some(test_case) = test_cases.try_next().await? {
        let input = test_case.input.into_bytes();

        let metrics = judge.run(input).await?;
        verdict = metrics.verdict;
        run_time = run_time.max(metrics.run_time);
        memory_usage = memory_usage.max(metrics.memory_usage);

        if let Err(error) = submission::add_detail(
            &mut *transaction,
            id,
            test_case.id,
            metrics.verdict.into(),
            metrics.run_time.as_millis() as i32,
            metrics.memory_usage.as_u64() as i32,
        )
        .await
        {
            tracing::error!(?error, "failed to save detail");
            break;
        }

        if verdict != Verdict::Accepted {
            break;
        }
    }
    transaction.commit().await?;

    Ok((verdict, run_time, memory_usage))
}

impl From<repository::model::Language> for code_executor::Language {
    fn from(val: repository::model::Language) -> Self {
        match val {
            repository::model::Language::Rust => code_executor::language::RUST,
            repository::model::Language::Cpp => code_executor::language::CPP,
            repository::model::Language::Python => code_executor::language::PYTHON,
            repository::model::Language::Java => code_executor::language::JAVA,
            _ => unreachable!(),
        }
    }
}
