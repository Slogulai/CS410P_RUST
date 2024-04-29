/*
use create::*;

#[derive(Debug)]
pub struct QuestionBase {
    file: File,
    questionmap: QuestionMap,
}

impl QuestionBase {
    pub fn new<P: AsRef<std::path::Path>> (
        db_path P,
        allow_empty: bool,
) -> Result<Self, std::io::Error> {
        let opened = File::option().read(true).write(true).open(&db_path);
        let mut file = match opened {
            Ok(f) => f,
            Err(e) => {
                if e.kind() != ErrorKind::NotFound || !allow_empty {
                    return Err(e);
                }
                let mut f = File::create_new(&db_path)?;
                let questionmap: QuestionMap = HashMap::new();
                let json = serde_json::to_string(&questionmap).unwrap();
                f.write_all(json.asbytes())?;
                f.sync_all()?;
                f.reqind()?;
                f
            }
        };
    }
}
*/

use crate::Question;
use std::collections::HashMap;
#[allow(unused)]
use serde::{Deserialize, Serialize};

pub struct Store {
    pub question_map: HashMap<String, Question>,
}

#[allow(unused)]
impl Store {
    pub fn new() -> Self {
        Self {
            question_map: Self::init(),
        }
    }
    pub fn init() -> HashMap<String, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json!")
    }
    /*
    fn init(&mut self) -> &mut Store {
        let question = Question::new(
            "1".to_string(),
            "What is the capital of France?".to_string(),
            "Paris".to_string(),
            Some(vec!["geography".to_string()]),
        );
        self.add_question(question)
    }
    */
    fn add_question(&mut self, question: Question) -> &mut Store {
        self.question_map.insert(question.id.clone(), question);
        self
    }
}