use serde::{Deserialize, Serialize};
use crate::domain::from_pg_row::FromPgRow;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Serialize,Deserialize, Debug)]
pub struct QuestionModel {
    id: Option<i32>,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl QuestionModel {
    pub fn new(id: Option::<i32>, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        QuestionModel {
            id,
            title,
            content,
            tags,
        }
    }
}

impl QuestionModel {
    pub fn get_id(&self) -> &Option<i32> { &self.id }
    pub fn get_title(&self) -> &str {
        &self.title[..]
    }
    pub fn get_content(&self) -> &str {
        &self.content[..]
    }
    pub fn get_tags(&self) -> &Option<Vec<String>> {
        &self.tags
    }
 }

impl QuestionModel {
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = Some(tags);
    }
}

impl FromPgRow for QuestionModel {
    fn from_pg_row(row: PgRow) -> Self {
        QuestionModel::new(
            Some::<i32>(row.get("id")),
            row.get("title"),
            row.get("content"),
            row.get("tags"),
        )
    }
}

