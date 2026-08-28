#[derive(sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum Verdict {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    CompilationError,
    MemoryLimitExceeded,
    RuntimeError,
    IdleTimeLimitExceeded,
}
