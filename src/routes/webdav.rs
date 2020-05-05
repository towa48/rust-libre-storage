use super::guards::WebDavAuth;
use super::guards::prelude::*;

#[get("/")]
pub fn list(_auth: WebDavAuth) -> Result<String, &'static str> {
    Ok("user".to_string())
}