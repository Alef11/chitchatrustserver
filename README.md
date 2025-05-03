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



## 📦 Coming Soon

- Installation & deployment guide
- Full API documentation
