use uuid::Uuid;

use crate::model::{Language, Submission};

#[allow(unused)]
pub fn get(id: Uuid) -> color_eyre::Result<Option<Submission>> {
    Ok(Some(Submission {
        problem_id: Uuid::new_v4(),
        team_id: Uuid::new_v4(),
        language: Language::Cpp,
        code: String::new(),
    }))
}
