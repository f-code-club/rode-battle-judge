use uuid::Uuid;

use crate::repository::{problem, submission};

pub async fn consume(raw: &[u8]) -> color_eyre::Result<()> {
    let id = Uuid::from_slice(raw)?;

    let s = submission::get(id)?;
    let p = problem::get(s.problem_id);
    println!("{:#?}", p);

    Ok(())
}
