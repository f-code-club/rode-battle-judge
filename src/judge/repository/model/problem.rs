use super::Language;

pub struct Problem {
    pub checker_language: Option<Language>,
    pub checker_path: Option<String>,
    pub time_limit: Option<i32>,
    pub memory_limit: Option<i32>,
    pub test_cases: Option<Vec<String>>,
}
