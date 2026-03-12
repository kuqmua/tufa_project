//todo
use proc_macro2::TokenStream as Ts2;
use quote::ToTokens;
naming_macros::gen_self_ucc_and_sc_str_and_ts!([
    ["self", "prms"],
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
    ["self", "cr"],
    ["self", "rd"],
    ["self", "rd", "inn"],
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
    ["self", "sel"],
    ["self", "sel", "without", "id"],
    ["self", "sel", "with", "id"],
    ["obj", "self", "sel"],
    ["obj", "with", "id", "self", "sel"],
    ["std", "opt", "opt", "obj", "self", "sel"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "sel"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "sel"
    ],
    ["self", "try", "new", "er"],
    ["obj", "self", "rd"],
    ["std", "opt", "opt", "obj", "self", "rd"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "rd"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "rd"
    ],
    ["obj", "self", "to", "cr"],
    ["std", "opt", "opt", "obj", "self", "to", "cr"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "to", "cr"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "to", "cr"
    ],
    ["obj", "with", "id", "self", "rd"],
    ["obj", "with", "id", "self", "to", "cr"],
    ["obj", "self", "rder"],
    ["obj", "with", "id", "self", "rder"],
    ["std", "opt", "opt", "obj", "self", "rder"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "rder"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "rder"
    ],
    ["self", "rder"],
    ["std", "opt", "opt", "obj", "self", "to", "cr", "orgn"],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "to", "cr", "orgn"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "to", "cr", "orgn"
    ],
    ["std", "opt", "opt", "obj", "self", "rd", "orgn"],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "rd", "orgn"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "rd", "orgn"
    ],
    ["self", "opt", "to", "upd"],
    ["self", "opt", "to", "upd", "orgn"],
    ["obj", "self", "opt", "to", "upd"],
    ["obj", "self", "opt", "to", "upd", "orgn"],
    [
        "std", "opt", "opt", "obj", "self", "opt", "to", "upd", "orgn"
    ],
    ["std", "opt", "opt", "obj", "self", "opt", "to", "upd"],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd", "orgn"
    ],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd",
        "orgn"
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
    ["self", "opt", "to", "upd", "try", "gen", "pg", "json", "er"],
    [
        "self", "opt", "to", "upd", "try", "gen", "pg", "json", "er", "with", "serde"
    ],
    ["self", "json", "arr", "change"],
    ["self", "to", "cr", "orgn"],
    ["self", "to", "cr", "with", "generated", "id"],
    ["self", "to", "cr", "without", "generated", "id"],
    ["self", "json", "arr", "change", "try", "gen", "er"],
    ["self", "field", "to", "upd"],
    ["self", "gen", "pg", "json", "to", "rd", "er"],
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
    ["not", "unq", "field", "self"],
    ["self", "rd", "without", "id"],
    ["self", "rd", "with", "id"],
    ["self", "opt", "to", "upd", "try", "new", "er"],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "rd", "try", "new", "er"
    ],
    ["self", "gen", "pg", "json", "to", "rd", "from", "vec", "er"],
    [
        "std", "opt", "opt", "obj", "self", "opt", "to", "upd", "try", "new", "er"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "rd", "try", "new",
        "er"
    ],
    [
        "self", "rd", "with", "or", "without", "id", "try", "from", "er"
    ],
    ["self", "sel", "try", "new", "er"],
    ["obj", "self", "sel", "try", "new", "er"],
    [
        "std", "opt", "opt", "obj", "self", "sel", "try", "new", "er"
    ],
    [
        "std", "vec", "vec", "obj", "with", "id", "self", "sel", "try", "new", "er"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "sel", "try", "new",
        "er"
    ],
    ["obj", "self", "opt", "to", "upd", "try", "gen", "er"],
    [
        "obj", "self", "opt", "to", "upd", "try", "gen", "pg", "json", "er", "with", "serde"
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
        "try", "gen", "pg", "json", "er"
    ],
    ["self", "with", "serde"],
    ["self", "try", "from", "env", "er"],
    ["get", "self"],
    ["try", "from", "std", "env", "var", "ok", "self", "er"],
    ["self", "opts"],
    ["er", "self"],
    ["not", "unq", "self"],
    ["is", "self", "upd", "exist"],
    ["self", "col", "rd", "permission"],
    ["self", "wh"],
    ["std", "opt", "opt", "self"],
    ["wh", "std", "opt", "opt", "self"],
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
    ["obj", "self", "col"],
    ["std", "opt", "opt", "obj", "self", "col"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "col"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "col"
    ],
    ["pg", "json", "self", "to", "cr"],
    ["obj", "self", "cr"],
    ["std", "opt", "opt", "obj", "self", "cr"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "cr"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "cr"
    ],
    ["pg", "json", "self", "sel"],
    ["pg", "json", "obj", "self", "sel"],
    ["pg", "json", "std", "opt", "opt", "obj", "self", "sel"],
    [
        "pg", "json", "std", "vec", "vec", "obj", "with", "id", "self", "sel"
    ],
    [
        "pg", "json", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "sel"
    ],
    ["pg", "json", "self", "sel", "try", "new", "er"],
    ["pg", "json", "obj", "self", "sel", "try", "new", "er"],
    [
        "pg", "json", "std", "opt", "opt", "obj", "self", "sel", "try", "new", "er"
    ],
    [
        "pg", "json", "std", "vec", "vec", "obj", "with", "id", "self", "sel", "try", "new", "er"
    ],
    [
        "pg", "json", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "sel",
        "try", "new", "er"
    ],
    ["pg", "json", "self", "rd"],
    ["pg", "json", "self", "rd", "without", "id"],
    ["pg", "json", "self", "rd", "with", "id"],
    ["self", "upd", "el"],
    ["pg", "json", "self", "opt", "to", "upd"],
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
        "pg", "json", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "opt",
        "to", "upd", "try", "gen", "pg", "json", "er"
    ],
    [
        "pg", "json", "obj", "self", "opt", "to", "upd", "try", "gen", "pg", "json", "type", "er",
        "with", "serde"
    ],
    [
        "pg", "json", "self", "opt", "to", "upd", "try", "gen", "pg", "json", "er", "with", "serde"
    ],
    [
        "pg", "json", "self", "rd", "with", "or", "without", "id", "try", "from", "er"
    ],
    ["pg", "type", "obj", "self"],
    ["pg", "type", "std", "opt", "opt", "obj", "self"],
    [
        "pg", "type", "std", "vec", "vec", "obj", "with", "id", "self"
    ],
    [
        "pg", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self"
    ],
    ["pg", "type", "obj", "self", "to", "cr"],
    ["pg", "type", "std", "opt", "opt", "obj", "self", "to", "cr"],
    [
        "pg", "type", "std", "vec", "vec", "obj", "with", "id", "self", "to", "cr"
    ],
    [
        "pg", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "to",
        "cr"
    ],
    ["pg", "type", "obj", "self", "to", "rd"],
    ["pg", "type", "std", "opt", "opt", "obj", "self", "to", "rd"],
    [
        "pg", "type", "std", "vec", "vec", "obj", "with", "id", "self", "to", "rd"
    ],
    [
        "pg", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "to",
        "rd"
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
    ["pg", "type", "obj", "self", "wh"],
    ["pg", "type", "std", "opt", "opt", "obj", "self", "wh"],
    [
        "pg", "type", "std", "vec", "vec", "obj", "with", "id", "self", "wh"
    ],
    [
        "pg", "type", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "wh"
    ],
    ["pg", "type", "self", "col"],
    ["pg", "type", "self", "to", "cr"],
    ["pg", "type", "self", "to", "rd"],
    ["pg", "type", "self", "to", "upd"],
    ["pg", "type", "self", "wh"],
    ["pg", "type", "self", "to", "del"],
    ["self", "as", "json"],
    ["self", "as", "json", "nn"],
    ["self", "as", "jsonb"],
    ["self", "as", "jsonb", "nn"],
    ["pg", "json", "obj", "self"],
    ["pg", "json", "std", "opt", "opt", "obj", "self"],
    [
        "pg", "json", "std", "vec", "vec", "obj", "with", "id", "self"
    ],
    [
        "pg", "json", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self"
    ],
    ["pg", "json", "obj", "self", "rd", "try", "new", "er"],
    [
        "pg", "json", "std", "opt", "opt", "obj", "self", "rd", "try", "new", "er"
    ],
    [
        "pg", "json", "std", "vec", "vec", "obj", "with", "id", "self", "rd", "try", "new", "er"
    ],
    [
        "pg", "json", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "rd",
        "try", "new", "er"
    ],
    ["pg", "json", "obj", "self", "rder"],
    ["pg", "json", "std", "opt", "opt", "obj", "self", "rder"],
    [
        "pg", "json", "std", "vec", "vec", "obj", "with", "id", "self", "rder"
    ],
    [
        "pg", "json", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "rder"
    ],
    ["pg", "type", "self", "to", "upd", "qp", "er"],
    ["pg", "type", "self", "as", "json", "to", "upd", "qp", "er"],
    [
        "pg", "type", "self", "as", "json", "nn", "to", "upd", "qp", "er"
    ],
    ["self", "wh", "try", "new", "er"],
    ["pg", "type", "self", "wh", "eq"],
    ["pg", "type", "self", "wh", "greater", "than"],
    ["pg", "type", "self", "wh", "btwn"],
    ["pg", "type", "self", "wh", "btwn", "try", "new", "er"],
    ["pg", "type", "self", "wh", "in"],
    ["pg", "type", "self", "wh", "in", "try", "new", "er"],
    ["pg", "type", "std", "opt", "opt", "self", "wh", "eq"],
    [
        "pg", "type", "std", "opt", "opt", "self", "wh", "greater", "than"
    ],
    ["pg", "type", "std", "opt", "opt", "self", "wh", "btwn"],
    ["pg", "type", "std", "opt", "opt", "self", "wh", "in"],
    ["std", "opt", "opt", "self", "wh"],
    ["pg", "type", "self", "wh", "case", "sensitive", "rgx"],
    [
        "pg",
        "type",
        "std",
        "opt",
        "opt",
        "self",
        "wh",
        "case",
        "sensitive",
        "rgx"
    ],
    [
        "pg",
        "type",
        "std",
        "opt",
        "opt",
        "self",
        "wh",
        "case",
        "insensitive",
        "rgx"
    ],
    ["pg", "type", "self", "wh", "case", "insensitive", "rgx"],
    [
        "pg",
        "type",
        "std",
        "opt",
        "opt",
        "self",
        "wh",
        "hexadecimal",
        "notation",
        "eq"
    ],
    ["pg", "type", "self", "wh", "hexadecimal", "notation", "eq"],
    ["pg", "type", "self", "wh", "len", "greater", "than"],
    [
        "pg", "type", "std", "opt", "opt", "self", "wh", "len", "greater", "than"
    ],
    [
        "pg", "type", "self", "wh", "len", "greater", "than", "try", "new", "er"
    ],
    [
        "pg",
        "type",
        "self",
        "wh",
        "eq",
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
        "wh",
        "eq",
        "to",
        "encoded",
        "string",
        "representation"
    ],
    [
        "pg",
        "type",
        "self",
        "wh",
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
        "wh",
        "v",
        "is",
        "contained",
        "within",
        "range"
    ],
    ["pg", "type", "self", "wh", "contains", "another", "range"],
    [
        "pg", "type", "std", "opt", "opt", "self", "wh", "contains", "another", "range"
    ],
    [
        "pg", "type", "std", "opt", "opt", "self", "wh", "is", "null"
    ],
    ["pg", "type", "self", "wh", "named"],
    ["self", "wh", "range", "len", "try", "new", "er"],
    ["self", "wh", "range", "len"],
    ["self", "nl"],
    ["self", "nn"],
    ["pg", "type", "self", "nn", "to", "cr"],
    ["pg", "type", "self", "nn", "to", "rd"],
    ["pg", "type", "self", "nn", "to", "upd"],
    ["pg", "type", "self", "nn", "to", "del"],
    ["self", "nn", "to", "del"],
    [
        "pg", "type", "self", "wh", "position", "eqs", "try", "new", "er"
    ],
    ["pg", "type", "self", "wh", "position", "eqs"],
    ["self", "as", "json", "nl"],
    ["self", "as", "jsonb", "nl"],
    ["obj", "self", "wh"],
    ["std", "opt", "opt", "obj", "self", "wh"],
    ["std", "vec", "vec", "obj", "with", "id", "self", "wh"],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "wh"
    ],
    ["pg", "json", "self", "wh"],
    ["pg", "json", "self", "wh", "eq"],
    ["pg", "json", "self", "wh", "btwn"],
    ["pg", "json", "self", "wh", "btwn", "try", "new", "er"],
    ["pg", "json", "self", "wh", "in"],
    ["pg", "json", "self", "wh", "in", "try", "new", "er"],
    [
        "pg",
        "type",
        "self",
        "wh",
        "case",
        "sensitive",
        "rgx",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "wh",
        "case",
        "sensitive",
        "rgx"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "wh",
        "case",
        "sensitive",
        "rgx",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "type",
        "self",
        "wh",
        "case",
        "insensitive",
        "rgx",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "wh",
        "case",
        "insensitive",
        "rgx",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "wh",
        "case",
        "insensitive",
        "rgx"
    ],
    [
        "pg", "json", "std", "opt", "opt", "self", "wh", "is", "null"
    ],
    ["pg", "json", "self", "wh", "is", "null"],
    ["pg", "json", "std", "opt", "opt", "self", "to", "cr"],
    ["pg", "json", "std", "opt", "opt", "self", "sel"],
    ["pg", "json", "std", "opt", "opt", "self", "rd"],
    ["pg", "json", "std", "opt", "opt", "self", "wh"],
    [
        "pg", "json", "self", "wh", "len", "greater", "than", "try", "new", "er"
    ],
    ["pg", "json", "self", "wh", "len", "greater", "than"],
    [
        "pg", "json", "self", "wh", "position", "eqs", "try", "new", "er"
    ],
    ["pg", "json", "self", "wh", "position", "eqs"],
    [
        "pg", "type", "self", "wh", "bit", "vec", "position", "eqs", "try", "new", "er"
    ],
    ["pg", "json", "self", "wh", "bit", "vec", "position", "eqs"],
    [
        "pg", "json", "self", "wh", "bit", "vec", "position", "eqs", "try", "new", "er"
    ],
    ["pg", "type", "self", "wh", "bit", "vec", "position", "eqs"],
    [
        "pg", "type", "self", "wh", "position", "greater", "than", "try", "new", "er"
    ],
    [
        "pg", "json", "self", "wh", "position", "greater", "than", "try", "new", "er"
    ],
    ["pg", "type", "self", "wh", "position", "greater", "than"],
    ["pg", "json", "self", "wh", "position", "greater", "than"],
    ["pg", "json", "self", "wh", "try", "new", "er"],
    [
        "pg",
        "type",
        "self",
        "wh",
        "position",
        "case",
        "sensitive",
        "rgx"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "wh",
        "position",
        "case",
        "sensitive",
        "rgx"
    ],
    [
        "pg",
        "type",
        "self",
        "wh",
        "position",
        "case",
        "sensitive",
        "rgx",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "wh",
        "position",
        "case",
        "sensitive",
        "rgx",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "type",
        "self",
        "wh",
        "position",
        "case",
        "insensitive",
        "rgx"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "wh",
        "position",
        "case",
        "insensitive",
        "rgx"
    ],
    [
        "pg",
        "type",
        "self",
        "wh",
        "position",
        "case",
        "insensitive",
        "rgx",
        "try",
        "new",
        "er"
    ],
    [
        "pg",
        "json",
        "type",
        "self",
        "wh",
        "position",
        "case",
        "insensitive",
        "rgx",
        "try",
        "new",
        "er"
    ],
    [
        "pg", "type", "self", "wh", "contains", "all", "els", "of", "arr", "try", "new", "er"
    ],
    [
        "pg", "json", "self", "wh", "contains", "all", "els", "of", "arr", "try", "new", "er"
    ],
    [
        "pg", "type", "self", "wh", "contains", "all", "els", "of", "arr"
    ],
    [
        "pg", "json", "self", "wh", "contains", "all", "els", "of", "arr"
    ],
    [
        "self", "wh", "bit", "vec", "position", "eqs", "try", "new", "er"
    ],
    ["self", "wh", "second", "dim"],
    ["self", "visitor"],
    ["self", "nn", "try", "new", "er"],
    ["self", "to", "upd", "qp", "er"],
    ["self", "len"],
    ["vec", "self", "arr", "nn"],
    ["vec", "self", "arr", "nl"],
    ["self", "opt", "to", "upd", "try", "gen", "er"],
    [
        "pg", "json", "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "opt",
        "to", "upd"
    ],
    [
        "std", "opt", "opt", "std", "vec", "vec", "obj", "with", "id", "self", "opt", "to", "upd",
        "try", "gen", "er"
    ],
    ["pg", "type", "wh", "self"],
    ["pg", "type", "wh", "self", "try", "new", "er"],
    [
        "pg", "type", "wh", "self", "try", "new", "er", "with", "serde"
    ],
    ["pg", "json", "wh", "self"],
    ["pg", "json", "wh", "self", "try", "new", "er"],
    [
        "pg", "json", "wh", "self", "try", "new", "er", "with", "serde"
    ],
    ["self", "without", "id", "rd"],
    ["self", "with", "id", "rd"],
    ["self", "without", "id", "rd", "try", "from", "er"],
    ["self", "with", "id", "rd", "try", "from", "er"],
    ["self", "tt"],
    ["self", "nn", "orgn"],
    ["self", "nl", "orgn"],
    ["self", "orgn"],
    ["self", "vec"],
    ["self", "pg", "type"],
    ["self", "with", "id"],
    ["self", "sel", "el"],
    ["self", "with", "id", "sel", "el"],
    ["self", "with", "id", "upd", "el"],
    ["self", "with", "id", "wh"],
    ["self", "rd", "try", "from", "er"],
    ["el", "self"],
    ["self", "prep", "pg", "er"],
    ["self", "wh", "many"],
    ["self", "wh", "many", "try", "new", "er"],
    ["std", "opt", "opt", "self", "wh", "many"],
    [
        "try", "from", "sqlx", "pg", "pg", "row", "with", "not", "empty", "unq", "vec", "self",
        "sel"
    ],
    ["upd", "qp", "self"],
    ["self", "tests"],
    ["self", "orgn", "try", "new", "er"],
    ["self", "orgn", "try", "new", "for", "de", "er"],
    ["self", "test", "cases"],
    ["self", "some", "v", "upd"],
    ["self", "some", "v", "rd"],
    ["self", "rd", "ids"],
    ["self", "upd", "h"],
    ["self", "rd", "ids", "h"],
    ["self", "last"],
    ["self", "crnt"],
    ["jsonb", "self"],
    ["self", "cr", "for", "query"],
    ["self", "upd", "for", "query", "el"],
    ["self", "dm", "prms"],
    ["self", "dm", "payload"],
    ["self", "dlo", "prms"],
    ["self", "dlo", "payload"],
    ["self", "try", "ro", "er"],
    ["self", "ro", "er", "with", "serde"],
    ["self", "um", "prms"],
    ["self", "um", "payload"],
    ["self", "try", "dlo", "er"],
    ["self", "dlo", "er", "with", "serde"],
    ["self", "h"],
    ["try", "self", "h"],
    ["d", "self"],
    ["d", "self", "if"],
    [
        "self", "rd", "ids", "to2", "dims", "vec", "rd", "inn", "acc"
    ],
    ["self", "gen", "pg", "json", "obj", "mod"],
    ["self", "gen", "pg", "tbl", "mod"]
]);
