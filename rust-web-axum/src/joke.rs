use crate::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Joke {
    pub id: JokeId,
    pub whos_there: String,
    pub answer_who: String,
    pub tags: Option<HashSet<String>>,
    pub source: Option<String>
}

impl Joke {
    pub fn new(id: JokeId, whos_there: String, answer_who: String, tags: Option<HashSet<String>>, source: Option<String>) -> Self {
        Self {
            id,
            whos_there,
            answer_who,
            tags,
            source
        }
    }
}