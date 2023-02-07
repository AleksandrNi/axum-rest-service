use std::ops::Deref;
use utils::error::app_error::AppGenericError;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use utils::error::app_repository_error::AppRepositoryError;
use utils::error::app_service_error::AppServiceError;

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

pub fn prepare_response<T>(result: Result<T, Box<dyn AppGenericError>>) -> Result<Json<T>, AppResponseError> {
    match result {
        Ok(data) => Ok(Json(data)),
        Err(boxErr) => match boxErr.deref() {
            AppRepositoryError => Err(AppResponseError::new(boxErr.get_code(), boxErr.get_message())),
            AppServiceError => Err(AppResponseError::new(boxErr.get_code(), boxErr.get_message())),
        }
    }
}