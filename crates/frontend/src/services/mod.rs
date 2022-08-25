pub mod auth;
pub mod form_actions;
pub mod requests;

use chrono::NaiveDate;

static DATE_FMT: &'static str = "%Y-%m-%d";

pub fn parse_date(date_string: String) -> NaiveDate {
    NaiveDate::parse_from_str(&date_string, DATE_FMT).unwrap_or_default()
}

pub fn format_date(date: NaiveDate) -> String {
    date.format(DATE_FMT).to_string()
}
