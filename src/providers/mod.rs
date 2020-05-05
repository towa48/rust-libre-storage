pub mod prelude;

//use diesel::SqliteConnection;
use rocket_contrib::database;
use crate::models::User;

/*
 * DbConnection
 */

#[database("DATABASE_URL")]
pub struct DbConnection(diesel::SqliteConnection);

/*
 * UsersProvider
 */

pub struct UsersProvider {
    conn: DbConnection,
}

impl UsersProvider {
    pub fn new(conn: DbConnection) -> Self {
        Self {conn: conn}
    }
}

pub trait IUsersProvider {
    fn get_user(&self, login: String) -> Option<User>;
}