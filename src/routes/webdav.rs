use crate::utils::basic_auth::BasicAuth;

#[get("/")]
pub fn list(auth: BasicAuth) -> Result<String, &'static str> {
    if auth.username.is_none() {
        return Err("not_auth");
    }

    Ok(auth.username.unwrap())
}