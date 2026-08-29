use o2o::o2o;

#[derive(sqlx::Type, o2o)]
#[sqlx(rename_all = "snake_case")]
#[from_owned(code_executor::Verdict)]
pub enum Verdict {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    CompilationError,
    MemoryLimitExceeded,
    RuntimeError,
    IdleTimeLimitExceeded,
}
