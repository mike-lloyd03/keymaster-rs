use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::string::String;
use yewdux::store::Store;

pub trait PrimaryKey {
    fn primary_key(&self) -> String;
}

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
}

impl PrimaryKey for Key {
    fn primary_key(&self) -> String {
        self.name.clone()
    }
}

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub can_login: bool,
    pub admin: bool,
}

impl PrimaryKey for User {
    fn primary_key(&self) -> String {
        self.username.clone()
    }
}

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub id: i64,
    pub user: String,
    pub key: String,
    pub date_out: NaiveDate,
    pub date_in: Option<NaiveDate>,
}

#[derive(Serialize, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Default, Store)]
pub struct SessionInfo {
    pub username: Option<String>,
    pub is_auth: bool,
    pub is_admin: bool,
    #[serde(skip)]
    pub fetched: bool,
}

#[derive(Clone, Default, PartialEq, Eq, Store)]
pub struct Notification {
    pub msg: Option<String>,
    pub lvl: Option<String>,
}
