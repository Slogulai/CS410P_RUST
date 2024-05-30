use crate::*;

pub async fn question_db() -> Result<MyDatabase, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;
    Ok(questionbase::MyDatabase { pool })
}

pub async fn set_database(pool: &MySqlPool) -> Result<(), sqlx::Error> {
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
    pool: MySqlPool,
}

//Functions to add
//update question
//delete question
/*
impl MyDatabase {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = MySqlPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    //Retrieve
    pub async fn get_questions(&self, limit: i64, offset: i64) -> Result<Vec<Question>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Question>("SELECT * FROM questions ORDER BY id LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;


        Ok(question)
    }

    pub async fn get_question(&self, id: i32) -> Result<Question, sqlx::Error> {
        let row: MySqlRow = sqlx::query("SELECT * FROM questions WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(rows)
    }

    pub async fn check_exists(&self, id: i32) -> Result<bool, sqlx::Error> {
        let row: Option<MySqlRow> = sqlx::query("SELECT * FROM questions WHERE id = $1")
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
*/
impl Deref for MyDatabase {
    type Target = MySqlPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}