use rocket_contrib::json::Json;
use crate::lib_http::ApiResult;
use crate::crypto::{create_password_hash, PasswordResult};

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
pub struct TokenResponse<'b> {
    token: &'b str,
}

impl<'b> TokenResponse<'b> {
    fn new(token: &str) -> TokenResponse {
        TokenResponse { token: token }
    }
}

#[post("/token", format = "json", data = "<request>")]
pub fn token(request: Json<Credentials>) -> ApiResult<TokenResponse> {
    let res: PasswordResult = create_password_hash(&request.password);
    let response = TokenResponse::new(&res.password);
    Ok(Json(response))
}