use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};

const SECS_FOR_DAY: u128 = 24 * 60 * 60;
const SECS_FOR_WEEK: u128 = SECS_FOR_DAY * 7;

fn to_datetime_str_from_secs(secs: i64) -> String {
    let datetime = DateTime::<Utc>::from_timestamp(secs, 0).expect("back to the future");
    datetime.format("%Y-%m-%dT%H:%M:%S").to_string()
}

fn get_terms(
    now_secs: u128,
) -> (
    u128, // near_term
    u128, // diff days to near_term
    u128, // next_term
    u128, // diff days to next_term
) {
    let current_day = now_secs / SECS_FOR_DAY * SECS_FOR_DAY;
    let rounded_epoch = current_day / SECS_FOR_WEEK * SECS_FOR_WEEK;
    let fri = rounded_epoch + SECS_FOR_DAY;
    let (near_term, next_term) = if current_day - rounded_epoch < 6 {
        (fri + SECS_FOR_WEEK * 4, fri + SECS_FOR_WEEK * 5)
    } else {
        (fri + SECS_FOR_WEEK * 5, fri + SECS_FOR_WEEK * 6)
    };
    (
        near_term,
        (near_term - now_secs) / SECS_FOR_DAY,
        next_term,
        (next_term - now_secs) / SECS_FOR_DAY,
    )
}

fn check_term(now_secs: u128) {
    println!(">>>> now: {}", to_datetime_str_from_secs(now_secs as i64));

    let (near, days_to_near, next, days_to_next) = get_terms(now_secs);
    println!(
        "near: {} ({})",
        to_datetime_str_from_secs(near as i64),
        near
    );
    println!("days to near: {}", days_to_near);
    println!(
        "next: {} ({})",
        to_datetime_str_from_secs(next as i64),
        next
    );
    println!("days to next: {}", days_to_next);
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

    check_term(now_secs);
    check_term(now_secs + SECS_FOR_DAY);
    check_term(now_secs + 2 * SECS_FOR_DAY);
    check_term(now_secs + 3 * SECS_FOR_DAY);
    check_term(now_secs + 4 * SECS_FOR_DAY);
    check_term(now_secs + 5 * SECS_FOR_DAY);
    check_term(now_secs + 6 * SECS_FOR_DAY);
    check_term(now_secs + 7 * SECS_FOR_DAY);
}
