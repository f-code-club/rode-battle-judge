pub enum Verdict {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    CompilationError,
    MemoryLimitExceeded,
    RuntimeError,
    IdleTimeLimitExceeded,
}

pub struct Metrics {
    pub verdict: Verdict,
    pub score: u32,
}
