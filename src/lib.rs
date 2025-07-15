use chrono::{NaiveDate, Datelike, Duration, Local};

/// Parses a date string in MM/DD/YYYY format and returns a NaiveDate.
/// Returns `None` if the string is invalid.
fn parse_us_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s.trim(), "%m/%d/%Y").ok()
}

/// Returns the end of the month for the given date.
/// If input is `None`, uses today's date.
/// If input is a `&str`, tries to parse it as "MM/DD/YYYY".
pub fn eom<T: Into<DateInput>>(input: T) -> NaiveDate {
    match input.into() {
        DateInput::Date(date) => end_of_month(date),
        DateInput::String(s) => {
            parse_us_date(&s)
                .map(end_of_month)
                .unwrap_or_else(|| end_of_month(Local::now().naive_local().date()))
        }
        DateInput::None => end_of_month(Local::now().naive_local().date()),
    }
}

fn end_of_month(date: NaiveDate) -> NaiveDate {
    let year = date.year();
    let month = date.month();
    let first_day_next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .expect("Invalid date computation");
    first_day_next_month - Duration::days(1)
}

pub enum DateInput {
    Date(NaiveDate),
    String(String),
    None,
}

impl From<&str> for DateInput {
    fn from(s: &str) -> Self {
        DateInput::String(s.to_string())
    }
}

impl From<NaiveDate> for DateInput {
    fn from(date: NaiveDate) -> Self {
        DateInput::Date(date)
    }
}

impl From<Option<NaiveDate>> for DateInput {
    fn from(opt: Option<NaiveDate>) -> Self {
        match opt {
            Some(d) => DateInput::Date(d),
            None => DateInput::None,
        }
    }
}
