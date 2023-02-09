use utils::error::app_error::AppGenericErrorTrait;
use utils::error::app_error::AppGenericError;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use serde_json::to_string;
use utils::error::app_repository_error::AppRepositoryError;
use utils::error::app_service_error::AppServiceError;
use utils::error::app_web_error::AppWebError;

pub struct AppResponseError {
    pub statusCode: StatusCode,
    pub message: String,
    pub code: String,
}

impl AppResponseError {
    pub fn new(statusCode: StatusCode, code: String, message: String) -> Self {
        AppResponseError { statusCode, code, message }
    }
}

#[derive(Serialize, Debug)]
pub struct AppResponseErrorBody {
    pub code: String,
    pub message: String,
}

impl AppResponseErrorBody {
    pub fn fromAppResponseError(err: AppResponseError) -> AppResponseErrorBody {
        AppResponseErrorBody { message: err.message, code: err.code }
    }
}

impl IntoResponse for AppResponseError {
    fn into_response(self) -> Response {
        (self.statusCode,
         Json(AppResponseErrorBody::fromAppResponseError(self))
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