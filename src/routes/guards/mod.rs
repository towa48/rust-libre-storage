pub mod prelude;
pub mod tests;

use crate::models::UserInfo;

pub struct WebDavAuth {
    pub user: UserInfo,
}