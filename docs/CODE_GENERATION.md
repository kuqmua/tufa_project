# Code generation architecture

## Overview

The `pg_crud/` subsystem generates Rust types and CRUD endpoint code from struct definitions annotated with derive macros. Each PostgreSQL type, JSON object, and table gets a full set of generated types for create, read, update, delete, query building, and WHERE clause filtering.

## Crate structure pattern

Each subsystem follows a three-layer pattern:

```
gen_<name>/          # proc-macro crate (thin wrapper, ~5-10 lines)
  -> calls gen_<name>_src::gen_<name>()

gen_<name>_src/      # implementation crate (the actual code generation logic)
  -> returns TokenStream

gen_<name>_test/     # test crate (applies the macro and runs tests)
gen_<name>_test_cnt/ # test content (shared test data)
```

## Subsystems and their roles

### pg_types (`pg_crud/pg_types/`)
Generates Rust wrapper types for PostgreSQL column types.

**Input**: Enum variants like `I16AsNnInt2`, `OptI32AsNlInt4`, `VecOfStringAsNnArrOfNnText`
**Output**: Newtype structs with:
- `sqlx::Encode`, `sqlx::Decode`, `sqlx::Type` impls
- Serde serialization/deserialization
- `PgType` trait impl (for query building)
- `WhFlt` trait impl (for WHERE clause filters)
- `new()` / `try_new()` constructors
- `DfltSomeOneEl` impl (default pagination element)

### pg_json (`pg_crud/pg_json/`)
Generates types for JSONB column handling.

**Input**: Enum variants describing JSON column types with nullability and array dimensions
**Output**: Similar to pg_types but for JSONB-stored values, including:
- JSON serialization round-trip
- Nested field access
- Array dimension handling

**Shared logic**: `gen_pg_json_cmn/` contains helpers shared between pg_json and pg_json_obj.

### pg_json_obj (`pg_crud/pg_json_obj/`)
Generates types for structured JSON objects stored in JSONB columns.

**Input**: Struct definition with typed fields (e.g., `Animal { field_0: i32, field_1: i32 }`)
**Output**: Full type hierarchy:
- `Tt` (total type) - the stored struct
- `Cr` (create) - fields needed for creation
- `CrForQuery` - create payload bound to SQL query
- `Rd` (read) - read result type
- `RdIds` - read with only IDs
- `RdInn` (read inner) - inner read result
- `Upd` (update) - partial update payload
- `UpdForQuery` - update payload bound to SQL query
- `Sel` (select) - column selection specification
- `Wh` (where) - WHERE clause filter types
- `WithId` variants for array elements with UUID identifiers

### pg_tbl (`pg_crud/pg_tbl/`)
Generates complete CRUD endpoints for a table.

**Input**: Struct annotated with `#[derive(GenPgTbl)]` and operation attributes
**Output**:
- Axum route handlers: `cm` (create many), `co` (create one), `rm` (read many), `ro` (read one), `um` (update many), `uo` (update one), `dm` (delete many), `dlo` (delete list one)
- Request/response types for each operation
- SQL query builders with parameterized queries
- Error enums per operation (via `#[pg_crud::cm_er_vrts{...}]` etc.)
- `routes()` function returning an Axum `Router`
- `prep_pg()` for table creation/migration

### wh_flts (`pg_crud/wh_flts/`)
Generates WHERE clause filter types.

**Input**: Config describing filter operations per column type
**Output**: Filter enums with variants like `Equal`, `In`, `GreaterThan`, `Between`, `Regex`, `LengthEqual`, `ContainsAllElsOfArr`, etc.

## Shared crates

| Crate | Purpose |
|-------|---------|
| `pg_crud_macros_cmn` | Shared proc-macro utilities: sqlx encode/decode generators, DTsBuilder patterns, query part helpers |
| `pg_crud_cmn` | Runtime types shared across generated code: `QpEr`, pagination types, `NotEmptyUnqVec` |
| `pg_crud_cmn_and_macros_cmn` | Types needed by both runtime and compile-time code |
| `pg_json_obj_cmn` | Runtime types for JSON object handling |
| `pg_types_cmn` | Runtime types for PostgreSQL type wrappers |

## Code generation flow

```
User struct definition (e.g., TblExample)
  |
  v
#[derive(GenPgTbl)] proc-macro
  |
  v
gen_pg_tbl_src::gen_pg_tbl() parses DeriveInput
  |
  +-> Reads config from #[pg_crud::gen_pg_tbl_config{...}]
  +-> Iterates over fields, determines PG types
  +-> For each CRUD operation, generates:
  |     - Handler function
  |     - Request/response types
  |     - Query builder (QP + QB types)
  |     - Error enum
  +-> Generates routes() function
  +-> Generates prep_pg() for DDL
  |
  v
TokenStream returned to compiler
```

## Key patterns

### DTsBuilder
Factory for `#[derive(...)]` attribute generation. Used to conditionally add derives like `Copy`, `Serialize`, `thiserror::Error`:

```rust
DTsBuilder::new()
    .make_pub()
    .d_debug()
    .d_clone()
    .d_copy_if(derive_copy)
    .d_serde_serialize()
    .d_serde_deserialize()
    .build_struct(&additional_attrs, &ident, &generics, &fields)
```

### Import enum
Controls whether generated code imports from `pg_crud::` (for library consumers) or uses local paths (for tests).

### ShouldWriteTsIntoFile
Config flag controlling whether generated TokenStream is also written to a `.rs` file for inspection. Used during development to debug generated code.
