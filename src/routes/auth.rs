use rocket_contrib::json::Json;
use crate::lib_http::ApiResult;
use crate::crypto::{create_password_hash, PasswordResult};
use std::borrow::Cow;

//const ERR_INVALID_REQUEST: &'static str = "invalid_request";
//const ERR_ACCESS_DENIED: &'static str = "access_denied";
const ERR_UNAUTHORIZED: &'static str = "unauthorized";
const ERR_GENERIC: &'static str = "server_error";

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
pub fn token(request: Json<Credentials>) -> ApiResult<TokenResponse> {
    let res: PasswordResult = create_password_hash(&request.password);
    println!("{}", res.salt);
    let response = TokenResponse::new(res.password);
    Ok(Json(response))
}