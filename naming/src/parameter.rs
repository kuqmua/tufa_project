naming_macros::generate_self_upper_camel_and_snake_case_stringified_and_token_stream!([
    ["self", "parameters"],
    ["self", "payload"],
    ["self", "payload", "with", "serialize", "deserialize"],
    ["self", "payload", "try", "from", "self", "payload", "with", "serialize", "deserialize"],
    ["self", "payload", "with", "serialize", "deserialize", "try", "from", "self", "payload"],
    ["self", "payload", "with", "serialize", "deserialize", "try", "from", "self", "payload", "error", "named"],
    ["try", "self"],
    ["try", "self", "response", "variants"],
    ["self", "payload", "element", "with", "serialize", "deserialize"],
    ["self", "payload", "element"],
    ["self", "payload", "element", "try", "from", "self", "payload", "with", "serialize", "deserialize"],
    ["self", "payload", "element", "try", "from", "self", "payload", "element", "with", "serialize", "deserialize", "error", "named"],
    ["try", "self", "error", "named"],
    ["try", "self", "request", "error"],
    ["self", "payload", "try", "from", "self", "payload", "with", "serialize", "deserialize", "error", "named"],
    ["try", "self", "with", "serialize", "deserialize"],
    ["tvfrr", "extraction", "logic", "try", "self"],
    ["try", "self", "generated", "logic", "error", "named"],
    ["try", "self", "generated", "logic", "desirable"],
    ["try", "self", "logic"],
    ["try", "self", "logic", "response", "variants"],
    ["try", "self", "logic", "error", "named"],
    ["self", "error", "named", "with", "serialize", "deserialize"],
    ["try", "self", "generated", "logic", "error", "named", "with", "serialize", "deserialize"],
    ["self", "payload", "example"],
    ["self", "create"],
    ["self", "read"],
    ["self", "read", "inner"],
    ["self", "update"],
    ["self", "delete"],
    ["object", "self"],
    ["std", "option", "option", "object", "self"],
    ["object", "with", "id", "self"],
    ["std", "vec", "vec", "object", "with", "id", "self"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self"],
    ["self", "select"],
    ["self", "select", "without", "id"],
    ["self", "select", "with", "id"],
    ["object", "self", "select"],
    ["object", "with", "id", "self", "select"],
    ["std", "option", "option", "object", "self", "select"],
    ["std", "vec", "vec", "object", "with", "id", "self", "select"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "select"],
    ["self", "try", "new", "error", "named"],
    ["object", "self", "read"],
    ["std", "option", "option", "object", "self", "read"],
    ["std", "vec", "vec", "object", "with", "id", "self", "read"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "read"],
    ["object", "self", "to", "create"],
    ["std", "option", "option", "object", "self", "to", "create"],
    ["std", "vec", "vec", "object", "with", "id", "self", "to", "create"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to", "create"],
    ["object", "with", "id", "self", "read"],
    ["object", "with", "id", "self", "to", "create"],
    ["object", "self", "reader"],
    ["object", "with", "id", "self", "reader"],
    ["std", "option", "option", "object", "self", "reader"],
    ["std", "vec", "vec", "object", "with", "id", "self", "reader"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "reader"],
    ["self", "reader"],
    ["std", "option", "option", "object", "self", "to", "create", "origin"],
    ["std", "vec", "vec", "object", "with", "id", "self", "to", "create", "origin"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to", "create", "origin"],
    ["std", "option", "option", "object", "self", "read", "origin"],
    ["std", "vec", "vec", "object", "with", "id", "self", "read", "origin"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "read", "origin"],
    ["self", "option", "to", "update"],
    ["self", "option", "to", "update", "origin"],
    ["object", "self", "option", "to", "update"],
    ["object", "self", "option", "to", "update", "origin"],
    ["std", "option", "option", "object", "self", "option", "to", "update", "origin"],
    ["std", "option", "option", "object", "self", "option", "to", "update"],
    ["std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "origin"],
    ["std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "origin"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update"],
    ["object", "with", "id", "self", "option", "to", "update"],
    ["self", "update", "with", "id"],
    ["std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change"],
    ["self", "option", "to", "update", "try", "generate", "postgresql", "json", "type", "error", "named"],
    ["self", "option", "to", "update", "try", "generate", "postgresql", "json", "type", "error", "named", "with", "serialize", "deserialize"],
    ["self", "json", "array", "change"],
    ["self", "to", "create", "origin"],
    ["self", "to", "create", "with", "generated", "id"],
    ["self", "to", "create", "without", "generated", "id"],
    ["self", "json", "array", "change", "try", "generate", "error", "named"],
    ["self", "field", "to", "update"],
    ["self", "generate", "postgresql", "json", "type", "to", "read", "error", "named"],
    ["self", "try", "generate", "json", "array", "element", "update", "bind", "increments", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change", "try", "new", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change", "try", "new", "error", "named"],
    ["not", "unique", "field", "self"],
    ["self", "read", "without", "id"],
    ["self", "read", "with", "id"],
    ["self", "option", "to", "update", "try", "new", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "read", "try", "new", "error", "named"],
    ["self", "generate", "postgresql", "json", "type", "to", "read", "from", "vec", "error", "named"],
    ["std", "option", "option", "object", "self", "option", "to", "update", "try", "new", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "read", "try", "new", "error", "named"],
    ["self", "read", "with", "or", "without", "id", "try", "from", "error", "named"],
    ["self", "select", "try", "new", "error", "named"],
    ["object", "self", "select", "try", "new", "error", "named"],
    ["std", "option", "option", "object", "self", "select", "try", "new", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "select", "try", "new", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "select", "try", "new", "error", "named"],
    ["object", "self", "option", "to", "update", "try", "generate", "error", "named"],
    ["object", "self", "option", "to", "update", "try", "generate", "postgresql", "json", "type", "error", "named", "with", "serialize", "deserialize"],
    ["std", "option", "option", "object", "self", "option", "to", "update", "try", "generate", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "try", "generate", "postgresql", "json", "type", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "try", "generate", "postgresql", "json", "type", "error", "named"],
    ["self", "with", "serialize", "deserialize"],
    ["self", "try", "from", "env", "error", "named"],
    ["get", "self"],
    ["try", "from", "std", "env", "var", "ok", "self", "error", "named"],
    ["self", "options"],
    ["error", "self"],
    ["not", "unique", "self"],
    ["is", "self", "update", "exist"],
    ["self", "column", "read", "permission"],
    ["self", "where"],
    ["std", "option", "option", "self"],
    ["where", "std", "option", "option", "self"],
    ["sqlx", "types", "json", "object", "self"],
    ["std", "option", "option", "sqlx", "types", "json", "object", "self"],
    ["sqlx", "types", "json", "std", "option", "option", "object", "self"],
    ["std", "option", "option", "sqlx", "types", "json", "std", "option", "option", "object", "self"],
    ["sqlx", "types", "json", "std", "vec", "vec", "object", "with", "id", "self"],
    ["std", "option", "option", "sqlx", "types", "json", "std", "vec", "vec", "object", "with", "id", "self"],
    ["sqlx", "types", "json", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self"],
    ["std", "option", "option", "sqlx", "types", "json", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self"],
    ["object", "self", "column"],
    ["std", "option", "option", "object", "self", "column"],
    ["std", "vec", "vec", "object", "with", "id", "self", "column"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "column"],
    ["postgresql", "json", "type", "self", "to", "create"],
    ["object", "self", "create"],
    ["std", "option", "option", "object", "self", "create"],
    ["std", "vec", "vec", "object", "with", "id", "self", "create"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "create"],
    ["postgresql", "json", "type", "self", "select"],
    ["postgresql", "json", "type", "object", "self", "select"],
    ["postgresql", "json", "type", "std", "option", "option", "object", "self", "select"],
    ["postgresql", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "select"],
    ["postgresql", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "select"],
    ["postgresql", "json", "type", "self", "select", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "object", "self", "select", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "std", "option", "option", "object", "self", "select", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "select", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "select", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "read"],
    ["postgresql", "json", "type", "self", "read", "without", "id"],
    ["postgresql", "json", "type", "self", "read", "with", "id"],
    ["self", "update", "element"],
    ["postgresql", "json", "type", "self", "option", "to", "update"],
    ["self", "update", "error", "named"],
    ["self", "update", "try", "new", "error", "named"],
    ["object", "self", "update"],
    ["std", "option", "option", "object", "self", "update"],
    ["std", "vec", "vec", "object", "with", "id", "self", "update"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "update"],
    ["std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "try", "generate", "error", "named"],
    [
        "postgresql",
        "json",
        "type",
        "std",
        "option",
        "option",
        "std",
        "vec",
        "vec",
        "object",
        "with",
        "id",
        "self",
        "option",
        "to",
        "update",
        "try",
        "generate",
        "postgresql",
        "json",
        "type",
        "error",
        "named"
    ],
    ["postgresql", "json", "type", "object", "self", "option", "to", "update", "try", "generate", "postgresql", "json", "type", "error", "named", "with", "serialize", "deserialize"],
    ["postgresql", "json", "type", "self", "option", "to", "update", "try", "generate", "postgresql", "json", "type", "error", "named", "with", "serialize", "deserialize"],
    ["postgresql", "json", "type", "self", "read", "with", "or", "without", "id", "try", "from", "error", "named"],
    ["postgresql", "type", "object", "self"],
    ["postgresql", "type", "std", "option", "option", "object", "self"],
    ["postgresql", "type", "std", "vec", "vec", "object", "with", "id", "self"],
    ["postgresql", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self"],
    ["postgresql", "type", "object", "self", "to", "create"],
    ["postgresql", "type", "std", "option", "option", "object", "self", "to", "create"],
    ["postgresql", "type", "std", "vec", "vec", "object", "with", "id", "self", "to", "create"],
    ["postgresql", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to", "create"],
    ["postgresql", "type", "object", "self", "to", "read"],
    ["postgresql", "type", "std", "option", "option", "object", "self", "to", "read"],
    ["postgresql", "type", "std", "vec", "vec", "object", "with", "id", "self", "to", "read"],
    ["postgresql", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to", "read"],
    ["postgresql", "type", "object", "self", "to", "update"],
    ["postgresql", "type", "std", "option", "option", "object", "self", "to", "update"],
    ["postgresql", "type", "std", "vec", "vec", "object", "with", "id", "self", "to", "update"],
    ["postgresql", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to", "update"],
    ["postgresql", "type", "object", "self", "where"],
    ["postgresql", "type", "std", "option", "option", "object", "self", "where"],
    ["postgresql", "type", "std", "vec", "vec", "object", "with", "id", "self", "where"],
    ["postgresql", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "where"],
    ["postgresql", "type", "self", "column"],
    ["postgresql", "type", "self", "to", "create"],
    ["postgresql", "type", "self", "to", "read"],
    ["postgresql", "type", "self", "to", "update"],
    ["postgresql", "type", "self", "where"],
    ["postgresql", "type", "self", "to", "delete"],
    ["self", "as", "json"],
    ["self", "as", "json", "not", "null"],
    ["self", "as", "jsonb"],
    ["self", "as", "jsonb", "not", "null"],
    ["postgresql", "json", "type", "object", "self"],
    ["postgresql", "json", "type", "std", "option", "option", "object", "self"],
    ["postgresql", "json", "type", "std", "vec", "vec", "object", "with", "id", "self"],
    ["postgresql", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self"],
    ["postgresql", "json", "type", "object", "self", "read", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "std", "option", "option", "object", "self", "read", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "read", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "read", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "object", "self", "reader"],
    ["postgresql", "json", "type", "std", "option", "option", "object", "self", "reader"],
    ["postgresql", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "reader"],
    ["postgresql", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "reader"],
    ["postgresql", "type", "self", "to", "update", "query", "part", "error", "named"],
    ["postgresql", "type", "self", "as", "json", "to", "update", "query", "part", "error", "named"],
    ["postgresql", "type", "self", "as", "json", "not", "null", "to", "update", "query", "part", "error", "named"],
    ["postgresql", "type", "self", "where", "element"],
    ["self", "where", "element"],
    ["self", "where", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "equal"],
    ["postgresql", "type", "self", "where", "element", "greater", "than"],
    ["postgresql", "type", "self", "where", "element", "between"],
    ["postgresql", "type", "self", "where", "element", "between", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "in"],
    ["postgresql", "type", "self", "where", "element", "in", "try", "new", "error", "named"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "equal"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "greater", "than"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "between"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "in"],
    ["std", "option", "option", "self", "where", "element"],
    ["postgresql", "type", "self", "where", "element", "case", "sensitive", "regular", "expression"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "case", "sensitive", "regular", "expression"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "case", "insensitive", "regular", "expression"],
    ["postgresql", "type", "self", "where", "element", "case", "insensitive", "regular", "expression"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "hexadecimal", "notation", "equal"],
    ["postgresql", "type", "self", "where", "element", "hexadecimal", "notation", "equal"],
    ["postgresql", "type", "self", "where", "element", "length", "more", "than"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "length", "more", "than"],
    ["postgresql", "type", "self", "where", "element", "length", "more", "than", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "equal", "to", "encoded", "string", "representation"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "equal", "to", "encoded", "string", "representation"],
    ["postgresql", "type", "self", "where", "element", "value", "is", "contained", "within", "range"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "value", "is", "contained", "within", "range"],
    ["postgresql", "type", "self", "where", "element", "contains", "another", "range"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "contains", "another", "range"],
    ["postgresql", "type", "std", "option", "option", "self", "where", "element", "is", "null"],
    ["postgresql", "type", "self", "where", "element", "named"],
    ["self", "where", "element", "range", "length", "try", "new", "error", "named"],
    ["self", "where", "element", "range", "length"],
    ["self", "nullable"],
    ["self", "not", "null"],
    ["postgresql", "type", "self", "not", "null", "to", "create"],
    ["postgresql", "type", "self", "not", "null", "to", "read"],
    ["postgresql", "type", "self", "not", "null", "to", "update"],
    ["postgresql", "type", "self", "not", "null", "to", "delete"],
    ["self", "not", "null", "to", "delete"],
    ["postgresql", "type", "self", "where", "element", "position", "equals", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "position", "equals"],
    ["self", "as", "json", "nullable"],
    ["self", "as", "jsonb", "nullable"],
    ["object", "self", "where", "element"],
    ["std", "option", "option", "object", "self", "where", "element"],
    ["std", "vec", "vec", "object", "with", "id", "self", "where", "element"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "where", "element"],
    ["object", "self", "where"],
    ["std", "option", "option", "object", "self", "where"],
    ["std", "vec", "vec", "object", "with", "id", "self", "where"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "where"],
    ["postgresql", "json", "type", "self", "where", "element"],
    ["postgresql", "json", "type", "self", "where"],
    ["postgresql", "json", "type", "self", "where", "element", "equal"],
    ["postgresql", "json", "type", "self", "where", "element", "between"],
    ["postgresql", "json", "type", "self", "where", "element", "between", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "where", "element", "in"],
    ["postgresql", "json", "type", "self", "where", "element", "in", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "case", "sensitive", "regular", "expression", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "where", "element", "case", "sensitive", "regular", "expression"],
    ["postgresql", "json", "type", "self", "where", "element", "case", "sensitive", "regular", "expression", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "case", "insensitive", "regular", "expression", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "where", "element", "case", "insensitive", "regular", "expression", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "where", "element", "case", "insensitive", "regular", "expression"],
    ["postgresql", "json", "type", "std", "option", "option", "self", "where", "element", "is", "null"],
    ["postgresql", "json", "type", "self", "where", "element", "is", "null"],
    ["postgresql", "json", "type", "std", "option", "option", "self", "to", "create"],
    ["postgresql", "json", "type", "std", "option", "option", "self", "select"],
    ["postgresql", "json", "type", "std", "option", "option", "self", "read"],
    ["postgresql", "json", "type", "std", "option", "option", "self", "where", "element"],
    ["postgresql", "json", "type", "std", "option", "option", "self", "where"],
    ["postgresql", "json", "type", "self", "where", "element", "length", "more", "than", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "where", "element", "length", "more", "than"],
    ["postgresql", "json", "type", "self", "where", "element", "position", "equals", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "where", "element", "position", "equals"],
    ["postgresql", "type", "self", "where", "element", "bit", "vec", "position", "equals", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "where", "element", "bit", "vec", "position", "equals"],
    ["postgresql", "json", "type", "self", "where", "element", "bit", "vec", "position", "equals", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "bit", "vec", "position", "equals"],
    ["postgresql", "type", "self", "where", "element", "position", "greater", "than", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "where", "element", "position", "greater", "than", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "position", "greater", "than"],
    ["postgresql", "json", "type", "self", "where", "element", "position", "greater", "than"],
    ["postgresql", "json", "type", "self", "where", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "position", "case", "sensitive", "regular", "expression"],
    ["postgresql", "json", "type", "self", "where", "element", "position", "case", "sensitive", "regular", "expression"],
    ["postgresql", "type", "self", "where", "element", "position", "case", "sensitive", "regular", "expression", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "where", "element", "position", "case", "sensitive", "regular", "expression", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "position", "case", "insensitive", "regular", "expression"],
    ["postgresql", "json", "type", "self", "where", "element", "position", "case", "insensitive", "regular", "expression"],
    ["postgresql", "type", "self", "where", "element", "position", "case", "insensitive", "regular", "expression", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "where", "element", "position", "case", "insensitive", "regular", "expression", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "contains", "all", "elements", "of", "array", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "self", "where", "element", "contains", "all", "elements", "of", "array", "try", "new", "error", "named"],
    ["postgresql", "type", "self", "where", "element", "contains", "all", "elements", "of", "array"],
    ["postgresql", "json", "type", "self", "where", "element", "contains", "all", "elements", "of", "array"],
    ["self", "where", "element", "bit", "vec", "position", "equals", "try", "new", "error", "named"],
    ["self", "where", "element", "second", "dimension"],
    ["self", "visitor"],
    ["self", "not", "null", "try", "new", "error", "named"],
    ["self", "to", "update", "query", "part", "error", "named"],
    ["self", "length"],
    ["vec", "self", "array", "not", "null"],
    ["vec", "self", "array", "nullable"],
    ["self", "option", "to", "update", "try", "generate", "error", "named"],
    ["postgresql", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "try", "generate", "error", "named"],
    ["postgresql", "type", "where", "element", "self"],
    ["postgresql", "type", "where", "element", "self", "try", "new", "error", "named"],
    ["postgresql", "type", "where", "element", "self", "try", "new", "error", "named", "with", "serialize", "deserialize"],
    ["postgresql", "json", "type", "where", "element", "self"],
    ["postgresql", "json", "type", "where", "element", "self", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "where", "element", "self", "try", "new", "error", "named", "with", "serialize", "deserialize"],
    ["self", "without", "id", "read"],
    ["self", "with", "id", "read"],
    ["self", "without", "id", "read", "try", "from", "error", "named"],
    ["self", "with", "id", "read", "try", "from", "error", "named"],
    ["self", "table", "type", "declaration"],
    ["self", "not", "null", "origin"],
    ["self", "nullable", "origin"],
    ["self", "origin"],
    ["self", "vec"],
    ["self", "postgresql", "type"],
    ["self", "with", "id"],
    ["self", "select", "element"],
    ["self", "with", "id", "select", "element"],
    ["self", "with", "id", "update", "element"],
    ["self", "with", "id", "where", "element"],
    ["self", "read", "try", "from", "error", "named"],
    ["element", "self"],
    ["self", "prepare", "postgresql", "error", "named"],
    ["self", "where", "many"],
    ["self", "where", "many", "try", "new", "error", "named"],
    ["std", "option", "option", "self", "where", "many"],
    ["try", "from", "sqlx", "postgres", "pg", "row", "with", "not", "empty", "unique", "enum", "vec", "self", "select"],
    ["update", "query", "part", "self"],
    ["self", "tests"],
    ["self", "origin", "try", "new", "error", "named"],
    ["self", "origin", "try", "new", "for", "deserialize", "error", "named"]
]);
