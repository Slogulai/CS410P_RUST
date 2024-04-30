//Source:: https://github.com/pdx-cs-rust-web/knock-knock/tree/main

use crate::Question;
use std::collections::HashMap;
//use serde::{Deserialize, Serialize};

pub struct Store {
    pub question_map: HashMap<String, Question>,
}

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
    /* May end up using these later
    pub fn get_questions(&self) -> HashMap<String, Question> {
        self.question_map.clone()
    }

    fn init(&mut self) -> &mut Store {
        let question = Question::new(
            "1".to_string(),
            "What is the capital of France?".to_string(),
            "Paris".to_string(),
            Some(vec!["geography".to_string()]),
        );
        self.add_question(question)
    }
    fn add_question(&mut self, question: Question) -> &mut Store {
        self.question_map.insert(question.id.clone(), question);
        self
    }
    */
}