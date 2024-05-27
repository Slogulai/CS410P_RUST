use crate::*;

pub async fn question_db() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}

pub async fn set_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS questions (
            id SERIAL PRIMARY KEY,
            question VARCHAR NOT NULL,
            answer VARCHAR NOT NULL,
            tags VARCHAR NOT NULL
        )
    "
    )
    .execute(pool)
    .await?;
    Ok(())
}
