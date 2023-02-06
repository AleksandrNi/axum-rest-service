use utils::error::app_error::{AppErrorData, AppGenericError};
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct AppResponseError {
    pub code: &'static str,
    pub message: String,
    // Repository(AppRepositoryError),
    // Service(AppServiceError),
    // Web(AppWebError),
}

impl AppResponseError {
    pub fn new(code: &'static str, message: &str) -> Self {
        AppResponseError {code, message: message.to_owned()}
    }
}

impl IntoResponse for AppResponseError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR,
         Json(self)
        ).into_response() as Response
    }
}

pub fn prepare_response<T>(result: Result<T, AppGenericError>) -> Result<Json<T>, AppResponseError> {
    match result {
        Ok(data) => Ok(Json(data)),
        Err(err) => match err {
            AppGenericError::Repository(e) => Err(AppResponseError::new(e.get_code(), e.get_message())),
            AppGenericError::Service(e) => Err(AppResponseError::new(e.get_code(), e.get_message()))
        }
    }
}