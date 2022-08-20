use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Default, Store)]
pub struct UserInfo {
    pub username: Option<String>,
    pub is_auth: bool,
    pub is_admin: bool,
}

#[derive(PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Key {
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
}

#[derive(PartialEq, Eq, Default, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub can_login: bool,
    pub admin: bool,
}

#[derive(PartialEq, Eq, Default, Clone)]
pub struct Assignment {
    pub id: i64,
    pub user: String,
    pub key: String,
    pub date_out: NaiveDate,
    pub date_in: Option<NaiveDate>,
}
