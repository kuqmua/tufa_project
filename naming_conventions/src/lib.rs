pub use naming_macros::AsRefStrEnumWithUnitFieldsToUpperCamelCaseStringified;
pub use naming_macros::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified;
pub use naming_macros::AsRefStrEnumWithUnitFieldsToScreamingSnakeCaseStringified;

pub use naming_conventions_common::AsRefStrToUpperCamelCaseStringified;
pub use naming_conventions_common::AsRefStrToUpperCamelCaseTokenStream;
pub use naming_conventions_common::AsRefStrToSnakeCaseStringified;
pub use naming_conventions_common::AsRefStrToSnakeCaseTokenStream;
pub use naming_conventions_common::AsRefStrToScreamingSnakeCaseStringified;
pub use naming_conventions_common::AsRefStrToScreamingSnakeCaseTokenStream;

pub use naming_conventions_common::DisplayToUpperCamelCaseStringified;
pub use naming_conventions_common::DisplayToUpperCamelCaseTokenStream;
pub use naming_conventions_common::DisplayToSnakeCaseStringified;
pub use naming_conventions_common::DisplayToSnakeCaseTokenStream;
pub use naming_conventions_common::DisplayToScreamingSnakeCaseStringified;
pub use naming_conventions_common::DisplayToScreamingSnakeCaseTokenStream;

pub use naming_conventions_common::ToTokensToUpperCamelCaseStringified;
pub use naming_conventions_common::ToTokensToUpperCamelCaseTokenStream;
pub use naming_conventions_common::ToTokensToSnakeCaseStringified;
pub use naming_conventions_common::ToTokensToSnakeCaseTokenStream;
pub use naming_conventions_common::ToTokensToScreamingSnakeCaseStringified;
pub use naming_conventions_common::ToTokensToScreamingSnakeCaseTokenStream;

pub const GITHUB_URL: &str = "https://github.com/kuqmua/tufa_project";
pub const SUPPORTS_ONLY_STRINGIFIED: &str = "supports only";
pub const SYN_FIELDS: &str = "syn::Fields";
pub const SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED: &str = "syn::GenericArgument::Type";
pub const IS_NONE_STRINGIFIED: &str = "is None";
pub const STD_STRINGIFIED: &str = "std";
pub const SQLX_TYPES_UUID_STRINGIFIED: &str = "sqlx::types::Uuid";
pub const FIELD_IDENT_IS_NONE: &str = "field.ident is None";
pub const SYN_TYPE_PATH: &str = "syn::Type::Path";

