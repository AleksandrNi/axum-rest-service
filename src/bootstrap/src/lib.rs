mod lifecyrcle;

use axum::extract::State;
use api::state::index::SharedData;
use lifecyrcle::init;
// use web::

pub async fn run () -> Result<(), ()> {
    let state:State<SharedData> = init::run().await;
    // start::run().await;
    println!("Hello world");
    Ok(())
}