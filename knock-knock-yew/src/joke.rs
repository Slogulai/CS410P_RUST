use crate::*;

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionStruct {
    pub id: String,
    pub question: String,
    pub answer: String,
    pub tags: Option<HashSet<String>>,
    pub source: Option<String>,
}

impl QuestionStruct {
    pub async fn get_question(key: Option<String>) -> Msg {
        let request = match &key {
            None => "http://localhost:3000/api/v1/question".to_string(),
            Some(ref key) => format!("http://localhost:3000/api/v1/question/{}", key,),
        };
        let response = http::Request::get(&request).send().await;
        match response {
            Err(e) => Msg::GotQuestion(Err(e)),
            Ok(data) => Msg::GotQuestion(data.json().await),
        }
    }
}
pub fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionProps {
    pub question: QuestionStruct,
}

#[function_component(Question)]
pub fn question(question: &QuestionProps) -> Html {
    let question = &question.question;
    html! { <>
        <div class="question">
            <span class="teller">{"Knock-Knock!"}</span><br/>
            <span class="tellee">{"Who's there?"}</span><br/>
            <span class="teller">{question.question.clone()}</span><br/>
            <span class="tellee">{format!("{} who?", &question.question)}</span><br/>
            <span class="teller">{question.answer.clone()}</span>
        </div>
        <span class="annotation">
            {format!("[id: {}", &question.id)}
            if let Some(ref tags) = question.tags {
                {format!("; tags: {}", &format_tags(tags))}
            }
            if let Some(ref source) = question.source {
                {format!("; source: {}", source)}
            }
            {"]"}
        </span>
    </> }
}
