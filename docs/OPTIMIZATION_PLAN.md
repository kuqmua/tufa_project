# Optimization plan

## Current state

| Metric | Value |
|--------|-------|
| Workspace crates | 77 |
| Proc-macro crates | 29 |
| Total Rust lines | ~92k |
| Generated code (pg_types + pg_json) | ~548k lines |
| Clean build (server) | 36.9s |
| Build artifacts | ~21GB |
| Critical path bottleneck | naming (11s), pg_json_nbr (8.6s), pg_types_numeric (8.6s) |

## Already done

- pg_types split into 3 parallel crates (17.5s -> 7.5s)
- pg_json split into 2 parallel crates (9.9s -> 8.6s)
- Feature gates for pg_json dimensions (dim1..all-dims)
- `Subset` and `max_dim` config params in gen_pg_json_src
- `mod_name` config param for unique module names per sub-crate
- `Subset`, `WithDimOne`, `WithoutDims` variants in gen_pg_types_src

---

## 1. Compile time (high priority)

### 1.1 Reduce generated code volume

**Problem:** pg_types generates 363k lines, pg_json generates 185k lines even with WithDimOne. This is the root cause of slow compilation.

**Analysis of pg_types expanded code (363k lines):**

| Trait impl | Lines | Count | Avg | Can optimize? |
|-----------|-------|-------|-----|--------------|
| Serialize (derive) | 19,033 | 801 | 23.8 | No |
| PgType | 19,524 | 111 | 175.9 | Partially — extract common logic into runtime functions |
| Debug (derive) | 14,554 | 991 | 14.7 | No |
| Clone (derive) | 9,734 | 914 | 10.6 | No |
| PartialEq (derive) | 9,014 | 859 | 10.5 | No |
| sqlx::Type | 4,104 | 399 | 10.3 | Yes — blanket impl via marker trait |
| DfltSomeOneEl | 2,759 | 501 | 5.5 | Yes — blanket impl for newtypes |
| sqlx::Encode | 2,600 | 325 | 8.0 | Yes — blanket impl via Deref |
| sqlx::Decode | 2,499 | 214 | 11.7 | Yes — blanket impl via newtype trait |
| ToErrString | 2,300 | 416 | 5.5 | Yes — blanket impl for Display |

**Steps:**

1. **Blanket impl for DfltSomeOneEl on newtypes.** Pattern: `Self(DfltSomeOneEl::dflt_some_one_el())` repeats 500+ times. Add blanket `impl<T: DfltSomeOneEl> DfltSomeOneEl for NewType<T>` in pg_crud_cmn. Estimated saving: ~2,700 lines.

2. **Blanket impl for ToErrString.** Pattern: `impl ToErrString for X { fn to_err_string(&self) -> String { format!("{self:?}") } }` repeats 416 times. Add blanket `impl<T: Debug> ToErrString for T`. Estimated saving: ~2,300 lines.

3. **Centralize sqlx::Type/Encode/Decode.** All newtype wrappers (Tt, Cr, Rd, Upd) delegate to Orgn. Generate a marker trait `PgJsonNewtype` and provide blanket impls for sqlx traits. Estimated saving: ~9,200 lines.

4. **Reduce PgType impl size.** Each impl is ~176 lines. Extract common query-building logic into runtime helper functions called from generated code. Target: reduce to ~50 lines per impl. Estimated saving: ~14,000 lines.

**Total estimated saving: ~28,000 lines (~8% of pg_types).** The remaining ~335k lines are mostly derives which cannot be reduced.

### 1.2 naming crate optimization

**Problem:** `naming` crate takes 10-11s to compile — it's on the critical path.

**Steps:**

1. Profile `naming` crate: `cargo expand -p naming 2>/dev/null | wc -l` to check generated code volume.
2. If large, apply same splitting strategy as pg_types.
3. Check if `naming_macros` can be simplified or cached.

### 1.3 Cargo profile tuning

**Problem:** No release/dev profile tuning configured.

**Steps:**

1. Add to workspace Cargo.toml:
   ```toml
   [profile.dev]
   debug = 1          # reduced debug info (faster linking)

   [profile.dev.package."*"]
   opt-level = 2      # optimize dependencies (compile once, use many times)

   [profile.release]
   lto = "thin"
   codegen-units = 1
   strip = true
   ```

