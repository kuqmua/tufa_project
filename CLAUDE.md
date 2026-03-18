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

# Run all tests (with features needed for clippy tests)
cargo nextest run --features test-utils

# Run all tests with cargo test
cargo test --features test-utils

# Run a single test
cargo test test_name -- --nocapture

# Run tests for a specific crate
cargo test -p crate_name

# Run clippy test for a specific gen_* crate
cargo test -p gen_wh_flts_test --features gen_wh_flts_test/test-utils --lib

# Database: start PostgreSQL via Docker
cd server && sudo docker-compose up -d && cd ..

# Database: run migrations
cd server && sqlx migrate run && cd ..

# Expand proc macro output
cargo expand -p crate_name
```

## Toolchain

This workspace uses **Rust nightly** (latest). Build caching via `sccache` is configured in `config.toml`.

## Architecture

Large Cargo workspace (~75 crates). Core domain is PostgreSQL CRUD code generation via proc macros.

### Code generation pipeline

Each pg_crud subsystem follows the pattern: `gen_*` (thin proc-macro wrapper) -> `gen_*_src` (implementation) -> `gen_*_test` (clippy validation) + `gen_*_test_cnt` (generated content target).

The `gen_*_test` crates write generated code into `gen_*_test_cnt/src/lib.rs`, run clippy on it, then restore the file. This validates generated code quality at compile time.

### `pg_crud/` -- Central code generation subsystem

- **`pg_types/`** -- PostgreSQL type mappings (Rust <-> Postgres), split into 3 parallel sub-crates: `pg_types_numeric`, `pg_types_chrono_net`, `pg_types_text_misc`
- **`pg_json/`** -- JSON column handling, split into `pg_json_nbr` (numeric) and `pg_json_other`
- **`pg_json_obj/`** -- JSON object schema generation
- **`pg_tbl/`** -- Table-level CRUD generation (create/read/update/delete)
- **`wh_flts/`** -- WHERE clause filter generation
- **`pg_crud_cmn/`** -- Shared runtime types (PgType, PgJson traits, error types)
- **`pg_crud_macros_cmn/`** -- Shared proc-macro utilities (DTsBuilder, sqlx impl generators, Import enum)

### Key generated type hierarchy

For each base PostgreSQL type (e.g., `I16AsInt2`), the macros generate 6+ variant structs:
- `Orgn` -- raw database value (newtype around Rust type)
- `Tt` -- transfer type (wraps Orgn)
- `Cr` -- create payload
- `Rd` -- read result (with sqlx Decode)
- `Upd` -- update payload
- `Sel` -- select column selector (unit struct)
- `Wh` -- WHERE clause enum with filter variants

### DTsBuilder

`DTsBuilder` (in `macros_helpers/gen_derive_ts_builder`) is a builder for generating struct/enum declarations with selective derive macros. Used by all gen_*_src crates.

### gen_mod_with_pub_use_ts

`pg_crud_macros_cmn::gen_mod_with_pub_use_ts` wraps generated code in a module with `#[allow(clippy::absolute_paths)]` and re-exports via `pub use mod::*`. Used by all gen_* crates.

### Other key crates

- **`server/`** -- Axum web server with Docker PostgreSQL
- **`config_lib/`** -- Config management via env vars with getter trait generation
- **`loc_lib/`** -- Error handling with source location tracking
- **`naming/`** -- Naming convention utilities. `naming/src/lib.rs` and `naming/src/prm.rs` contain large macro calls generating snake_case/UpperCamelCase/TokenStream variants
- **`macros_helpers/`** -- General macro helpers
- **`token_patterns/`** -- Reusable TokenStream patterns (crate paths, trait names) used in quote! blocks
- **`to_err_string/`** -- ToErrString trait with type-specific implementations (Display, Debug, custom)

## Naming Conventions

Short abbreviations used throughout (see `NAMING.md` for full list):
- `_ts` = TokenStream, `_sc` = snake_case, `_ucc` = UpperCamelCase, `_dq` = double-quoted
- `pk` = primary key, `cr` = create, `rd` = read, `upd` = update, `del` = delete
- `fi` = field, `ft` = field type, `nn` = non-null, `nl` = nullable
- `qb` = query bind, `qp` = query part, `pgn` = pagination, `vrt` = variant
- `cmn` = common, `flt` = filter, `prm` = parameter, `drvd` = derived

## Critical Rules

- **`expect()` messages must contain 8 leading characters from a random UUID v4** (e.g., `expect("ac209d5a ...")`)
- **No `unwrap()`** -- use `expect()` with UUID prefix or proper error handling
- **No `expect()`/`panic!()` in library code** -- only allowed in proc-macro, tests, or generated test code inside `quote!`
- **No `unsafe`** -- `unsafe_code = "deny"` is set workspace-wide
- **No blank lines between code**
- Workspace-level dependencies only -- use `dependency.workspace = true` in crate `Cargo.toml`
- Extremely strict clippy config with nearly all lints set to `deny` (see workspace `Cargo.toml`)
- `arithmetic_side_effects`, `indexing_slicing`, `as_conversions`, `string_slice` are all denied
- `clippy::arbitrary_source_item_ordering` is denied -- struct fields, enum variants, and items must be alphabetically ordered
- `clippy::shadow_reuse` and `clippy::shadow_unrelated` are denied -- no variable shadowing
- Use `thiserror` for error enums
- Use abbreviations in names (see naming conventions above)
- Variables inside `quote!` blocks use `_[a-f0-9]{8}` hex suffixes to prevent shadowing in generated code (e.g., `v_f06128be`, `acc_14596a52`)
- Pre-commit hook runs `cargo fmt --check` + `cargo clippy` on the full workspace

## Compilation notes

- Generated code volume is large (~693k lines expanded across all crates)
- `pg_types` and `pg_json` are split into parallel sub-crates to improve build parallelism
- Top bottlenecks: pg_json_nbr (7.4s), pg_types_numeric (7.3s), pg_types_chrono_net (7.2s)
- `gen_pg_json` macro supports `Subset` config to generate only specific PgJson types
- `gen_pg_types` macro supports `Subset` config to generate only specific PgType types
- `dev` profile uses `debug = 1` (line tables only) for faster builds
