use serde::{Deserialize, Serialize};
use service::question::question_dto::QuestionDto;

#[derive(Deserialize)]
pub struct QuestionRequest {
    #[serde(default, with = "::serde_with::rust::double_option")]
    id: Option<Option<i32>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    title: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    content: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    tags: Option<Option<Vec<String>>>,
}

impl QuestionRequest {
    pub fn get_id(&self) -> &Option<Option<i32>> {&self.id}
    pub fn get_title(&self) -> &Option<Option<String>> {&self.title}
    pub fn get_content(&self) -> &Option<Option<String>>{&self.content}
    pub fn get_tags(&self) -> &Option<Option<Vec<String>>>{&self.tags}
}

#[derive(Serialize)]
pub struct QuestionResponse {
    id: i32,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl QuestionResponse {
    pub fn new (id: i32, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        QuestionResponse{id, title, content, tags}
    }
}

impl From<QuestionRequest> for QuestionDto {
    fn from(value: QuestionRequest) -> Self {
        QuestionDto::new(value.id, value.title, value.content, value.tags)
    }
}

impl From<QuestionDto> for QuestionResponse {
    fn from(value: QuestionDto) -> Self {
        QuestionResponse::new(value.get_id().as_ref().unwrap().unwrap(),
                              value.get_title().as_ref().unwrap().as_ref().unwrap().to_owned(),
                              value.get_content().as_ref().unwrap().as_ref().unwrap().to_owned(),
                              value.get_tags().as_ref().unwrap().to_owned(),
        )
    }
}
