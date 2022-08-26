pub mod auth;
pub mod form_actions;
pub mod requests;

use chrono::NaiveDate;
use yew_router::prelude::{AnyHistory, History};

use crate::{components::notifier::notify_error, routes::Route};

static DATE_FMT: &'static str = "%Y-%m-%d";

pub fn parse_date(date_string: String) -> NaiveDate {
    NaiveDate::parse_from_str(&date_string, DATE_FMT).unwrap_or_default()
}

pub fn format_date(date: NaiveDate) -> String {
    date.format(DATE_FMT).to_string()
}

pub fn handle_unauthorized(history: AnyHistory) {
    history.push(Route::Login);
    notify_error("You must log in to access this page");
}
