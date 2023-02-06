use utils::error::app_repository_error::AppRepositoryError;
use utils::error::app_service_error::AppServiceError;
use utils::error::app_web_error::AppWebError;
use utils::error::app_error::{AppErrorData, AppGenericError};
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Repository(AppRepositoryError),
    Service(AppServiceError),
    // Web(AppWebError),
}

#[derive(Serialize)]
pub struct AppErrorBody {
    pub code: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::Repository(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.get_code(), err.get_message().to_owned()),
            AppError::Service(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.get_code(), err.get_message().to_owned()),
            // AppError::Web(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.get_code(), err.get_message().to_owned()),
        };
        (
            status,
            Json(AppErrorBody {
                code: code.to_string(),
                message: message,
            })
        ).into_response() as Response
    }
}

pub fn prepareResponse<T>(result: Result<T,AppGenericError>) -> Result<Json<T>, AppError> {
    match  result {
        Ok(data) => Ok(Json(data)),
        Err(err) => match err {
            AppGenericError::Repository(e) => Err(AppError::Repository(e)),
            AppGenericError::Service(e) => Err(AppError::Service(e))
        }
    }
}