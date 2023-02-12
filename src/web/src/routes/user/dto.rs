use serde::{Deserialize, Serialize};
use service::user::user_dto::UserDto;

#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}


#[derive(Serialize)]
pub struct UserCreateResponse {
    id: u32,
    name: String,
    token: Option<String>,
}

impl UserCreateResponse {
    pub fn new(id: u32, name: String) -> UserCreateResponse {
        UserCreateResponse { id, name, token: None }
    }
}

impl UserCreateResponse {
    pub fn get_id(&self) -> u32 { self.id }
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_token(&self) -> &Option<String> { &self.token }
}

impl UserCreateResponse {
    pub fn set_token(&mut self, token: String) { self.token = Some(token) }
}

#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

impl From<UserCreateRequest> for UserDto {
    fn from(value: UserCreateRequest) -> Self {
        UserDto::new(None, Some(value.name), Some(value.email), Some(value.password), None)
    }
}

impl From<UserDto> for UserCreateResponse {
    fn from(value: UserDto) -> Self {
        UserCreateResponse::new(*value.get_id().as_ref().unwrap(),
                                value.get_name().as_ref().unwrap().to_owned())
    }
}

impl From<UserLoginRequest> for UserDto {
    fn from(value: UserLoginRequest) -> Self {
        UserDto::new(None, None, Some(value.email), Some(value.password), None)
    }
}