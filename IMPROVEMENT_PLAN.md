# tufa_project improvement plan

## 1. Code quality

### 1.4 File size
- [ ] `gen_pg_json_obj_src/src/lib.rs` (6909 lines) - split into modules — **NOTE**: single monolithic function, splitting requires extracting ~12 helper functions with large context structs. High risk.
- [ ] `gen_pg_types_src/src/lib.rs` (5575 lines) - split into modules — same monolithic pattern
- [ ] `gen_pg_tbl_src/src/lib.rs` (5932 lines) - split into modules — same pattern, 65 internal functions
- [ ] `gen_pg_json_src/src/lib.rs` (3317 lines) - split into modules — same pattern

### 1.5 Error handling
- [ ] Verify all `thiserror` error enums have informative messages

## 3. Dependencies and build

### 3.2 Compile time
- [ ] Consider splitting heavy proc-macro crates for incremental compilation

## 4. CI/CD

### 4.2 Automation
- [ ] Automatic changelog generation from commits

## 6. Server and API

### 6.1 Axum server
- [ ] Add rate limiting

### 6.2 API design
- [ ] Check API endpoint consistency (naming, HTTP methods, status codes)
- [ ] Ensure all endpoints are documented via utoipa/OpenAPI
- [ ] Add API versioning (`/api/v1/...`)
- [ ] Check pagination in all list endpoints
- [ ] Add input validation at system boundaries

### 6.3 Database
- [ ] Add indexes for frequently used WHERE conditions
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
