# Setup Guide

## Table of Contents
- [Development Commands](#development-commands)
- [Docker Setup](#docker-setup)
- [Database Management](#database-management)
- [Project Management](#project-management)
- [Testing](#testing)
- [Performance Optimization](#performance-optimization)
- [Troubleshooting](#troubleshooting)

## Development Commands

### Common Development Commands
```bash
# Run tests with output
cargo test --features test-utils -- --nocapture

# Run tests with SQLx debug logging
RUST_LOG=sqlx=debug cargo test --features test-utils -- --nocapture

# Run clippy with warnings disabled
RUSTFLAGS="-Awarnings" cargo clippy --all-targets --all-features

# Run tests with warnings disabled and SQLx debug logging
RUSTFLAGS="-Awarnings" RUST_LOG=sqlx=debug cargo test --features test-utils -- --nocapture
```

### Cargo Watch
```bash
# Watch for changes and run checks, tests, and application
cargo watch -x check -x test -x "run | bunyan"

# Quiet watch mode
cargo watch -q -x run
```

### Code Formatting
You can tune rustfmt for a project with a configuration file, `rustfmt.toml`.
Details can be found in [rustfmt](https://github.com/rust-lang/rustfmt#configuring-rustfmt)

### Dependency Management
```bash
# Check for vulnerabilities
cargo install cargo-audit
cargo audit

# Remove unused dependencies
cargo install cargo-udeps
cargo +nightly udeps

# View dependency tree
cargo tree
```

### Compilation Cache
```bash
# Install sccache for faster compilation
cargo install sccache --locked
```

## Docker Setup

### Start Docker Daemon
```bash
sudo dockerd
```

### Build Docker Container
```bash
# Install nightly toolchain
rustup install nightly

# Add musl target
rustup target add x86_64-unknown-linux-musl

# Build release
cargo +nightly build --release

# Build Docker image
sudo docker build -t server-image .
```

### Run Docker Container
```bash
# Run container with environment file
docker run --env-file .env --name server-container -p 8000:8000 --rm -it server-image

# Run with custom image
sudo docker run -p 8000:8000 session_based_authentication
```

### Manage Docker Containers
```bash
# Stop container
sudo docker stop server-container

# Remove container
sudo docker rm server-container

# Remove unused containers and images
sudo docker system prune -f
```

### Docker Compose
```bash
# Start services
sudo docker-compose up -d

# Stop services
sudo docker-compose down
```

### Docker Volumes
Default docker volumes folder on Linux: `/var/lib/docker/volumes`

## Database Management

### PostgreSQL Setup
```bash
# Pull and run PostgreSQL container
sudo docker run -p 5432:5432/tcp --name postgres-tufa-wsl2 -v ~/db-volumes/postgresql-volumes/tufa-dev-volume -e POSTGRES_PASSWORD=postgres -d postgres:latest
```

### Start PostgreSQL
```bash
# Start container
sudo docker start postgres-tufa-wsl2

# Start with docker-compose
sudo docker-compose -f docker-compose.yml up -d
```

### SQLx CLI
```bash
# Install SQLx CLI
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres

# Add migration
sqlx migrate add create_subscriptions_table

# Run migrations
sqlx migrate run

# Prepare queries for offline verification
sqlx prepare
# or
cargo sqlx prepare -- --lib
```

### Database Scripts
```bash
# Initialize database
sudo ./scripts/init_db.sh

# Run migrations from project root
cd server && sqlx migrate run && cd ..
```

## Project Management

### Create New Library
```bash
cargo new example_lib --lib
```

### Start Development
```bash
# Watch for changes and run checks, tests, and application
cargo watch -x check -x test -x "run"
```

### Custom Linker Dependencies

#### Windows
```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

Add to `.cargo/config.toml`:
```toml
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

#### Linux
Ubuntu:
```bash
sudo apt-get install lld clang
```

Arch:
```bash
sudo pacman -S lld clang
```

Add to `.cargo/config.toml`:
```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
```

#### MacOS
```bash
brew install michaeleisel/zld/zld
```

Add to `.cargo/config.toml`:
```toml
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
```

## Testing

### Run Tests
```bash
# Run CI tests
cd libs/tests_lib && cargo test ci -- --show-output

# Run local tests
cd libs/tests_lib && cargo test local -- --show-output

# Run with output
cargo test -- --nocapture
```

### Property-Based Testing
There are two mainstream options for property-based testing in the Rust ecosystem: `quickcheck` and `proptest`.

### Large Test Suites
If you have large test suite with a flat file structure, you'll soon be building tens of executable every time you run `cargo test`. While each executable is compiled in parallel, the linking phase is instead entirely sequential! Bundling all your test cases in a single executable reduces the time spent compiling your test suite in CI.

If you are running Linux, you might see errors like:
```
thread 'actix-rt:worker' panicked at 
'Can not create Runtime: Os { code: 24, kind: Other, message: "Too many open files" }',
```

This is due to a limit enforced by the operating system on the maximum number of open file descriptors (including sockets) for each process. The limit is usually set to 1024, but you can raise it with `ulimit -n X` (e.g. `ulimit -n 10000`) to resolve the issue.

## Performance Optimization

### Smaller Docker Builds
We could go even smaller by using `rust:1.59.0-alpine`, but we would have to cross-compile to the linux-musl target - out of scope for now. Check out `rust-musl-builder` if you are interested in generating tiny Docker images.

Another option to reduce the size of our binary further is stripping symbols from it.

### Logging
```bash
# Install logs formatter
cargo install bunyan

# Run server with formatted logs
cargo run -q | bunyan
```

## Troubleshooting

### Permission Errors
```bash
# Fix permission denied errors
cd .. sudo chmod -R 777 server && cd server

# Fix I/O error: Permission denied (os error 13)
sudo chown -R $(whoami) session_based_authentication/
```

### WSL Connection Issues
If database clients cannot connect to database in WSL2:
```cmd
wsl --shutdown
```
Then reopen WSL.

### Volume Permissions
```bash
# Give privileges to volumes folder
sudo chown -R username /folderexample
```
(/db-volumes/mongodb or postgresql)

### Start Command
```bash
cd libs/tests_lib && cargo test local && cd .. && cd .. && cargo run