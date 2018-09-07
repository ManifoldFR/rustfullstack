use schema::users;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Clone, Queryable, Serialize, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: NaiveDateTime
}

impl User {
    pub fn get_list(conn: &PgConnection) -> Vec<User> {
        use schema::users::dsl::*;

        let results = users.load::<User>(conn)
            .expect("Error loading posts");
        results
    }
}

