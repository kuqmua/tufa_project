# Plan: compile time optimization for gen_pg_json

## Problem

`gen_pg_json::gen_pg_json!` with `"vrt": "All"` generates ~400,000 lines of code from 806 type variants. This is the main compilation bottleneck (8.7s for `pg_json` crate alone, 16.7s for `pg_types`).

### Variant count breakdown

| Pattern | Records | % of total |
|---------|---------|------------|
| Stdrt (no arrays) | 26 | 3% |
| ArrDim1 | 52 | 6% |
| ArrDim2 | 104 | 13% |
| ArrDim3 | 208 | 26% |
| ArrDim4 | 416 | 52% |
| **Total** | **806** | **100%** |

13 base types x (2 + 4 + 8 + 16 + 32) nullability combinations = 806 variants.

Dim3+Dim4 = 624 variants = **77% of total generated code**.

Each variant generates ~496 lines (newtypes, sqlx impls, serde, traits, WHERE filters).

## Optimization strategies

### Strategy 1: Split into per-dimension crates (highest impact, medium effort)

Split `pg_json` into multiple crates, one per dimension level:

```
pg_crud/pg_json/          -> pg_json_stdrt (26 variants)
pg_crud/pg_json_dim1/     -> pg_json_dim1 (52 variants)
pg_crud/pg_json_dim2/     -> pg_json_dim2 (104 variants)
pg_crud/pg_json_dim3/     -> pg_json_dim3 (208 variants)
pg_crud/pg_json_dim4/     -> pg_json_dim4 (416 variants)
```

**Benefits:**
- Parallel compilation of all 5 crates (currently serial: one macro call)
- Incremental compilation: changing dim1 doesn't recompile dim4
- With 5 parallel crates, wall-clock time drops from ~9s to ~4-5s (dim4 is bottleneck)
- Can `#[cfg(feature)]` gate higher dimensions for dev builds

**Implementation:**
1. Each crate calls `gen_pg_json!` with `ConfigVrt::Concrete(...)` listing only its dimension
2. A parent `pg_json` crate re-exports all via `pub use pg_json_stdrt::*; pub use pg_json_dim1::*;` etc.
3. `pg_crud` re-exports everything as before

**Risk:** Medium. Need to verify no cross-dimension dependencies in generated code.

### Strategy 2: Feature-gate higher dimensions (highest impact for dev, low effort)

Add features to `pg_json` to optionally include higher dimensions:

```toml
[features]
default = ["dim1"]
dim1 = []
dim2 = ["dim1"]
dim3 = ["dim2"]
dim4 = ["dim3"]
all-dims = ["dim4"]
```

In `pg_json/src/lib.rs`:
```rust
gen_pg_json::gen_pg_json!({
    "vrt": "WithoutDims"  // always: 26 variants
});
#[cfg(feature = "dim1")]
gen_pg_json::gen_pg_json!({
    "vrt": { "Concrete": [...dim1 only...] }
});
// ...etc
```

**Benefits:**
- `cargo build` in dev: only Stdrt+Dim1 = 78 variants (~39k lines vs 400k)
- CI/release: `--features all-dims` for full type coverage
- 10x reduction in generated code for typical dev workflow
- Minimal code changes

**Drawback:** Multiple macro invocations might not work (need to check for duplicate type definitions). May need Strategy 1 instead.

### Strategy 3: Move generation to build.rs (medium impact, high effort)

Replace proc-macro with `build.rs` that writes generated code to `$OUT_DIR`:

```rust
// build.rs
fn main() {
    let code = gen_pg_json_src::gen_pg_json(&input);
    std::fs::write(format!("{}/generated.rs", std::env::var("OUT_DIR").unwrap()), code);
}

// lib.rs
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
```

**Benefits:**
- `build.rs` output is cached by cargo (not rerun unless inputs change)
- Proc-macro expansion happens every compile; build.rs only on dependency changes
- Can split output into multiple files for parallel compilation

**Drawback:** Big refactor. Proc-macro dependencies need restructuring. Generated code becomes harder to debug (no `cargo expand`).

### Strategy 4: Reduce generated code per variant (medium impact, medium effort)

Analyze the ~496 lines generated per variant. Look for:

1. **Shared impls via blanket traits**: Many generated impls may be structurally identical (e.g., `sqlx::Encode` for all Nn types). Extract a generic impl with a trait bound instead of generating N copies.

2. **Macro-based deduplication**: Instead of generating full impl blocks, generate macro invocations that expand to the impl. The macro is compiled once; invocations are cheaper.

3. **Reduce WhFlt variants**: WHERE filter types are generated per JSON type. If filters are structurally identical across types, use generics.

**Example:** Instead of generating:
```rust
impl sqlx::Encode<'_, sqlx::Postgres> for I8AsJsonbNbr { ... }
impl sqlx::Encode<'_, sqlx::Postgres> for I16AsJsonbNbr { ... }
// ...repeated 806 times
```

Generate a blanket impl:
```rust
impl<T: AsJsonbValue> sqlx::Encode<'_, sqlx::Postgres> for T { ... }
```

**Benefits:** Reduces generated code from ~400k to potentially ~100-150k lines.

**Drawback:** Requires deep understanding of what each generated impl does. Some impls genuinely differ per type.

### Strategy 5: Parallelize macro expansion (already partially done)

The macro already uses `rayon::par_iter()` for parallel code generation. This helps macro execution time but NOT the compilation of the resulting 400k lines by rustc.

**Already implemented.**

## Status

### Strategy 2: DONE

Feature gates added to `pg_json` and `pg_types`. Controlled via `default = ["dim1"]`, `all-dims` feature.

**Result:** Feature gates work for All vs WithDimOne switching, but even `WithDimOne` generates massive code:
- `pg_types` WithDimOne = **363,634 lines** (17.5s)
- `pg_json` WithDimOne = **184,666 lines** (9.3s)

The bottleneck is not higher dimensions — it's the base variant count. Even Stdrt alone generates ~2,370 lines per base type x 30+ types.

### Next steps

1. **Strategy 4** (reduce code per variant) — highest impact now. Analyze what's duplicated in generated impls. Replace N identical impls with blanket trait impls.
2. **Strategy 1** (split crates) — enables parallel compilation of Stdrt and ArrDim1 crates.
3. **Strategy 3** (build.rs) — only if strategies 1+4 are insufficient.

## Estimated impact

| Strategy | Dev build time | CI build time | Effort |
|----------|---------------|---------------|--------|
| Current | ~9s pg_json | ~9s | - |
| Strategy 1 (split crates) | ~5s (parallel) | ~5s | 2-3 days |
| Strategy 2 (feature gates) | ~1-2s (dim0+dim1 only) | ~9s (all) | 1 day |
| Strategy 4 (reduce codegen) | ~3-4s | ~3-4s | 1-2 weeks |
| Strategy 1+2 combined | ~1-2s dev, ~3s CI | ~3s | 3-4 days |

## Same approach for pg_types

`pg_types` has the identical problem (16.7s compile). The same strategies apply — split by dimension or feature-gate.
