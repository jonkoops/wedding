use sqlx::{sqlite::SqliteConnectOptions, FromRow, SqlitePool};
use std::{convert::From, env, str::FromStr};

#[derive(FromRow)]
pub struct Invitation {
    pub id: i64,
    pub status: InvitationStatus,
    pub code: String,
    pub email: Option<String>,
}

pub enum InvitationStatus {
    Pending,
    Accepted,
    Rejected,
}

impl From<String> for InvitationStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "Pending" => InvitationStatus::Pending,
            "Accepted" => InvitationStatus::Accepted,
            "Rejected" => InvitationStatus::Rejected,
            _ => panic!("Invalid invitation status"),
        }
    }
}

#[derive(FromRow)]
pub struct Guest {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub invitation_id: i64,
}

pub async fn create_and_connect() -> SqlitePool {
    let url = env::var("DATABASE_URL").unwrap();
    let options = SqliteConnectOptions::from_str(&url)
        .unwrap()
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await.unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    pool
}

pub async fn get_invitation_by_code(
    pool: &SqlitePool,
    code: &str,
) -> Result<Option<Invitation>, sqlx::Error> {
    sqlx::query_as!(
        Invitation,
        r#"SELECT id, status, code, email FROM invitations WHERE code = ?"#,
        code
    )
    .fetch_optional(pool)
    .await
}
