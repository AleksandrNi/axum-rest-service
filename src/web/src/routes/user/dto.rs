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
    token: String,
}

impl UserCreateResponse {
    pub fn new(id: u32, name: String, token: String) -> UserCreateResponse {
        UserCreateResponse { id, name, token }
    }
}

#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

impl From<UserCreateRequest> for UserDto {
    fn from(value: UserCreateRequest) -> Self {
        UserDto::new(None, Some(value.name), Some(value.email), Some(value.password), None, None)
    }
}

impl From<UserDto> for UserCreateResponse {
    fn from(value: UserDto) -> Self {
        UserCreateResponse::new(*value.get_id().as_ref().unwrap(),
                                value.get_name().as_ref().unwrap().to_owned(),
                                value.get_token().as_ref().unwrap().to_owned())
    }
}