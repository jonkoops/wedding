use askama_axum::Template;
use axum::{extract::Query, routing::get, Router};
use serde::{Deserialize, Serialize};
use time::Duration;
use tokio::net::TcpListener;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));
    let app = Router::new()
        .route("/", get(index))
        .route("/rsvp", get(rsvp))
        .layer(session_layer);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

const RSVP_STATUS_KEY: &str = "rsvp";
const RSVP_PASSWORD: &str = "azula";

async fn rsvp(session: Session, query_params: Query<RsvpQueryParams>) -> RsvpTemplate {
    let rsvp_status: RsvpStatus = session
        .get(RSVP_STATUS_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    if !rsvp_status.needs_password {
        RsvpTemplate::default();
    }

    match query_params.0.code {
        Some(code) if code != RSVP_PASSWORD => {
            return RsvpTemplate {
                needs_password: true,
                wrong_password: true,
            };
        }
        Some(code) if code == RSVP_PASSWORD => {
            session
                .insert(
                    RSVP_STATUS_KEY,
                    RsvpStatus {
                        needs_password: false,
                    },
                )
                .await
                .unwrap();

            return RsvpTemplate {
                needs_password: false,
                ..Default::default()
            };
        }
        _ => {
            return RsvpTemplate {
                needs_password: true,
                ..Default::default()
            }
        }
    }
}

#[derive(Deserialize)]
struct RsvpQueryParams {
    code: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct RsvpStatus {
    needs_password: bool,
}

impl Default for RsvpStatus {
    fn default() -> Self {
        Self {
            needs_password: true,
        }
    }
}

#[derive(Default, Template)]
#[template(path = "rsvp.html")]
struct RsvpTemplate {
    needs_password: bool,
    wrong_password: bool,
}
