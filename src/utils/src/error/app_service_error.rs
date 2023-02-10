use crate::error::app_error::{AppErrorBody, AppGenericError};

// body
fn general_error_body(err: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occured: '{}'", err), "appServiceError001")
}

fn general_error_for_param_body(param: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occured for param: '{}'", param), "appServiceError002")
}

fn general_error_for_param_value_body(param: String, value: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occured for param: '{}' value: '{}'", param, value), "appServiceError003")
}

fn entity_does_not_exists_for_param_value(param: String, value: String) -> AppErrorBody {
    AppErrorBody::new(format!("Entity does not exist for param: '{}' value: '{}'", param, value), "appServiceError004")
}

fn parameter_must_be_provided(param: String) -> AppErrorBody {
    AppErrorBody::new(format!("Parameter must be provided: '{}'", param), "appServiceError005")
}

pub struct AppServiceError;

impl AppServiceError {
    pub fn general_error(err: String) -> AppGenericError {
        AppGenericError::Service(general_error_body(err))
    }

    pub fn general_error_for_param(param: String) -> AppGenericError {
        AppGenericError::Service(general_error_for_param_body(param))
    }

    pub fn general_error_for_param_value(param: String, value: String) -> AppGenericError {
        AppGenericError::Service(general_error_for_param_value_body(param, value))
    }
    pub fn entity_does_not_exists_for_param_value(param: String, value: String) -> AppGenericError {
        AppGenericError::Service(entity_does_not_exists_for_param_value(param, value))
    }
    pub fn parameter_must_be_provided(param: String) -> AppGenericError {
        AppGenericError::Service(parameter_must_be_provided(param))

    }
}