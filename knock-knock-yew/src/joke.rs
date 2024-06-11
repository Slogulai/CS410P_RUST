use crate::*;

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionModel {
    pub id: String,
    pub question: String,
    pub answer: String,
    pub tags: String,
}

impl QuestionModel {
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

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionProps {
    pub question: QuestionModel,
}

#[function_component(Question)]
pub fn question(question: &QuestionProps) -> Html {
    let question = &question.question;
    html! { <>
        <div class="question">
            <span class="teller">{"Question: "}</span><br/>
            <span class="tellee">{"Answer: "}</span><br/>
            <span class="teller">{question.question.clone()}</span><br/>
            <span class="teller">{question.answer.clone()}</span>
        </div>
    </> }
}
