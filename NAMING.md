# Naming conventions

## Suffixes
- `_ts` TokenStream
- `_sc` snake case string
- `_ucc` upper camel case string
- `_dq` double-quoted string
- `_h` handle

## Domain abbreviations

### CRUD operations
- `cr` create
- `rd` read
- `upd` update
- `del` delete
- `sel` select

### Query building
- `qb` query bind
- `qp` query part
- `wh` where (filter context)
- `flt` filter
- `fq` for_query
- `prm` param (single parameter)
- `prms` params (parameters, plural)
- `pld` payload
- `pgn` pagination
- `pk` primary key
- `col` column

### Comparison operators
- `eq` equal
- `gt` greater_than
- `lt` less_than
- `btwn` between
- `rgx` regex

### Type system
- `nn` non-null
- `nl` nullable
- `tt` table_type
- `tbl` table
- `rng` range
- `unq` unique
- `ne` not_empty
- `arr` array
- `opt` optional (Option)
- `wi` with_id

### Code generation
- `gen` generate
- `mb` maybe (optional generated block)
- `acc` accumulator
- `dcl` declaration
- `init` initialization
- `incr` increment
- `dflt` default
- `inn` inner
- `st` struct
- `txn` transaction
- `msg` message

### Identifiers & types
- `fi` field (in iteration context)
- `ft` field type
- `flds` fields (plural, non-iteration)
- `vrt` variant
- `ident` syn::Ident
- `loc` location
- `er` error
- `op` operation
- `nbr` number
- `orgn` origin
- `cmn` common
- `len` length
- `eq` equal
- `crnt` current
- `oprtr` operator
- `stdrt` standard (standard pattern, non-array)
- `updd` updated (past tense, e.g., `SelOnlyUpddIds`)
- `crd` created (past tense, e.g., `SelOnlyCrdIds`)
- `al` alias (type alias)
- `dim` dimension (array dimension)
- `el` element
