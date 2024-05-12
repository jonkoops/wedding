use askama_axum::Template;
use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::{app::DbState, db};

const RSVP_STATUS_KEY: &str = "rsvp";

#[derive(Deserialize)]
pub struct RsvpQueryParams {
    code: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct RsvpStatus {
    needs_code: bool,
}

impl Default for RsvpStatus {
    fn default() -> Self {
        Self { needs_code: true }
    }
}

#[derive(Default, Template)]
#[template(path = "rsvp.html")]
pub struct RsvpTemplate {
    needs_code: bool,
    wrong_code: bool,
}

pub async fn route_handler(
    State(db): State<DbState>,
    session: Session,
    query_params: Query<RsvpQueryParams>,
) -> RsvpTemplate {
    let rsvp_status: RsvpStatus = session
        .get(RSVP_STATUS_KEY)
        .await
        .unwrap()
        .unwrap_or_default();

    if !rsvp_status.needs_code {
        return RsvpTemplate::default();
    }

    let Some(code) = query_params.0.code else {
        return RsvpTemplate {
            needs_code: true,
            wrong_code: false,
        };
    };

    let invitation = db::get_invitation_by_code(&db.pool, &code).await;

    if invitation.is_some() {
        session
            .insert(RSVP_STATUS_KEY, RsvpStatus { needs_code: false })
            .await
            .unwrap();

        return RsvpTemplate {
            needs_code: false,
            wrong_code: false,
        };
    }

    RsvpTemplate {
        needs_code: true,
        wrong_code: true,
    }
}
