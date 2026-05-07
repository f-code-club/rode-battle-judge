use byte_unit::Byte;
use chrono::Duration;

use super::TestCase;

pub struct Problem {
    pub time_limit: Duration,
    pub memory_limit: Byte,
    pub test_cases: Vec<TestCase>,
}
