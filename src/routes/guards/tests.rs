#[cfg(test)]
mod test {
    use rocket::local::Client;
    use rocket::http::Status;
    use rocket::request::FromRequest;

    use crate::routes::guards::WebDavAuth;
    use crate::routes::guards::prelude::*;

    #[test]
    fn should_return_unauthorized_when_no_header() {
        let rocket = rocket::ignite();
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.get("/test");

        let outcome = WebDavAuth::from_request(req.inner());

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
