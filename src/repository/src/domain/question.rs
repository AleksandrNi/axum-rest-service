use serde::{Deserialize, Serialize};
use crate::domain::from_pg_row::FromPgRow;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Serialize,Deserialize, Debug)]
pub struct Question {
    id: Option<i32>,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    pub fn new(id: Option::<i32>, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl Question {
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

impl FromPgRow for Question {
    fn from_pg_row(row: PgRow) -> Self {
        Question::new(
            Some::<i32>(row.get("id")),
            row.get("title"),
            row.get("content"),
            row.get("tags"),
        )
    }
}

