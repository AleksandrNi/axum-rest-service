use crate::domain::from_pg_row::FromPgRow;
use crate::domain::question::Question;
use sqlx::postgres::PgRow;
use sqlx::{Postgres, Row, Transaction};
use utils::error::app_repository_error::AppRepositoryError;

// use dom
const QUERY_SELECT_QUESTIONS_WITH_LIMIT_AND_OFFSET: &str =
    "SELECT * from questions LIMIT $1 OFFSET $2";

pub async fn get_questions(tx: &mut Transaction<'static, Postgres>) -> Result<Vec<Question>, AppRepositoryError> {
    let limit = 100;
    let offset = 0;
    match sqlx::query(QUERY_SELECT_QUESTIONS_WITH_LIMIT_AND_OFFSET)
        .bind(limit)
        .bind(offset)
        .map(|row: PgRow| Question::from_pg_row(row))
        .fetch_all(&mut *tx)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(
            AppRepositoryError::general_error(&err.to_string()[..])
        )
    }
}