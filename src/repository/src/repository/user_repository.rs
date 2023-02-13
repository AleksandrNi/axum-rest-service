use sqlx::{Postgres, Transaction};
use sqlx::postgres::PgRow;
use utils::error::app_error::AppGenericError;
use utils::error::app_repository_error::AppRepositoryError;
use crate::domain::user::UserModel;

const QUERY_USER_CREATE: &str =
    "INSERT INTO users (username, email, password_hash) values ($1, $2, $3) RETURNING *";

const QUERY_USER_LOAD_BY_EMAIL: &str =
    "SELECT * FROM users WHERE email = $1 AND deleted_at is NULL";

const QUERY_USER_LOAD_BY_ID: &str =
    "SELECT * FROM users WHERE id = $1 AND deleted_at is NULL";

const QUERY_USER_UPDATE: &str =
    "UPDATE users SET username = $1, email = $2, password_hash = $3 WHERE id = $4 AND deleted_at is NULL RETURNING *";

pub async fn user_create(tx: &mut Transaction<'static, Postgres>, user_model: UserModel) -> Result<UserModel, AppGenericError> {
    match sqlx::query(QUERY_USER_CREATE)
        .bind(user_model.name)
        .bind(user_model.email)
        .bind(user_model.password)
        .map(|row: PgRow| UserModel::from(row))
        .fetch_one(&mut *tx)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}

pub async fn user_load_by_email(tx: &mut Transaction<'static, Postgres>, user_model: UserModel) -> Result<UserModel, AppGenericError> {
    match sqlx::query(QUERY_USER_LOAD_BY_EMAIL)
        .bind(user_model.email)
        .map(|row: PgRow| UserModel::from(row))
        .fetch_one(&mut *tx)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}
pub async fn user_load_by_id(tx: &mut Transaction<'static, Postgres>, user_model: UserModel) -> Result<UserModel, AppGenericError> {
    match sqlx::query(QUERY_USER_LOAD_BY_ID)
        .bind(user_model.id.unwrap() as i32)
        .map(|row: PgRow| UserModel::from(row))
        .fetch_one(&mut *tx)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}

pub async fn user_update(tx: &mut Transaction<'static, Postgres>, user_model: UserModel) -> Result<UserModel, AppGenericError> {
    match sqlx::query(QUERY_USER_UPDATE)
        .bind(user_model.name)
        .bind(user_model.email)
        .bind(user_model.password)
        .bind(user_model.id.unwrap() as i32)
        .map(|row: PgRow| UserModel::from(row))
        .fetch_one(&mut *tx)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}