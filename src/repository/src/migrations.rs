use tracing::info;
use utils::core::db::get_connection;

pub async fn run() {
    info!("execute : initializing db migrations ...");
    // // Read migrations from a local folder: ./migrations
    // let m = Migrator::new(Path::new("./migrations/")).await
    //     .unwrap_or_else(|err| panic!("Migrator is not created: {}", err));
    // m.run(pool).await
    //     .unwrap_or_else(|_| panic!("Migrations is not applied"));
    let pool = get_connection().await;
    sqlx::migrate!("./migrations/")
        .run(pool)
        .await
        .unwrap_or_else(|_| panic!("Migrations is not applied"));
    info!("executed: initializing db migrations");
}
