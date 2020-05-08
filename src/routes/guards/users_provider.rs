use rocket::{Outcome, Request};
use rocket::request::{FromRequest};
use crate::providers::{UsersProvider,DbConnection};

/*
 * UsersProvider
 */

impl<'a, 'r> FromRequest<'a, 'r> for UsersProvider {
    type Error = &'static str;

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        let conn_outcome = request.guard::<DbConnection>();
        match conn_outcome {
            Outcome::Success(conn) => {
                let users_provider = UsersProvider::new(conn); // TODO: use pool
                Outcome::Success(users_provider)
            },
            _ => panic!("Expected DbConnection outcome!")
        }
    }
}