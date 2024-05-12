use axum::{routing::get, Router};
use time::Duration;
use tower_http::services::ServeFile;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

use crate::routes;

#[derive(Clone)]
pub struct DbState {
    pub pool: sqlx::SqlitePool,
}

pub async fn create_router(pool: sqlx::Pool<sqlx::Sqlite>) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(10)));

    Router::new()
        .route("/", get(routes::index::route_handler))
        .route("/rsvp", get(routes::rsvp::route_handler))
        .route_service(
            "/vendor/alpinejs.js",
            ServeFile::new("node_modules/alpinejs/dist/module.esm.js"),
        )
        .layer(session_layer)
        .with_state(DbState { pool })
}
