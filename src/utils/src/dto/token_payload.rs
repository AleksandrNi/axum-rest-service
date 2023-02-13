pub struct TokenPayload {
    user_id: u32,
}

impl TokenPayload {
    pub fn new (user_id: u32) -> Self {
        TokenPayload { user_id }
    }
}

impl TokenPayload {
    pub fn get_user_id(&self) -> u32 {
        self.user_id
    }
}