use crate::error::app_error::{AppGenericErrorTrait, AppErrorBody, AppGenericError};

// body
fn general_error_body(err: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occurred: '{}'", err), "appRepositoryError001")
}

fn general_error_for_param_body(param: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occurred for param: '{}'", param), "appRepositoryError002")
}

fn general_error_for_param_value_body(param: String, value: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occurred for param: '{}' value: '{}'", param, value), "appRepositoryError003")
}


pub struct AppRepositoryError;

impl AppRepositoryError {
    pub fn general_error(err: String) -> AppGenericError {
        AppGenericError::Repository(general_error_body(err))
    }

    pub fn general_error_for_param(param: String) -> AppGenericError {
        AppGenericError::Repository(general_error_for_param_body(param))
    }

    pub fn general_error_for_param_value(param: String, value: String) -> AppGenericError {
        AppGenericError::Repository(general_error_for_param_value_body(param, value))
    }
}