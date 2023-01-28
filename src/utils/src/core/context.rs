use api::state::Context;

async fn create_context() -> Context {
    Context {
        user: None,
    }
}