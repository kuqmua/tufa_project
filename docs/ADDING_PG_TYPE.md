# Adding a new PostgreSQL type

## Steps

### 1. Define the type variant

In `pg_crud/pg_types/gen_pg_types_src/src/lib.rs`, the `RustTypeName` enum lists all supported Rust-to-Postgres type mappings. Add a new variant following the naming convention:

```
{RustType}AsNn{PgType}       # non-null
Opt{RustType}AsNl{PgType}    # nullable
VecOf{RustType}AsNnArrOfNn{PgType}  # non-null array of non-null
// ... and other nullability/array combinations
```

### 2. Add type mapping

In the same file, add the mapping in the match arms that convert between Rust types and PostgreSQL types. Key sections:
- SQL type name (e.g., `INT2`, `TEXT`, `TIMESTAMPTZ`)
- sqlx Rust type (e.g., `i16`, `String`, `sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>`)
- Whether the type supports `Copy`

### 3. Register in pg_types config

The type must be included in the config JSON passed to `#[derive(GenPgTypes)]`. This is typically in the test crate that exercises the macro.

### 4. Verify generated code

Use `ShouldWriteTsIntoFile::True` in the config to write generated code to a file for inspection:

```bash
cargo expand pg_types --lib  # or check the written file
```

### 5. Test

Run the pg_types tests:

```bash
cargo test -p gen_pg_types_test
```

## Naming convention

Types follow the pattern: `{RustType}As{Nullability}{PgType}{InitMethod}`

- **RustType**: The Rust standard/library type (e.g., `I16`, `String`, `SqlxTypesUuidUuid`)
- **Nullability**: `Nn` (non-null) or `Nl` (nullable, wrapped in `Option`)
- **PgType**: PostgreSQL type name (e.g., `Int2`, `Text`, `Uuid`)
- **InitMethod** (optional): `InitByPg` (server-generated, e.g., serial), `InitByClient`, `V4` (UUID v4)

Array types add `VecOf` prefix and `ArrOf` in the PG type section.
