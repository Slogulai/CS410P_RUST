//use anyhow::Error as BoxError;

use crate::*;
//~~~~~~QUESTIONS STUFF~~~~~~~~
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
/*
async fn return_error(r: Rejection) -> Result<impl IntoResponse, Infallible> {
    let (code, message) = if r.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found")
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    };
    Ok((code, message).into_response())
}
*/
impl Question {
    pub fn new(id: String, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
    /*
    pub fn unwrap(self) -> (String, String, String, Option<Vec<String>>) {
        (self.id, self.title, self.content, self.tags)
    }
    //Rust isnt liking this function
    fn update_title(&self, new_title: String) -> Self {
        Question::new(self.id, new_title, self.content, self.tags)
    }
    */
}

impl From<serde_json::Value> for Question {
    fn from(item: serde_json::Value) -> Self {
        let tags_value = item["tags"].as_array();
        let tags = tags_value.map(|array| array.iter().map(|x| x.as_str().unwrap().to_string()).collect());

        Question {
            id: item["id"].as_str().unwrap().to_string(),
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

pub type DB = Arc<Mutex<HashMap<String, Question>>>;
pub fn question_db() -> DB {
    Arc::new(Mutex::new(HashMap::new()))
}
#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateQuestionSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}
