mod app;
mod db;
mod routes;

use axum::serve;
use dotenv::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = db::create_and_connect().await;
    let app = app::create_router(pool).await;
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, app).await.unwrap();
}
