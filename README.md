# 📨 Rust ChatApp Backend

This is the backend for a self-hostable chat application, built in Rust with modular design and future extensibility in mind.

## 🔧 Features

- **Modular Rust Architecture** – Clean and extensible codebase built for maintainability.
- **Direct Messages & Group Chats** – Core messaging support for one-on-one and group communication.
- **Self-hostable** – Easily deployable via Docker Compose.
- **Custom `Xtime` Library** – A lightweight and user-defined time system to avoid external datetime dependencies like `chrono::DateTime`.
- **Secure Password Handling** – Passwords are stored encrypted using SHA-256 hashing.
- **MariaDB Integration** – User and message data is persisted in a MariaDB database.
- **Dynamic Environment Setup** – A `.bat` script is included to prompt for and generate your `.env` file (next to the `docker-compose.yml`) for database configuration.

## 🗃️ Database

- User data includes:
  - `uuid`, `username`, `email`
  - Timestamps for `created_at` and `last_online` stored using `Xtime`
- Messages are handled via a unified message structure that distinguishes between direct and group messages.

## 🛠️ Structure Overview

- `utils/xtime.rs` – Custom time system replacing `chrono::DateTime`
- `utils/encryption.rs` – Password hashing and verification
- `db/` – Database logic for users and message handling
- `.env` – Created via the provided batch script for secure local setup

## ⏱️ Xtime Library

The `Xtime` library was created as a custom time solution to avoid the complexities and overhead of external libraries like `chrono::DateTime`. It provides a lightweight and simplified way of handling and representing time within the application.

### Why `Xtime`?

- **Customizability**: The standard datetime libraries often provide too much functionality for our needs. `Xtime` is designed to keep things simple while offering enough features to handle the application’s needs.
- **Consistency**: With `Xtime`, we use a fixed structure and format that doesn’t rely on external libraries, ensuring consistent behavior across all instances.
- **Simplicity**: It allows easy parsing and formatting without the need for complicated third-party dependencies.

### What `Xtime` Can Do

- **Time Representation**: `Xtime` uses a structure with separate fields for seconds, minutes, hours, days, months, and years.
- **Format Conversion**: It provides simple methods for converting to and from different formats, such as converting to a string for database insertion (`YYYY-MM-DD HH:mm:ss` format).
- **Current Time**: You can generate the current time using `Xtime::now()`, which fetches the current time from the system and returns it in the custom format.
- **Parsing**: It also supports parsing times from strings in a predefined format to ensure that no external libraries are needed to parse or format the time.

## 📄 Logging System

The ChatApp backend includes a simple but powerful custom logging system that handles both runtime diagnostics and error handling. This helps track program activity and pinpoint issues without relying on external crates.

### Overview

At the heart of the logger is a thread-safe, lazily-initialized global file writer. It ensures that all log messages are written to a persistent `output.log` file and echoed to the console.

```rust
static LOG_FILE: LazyLock<Mutex<File>> = LazyLock::new(|| {
    // Initializes and opens the log file in append mode
});
```

* Logs are **timestamped** using `chrono::Local::now()`.
* Messages are formatted with `[timestamp](source_file) message`.
* Uses `Mutex` to allow **thread-safe** writes to the log file.
* Automatically creates `output.log` on first run.

### 📲 Example Output

```
[18-05-2025 16:42:13](main.rs) Failed to initialize database: Error(ConnectionRefused)
```

### 🛠️ `log_message` Function

This function handles the actual writing to the log file:

```rust
pub fn log_message(message: &str, source_file: &str)
```

* `message`: The message to log.
* `source_file`: The name of the file or module (use `file!()` macro).

Each message is timestamped, written to the `output.log` file, and printed to `stdout`.

---

## 🧠 Error Handling with `LogExpect`

To simplify error handling and ensure important errors are always logged before crashing the program, the project introduces a custom trait called `LogExpect`.

### ⚖️ How It Works

Instead of using:

```rust
db::init_db().expect("Failed to initialize database");
```

You can use:

```rust
db::init_db().log_expect("Failed to initialize database", file!());
```

This has two key benefits:

1. **Logs the error** using `log_message()` before the program panics.
2. **Includes file context** via `file!()` for easier debugging.

### 🗱️ Trait Definition

```rust
pub trait LogExpect<T> {
    fn log_expect(self, msg: &str, source_file: &str) -> T;
}
```

Implemented for all `Result<T, E>` types where `E: Debug`. The trait:

* Extracts the `Ok(T)` value normally.
* On `Err(E)`, logs the error along with a custom message.
* Panics immediately afterward with the same message.

### 💡 Example Usage

```rust
use crate::utils::logger::LogExpect;

fn start() {
    db::init_db().log_expect("Failed to initialize database", file!());
}
```

This approach combines **panic handling** and **diagnostic logging** into one clean API, and helps surface critical failures during development and production.



## 📦 Coming Soon

- Installation & deployment guide
- Full API documentation
