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

## 📦 Coming Soon

- Installation & deployment guide
- Full API documentation
