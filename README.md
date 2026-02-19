# Tufa Project

A comprehensive Rust-based project for building web applications with PostgreSQL integration.

## Table of Contents
- [Project Overview](#project-overview)
- [Key Features](#key-features)
- [Project Structure](#project-structure)
- [Setup and Installation](#setup-and-installation)
- [Usage](#usage)
- [Modules](#modules)
- [Contributing](#contributing)
- [License](#license)

## Project Overview

Tufa Project is a Rust workspace containing multiple crates designed to facilitate the development of web applications with PostgreSQL database integration. The project includes various utilities, macros, and libraries to streamline common development tasks.

## Key Features

- PostgreSQL CRUD operations with code generation
- Advanced er handling with detailed context
- Configuration management
- Git information integration
- Type-safe database interactions
- Extensible macro system
- JSON schema validation for PostgreSQL

## Project Structure

This is a Rust workspace project with the following main components:

- `postgresql_crud`: Core crate for PostgreSQL CRUD operations
- `config_lib`: Configuration management utilities
- `git_info`: Git repository information tools
- `location_lib`: Advanced er handling with context
- `from_sqlx_postgres_er`: SQLx PostgreSQL er conversion
- `from_str`: String parsing utilities
- `gen_quotes`: Quote generation utilities
- `macros_helpers`: Helper macros for code generation
- And many more utility crates...

## Setup and Installation

### Prerequisites

- Rust (latest stable version)
- PostgreSQL database
- Docker (for containerized deployments)

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd tufa_project
   ```

2. Initialize submodules:
   ```bash
   git submodule update --init --recursive --checkout
   ```

3. Build the project:
   ```bash
   cargo build
   ```

### Database Setup

1. Start the database:
   ```bash
   cd server && sudo docker-compose up -d && cd ..
   ```

2. Run migrations:
   ```bash
   cd server && sqlx migrate run && cd ..
   ```

## Usage

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with features
cargo test --features test-utils -- --nocapture

# Run with debug logging
RUST_LOG=sqlx=debug cargo test --features test-utils -- --nocapture
```

### Code Quality

```bash
# Check for ers
cargo check

# Run clippy lints
cargo clippy

# Check with specific flags
RUSTFLAGS="-Awarnings" cargo clippy --all-targets --all-features
```

### Development

```bash
# Start development with file watching
cargo watch -x check -x test -x "run"
```

## Modules

### postgresql_crud
Core functionality for PostgreSQL CRUD operations with automatic code generation for tables, types, and JSON objects.

### config_lib
Configuration management with environment variable parsing and type-safe accessors.

### git_info
Compile-time and runtime Git repository information retrieval.

### location_lib
Advanced er handling system with detailed context and source tracking.

### from_sqlx_postgres_er
Utilities for converting SQLx PostgreSQL ers to application-specific ers.

### from_str
Safe string parsing with detailed er context.

### gen_quotes
Utilities for generating quotes and text content.

### macros_helpers
Collection of helper macros for code generation and boilerplate reduction.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
