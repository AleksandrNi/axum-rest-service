use crate::error::app_error::{AppErrorBody, AppGenericError};

// body
fn general_error_body(err: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occurred: '{}'", err), "appWebError001")
}

fn general_error_for_param_body(param: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occurred for param: '{}'", param), "appWebError002")
}

fn general_error_for_param_value_body(param: String, value: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occurred for param: '{}' value: '{}'", param, value), "appWebError003")
}
fn failed_getting_db_connection(err: String) -> AppErrorBody {
    AppErrorBody::new(format!("Failed getting db connection: {}", err), "appWebError004")
}


pub struct AppWebError;

impl AppWebError {
    pub fn general_error(err: String) -> AppGenericError {
        AppGenericError::Web(general_error_body(err))
    }

    pub fn general_error_for_param(param: String) -> AppGenericError {
        AppGenericError::Web(general_error_for_param_body(param))
    }

    pub fn general_error_for_param_value(param: String, value: String) -> AppGenericError {
        AppGenericError::Web(general_error_for_param_value_body(param, value))
    }

    pub fn failed_getting_db_connection(err: String) -> AppGenericError {
        AppGenericError::Web(failed_getting_db_connection(err))
    }
}