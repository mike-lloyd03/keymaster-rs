pub mod auth;
pub mod form_actions;
pub mod requests;

use chrono::NaiveDate;
use yew_router::prelude::{AnyHistory, History};
use yewdux::prelude::Dispatch;

use crate::{components::notifier::notify, routes::Route, types::Notification};

static DATE_FMT: &'static str = "%Y-%m-%d";

pub fn parse_date(date_string: String) -> NaiveDate {
    NaiveDate::parse_from_str(&date_string, DATE_FMT).unwrap_or_default()
}

pub fn format_date(date: NaiveDate) -> String {
    date.format(DATE_FMT).to_string()
}

pub fn handle_unauthorized(history: AnyHistory, notify_dispatch: Dispatch<Notification>) {
    history.push(Route::Login);
    notify(
        &notify_dispatch,
        "You must log in to access this page".into(),
        "error".into(),
    );
}
