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
    ["generate", "postgresql", "query", "part", "to", "read", "from", "self", "vec"],
    ["generate", "postgresql", "query", "part", "to", "read", "from", "self", "vec", "error", "named"],
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
    ["field", "vec"]
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
    ["self", "options", "to", "read"],
    ["generic", "self"],
    ["std", "option", "option", "generic", "self"],
    ["generic", "with", "id", "self"],
    ["std", "vec", "vec", "generic", "with", "id", "self"],
    ["std", "option", "option", "std", "vec", "vec", "generic", "with", "id", "self"],
    ["self", "field", "reader"],
    ["self", "field", "to", "read"],
    ["self", "with", "id", "field", "to", "read"],
    ["generic", "self", "field", "reader"],
    ["generic", "with", "id", "self", "field", "reader"],
    ["std", "option", "option", "generic", "self", "field", "reader"],
    ["std", "vec", "vec", "generic", "with", "id", "self", "field", "reader"],
    ["std", "option", "option", "std", "vec", "vec", "generic", "with", "id", "self", "field", "reader"],
    ["self", "try", "new", "error", "named"],
    ["generic", "self", "options", "to", "read"],
    ["std", "option", "option", "generic", "self", "options", "to", "read"],
    ["std", "vec", "vec", "generic", "with", "id", "self", "options", "to", "read"],
    ["std","option", "option","std", "vec", "vec", "generic", "with", "id", "self", "options", "to", "read"],
    ["generic", "self", "to", "create"],
    ["std", "option", "option", "generic", "self", "to", "create"],
    ["std", "vec", "vec", "generic", "with", "id", "self", "to", "create"],
    ["std", "option", "option", "std", "vec", "vec", "generic", "with", "id", "self", "to", "create"],
    ["generic", "with", "id", "self", "options", "to", "read"],
    ["generic", "with", "id", "self", "to", "create"],
    ["generic", "self", "reader"],
    ["generic", "with", "id", "self", "reader"],
    ["std", "option", "option", "generic", "self", "reader"],
    ["std", "vec", "vec", "generic", "with", "id", "self", "reader"],
    ["std", "option", "option", "std", "vec", "vec", "generic", "with", "id", "self", "reader"],
    ["self", "reader"],
    ["std", "option", "option", "generic", "self", "to", "create", "origin"],
    ["std", "vec", "vec", "generic", "with", "id", "self", "to", "create", "origin"],
    ["std", "option", "option", "std", "vec", "vec", "generic", "with", "id", "self", "to", "create", "origin"],
    ["std", "option", "option", "generic", "self", "options", "to", "read", "origin"],
    ["std", "vec", "vec", "generic", "with", "id", "self", "options", "to", "read", "origin"],
    ["std", "option", "option", "std", "vec", "vec", "generic", "with", "id", "self", "options", "to", "read", "origin"],
    ["self", "option", "to", "update"],
    ["generic", "self", "option", "to", "update"]
]);

pub trait SwaggerUrlPathSelfQuotesStringified {
    fn swagger_url_path_self_quotes_stringified(&self, table_name_stringified: &str) -> std::string::String;
}

impl<T> SwaggerUrlPathSelfQuotesStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn swagger_url_path_self_quotes_stringified(&self, table_name_stringified: &str) -> std::string::String {
        proc_macro_common::generate_quotes::double_quotes_stringified(&format!("/{}/{}", table_name_stringified, self.to_snake_case_stringified(),))
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
        value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait UrlHandleSelfSnakeCaseStringified {
    fn url_handle_self_snake_case_stringified(&self, table_name_stringified: &str) -> std::string::String;
}

impl<T> UrlHandleSelfSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn url_handle_self_snake_case_stringified(&self, table_name_stringified: &str) -> std::string::String {
        format!("\"{{}}/{}/{}\"", table_name_stringified, self.to_snake_case_stringified())
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
        value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}
// fn generate_url_handle_token_stream(
//     table_name_stringified: &str,
//     operation_name_snake_case_stringified: &str,
//     proc_macro_name_upper_camel_case_ident_stringified: &str,
// ) -> proc_macro2::TokenStream {
//     let url_handle_stringified = format!("\"{{}}/{table_name_stringified}/{operation_name_snake_case_stringified}\"");
//     url_handle_stringified.parse::<proc_macro2::TokenStream>()
//     .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {url_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
// }