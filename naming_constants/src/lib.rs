pub const GITHUB_URL: &str = "https://github.com/kuqmua/tufa_project";
pub const SUPPORTS_ONLY_STRINGIFIED: &str = "supports only";
pub const SYN_FIELDS: &str = "syn::Fields";
pub const SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED: &str = "syn::GenericArgument::Type";
pub const IS_NONE_STRINGIFIED: &str = "is None";
pub const STD_STRINGIFIED: &str = "std";
pub const SQLX_TYPES_UUID_STRINGIFIED: &str = "sqlx::types::Uuid";
pub const FIELD_IDENT_IS_NONE: &str = "field.ident is None";
pub const SYN_TYPE_PATH: &str = "syn::Type::Path";

gen_naming_trait_impl_vec::gen_naming_trait_impl_vec!(
    "type",
    "in",
    "as",
    "where",
    "named",
    "unnamed",
    "error",
    "occurence",
    "string",
    "parameters",
    "payload",
    "element",
    "try",
    "from",
    "path",
    "key",
    "keys",
    "value",
    "vec",
    "reference",
    "with",
    "serialize",
    "deserialize",
    "request",
    "response",
    "variants",
    "options",
    "code",
    "config",
    "is",
    "none",
    "str",
    "uuid",
    "wrapper",
    "possible",
    "source",
    "display",
    "foreign",
    "to",
    "into",
    "get",
    "column",
    "select",
    "order",
    "by",
    "not",
    "found",
    "desirable",
    "rollback",
    "limit",
    "offset",
    "client",
    "server",
    "no",
    "fields",
    "commit",
    "begin",
    "acc",
    "query",
    "update",
    "set",
    "insert",
    "values",
    "delete",
    "and",
    "unnest",
    "configuration",
    "database",
    "io",
    "tls",
    "protocol",
    "row",
    "index",
    "out",
    "of",
    "bounds",
    "decode",
    "pool",
    "timed",
    "closed",
    "worker",
    "crashed",
    "migrate",
    "json",
    "data",
    "syntax",
    "missing",
    "content",
    "bytes",
    "rejection",
    "expected",
    "unexpected",
    "case",
    "status",
    "failed",
    "text",
    "reqwest",
    "headers",
    "result",
    "serde",
    "debug",
    "or",
    "asc",
    "desc",
    "unique",
    "many",
    "std",
    "option",
    "primary",
    "inner",
    "existing",
    "non",
    "current",
    "len",
    "sqlx",
    "returning",
    "app",
    "state",
    "permission",
    "tvfrr",
    "read",
    "logic",
    "extraction",
    "generated",
    "route",
    "body",
    "env",
    "collections",
    "postgresql",
    "bind",
    "version",
    "checked",
    "add",
    "upper",
    "snake",
    "camel",
    "stringified",
    "token",
    "stream",
    "done",
    "but",
    "operation",
    "generate",
    "increments",
    "additional",
    "http",
    "codes",
    "extract",
    "location",
    "pg",
    "connection",
    "binded",
    "axum",
    "check",
    "size",
    "handle",
    "url",
    "future"
);

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
        quote::quote!{HashMap}.to_tokens(tokens)
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
        quote::quote!{hashmap}.to_tokens(tokens)
    }
}

///////////
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebug;
impl quote::ToTokens for DeriveDebug {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote!{#[derive(Debug)]}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxAcquire;
impl quote::ToTokens for SqlxAcquire {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {sqlx::Acquire}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AxumExtractRejectionJsonRejection;
impl quote::ToTokens for AxumExtractRejectionJsonRejection {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {axum::extract::rejection::JsonRejection}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AxumResponseIntoResponse;
impl quote::ToTokens for AxumResponseIntoResponse {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {axum::response::IntoResponse}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ReqwestError;
impl quote::ToTokens for ReqwestError {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {reqwest::Error}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ReqwestHeaderHeaderMap;
impl quote::ToTokens for ReqwestHeaderHeaderMap {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {reqwest::header::HeaderMap}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HttpStatusCode;
impl quote::ToTokens for HttpStatusCode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {http::StatusCode}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxRow;
impl quote::ToTokens for SqlxRow {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {sqlx::Row}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SerdeSerialize;
impl quote::ToTokens for SerdeSerialize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {serde::Serialize}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SerdeDeserialize;
impl quote::ToTokens for SerdeDeserialize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {serde::Deserialize}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UtoipaToSchema;
impl quote::ToTokens for UtoipaToSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {utoipa::ToSchema}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ErrorOccurenceLibErrorOccurence;
impl quote::ToTokens for ErrorOccurenceLibErrorOccurence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {error_occurence_lib::ErrorOccurence}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ThiserrorError;
impl quote::ToTokens for ThiserrorError {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {thiserror::Error}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RefStdPrimitiveStr;
impl quote::ToTokens for RefStdPrimitiveStr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {&std::primitive::str}.to_tokens(tokens)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdStringString;
impl quote::ToTokens for StdStringString {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::string::String}.to_tokens(tokens)
    }
}