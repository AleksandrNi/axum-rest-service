use crate::error::app_error::AppGenericError;


const REPOSITORY_001: &str = "appServiceError001";
const REPOSITORY_002: &str = "appServiceError002";
const REPOSITORY_003: &str = "appServiceError003";

#[derive(Debug)]
pub struct AppServiceError {
    message: String,
    code: &'static str,
}

impl <'a> AppServiceError {
    pub fn general_error(err: &str) -> Box<Self> {
        Box::new(AppServiceError { message: format!("General error occured: {}", err), code:  REPOSITORY_001})
    }
    pub fn general_error_for_param(param: &str) -> Box<Self> {
        Box::new(AppServiceError { message: format!("General error occured for param: {}", param), code: REPOSITORY_002 })
    }
    pub fn general_error_for_param_value(param: &str, value: &str) -> Box<Self> {
        Box::new(AppServiceError { message: format!("General error occured for param: {} value: {}", param, value), code: REPOSITORY_003 })
    }
}

impl AppGenericError for AppServiceError {
    fn get_message(& self) -> & str {
        &self.message[..]
    }

    fn get_code(&self) -> &'static str {
        self.code
    }
}