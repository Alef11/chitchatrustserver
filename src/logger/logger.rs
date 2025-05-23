use std::fmt::Debug;
use std::io::Write;
use std::sync::Mutex;
use std::{fs::File, path::Path, sync::LazyLock};

use chrono_tz::Europe::Berlin;

pub trait LogExpect<T> {
    fn log_expect(self, msg: &str, source_file: &str) -> T;
}

impl<T, E: Debug> LogExpect<T> for Result<T, E> {
    fn log_expect(self, msg: &str, source_file: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                let error_message = format!("{msg}: {err:?}");
                log_error(&error_message, source_file);
                panic!("{msg}: {err:?}");
            }
        }
    }
}

static LOG_FILE: LazyLock<Mutex<File>> = LazyLock::new(|| {
    let path = "output.log";
    if !Path::new(path).exists() {
        File::create(path).expect("Failed to create log file");
    }

    let file = File::options()
        .append(true)
        .open(path)
        .expect("Failed to open log file");

    Mutex::new(file)
});

#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        $crate::logger::logger::log_message($msg, file!());
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::logger::logger::log_message(&format!($fmt, $($arg)*), file!());
    };
}

pub fn log_message(message: &str, source_file: &str) {
    let current_time = chrono::Utc::now().with_timezone(&Berlin);
    let formatted_time = current_time.format("%d-%m-%Y %H:%M:%S").to_string();

    let log_message = format!("[{}]({}) {}", formatted_time, source_file, message);

    let mut file = LOG_FILE.lock().unwrap();
    writeln!(file, "{log_message}").expect("Failed to write to log file");
    println!("{log_message}");
}

pub fn log_error(message: &str, source_file: &str) {
    let current_time = chrono::Local::now();
    let formatted_time = current_time.format("%d-%m-%Y %H:%M:%S").to_string();

    let log_message = format!("[ERROR] [{}]({}) {}", formatted_time, source_file, message);

    let mut file = LOG_FILE.lock().unwrap();
    writeln!(file, "{log_message}").expect("Failed to write to log file");
    eprintln!("{log_message}");
}
