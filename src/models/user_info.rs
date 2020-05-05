use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub date_created: NaiveDateTime,
    pub is_admin: bool,
    pub should_initialize: bool,
}