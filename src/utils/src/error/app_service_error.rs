use crate::error::app_error::AppErrorData;

const REPOSITORY_001: &str = "appServiceError001";
const REPOSITORY_002: &str = "appServiceError002";
const REPOSITORY_003: &str = "appServiceError003";

#[derive(Debug)]
pub struct AppServiceError {
    message: String,
    code: &'static str,
}

impl <'a> AppServiceError {
    pub fn general_error(err: &str) -> Self {
        AppServiceError { message: format!("General error occured: {}", err), code:  REPOSITORY_001}
    }
    pub fn general_error_for_param(param: &str) -> Self {
        AppServiceError { message: format!("General error occured for param: {}", param), code: REPOSITORY_002 }
    }
    pub fn general_error_for_param_value(param: &str, value: &str) -> Self {
        AppServiceError { message: format!("General error occured for param: {} value: {}", param, value), code: REPOSITORY_003 }
    }
}

impl <'a> AppErrorData<'a> for AppServiceError {
    fn get_message(&'a self) -> &'a str {
        &self.message[..]
    }

    fn get_code(&self) -> &'static str {
        self.code
    }
}