use rocket_contrib::json::Json;
use rocket::http::Status;
use crate::lib_http::ApiResponse;
use crate::crypto::get_password_hash;
use crate::models::User;
use crate::providers::{UsersProvider, IUsersProvider};
use std::borrow::Cow;

//const ERR_INVALID_REQUEST: &'static str = "invalid_request";
//const ERR_ACCESS_DENIED: &'static str = "access_denied";
const ERR_UNAUTHORIZED: &'static str = "unauthorized";
//const ERR_GENERIC: &'static str = "server_error";

#[derive(Serialize, Deserialize)]
pub struct Credentials<'a> {
    user: &'a str,
    password: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse<'a> {
    token: Cow<'a, str>,
}

impl<'a> TokenResponse<'a> {
    fn new<S>(token: S) -> TokenResponse<'a>
        where S: Into<Cow<'a, str>>
    {
        TokenResponse { token: token.into() }
    }
}

#[post("/token", format = "json", data = "<request>")]
pub fn token(request: Json<Credentials>, provider: UsersProvider) -> ApiResponse<TokenResponse> {
    use crate::providers::prelude::*;

    let user: Option<User> = provider.get_user(request.user.to_owned());
    if user.is_none() {
        return ApiResponse::<TokenResponse>::err(ERR_UNAUTHORIZED, "", Status::Unauthorized);
    }

    let exist_user = user.unwrap();
    let password_hash: String = get_password_hash(request.password, &exist_user.salt);
    if password_hash != exist_user.password_hash {
        return ApiResponse::<TokenResponse>::err(ERR_UNAUTHORIZED, "", Status::Unauthorized);
    }

    ApiResponse::<TokenResponse>::ok(TokenResponse::new("TOKEN"))
}