2. Measure impact on clean build and incremental build.

### 1.4 Split gen_pg_tbl_src (5,932 lines)

**Problem:** Single monolithic function generating all CRUD code. Compiles sequentially.

**Steps:**

1. Add `Subset` support to gen_pg_tbl_src (like pg_types/pg_json).
2. Split into per-operation crates if types are independent.
3. If types are cross-referencing, keep as single crate but reduce generated code.

### 1.5 Incremental build optimization

**Problem:** Touching one file in pg_crud recompiles everything downstream.

**Steps:**

1. Map dependency graph: `cargo tree -p server --depth 3 --edges normal`.
2. Identify unnecessary transitive dependencies.
3. Consider `#[cfg(feature = "test-utils")]` to avoid compiling test code in dev builds.
4. Move test-utils behind a separate crate rather than feature flags.

---

## 2. Workspace structure (medium priority)

### 2.1 Merge ultra-small crates

**Problem:** 13 crates under 50 lines add workspace complexity without benefit.

**Candidates:**

| Crate | Lines | Merge into |
|-------|-------|-----------|
| constants (6 lines) | 4 constants | cmn_routes |
| http_logic (4 lines) | 1 trait | route_validators |
| panic_loc (15 lines) | 1 function | macros_helpers |
| app_state (10 lines) | 1 trait | server_config |
| server_port_cmn (6 lines) | 2 constants | server_port |
| gen_pg_json_cmn (6 lines) | 1 function | pg_crud_macros_cmn |

**Steps:**

1. For each candidate, check reverse dependencies (`cargo tree --invert -p <crate>`).
2. Move code to target crate.
3. Update all dependent Cargo.toml files.
4. Remove merged crate from workspace.
5. Verify `cargo check` and `cargo clippy`.

### 2.2 Remove empty test_cnt crates

Four crates contain only empty `lib.rs`:
- gen_pg_json_obj_test_cnt
- gen_pg_json_test_cnt
- gen_pg_tbl_test_cnt
- gen_pg_types_test_cnt

**Steps:**

1. Check if they are used as dependencies anywhere.
2. If unused, remove from workspace.
3. If used as placeholders, document why.

---

## 3. Server improvements (medium priority)

### 3.1 Connection pool configuration

**Problem:** Pool max_connections is configurable via env but has no idle timeout, min connections, or acquire timeout settings.

**Steps:**

1. Add env vars: `PG_POOL_MIN_CONNECTIONS`, `PG_POOL_IDLE_TIMEOUT_SECS`, `PG_POOL_ACQUIRE_TIMEOUT_SECS`.
2. Apply to PgPoolOptions builder.
3. Set reasonable defaults (min: 5, idle_timeout: 300s, acquire_timeout: 30s).

### 3.2 Graceful shutdown improvements

**Problem:** Current shutdown only waits for Ctrl-C. No drain period for in-flight requests.

**Steps:**

1. Use `axum::serve(...).with_graceful_shutdown(shutdown_signal())`.
2. Add configurable drain timeout.
3. Close database pool on shutdown.

### 3.3 Health check endpoint

**Problem:** `/health_check` exists but doesn't verify database connectivity.

**Steps:**

1. Add `SELECT 1` query to health check handler.
2. Return 503 if database is unreachable.
3. Add `/readiness` endpoint that checks all dependencies.

### 3.4 OpenAPI/Swagger

**Problem:** utoipa dependency exists but OpenAPI spec is not generated (TODO in code).

**Steps:**

1. Add `#[derive(utoipa::ToSchema)]` to request/response types.
2. Generate OpenAPI spec from generated CRUD routes.
3. Serve Swagger UI at `/swagger-ui`.

---

## 4. Error handling (low priority)

### 4.1 Replace expect() with compile_error!() in proc-macros

**Problem:** 259 `expect()` calls in proc-macro code. When input is invalid, panic message is unhelpful.

**Priority targets (input validation):**

| File | Line | Description |
|------|------|------------|
| gen_pg_tbl_src/src/lib.rs | 438 | Missing primary key field |
| gen_pg_tbl_src/src/lib.rs | 2261 | Missing loc attribute |
| gen_pg_json_obj_src/src/lib.rs | 257 | Missing field ident |

**Steps:**

