use chrono::{Datelike, NaiveDate};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    // Middle day of the year is always July 2
    let date = NaiveDate::from_ymd_opt(year as i32, 7, 2).unwrap();
    let weekday = date.weekday();

    // If this is a leap year, return None
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        return None;
    }

    // ELse, return the weekday
    Some(weekday)
}
