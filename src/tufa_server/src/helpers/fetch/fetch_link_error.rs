use tufa_common::where_was::WhereWas;

#[derive(Debug)]
pub enum FetchLinkErrorEnum {
    ReqwestBlockingGet {
        source: reqwest::Error,
        where_was: WhereWas,
    },
    StatusCode {
        source: reqwest::StatusCode,
        where_was: WhereWas,
    },
    ResponseText {
        source: reqwest::Error,
        where_was: WhereWas,
    },
}
