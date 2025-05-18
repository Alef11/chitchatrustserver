use std::{fs::File, path::Path, sync::LazyLock};
use std::sync::Mutex;
use std::io::Write;
use std::fmt::Debug;

pub trait LogExpect<T> {
    fn log_expect(self, msg: &str, source_file: &str) -> T;
}

impl<T, E: Debug> LogExpect<T> for Result<T, E> {
    fn log_expect(self, msg: &str, source_file: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                let error_message = format!("{msg}: {err:?}");
                log_message(&error_message, source_file);
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

pub fn log_message(message: &str, source_file: &str) {

    let current_time = chrono::Local::now();
    let formatted_time = current_time.format("%d-%m-%Y %H:%M:%S").to_string();

    let log_message = format!("[{}]({}) {}", formatted_time, source_file, message);

    let mut file = LOG_FILE.lock().unwrap();
    writeln!(file, "{log_message}").expect("Failed to write to log file");
    println!("{log_message}");
}

