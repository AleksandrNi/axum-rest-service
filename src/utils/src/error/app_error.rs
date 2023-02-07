pub trait AppGenericError {
    fn get_message(&self) -> &str;
    fn get_code(&self) -> &'static str;
}