# Refactoring plan

## Current state (after optimizations)

| Metric | Before | Now |
|--------|--------|-----|
| Clean build (server) | 42.6s | 32.3s (-24%) |
| Source code | ~92k lines | ~37k lines |
| Generated code (expanded) | ~693k lines | ~693k lines |
| Workspace crates | 77 | 71 |
| naming entries (lib.rs) | 684 | 350 |
| naming entries (prm.rs) | 372 | 60 |

### Top compilation bottlenecks

| Crate | Time | Type |
|-------|------|------|
| pg_json_nbr | 7.4s | generated (142k lines expanded) |
| pg_types_numeric | 7.3s | generated (141k lines expanded) |
| pg_types_chrono_net | 7.2s | generated (151k lines expanded) |
| gen_pg_json_obj_src | 5.7s | proc-macro impl (6.8k lines source) |
| sqlx-postgres | 5.6s | external dependency |
| gen_pg_tbl_src | 3.9s | proc-macro impl (5.9k lines source) |
| server_types | 3.8s | generated (78k lines expanded) |

### What was already done

- pg_types split into 3 parallel sub-crates
- pg_json split into 2 parallel sub-crates with `Subset`, `max_dim`, `mod_name` config
- Feature gates for pg_json dimensions
- `debug = 1` in dev profile
- Removed 3 unused crates (constants, http_logic, server_port_cmn)
- Removed 617 unused naming entries
- 6 default methods in PgJson trait
- `fi_jsonb_build_obj_v` moved to pg_crud_cmn
- 5x `#[serde(try_from)]` replacing manual Deserialize (ServerPort, PgnStartsWithZero, PgnStartsWithOne, UnsignedPartOfI32, NotZeroUnsignedPartOfI32)
- Simplified `gen_impl_de_for_struct_ts` (180 -> 30 lines, Raw struct + try_new)
- Simplified UmPayload Deserialize (Visitor -> direct deserialize + try_new)

---

## Phase 1: Reduce generated code volume (high impact)

### 1.1 Blanket impls for trivial traits

**Problem:** Each generated type gets its own impl for traits where the body is identical across all types.

**DfltSomeOneEl** — 2,020 lines in pg_json_nbr alone (420 impls):
```rust
// Pattern 1: Orgn types (unique per type)
impl DfltSomeOneEl for I8AsNnJsonbNbrOrgn {
    fn dflt_some_one_el() -> Self { Self(Default::default()) }
}
// Pattern 2: Wrapper types (always delegates)
impl DfltSomeOneEl for I8AsNnJsonbNbrTt {
    fn dflt_some_one_el() -> Self { Self(DfltSomeOneEl::dflt_some_one_el()) }
}
```

**Steps:**
1. Add marker trait `PgJsonNewtype` for Tt/Cr/Rd/Upd wrappers in pg_crud_cmn.
2. Add blanket `impl<T: DfltSomeOneEl, W: PgJsonNewtype<Inner=T>> DfltSomeOneEl for W`.
3. Stop generating individual DfltSomeOneEl impls for wrapper types in gen_pg_json_src.
4. Estimated saving: ~1,500 lines per sub-crate.

**ToErrString** — 630 lines (90 impls), all identical:
```rust
impl ToErrString for X { fn to_err_string(&self) -> String { format!("{self:?}") } }
```

**Steps:**
1. Change `ToErrString` in loc_lib to blanket `impl<T: Debug> ToErrString for T`.
2. Remove all generated `ToErrString` impls.
3. Estimated saving: ~630 lines per sub-crate.

**Note:** Blanket impls may conflict with existing specializations. Test carefully.

### 1.2 Consolidate sqlx::Type/Encode for newtype wrappers

**Problem:** 300 sqlx::Type impls (3,400 lines) and 300 sqlx::Encode impls (2,560 lines). Most delegate to inner type.

```rust
// Repeated 180 times:
impl sqlx::Type<Postgres> for XxxTt {
    fn compatible(ty: &...) -> bool { <XxxOrgn as sqlx::Type<Postgres>>::compatible(ty) }
    fn type_info() -> ... { <XxxOrgn as sqlx::Type<Postgres>>::type_info() }
}
```

