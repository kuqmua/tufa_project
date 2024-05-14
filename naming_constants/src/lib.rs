pub const GITHUB_URL: &str = "https://github.com/kuqmua/tufa_project";
pub const SUPPORTS_ONLY_STRINGIFIED: &str = "supports only";
pub const SYN_FIELDS: &str = "syn::Fields";
pub const SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED: &str = "syn::GenericArgument::Type";
pub const IS_NONE_STRINGIFIED: &str = "is None";
pub const STD_STRINGIFIED: &str = "std";
pub const SQLX_TYPES_UUID_STRINGIFIED: &str = "sqlx::types::Uuid";
pub const FIELD_IDENT_IS_NONE: &str = "field.ident is None";

pub trait Naming {
    fn upper_camel_case_stringified() -> &'static str;
    fn upper_camel_case_token_stream() -> proc_macro2::TokenStream;
    fn snake_case_stringified() -> &'static str;
    fn snake_case_token_stream() -> proc_macro2::TokenStream;
}
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
    "add"
);

#[derive(Debug, Clone, Copy)]
pub struct HashMap;
impl Naming for HashMap {
    fn upper_camel_case_stringified() -> &'static str {
        "HashMap"
    }
    fn upper_camel_case_token_stream() -> proc_macro2::TokenStream {
        quote::quote! { HashMap }
    }
    fn snake_case_stringified() -> &'static str {
        "hashmap"
    }
    fn snake_case_token_stream() -> proc_macro2::TokenStream {
        quote::quote! { hashmap }
    }
}