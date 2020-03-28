use rocket_contrib::json::Json;

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
pub fn token(request: Json<Credentials>) -> Json<TokenResponse> {
    // TODO: should we copy string here?
    let response = TokenResponse::new(&request.user.clone());
    Json(response)
}