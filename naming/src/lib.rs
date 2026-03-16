pub mod prm;
use gen_quotes::dq_str;
pub use naming_cmn::{
    AsRefStrToScStr, AsRefStrToScTs, AsRefStrToUccStr, AsRefStrToUccTs, AsRefStrToUpperScStr,
    AsRefStrToUpperScTs, DisplayToScStr, DisplayToScTs, DisplayToUccStr, DisplayToUccTs,
    DisplayToUpperScStr, DisplayToUpperScTs, ToTokensToScStr, ToTokensToScTs, ToTokensToUccStr,
    ToTokensToUccTs, ToTokensToUpperScStr, ToTokensToUpperScTs,
};
pub use naming_macros::{
    AsRefStrEnumWithUnitFieldsToScStr, AsRefStrEnumWithUnitFieldsToUccStr,
    AsRefStrEnumWithUnitFieldsToUpperScStr,
};
use optml::Optml;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use std::fmt::{Display, Formatter, Result as FmtResult};
pub const GITHUB_URL: &str = "https://github.com/kuqmua/tufa_project";
naming_macros::gen_ucc_and_sc_str_and_ts!([
    ["pk"],
    ["serde"],
    ["with", "serde"],
    ["loc"],
    ["failed", "to", "get", "res", "text"],
    ["de", "res"],
    ["status", "code"],
    ["res", "text"],
    ["order", "by"],
    ["not", "unq", "pk"],
    ["into", "serde", "version"],
    ["app", "state"],
    ["qp", "er"],
    ["serde", "json", "to", "string"],
    ["endpoint", "loc"],
    ["query", "string"],
    ["binded", "query"],
    ["not", "unq", "field"],
    ["body", "bytes"],
    ["check", "body", "size"],
    ["expected", "res"],
    ["pg", "crud"],
    ["cm", "er", "vrts"],
    ["co", "er", "vrts"],
    ["rm", "er", "vrts"],
    ["ro", "er", "vrts"],
    ["um", "er", "vrts"],
    ["uo", "er", "vrts"],
    ["dm", "er", "vrts"],
    ["dlo", "er", "vrts"],
    ["cmn", "er", "vrts"],
    ["cm", "logic"],
    ["co", "logic"],
    ["rm", "logic"],
    ["ro", "logic"],
    ["um", "logic"],
    ["uo", "logic"],
    ["dm", "logic"],
    ["dlo", "logic"],
    ["cmn", "logic"],
    ["row", "and", "rollback"],
    ["serde", "json"],
    ["pool", "connection"],
    ["in"],
    ["as"],
    ["wh"],
    ["er"],
    ["string"],
    ["prms"],
    ["payload"],
    ["el"],
    ["value"],
    ["req"],
    ["res"],
    ["config"],
    ["is"],
    ["to"],
    ["col"],
    ["sel"],
    ["order"],
    ["by"],
    ["not"],
    ["desirable"],
    ["rollback"],
    ["fields"],
    ["commit"],
    ["begin"],
    ["query"],
    ["upd"],
    ["del"],
    ["and"],
    ["row"],
    ["pool"],
    ["reqwest"],
    ["headers"],
    ["or"],
    ["asc"],
    ["desc"],
    ["opt"],
    ["rd"],
    ["rd", "inn"],
    ["body"],
    ["pg"],
    ["incr"],
    ["url"],
    ["future"],
    ["end"],
    ["rows"],
    ["executor"],
    ["prefix"],
    ["id"],
    ["pgn"],
    ["std", "opt", "opt", "obj", "acc"],
    ["not", "unq", "id", "in", "json", "del", "arr"],
    [
        "not", "unq", "id", "in", "json", "upd", "and", "del", "arrs"
    ],
    ["gen", "jsonb", "set", "target"],
    ["all", "fields", "are", "none"],
    ["self"],
    ["cr", "qp"],
    ["cr", "qb"],
    ["sel", "qp"],
    ["jsonb", "set", "accumulator"],
    ["jsonb", "set", "target"],
    ["jsonb", "set", "path"],
    ["col", "field"],
    ["col", "field", "for", "er", "msg"],
    ["fi"],
    ["pg", "json"],
    ["cr"],
    ["dotenv"],
    ["std", "env", "var", "er"],
    ["env", "var", "name"],
    ["try", "from", "std", "env", "var", "ok"],
    ["tbl", "name"],
    ["dflt", "some", "one", "el"],
    ["all", "vrts", "dflt", "some", "one", "el"],
    ["loc", "lib"],
    ["pub"],
    ["self", "cr"],
    ["self", "sel"],
    ["self", "rd"],
    ["pg", "type"],
    ["true"],
    ["false"],
    ["upd", "qp"],
    ["upd", "qb"],
    ["qp"],
    ["qb"],
    ["btwn"],
    ["add", "oprtr"],
    ["eq"],
    ["greater", "than"],
    ["eq", "to", "encoded", "string", "representation"],
    ["find", "ranges", "within", "given", "range"],
    [
        "find", "ranges", "that", "fully", "contain", "the", "given", "range"
    ],
    ["start"],
    ["strictly", "to", "left", "of", "range"],
    ["strictly", "to", "right", "of", "range"],
    ["included", "lower", "bound"],
    ["excluded", "upper", "bound"],
    ["greater", "than", "included", "lower", "bound"],
    ["greater", "than", "excluded", "upper", "bound"],
    ["overlap", "with", "range"],
    ["adjacent", "with", "range"],
    ["range", "len"],
    ["before"],
    ["crnt", "date"],
    ["crnt", "time"],
    ["greater", "than", "crnt", "date"],
    ["greater", "than", "crnt", "time"],
    ["crnt", "timestamp"],
    ["greater", "than", "crnt", "timestamp"],
    ["self", "wh"],
    ["dim", "one", "contains", "all", "els", "of", "arr"],
    ["dim", "two", "contains", "all", "els", "of", "arr"],
    ["dim", "three", "contains", "all", "els", "of", "arr"],
    ["dim", "four", "contains", "all", "els", "of", "arr"],
    ["dim", "one", "overlaps", "with", "arr"],
    ["dim", "two", "overlaps", "with", "arr"],
    ["dim", "three", "overlaps", "with", "arr"],
    ["dim", "four", "overlaps", "with", "arr"],
    ["dim", "one", "len", "eq"],
    ["dim", "two", "len", "eq"],
    ["dim", "three", "len", "eq"],
    ["dim", "four", "len", "eq"],
    ["dim", "one", "len", "greater", "than"],
    ["dim", "two", "len", "greater", "than"],
    ["dim", "three", "len", "greater", "than"],
    ["dim", "four", "len", "greater", "than"],
    ["dim", "one", "all", "els", "eq"],
    ["dim", "two", "all", "els", "eq"],
    ["dim", "three", "all", "els", "eq"],
    ["dim", "four", "all", "els", "eq"],
    ["dim", "one", "contains", "el", "greater", "than"],
    ["dim", "two", "contains", "el", "greater", "than"],
    ["dim", "three", "contains", "el", "greater", "than"],
    ["dim", "four", "contains", "el", "greater", "than"],
    ["dim", "one", "all", "els", "greater", "than"],
    ["dim", "two", "all", "els", "greater", "than"],
    ["dim", "three", "all", "els", "greater", "than"],
    ["dim", "four", "all", "els", "greater", "than"],
    ["months"],
    ["days"],
    ["microseconds"],
    ["date"],
    ["time"],
    ["pg", "type", "wh", "flt"],
    ["is", "pk"],
    ["cr", "tbl", "col", "qp"],
    ["tt"],
    ["mut"],
    ["boolean"],
    ["nbr"],
    ["vec", "of"],
    ["arr", "of"],
    ["jsonb", "obj"],
    ["with", "id"],
    ["uuid", "uuid", "as", "nn", "jsonb", "string"],
    ["rgx"],
    ["dim", "one", "rgx"],
    ["dim", "two", "rgx"],
    ["dim", "three", "rgx"],
    ["dim", "four", "rgx"],
    ["dim", "one", "contains", "el", "rgx"],
    ["dim", "two", "contains", "el", "rgx"],
    ["dim", "three", "contains", "el", "rgx"],
    ["dim", "four", "contains", "el", "rgx"],
    ["dim", "one", "all", "els", "rgx"],
    ["dim", "two", "all", "els", "rgx"],
    ["dim", "three", "all", "els", "rgx"],
    ["dim", "four", "all", "els", "rgx"],
    ["dim", "one", "eq"],
    ["dim", "two", "eq"],
    ["dim", "three", "eq"],
    ["dim", "four", "eq"],
    ["dim", "one", "greater", "than"],
    ["dim", "two", "greater", "than"],
    ["dim", "three", "greater", "than"],
    ["dim", "four", "greater", "than"],
    ["dim", "one", "in"],
    ["dim", "two", "in"],
    ["dim", "three", "in"],
    ["dim", "four", "in"],
    ["dim", "one", "btwn"],
    ["dim", "two", "btwn"],
    ["dim", "three", "btwn"],
    ["dim", "four", "btwn"],
    ["dim", "one", "before"],
    ["dim", "one", "crnt", "date"],
    ["dim", "one", "greater", "than", "crnt", "date"],
    ["dim", "one", "crnt", "timestamp"],
    ["dim", "one", "greater", "than", "crnt", "timestamp"],
    ["dim", "one", "crnt", "time"],
    ["dim", "one", "greater", "than", "crnt", "time"],
    [
        "dim",
        "one",
        "eq",
        "to",
        "encoded",
        "string",
        "representation"
    ],
    ["dim", "one", "find", "ranges", "within", "given", "range"],
    [
        "dim", "one", "find", "ranges", "that", "fully", "contain", "the", "given", "range"
    ],
    ["dim", "one", "strictly", "to", "left", "of", "range"],
    ["dim", "one", "strictly", "to", "right", "of", "range"],
    ["dim", "one", "included", "lower", "bound"],
    ["dim", "one", "excluded", "upper", "bound"],
    [
        "dim", "one", "greater", "than", "included", "lower", "bound"
    ],
    [
        "dim", "one", "greater", "than", "excluded", "upper", "bound"
    ],
    ["dim", "one", "overlap", "with", "range"],
    ["dim", "one", "adjacent", "with", "range"],
    ["dim", "one", "range", "len"],
    ["dims"],
    ["dims", "ies"],
    ["len", "eq"],
    ["len", "greater", "than"],
    ["contains", "all", "els", "of", "arr"],
    ["overlaps", "with", "arr"],
    ["contains", "el", "rgx"],
    ["all", "els", "rgx"],
    ["all", "els", "eq"],
    ["contains", "el", "greater", "than"],
    ["all", "els", "greater", "than"],
    ["cr", "extension", "if", "not", "exists", "pg", "jsonschema"],
    ["cr", "extension", "if", "not", "exists", "uuid", "ossp"],
    ["prep", "pg"],
    ["prep", "pg", "tbl"],
    ["header", "content", "type", "app", "json", "not", "found"],
    ["wh", "many"],
    ["no", "fields", "provided"],
    ["extra", "prms"],
    ["gen", "sel", "qp"],
    ["upd", "qp", "pk"],
    ["gen", "col", "queals", "v", "comma", "uo", "qp"],
    ["pk", "qp"],
    ["cols"],
    ["gen", "when", "col", "id", "then", "v", "um", "qp"],
    ["contains", "null", "byte"],
    ["pg", "type", "test", "cases"],
    ["included", "start", "greater", "than", "included", "end"],
    ["included", "start", "greater", "than", "excluded", "end"],
    ["excluded", "start", "greater", "than", "included", "end"],
    ["excluded", "start", "greater", "than", "excluded", "end"],
    ["included", "end", "cannot", "be", "max"],
    ["earlier", "date", "not", "supported"],
    ["earliest", "supported", "date"],
    [
        "invalid",
        "hour",
        "or",
        "minute",
        "or",
        "second",
        "or",
        "microsecond"
    ],
    ["hour"],
    ["min"],
    ["sec"],
    ["micro"],
    ["minute"],
    ["second"],
    ["microsecond"],
    ["nanosecond", "precision", "is", "not", "supported"],
    ["date", "naive"],
    ["nanosecond"],
    ["included"],
    ["excluded"],
    ["unbounded"],
    ["normalize"],
    ["new"],
    ["try", "new"],
    ["pg", "json", "test", "cases"],
    ["pg", "pool"],
    ["pg", "pool", "for", "tokio", "spawn", "sync", "move"],
    ["ident", "cr", "dflt"],
    ["sel", "pk"],
    ["sel", "qp", "pg", "type"],
    ["rd", "ids"],
    ["sel", "only", "ids", "qp"],
    ["sel", "only", "updd", "ids", "qp"],
    ["cr", "upd", "del", "are", "empty"],
    ["upd", "to", "rd", "ids"],
    ["self", "rd", "ids", "h"],
    ["cmn", "rd", "ids", "from", "co"],
    ["gen", "pg", "tbl", "pk"],
    ["try", "bind"],
    ["sel", "only", "updd", "ids", "qb"],
    ["cr", "for", "query"],
    [
        "rd", "ids", "to", "opt", "v", "rd", "dflt", "some", "one", "el"
    ],
    ["opt", "upd"],
    ["sel", "only", "crd", "ids", "qp"],
    ["sel", "only", "crd", "ids", "qb"],
    ["upd", "for", "query"],
    ["upd", "for", "query", "vec"],
    ["rd", "ids", "and", "cr", "into", "opt", "v", "rd"],
    ["dflt", "some", "one", "el", "max", "page", "size"],
    [
        "all", "vrts", "dflt", "some", "one", "el", "max", "page", "size"
    ],
    ["ids", "are", "not", "unq"],
    ["pg", "type", "pk"],
    ["pg", "type", "not", "pk"],
    ["rd", "ids", "and", "cr", "into", "wh", "eq"],
    ["rd", "ids", "and", "cr", "into", "rd"],
    ["rd", "ids", "and", "cr", "into", "tt"],
    [
        "rd", "inn", "into", "rd", "with", "new", "or", "try", "new", "unwraped"
    ],
    [
        "rd", "inn", "into", "upd", "with", "new", "or", "try", "new", "unwraped"
    ],
    ["rd", "ids", "into", "opt", "v", "rd", "inn"],
    ["previous", "rd", "and", "opt", "upd", "into", "rd"],
    [
        "rd", "ids", "and", "cr", "into", "vec", "wh", "eq", "using", "fields"
    ],
    ["eq", "oprtr"],
    ["pg", "type", "eq", "oprtr"],
    ["rd", "ids", "into", "tt"],
    ["rd", "ids", "into", "rd"],
    ["rd", "ids", "into", "upd"],
    ["rd", "into", "tt"],
    ["opt", "vec", "cr"],
    ["rd", "ids", "to2", "dims", "vec", "rd", "inn"],
    [
        "rd", "ids", "and", "cr", "into", "opt", "vec", "wh", "eq", "to", "json", "field"
    ],
    [
        "rd", "ids", "and", "cr", "into", "vec", "wh", "eq", "to", "json", "field"
    ],
    [
        "cr", "into", "pg", "type", "opt", "vec", "wh", "dim", "one", "eq"
    ],
    [
        "rd", "ids", "and", "tt", "into", "pg", "type", "opt", "wh", "greater", "than"
    ],
    ["pg", "type", "opt", "vec", "wh", "greater", "than", "test"],
    [
        "rd", "ids", "and", "cr", "into", "pg", "json", "opt", "vec", "wh", "dim", "one", "eq"
    ],
    [
        "rd", "ids", "and", "cr", "into", "pg", "json", "opt", "vec", "wh", "dim", "two", "eq"
    ],
    [
        "rd", "ids", "and", "cr", "into", "pg", "json", "opt", "vec", "wh", "dim", "three", "eq"
    ],
    [
        "rd", "ids", "and", "cr", "into", "pg", "json", "opt", "vec", "wh", "dim", "four", "eq"
    ],
    ["prep", "extensions"],
    ["tbl"],
    ["routes"],
    ["routes", "h"],
    [
        "cr", "into", "pg", "json", "opt", "vec", "wh", "len", "greater", "than"
    ],
    ["cr", "into", "pg", "json", "opt", "vec", "wh", "len", "eq"],
    ["from", "h"],
    [
        "rd", "ids", "and", "cr", "into", "pg", "json", "opt", "vec", "wh", "greater", "than"
    ],
    [
        "rd", "ids", "and", "cr", "into", "pg", "json", "opt", "vec", "wh", "btwn"
    ],
    [
        "rd", "ids", "and", "cr", "into", "pg", "json", "opt", "vec", "wh", "in"
    ],
    [
        "rd", "ids", "and", "cr", "into", "pg", "json", "opt", "vec", "wh", "rgx"
    ],
    [
        "rd", "ids", "and", "cr", "into", "pg", "json", "opt", "vec", "wh", "contains", "el",
        "greater", "than"
    ],
    [
        "rd", "ids", "and", "cr", "into", "pg", "json", "opt", "vec", "wh", "contains", "el", "rgx"
    ],
    ["executor", "acquire"],
    ["gen", "pg", "json", "mod"],
    ["gen", "pg", "types", "mod"],
    ["to", "err", "string"],
    ["body", "size", "er"],
    ["max"],
    ["near", "zero"],
    ["negative", "less", "typical"],
    ["negative", "more", "typical"],
    ["positive", "less", "typical"],
    ["positive", "more", "typical"],
    ["v"],
    ["not", "uuid"]
]);
#[derive(Debug, Clone, Copy, Optml)]
pub struct HashMap;
#[derive(Debug, Clone, Copy, Optml)]
pub struct HashMapUcc;
impl Display for HashMapUcc {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "HashMap")
    }
}
impl ToTokens for HashMapUcc {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {HashMap}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct HashMapSc;
impl Display for HashMapSc {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "hashmap")
    }
}
impl ToTokens for HashMapSc {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {hashmap}.to_tokens(tokens);
    }
}
pub trait DisplayPlusToTokens: Display + ToTokens {}
impl<T> DisplayPlusToTokens for T where T: Display + ToTokens {}
pub trait SwaggerUrlPathSelfQuotesStr {
    fn swagger_url_path_self_quotes_str(&self, v: &str) -> String;
}
impl<T> SwaggerUrlPathSelfQuotesStr for T
where
    T: AsRefStrToScStr,
{
    fn swagger_url_path_self_quotes_str(&self, v: &str) -> String {
        dq_str(&format!("/{}/{}", v, self.case(),))
    }
}
pub trait SwaggerUrlPathSelfQuotesTokenStream {
    fn swagger_url_path_self_quotes_ts(&self, v: &str) -> Ts2;
}
impl<T> SwaggerUrlPathSelfQuotesTokenStream for T
where
    T: SwaggerUrlPathSelfQuotesStr,
{
    fn swagger_url_path_self_quotes_ts(&self, v: &str) -> Ts2 {
        self.swagger_url_path_self_quotes_str(v)
            .parse::<Ts2>()
            .expect("f292686b")
    }
}
