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

---

## Phase 1: Reduce generated code volume (high impact)

### 1.1 Blanket impls for trivial traits

**DfltSomeOneEl** — needs marker trait `PgJsonNewtype` on generated wrapper types (Tt/Cr/Rd/Upd). Requires changes to DTsBuilder + all gen_*_src.

**ToErrString** — cannot use blanket `impl<T: Debug>` because existing impls use different formats (Display, Debug, custom message) for different types.

**sqlx::Type/Encode** — same marker trait approach as DfltSomeOneEl.

**Status:** Blocked — all require marker trait infrastructure on generated types. Medium-high effort.

### ~~1.2 Reduce schemars::JsonSchema verbosity~~

Cannot remove JsonSchema from Tt/Rd — they are used as fields in API structs (Cr, Upd, Sel) that derive JsonSchema. The PgJson trait requires `UtoipaToSchemaAndSchemarsJsonSchemaAl` bound on Tt and Rd.

---

## Phase 2: Simplify manual Deserialize impls (medium impact)

### 2.1 Create a derive macro for try_new + Deserialize pattern

5 remaining manual Deserialize impls use the same pattern:

| Type | Location | Lines |
|------|----------|-------|
| PgTypeWh<T> | pg_crud_cmn | ~150 |
| NotEmptyUnqVec<T> | pg_crud_cmn | ~80 |
| PgJsonNotEmptyUnqVec<T> | wh_flts | ~70 |
| PgTypeNotEmptyUnqVec<T> | wh_flts | ~70 |
| Btwn<T> | wh_flts | ~170 |

Create `#[derive(DeserializeViaTryFrom)]` proc-macro. Estimated saving: ~540 lines.

---

## Phase 3: Code generation architecture (high effort, high impact)

### ~~3.1 Split gen_pg_json_obj_src into modules~~

106 closures capturing shared scope — cannot extract to functions/modules without passing dozens of parameters. Would require complete rewrite.

### ~~3.2 Split gen_pg_tbl_src into modules~~

Same issue as 3.1 — deeply nested closures with shared captured state.

### ~~3.3 Evaluate build.rs for large generated code~~

Prototyped with pg_types_text_misc: 3.7s → 20.6s (5.6x slower). build.rs requires sequential compilation of gen_pg_types_src as build-dependency, blocking parallelism. Proc-macro approach is faster.

---

## Phase 4: Server improvements

### 4.1 OpenAPI generation

utoipa dependency exists, TODO comments in code, `/swagger-ui` constant defined but not functional.

### 4.2 Configurable CORS origins from environment

Verify multiple origins can be specified (comma-separated). Add documentation in .envexample.

---

## Phase 5: Testing

### 5.1 Coverage measurement

Add `cargo-llvm-cov`. Target: 70% coverage for pg_crud_cmn, wh_flts, route_validators.

### 5.2 Snapshot tests for generated code

Add `insta` dependency. Snapshot test comparing expanded output per gen_*_test crate.

### 5.3 Use cargo-nextest

Parallel test execution via cargo-nextest.

---

## Execution priority

| Phase | Item | Impact | Effort |
|-------|------|--------|--------|
| 1.2 | Reduce JsonSchema derives | -3s build | Medium |
| 3.1 | Split gen_pg_json_obj_src | maintainability | Medium |
| 3.3 | build.rs for generated code | -2-5s build | High |
| 2.1 | DeserializeViaTryFrom derive | -540 lines | High |
| 4.1 | OpenAPI generation | API docs | Medium |
| 5.1-5.3 | Testing | quality | Medium |
| 1.1 | Blanket impls (marker traits) | -8k lines | High |

## Constraints

- **No `unwrap()`** — use `expect()` with UUID prefix
- **No `unsafe`** — denied workspace-wide
- **No blank lines** between code
- **Alphabetical ordering** enforced by clippy for struct fields, enum variants, impl blocks
- **`#[serde(try_from)]` + explicit TryFrom** does not work in proc-macro generated code on nightly (blanket TryFrom conflict)
- **Generic types** cannot use `#[serde(try_from)]` when error type has trait bounds on T (e.g., `T: ToErrString`)
- **naming macro entries** cannot be split across crates (all entries in single macro call)
