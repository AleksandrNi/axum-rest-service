use crate::error::app_error::AppGenericError;


const WEB_ERROR_001: &str = "appWebError001";
const WEB_ERROR_002: &str = "appWebError002";
const WEB_ERROR_003: &str = "appWebError003";

#[derive(Debug)]
pub struct AppWebError  {
    message: String,
    code: &'static str,
}
impl AppWebError{
    pub fn general_error(err: &str) -> Box<Self> {
        Box::new(AppWebError { message: format!("General error occured: {}", err), code: &WEB_ERROR_001 })
    }
    pub fn general_error_for_param(param: &str) -> Box<Self> {
        Box::new(AppWebError { message: format!("General error occured for param: {}", param), code: &WEB_ERROR_002 })
    }
    pub fn general_error_for_param_value(param: &str, value: &str) -> Box<Self> {
        Box::new(AppWebError { message: format!("General error occured for param: {} value: {}", param, value), code: &WEB_ERROR_003 })
    }
}

impl AppGenericError for AppWebError {
    fn get_message(& self) ->  & str {
        &self.message[..]
    }

    fn get_code(&self) -> &'static str {
        &self.code
    }
}