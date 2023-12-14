use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};

const SECS_FOR_DAY: u128 = 24 * 60 * 60;
const SECS_FOR_WEEK: u128 = SECS_FOR_DAY * 7;

fn to_datetime_str_from_secs(secs: i64) -> String {
    let datetime = DateTime::<Utc>::from_timestamp(secs, 0).expect("back to the future");
    datetime.format("%Y-%m-%dT%H:%M:%S").to_string()
}

fn main() {
    println!("Hello, world!");

    let now = SystemTime::now();
    println!("SystemTime::now: {:?}", now);
    let unixtime = now.duration_since(UNIX_EPOCH).expect("back to the future");
    println!("unixtime = {}", unixtime.as_nanos());
    let now_secs = unixtime.as_nanos() / (1000 * 1000000);
    println!("now: {}", now_secs);
    println!("now (std): {}", to_datetime_str_from_secs(now_secs as i64));

    let rounded_to_week = now_secs / SECS_FOR_WEEK * SECS_FOR_WEEK;
    println!("to week: {}", rounded_to_week);
    println!(
        "to week (std): {}",
        to_datetime_str_from_secs(rounded_to_week as i64)
    );
}
