pub mod auth;
pub mod form_actions;
pub mod requests;

use chrono::NaiveDate;

use crate::types::User;

static DATE_FMT: &'static str = "%Y-%m-%d";

pub fn parse_date(date_string: String) -> NaiveDate {
    NaiveDate::parse_from_str(&date_string, DATE_FMT).unwrap_or_default()
}

pub fn format_date(date: NaiveDate) -> String {
    date.format(DATE_FMT).to_string()
}

/// Returns the given user's display name. If it isn't found, it will return the user's username.
pub fn get_display_name(users: &Vec<User>, username: String) -> String {
    users
        .iter()
        .filter(|u| u.username == username.clone())
        .map(|u| u.display_name.clone().unwrap_or(username.clone()))
        .next()
        .unwrap_or(username)
}
