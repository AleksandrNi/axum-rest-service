use sqlx::Row;
use sqlx::postgres::PgRow;
use crate::domain::from_pg_row::FromPgRow;

pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
pub struct QuestionId(String);


impl Question {
    pub fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>
    ) -> Self {
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
            row.get("tags")
        )
    }
}