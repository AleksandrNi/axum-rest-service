pub trait AppGenericErrorTrait {
    fn get_message(&self) -> String;
    fn get_code(&self) -> String;
}

#[derive(Debug)]
pub struct AppErrorBody {
    message: String,
    code: String,
}

impl AppErrorBody {
    pub fn new (message: String, code: &str) -> Self {
        AppErrorBody {message, code: code.to_owned()}
    }
}

impl AppGenericErrorTrait for AppErrorBody {
    fn get_message(&self) -> String {
        self.message.to_owned()
    }
    fn get_code(&self) -> String {
        self.code.to_owned()
    }
}

#[derive(Debug)]
pub enum AppGenericError {
    Repository(AppErrorBody),
    Service(AppErrorBody),
    Web(AppErrorBody),
}