naming_macros::generate_upper_camel_and_snake_case_stringified_and_token_stream!([
    ["primary", "key"],
    ["serialize", "deserialize"],
    ["with", "serialize", "deserialize"],
    ["error", "occurence"],
    ["code", "occurence"],
    ["is", "none"],
    ["error", "named"],
    ["try", "from"],
    ["from", "str"],
    ["column", "not", "found"],
    ["request", "error"],
    ["row", "not", "found"],
    ["type", "not", "found"],
    ["column", "index", "out", "of", "bounds"],
    ["column", "decode"],
    ["pool", "timed", "out"],
    ["pool", "closed"],
    ["worker", "crashed"],
    ["json", "data", "error"],
    ["json", "syntax", "error"],
    ["missing", "json", "content", "type"],
    ["bytes", "rejection"],
    ["unexpected", "case"],
    ["expected", "type"],
    ["unexpected", "status", "code"],
    ["failed", "to", "get", "response", "text"],
    ["deserialize", "response"],
    ["status", "code"],
    ["response", "text"],
    ["response", "text", "result"],
    ["std", "option", "option"],
    ["wrapper", "vec", "column"],
    ["order", "by"],
    ["primary", "keys"],
    ["not", "unique", "primary", "key"],
    ["response", "variants"],
    ["tvfrr", "extraction", "logic"],
    ["bind", "query"],
    ["into", "serialize", "deserialize", "version"],
    ["checked", "add"],
    ["primary", "key", "from", "row", "and", "rollback"],
    ["non", "existing", "primary", "keys"],
    ["non", "existing", "primary", "keys", "and", "rollback"],
    ["commit", "failed"],
    ["operation", "done", "but", "primary", "key", "inner", "type", "try", "from", "primary", "key", "inner", "type", "with", "serialize", "deserialize", "failed"],
    ["app", "state"],
    ["column", "select"],
    ["options", "try", "from", "sqlx", "row"],
    ["try", "generate", "bind", "increments", "error", "named"],
    ["primary", "key", "try", "from", "sqlx", "row"],
    ["extraction", "result"],
    ["serde", "json", "to", "string"],
    ["operation", "done", "but", "primary", "key", "inner", "type", "try", "from", "primary", "key", "inner", "type", "with", "serialize", "deserialize", "failed", "in", "server"],
    ["operation", "done", "but", "primary", "key", "inner", "type", "try", "from", "primary", "key", "inner", "type", "with", "serialize", "deserialize", "failed", "in", "client"],
    [
        "operation",
        "done",
        "but",
        "primary",
        "key",
        "inner",
        "type",
        "try",
        "from",
        "primary",
        "key",
        "inner",
        "type",
        "with",
        "serialize",
        "deserialize",
        "failed",
        "in",
        "client",
        "error",
        "unnamed"
    ],
    [
        "operation",
        "done",
        "but",
        "primary",
        "key",
        "inner",
        "type",
        "try",
        "from",
        "primary",
        "key",
        "inner",
        "type",
        "with",
        "serialize",
        "deserialize",
        "failed",
        "in",
        "client",
        "many"
    ],
    ["no", "payload", "fields"],
    ["no", "payload", "parameters"],
    ["try", "extract", "value"],
    ["server", "location"],
    ["pg", "connection"],
    ["query", "string"],
    ["binded", "query"],
    ["current", "vec", "len"],
    ["into", "inner", "type", "vec"],
    ["not", "unique", "column"],
    ["get", "axum", "http", "status", "code"],
    ["body", "bytes"],
    ["into", "response"],
    ["primary", "key", "inner", "type", "try", "from", "primary", "key", "inner", "type", "with", "serialize", "deserialize", "in", "client"],
    ["check", "commit"],
    ["check", "commit", "error", "named"],
    ["check", "body", "size"],
    ["source", "handle"],
    ["column", "decode", "index"],
    ["expected", "response"],
    ["postgresql", "crud"],
    ["check", "body", "size", "error", "named"],
    ["create", "many", "additional", "error", "variants"],
    ["create", "one", "additional", "error", "variants"],
    ["read", "many", "additional", "error", "variants"],
    ["read", "one", "additional", "error", "variants"],
    ["update", "many", "additional", "error", "variants"],
    ["update", "one", "additional", "error", "variants"],
    ["delete", "many", "additional", "error", "variants"],
    ["delete", "one", "additional", "error", "variants"],
    ["common", "additional", "error", "variants"],
    ["create", "many", "additional", "route", "logic"],
    ["create", "one", "additional", "route", "logic"],
    ["read", "many", "additional", "route", "logic"],
    ["read", "one", "additional", "route", "logic"],
    ["update", "many", "additional", "route", "logic"],
    ["update", "one", "additional", "route", "logic"],
    ["delete", "many", "additional", "route", "logic"],
    ["delete", "one", "additional", "route", "logic"],
    ["common", "additional", "route", "logic"],
    ["not", "unique", "field", "vec"],
    ["rollback", "error"],
    ["not", "unique", "primary", "key", "with", "serialize", "deserialize"],
    ["not", "unique"],
    ["no", "payload", "fields", "primary", "key"],
    ["no", "primary", "keys"],
    ["row", "and", "rollback"],
    ["postgres", "transaction"],
    ["expected", "primary", "keys"],
    ["results", "vec"],
    ["serde", "json"],
    ["unexpected", "rows", "length"],
    ["unexpected", "rows", "length", "and", "rollback"],
    ["expected", "length"],
    ["got", "length"],
    ["pool", "connection"],
    ["type"],
    ["in"],
    ["as"],
    ["where"],
    ["named"],
    ["unnamed"],
    ["error"],
    ["occurence"],
    ["string"],
    ["parameters"],
    ["payload"],
    ["element"],
    ["try"],
    ["from"],
    ["path"],
    ["key"],
    ["keys"],
    ["value"],
    ["vec"],
    ["reference"],
    ["with"],
    ["serialize"],
    ["deserialize"],
    ["request"],
    ["response"],
    ["variants"],
    ["options"],
    ["code"],
    ["config"],
    ["is"],
    ["none"],
    ["str"],
    ["uuid"],
    ["wrapper"],
    ["possible"],
    ["source"],
    ["display"],
    ["foreign"],
    ["to"],
    ["into"],
    ["get"],
    ["column"],
    ["select"],
    ["order"],
    ["by"],
    ["not"],
    ["found"],
    ["desirable"],
    ["rollback"],
    ["limit"],
    ["offset"],
    ["client"],
    ["server"],
    ["no"],
    ["fields"],
    ["commit"],
    ["begin"],
    ["acc"],
    ["query"],
    ["update"],
    ["set"],
    ["insert"],
    ["values"],
    ["delete"],
    ["and"],
    ["unnest"],
    ["configuration"],
    ["database"],
    ["io"],
    ["tls"],
    ["protocol"],
    ["row"],
    ["index"],
    ["out"],
    ["of"],
    ["bounds"],
    ["decode"],
    ["pool"],
    ["timed"],
    ["closed"],
    ["worker"],
    ["crashed"],
    ["migrate"],
    ["json"],
    ["data"],
    ["syntax"],
    ["missing"],
    ["content"],
    ["bytes"],
    ["rejection"],
    ["expected"],
    ["unexpected"],
    ["case"],
    ["status"],
    ["failed"],
    ["text"],
    ["reqwest"],
    ["headers"],
    ["result"],
    ["serde"],
    ["debug"],
    ["or"],
    ["asc"],
    ["desc"],
    ["unique"],
    ["many"],
    ["std"],
    ["option"],
    ["primary"],
    ["inner"],
    ["existing"],
    ["non"],
    ["current"],
    ["len"],
    ["sqlx"],
    ["returning"],
    ["app"],
    ["state"],
    ["permission"],
    ["tvfrr"],
    ["read"],
    ["logic"],
    ["extraction"],
    ["generated"],
    ["route"],
    ["body"],
    ["env"],
    ["collections"],
    ["postgresql"],
    ["bind"],
    ["version"],
    ["checked"],
    ["add"],
    ["upper"],
    ["snake"],
    ["camel"],
    ["stringified"],
    ["token"],
    ["stream"],
    ["done"],
    ["but"],
    ["operation"],
    ["generate"],
    ["increments"],
    ["increment"],
    ["additional"],
    ["http"],
    ["codes"],
    ["extract"],
    ["location"],
    ["pg"],
    ["connection"],
    ["binded"],
    ["axum"],
    ["check"],
    ["size"],
    ["handle"],
    ["url"],
    ["future"],
    ["field"],
    ["else"],
    ["end"],
    ["when"],
    ["then"],
    ["rows"],
    ["results"],
    ["executor"],
    ["prefix"],
    ["filter"],
    ["empty", "column", "json", "reader"],
    ["not", "unique", "column", "json", "reader"],
    ["generate", "postgresql", "query", "part", "to", "read", "from", "vec"],
    ["generate", "postgresql", "query", "part", "to", "read", "from", "vec", "error", "named"],
    ["generate", "postgresql", "query", "part", "to", "read", "error", "named"],
    ["option", "to", "read"],
    ["options", "to", "read"],
    ["option", "to", "update"],
    ["options", "to", "update"],
    ["options", "to", "update", "try", "generate", "bind", "increments", "error", "named"],
    ["id"],
    ["to", "create"],
    ["reader"],
    ["field", "to", "read"],
    ["field", "to", "update"],
    ["json", "array", "element", "change"],
    ["not", "unique", "id"],
    ["try", "generate", "json", "array", "element", "update", "bind", "increments"],
    ["try", "generate", "json", "array", "element", "update", "bind", "increments", "error", "named"],
    ["try", "generate", "json", "array", "element", "delete", "bind", "increments"],
    ["try", "generate", "json", "array", "element", "delete", "bind", "increments", "error", "named"],
    ["try", "generate", "json", "array", "element", "create", "bind", "increments"],
    ["try", "generate", "json", "array", "element", "create", "bind", "increments", "error", "named"],
    ["std", "vec", "vec", "generic"],
    ["field", "reader"],
    ["fields", "filter", "is", "empty"],
    ["not", "unique", "field", "filter"],
    ["pagination"],
    ["field", "vec"],
    ["option", "to", "update", "try", "generate", "bind", "increments", "error", "named"],
    ["object", "acc"],
    ["std", "option", "option", "object", "acc"],
    ["create", "update", "delete", "check", "fields", "are", "empty"],
    ["not", "unique", "id", "in", "json", "update", "array"],
    ["not", "unique", "id", "in", "json", "delete", "array"],
    ["not", "unique", "id", "in", "json", "update", "and", "delete", "arrays"],
    ["fields", "are", "empty"],
    ["wrap", "into", "jsonb", "build", "object"],
    ["generate", "jsonb", "set", "target"],
    ["generate", "jsonb", "set", "path"],
    ["column", "name", "and", "maybe", "field", "getter", "field", "ident"],
    ["column", "name", "and", "maybe", "field", "getter", "for", "error", "message", "field", "ident"],
    ["generate", "not", "unique", "field"],
    ["all", "fields", "are", "none"],
    ["self"],
    ["option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["try", "generate", "postgresql", "query", "part", "to", "create"],
    ["bind", "value", "to", "postgresql", "query", "part", "to", "create"],
    ["generate", "postgresql", "query", "part", "to", "read"],
    ["try", "generate", "postgresql", "query", "part", "to", "update"],
    ["bind", "value", "to", "postgresql", "query", "part", "to", "update"],
    ["jsonb", "set", "accumulator"],
    ["jsonb", "set", "target"],
    ["jsonb", "set", "path"],
    ["column", "name", "and", "maybe", "field", "getter"],
    ["column", "name", "and", "maybe", "field", "getter", "for", "error", "message"],
    ["field", "ident"],
    ["postgresql", "json", "type"],
    ["postgresql", "json", "type", "try", "generate", "postgresql", "query", "part", "to", "create", "error", "named"],
    ["create"],
    ["dotenv"],
    ["std", "env", "var", "error"],
    ["env", "var", "name"],
    ["try", "from", "std", "env", "var", "ok"],
    ["table", "name"],
    //todo remove "second" later
    ["generate", "postgresql", "crud", "second", "primary", "key"],
    ["std", "default", "default", "but", "std", "option", "option", "is", "always", "some", "and", "std", "vec", "vec", "always", "contains", "one", "element"],
    ["default", "but", "std", "option", "option", "is", "always", "some", "and", "std", "vec", "vec", "always", "contains", "one", "element"],
    ["all", "enum", "variants", "array", "std", "default", "default", "but", "std", "option", "option", "is", "always", "some", "and", "std", "vec", "vec", "always", "contains", "one", "element"],
    ["all", "enum", "variants", "array", "default", "but", "std", "option", "option", "is", "always", "some", "and", "std", "vec", "vec", "always", "contains", "one", "element"]
]);

#[derive(Debug, Clone, Copy)]
pub struct HashMap;
#[derive(Debug, Clone, Copy)]
pub struct HashMapUpperCamelCase;
impl std::fmt::Display for HashMapUpperCamelCase {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "HashMap")
    }
}
impl quote::ToTokens for HashMapUpperCamelCase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {HashMap}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HashMapSnakeCase;
impl std::fmt::Display for HashMapSnakeCase {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "hashmap")
    }
}
impl quote::ToTokens for HashMapSnakeCase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {hashmap}.to_tokens(tokens)
    }
}

