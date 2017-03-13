extern crate chrono;
use chrono::{DateTime, Duration, TimeZone};

pub fn after<T: TimeZone>(start_date: DateTime<T>) -> DateTime<T> {
    let gigasecond = Duration::seconds(1000000000);
    start_date + gigasecond
}
