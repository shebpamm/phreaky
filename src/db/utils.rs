use color_eyre::eyre::Result;
use chrono::{Utc, DateTime, NaiveDateTime, TimeZone};

pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>> {
    let naive = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")?;
    Ok(Utc.from_utc_datetime(&naive))
}
