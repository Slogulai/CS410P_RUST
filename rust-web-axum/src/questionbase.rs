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
            content VARCHAR NOT NULL,
            tags VARCHAR NOT NULL
        )
    "
    )
    .execute(pool)
    .await?;
    Ok(())
}


pub struct MyDatabase {
    pool: PgPool,
}

impl MyDatabase {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    //Retrieve
    pub async fn get_question(&self, id: i32) -> Result<Question, sqlx::Error> {
        let row: PgRow = sqlx::query("SELECT * FROM questions WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        let question = Question {
            id: row.get(0),
            title: row.get(1),
            content: row.get(2),
            tags: row.get(3),
        };

        Ok(question)
    }

    pub async fn get_questions(&self, limit: i64, offset: i64) -> Result<Vec<Question>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Question>("SELECT * FROM questions ORDER BY id LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }


    pub async fn create_question(&self) -> Result<Question, sqlx::Error> {
        let row = sqlx::query("INSERT INTO questions (question, content, tags) VALUES ($1, $2, $3) RETURNING *")
            .bind("What is the meaning of life?")
            .bind("42")
            .bind("philosophy")
            .fetch_one(&self.pool)
            .await?;

        let question = Question {
            id: row.get(0),
            title: row.get(1),
            content: row.get(2),
            tags: row.get(3),
        };

        Ok(question)
    }

    // Add more methods for other database operations...
}