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

### 6.3 Database
- [ ] Add indexes for frequently used WHERE conditions

## 9. Performance

### 9.1 Runtime
- [ ] Profile server endpoints under load
- [ ] Check for N+1 query problems
- [ ] Consider caching for frequently read data
- [ ] Check API response sizes for over-fetching

### 9.2 Code generation
- [ ] Verify `TokenStream` concatenation is efficient

## 10. Architecture

### 10.2 Code generation
- [ ] Consider proc-macro alternatives for some generators (build.rs, code generation tools)
- [ ] Evaluate using `syn` for parsing instead of manual string parsing
- [ ] Standardize code generation pattern across all `gen_*` crates
- [ ] Add snapshot tests for generated code (`cargo expand` + insta)

## 11. Logging and observability

- [ ] Consider metrics (prometheus/metrics crate)

## 12. Config library

- [ ] Consider hot-reload for non-critical configuration parameters
