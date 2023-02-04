use service::question;
use repository::domain::question::Question;

use utils::error::app_repository_error::AppRepositoryError;
use utils::error::app_service_error::AppServiceError;
use utils::error::app_web_error::AppWebError;
use utils::error::app_error::AppErrorData;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;


// pub async fn get_questions() -> Result<Question, AppError> {
//     // let result = question::get_questions().await.unwrap();
//     Ok(Question {
//         title: "title".to_string(),
//         content: "content".to_string(),
//     })
// }

pub async fn get_questions() -> Result<Json<Vec<Question>>, AppError> {
    let result = question::get_questions().await.unwrap();
    Ok(Json(result))
}

#[derive(Debug)]
pub enum AppError {
    Repository(AppRepositoryError),
    Service(AppServiceError),
    Web(AppWebError),
}

// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//
//         let (status, message) = match self {
//             AppError::Repository(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.get_message().to_owned()),
//             AppError::Service(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.get_message().to_owned()),
//             AppError::Web(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.get_message().to_owned()),
//         };
//         let body = Json(json!({
//             "error": message,
//         }));
//
//         (status, body).into_response()
//     }
// }

#[derive(Serialize)]
pub struct AppErrorBody {
    // pub code: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Repository(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.get_message().to_owned()),
            AppError::Service(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.get_message().to_owned()),
            AppError::Web(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.get_message().to_owned()),
        };
        (
            status,
            Json(AppErrorBody {
                // code: &self.code,
                message: message,
            })
        ).into_response() as Response
    }
}