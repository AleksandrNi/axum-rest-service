use crate::error::app_error::{AppErrorBody, AppGenericError};

// body
fn general_error_body(err: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occured: {}", err), "appWebError001")
}

fn general_error_for_param_body(param: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occured for param: {}", param), "appWebError002")
}

fn general_error_for_param_value_body(param: String, value: String) -> AppErrorBody {
    AppErrorBody::new(format!("General error occured for param: {} value: {}", param, value), "appWebError003")
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
}