**Steps:**
1. Add trait `SqlxDelegateType { type Inner; }` to pg_crud_cmn.
2. Add blanket `impl<T: SqlxDelegateType> sqlx::Type<Postgres> for T where T::Inner: sqlx::Type<Postgres>`.
3. Same for sqlx::Encode via Deref to inner.
4. Estimated saving: ~5,900 lines per sub-crate.

### 1.3 Reduce schemars::JsonSchema verbosity

**Problem:** 13,542 lines for JsonSchema impls — largest single trait. Each derive generates ~35 lines per struct.

**Steps:**
1. Investigate if `#[schemars(transparent)]` can be used for newtype wrappers.
2. If not, check if custom `impl JsonSchema` via delegation reduces code vs derive.
3. Consider removing JsonSchema from internal types (Tt, Cr, Rd, Upd, CrForQuery, UpdForQuery) — only Sel and Wh need schema for API.

---

## Phase 2: Simplify manual Deserialize impls (medium impact)

### 2.1 Create a derive macro for try_new + Deserialize pattern

**Problem:** 5 remaining manual Deserialize impls in source code use the same pattern: deserialize intermediate type -> validate via try_new/try_from.

| Type | Location | Lines | Inner type |
|------|----------|-------|------------|
| PgTypeWh<T> | pg_crud_cmn | ~150 | struct {fields: Option<PgTypeWh<T>>} |
| NotEmptyUnqVec<T> | pg_crud_cmn | ~80 | Vec<T> |
| PgJsonNotEmptyUnqVec<T> | wh_flts | ~70 | Vec<T> |
| PgTypeNotEmptyUnqVec<T> | wh_flts | ~70 | Vec<T> |
| Btwn<T> | wh_flts | ~170 | struct {start: T, end: T} |

**Steps:**
1. Create `#[derive(DeserializeViaTryFrom)]` proc-macro in macros_helpers.
2. Macro generates: deserialize inner type, call `try_from`, map error.
3. For newtypes: `#[deserialize_via(Vec<T>)]`
4. For structs: `#[deserialize_via(BtwnRaw<T>)]` with auto-generated Raw struct.
5. Replace all 5 manual impls.
6. Estimated saving: ~540 lines of boilerplate.

### 2.2 Reuse gen_impl_de_for_struct_ts in gen_pg_json_obj_src

**Problem:** gen_pg_json_obj_src (line 2622) has its own ~80 line manual Deserialize generation for `#ident_upd_ucc`, marked with `//todo mb reuse?`.

**Steps:**
1. Replace inline Deserialize generation at line 2622 with call to `gen_impl_de_for_struct_ts`.
2. Match the same pattern as gen_pg_tbl_src.

---

## Phase 3: Workspace structure (low impact, reduces complexity)

### 3.1 Merge app_state into server_config

**app_state** (10 lines) defines `GetPgPool` trait and re-exports config traits. server_config already depends on it.

**Steps:**
1. Move `GetPgPool` trait to server_config.
2. Update all `use app_state::` to `use server_config::`.
3. Remove app_state from workspace.

### 3.2 Evaluate merging panic_loc into macros_helpers

**panic_loc** (15 lines) — single function `panic_loc()` setting panic hook. Used by 2 proc-macro crates.

**Steps:**
1. Check if macros_helpers already depends on panic_loc.
2. If so, inline the function and remove the crate.
3. If not, consider whether the dependency direction makes sense.

### 3.3 Consider merging gen_pg_json_cmn into pg_crud_cmn

**gen_pg_json_cmn** (6 lines) has one function `fi_jsonb_build_obj_v` which is already duplicated in `pg_crud_cmn`.

**Steps:**
1. Replace all `gen_pg_json_cmn::fi_jsonb_build_obj_v` calls with `pg_crud_cmn::fi_jsonb_build_obj_v`.
2. Remove gen_pg_json_cmn from workspace.
3. Update pg_json_nbr and pg_json_other Cargo.toml.

---

## Phase 4: Code generation architecture (high effort, high impact)

### 4.1 Split gen_pg_json_obj_src into modules

**Problem:** 6,820 lines in single lib.rs (5.7s compile time). Already has types.rs and cfg.rs modules but they contain only ~120 lines.

**Steps:**
1. Extract helper enums and config structs (~200 lines).
2. Extract WHERE clause generation (~1,500 lines).
3. Extract sqlx impl generation (~800 lines).
4. Extract test case generation (~1,000 lines).
5. Keep main orchestration in lib.rs (~3,300 lines).

