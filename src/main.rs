mod db;
mod github_api;
mod models;
mod server;

use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // Initialize database
    db::init_db().expect("Failed to initialize database");
    tracing::info!("Database initialized");

    // Start periodic refresh task
    let refresh_handle = tokio::spawn(async {
        loop {
            match github_api::fetch_and_store_issues().await {
                Ok(_) => tracing::info!("Periodic issue fetch completed"),
                Err(e) => tracing::error!("Periodic issue fetch failed: {:?}", e),
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(20 * 60)).await;
        }
    });

    // Start web server
    server::start_server().await;

    refresh_handle.abort();
}
