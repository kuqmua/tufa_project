# tufa_project improvement plan

## 1. Code quality

### 1.1 TODO/FIXME/HACK comments (115+)
- [ ] Process all TODO comments in `pg_crud/pg_types/gen_pg_types_src/src/lib.rs`
- [ ] Process all TODO comments in `pg_crud/pg_json_obj/gen_pg_json_obj_src/src/lib.rs`
- [ ] Process all TODO comments in `pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs`
- [ ] Process all TODO comments in `pg_crud/pg_json/gen_pg_json_src/src/lib.rs`
- [ ] Process all TODO comments in `pg_crud/wh_flts/gen_wh_flts_src/src/lib.rs`
- [ ] Process FIXME and HACK comments across all crates
- [ ] Remove outdated TODOs that are already resolved

### 1.2 Commented-out code
- [x] Review and remove commented-out code in `gen_pg_types_src/src/lib.rs`
- [x] Review and remove commented-out code in `gen_pg_json_obj_src/src/lib.rs`
- [x] Review and remove commented-out code in `gen_pg_tbl_src/src/lib.rs`
- [x] Review and remove commented-out code in `gen_pg_json_src/src/lib.rs`
- [x] Review and remove commented-out code in `server/src/`

### 1.3 Code reuse in macros
- [ ] Extract common DTsBuilder patterns across `pg_types`, `pg_json`, `pg_json_obj`, `pg_tbl`
- [ ] Find duplicate `quote!{}` blocks for sqlx encode/decode across crates
- [ ] Move common impl block generators into `pg_crud_macros_cmn`
- [ ] Check `gen_pg_json_cmn` completeness - ensure all shared patterns between `pg_json` and `pg_json_obj` are extracted
- [ ] Unify error enum generation (serde_er_enum_d_ts_builder is used in 3+ crates)
- [ ] Consider moving common DTsBuilder configurations into `pg_crud_macros_cmn`

### 1.4 File size
- [ ] `gen_pg_json_obj_src/src/lib.rs` (~6930 lines) - split into modules
- [ ] `gen_pg_types_src/src/lib.rs` (~5645 lines) - split into modules
- [ ] `gen_pg_tbl_src/src/lib.rs` (~3800 lines) - split into modules
- [ ] `gen_pg_json_src/src/lib.rs` (~3300 lines) - split into modules
- [ ] `gen_wh_flts_src/src/lib.rs` - evaluate need for splitting

### 1.5 Error handling
- [x] Verify all `expect()` calls have 8-char UUID prefix (all 395 calls comply)
- [x] Replace remaining `unwrap()` with `expect()` + UUID (0 unwrap found)
- [x] Verify no `expect()`/`panic!()` in library code - 4 justified exceptions (trait API, const fn)
- [ ] Consider replacing `expect()` in proc-macro with `compile_error!()` where possible for better error messages
- [ ] Verify all `thiserror` error enums have informative messages

## 2. Testing (1,758 tests)

### 2.1 Test coverage
- [ ] Add tests for `pg_crud_macros_cmn` - macro utilities
- [ ] Add tests for `pg_crud_cmn` - shared types
- [ ] Add tests for `pg_crud_cmn_and_macros_cmn`
- [ ] Add tests for `naming` crates
- [ ] Add tests for `config_lib` and its subcrates
- [ ] Add tests for `loc_lib` - error location tracking
- [ ] Evaluate edge case coverage in existing tests

### 2.2 Test structure
- [ ] Unify `gen_*_test` / `gen_*_test_cnt` pattern - verify all crates follow it
- [ ] Verify tests run in CI
- [ ] Add integration tests for server endpoints
- [ ] Consider property-based testing for code generators

### 2.3 Test infrastructure
- [ ] Set up separate test DB from dev DB
- [ ] Add fixtures/factories for test data
- [ ] Consider `sqlx::test` for database tests with automatic rollback

## 3. Dependencies and build

### 3.1 Dependencies
- [x] Audit workspace dependencies for unused ones (all "unused" were false positives due to proc-macro codegen)
- [x] Check dependency versions are up to date (bytes 1.11.0->1.11.1, time 0.3.44->0.3.47)
- [x] Run `cargo audit` for known vulnerabilities (fixed 2/3, rsa has no fix available)
- [ ] Consider `cargo-deny` for dependency policy
- [ ] Verify all dependencies are declared at workspace level (no version duplication)

