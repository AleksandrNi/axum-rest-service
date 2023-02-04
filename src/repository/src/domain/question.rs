use serde::{Deserialize, Serialize};
use crate::domain::from_pg_row::FromPgRow;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Serialize,Deserialize)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionId(i32);

impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromPgRow for Question {
    fn from_pg_row(row: PgRow) -> Self {
        Question::new(
            QuestionId(row.get("id")),
            row.get("title"),
            row.get("content"),
            row.get("tags"),
        )
    }
}
