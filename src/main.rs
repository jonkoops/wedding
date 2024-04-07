use askama_axum::Template;
use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/rsvp", get(rsvp));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> IndexTemplate {
    IndexTemplate {}
}

async fn rsvp() -> RsvpTemplate {
    RsvpTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "rsvp.html")]
struct RsvpTemplate {}
