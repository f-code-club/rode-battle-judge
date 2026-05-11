use uuid::Uuid;

use crate::model::Language;

#[derive(Debug)]
pub struct Submission {
    pub problem_id: Uuid,
    pub team_id: Uuid,
    pub language: Language,
    pub code: String,
}