pub trait StdFmtDisplayPlusQuoteToTokens: std::fmt::Display + quote::ToTokens {}
impl<T> StdFmtDisplayPlusQuoteToTokens for T where T: std::fmt::Display + quote::ToTokens {}

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
    ["try", "self", "generated", "route", "logic", "error", "named"],
    ["try", "self", "generated", "route", "logic", "desirable"],
    ["try", "self", "route", "logic"],
    ["try", "self", "route", "logic", "response", "variants"],
    ["try", "self", "route", "logic", "error", "named"],
    ["try", "self", "route", "logic", "error", "named", "with", "serialize", "deserialize"],
    ["try", "self", "generated", "route", "logic", "error", "named", "with", "serialize", "deserialize"],
    ["self", "payload", "example", "route", "logic"],
    ["self", "to", "create"],
    ["self", "to", "read"],
    ["self", "to", "update"],
    ["self", "to", "delete"],
    ["self", "options", "to", "read"],
    ["object", "self"],
    ["std", "option", "option", "object", "self"],
    ["object", "with", "id", "self"],
    ["std", "vec", "vec", "object", "with", "id", "self"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self"],
    ["self", "field", "reader"],
    ["self", "field", "to", "read", "without", "id"],
    ["self", "field", "to", "read", "with", "id"],
    ["object", "self", "field", "reader"],
    ["object", "with", "id", "self", "field", "reader"],
    ["std", "option", "option", "object", "self", "field", "reader"],
    ["std", "vec", "vec", "object", "with", "id", "self", "field", "reader"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "field", "reader"],
    ["self", "try", "new", "error", "named"],
    ["object", "self", "options", "to", "read"],
    ["std", "option", "option", "object", "self", "options", "to", "read"],
    ["std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read"],
    ["std","option", "option","std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read"],
    ["object", "self", "to", "create"],
    ["std", "option", "option", "object", "self", "to", "create"],
    ["std", "vec", "vec", "object", "with", "id", "self", "to", "create"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to", "create"],
    ["object", "with", "id", "self", "options", "to", "read"],
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
    ["std", "option", "option", "object", "self", "options", "to", "read", "origin"],
    ["std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read", "origin"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read", "origin"],
    ["self", "option", "to", "update"],
    ["object", "self", "option", "to", "update"],
    ["std", "option", "option", "object", "self", "option", "to", "update", "origin"],
    ["std", "option", "option", "object", "self", "option", "to", "update"],
    ["std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "origin"],
    ["std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "origin"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update"],
    ["object", "with", "id", "self", "option", "to", "update"],
    ["std", "vec", "vec", "object", "with", "id", "self", "options", "to", "update"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "options", "to", "update"],
    ["object", "self", "option", "to", "update", "origin"],
    ["std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change"],
    ["self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named", "with", "serialize", "deserialize"],
    ["self", "option", "to", "update", "origin"],
    ["self", "json", "array", "change"],
    ["self", "to", "create", "origin"],
    ["self", "options", "to", "update"],
    ["self", "to", "create", "with", "generated", "id"],
    ["self", "to", "create", "without", "generated", "id"],
    ["self", "json", "array", "change", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["self", "field", "to", "update"],
    ["self", "generate", "postgresql", "query", "part", "to", "read", "error", "named"],
    ["self", "try", "generate", "json", "array", "element", "update", "bind", "increments", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change", "try", "new", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change", "try", "new", "error", "named"],
    ["not", "unique", "field", "self"],
    ["self", "options", "to", "update", "try", "new", "error", "named"],
    ["self", "options", "to", "read", "without", "id"],
    ["self", "options", "to", "read", "with", "id"],
    ["self", "option", "to", "update", "try", "new", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read", "try", "new", "error", "named"],
    ["self", "generate", "postgresql", "query", "part", "to", "read", "from", "vec", "error", "named"],
    ["std", "option", "option", "object", "self", "option", "to", "update", "try", "new", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read", "try", "new", "error", "named"],
    ["self", "field", "to", "read"],
    ["self", "options", "to", "read", "with", "or", "without", "id", "try", "from", "error", "named"],
    ["self", "field", "reader", "try", "new", "error", "named"],
    ["object", "self", "field", "reader", "try", "new", "error", "named"],
    ["std", "option", "option", "object", "self", "field", "reader", "try", "new", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "field", "reader", "try", "new", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "field", "reader", "try", "new", "error", "named"],
    ["object", "self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["object", "self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named", "with", "serialize", "deserialize"],
    ["std", "option", "option", "object", "self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["self", "with", "serialize", "deserialize"],
    ["self", "try", "from", "env", "error", "named"],
    ["get", "self"],
    ["try", "from", "std", "env", "var", "ok", "self", "error", "named"],
    ["self", "options"],
    ["error", "self"],
    ["not", "unique", "self"],
    ["is", "self", "update", "exist"],
    ["self", "column"],
    ["self", "column", "read", "permission"],
    ["self", "where"]
    
]);

pub trait SwaggerUrlPathSelfQuotesStringified {
    fn swagger_url_path_self_quotes_stringified(&self, table_name_stringified: &str) -> std::string::String;
}

impl<T> SwaggerUrlPathSelfQuotesStringified for T
where
    T: naming_conventions_common::AsRefStrToSnakeCaseStringified,
{
    fn swagger_url_path_self_quotes_stringified(&self, table_name_stringified: &str) -> std::string::String {
        generate_quotes::double_quotes_stringified(&format!("/{}/{}", table_name_stringified, self.new(),))
    }
}

pub trait SwaggerUrlPathSelfQuotesTokenStream {
    fn swagger_url_path_self_quotes_token_stream(&self, table_name_stringified: &str) -> proc_macro2::TokenStream;
}

impl<T> SwaggerUrlPathSelfQuotesTokenStream for T
where
    T: SwaggerUrlPathSelfQuotesStringified,
{
    fn swagger_url_path_self_quotes_token_stream(&self, table_name_stringified: &str) -> proc_macro2::TokenStream {
        let value = self.swagger_url_path_self_quotes_stringified(table_name_stringified);
        value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait UrlHandleSelfSnakeCaseStringified {
    fn url_handle_self_snake_case_stringified(&self, table_name_stringified: &str) -> std::string::String;
}

impl<T> UrlHandleSelfSnakeCaseStringified for T
where
    T: naming_conventions_common::AsRefStrToSnakeCaseStringified,
{
    fn url_handle_self_snake_case_stringified(&self, table_name_stringified: &str) -> std::string::String {
        format!("\"{{}}/{}/{}\"", table_name_stringified, self.new())
    }
}

pub trait UrlHandleSelfSnakeCaseTokenStream {
    fn url_handle_self_snake_case_token_stream(&self, table_name_stringified: &str) -> proc_macro2::TokenStream;
}

impl<T> UrlHandleSelfSnakeCaseTokenStream for T
where
    T: UrlHandleSelfSnakeCaseStringified,
{
    fn url_handle_self_snake_case_token_stream(&self, table_name_stringified: &str) -> proc_macro2::TokenStream {
        let value = self.url_handle_self_snake_case_stringified(table_name_stringified);
        value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}
// fn generate_url_handle_token_stream(
//     table_name_stringified: &str,
//     operation_name_snake_case_stringified: &str,
//     proc_macro_name_upper_camel_case_ident_stringified: &str,
// ) -> proc_macro2::TokenStream {
//     let url_handle_stringified = format!("\"{{}}/{table_name_stringified}/{operation_name_snake_case_stringified}\"");
//     url_handle_stringified.parse::<proc_macro2::TokenStream>()
//     .unwrap_or_else(|_| panic!("{url_handle_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
// }



















////////////////////////////////////////////////////
// #[derive(Debug)]
// pub struct GenericSelfFieldReaderUpperCamelCase(std::string::String);
// impl GenericSelfFieldReaderUpperCamelCase {
//     fn wrap(value: &dyn std::fmt::Display) -> Self {
//         Self(format!("Generic{value}FieldReader"))
//     }
//     pub fn from_dyn_std_fmt_display(value: &dyn std::fmt::Display) -> Self {
//         Self::wrap(&generate_quotes::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&value.to_string()))
//     }
//     pub fn from_dyn_quote_to_tokens(value: &dyn quote::ToTokens) -> Self {
//         Self::wrap(&generate_quotes::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&{
//             let mut tokens = proc_macro2::TokenStream::new();
//             quote::ToTokens::to_tokens(&value, &mut tokens);
//             tokens
//         }.to_string()))
//     }
// }
// impl std::fmt::Display for GenericSelfFieldReaderUpperCamelCase {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{}", self.0)
//     }
// }
// impl quote::ToTokens for GenericSelfFieldReaderUpperCamelCase {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         let value_stringified = self.to_string();
//         let value_token_stream = value_stringified.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("failed to parse stringified GenericSelfFieldReaderUpperCamelCase into proc_macro2::TokenStream: {value_stringified}"));
//         value_token_stream.to_tokens(tokens)
//     }
// }


// #[derive(Debug)]
// pub struct GenericSelfFieldReaderSnakeCase(std::string::String);
// impl GenericSelfFieldReaderSnakeCase {
//     fn wrap(value: &dyn std::fmt::Display) -> Self {
//         Self(format!("generic_{value}_field_reader"))
//     }
//     pub fn from_dyn_std_fmt_display(value: &dyn std::fmt::Display) -> Self {
//         Self::wrap(&generate_quotes::naming_conventions::AsRefStrToSnakeCaseStringified::new(&value.to_string()))
//     }
//     pub fn from_dyn_quote_to_tokens(value: &dyn quote::ToTokens) -> Self {
//         Self::wrap(&generate_quotes::naming_conventions::AsRefStrToSnakeCaseStringified::new(&{
//             let mut tokens = proc_macro2::TokenStream::new();
//             quote::ToTokens::to_tokens(&value, &mut tokens);
//             tokens
//         }.to_string()))
//     }
// }
// impl std::fmt::Display for GenericSelfFieldReaderSnakeCase {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{}", self.0)
//     }
// }
// impl quote::ToTokens for GenericSelfFieldReaderSnakeCase {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         let value_stringified = self.to_string();
//         let value_token_stream = value_stringified.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("failed to parse stringified GenericSelfFieldReaderSnakeCase into proc_macro2::TokenStream: {value_stringified}"));
//         value_token_stream.to_tokens(tokens)
//     }
// }





/////////
// pub struct StdOptionOptionGenericAccUpperCamelCase;
// impl std::fmt::Display for StdOptionOptionGenericAccUpperCamelCase {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(formatter, "StdOptionOptionGenericAcc")
//     }
// }
// impl quote::ToTokens for StdOptionOptionGenericAccUpperCamelCase {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         quote::quote! { StdOptionOptionGenericAcc }.to_tokens(tokens)
//     }
// }
// pub struct StdOptionOptionGenericAccSnakeCase;
// impl std::fmt::Display for StdOptionOptionGenericAccSnakeCase {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(formatter, "std_option_option_generic_acc")
//     }
// }
// impl quote::ToTokens for StdOptionOptionGenericAccSnakeCase {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         quote::quote! { std_option_option_generic_acc }.to_tokens(tokens)
//     }
// }
////////


// pub trait GenericSelfFieldReaderUpperCamelCaseStringified {
//     fn generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String;
// }
// impl<T> GenericSelfFieldReaderUpperCamelCaseStringified for T
// where
//     T: generate_quotes::naming_conventions::ToUpperCamelCaseStringified,
// {
//     fn generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String {
//         format!("Generic{}FieldReader", self.to_upper_camel_case_stringified(),)
//     }
// }
// pub trait GenericSelfFieldReaderSnakeCaseStringified {
//     fn generic_self_field_reader_snake_case_stringified(&self) -> std::string::String;
// }
// impl<T> GenericSelfFieldReaderSnakeCaseStringified for T
// where
//     T: generate_quotes::naming_conventions::ToSnakeCaseStringified,
// {
//     fn generic_self_field_reader_snake_case_stringified(&self) -> std::string::String {
//         format!("generic_{}_field_reader", self.to_snake_case_stringified(),)
//     }
// }
// pub trait GenericSelfFieldReaderUpperCamelCaseTokenStream {
//     fn generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> GenericSelfFieldReaderUpperCamelCaseTokenStream for T
// where
//     T: GenericSelfFieldReaderUpperCamelCaseStringified,
// {
//     fn generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.generic_self_field_reader_upper_camel_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait GenericSelfFieldReaderSnakeCaseTokenStream {
//     fn generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> GenericSelfFieldReaderSnakeCaseTokenStream for T
// where
//     T: GenericSelfFieldReaderSnakeCaseStringified,
// {
//     fn generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.generic_self_field_reader_snake_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified for T
// where
//     T: quote::ToTokens,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String {
//         format!("Generic{}FieldReader", generate_quotes::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&quote::quote! { #self }.to_string()),)
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified(&self) -> std::string::String;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified for T
// where
//     T: quote::ToTokens,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified(&self) -> std::string::String {
//         format!("generic_{}_field_reader", generate_quotes::naming_conventions::AsRefStrToSnakeCaseStringified::new(&quote::quote! { #self }.to_string()),)
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseTokenStream {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseTokenStream for T
// where
//     T: ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseTokenStream {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseTokenStream for T
// where
//     T: ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait GenericSelfFieldReaderUpperCamelCaseStringified {
//     fn generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String;
// }
// impl<T> GenericSelfFieldReaderUpperCamelCaseStringified for T
// where
//     T: generate_quotes::naming_conventions::ToUpperCamelCaseStringified,
// {
//     fn generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String {
//         format!("Generic{}FieldReader", self.to_upper_camel_case_stringified(),)
//     }
// }
// pub trait GenericSelfFieldReaderSnakeCaseStringified {
//     fn generic_self_field_reader_snake_case_stringified(&self) -> std::string::String;
// }
// impl<T> GenericSelfFieldReaderSnakeCaseStringified for T
// where
//     T: generate_quotes::naming_conventions::ToSnakeCaseStringified,
// {
//     fn generic_self_field_reader_snake_case_stringified(&self) -> std::string::String {
//         format!("generic_{}_field_reader", self.to_snake_case_stringified(),)
//     }
// }
// pub trait GenericSelfFieldReaderUpperCamelCaseTokenStream {
//     fn generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> GenericSelfFieldReaderUpperCamelCaseTokenStream for T
// where
//     T: GenericSelfFieldReaderUpperCamelCaseStringified,
// {
//     fn generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.generic_self_field_reader_upper_camel_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait GenericSelfFieldReaderSnakeCaseTokenStream {
//     fn generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> GenericSelfFieldReaderSnakeCaseTokenStream for T
// where
//     T: GenericSelfFieldReaderSnakeCaseStringified,
// {
//     fn generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.generic_self_field_reader_snake_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified for T
// where
//     T: quote::ToTokens,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String {
//         format!("Generic{}FieldReader", generate_quotes::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&quote::quote! { #self }.to_string()),)
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified(&self) -> std::string::String;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified for T
// where
//     T: quote::ToTokens,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified(&self) -> std::string::String {
//         format!("generic_{}_field_reader", generate_quotes::naming_conventions::AsRefStrToSnakeCaseStringified::new(&quote::quote! { #self }.to_string()),)
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseTokenStream {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseTokenStream for T
// where
//     T: ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseTokenStream {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseTokenStream for T
// where
//     T: ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }

////////////////////////////////////

