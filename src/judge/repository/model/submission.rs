use uuid::Uuid;

use super::Language;

pub struct Submission {
    pub problem_id: Uuid,
    pub account_id: Uuid,
    pub language: Language,
    pub code: String,
}
