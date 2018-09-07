use schema::users;
use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Queryable, Serialize, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Clone, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String
}

impl User {
    pub fn get_list(conn: &PgConnection) -> Vec<User> {
        use schema::users::dsl::*;

        let results = users.load::<User>(conn)
            .expect("Error loading posts");
        results
    }

    pub fn create(conn: &PgConnection, new_user: NewUser) -> User {
        use schema::users;
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .expect("Error adding new user.")
    }
}