### 3.2 Compile time
- [ ] Profile compile time (`cargo build --timings`)
- [ ] Identify heaviest crates and optimize
- [ ] Verify `sccache` configuration effectiveness
- [ ] Consider splitting heavy proc-macro crates for incremental compilation
- [ ] Evaluate `--all-features` impact on build time

### 3.3 Cargo.toml
- [ ] Verify all crates have correct metadata (description, license, etc.)
- [ ] Remove `publish = false` where not needed (or add where needed)
- [ ] Check feature flags for unused features

## 4. CI/CD

### 4.1 CI setup
- [ ] Add GitHub Actions workflow (or verify existing one)
- [ ] CI should run: `cargo fmt --check`, `cargo clippy`, `cargo test`
- [ ] Add `cargo doc` check to CI
- [ ] Add `cargo audit` to CI pipeline
- [ ] Configure cargo registry and target caching in CI

### 4.2 Automation
- [ ] Add pre-commit hooks for fmt and clippy
- [ ] Consider `cargo-make` or `just` for standardizing dev commands
- [ ] Automatic changelog generation from commits

## 5. Docker and deployment

### 5.1 Docker
- [x] Review and update `docker-compose.yml` in `server/`
- [x] Add health checks for PostgreSQL container
- [x] Add volume for DB data persistence (already existed)
- [ ] Consider multi-stage Docker build for production server image
- [x] Add `.dockerignore` for build context optimization (already existed)

### 5.2 Deployment
- [ ] Document deployment process
- [ ] Set up environment-specific configurations
- [ ] Consider containerizing the entire application

## 6. Server and API

### 6.1 Axum server
- [ ] Review route structure - module organization
- [ ] Add request logging middleware
- [ ] Add error handling middleware (unified error response)
- [ ] Review CORS settings
- [ ] Add rate limiting
- [ ] Review graceful shutdown

### 6.2 API design
- [ ] Check API endpoint consistency (naming, HTTP methods, status codes)
- [ ] Ensure all endpoints are documented via utoipa/OpenAPI
- [ ] Add API versioning (`/api/v1/...`)
- [ ] Check pagination in all list endpoints
- [ ] Add input validation at system boundaries

### 6.3 Database
- [ ] Check all migrations for idempotency
- [ ] Add indexes for frequently used WHERE conditions
- [ ] Review connection pool settings
- [ ] Consider prepared statements for frequent queries
- [ ] Add slow query monitoring

## 7. Security

### 7.1 General
- [x] Verify `.env` files are excluded from git (`.gitignore`) - added `telegram_bot/.env`, created `.env.example`
- [ ] Review all endpoints for authentication/authorization
- [x] Verify SQL injection protection (sqlx parameterized queries are used)
- [x] Verify no hardcoded secrets in code (clean, except dev docker-compose)
- [ ] Consider using `secrecy` crate for sensitive data

### 7.2 Dependencies
- [ ] Regular `cargo audit`
- [ ] Set up Dependabot or Renovate for automatic dependency updates

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
- [x] Optimize `Vec` allocations - use `with_capacity` where size is known (8 places fixed)
- [ ] Verify `TokenStream` concatenation is efficient

## 10. Architecture

### 10.1 Workspace structure
- [ ] Evaluate necessity of each of ~70 crates - consider merging small ones
- [ ] Check dependency graph between crates for cycles or redundancy
- [ ] Standardize structure of each `gen_*` subproject
- [ ] Consider workspace inheritance for shared Cargo.toml settings

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

- [ ] Set up structured logging (`tracing` crate)
- [ ] Add request ID for request tracing
- [ ] Configure log levels per module
- [ ] Consider metrics (prometheus/metrics crate)
- [ ] Add health check endpoint (`/health`, `/ready`)

## 12. Config library

- [ ] Verify configuration validation at startup
- [ ] Add default values where appropriate
- [ ] Document all environment variables
- [ ] Consider hot-reload for non-critical configuration parameters
