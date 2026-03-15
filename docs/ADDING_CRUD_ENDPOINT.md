# Adding a new CRUD endpoint (table)

## Steps

### 1. Create the table struct crate

Create a new crate (e.g., `server_tbl_users/`) with a struct annotated with `#[derive(GenPgTbl)]`:

```rust
use optml::Optml;

#[derive(Debug, Clone, Copy, pg_crud::GenPgTbl, Optml)]
#[pg_crud::gen_pg_tbl_config{{
    "tests_write_into_file": "False",
    "cmn_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[pg_crud::cm_er_vrts{enum CmErVrts{}}]
#[pg_crud::co_er_vrts{enum CoErVrts{}}]
#[pg_crud::rm_er_vrts{enum RmErVrts{}}]
#[pg_crud::ro_er_vrts{enum RoErVrts{}}]
#[pg_crud::um_er_vrts{enum UmErVrts{}}]
#[pg_crud::uo_er_vrts{enum UoErVrts{}}]
#[pg_crud::dm_er_vrts{enum DmErVrts{}}]
#[pg_crud::dlo_er_vrts{enum DloErVrts{}}]
#[pg_crud::cmn_er_vrts{enum CmnErVrts{}}]
#[pg_crud::cm_logic{}]
#[pg_crud::co_logic{}]
#[pg_crud::rm_logic{}]
#[pg_crud::ro_logic{}]
#[pg_crud::um_logic{}]
#[pg_crud::uo_logic{}]
#[pg_crud::dm_logic{}]
#[pg_crud::dlo_logic{}]
#[pg_crud::cmn_logic{}]
pub struct TblUsers {
    #[gen_pg_tbl_pk]
    pub pk_col: pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg,
    pub name: pg_crud::StringAsNnText,
    pub email: pg_crud::StringAsNnText,
}
```

### 2. Add to workspace

In root `Cargo.toml`, add the new crate to workspace members and dependencies.

### 3. Register routes in server

In `server/src/main.rs`, merge the new table's routes:

```rust
use server_tbl_users::TblUsers;

// In the async block:
TblUsers::prep_pg(&pg_pool).await.expect("...");

let api_routes = Router::new()
    .merge(cmn_routes(...))
    .merge(TblExample::routes(...))
    .merge(TblUsers::routes(Arc::clone(&app_state)));  // add this
```

### 4. Generated endpoints

The macro generates these endpoints under `/{table_name_snake_case}/`:

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/cm` | POST | Create multiple rows |
| `/co` | POST | Create one row |
| `/rm` | POST | Read multiple (with WHERE filters) |
| `/ro` | POST | Read one |
| `/um` | PATCH | Update multiple |
| `/uo` | PATCH | Update one |
| `/dm` | DELETE | Delete multiple |
| `/dlo` | DELETE | Delete list one |
| `/*_payload_example` | GET | Example request payload |

### 5. Error variants

Each operation has its own error enum (`CmErVrts`, `RmErVrts`, etc.). Add custom error variants for business logic validation:

```rust
#[pg_crud::co_er_vrts{
    enum CoErVrts {
        EmailAlreadyExists {
            #[eo_to_err_string_serde]
            email: String,
            loc: loc_lib::loc::Loc,
        },
    }
}]
```

### 6. Custom logic

The `#[pg_crud::co_logic{}]` attributes accept custom logic closures that run before/after the SQL operation.

### 7. Test

```bash
cargo test -p server_tbl_users
```
