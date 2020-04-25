use super::lib::establish_connection;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub date_created: NaiveDateTime,
    pub salt: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub should_initialize: bool,
}

pub fn get_user(login: String) -> Option<User> {
    use super::super::schema::users::dsl::*;

    let connection = establish_connection();
    users
        .filter(username.eq(login))
        .first::<User>(&connection)
        .optional()
        .unwrap()
}