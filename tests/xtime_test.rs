use chitchatrustserver::utils::xtime::Xtime;
use chrono::{Datelike, Timelike};
use chrono_tz::Europe::Berlin;

#[test]
fn test_time_string() {
    let time = Xtime::new(30, 45, 12, 15, 8, 2023);
    let time_string = time.to_string();
    assert_eq!(time_string, "30-45-12-15-08-2023");
}

#[test]
fn test_time_string_invalid() {
    let time = Xtime::new(30, 45, 12, 15, 8, 2023);
    let time_string = time.to_string();
    assert_ne!(time_string, "30-45-12-15-08-2024");
}

#[test]
fn test_time_string_invalid_format() {
    let time = Xtime::new(30, 45, 12, 15, 8, 2023);
    let time_string = time.to_string();
    assert_ne!(time_string, "30-45-12-15-08-2023-00");
}

#[test]
fn test_time_from_string() {
    let time_string = "30-45-12-15-08-2023";
    let time = Xtime::from_string(time_string);
    assert_eq!(time.seconds, 30);
    assert_eq!(time.minutes, 45);
    assert_eq!(time.hours, 12);
    assert_eq!(time.days, 15);
    assert_eq!(time.months, 8);
    assert_eq!(time.years, 2023);
}

#[test]
fn test_now() {
    let now = chrono::Utc::now().with_timezone(&Berlin);
    let time = Xtime::now();
    assert_eq!(time.seconds, now.second() as u8);
    assert_eq!(time.minutes, now.minute() as u8);
    assert_eq!(time.hours, now.hour() as u8);
    assert_eq!(time.days, now.day() as u8);
    assert_eq!(time.months, now.month() as u8);
    assert_eq!(time.years, now.year() as u16);

    println!("Current time: {}", time.to_string());
}
