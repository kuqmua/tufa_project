pub const GITHUB_URL: &str = "https://github.com/kuqmua/tufa_project";
pub const SUPPORTS_ONLY_STRINGIFIED: &str = "supports only";
pub const SYN_FIELDS: &str = "syn::Fields";
pub const SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT: &str =
    "proc_macro_helpers::error_occurence::supported_container::SupportedContainer::";
pub const SUPPORTED_ENUM_VARIANT_STRINGIFIED: &str =
    "proc_macro_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant";
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
gen_naming_trait_impl::gen_naming_trait_impl_vec!(
    named,
    unnamed,
    error
    // ,
    // occurence,
    // string,
    // parameters,
    // payload,
    // element,
    // try,
    // from,
    // path,
    // key,
    // keys,
    // value,
    // vec,
    // HashMap,
    // reference,
    // with,
    // serialize,
    // deserialize,
    // request,
    // response,
    // variants,
    // options,
    // code,
    // config,
    // is,
    // none,
    // str,
    // uuid,
    // wrapper,
    // possible,
    // source,
    // display,
    // foreign,
    // type,
    // to,
    // into,
    // get,
    // column,
    // select,
    // order,
    // by,
    // not,
    // found,
    // desirable,
    // rollback,
    // limit,
    // offset,
    // in,
    // client,
    // server,
    // no,
    // fields,
    // commit,
    // begin,
    // acc,
    // query,
    // update,
    // as,
    // set,
    // insert,
    // values,
    // delete,
    // where,
    // and,
    // unnest,
    // configuration,
    // database,
    // io,
    // tls,
    // protocol,
    // row,
    // index,
    // out,
    // of,
    // bounds,
    // decode,
    // pool,
    // timed,
    // closed,
    // worker,
    // crashed,
    // migrate,
    // json,
    // data,
    // syntax,
    // missing,
    // content,
    // bytes,
    // rejection,
    // expected,
    // unexpected,
    // case,
    // status,
    // failed,
    // text,
    // reqwest,
    // headers,
    // result,
    // serde,
    // debug,
    // or,
    // asc,
    // desc,
    // unique,
    // many,
    // std,
    // option,
    // primary,
    // inner,
    // existing,
    // non,
    // current,
    // len,
    // sqlx,
    // returning,
    // app,
    // state,
    // permission,
    // tvfrr
);

pub const NAMED: &str = "named";
pub const UNNAMED: &str = "unnamed";
pub const ERROR: &str = "error";
pub const OCCURENCE: &str = "occurence";
pub const STRING: &str = "string";
pub const PARAMETERS: &str = "parameters";
pub const PAYLOAD: &str = "payload";
pub const ELEMENT: &str = "element";
pub const TRY: &str = "try";
pub const FROM: &str = "from";
pub const PATH: &str = "path";
pub const KEY: &str = "key";
pub const KEYS: &str = "keys";
pub const VALUE: &str = "value";
pub const VEC: &str = "vec";
pub const HASHMAP: &str = "HashMap";
pub const REFERENCE: &str = "reference";
pub const WITH: &str = "with";
pub const SERIALIZE: &str = "serialize";
pub const DESERIALIZE: &str = "deserialize";
pub const REQUEST: &str = "request";
pub const RESPONSE: &str = "response";
pub const VARIANTS: &str = "variants";
pub const OPTIONS: &str = "options";
pub const CODE: &str = "code";
pub const CONFIG: &str = "config";
pub const IS: &str = "is";
pub const NONE: &str = "none";
pub const STR: &str = "str";
pub const UUID: &str = "uuid";
pub const WRAPPER: &str = "wrapper";
pub const POSSIBLE: &str = "possible";
pub const SOURCE: &str = "source";
pub const DISPLAY: &str = "display";
pub const FOREIGN: &str = "foreign";
pub const TYPE: &str = "type";
pub const TO: &str = "to";
pub const INTO: &str = "into";
pub const GET: &str = "get";
pub const COLUMN: &str = "column";
pub const SELECT: &str = "select";
pub const ORDER: &str = "order";
pub const BY: &str = "by";
pub const NOT: &str = "not";
pub const FOUND: &str = "found";
pub const DESIRABLE: &str = "desirable";
pub const ROLLBACK: &str = "rollback";
pub const LIMIT: &str = "limit";
pub const OFFSET: &str = "offset";
pub const IN: &str = "in";
pub const CLIENT: &str = "client";
pub const SERVER: &str = "server";
pub const NO: &str = "no";
pub const FIELDS: &str = "fields";
pub const COMMIT: &str = "commit";
pub const BEGIN: &str = "begin";
pub const ACC: &str = "acc";
pub const QUERY: &str = "query";
pub const UPDATE: &str = "update";
pub const AS: &str = "as";
pub const SET: &str = "set";
pub const INSERT: &str = "insert";
pub const VALUES: &str = "values";
pub const DELETE: &str = "delete";
pub const WHERE: &str = "where";
pub const AND: &str = "and";
pub const UNNEST: &str = "unnest";
pub const CONFIGURATION: &str = "configuration";
pub const DATABASE: &str = "database";
pub const IO: &str = "io";
pub const TLS: &str = "tls";
pub const PROTOCOL: &str = "protocol";
pub const ROW: &str = "row";
pub const INDEX: &str = "index";
pub const OUT: &str = "out";
pub const OF: &str = "of";
pub const BOUNDS: &str = "bounds";
pub const DECODE: &str = "decode";
pub const POOL: &str = "pool";
pub const TIMED: &str = "timed";
pub const CLOSED: &str = "closed";
pub const WORKER: &str = "worker";
pub const CRASHED: &str = "crashed";
pub const MIGRATE: &str = "migrate";
pub const JSON: &str = "json";
pub const DATA: &str = "data";
pub const SYNTAX: &str = "syntax";
pub const MISSING: &str = "missing";
pub const CONTENT: &str = "content";
pub const BYTES: &str = "bytes";
pub const REJECTION: &str = "rejection";
pub const EXPECTED: &str = "expected";
pub const UNEXPECTED: &str = "unexpected";
pub const CASE: &str = "case";
pub const STATUS: &str = "status";
pub const FAILED: &str = "failed";
pub const TEXT: &str = "text";
pub const REQWEST: &str = "reqwest";
pub const HEADERS: &str = "headers";
pub const RESULT: &str = "result";
pub const SERDE: &str = "serde";
pub const DEBUG: &str = "debug";
pub const OR: &str = "or";
pub const ASC: &str = "asc";
pub const DESC: &str = "desc";
pub const UNIQUE: &str = "unique";
pub const MANY: &str = "many";
pub const STD: &str = "std";
pub const OPTION: &str = "option";
pub const PRIMARY: &str = "primary";
pub const INNER: &str = "inner";
pub const EXISTING: &str = "existing";
pub const NON: &str = "non";
pub const CURRENT: &str = "current";
pub const LEN: &str = "len";
pub const SQLX: &str = "sqlx";
pub const RETURNING: &str = "returning";
pub const APP: &str = "app";
pub const STATE: &str = "state";
pub const PERMISSION: &str = "permission";
pub const TVFRR: &str = "tvfrr";



//
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Named;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Unnamed;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Occurence;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct String;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Parameters;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Paylaod;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Element;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Try;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct From;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Path;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Key;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Keys;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Value;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Vec;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Hashmap;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Reference;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct With;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Serialize;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Deserialize;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Request;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Response;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct Variants;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
// #[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
// pub struct ;
//
#[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
pub struct Read;
#[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
pub struct Logic;
#[derive(gen_naming_trait_impl::GenNamingTraitImpl)]
pub struct Extraction;