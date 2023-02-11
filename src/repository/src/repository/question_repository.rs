use crate::domain::question::QuestionModel;
use sqlx::postgres::PgRow;
use sqlx::{Postgres, Row, Transaction};
use utils::core::db::Tx;
use utils::error::app_error::AppGenericError;
use utils::error::app_repository_error::AppRepositoryError;

// use dom
const QUERY_SELECT_QUESTIONS_WITH_LIMIT_AND_OFFSET: &str =
    "SELECT * from questions LIMIT $1 OFFSET $2";

const QUERY_CREATE_QUESTION: &str =
    "INSERT INTO questions (title, content, tags) values ($1, $2, $3) RETURNING *";

const QUERY_SELECT_QUESTION_BY_ID: &str =
    "SELECT * FROM questions WHERE id = $1";

const QUERY_DELETE_QUESTION_BY_ID: &str =
    "DELETE FROM questions WHERE id = $1";

const QUERY_PATCH_QUESTION: &str =
    "UPDATE questions \
    SET title = $1, content = $2, tags = $3 \
    WHERE id = $4 RETURNING *";

pub async fn get_questions(tx: &mut Transaction<'static, Postgres>) -> Result<Vec<QuestionModel>, AppGenericError> {
    // let connection = get_connection().await;
    let limit = 100;
    let offset = 0;
    match sqlx::query(QUERY_SELECT_QUESTIONS_WITH_LIMIT_AND_OFFSET)
        .bind(limit)
        .bind(offset)
        .map(|row: PgRow| QuestionModel::from(row))
        .fetch_all(&mut *tx)
        // .fetch_all(&mut tx)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}


pub async fn post_question(
    tx: &mut Transaction<'static, Postgres>, question: QuestionModel) -> Result<QuestionModel, AppGenericError> {
    // let connection = get_connection().await;

    match sqlx::query(QUERY_CREATE_QUESTION)
        .bind(question.get_title())
        .bind(question.get_content())
        .bind(question.get_tags())
        .map(|row: PgRow| QuestionModel::from(row))
        .fetch_one(&mut *tx)
        // .fetch_one(connection)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}

pub async fn get_question_by_id(tx: &mut Transaction<'static, Postgres>, id: i32) -> Result<QuestionModel, AppGenericError> {
    // let connection = get_connection().await;

    match sqlx::query(QUERY_SELECT_QUESTION_BY_ID)
        .bind(id)
        .map(|row: PgRow| QuestionModel::from(row))
        .fetch_one(&mut *tx)
        // .fetch_one(connection)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}

pub async fn delete_question_by_id(tx: &mut Transaction<'static, Postgres>, id: i32) -> Result<bool, AppGenericError> {
    match sqlx::query(QUERY_DELETE_QUESTION_BY_ID)
        .bind(id)
        .execute(&mut *tx)
        // .fetch_one(connection)
        .await {
        Ok(_) => Ok(true),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}

pub async fn patch_question(
    tx: &mut Transaction<'static, Postgres>, question: QuestionModel) -> Result<QuestionModel, AppGenericError> {
    // let connection = get_connection().await;

    match sqlx::query(QUERY_PATCH_QUESTION)
        .bind(question.get_title())
        .bind(question.get_content())
        .bind(question.get_tags())
        .bind(question.get_id())
        .map(|row: PgRow| QuestionModel::from(row))
        .fetch_one(&mut *tx)
        // .fetch_one(connection)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}