# tufa_project improvement plan

## 1. Code quality

### 1.3 Code reuse in macros
- [x] Extract common DTsBuilder patterns across `pg_types`, `pg_json`, `pg_json_obj`, `pg_tbl` — added `serde_er_enum_d_ts_builder()` and `er_enum_d_ts_builder()` factory functions to `pg_crud_macros_cmn`
- [x] Find duplicate `quote!{}` blocks for sqlx encode/decode across crates — already centralized in `pg_crud_macros_cmn`
- [x] Move common impl block generators into `pg_crud_macros_cmn` — already done
- [x] Check `gen_pg_json_cmn` completeness — checked, remaining patterns are closures capturing local variables (not extractable)
- [x] Unify error enum generation (serde_er_enum_d_ts_builder is used in 3+ crates) — unified via factory functions
- [x] Consider moving common DTsBuilder configurations into `pg_crud_macros_cmn` — done

### 1.4 File size
- [ ] `gen_pg_json_obj_src/src/lib.rs` (6909 lines) - split into modules — **NOTE**: single monolithic function, splitting requires extracting ~12 helper functions with large context structs. High risk.
- [ ] `gen_pg_types_src/src/lib.rs` (5575 lines) - split into modules — same monolithic pattern
- [ ] `gen_pg_tbl_src/src/lib.rs` (5932 lines) - split into modules — same pattern, 65 internal functions
- [ ] `gen_pg_json_src/src/lib.rs` (3317 lines) - split into modules — same pattern
- [x] `gen_wh_flts_src/src/lib.rs` - evaluate need for splitting — `gen_wh_flts_src` does not exist, `gen_wh_flts/src/lib.rs` is 1660 lines (acceptable)

### 1.5 Error handling
- [x] Consider replacing `expect()` in proc-macro with `compile_error!()` where possible — replaced entry-point `parse2().expect()` and config `from_str().expect()` in all 5 gen_* crates with `compile_error!()` for better error messages
- [ ] Verify all `thiserror` error enums have informative messages

## 3. Dependencies and build

### 3.1 Dependencies
- [x] Consider `cargo-deny` for dependency policy — installed and configured in `deny.toml`

### 3.2 Compile time
- [ ] Consider splitting heavy proc-macro crates for incremental compilation

### 3.3 Cargo.toml
- [x] Check feature flags for unused features — removed 9 unused features: reqwest (cookies, gzip, brotli, deflate, multipart, stream, http2), tower-http (fs), tracing (valuable)

## 4. CI/CD

### 4.2 Automation
- [x] Consider `cargo-make` or `just` for standardizing dev commands — created `Justfile` with build/test/lint/ci/db/expand commands
- [ ] Automatic changelog generation from commits

## 5. Docker and deployment

### 5.1 Docker
- [x] Consider multi-stage Docker build for production server image — created `Dockerfile` with builder/runtime stages and `.dockerignore`

## 6. Server and API

### 6.1 Axum server
- [x] Review route structure - module organization — routes are auto-generated via proc-macros, structure is appropriate
- [x] Review CORS settings — made CORS configurable via `CORS_ALLOW_ORIGIN` env var (comma-separated origins)
- [ ] Add rate limiting

### 6.2 API design
- [ ] Check API endpoint consistency (naming, HTTP methods, status codes)
- [ ] Ensure all endpoints are documented via utoipa/OpenAPI
- [ ] Add API versioning (`/api/v1/...`)
- [ ] Check pagination in all list endpoints
- [ ] Add input validation at system boundaries

### 6.3 Database
- [ ] Add indexes for frequently used WHERE conditions
- [x] Review connection pool settings — made `PG_POOL_MAX_CONNECTIONS` configurable via env var (was hardcoded to 50)
- [ ] Add slow query monitoring

## 7. Security

### 7.1 General
- [ ] Review all endpoints for authentication/authorization

## 8. Documentation

### 8.1 Code
- [ ] Add doc comments to public API in `pg_crud_cmn`
- [ ] Add doc comments to public API in `loc_lib`
- [ ] Add doc comments to key types in `naming`
- [ ] Add usage examples in doc comments for main generators

### 8.2 Project
- [ ] Update `NAMING.md` if there are new abbreviations
- [ ] Document code generation architecture (how `gen_*` crates work together)
- [ ] Document the process of adding a new PostgreSQL type
- [ ] Document the process of adding a new CRUD endpoint

## 9. Performance

### 9.1 Runtime
- [ ] Profile server endpoints under load
- [ ] Check for N+1 query problems
- [ ] Consider caching for frequently read data
- [ ] Check API response sizes for over-fetching

### 9.2 Code generation
- [ ] Profile proc-macro execution time
- [ ] Verify `TokenStream` concatenation is efficient

## 10. Architecture

### 10.1 Workspace structure
- [ ] Evaluate necessity of each of ~70 crates - consider merging small ones
- [ ] Check dependency graph between crates for cycles or redundancy
- [ ] Standardize structure of each `gen_*` subproject

### 10.2 Code generation
- [ ] Consider proc-macro alternatives for some generators (build.rs, code generation tools)
- [ ] Evaluate using `syn` for parsing instead of manual string parsing
- [ ] Standardize code generation pattern across all `gen_*` crates
- [ ] Add snapshot tests for generated code (`cargo expand` + insta)

### 10.3 Telegram bot
- [ ] Review command handler structure
- [ ] Add error recovery for the bot
- [ ] Review bot logging

## 11. Logging and observability

- [ ] Configure log levels per module
- [ ] Consider metrics (prometheus/metrics crate)

## 12. Config library

- [ ] Consider hot-reload for non-critical configuration parameters
