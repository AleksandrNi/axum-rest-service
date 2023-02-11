use sqlx::{Postgres, Transaction};
use sqlx::postgres::PgRow;
use utils::error::app_error::AppGenericError;
use utils::error::app_repository_error::AppRepositoryError;
use crate::domain::user::UserModel;

const QUERY_CREATE_USER: &str =
    "INSERT INTO users (username, email, password_hash, token) values ($1, $2, $3, $4) RETURNING *";

pub async fn user_create(tx: &mut Transaction<'static, Postgres>, user_model: UserModel) -> Result<UserModel, AppGenericError> {
    match sqlx::query(QUERY_CREATE_USER)
        .bind(user_model.name)
        .bind(user_model.email)
        .bind(user_model.password)
        .bind(user_model.token)
        .map(|row: PgRow| UserModel::from(row))
        .fetch_one(&mut *tx)
        // .fetch_one(connection)
        .await {
        Ok(data) => Ok(data),
        Err(err) => Err(AppRepositoryError::general_error(err.to_string()))
    }
}