//todo
use proc_macro2::TokenStream as Ts2;
use quote::ToTokens;
naming_macros::gen_self_ucc_and_sc_str_and_ts!([
    ["self", "parameters"],
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
    ["try", "self", "response", "vrts"],
    ["self", "payload", "element", "with", "serde"],
    ["self", "payload", "element"],
    [
        "self", "payload", "element", "try", "from", "self", "payload", "with", "serde"
    ],
    [
        "self", "payload", "element", "try", "from", "self", "payload", "element", "with", "serde",
        "er"
    ],
    ["try", "self", "er"],
    ["try", "self", "request", "er"],
    [
        "self", "payload", "try", "from", "self", "payload", "with", "serde", "er"
    ],
    ["try", "self", "with", "serde"],
    ["tvfrr", "extraction", "logic", "try", "self"],
    ["try", "self", "generated", "logic", "er"],
    ["try", "self", "generated", "logic", "desirable"],
    ["try", "self", "logic"],
    ["try", "self", "logic", "response", "vrts"],
    ["try", "self", "logic", "er"],
    ["self", "er", "with", "serde"],
    ["try", "self", "generated", "logic", "er", "with", "serde"],
    ["self", "payload", "example"],
    ["self", "create"],
    ["self", "read"],
    ["self", "read", "inner"],
    ["self", "update"],
    ["self", "update", "for", "query"],
    ["self", "delete"],
    ["object", "self"],
    ["std", "option", "option", "object", "self"],
    ["object", "with", "id", "self"],
    ["std", "vec", "vec", "object", "with", "id", "self"],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self"
    ],
    ["self", "select"],
    ["self", "select", "without", "id"],
    ["self", "select", "with", "id"],
    ["object", "self", "select"],
    ["object", "with", "id", "self", "select"],
    ["std", "option", "option", "object", "self", "select"],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "select"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "select"
    ],
    ["self", "try", "new", "er"],
    ["object", "self", "read"],
    ["std", "option", "option", "object", "self", "read"],
    ["std", "vec", "vec", "object", "with", "id", "self", "read"],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "read"
    ],
    ["object", "self", "to", "create"],
    ["std", "option", "option", "object", "self", "to", "create"],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "to", "create"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to",
        "create"
    ],
    ["object", "with", "id", "self", "read"],
    ["object", "with", "id", "self", "to", "create"],
    ["object", "self", "reader"],
    ["object", "with", "id", "self", "reader"],
    ["std", "option", "option", "object", "self", "reader"],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "reader"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "reader"
    ],
    ["self", "reader"],
    [
        "std", "option", "option", "object", "self", "to", "create", "origin"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "to", "create", "origin"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to",
        "create", "origin"
    ],
    [
        "std", "option", "option", "object", "self", "read", "origin"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "read", "origin"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "read",
        "origin"
    ],
    ["self", "option", "to", "update"],
    ["self", "option", "to", "update", "origin"],
    ["object", "self", "option", "to", "update"],
    ["object", "self", "option", "to", "update", "origin"],
    [
        "std", "option", "option", "object", "self", "option", "to", "update", "origin"
    ],
    [
        "std", "option", "option", "object", "self", "option", "to", "update"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "origin"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option",
        "to", "update", "origin"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option",
        "to", "update"
    ],
    ["object", "with", "id", "self", "option", "to", "update"],
    ["self", "update", "with", "id"],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "json",
        "array", "change"
    ],
    [
        "self", "option", "to", "update", "try", "gen", "pg", "json", "type", "er"
    ],
    [
        "self", "option", "to", "update", "try", "gen", "pg", "json", "type", "er", "with", "serde"
    ],
    ["self", "json", "array", "change"],
    ["self", "to", "create", "origin"],
    ["self", "to", "create", "with", "generated", "id"],
    ["self", "to", "create", "without", "generated", "id"],
    ["self", "json", "array", "change", "try", "gen", "er"],
    ["self", "field", "to", "update"],
    ["self", "gen", "pg", "json", "type", "to", "read", "er"],
    [
        "self",
        "try",
        "gen",
        "json",
        "array",
        "element",
        "update",
        "bind",
        "increments",
        "er"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change", "try",
        "new", "er"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "json",
        "array", "change", "try", "new", "er"
    ],
    ["not", "unique", "field", "self"],
    ["self", "read", "without", "id"],
    ["self", "read", "with", "id"],
    ["self", "option", "to", "update", "try", "new", "er"],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "read", "try", "new", "er"
    ],
    [
        "self", "gen", "pg", "json", "type", "to", "read", "from", "vec", "er"
    ],
    [
        "std", "option", "option", "object", "self", "option", "to", "update", "try", "new", "er"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "read",
        "try", "new", "er"
    ],
    [
        "self", "read", "with", "or", "without", "id", "try", "from", "er"
    ],
    ["self", "select", "try", "new", "er"],
    ["object", "self", "select", "try", "new", "er"],
    [
        "std", "option", "option", "object", "self", "select", "try", "new", "er"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "select", "try", "new", "er"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "select",
        "try", "new", "er"
    ],
    [
        "object", "self", "option", "to", "update", "try", "gen", "er"
    ],
    [
        "object", "self", "option", "to", "update", "try", "gen", "pg", "json", "type", "er",
        "with", "serde"
    ],
    [
        "std", "option", "option", "object", "self", "option", "to", "update", "try", "gen", "er"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "try",
        "gen", "pg", "json", "type", "er"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option",
        "to", "update", "try", "gen", "pg", "json", "type", "er"
    ],
    ["self", "with", "serde"],
    ["self", "try", "from", "env", "er"],
    ["get", "self"],
    ["try", "from", "std", "env", "var", "ok", "self", "er"],
    ["self", "options"],
    ["er", "self"],
    ["not", "unique", "self"],
    ["is", "self", "update", "exist"],
    ["self", "column", "read", "permission"],
    ["self", "where"],
    ["std", "option", "option", "self"],
    ["where", "std", "option", "option", "self"],
    ["sqlx", "types", "json", "object", "self"],
    [
        "std", "option", "option", "sqlx", "types", "json", "object", "self"
    ],
    [
        "sqlx", "types", "json", "std", "option", "option", "object", "self"
    ],
    [
        "std", "option", "option", "sqlx", "types", "json", "std", "option", "option", "object",
        "self"
    ],
    [
        "sqlx", "types", "json", "std", "vec", "vec", "object", "with", "id", "self"
    ],
    [
        "std", "option", "option", "sqlx", "types", "json", "std", "vec", "vec", "object", "with",
        "id", "self"
    ],
    [
        "sqlx", "types", "json", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self"
    ],
    [
        "std", "option", "option", "sqlx", "types", "json", "std", "option", "option", "std",
        "vec", "vec", "object", "with", "id", "self"
    ],
    ["object", "self", "column"],
    ["std", "option", "option", "object", "self", "column"],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "column"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "column"
    ],
    ["pg", "json", "type", "self", "to", "create"],
    ["object", "self", "create"],
    ["std", "option", "option", "object", "self", "create"],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "create"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "create"
    ],
    ["pg", "json", "type", "self", "select"],
    ["pg", "json", "type", "object", "self", "select"],
    [
        "pg", "json", "type", "std", "option", "option", "object", "self", "select"
    ],
    [
        "pg", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "select"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self", "select"
    ],
    ["pg", "json", "type", "self", "select", "try", "new", "er"],
    [
        "pg", "json", "type", "object", "self", "select", "try", "new", "er"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "object", "self", "select", "try", "new",
        "er"
    ],
    [
        "pg", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "select", "try",
        "new", "er"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self", "select", "try", "new", "er"
    ],
    ["pg", "json", "type", "self", "read"],
    ["pg", "json", "type", "self", "read", "without", "id"],
    ["pg", "json", "type", "self", "read", "with", "id"],
    ["self", "update", "element"],
    ["pg", "json", "type", "self", "option", "to", "update"],
    ["self", "update", "er"],
    ["self", "update", "try", "new", "er"],
    ["object", "self", "update"],
    ["std", "option", "option", "object", "self", "update"],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "update"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "update"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "try",
        "gen", "er"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self", "option", "to", "update", "try", "gen", "pg", "json", "type", "er"
    ],
    [
        "pg", "json", "type", "object", "self", "option", "to", "update", "try", "gen", "pg",
        "json", "type", "er", "with", "serde"
    ],
    [
        "pg", "json", "type", "self", "option", "to", "update", "try", "gen", "pg", "json", "type",
        "er", "with", "serde"
    ],
    [
        "pg", "json", "type", "self", "read", "with", "or", "without", "id", "try", "from", "er"
    ],
    ["pg", "type", "object", "self"],
    ["pg", "type", "std", "option", "option", "object", "self"],
    [
        "pg", "type", "std", "vec", "vec", "object", "with", "id", "self"
    ],
    [
        "pg", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id",
        "self"
    ],
    ["pg", "type", "object", "self", "to", "create"],
    [
        "pg", "type", "std", "option", "option", "object", "self", "to", "create"
    ],
    [
        "pg", "type", "std", "vec", "vec", "object", "with", "id", "self", "to", "create"
    ],
    [
        "pg", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id",
        "self", "to", "create"
    ],
    ["pg", "type", "object", "self", "to", "read"],
    [
        "pg", "type", "std", "option", "option", "object", "self", "to", "read"
    ],
    [
        "pg", "type", "std", "vec", "vec", "object", "with", "id", "self", "to", "read"
    ],
    [
        "pg", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id",
        "self", "to", "read"
    ],
    ["pg", "type", "object", "self", "to", "update"],
    [
        "pg", "type", "std", "option", "option", "object", "self", "to", "update"
    ],
    [
        "pg", "type", "std", "vec", "vec", "object", "with", "id", "self", "to", "update"
    ],
    [
        "pg", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id",
        "self", "to", "update"
    ],
    ["pg", "type", "object", "self", "where"],
    [
        "pg", "type", "std", "option", "option", "object", "self", "where"
    ],
    [
        "pg", "type", "std", "vec", "vec", "object", "with", "id", "self", "where"
    ],
    [
        "pg", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id",
        "self", "where"
    ],
    ["pg", "type", "self", "column"],
    ["pg", "type", "self", "to", "create"],
    ["pg", "type", "self", "to", "read"],
    ["pg", "type", "self", "to", "update"],
    ["pg", "type", "self", "where"],
    ["pg", "type", "self", "to", "delete"],
    ["self", "as", "json"],
    ["self", "as", "json", "not", "null"],
    ["self", "as", "jsonb"],
    ["self", "as", "jsonb", "not", "null"],
    ["pg", "json", "type", "object", "self"],
    [
        "pg", "json", "type", "std", "option", "option", "object", "self"
    ],
    [
        "pg", "json", "type", "std", "vec", "vec", "object", "with", "id", "self"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self"
    ],
    [
        "pg", "json", "type", "object", "self", "read", "try", "new", "er"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "object", "self", "read", "try", "new",
        "er"
    ],
    [
        "pg", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "read", "try",
        "new", "er"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self", "read", "try", "new", "er"
    ],
    ["pg", "json", "type", "object", "self", "reader"],
    [
        "pg", "json", "type", "std", "option", "option", "object", "self", "reader"
    ],
    [
        "pg", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "reader"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self", "reader"
    ],
    ["pg", "type", "self", "to", "update", "query", "part", "er"],
    [
        "pg", "type", "self", "as", "json", "to", "update", "query", "part", "er"
    ],
    [
        "pg", "type", "self", "as", "json", "not", "null", "to", "update", "query", "part", "er"
    ],
    ["self", "where", "try", "new", "er"],
    ["pg", "type", "self", "where", "equal"],
    ["pg", "type", "self", "where", "greater", "than"],
    ["pg", "type", "self", "where", "between"],
    ["pg", "type", "self", "where", "between", "try", "new", "er"],
    ["pg", "type", "self", "where", "in"],
    ["pg", "type", "self", "where", "in", "try", "new", "er"],
    [
        "pg", "type", "std", "option", "option", "self", "where", "equal"
    ],
    [
        "pg", "type", "std", "option", "option", "self", "where", "greater", "than"
    ],
    [
        "pg", "type", "std", "option", "option", "self", "where", "between"
    ],
    [
        "pg", "type", "std", "option", "option", "self", "where", "in"
    ],
    ["std", "option", "option", "self", "where"],
    [
        "pg",
        "type",
        "self",
        "where",
        "case",
        "sensitive",
        "regular",
        "expression"
    ],
    [
        "pg",
        "type",
        "std",
        "option",
        "option",
        "self",
        "where",
        "case",
        "sensitive",
        "regular",
        "expression"
    ],
    [
        "pg",
        "type",
        "std",
        "option",
        "option",
        "self",
        "where",
        "case",
        "insensitive",
        "regular",
        "expression"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "case",
        "insensitive",
        "regular",
        "expression"
    ],
    [
        "pg",
        "type",
        "std",
        "option",
        "option",
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
        "pg", "type", "std", "option", "option", "self", "where", "length", "greater", "than"
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
        "option",
        "option",
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
        "value",
        "is",
        "contained",
        "within",
        "range"
    ],
    [
        "pg",
        "type",
        "std",
        "option",
        "option",
        "self",
        "where",
        "value",
        "is",
        "contained",
        "within",
        "range"
    ],
    [
        "pg", "type", "self", "where", "contains", "another", "range"
    ],
    [
        "pg", "type", "std", "option", "option", "self", "where", "contains", "another", "range"
    ],
    [
        "pg", "type", "std", "option", "option", "self", "where", "is", "null"
    ],
    ["pg", "type", "self", "where", "named"],
    ["self", "where", "range", "length", "try", "new", "er"],
    ["self", "where", "range", "length"],
    ["self", "nullable"],
    ["self", "not", "null"],
    ["pg", "type", "self", "not", "null", "to", "create"],
    ["pg", "type", "self", "not", "null", "to", "read"],
    ["pg", "type", "self", "not", "null", "to", "update"],
    ["pg", "type", "self", "not", "null", "to", "delete"],
    ["self", "not", "null", "to", "delete"],
    [
        "pg", "type", "self", "where", "position", "equals", "try", "new", "er"
    ],
    ["pg", "type", "self", "where", "position", "equals"],
    ["self", "as", "json", "nullable"],
    ["self", "as", "jsonb", "nullable"],
    ["object", "self", "where"],
    ["std", "option", "option", "object", "self", "where"],
    ["std", "vec", "vec", "object", "with", "id", "self", "where"],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "where"
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
        "regular",
        "expression",
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
        "regular",
        "expression"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "where",
        "case",
        "sensitive",
        "regular",
        "expression",
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
        "regular",
        "expression",
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
        "regular",
        "expression",
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
        "regular",
        "expression"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "self", "where", "is", "null"
    ],
    ["pg", "json", "type", "self", "where", "is", "null"],
    [
        "pg", "json", "type", "std", "option", "option", "self", "to", "create"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "self", "select"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "self", "read"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "self", "where"
    ],
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
        "regular",
        "expression"
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
        "regular",
        "expression"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "position",
        "case",
        "sensitive",
        "regular",
        "expression",
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
        "regular",
        "expression",
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
        "regular",
        "expression"
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
        "regular",
        "expression"
    ],
    [
        "pg",
        "type",
        "self",
        "where",
        "position",
        "case",
        "insensitive",
        "regular",
        "expression",
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
        "regular",
        "expression",
        "try",
        "new",
        "er"
    ],
    [
        "pg", "type", "self", "where", "contains", "all", "elements", "of", "array", "try", "new",
        "er"
    ],
    [
        "pg", "json", "type", "self", "where", "contains", "all", "elements", "of", "array", "try",
        "new", "er"
    ],
    [
        "pg", "type", "self", "where", "contains", "all", "elements", "of", "array"
    ],
    [
        "pg", "json", "type", "self", "where", "contains", "all", "elements", "of", "array"
    ],
    [
        "self", "where", "bit", "vec", "position", "equals", "try", "new", "er"
    ],
    ["self", "where", "second", "dim"],
    ["self", "visitor"],
    ["self", "not", "null", "try", "new", "er"],
    ["self", "to", "update", "query", "part", "er"],
    ["self", "length"],
    ["vec", "self", "array", "not", "null"],
    ["vec", "self", "array", "nullable"],
    ["self", "option", "to", "update", "try", "gen", "er"],
    [
        "pg", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self", "option", "to", "update"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option",
        "to", "update", "try", "gen", "er"
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
    ["self", "table", "type", "declaration"],
    ["self", "not", "null", "origin"],
    ["self", "nullable", "origin"],
    ["self", "origin"],
    ["self", "vec"],
    ["self", "pg", "type"],
    ["self", "with", "id"],
    ["self", "select", "element"],
    ["self", "with", "id", "select", "element"],
    ["self", "with", "id", "update", "element"],
    ["self", "with", "id", "where"],
    ["self", "read", "try", "from", "er"],
    ["element", "self"],
    ["self", "prepare", "pg", "er"],
    ["self", "where", "many"],
    ["self", "where", "many", "try", "new", "er"],
    ["std", "option", "option", "self", "where", "many"],
    [
        "try", "from", "sqlx", "pg", "pg", "row", "with", "not", "empty", "unique", "vec", "self",
        "select"
    ],
    ["update", "query", "part", "self"],
    ["self", "tests"],
    ["self", "origin", "try", "new", "er"],
    ["self", "origin", "try", "new", "for", "deserialize", "er"],
    ["self", "test", "cases"],
    ["self", "some", "value", "update"],
    ["self", "some", "value", "read"],
    ["self", "read", "only", "ids"],
    ["self", "update", "handle"],
    ["self", "read", "only", "ids", "handle"],
    ["self", "last"],
    ["self", "current"],
    ["jsonb", "self"],
    ["self", "create", "for", "query"],
    ["self", "update", "for", "query", "element"],
    ["self", "delete", "many", "parameters"],
    ["self", "delete", "many", "payload"],
    ["self", "delete", "one", "parameters"],
    ["self", "delete", "one", "payload"],
    ["self", "try", "read", "one", "er"],
    ["self", "read", "one", "er", "with", "serde"],
    ["self", "update", "many", "parameters"],
    ["self", "update", "many", "payload"],
    ["self", "try", "delete", "one", "er"],
    ["self", "delete", "one", "er", "with", "serde"],
    ["self", "handle"],
    ["try", "self", "handle"],
    ["derive", "self"],
    ["derive", "self", "if"],
    [
        "self", "read", "only", "ids", "to", "two", "dimal", "vec", "read", "inner", "acc"
    ],
    ["self", "gen", "pg", "json", "object", "type", "mod"],
    ["self", "gen", "pg", "table", "mod"]
]);
