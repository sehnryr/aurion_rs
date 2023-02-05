use chrono::{DateTime, Datelike, Local, TimeZone, Utc};

/// Returns the default start date for the current school year
///
/// The default start date is the first day of August of the current year
pub fn school_start() -> DateTime<Utc> {
    let now = Local::now();
    let mut start = Local
        .with_ymd_and_hms(now.year(), 8, 1, 0, 0, 0)
        .single()
        .unwrap();
    // If the current date is before start, then substract 1 year from start
    if now < start {
        start = start.with_year(start.year() - 1).unwrap();
    }
    start.with_timezone(&Utc)
}

/// Returns the default end date for the current school year
///
/// The default end date is the last day of July of the next year
pub fn school_end() -> DateTime<Utc> {
    let now = Local::now();
    let mut end = Local
        .with_ymd_and_hms(now.year(), 7, 31, 23, 59, 59)
        .single()
        .unwrap();
    // If the current date is after end, then add 1 year to end
    if now > end {
        end = end.with_year(end.year() + 1).unwrap();
    }
    end.with_timezone(&Utc)
}
