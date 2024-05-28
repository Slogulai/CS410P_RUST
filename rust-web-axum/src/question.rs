//use anyhow::Error as BoxError;

use crate::*;
//~~~~~~QUESTIONS STUFF~~~~~~~~
#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(sqlx::FromRow)]
pub struct Question {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl From<serde_json::Value> for Question {
    fn from(item: serde_json::Value) -> Self {
        let tags_value = item["tags"].as_array();
        let tags = tags_value.map(|array| array.iter().map(|x| x.as_str().unwrap().to_string()).collect());

        Question {
            id: item["id"].as_i64().unwrap() as i32,
            title: item["title"].as_str().unwrap().to_string(),
            content: item["content"].as_str().unwrap().to_string(),
            tags,
        }
    }
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

//~~~~~~DB STUFF~~~~~~~~
//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/

#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateQuestionSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}
