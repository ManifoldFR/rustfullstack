use schema::users;
use diesel::prelude::*;
use chrono::{DateTime,Local};

#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub created_at: DateTime<Local>
}