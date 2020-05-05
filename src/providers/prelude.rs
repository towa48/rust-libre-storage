use diesel::prelude::*;
use crate::models::{User};
use crate::providers::{UsersProvider,IUsersProvider};

impl IUsersProvider for UsersProvider {
    fn get_user(&self, login: String) -> Option<User> {
        use super::super::schema::users::dsl::*;

        users
            .filter(username.eq(login))
            .first::<User>(&*self.conn)
            .optional()
            .unwrap()
    }
}