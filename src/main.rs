mod routes;

use axum::{routing::get, Router};
use time::Duration;
use tokio::net::TcpListener;
use tower_http::services::ServeFile;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(10)));
    let app = Router::new()
        .route("/", get(routes::index::route_handler))
        .route("/rsvp", get(routes::rsvp::route_handler))
        .route_service(
            "/vendor/alpinejs.js",
            ServeFile::new("node_modules/alpinejs/dist/module.esm.js"),
        )
        .layer(session_layer);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
