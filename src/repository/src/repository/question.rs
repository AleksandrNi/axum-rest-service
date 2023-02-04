use sqlx::{Postgres, Row, Transaction};
use sqlx::postgres::PgRow;
use crate::domain::question::Question;
use crate::domain::from_pg_row::FromPgRow;

// use dom
const QUERY_SELECT_QUESTIONS_WITH_LIMIT_AND_OFFSET: &str = "SELECT * from questions LIMIT $1 OFFSET $2";


pub async fn get_questions(tx: &mut Transaction<'static, Postgres>) -> Vec<Question> {
    sqlx::query(QUERY_SELECT_QUESTIONS_WITH_LIMIT_AND_OFFSET)
        // .bind(limit)
        // .bind(offset)
        .map(|row: PgRow| Question::from_pg_row(row))
        .fetch_all(&mut *tx)
        .await.unwrap()
}