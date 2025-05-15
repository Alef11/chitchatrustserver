# Use the official Rust image as the build environment
FROM rust:latest AS builder

# Create a new directory for the app source code
WORKDIR /app

# Copy your Cargo.toml and Cargo.lock first to leverage Docker cache for dependencies
COPY Cargo.toml Cargo.lock ./

# Create an empty src/main.rs so that dependencies can be built independently
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release || true

# Remove the dummy main.rs and copy the actual source code
RUN rm -rf src
COPY src ./src

# Build the actual project in release mode
RUN cargo build --release

# Use a smaller base image for the final build
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/chitchatrustserver /usr/local/bin/chitchatrustserver

CMD ["chitchatrustserver"]
