use service::user::user_dto::UserDto;
use utils::dto::token_payload::TokenPayload;

pub struct UserPayload {
    user_dto: UserDto,
    token_payload: TokenPayload,
}

impl UserPayload {
    pub fn new(user_dto: UserDto, token_payload: TokenPayload) -> Self {
        UserPayload { user_dto, token_payload }
    }
}

impl UserPayload {
    pub fn get_user_dto(&self) -> &UserDto {
        &self.user_dto
    }
    pub fn get_token_payload(&self) -> &TokenPayload {
        &self.token_payload
    }
}