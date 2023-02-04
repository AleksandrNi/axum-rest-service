use crate::error::app_repository_error::AppRepositoryError;
use crate::error::app_service_error::AppServiceError;
use crate::error::app_web_error::AppWebError;

use axum::{
    async_trait,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub trait AppErrorData<'a> {
    fn get_message(&'a self) -> &'a str;

    fn get_code(&self) -> &'static str;
}



// pub enum AppError {
//     /// Something went wrong when calling the user repo.
//     UserRepo(UserRepoError),
//     /// Something went wrong when calling the user serice.
//     // UserService(UserServiceError)
// }
// impl From<UserRepoError> for AppError {
//     fn from(inner: UserRepoError) -> Self {
//         AppError::UserRepo(inner)
//     }
// }
//
// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         let (status, error_message) = match self {
//             AppError::UserRepo(UserRepoError::NotFound) => {
//                 (StatusCode::NOT_FOUND, "User not found")
//             }
//             AppError::UserRepo(UserRepoError::InvalidUsername) => {
//                 (StatusCode::UNPROCESSABLE_ENTITY, "Invalid username")
//             }
//         };
//
//         let body = Json(json!({
//             "error": error_message,
//         }));
//
//         (status, body).into_response()
//     }
// }