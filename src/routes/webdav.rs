use super::guards::WebDavAuth;

#[get("/")]
pub fn list(_auth: WebDavAuth) -> Result<String, &'static str> {
    Ok("user".to_string())
}