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

    // Fetch issues on startup
    github_api::fetch_and_store_issues().await.expect("Failed to fetch issues on startup");
    tracing::info!("Initial issue fetch completed");

    // Start periodic refresh task
    let refresh_handle = tokio::spawn(async {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(20 * 60)).await;
            match github_api::fetch_and_store_issues().await {
                Ok(_) => tracing::info!("Periodic issue fetch completed"),
                Err(e) => tracing::error!("Periodic issue fetch failed: {:?}", e),
            }
        }
    });

    // Start web server
    server::start_server().await;

    refresh_handle.abort();
}
