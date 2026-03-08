//todo
use proc_macro2::TokenStream as Ts2;
use quote::ToTokens;
naming_macros::gen_self_ucc_and_sc_str_and_ts!([
    ["self", "params"],
    ["self", "payload"],
    ["self", "payload", "with", "serde"],
    [
        "self", "payload", "try", "from", "self", "payload", "with", "serde"
    ],
    [
        "self", "payload", "with", "serde", "try", "from", "self", "payload"
    ],
    [
        "self", "payload", "with", "serde", "try", "from", "self", "payload", "er"
    ],
    ["try", "self"],
    ["try", "self", "res", "vrts"],
    ["self", "payload", "el", "with", "serde"],
    ["self", "payload", "el"],
    [
        "self", "payload", "el", "try", "from", "self", "payload", "with", "serde"
    ],
    [
        "self", "payload", "el", "try", "from", "self", "payload", "el", "with", "serde", "er"
    ],
    ["try", "self", "er"],
    ["try", "self", "req", "er"],
    [
        "self", "payload", "try", "from", "self", "payload", "with", "serde", "er"
    ],
    ["try", "self", "with", "serde"],
    ["tvfrr", "extraction", "logic", "try", "self"],
    ["try", "self", "generated", "logic", "er"],
    ["try", "self", "generated", "logic", "desirable"],
    ["try", "self", "logic"],
    ["try", "self", "logic", "res", "vrts"],
    ["try", "self", "logic", "er"],
    ["self", "er", "with", "serde"],
    ["try", "self", "generated", "logic", "er", "with", "serde"],
    ["self", "payload", "example"],
    ["self", "create"],
    ["self", "read"],
    ["self", "read", "inner"],
    ["self", "upd"],
    ["self", "upd", "for", "query"],
    ["self", "del"],
    ["obj", "self"],
    ["std", "opt", "opt", "obj", "self"],
    ["obj", "with", "id", "self"],
    ["std", "vec", "vec", "obj", "with", "id", "self"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self"
    ],
    ["self", "select"],
    ["self", "select", "without", "id"],
    ["self", "select", "with", "id"],
    ["obj", "self", "select"],
    ["obj", "with", "id", "self", "select"],
    ["std", "opt", "opt", "obj", "self", "select"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "select"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "select"
    ],
    ["self", "try", "new", "er"],
    ["obj", "self", "read"],
    ["std", "opt", "opt", "obj", "self", "read"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "read"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "read"
    ],
    ["obj", "self", "to", "create"],
    ["std", "opt", "opt", "obj", "self", "to", "create"],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "to", "create"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "to", "create"
    ],
    ["obj", "with", "id", "self", "read"],
    ["obj", "with", "id", "self", "to", "create"],
    ["obj", "self", "reader"],
    ["obj", "with", "id", "self", "reader"],
    ["std", "opt", "opt", "obj", "self", "reader"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "reader"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "reader"
    ],
    ["self", "reader"],
    ["std", "opt", "opt", "obj", "self", "to", "create", "origin"],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "to", "create", "origin"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "to", "create",
        "origin"
    ],
    ["std", "opt", "opt", "obj", "self", "read", "origin"],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "read", "origin"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "read", "origin"
    ],
    ["self", "opt", "to", "upd"],
    ["self", "opt", "to", "upd", "origin"],
    ["obj", "self", "opt", "to", "upd"],
    ["obj", "self", "opt", "to", "upd", "origin"],
    [
        "std", "opt", "opt", "obj", "self", "opt", "to", "upd", "origin"
    ],
    ["std", "opt", "opt", "obj", "self", "opt", "to", "upd"],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd", "origin"
    ],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd",
        "origin"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd"
    ],
    ["obj", "with", "id", "self", "opt", "to", "upd"],
    ["self", "upd", "with", "id"],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "json", "arr", "change"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "json", "arr",
        "change"
    ],
    [
        "self", "opt", "to", "upd", "try", "gen", "pg", "json", "type", "er"
    ],
    [
        "self", "opt", "to", "upd", "try", "gen", "pg", "json", "type", "er", "with", "serde"
    ],
    ["self", "json", "arr", "change"],
    ["self", "to", "create", "origin"],
    ["self", "to", "create", "with", "generated", "id"],
    ["self", "to", "create", "without", "generated", "id"],
    ["self", "json", "arr", "change", "try", "gen", "er"],
    ["self", "field", "to", "upd"],
    ["self", "gen", "pg", "json", "type", "to", "read", "er"],
    [
        "self", "try", "gen", "json", "arr", "el", "upd", "bind", "incrs", "er"
    ],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "json", "arr", "change", "try", "new",
        "er"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "json", "arr",
        "change", "try", "new", "er"
    ],
    ["not", "unique", "field", "self"],
    ["self", "read", "without", "id"],
    ["self", "read", "with", "id"],
    ["self", "opt", "to", "upd", "try", "new", "er"],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "read", "try", "new", "er"
    ],
    [
        "self", "gen", "pg", "json", "type", "to", "read", "from", "vec", "er"
    ],
    [
        "std", "opt", "opt", "obj", "self", "opt", "to", "upd", "try", "new", "er"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "read", "try",
        "new", "er"
    ],
    [
        "self", "read", "with", "or", "without", "id", "try", "from", "er"
    ],
    ["self", "select", "try", "new", "er"],
    ["obj", "self", "select", "try", "new", "er"],
    [
        "std", "opt", "opt", "obj", "self", "select", "try", "new", "er"
    ],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "select", "try", "new", "er"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "select", "try",
        "new", "er"
    ],
    ["obj", "self", "opt", "to", "upd", "try", "gen", "er"],
    [
        "obj", "self", "opt", "to", "upd", "try", "gen", "pg", "json", "type", "er", "with",
        "serde"
    ],
    [
        "std", "opt", "opt", "obj", "self", "opt", "to", "upd", "try", "gen", "er"
    ],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd", "try", "gen", "pg",
        "json", "type", "er"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd",
        "try", "gen", "pg", "json", "type", "er"
    ],
    ["self", "with", "serde"],
    ["self", "try", "from", "env", "er"],
    ["get", "self"],
    ["try", "from", "std", "env", "var", "ok", "self", "er"],
    ["self", "opts"],
    ["er", "self"],
    ["not", "unique", "self"],
    ["is", "self", "upd", "exist"],
    ["self", "column", "read", "permission"],
    ["self", "where"],
    ["std", "opt", "opt", "self"],
    ["where", "std", "opt", "opt", "self"],
    ["sqlx", "types", "json", "obj", "self"],
    ["std", "opt", "opt", "sqlx", "types", "json", "obj", "self"],
    ["sqlx", "types", "json", "std", "opt", "opt", "obj", "self"],
    [
        "std", "opt", "opt", "sqlx", "types", "json", "std", "opt", "opt", "obj", "self"
    ],
    [
        "sqlx", "types", "json", "std", "vec", "vec", "obj", "with", "id", "self"
    ],
    [
        "std", "opt", "opt", "sqlx", "types", "json", "std", "vec", "vec", "obj", "with", "id",
        "self"
    ],
    [
        "sqlx", "types", "json", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id",
        "self"
    ],
    [
        "std", "opt", "opt", "sqlx", "types", "json", "std", "opt", "opt", "std", "vec", "vec",
        "obj", "with", "id", "self"
    ],
    ["obj", "self", "column"],
    ["std", "opt", "opt", "obj", "self", "column"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "column"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "column"
    ],
    ["pg", "json", "type", "self", "to", "create"],
    ["obj", "self", "create"],
    ["std", "opt", "opt", "obj", "self", "create"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "create"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "create"
    ],
    ["pg", "json", "type", "self", "select"],
    ["pg", "json", "type", "obj", "self", "select"],
    [
        "pg", "json", "type", "std", "opt", "opt", "obj", "self", "select"
    ],
    [
        "pg", "json", "type", "std", "vec", "vec", "obj", "with", "id", "self", "select"
    ],
    [
        "pg", "json", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id",
        "self", "select"
    ],
    ["pg", "json", "type", "self", "select", "try", "new", "er"],
    [
        "pg", "json", "type", "obj", "self", "select", "try", "new", "er"
    ],
    [
        "pg", "json", "type", "std", "opt", "opt", "obj", "self", "select", "try", "new", "er"
    ],
    [
        "pg", "json", "type", "std", "vec", "vec", "obj", "with", "id", "self", "select", "try",
        "new", "er"
    ],
    [
        "pg", "json", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id",
        "self", "select", "try", "new", "er"
    ],
    ["pg", "json", "type", "self", "read"],
    ["pg", "json", "type", "self", "read", "without", "id"],
    ["pg", "json", "type", "self", "read", "with", "id"],
    ["self", "upd", "el"],
    ["pg", "json", "type", "self", "opt", "to", "upd"],
    ["self", "upd", "er"],
    ["self", "upd", "try", "new", "er"],
    ["obj", "self", "upd"],
    ["std", "opt", "opt", "obj", "self", "upd"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "upd"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "upd"
    ],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd", "try", "gen", "er"
    ],
    [
        "pg", "json", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id",
        "self", "opt", "to", "upd", "try", "gen", "pg", "json", "type", "er"
    ],
    [
        "pg", "json", "type", "obj", "self", "opt", "to", "upd", "try", "gen", "pg", "json",
        "type", "er", "with", "serde"
    ],
    [
        "pg", "json", "type", "self", "opt", "to", "upd", "try", "gen", "pg", "json", "type", "er",
        "with", "serde"
    ],
    [
        "pg", "json", "type", "self", "read", "with", "or", "without", "id", "try", "from", "er"
    ],
    ["pg", "type", "obj", "self"],
    ["pg", "type", "std", "opt", "opt", "obj", "self"],
    [
        "pg", "type", "std", "vec", "vec", "obj", "with", "id", "self"
    ],
    [
        "pg", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self"
    ],
    ["pg", "type", "obj", "self", "to", "create"],
    [
        "pg", "type", "std", "opt", "opt", "obj", "self", "to", "create"
    ],
    [
        "pg", "type", "std", "vec", "vec", "obj", "with", "id", "self", "to", "create"
    ],
    [
        "pg", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "to",
        "create"
    ],
    ["pg", "type", "obj", "self", "to", "read"],
    [
        "pg", "type", "std", "opt", "opt", "obj", "self", "to", "read"
    ],
    [
        "pg", "type", "std", "vec", "vec", "obj", "with", "id", "self", "to", "read"
    ],
    [
        "pg", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "to",
        "read"
    ],
    ["pg", "type", "obj", "self", "to", "upd"],
    [
        "pg", "type", "std", "opt", "opt", "obj", "self", "to", "upd"
    ],
    [
        "pg", "type", "std", "vec", "vec", "obj", "with", "id", "self", "to", "upd"
    ],
    [
        "pg", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "to",
        "upd"
    ],
    ["pg", "type", "obj", "self", "where"],
    ["pg", "type", "std", "opt", "opt", "obj", "self", "where"],
    [
        "pg", "type", "std", "vec", "vec", "obj", "with", "id", "self", "where"
    ],
    [
        "pg", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self",
        "where"
    ],
    ["pg", "type", "self", "column"],
    ["pg", "type", "self", "to", "create"],
    ["pg", "type", "self", "to", "read"],
    ["pg", "type", "self", "to", "upd"],
    ["pg", "type", "self", "where"],
    ["pg", "type", "self", "to", "del"],
    ["self", "as", "json"],
    ["self", "as", "json", "not", "null"],
    ["self", "as", "jsonb"],
    ["self", "as", "jsonb", "not", "null"],
    ["pg", "json", "type", "obj", "self"],
    ["pg", "json", "type", "std", "opt", "opt", "obj", "self"],
    [
        "pg", "json", "type", "std", "vec", "vec", "obj", "with", "id", "self"
    ],
    [
        "pg", "json", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self"
    ],
    [
        "pg", "json", "type", "obj", "self", "read", "try", "new", "er"
    ],
    [
        "pg", "json", "type", "std", "opt", "opt", "obj", "self", "read", "try", "new", "er"
    ],
    [
        "pg", "json", "type", "std", "vec", "vec", "obj", "with", "id", "self", "read", "try",
        "new", "er"
    ],
    [
        "pg", "json", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id",
        "self", "read", "try", "new", "er"
    ],
    ["pg", "json", "type", "obj", "self", "reader"],
    [
        "pg", "json", "type", "std", "opt", "opt", "obj", "self", "reader"
    ],
    [
        "pg", "json", "type", "std", "vec", "vec", "obj", "with", "id", "self", "reader"
    ],
    [
        "pg", "json", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id",
        "self", "reader"
    ],
    ["pg", "type", "self", "to", "upd", "qp", "er"],
    ["pg", "type", "self", "as", "json", "to", "upd", "qp", "er"],
    [
        "pg", "type", "self", "as", "json", "not", "null", "to", "upd", "qp", "er"
    ],
    ["self", "where", "try", "new", "er"],
    ["pg", "type", "self", "where", "equal"],
    ["pg", "type", "self", "where", "greater", "than"],
    ["pg", "type", "self", "where", "between"],
    ["pg", "type", "self", "where", "between", "try", "new", "er"],
    ["pg", "type", "self", "where", "in"],
    ["pg", "type", "self", "where", "in", "try", "new", "er"],
    ["pg", "type", "std", "opt", "opt", "self", "where", "equal"],
    [
        "pg", "type", "std", "opt", "opt", "self", "where", "greater", "than"
    ],
    [
        "pg", "type", "std", "opt", "opt", "self", "where", "between"
    ],
    ["pg", "type", "std", "opt", "opt", "self", "where", "in"],
    ["std", "opt", "opt", "self", "where"],
    ["pg", "type", "self", "where", "case", "sensitive", "regex"],
    [
        "pg",
        "type",
        "std",
        "opt",
        "opt",
        "self",
        "where",
        "case",
        "sensitive",
        "regex"
    ],
    [
        "pg",
        "type",
        "std",
        "opt",
        "opt",
        "self",
        "where",
        "case",
        "insensitive",
        "regex"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "case",
        "insensitive",
        "regex"
    ],
    [
        "pg",
        "type",
        "std",
        "opt",
        "opt",
        "self",
        "where",
        "hexadecimal",
        "notation",
        "equal"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "hexadecimal",
        "notation",
        "equal"
    ],
    ["pg", "type", "self", "where", "length", "greater", "than"],
    [
        "pg", "type", "std", "opt", "opt", "self", "where", "length", "greater", "than"
    ],
    [
        "pg", "type", "self", "where", "length", "greater", "than", "try", "new", "er"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "equal",
        "to",
        "encoded",
        "string",
        "representation"
    ],
    [
        "pg",
        "type",
        "std",
        "opt",
        "opt",
        "self",
        "where",
        "equal",
        "to",
        "encoded",
        "string",
        "representation"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "v",
        "is",
        "contained",
        "within",
        "range"
    ],
    [
        "pg",
        "type",
        "std",
        "opt",
        "opt",
        "self",
        "where",
        "v",
        "is",
        "contained",
        "within",
        "range"
    ],
    [
        "pg", "type", "self", "where", "contains", "another", "range"
    ],
    [
        "pg", "type", "std", "opt", "opt", "self", "where", "contains", "another", "range"
    ],
    [
        "pg", "type", "std", "opt", "opt", "self", "where", "is", "null"
    ],
    ["pg", "type", "self", "where", "named"],
    ["self", "where", "range", "length", "try", "new", "er"],
    ["self", "where", "range", "length"],
    ["self", "nullable"],
    ["self", "not", "null"],
    ["pg", "type", "self", "not", "null", "to", "create"],
    ["pg", "type", "self", "not", "null", "to", "read"],
    ["pg", "type", "self", "not", "null", "to", "upd"],
    ["pg", "type", "self", "not", "null", "to", "del"],
    ["self", "not", "null", "to", "del"],
    [
        "pg", "type", "self", "where", "position", "equals", "try", "new", "er"
    ],
    ["pg", "type", "self", "where", "position", "equals"],
    ["self", "as", "json", "nullable"],
    ["self", "as", "jsonb", "nullable"],
    ["obj", "self", "where"],
    ["std", "opt", "opt", "obj", "self", "where"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "where"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "where"
    ],
    ["pg", "json", "type", "self", "where"],
    ["pg", "json", "type", "self", "where", "equal"],
    ["pg", "json", "type", "self", "where", "between"],
    [
        "pg", "json", "type", "self", "where", "between", "try", "new", "er"
    ],
    ["pg", "json", "type", "self", "where", "in"],
    [
        "pg", "json", "type", "self", "where", "in", "try", "new", "er"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "case",
        "sensitive",
        "regex",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "where",
        "case",
        "sensitive",
        "regex"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "where",
        "case",
        "sensitive",
        "regex",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "case",
        "insensitive",
        "regex",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "where",
        "case",
        "insensitive",
        "regex",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "where",
        "case",
        "insensitive",
        "regex"
    ],
    [
        "pg", "json", "type", "std", "opt", "opt", "self", "where", "is", "null"
    ],
    ["pg", "json", "type", "self", "where", "is", "null"],
    [
        "pg", "json", "type", "std", "opt", "opt", "self", "to", "create"
    ],
    ["pg", "json", "type", "std", "opt", "opt", "self", "select"],
    ["pg", "json", "type", "std", "opt", "opt", "self", "read"],
    ["pg", "json", "type", "std", "opt", "opt", "self", "where"],
    [
        "pg", "json", "type", "self", "where", "length", "greater", "than", "try", "new", "er"
    ],
    [
        "pg", "json", "type", "self", "where", "length", "greater", "than"
    ],
    [
        "pg", "json", "type", "self", "where", "position", "equals", "try", "new", "er"
    ],
    ["pg", "json", "type", "self", "where", "position", "equals"],
    [
        "pg", "type", "self", "where", "bit", "vec", "position", "equals", "try", "new", "er"
    ],
    [
        "pg", "json", "type", "self", "where", "bit", "vec", "position", "equals"
    ],
    [
        "pg", "json", "type", "self", "where", "bit", "vec", "position", "equals", "try", "new",
        "er"
    ],
    [
        "pg", "type", "self", "where", "bit", "vec", "position", "equals"
    ],
    [
        "pg", "type", "self", "where", "position", "greater", "than", "try", "new", "er"
    ],
    [
        "pg", "json", "type", "self", "where", "position", "greater", "than", "try", "new", "er"
    ],
    ["pg", "type", "self", "where", "position", "greater", "than"],
    [
        "pg", "json", "type", "self", "where", "position", "greater", "than"
    ],
    ["pg", "json", "type", "self", "where", "try", "new", "er"],
    [
        "pg",
        "type",
        "self",
        "where",
        "position",
        "case",
        "sensitive",
        "regex"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "where",
        "position",
        "case",
        "sensitive",
        "regex"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "position",
        "case",
        "sensitive",
        "regex",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "where",
        "position",
        "case",
        "sensitive",
        "regex",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "position",
        "case",
        "insensitive",
        "regex"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "where",
        "position",
        "case",
        "insensitive",
        "regex"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "position",
        "case",
        "insensitive",
        "regex",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "where",
        "position",
        "case",
        "insensitive",
        "regex",
        "try",
        "new",
        "er"
    ],
    [
        "pg", "type", "self", "where", "contains", "all", "els", "of", "arr", "try", "new", "er"
    ],
    [
        "pg", "json", "type", "self", "where", "contains", "all", "els", "of", "arr", "try", "new",
        "er"
    ],
    [
        "pg", "type", "self", "where", "contains", "all", "els", "of", "arr"
    ],
    [
        "pg", "json", "type", "self", "where", "contains", "all", "els", "of", "arr"
    ],
    [
        "self", "where", "bit", "vec", "position", "equals", "try", "new", "er"
    ],
    ["self", "where", "second", "dim"],
    ["self", "visitor"],
    ["self", "not", "null", "try", "new", "er"],
    ["self", "to", "upd", "qp", "er"],
    ["self", "length"],
    ["vec", "self", "arr", "not", "null"],
    ["vec", "self", "arr", "nullable"],
    ["self", "opt", "to", "upd", "try", "gen", "er"],
    [
        "pg", "json", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id",
        "self", "opt", "to", "upd"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd",
        "try", "gen", "er"
    ],
    ["pg", "type", "where", "self"],
    ["pg", "type", "where", "self", "try", "new", "er"],
    [
        "pg", "type", "where", "self", "try", "new", "er", "with", "serde"
    ],
    ["pg", "json", "type", "where", "self"],
    ["pg", "json", "type", "where", "self", "try", "new", "er"],
    [
        "pg", "json", "type", "where", "self", "try", "new", "er", "with", "serde"
    ],
    ["self", "without", "id", "read"],
    ["self", "with", "id", "read"],
    ["self", "without", "id", "read", "try", "from", "er"],
    ["self", "with", "id", "read", "try", "from", "er"],
    ["self", "table", "type"],
    ["self", "not", "null", "origin"],
    ["self", "nullable", "origin"],
    ["self", "origin"],
    ["self", "vec"],
    ["self", "pg", "type"],
    ["self", "with", "id"],
    ["self", "select", "el"],
    ["self", "with", "id", "select", "el"],
    ["self", "with", "id", "upd", "el"],
    ["self", "with", "id", "where"],
    ["self", "read", "try", "from", "er"],
    ["el", "self"],
    ["self", "prep", "pg", "er"],
    ["self", "where", "many"],
    ["self", "where", "many", "try", "new", "er"],
    ["std", "opt", "opt", "self", "where", "many"],
    [
        "try", "from", "sqlx", "pg", "pg", "row", "with", "not", "empty", "unique", "vec", "self",
        "select"
    ],
    ["upd", "qp", "self"],
    ["self", "tests"],
    ["self", "origin", "try", "new", "er"],
    ["self", "origin", "try", "new", "for", "deserialize", "er"],
    ["self", "test", "cases"],
    ["self", "some", "v", "upd"],
    ["self", "some", "v", "read"],
    ["self", "read", "ids"],
    ["self", "upd", "h"],
    ["self", "read", "ids", "h"],
    ["self", "last"],
    ["self", "current"],
    ["jsonb", "self"],
    ["self", "create", "for", "query"],
    ["self", "upd", "for", "query", "el"],
    ["self", "dm", "params"],
    ["self", "dm", "payload"],
    ["self", "dlo", "params"],
    ["self", "dlo", "payload"],
    ["self", "try", "ro", "er"],
    ["self", "ro", "er", "with", "serde"],
    ["self", "um", "params"],
    ["self", "um", "payload"],
    ["self", "try", "dlo", "er"],
    ["self", "dlo", "er", "with", "serde"],
    ["self", "h"],
    ["try", "self", "h"],
    ["d", "self"],
    ["d", "self", "if"],
    [
        "self", "read", "ids", "to2", "dims", "vec", "read", "inner", "acc"
    ],
    ["self", "gen", "pg", "json", "obj", "type", "mod"],
    ["self", "gen", "pg", "table", "mod"]
]);
