# tufa_project improvement plan

## 1. Code quality

### 1.4 File size
- [ ] `gen_pg_json_obj_src/src/lib.rs` (6909 lines) - split into modules
- [ ] `gen_pg_types_src/src/lib.rs` (5575 lines) - split into modules
- [ ] `gen_pg_tbl_src/src/lib.rs` (5932 lines) - split into modules
- [ ] `gen_pg_json_src/src/lib.rs` (3317 lines) - split into modules

## 3. Dependencies and build

### 3.2 Compile time
- [ ] Consider splitting heavy proc-macro crates for incremental compilation

## 4. CI/CD

### 4.2 Automation
- [ ] Automatic changelog generation from commits

## 6. Server and API

### 6.2 API design
- [ ] Ensure all endpoints are documented via utoipa/OpenAPI
- [x] Add API versioning (`/api/v1/...`)

### 6.3 Database
- [ ] Add indexes for frequently used WHERE conditions

## 8. Documentation

### 8.1 Code
- [ ] Add usage examples in doc comments for main generators

### 8.2 Project
- [x] Update `NAMING.md` if there are new abbreviations (added: oprtr, stdrt, updd, crd, al, dim, el)
- [x] Document code generation architecture (see `docs/CODE_GENERATION.md`)
- [x] Document the process of adding a new PostgreSQL type (see `docs/ADDING_PG_TYPE.md`)
- [x] Document the process of adding a new CRUD endpoint (see `docs/ADDING_CRUD_ENDPOINT.md`)

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
- [x] Check dependency graph between crates for cycles or redundancy (no issues found)
- [ ] Standardize structure of each `gen_*` subproject

### 10.2 Code generation
- [ ] Consider proc-macro alternatives for some generators (build.rs, code generation tools)
- [ ] Evaluate using `syn` for parsing instead of manual string parsing
- [ ] Standardize code generation pattern across all `gen_*` crates
- [ ] Add snapshot tests for generated code (`cargo expand` + insta) — note: `ShouldWriteTsIntoFile` already provides manual inspection capability

### 10.3 Telegram bot
- [x] Review command handler structure (simple, uses teloxide `repl()`)
- [x] Add error recovery for the bot (teloxide `repl()` handles this automatically)
- [x] Review bot logging (minimal but adequate for current scope)
- [x] Fix GitInfo command to return real git commit link instead of hardcoded string

## 11. Logging and observability

- [ ] Consider metrics (prometheus/metrics crate)

## 12. Config library

- [ ] Consider hot-reload for non-critical configuration parameters
