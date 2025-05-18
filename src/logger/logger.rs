use std::{fs::File, path::Path, sync::LazyLock};
use futures::lock::Mutex;
use std::io::Write;

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

pub async fn log_message(message: &str) {
    let current_time = chrono::Local::now();
    let mut formatted_time = current_time.format("%d-%m-%Y %H:%M:%S").to_string();
    formatted_time = format!("[{}] ", formatted_time);

    let log_message = format!("{}{}", formatted_time, message);

    let mut file = LOG_FILE.lock().await;
    writeln!(file, "{log_message}").expect("Failed to write to log file");
    println!("{log_message}");
}