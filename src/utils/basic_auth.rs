use rocket::Request;
use rocket::http::Status;
use rocket::request::FromRequest;

pub struct BasicAuth {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = &'static str;

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        let auth_pair: Vec<&str> = request.headers().get_one("Authorization").unwrap_or("")
            .split_whitespace()
            .collect();
        match (auth_pair.get(0), auth_pair.get(1)) {
            (Some(&"Basic"), Some(&token)) => {
                let bytes = base64::decode(&token).unwrap_or(vec![]);
                let strigfy: Vec<String> = std::str::from_utf8(&bytes)
                    .unwrap_or("")
                    .split(":")
                    .map(ToOwned::to_owned)
                    .collect();
                rocket::Outcome::Success(BasicAuth {
                    username: strigfy.get(0).map(ToOwned::to_owned),
                    password: strigfy.get(1).map(ToOwned::to_owned)
                })
            },
            _ => rocket::Outcome::Failure((Status::Unauthorized, "No basic authorization"))
        }
    }
}