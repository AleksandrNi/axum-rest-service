use crate::domain::from_pg_row::FromPgRow;
use crate::domain::question::Question;
use sqlx::postgres::PgRow;
use sqlx::{Postgres, Row, Transaction};
use utils::core::db::get_connection;
use utils::error::app_error::AppGenericError;
use utils::error::app_repository_error::AppRepositoryError;

// use dom
const QUERY_SELECT_QUESTIONS_WITH_LIMIT_AND_OFFSET: &str =
    "SELECT * from questions LIMIT $1 OFFSET $2";

const QUERY_CREATE_QUESTION: &str =
    "INSERT INTO questions (title, content, tags) values ($1, $2, $3) RETURNING *";

const QUERY_SELECT_QUESTION: &str =
    "SELECT * FROM questions WHERE id = $1";

pub async fn get_questions(tx: &mut Transaction<'static, Postgres>) -> Result<Vec<Question>, AppGenericError> {
    let connection = get_connection().await;
    let limit = 100;
    let offset = 0;
    match sqlx::query(QUERY_SELECT_QUESTIONS_WITH_LIMIT_AND_OFFSET)
        .bind(limit)
        .bind(offset)
        .map(|row: PgRow| Question::from_pg_row(row))
        // .fetch_all(&mut *tx)
        .fetch_all(connection)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}


pub async fn post_question(
    tx: &mut Transaction<'static, Postgres>, question: Question) -> Result<Question, AppGenericError> {
    let connection = get_connection().await;

    match sqlx::query(QUERY_CREATE_QUESTION)
        .bind(question.get_title())
        .bind(question.get_content())
        .bind(question.get_tags())
        .map(|row: PgRow| Question::from_pg_row(row))
        // .fetch_one(&mut *tx)
        .fetch_one(connection)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}

pub async fn get_question_by_id(tx: &mut Transaction<'static, Postgres>, id: i32) -> Result<Question, AppGenericError> {
    let connection = get_connection().await;

    match sqlx::query(QUERY_SELECT_QUESTION)
        .bind(id)
        .map(|row: PgRow| Question::from_pg_row(row))
        // .fetch_one(&mut *tx)
        .fetch_one(connection)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}