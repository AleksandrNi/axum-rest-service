// use std::path::Path;
// use sqlx::migrate::Migrator;
use utils::core::db::get_connection;
use tracing::info;

pub async fn run () {
    // // Read migrations from a local folder: ./migrations
    // let m = Migrator::new(Path::new("./migrations/")).await
    //     .unwrap_or_else(|err| panic!("Migrator is not created: {}", err));
    // m.run(pool).await
    //     .unwrap_or_else(|_| panic!("Migrations is not applied"));

    let pool = get_connection().await;
    sqlx::migrate!("./migrations/").run(pool).await.unwrap();
    info!("executed: initializing db migrations");
}