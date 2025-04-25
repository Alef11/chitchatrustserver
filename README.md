# ChitChatRustServer

ChitChatRustServer is a chat application currently under development. It aims to provide a secure and efficient platform for private messaging and group chats. Built with Rust, the project focuses on performance, scalability, and security.

## Features (Planned)
- **Private Messaging**: Secure one-on-one conversations.
- **Group Chats**: Create and manage group conversations.
- **End-to-End Encryption**: Ensure privacy and security for all messages.
- **User Authentication**: Robust user management and authentication system.
- **Real-Time Communication**: Instant message delivery using efficient protocols.

### Key Components
- **Encryption Utilities**: Located in [`src/utils/encryption.rs`](src/utils/encryption.rs), this module handles hashing and password verification using the SHA-256 algorithm.
- **Modules**: Core functionalities like user management and messaging are implemented in the `src/modules` directory.

## Getting Started
### Prerequisites
- Rust (latest stable version)

### Installation
1. Clone the repository:
   ```sh
   git clone https://github.com/Alef11/chitchatrustserver.git
   cd chitchatrustserver