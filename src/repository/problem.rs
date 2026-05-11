use byte_unit::Byte;
use chrono::Duration;
use uuid::Uuid;

use crate::model::{Language, Problem};

#[allow(unused)]
pub fn get(id: Uuid) -> color_eyre::Result<Problem> {
    Ok(Problem {
        checker_language: Language::Cpp,
        checker_path: "checker".to_string(),
        time_limit: Duration::seconds(1),
        memory_limit: Byte::GIBIBYTE,
        test_cases: vec![],
    })
}
