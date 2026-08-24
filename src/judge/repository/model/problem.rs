use std::time::Duration;

use super::Language;

pub struct Problem {
    pub checker_language: Language,
    pub checker_path: String,
    pub time_limit: Duration,
    pub memory_limit: i32,
}
