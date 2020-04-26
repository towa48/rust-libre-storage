use rocket::Request;
use rocket::http::Status;
use rocket::request::FromRequest;
use crate::providers::users::{get_user,UserInfo};
use crate::crypto::get_password_hash;

const ERR_NO_BASIC: &'static str = "No basic authorization";
const ERR_WRONG_CREDENTIALS: &'static str = "Wrong credentials";

pub struct WebDavAuth {
    pub user: UserInfo,
}

impl<'a, 'r> FromRequest<'a, 'r> for WebDavAuth {
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
                let username = strigfy.get(0).map(ToOwned::to_owned);
                let password = strigfy.get(1).map(ToOwned::to_owned);
                if username.is_none() || password.is_none() {
                    return rocket::Outcome::Failure((Status::Unauthorized, ERR_WRONG_CREDENTIALS));
                }
                let user = get_user(username.unwrap());
                if user.is_none() {
                    return rocket::Outcome::Failure((Status::Unauthorized, ERR_WRONG_CREDENTIALS));
                }
                let exist_user = user.unwrap();
                let password_hash = get_password_hash(&password.unwrap(), &exist_user.salt);
                if password_hash != exist_user.password_hash {
                    return rocket::Outcome::Failure((Status::Unauthorized, ERR_WRONG_CREDENTIALS));
                }
                rocket::Outcome::Success(WebDavAuth {
                    user: exist_user.to_info(),
                })
            },
            _ => rocket::Outcome::Failure((Status::Unauthorized, ERR_NO_BASIC))
        }
    }
}