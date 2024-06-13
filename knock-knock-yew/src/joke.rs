use crate::*;



#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionModel {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub tags: String,
}
#[derive(serde::Deserialize)]
struct Response {
    data: Data,
}

#[derive(serde::Deserialize)]
struct Data {
    question: QuestionModel,
}

impl QuestionModel {
    pub async fn get_question(key: Option<String>) -> Msg {
        let request = match &key {
            None => "http://localhost:8000/question/1".to_string(),
            Some(ref key) => format!("http://localhost:8000/question/{}", key,),
        };
        let response = http::Request::get(&request).send().await;
        match response {
            Err(e) => Msg::GotQuestion(Err(e)),
            Ok(data) => {
                let response: Result<Response, _> = data.json().await;
                match response {
                    Ok(response) => Msg::GotQuestion(Ok(response.data.question)),
                    Err(e) => Msg::GotQuestion(Err(e)),
                }
            }
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
            <span class="teller">{question.question.clone()}</span><br/>
            <span class="tellee">{"Answer: "}</span><br/>
            <span class="teller">{question.answer.clone()}</span><br/>
        </div>
    </> }
}
