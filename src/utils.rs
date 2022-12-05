use chrono::{DateTime, Utc};

const DT_FORMAT: &str = "%B %e, %Y %l:%M %p";

pub fn get_fmt_datetime() -> String {
    let now: DateTime<Utc> = Utc::now();
    format!("{} UTC", now.format(&DT_FORMAT))
}