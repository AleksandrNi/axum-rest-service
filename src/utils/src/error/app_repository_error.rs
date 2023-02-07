use crate::error::app_error::AppGenericError;


const WEB_REPOSITORY_001: &str = "appRepositoryError001";
const WEB_REPOSITORY_002: &str = "appRepositoryError002";
const WEB_REPOSITORY_003: &str = "appRepositoryError003";

#[derive(Debug)]
pub struct AppRepositoryError  {
    message: String,
    code: &'static str,
}

impl AppRepositoryError {
    pub fn general_error(err: &str) -> Box<Self> {
        Box::new(AppRepositoryError { message: format!("General error occured: {}", err), code: "appRepositoryError001" })
    }
    pub fn general_error_for_param(param: &str) -> Box<Self> {
        Box::new(AppRepositoryError { message: format!("General error occured for param: {}", param), code: "appRepositoryError002" })
    }
    pub fn general_error_for_param_value(param: &str, value: &str) -> Box<Self> {
        Box::new(AppRepositoryError { message: format!("General error occured for param: {} value: {}", param, value), code: "appRepositoryError003" })
    }
}

impl AppGenericError for AppRepositoryError {
    fn get_message(& self) -> & str {
        &self.message[..]
    }

    fn get_code(&self) -> &'static str {
        self.code
    }
}