### 4.2 Split gen_pg_tbl_src into modules

**Problem:** 5,932 lines in single lib.rs (3.9s compile time). Single function `gen_pg_tbl()`.

**Steps:**
1. Extract `Op` enum and related types into `types.rs`.
2. Extract per-operation (Cm, Co, Rm, Ro, Um, Uo, Dm, Dlo) generation into separate modules.
3. Extract common helpers (field generation, query building) into `helpers.rs`.

### 4.3 Evaluate build.rs for large generated code

**Problem:** pg_types generates 363k lines, pg_json generates 185k lines. Proc-macro re-expansion happens on every compilation of dependent crates.

**Alternative:** Generate code via build.rs into `OUT_DIR`, include via `include!`. Code is generated once and cached until inputs change.

**Steps:**
1. Prototype with one small sub-crate (pg_types_text_misc).
2. Move `gen_pg_types!` invocation into build.rs.
3. In lib.rs: `include!(concat!(env!("OUT_DIR"), "/generated.rs"));`
4. Measure compile time difference.
5. If beneficial, apply to all gen_* crates.

**Risk:** build.rs runs before compilation, so proc-macro crate still needs to compile first. Main benefit: avoids re-expansion when only downstream code changes.

---

## Phase 5: Server improvements (independent of code generation)

### 5.1 Health check with database

**Current:** `/health_check` returns 200 OK without checking anything.

**Steps:**
1. Add `SELECT 1` query to health check handler.
2. Return 503 if database unreachable.

### 5.2 OpenAPI generation

**Current:** utoipa dependency exists, TODO comments in code, `/swagger-ui` constant defined but not functional.

**Steps:**
1. Add `#[utoipa::path]` annotations to route handlers.
2. Generate OpenAPI spec from `utoipa::OpenApi`.
3. Serve swagger-ui at `/swagger-ui` endpoint.

### 5.3 Configurable CORS origins from environment

**Current:** CORS origins configurable via env var.

**Steps:**
1. Verify multiple origins can be specified (comma-separated).
2. Add documentation in .envexample.

---

## Phase 6: Testing (independent)

### 6.1 Coverage measurement

**Steps:**
1. Add `cargo-llvm-cov` to CI.
2. Measure baseline coverage.
3. Target: 70% coverage for pg_crud_cmn, wh_flts, route_validators.

### 6.2 Snapshot tests for generated code

**Steps:**
1. Add `insta` dependency.
2. For each gen_*_test crate, add snapshot test comparing expanded output.
3. Prevents accidental changes to generated API.

### 6.3 Use cargo-nextest

**Steps:**
1. Add `cargo-nextest` to CI for parallel test execution.
2. Configure in `.config/nextest.toml`.

---

## Execution priority

| Phase | Item | Impact on build | Effort | Priority |
|-------|------|----------------|--------|----------|
| 1.1 | Blanket DfltSomeOneEl + ToErrString | -2s | Low | 1 |
| 1.2 | Blanket sqlx::Type/Encode | -3s | Medium | 2 |
| 2.2 | Reuse gen_impl_de_for_struct_ts in pg_json_obj | cleanup | Low | 3 |
| 3.3 | Merge gen_pg_json_cmn | -1 crate | Low | 4 |
| 1.3 | Reduce JsonSchema derives | -3s | Medium | 5 |
| 4.1 | Split gen_pg_json_obj_src | maintainability | Medium | 6 |
| 4.3 | build.rs for generated code | -2-5s | High | 7 |
| 2.1 | DeserializeViaTryFrom derive | cleanup | High | 8 |
| 5.1-5.3 | Server improvements | - | Medium | 9 |
| 6.1-6.3 | Testing | - | Medium | 10 |

## Constraints

- **No `unwrap()`** — use `expect()` with UUID prefix
- **No `unsafe`** — denied workspace-wide
- **No blank lines** between code
- **Alphabetical ordering** enforced by clippy for struct fields, enum variants, impl blocks
- **`#[serde(try_from)]` + explicit TryFrom** does not work in proc-macro generated code on nightly (blanket TryFrom conflict)
- **Generic types** cannot use `#[serde(try_from)]` when error type has trait bounds on T (e.g., `T: ToErrString`)
- **naming macro entries** cannot be split across crates (all entries in single macro call)
