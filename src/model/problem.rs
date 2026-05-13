use byte_unit::Byte;
use chrono::Duration;

use crate::model::{Language, TestCase};

#[derive(Debug)]
pub struct Problem {
    pub checker_language: Language,
    pub checker_path: String,
    pub time_limit: Duration,
    pub memory_limit: Byte,
    pub test_cases: Vec<TestCase>,
}
