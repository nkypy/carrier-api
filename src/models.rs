use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorReply<'a> {
    pub error_code: usize,
    pub error_message: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub uid: usize,
    pub exp: usize,
}

pub struct Store {
    pub db: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoReply {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthReply {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<InfoReply>>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub is_banned: bool,
    pub is_deleted: bool,
    pub create_at: SystemTime,
    pub updated_at: SystemTime,
}

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let conn = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    conn.clone()
}
