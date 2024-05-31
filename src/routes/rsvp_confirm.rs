use askama_axum::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Guest {
    first_name: String,
    last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reservation {
    attending: bool,
    email: String,
    // guests: Vec<Guest>,
}

pub async fn route_handler(body: String) -> impl IntoResponse {
    let reservation: Reservation = serde_qs::from_str(&body).unwrap();
    println!("Parsed reservation: {:?}", reservation)
}
