use api::state::Context;

struct StateUser {
    email: String,
    token: Option<String>,
}

async fn create_context() -> Context {
    Context { user: None }
}
