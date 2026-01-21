use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
    Json,
};
use serde::Deserialize;
use tower_http::services::ServeDir;
use std::net::SocketAddr;

use crate::models::PaginatedResponse;
use crate::db;

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<i64>,
    min_stars: Option<i64>,
}

pub async fn start_server() {
    let app = Router::new()
        .route("/api/issues", get(get_issues))
        .nest_service("/", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 27412));
    tracing::info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

const MAX_PAGE: i64 = 4;

async fn get_issues(
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, StatusCode> {
    // Limit page to MAX_PAGE (4)
    let page = params.page.unwrap_or(1).max(1).min(MAX_PAGE);
    let min_stars = params.min_stars.unwrap_or(50);
    let per_page = 30i64;

    match db::get_paginated_issues(page, per_page, min_stars) {
        Ok((issues, total_count)) => {
            let total_pages = (total_count + per_page - 1) / per_page;
            
            let response = PaginatedResponse {
                issues,
                total_count,
                page,
                per_page,
                total_pages,
            };
            
            Ok(Json(response))
        }
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
