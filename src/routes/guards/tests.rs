#[cfg(test)]
mod test {
    use chrono::NaiveDateTime;
    use rocket::local::Client;
    use rocket::http::Status;
    use crate::routes::guards::webdav_auth::get_webdav_auth;
    use crate::providers::IUsersProvider;
    use crate::models::User;
    use crate::crypto::create_password_hash;

    const TEST_LOGIN: &'static str = "admin";
    const TEST_PASSWORD: &'static str = "admin";

    struct TestUsersProvider {
        username: String,
        salt_hash: String,
        password_hash: String,
    }

    impl TestUsersProvider {
        fn new(username: &'static str, password: &'static str) -> Self {
            let result = create_password_hash(password);
            Self { username: username.to_owned(), salt_hash: result.salt.into_owned(), password_hash: result.password.into_owned() }
        }
    }

    impl IUsersProvider for TestUsersProvider {
        fn get_user(&self, login: String) -> Option<User> {
            if login == TEST_LOGIN {
                let dt: NaiveDateTime = chrono::Utc::now().naive_utc();
                return Some(User { id: 1, username: self.username.to_owned(), salt: self.salt_hash.to_owned(), password_hash: self.password_hash.to_owned(), date_created: dt, is_admin: false, should_initialize: false });
            }

            None
        }
    }

    #[test]
    fn should_return_unauthorized_when_no_header() {
        let rocket = rocket::ignite();
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.get("/test");

        let outcome = get_webdav_auth(req.inner(), TestUsersProvider::new(TEST_LOGIN, TEST_PASSWORD));

        assert_eq!(outcome.is_success(), false);
        assert_eq!(outcome.failed(), Some((Status::Unauthorized, "No basic authorization")));
    }
}

//#[test]
//fn should_return_unauthorized_when_user_notfound() {
    // Append authorization header.
    //req.add_header(Header::new("Authorization", ""));
    //h_headers.append_raw("Authorization".to_string(), "Basic dXNlcjp1c2Vy".as_bytes().into());
//}

//#[test]
//fn should_return_unauthorized_when_password_incorrect() {
    // Append authorization header.
    //req.add_header(Header::new("Authorization", ""));
    //h_headers.append_raw("Authorization".to_string(), "Basic YWRtaW46MTIz".as_bytes().into());
//}

//#[test]
//fn should_return_user_when_password_is_correct() {
    // Append authorization header.
    //req.add_header(Header::new("Authorization", ""));
    //h_headers.append_raw("Authorization".to_string(), "Basic YWRtaW46YWRtaW4=".as_bytes().into());
//}
