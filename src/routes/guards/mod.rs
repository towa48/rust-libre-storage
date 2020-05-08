pub mod tests;
pub mod prelude;
pub mod users_provider;
pub mod webdav_auth;

use crate::models::UserInfo;

pub struct WebDavAuth {
    pub user: UserInfo,
}