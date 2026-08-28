#[derive(sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum Language {
    Rust,
    Cpp,
    Python,
    Java,
    Html,
}
