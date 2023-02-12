use chrono::{DateTime, Utc};
use repository::domain::user::UserModel;

pub struct UserDto {
    pub (crate) id: Option<u32>,
    pub (crate) name: Option<String>,
    pub (crate) email: Option<String>,
    pub (crate) password: Option<String>,
    pub (crate) deleted_at: Option<DateTime<Utc>>,
}

impl UserDto {
    pub fn new(
        id: Option<u32>,
        name: Option<String>,
        email: Option<String>,
        password: Option<String>,
        deleted_at: Option<DateTime<Utc>>) -> UserDto {
        UserDto { id, name, email, password, deleted_at }
    }
}

impl UserDto {
    pub fn get_id(&self) -> &Option<u32> { &self.id }
    pub fn get_name(&self) -> &Option<String> { &self.name }
    pub fn get_email(&self) -> &Option<String> { &self.email }
    pub fn get_password(&self) -> &Option<String> { &self.password }
    pub fn get_deleted_at(&self) -> &Option<DateTime<Utc>> { &self.deleted_at }
}

impl From<UserDto> for UserModel {
    fn from(value: UserDto) -> Self {
        UserModel::new(value.id, value.name, value.email, value.password, value.deleted_at)
    }
}

impl From<UserModel> for UserDto {
    fn from(value: UserModel) -> Self {
        UserDto::new(value.get_id().to_owned(), value.get_name().to_owned(), value.get_email().to_owned(), value.get_password().to_owned(),  value.get_deleted_at().to_owned())
    }
}