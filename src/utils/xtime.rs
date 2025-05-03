//xtime einfach für coolen Namen wollte ich hier angemerkt haben
//im endeffekt ist es einfach ein eigenes Zeitsystem weil alle anderen irgendwie kacke sind

use chrono::{Datelike, Timelike};
use chrono_tz::Europe::Berlin;

pub struct Xtime {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub days: u8,
    pub months: u8,
    pub years: u16,
}

impl Xtime {
    pub fn new(seconds: u8, minutes: u8, hours: u8, days: u8, months: u8, years: u16) -> Self {
        if seconds > 60 {
            panic!("Seconds must be between 0 and 60");
        }
        if minutes > 60 {
            panic!("Minutes must be between 0 and 60");
        }
        if hours > 24 {
            panic!("Hours must be between 0 and 24");
        }
        if days > 31 {
            panic!("Days must be between 0 and 31");
        }
        if months > 12 {
            panic!("Months must be between 0 and 12");
        }
        Xtime {
            seconds,
            minutes,
            hours,
            days,
            months,
            years,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:02}-{:02}-{:02}-{:02}-{:02}-{:04}",
            self.seconds, self.minutes, self.hours, self.days, self.months, self.years
        )
    }

    pub fn from_string(time: &str) -> Self {
        let parts: Vec<&str> = time.split('-').collect();
        if parts.len() != 6 {
            panic!("Invalid time format. Expected format is ss-mm-hh-dd-MM-yyyy");
        }
        let seconds = parts[0].parse::<u8>().unwrap();
        let minutes = parts[1].parse::<u8>().unwrap();
        let hours = parts[2].parse::<u8>().unwrap();
        let days = parts[3].parse::<u8>().unwrap();
        let months = parts[4].parse::<u8>().unwrap();
        let years = parts[5].parse::<u16>().unwrap();
        Xtime::new(seconds, minutes, hours, days, months, years)
    }

    pub fn now() -> Self {
        let now = chrono::Utc::now().with_timezone(&Berlin);
        Xtime::new(
            now.second() as u8,
            now.minute() as u8,
            now.hour() as u8,
            now.day() as u8,
            now.month() as u8,
            now.year() as u16,
        )
    }

    pub fn to_mariadb_datetime(&self) -> String {
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.years, self.months, self.days, self.hours, self.minutes, self.seconds
        )
    }

    pub fn from_mariadb_datetime(datetime: &str) -> Self {
        let parts: Vec<&str> = datetime.split(' ').collect();
        if parts.len() != 2 {
            panic!("Invalid datetime format. Expected format is yyyy-MM-dd hh:mm:ss");
        }
        let date_parts: Vec<&str> = parts[0].split('-').collect();
        let time_parts: Vec<&str> = parts[1].split(':').collect();
        if date_parts.len() != 3 || time_parts.len() != 3 {
            panic!("Invalid datetime format. Expected format is yyyy-MM-dd hh:mm:ss");
        }
        let years = date_parts[0].parse::<u16>().unwrap();
        let months = date_parts[1].parse::<u8>().unwrap();
        let days = date_parts[2].parse::<u8>().unwrap();
        let hours = time_parts[0].parse::<u8>().unwrap();
        let minutes = time_parts[1].parse::<u8>().unwrap();
        let seconds = time_parts[2].parse::<u8>().unwrap();
        Xtime::new(seconds, minutes, hours, days, months, years)
    }
}
