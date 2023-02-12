use utils::error::app_error::AppGenericErrorTrait;
use utils::error::app_error::AppGenericError;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

pub struct AppResponseError {
    pub status_code: StatusCode,
    pub message: String,
    pub code: String,
}

impl AppResponseError {
    pub fn new(status_code: StatusCode, code: String, message: String) -> Self {
        AppResponseError { status_code, code, message }
    }
}

#[derive(Serialize, Debug)]
pub struct AppResponseErrorBody {
    pub code: String,
    pub message: String,
}

impl AppResponseErrorBody {
    pub fn from_app_response_error(err: AppResponseError) -> AppResponseErrorBody {
        AppResponseErrorBody { message: err.message, code: err.code }
    }
}

impl IntoResponse for AppResponseError {
    fn into_response(self) -> Response {
        (self.status_code,
         Json(AppResponseErrorBody::from_app_response_error(self))
        ).into_response() as Response
    }
}

pub fn prepare_response<T>(result: Result<T, AppGenericError>) -> Result<Json<T>, AppResponseError> {
    match result {
        Ok(data) => Ok(Json(data)),
        Err(generic_error) => match generic_error {
            AppGenericError::Repository(err) => Err(AppResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, err.get_code(), err.get_message())),
            AppGenericError::Service(err) => Err(AppResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, err.get_code(), err.get_message())),
            AppGenericError::Web(err) =>
                Err(AppResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, err.get_code(), err.get_message())),
        }
    }
}