1. For each `expect()` that validates user input (struct attributes, field names, configs):
   - Replace with `match` + `return quote! { compile_error!("descriptive message"); }`.
2. Keep `expect()` for truly impossible states (already validated upstream).
3. Target: convert 30-50 highest-impact expect() calls.

### 4.2 Improve thiserror enum messages

**Problem:** Some error variants only carry `loc: Loc` without context about what failed.

**Steps:**

1. `QpEr::CheckedAdd` — add operand values to variant.
2. `NotEmptyUnqVecTryNewEr::IsEmpty` — add type context.
3. Audit remaining error enums for information completeness.

---

## 5. Testing (low priority)

### 5.1 Test infrastructure

**Problem:** Test coverage is unclear. Only test_cnt crates and feature-gated test-utils.

**Steps:**

1. Add `cargo-tarpaulin` or `cargo-llvm-cov` to CI for coverage measurement.
2. Establish baseline coverage.
3. Add integration tests for generated CRUD operations.
4. Consider `cargo-nextest` for faster parallel test execution (already mentioned in todo.md).

### 5.2 Snapshot tests for generated code

**Steps:**

1. Add `insta` crate for snapshot testing.
2. For each `gen_*_src` crate, add a test that:
   - Calls the macro with a minimal config.
   - Snapshots the generated TokenStream.
3. This prevents accidental changes to generated code structure.

---

## 6. CI/CD (low priority)

### 6.1 CI speed

**Problem:** CI runs fmt, clippy, doc, audit, test sequentially.

**Steps:**

1. Parallelize independent jobs (fmt + audit can run simultaneously with clippy + test).
2. Use `cargo-binstall` for faster tool installation.
3. Cache sccache artifacts between runs.
4. Split into matrix: `[check, test, lint]`.

### 6.2 Dependency security

**Steps:**

1. Add `cargo-deny` to CI (license + vulnerability checking).
2. Create `deny.toml` with license allowlist and advisory database.
3. Replace `cargo-audit` (currently used) with `cargo-deny` for unified policy.

---

## 7. Dependencies (low priority)

### 7.1 Feature trimming

**Already done:** Removed 9 unused features from reqwest, tower-http, tracing.

**Remaining:**

1. `syn = { features = ["full", "visit", "visit-mut", "extra-traits"] }` — check if all 4 features are needed in every proc-macro crate, or if some only need `"full"`.
2. `sqlx = { features = [...10 features...] }` — check if each crate needs all 10 features or a subset.

### 7.2 Dependency deduplication

**Steps:**

1. Run `cargo tree --duplicates` to find duplicate dependency versions.
2. Align versions where possible.
3. Target: zero duplicate crate versions.

---

## 8. Architecture (low priority)

### 8.1 Code generation architecture

**Problem:** Each gen_*_src crate is a single monolithic function (3,000-6,900 lines).

**Long-term steps:**

1. Extract helper types (enums, config structs) into separate modules within each gen_*_src crate.
2. Extract quote generation helpers into shared utilities in pg_crud_macros_cmn.
3. Consider code generation via build.rs instead of proc-macros for large outputs (avoids re-expansion on every compile).

### 8.2 Reduce proc-macro count

**Problem:** 29 proc-macro crates create compilation bottlenecks (proc-macros compile for host, not target).

**Steps:**

1. Merge thin proc-macro wrappers into their `_src` crates where possible.
2. Example: `gen_pg_json` (5 lines) could be merged into `gen_pg_json_src` by making it a proc-macro crate directly.
3. Target: reduce from 29 to ~20 proc-macro crates.

---

## Execution priority

| # | Item | Impact | Effort | Priority |
|---|------|--------|--------|----------|
| 1.3 | Cargo profile tuning | High | Low | Do first |
| 1.1 | Reduce generated code (blanket impls) | High | Medium | Do second |
| 1.2 | naming crate optimization | High | Medium | Do third |
| 2.1 | Merge ultra-small crates | Medium | Low | Do fourth |
| 3.1-3.4 | Server improvements | Medium | Medium | Do fifth |
| 1.4 | Split gen_pg_tbl_src | Medium | High | Evaluate later |
| 1.5 | Incremental build optimization | Medium | Medium | Evaluate later |
| 4-8 | Everything else | Low | Varies | As needed |
