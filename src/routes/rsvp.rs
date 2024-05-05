use askama_axum::Template;
use axum::extract::Query;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

const RSVP_STATUS_KEY: &str = "rsvp";
const RSVP_PASSWORD: &str = "azula";

#[derive(Deserialize)]
pub struct RsvpQueryParams {
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
pub struct RsvpTemplate {
    needs_password: bool,
    wrong_password: bool,
}

pub async fn route_handler(session: Session, query_params: Query<RsvpQueryParams>) -> RsvpTemplate {
    let rsvp_status: RsvpStatus = session
        .get(RSVP_STATUS_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    if !rsvp_status.needs_password {
        return RsvpTemplate::default();
    }

    match query_params.0.code {
        Some(code) if code != RSVP_PASSWORD => RsvpTemplate {
            needs_password: true,
            wrong_password: true,
        },
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

            RsvpTemplate {
                needs_password: false,
                ..Default::default()
            }
        }
        _ => RsvpTemplate {
            needs_password: true,
            ..Default::default()
        },
    }
}
