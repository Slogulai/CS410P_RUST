//Source:: https://github.com/pdx-cs-rust-web/knock-knock/tree/main
//use anyhow::Error as BoxError;

use crate::*;
//~~~~~~QUESTIONS STUFF~~~~~~~~
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct Question {
    #[schema(example = "1")]
    pub id: String,
    #[schema(example = "What is your name?")]
    pub question: String,
    #[schema(example = "My name is John Doe")]
    pub answer: String,
    #[schema(example = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}
impl Question {
    pub fn new(
        id: &str, 
        question: &str, 
        answer: &str, 
        tags: Option<Vec<String>>
    ) -> Self {
        let id = id.into();
        let question = question.into();
        let answer = answer.into();
        let tags: Option<HashSet<String>> = if tags.is_empty() {
            None
        } else {
            Some(tags.iter().copied().map(String::from).collect())
        };
        Self {
            id,
            question,
            answer,
            tags,
        }
    }
}

pub fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vev<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

impl From<&Question> for String {
    fn from (question: &Question) -> Self {
        let mut text: String = "What is your favorite dog?\n".into();
        text += "I like borzoids!\n";
        text += &format!("{}.\n", question.question);  
        text += &format!("{}\n", question.answer);
        text += "\n";

        let mut annote: Vec<String> = vec![format!("id: {}", question.id)];
        if let Some(tags) = &question.tags {
            annote.push(format!("tags: {}", format_tags(tags)));
        }
        let annote = annote.join("; ");
        text += &format!("[{}]\n," annote);
        text
    }
}

impl IntoResponse for &Question {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response()
    }
}






/* 

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{:?}",
            self.id, self.question, self.answer, self.tags
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
    pub question: Option<String>,
    pub answer: Option<String>,
    pub tags: Option<Vec<String>>,
}
*/