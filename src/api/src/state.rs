use sqlx::types::Uuid;

pub struct ContextUser {
    name: String,
    id: Uuid,
    token: String,
}

pub struct Context {
    pub user: Option<ContextUser>,
}
