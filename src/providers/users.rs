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

impl User {
    pub fn to_info(&self) -> UserInfo {
        UserInfo { id: self.id, username: self.username.to_owned(), date_created: self.date_created, is_admin: self.is_admin, should_initialize: self.should_initialize }
    }
}

#[derive(Queryable)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub date_created: NaiveDateTime,
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