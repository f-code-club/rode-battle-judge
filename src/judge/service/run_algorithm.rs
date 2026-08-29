use std::time::Duration;

use byte_unit::Byte;
use code_executor::{Code, Judge, Resource, Verdict};

use crate::{
    judge::repository::{
        self,
        model::{Problem, Submission},
    },
    shared::Storage,
};

pub async fn run_algorithm(
    storage: &Storage,
    sub: Submission,
    prob: Problem,
) -> color_eyre::Result<Verdict> {
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
        .time_limit(Duration::from_secs(time_limit as u64))
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
        Err(verdict) => return Ok(verdict),
    };

    let test_cases = prob.test_cases.unwrap_or(vec![]);
    let metrics = judge
        .batch_run(test_cases.iter().map(|x| x.as_bytes()))
        .await?;

    Ok(metrics.verdict)
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
