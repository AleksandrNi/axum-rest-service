use serde::{Deserialize, Serialize};
use repository::domain::question::QuestionModel;

#[derive(Serialize, Deserialize)]
pub struct QuestionDto {
    id: Option<Option<i32>>,
    title: Option<Option<String>>,
    content: Option<Option<String>>,
    tags: Option<Option<Vec<String>>>,
}


impl QuestionDto {
    pub fn new(id: Option<Option<i32>>, title: Option<Option<String>>, content: Option<Option<String>>, tags: Option<Option<Vec<String>>>) -> Self {
        QuestionDto { id, title, content, tags }
    }
}

impl From<QuestionModel> for QuestionDto {
    fn from(value: QuestionModel) -> Self {
        QuestionDto::new(
            Some(*value.get_id()),
            Some(Some(value.get_title().to_owned())),
            Some(Some(value.get_content().to_owned())),
            Some(value.get_tags().to_owned()))
    }
}

impl From<QuestionDto> for QuestionModel {
    fn from(value: QuestionDto) -> Self {
        QuestionModel::new(
            unwrap_option(value.id),
            unwrap_option(value.title).unwrap(),
            unwrap_option(value.content).unwrap(),
            unwrap_option(value.tags),
        )
    }
}

fn unwrap_option<T>(value: Option<Option<T>>) -> Option<T> {
    match value {
        Some(v) => v,
        None => None
    }
}

impl QuestionDto {
    pub fn get_id(&self) -> &Option<Option<i32>> {
        &self.id
    }
    pub fn get_title(&self) -> &Option<Option<String>> {
        &self.title
    }
    pub fn get_content(&self) -> &Option<Option<String>> {
        &self.content
    }
    pub fn get_tags(&self) -> &Option<Option<Vec<String>>> {
        &self.tags
    }
}