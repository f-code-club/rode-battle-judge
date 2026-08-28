use std::env;

use sqlx::PgPool;

const DATABASE_URL_ENV: &str = "DATABASE_URL";

pub async fn connect() -> color_eyre::Result<PgPool> {
    let database_url = env::var(DATABASE_URL_ENV)?;
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}
