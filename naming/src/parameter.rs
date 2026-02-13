//todo
use proc_macro2::TokenStream as Ts2;
use quote::ToTokens;
naming_macros::gen_self_upper_camel_and_sc_str_and_ts!([
    ["self", "parameters"],
    ["self", "payload"],
    ["self", "payload", "with", "serialize", "deserialize"],
    [
        "self",
        "payload",
        "try",
        "from",
        "self",
        "payload",
        "with",
        "serialize",
        "deserialize"
    ],
    [
        "self",
        "payload",
        "with",
        "serialize",
        "deserialize",
        "try",
        "from",
        "self",
        "payload"
    ],
    [
        "self",
        "payload",
        "with",
        "serialize",
        "deserialize",
        "try",
        "from",
        "self",
        "payload",
        "error",
        "named"
    ],
    ["try", "self"],
    ["try", "self", "response", "variants"],
    [
        "self",
        "payload",
        "element",
        "with",
        "serialize",
        "deserialize"
    ],
    ["self", "payload", "element"],
    [
        "self",
        "payload",
        "element",
        "try",
        "from",
        "self",
        "payload",
        "with",
        "serialize",
        "deserialize"
    ],
    [
        "self",
        "payload",
        "element",
        "try",
        "from",
        "self",
        "payload",
        "element",
        "with",
        "serialize",
        "deserialize",
        "error",
        "named"
    ],
    ["try", "self", "error", "named"],
    ["try", "self", "request", "error"],
    [
        "self",
        "payload",
        "try",
        "from",
        "self",
        "payload",
        "with",
        "serialize",
        "deserialize",
        "error",
        "named"
    ],
    ["try", "self", "with", "serialize", "deserialize"],
    ["tvfrr", "extraction", "logic", "try", "self"],
    ["try", "self", "generated", "logic", "error", "named"],
    ["try", "self", "generated", "logic", "desirable"],
    ["try", "self", "logic"],
    ["try", "self", "logic", "response", "variants"],
    ["try", "self", "logic", "error", "named"],
    ["self", "error", "named", "with", "serialize", "deserialize"],
    [
        "try",
        "self",
        "generated",
        "logic",
        "error",
        "named",
        "with",
        "serialize",
        "deserialize"
    ],
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
    ["self", "try", "new", "error", "named"],
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
        "self", "option", "to", "update", "try", "gen", "pg", "json", "type", "error", "named"
    ],
    [
        "self",
        "option",
        "to",
        "update",
        "try",
        "gen",
        "pg",
        "json",
        "type",
        "error",
        "named",
        "with",
        "serialize",
        "deserialize"
    ],
    ["self", "json", "array", "change"],
    ["self", "to", "create", "origin"],
    ["self", "to", "create", "with", "generated", "id"],
    ["self", "to", "create", "without", "generated", "id"],
    [
        "self", "json", "array", "change", "try", "gen", "error", "named"
    ],
    ["self", "field", "to", "update"],
    [
        "self", "gen", "pg", "json", "type", "to", "read", "error", "named"
    ],
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
        "error",
        "named"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change", "try",
        "new", "error", "named"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "json",
        "array", "change", "try", "new", "error", "named"
    ],
    ["not", "unique", "field", "self"],
    ["self", "read", "without", "id"],
    ["self", "read", "with", "id"],
    [
        "self", "option", "to", "update", "try", "new", "error", "named"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "read", "try", "new", "error", "named"
    ],
    [
        "self", "gen", "pg", "json", "type", "to", "read", "from", "vec", "error", "named"
    ],
    [
        "std", "option", "option", "object", "self", "option", "to", "update", "try", "new",
        "error", "named"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "read",
        "try", "new", "error", "named"
    ],
    [
        "self", "read", "with", "or", "without", "id", "try", "from", "error", "named"
    ],
    ["self", "select", "try", "new", "error", "named"],
    ["object", "self", "select", "try", "new", "error", "named"],
    [
        "std", "option", "option", "object", "self", "select", "try", "new", "error", "named"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "select", "try", "new", "error",
        "named"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "select",
        "try", "new", "error", "named"
    ],
    [
        "object", "self", "option", "to", "update", "try", "gen", "error", "named"
    ],
    [
        "object",
        "self",
        "option",
        "to",
        "update",
        "try",
        "gen",
        "pg",
        "json",
        "type",
        "error",
        "named",
        "with",
        "serialize",
        "deserialize"
    ],
    [
        "std", "option", "option", "object", "self", "option", "to", "update", "try", "gen",
        "error", "named"
    ],
    [
        "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "try",
        "gen", "pg", "json", "type", "error", "named"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option",
        "to", "update", "try", "gen", "pg", "json", "type", "error", "named"
    ],
    ["self", "with", "serialize", "deserialize"],
    ["self", "try", "from", "env", "error", "named"],
    ["get", "self"],
    [
        "try", "from", "std", "env", "var", "ok", "self", "error", "named"
    ],
    ["self", "options"],
    ["error", "self"],
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
    [
        "pg", "json", "type", "self", "select", "try", "new", "error", "named"
    ],
    [
        "pg", "json", "type", "object", "self", "select", "try", "new", "error", "named"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "object", "self", "select", "try", "new",
        "error", "named"
    ],
    [
        "pg", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "select", "try",
        "new", "error", "named"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self", "select", "try", "new", "error", "named"
    ],
    ["pg", "json", "type", "self", "read"],
    ["pg", "json", "type", "self", "read", "without", "id"],
    ["pg", "json", "type", "self", "read", "with", "id"],
    ["self", "update", "element"],
    ["pg", "json", "type", "self", "option", "to", "update"],
    ["self", "update", "error", "named"],
    ["self", "update", "try", "new", "error", "named"],
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
        "gen", "error", "named"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self", "option", "to", "update", "try", "gen", "pg", "json", "type", "error",
        "named"
    ],
    [
        "pg",
        "json",
        "type",
        "object",
        "self",
        "option",
        "to",
        "update",
        "try",
        "gen",
        "pg",
        "json",
        "type",
        "error",
        "named",
        "with",
        "serialize",
        "deserialize"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "option",
        "to",
        "update",
        "try",
        "gen",
        "pg",
        "json",
        "type",
        "error",
        "named",
        "with",
        "serialize",
        "deserialize"
    ],
    [
        "pg", "json", "type", "self", "read", "with", "or", "without", "id", "try", "from",
        "error", "named"
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
        "pg", "json", "type", "object", "self", "read", "try", "new", "error", "named"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "object", "self", "read", "try", "new",
        "error", "named"
    ],
    [
        "pg", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "read", "try",
        "new", "error", "named"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self", "read", "try", "new", "error", "named"
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
    [
        "pg", "type", "self", "to", "update", "query", "part", "error", "named"
    ],
    [
        "pg", "type", "self", "as", "json", "to", "update", "query", "part", "error", "named"
    ],
    [
        "pg", "type", "self", "as", "json", "not", "null", "to", "update", "query", "part",
        "error", "named"
    ],
    ["self", "where", "try", "new", "error", "named"],
    ["pg", "type", "self", "where", "equal"],
    ["pg", "type", "self", "where", "greater", "than"],
    ["pg", "type", "self", "where", "between"],
    [
        "pg", "type", "self", "where", "between", "try", "new", "error", "named"
    ],
    ["pg", "type", "self", "where", "in"],
    [
        "pg", "type", "self", "where", "in", "try", "new", "error", "named"
    ],
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
        "pg", "type", "self", "where", "length", "greater", "than", "try", "new", "error", "named"
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
    [
        "self", "where", "range", "length", "try", "new", "error", "named"
    ],
    ["self", "where", "range", "length"],
    ["self", "nullable"],
    ["self", "not", "null"],
    ["pg", "type", "self", "not", "null", "to", "create"],
    ["pg", "type", "self", "not", "null", "to", "read"],
    ["pg", "type", "self", "not", "null", "to", "update"],
    ["pg", "type", "self", "not", "null", "to", "delete"],
    ["self", "not", "null", "to", "delete"],
    [
        "pg", "type", "self", "where", "position", "equals", "try", "new", "error", "named"
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
        "pg", "json", "type", "self", "where", "between", "try", "new", "error", "named"
    ],
    ["pg", "json", "type", "self", "where", "in"],
    [
        "pg", "json", "type", "self", "where", "in", "try", "new", "error", "named"
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
        "error",
        "named"
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
        "error",
        "named"
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
        "error",
        "named"
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
        "error",
        "named"
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
        "pg", "json", "type", "self", "where", "length", "greater", "than", "try", "new", "error",
        "named"
    ],
    [
        "pg", "json", "type", "self", "where", "length", "greater", "than"
    ],
    [
        "pg", "json", "type", "self", "where", "position", "equals", "try", "new", "error", "named"
    ],
    ["pg", "json", "type", "self", "where", "position", "equals"],
    [
        "pg", "type", "self", "where", "bit", "vec", "position", "equals", "try", "new", "error",
        "named"
    ],
    [
        "pg", "json", "type", "self", "where", "bit", "vec", "position", "equals"
    ],
    [
        "pg", "json", "type", "self", "where", "bit", "vec", "position", "equals", "try", "new",
        "error", "named"
    ],
    [
        "pg", "type", "self", "where", "bit", "vec", "position", "equals"
    ],
    [
        "pg", "type", "self", "where", "position", "greater", "than", "try", "new", "error",
        "named"
    ],
    [
        "pg", "json", "type", "self", "where", "position", "greater", "than", "try", "new",
        "error", "named"
    ],
    ["pg", "type", "self", "where", "position", "greater", "than"],
    [
        "pg", "json", "type", "self", "where", "position", "greater", "than"
    ],
    [
        "pg", "json", "type", "self", "where", "try", "new", "error", "named"
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
        "error",
        "named"
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
        "error",
        "named"
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
        "error",
        "named"
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
        "error",
        "named"
    ],
    [
        "pg", "type", "self", "where", "contains", "all", "elements", "of", "array", "try", "new",
        "error", "named"
    ],
    [
        "pg", "json", "type", "self", "where", "contains", "all", "elements", "of", "array", "try",
        "new", "error", "named"
    ],
    [
        "pg", "type", "self", "where", "contains", "all", "elements", "of", "array"
    ],
    [
        "pg", "json", "type", "self", "where", "contains", "all", "elements", "of", "array"
    ],
    [
        "self", "where", "bit", "vec", "position", "equals", "try", "new", "error", "named"
    ],
    ["self", "where", "second", "dimension"],
    ["self", "visitor"],
    ["self", "not", "null", "try", "new", "error", "named"],
    ["self", "to", "update", "query", "part", "error", "named"],
    ["self", "length"],
    ["vec", "self", "array", "not", "null"],
    ["vec", "self", "array", "nullable"],
    [
        "self", "option", "to", "update", "try", "gen", "error", "named"
    ],
    [
        "pg", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with",
        "id", "self", "option", "to", "update"
    ],
    [
        "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option",
        "to", "update", "try", "gen", "error", "named"
    ],
    ["pg", "type", "where", "self"],
    [
        "pg", "type", "where", "self", "try", "new", "error", "named"
    ],
    [
        "pg",
        "type",
        "where",
        "self",
        "try",
        "new",
        "error",
        "named",
        "with",
        "serialize",
        "deserialize"
    ],
    ["pg", "json", "type", "where", "self"],
    [
        "pg", "json", "type", "where", "self", "try", "new", "error", "named"
    ],
    [
        "pg",
        "json",
        "type",
        "where",
        "self",
        "try",
        "new",
        "error",
        "named",
        "with",
        "serialize",
        "deserialize"
    ],
    ["self", "without", "id", "read"],
    ["self", "with", "id", "read"],
    [
        "self", "without", "id", "read", "try", "from", "error", "named"
    ],
    [
        "self", "with", "id", "read", "try", "from", "error", "named"
    ],
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
    ["self", "read", "try", "from", "error", "named"],
    ["element", "self"],
    ["self", "prepare", "pg", "error", "named"],
    ["self", "where", "many"],
    ["self", "where", "many", "try", "new", "error", "named"],
    ["std", "option", "option", "self", "where", "many"],
    [
        "try", "from", "sqlx", "pg", "pg", "row", "with", "not", "empty", "unique", "vec", "self",
        "select"
    ],
    ["update", "query", "part", "self"],
    ["self", "tests"],
    ["self", "origin", "try", "new", "error", "named"],
    [
        "self",
        "origin",
        "try",
        "new",
        "for",
        "deserialize",
        "error",
        "named"
    ],
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
    ["self", "try", "read", "one", "error", "named"],
    [
        "self",
        "read",
        "one",
        "error",
        "named",
        "with",
        "serialize",
        "deserialize"
    ],
    ["self", "update", "many", "parameters"],
    ["self", "update", "many", "payload"],
    ["self", "try", "delete", "one", "error", "named"],
    [
        "self",
        "delete",
        "one",
        "error",
        "named",
        "with",
        "serialize",
        "deserialize"
    ],
    ["self", "handle"],
    ["try", "self", "handle"],
    ["derive", "self"],
    ["derive", "self", "if"],
    [
        "self",
        "read",
        "only",
        "ids",
        "to",
        "two",
        "dimensional",
        "vec",
        "read",
        "inner",
        "acc"
    ],
    ["self", "gen", "pg", "json", "object", "type", "mod"],
    ["self", "gen", "pg", "table", "mod"]
]);
