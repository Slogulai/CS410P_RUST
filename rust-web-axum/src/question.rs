
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
#[derive(Deserialize, Serialize, Clone)]
pub struct QuestionId(String);
impl FromStr for QuestionId {
    type Err = Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No ID provided!")),
        }
    }
}
impl std::fmt::Debug for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
*/

#[allow(unused)]
impl Question {
    pub fn new(id: String, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
    pub fn unwrap(self) -> (String, String, String, Option<Vec<String>>) {
        (self.id, self.title, self.content, self.tags)
    }
    /*
    //Rust isnt liking this function
    fn update_title(&self, new_title: String) -> Self {
        Question::new(self.id, new_title, self.content, self.tags)
    }
    */
}


impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{:?}",
            self.id.0, self.title, self.content, self.tags
        )
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

//~~~~~~DB STUFF~~~~~~~~
//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/

//#[allow(unused)]
pub type DB = Arc<Mutex<Vec<Question>>>;
#[allow(unused)]
pub fn question_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
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
