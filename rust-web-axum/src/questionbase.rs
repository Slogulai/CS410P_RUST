use crate::*;

pub async fn question_db() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("question_data_base").expect("DATABASE_URL must be set");
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

//Functions to add
//check exists
//insert question
//update question
//delete question

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

    pub async fn check_exists(&self, id: i32) -> Result<bool, sqlx::Error> {
        let row: Option<PgRow> = sqlx::query("SELECT * FROM questions WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.is_some())
    }

    pub async fn insert(&self, question: &Question) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO questions (question, content, tags) VALUES ($1, $2, $3)")
            .bind(&question.title)
            .bind(&question.content)
            .bind(&question.tags)
            .execute(&self.pool)
            .await?;

        Ok(())
    }


    // Add more methods for other database operations...
}
impl Deref for MyDatabase {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}