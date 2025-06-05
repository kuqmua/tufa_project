pub mod parameter;

pub use naming_macros::AsRefStrEnumWithUnitFieldsToScreamingSnakeCaseStringified;
pub use naming_macros::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified;
pub use naming_macros::AsRefStrEnumWithUnitFieldsToUpperCamelCaseStringified;

pub use naming_common::AsRefStrToScreamingSnakeCaseStringified;
pub use naming_common::AsRefStrToScreamingSnakeCaseTokenStream;
pub use naming_common::AsRefStrToSnakeCaseStringified;
pub use naming_common::AsRefStrToSnakeCaseTokenStream;
pub use naming_common::AsRefStrToUpperCamelCaseStringified;
pub use naming_common::AsRefStrToUpperCamelCaseTokenStream;

pub use naming_common::DisplayToScreamingSnakeCaseStringified;
pub use naming_common::DisplayToScreamingSnakeCaseTokenStream;
pub use naming_common::DisplayToSnakeCaseStringified;
pub use naming_common::DisplayToSnakeCaseTokenStream;
pub use naming_common::DisplayToUpperCamelCaseStringified;
pub use naming_common::DisplayToUpperCamelCaseTokenStream;

pub use naming_common::ToTokensToScreamingSnakeCaseStringified;
pub use naming_common::ToTokensToScreamingSnakeCaseTokenStream;
pub use naming_common::ToTokensToSnakeCaseStringified;
pub use naming_common::ToTokensToSnakeCaseTokenStream;
pub use naming_common::ToTokensToUpperCamelCaseStringified;
pub use naming_common::ToTokensToUpperCamelCaseTokenStream;

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
    ["query", "part", "error", "named"],
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
    ["read", "inner"],
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
    ["generate", "postgresql", "json", "type", "to", "read", "from", "vec"],
    ["generate", "postgresql", "json", "type", "to", "read", "from", "vec", "error", "named"],
    ["generate", "postgresql", "json", "type", "to", "read", "error", "named"],
    ["option", "to", "read"],
    ["option", "to", "update"],
    ["id"],
    ["to", "create"],
    ["reader"],
    ["field", "to", "update"],
    ["json", "array", "element", "change"],
    ["not", "unique", "id"],
    ["std", "vec", "vec", "generic"],
    ["fields", "filter", "is", "empty"],
    ["not", "unique", "field", "filter"],
    ["pagination"],
    ["field", "vec"],
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
    ["option", "to", "update", "try", "generate", "postgresql", "json", "type", "error", "named"],
    ["create", "query", "part"],
    ["create", "query", "bind"],
    ["select", "query", "part"],
    ["jsonb", "set", "accumulator"],
    ["jsonb", "set", "target"],
    ["jsonb", "set", "path"],
    ["column", "name", "and", "maybe", "field", "getter"],
    ["column", "name", "and", "maybe", "field", "getter", "for", "error", "message"],
    ["field", "ident"],
    ["postgresql", "json", "type"],
    ["create", "query", "part", "error", "named"],
    ["create"],
    ["dotenv"],
    ["std", "env", "var", "error"],
    ["env", "var", "name"],
    ["try", "from", "std", "env", "var", "ok"],
    ["table", "name"],
    ["generate", "postgresql", "crud", "primary", "key"],
    ["default", "but", "option", "is", "always", "some", "and", "vec", "always", "contains", "one", "element"],
    ["all", "enum", "variants", "array", "default", "but", "option", "is", "always", "some", "and", "vec", "always", "contains", "one", "element"],
    ["all", "enum", "variants", "array", "default", "but", "std", "option", "option", "is", "always", "some", "and", "std", "vec", "vec", "always", "contains", "one", "element"],
    ["generate", "postgresql", "json", "type"],
    ["to", "std", "string", "string"],
    ["error", "occurence", "lib"],
    ["logical", "operator"],
    ["pub"],
    ["self", "create"],
    ["self", "to", "read"],
    ["self", "select"],
    ["self", "read"],
    ["postgresql", "crud", "base", "self", "traits"],
    ["postgresql", "base", "type"],
    ["postgresql", "base", "type", "self"],
    ["postgresql", "base", "type", "primary", "key"],
    ["postgresql", "base", "type", "primary", "key", "self"],
    ["postgresql", "base", "type", "std", "option", "option", "self"],
    ["postgresql", "type", "self", "traits"],
    ["postgresql", "type"],
    ["self", "column"],
    ["postgresql", "json", "type", "object", "self", "select"],
    ["postgresql", "json", "type", "std", "option", "option", "object", "self", "select"],
    ["postgresql", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "select"],
    ["postgresql", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "select"],
    ["true"],
    ["false"],
    ["column", "name", "and", "maybe", "field", "getter", "handle"],
    ["null"],
    ["update", "query", "part"],
    ["update", "query", "bind"],
    ["query", "part"],
    ["query", "bind"],
    ["crate"],
    ["between"],
    ["is", "need", "to", "add", "logical", "operator"],
    ["case", "sensitive", "regular", "expression"],
    ["case", "insensitive", "regular", "expression"],
    ["equal"],
    ["greater", "than"],
    ["hexadecimal", "notation", "equal"],
    ["length", "more", "than"],
    ["equal", "to", "encoded", "string", "representation"],
    ["value", "is", "contained", "within", "range"],
    ["contains", "another", "range"],
    ["is", "null"],
    ["try", "new", "error", "named"],
    ["start", "more", "or", "equal", "to", "end"],
    ["is", "empty"],
    ["length", "is", "negative"],
    ["start", "is", "equal", "to", "end"],
    ["start"],
    ["strictly", "to", "left", "of", "range"],
    ["strictly", "to", "right", "of", "range"],
    ["included", "lower", "bound"],
    ["excluded", "upper", "bound"],
    ["greater", "than", "lower", "bound"],
    ["overlap", "with", "range"],
    ["adjacent", "with", "range"],
    ["range", "length"],
    ["length", "is", "negative", "or", "zero"],
    ["before"],
    ["current", "date"],
    ["current", "time"],
    ["greater", "than", "current", "date"],
    ["greater", "than", "current", "time"],
    ["current", "timestamp"],
    ["greater", "than", "current", "timestamp"],
    ["position", "equal"],
    ["position", "is", "less", "or", "equal", "zero"],
    ["self", "where", "element"],
    ["where", "element"],
    ["self", "where"],
    ["position"],
    ["bit", "vec", "position", "equal"],
    ["position", "is", "less", "than", "zero"],
    ["position", "greater", "than"],
    ["position", "case", "sensitive", "regular", "expression"],
    ["position", "case", "insensitive", "regular", "expression"],
    ["contains", "all", "elements", "of", "array"],
    ["contained", "in", "array"],
    ["overlaps", "with", "array"],
    ["length", "equal"],
    ["all", "elements", "equal"],
    ["contains", "element", "greater", "than"],
    ["all", "elements", "greater", "than"],
    ["contains", "element", "case", "sensitive", "regular", "expression"],
    ["contains", "element", "case", "insensitive", "regular", "expression"],
    ["all", "elements", "case", "sensitive", "regular", "expression"],
    ["all", "elements", "case", "insensitive", "regular", "expression"],
    ["contains", "element"],
    ["equal", "second", "dimension"],
    ["from", "calendar", "date"],
    ["less", "than", "minimum", "postgresql", "value"],
    ["digits"],
    ["scale"],
    ["year"],
    ["month"],
    ["day"],
    ["months"],
    ["days"],
    ["microseconds"],
    ["date"],
    ["time"],
    ["std", "vec", "vec"],
    ["std", "vec", "vec", "std", "vec", "vec"],
    ["uuid", "uuid"],
    ["fixed", "length"],
    ["postgresql", "type", "where", "filter"],
    ["is", "primary", "key"],
    ["create", "table", "column", "query", "part"],
    ["table", "type", "declaration"],
    ["mut"],
    ["boolean"],
    ["object"],
    ["array"],
    ["number"],
    ["integer"],
    ["not", "null"],
    ["nullable"],
    ["vec", "of"],
    ["array", "of"],
    ["array", "length", "dimension", "one"],
    ["array", "length", "more", "than", "dimension", "one"],
    ["jsonb", "object"],
    ["with", "id"],
    ["uuid", "uuid", "as", "not", "null", "jsonb", "string"],
    ["jsonb"],
    ["regular", "expression"],
    ["position", "regular", "expression"],
    ["contains", "element", "regular", "expression"],
    ["all", "elements", "regular", "expression"],
    ["dimension", "one", "position", "equal"],
    ["dimension", "two", "position", "equal"]
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
        quote::quote! {HashMap}.to_tokens(tokens);
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
        quote::quote! {hashmap}.to_tokens(tokens);
    }
}

pub trait StdFmtDisplayPlusQuoteToTokens: std::fmt::Display + quote::ToTokens {}
impl<T> StdFmtDisplayPlusQuoteToTokens for T where T: std::fmt::Display + quote::ToTokens {}

pub trait SwaggerUrlPathSelfQuotesStringified {
    fn swagger_url_path_self_quotes_stringified(&self, table_name_stringified: &str) -> std::string::String;
}

impl<T> SwaggerUrlPathSelfQuotesStringified for T
where
    T: naming_common::AsRefStrToSnakeCaseStringified,
{
    fn swagger_url_path_self_quotes_stringified(&self, table_name_stringified: &str) -> std::string::String {
        generate_quotes::double_quotes_stringified(&format!("/{}/{}", table_name_stringified, self.case(),))
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
    T: naming_common::AsRefStrToSnakeCaseStringified,
{
    fn url_handle_self_snake_case_stringified(&self, table_name_stringified: &str) -> std::string::String {
        format!("\"{{}}/{}/{}\"", table_name_stringified, self.case())
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
