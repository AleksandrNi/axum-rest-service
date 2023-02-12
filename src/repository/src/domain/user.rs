use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserModel {
    pub(crate) id: Option<u32>,
    pub(crate) name: Option<String>,
    pub(crate) email: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) deleted_at: Option<DateTime<Utc>>,
}

impl UserModel {
    pub fn new(
        id: Option<u32>,
        name: Option<String>,
        email: Option<String>,
        password: Option<String>,
        deleted_at: Option<DateTime<Utc>>) -> UserModel {
        UserModel { id, name, email, password, deleted_at }
    }
}

impl UserModel {
    pub fn get_id(&self) -> &Option<u32> { &self.id }
    pub fn get_name(&self) -> &Option<String> { &self.name }
    pub fn get_email(&self) -> &Option<String> { &self.email }
    pub fn get_password(&self) -> &Option<String> { &self.password }
    pub fn get_deleted_at(&self) -> &Option<DateTime<Utc>> { &self.deleted_at }
}

impl From<PgRow> for UserModel {
    fn from(value: PgRow) -> Self {
        let id: i32 = value.get("id");
        UserModel::new(
            Some::<u32>(id as u32),
            Some(value.get("username")),
            Some(value.get("email")),
            Some(value.get("password_hash")),
            value.get("deleted_at"),
        )
    }
}
