# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

```bash
# Build
cargo build

# Format
cargo fmt

# Lint (run before completing any task)
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test

# Run a single test
cargo test test_name -- --nocapture

# Run tests for a specific crate
cargo test -p crate_name

# Database: start PostgreSQL via Docker
cd server && sudo docker-compose up -d && cd ..

# Database: run migrations
cd server && sqlx migrate run && cd ..

# Expand proc macro output
cargo expand your::path::to::module --lib
```

## Toolchain

This workspace uses **Rust nightly** (latest). Build caching via `sccache` is configured in `config.toml`.

## Architecture

Large Cargo workspace (~70 crates). Core domain is PostgreSQL CRUD code generation.

### `pg_crud/` — Central code generation subsystem

Each subdirectory follows a consistent pattern with `gen_*` (proc-macro crate), `gen_*_src` (macro implementation logic), `gen_*_test`, and `gen_*_test_cnt` (test content) crates:

- **`pg_types/`** — PostgreSQL type mappings (Rust ↔ Postgres), with `pg_types_common` for shared types
- **`pg_table/`** — Table-level CRUD generation (create/read/update/delete)
- **`pg_json/`** — JSON column handling, with `gen_pg_json_common` for shared logic
- **`pg_json_obj/`** — JSON object schema generation, with `pg_json_obj_common`
- **`wh_filters/`** — WHERE clause filter generation
- **`pg_crud_common/`** — Shared types across pg_crud crates
- **`pg_crud_macros_common/`** — Shared proc-macro utilities
- **`pg_crud_common_and_macros_common/`** — Types shared between runtime and macro code

### Other key crates

- **`server/`** — Axum web server (has `docker-compose.yml` for PostgreSQL)
- **`config_lib/`** — Config management with sub-crates for env parsing, getter trait generation, server port validation
- **`location_lib/`** — Error handling with source location tracking
- **`naming/`** — Naming convention utilities (`naming_common`, `naming_macros`)
- **`macros_helpers/`** — General macro helpers including `gen_derive_ts_builder`
- **`telegram_bot/`** — Teloxide-based Telegram bot

## Naming Conventions

Short abbreviations are used throughout (see `NAMING.md` for full list):
- `_ts` = TokenStream, `_sc` = snake_case, `_ucc` = UpperCamelCase, `_dq` = double-quoted
- `pk` = primary key, `cr` = create, `rd` = read, `upd` = update, `del` = delete
- `fi` = field, `ft` = field type, `nn` = non-null, `nl` = nullable
- `qb` = query bind, `qp` = query part, `pgn` = pagination, `vrt` = variant

## Critical Rules

- **`expect()` messages must contain 8 leading characters from a random UUID v4** (e.g., `expect("ac209d5a ...")`)
- **No `unwrap()`** — use `expect()` with UUID prefix or proper error handling
- **No `expect()`/`panic!()` in library code** — only allowed in proc-macro, tests, or generated test code inside `quote!`
- **No `unsafe`** — `unsafe_code = "deny"` is set workspace-wide
- **No blank lines between code**
- Workspace-level dependencies only — use `dependency.workspace = true` in crate `Cargo.toml`
- Extremely strict clippy config with nearly all lints set to `deny` (see workspace `Cargo.toml`)
- `arithmetic_side_effects`, `indexing_slicing`, `as_conversions`, `string_slice` are all denied
- Use `thiserror` for error enums
- Keep generated functions/closures inside their usage scope
- Use abbreviations in names (see naming conventions above)
