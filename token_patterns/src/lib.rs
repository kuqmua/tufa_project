use optml::Optml;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
#[derive(Debug, Clone, Copy, Optml)]
pub struct SqlxAcquire;
impl ToTokens for SqlxAcquire {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::Acquire}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct AxumExtractRejectionJsonRejection;
impl ToTokens for AxumExtractRejectionJsonRejection {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {axum::extract::rejection::JsonRejection}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct AxumResIntoRes;
impl ToTokens for AxumResIntoRes {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {axum::response::IntoResponse}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct ReqwestEr;
impl ToTokens for ReqwestEr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {reqwest::Error}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct ReqwestHeaderHeaderMap;
impl ToTokens for ReqwestHeaderHeaderMap {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {reqwest::header::HeaderMap}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct HttpStatusCode;
impl ToTokens for HttpStatusCode {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {http::StatusCode}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct SqlxRow;
impl ToTokens for SqlxRow {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::Row}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct SerdeSerialize;
impl ToTokens for SerdeSerialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {serde::Serialize}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct SerdeDeserialize;
impl ToTokens for SerdeDeserialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {serde::Deserialize}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct UtoipaToSchema;
impl ToTokens for UtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {utoipa::ToSchema}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct SchemarsJsonSchema;
impl ToTokens for SchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {schemars::JsonSchema}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct LocLibLoc;
impl ToTokens for LocLibLoc {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {loc_lib::Location}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct ThiserrorError;
impl ToTokens for ThiserrorError {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {thiserror::Error}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct Char;
impl ToTokens for Char {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {char}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct RefStr;
impl ToTokens for RefStr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {&str}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct StringTs;
impl ToTokens for StringTs {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {String}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct DeriveDebug;
impl ToTokens for DeriveDebug {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, Optml)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct DeriveDebugThiserrorLoc;
impl ToTokens for DeriveDebugThiserrorLoc {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, thiserror::Error, loc_lib::Location, Optml)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct DeriveDebugUtoipaToSchema;
impl ToTokens for DeriveDebugUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, utoipa::ToSchema, Optml)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct DeriveDebugSerdeSerializeSerdeDeserialize;
impl ToTokens for DeriveDebugSerdeSerializeSerdeDeserialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, serde::Serialize, serde::Deserialize, Optml)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
impl ToTokens for DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Optml)]}
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct DeriveDebugCloneCopy;
impl ToTokens for DeriveDebugCloneCopy {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, Clone, Copy, Optml)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct StrSqlxColIndex;
impl ToTokens for StrSqlxColIndex {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {&'lt str: sqlx::ColumnIndex<R>,}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct SqlxDecodeDecodeDatabase;
impl ToTokens for SqlxDecodeDecodeDatabase {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::decode::Decode<'lt, R::Database>}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct SqlxTypesTypeDatabase;
impl ToTokens for SqlxTypesTypeDatabase {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::Type<R::Database>}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct LocLibLocLoc;
impl ToTokens for LocLibLocLoc {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {loc_lib::loc::Loc}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct LocScDoubleDotSpaceLocLibLocLoc;
impl ToTokens for LocScDoubleDotSpaceLocLibLocLoc {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {loc: loc_lib::loc::Loc}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct CoreDefault;
impl ToTokens for CoreDefault {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {::core::default::Default::default()}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct SqlxTypesTimeTimeMidnight;
impl ToTokens for SqlxTypesTimeTimeMidnight {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::time::Time::MIDNIGHT}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct SqlxTypesTimeOffsetDateTimeUnixEpoch;
impl ToTokens for SqlxTypesTimeOffsetDateTimeUnixEpoch {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::time::OffsetDateTime::UNIX_EPOCH}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct Er0;
impl ToTokens for Er0 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_0}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct Er1;
impl ToTokens for Er1 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_1}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct Er2;
impl ToTokens for Er2 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_2}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct Er3;
impl ToTokens for Er3 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_3}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct FieldAttrSerdeSkipSerializingIfOptIsNone;
impl ToTokens for FieldAttrSerdeSkipSerializingIfOptIsNone {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[serde(skip_serializing_if = "Option::is_none")]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct Bool;
impl ToTokens for Bool {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {bool}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct U8;
impl ToTokens for U8 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct U16;
impl ToTokens for U16 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct U32;
impl ToTokens for U32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct U64;
impl ToTokens for U64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct I8;
impl ToTokens for I8 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct I16;
impl ToTokens for I16 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct I32;
impl ToTokens for I32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct I64;
impl ToTokens for I64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct F32;
impl ToTokens for F32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {f32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct F64;
impl ToTokens for F64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {f64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct UuidUuid;
impl ToTokens for UuidUuid {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {uuid::Uuid}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct StdFmtDisplay;
impl ToTokens for StdFmtDisplay {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {std::fmt::Display}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct CrateDfltSomeOneEl;
impl ToTokens for CrateDfltSomeOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let dflt_some_one_el_ucc = dflt_some_one_el_ucc();
        quote! {
            #crate_path_ts
            #dflt_some_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct CrateDfltSomeOneElCall;
impl ToTokens for CrateDfltSomeOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let dflt_some_one_el_ucc = dflt_some_one_el_ucc();
        let path_dflt_some_one_el_call = path_dflt_some_one_el_call();
        quote! {
            #crate_path_ts
            #dflt_some_one_el_ucc
            #path_dflt_some_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudDfltSomeOneEl;
impl ToTokens for PgCrudDfltSomeOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let dflt_some_one_el_ucc = dflt_some_one_el_ucc();
        quote! {
            #pg_crud
            #dflt_some_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudDfltSomeOneElCall;
impl ToTokens for PgCrudDfltSomeOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let path_dflt_some_one_el_call = path_dflt_some_one_el_call();
        quote! {
            #PgCrudDfltSomeOneEl
            #path_dflt_some_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudCmnDfltSomeOneEl;
impl ToTokens for PgCrudCmnDfltSomeOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_cmn = pg_crud_cmn();
        let dflt_some_one_el_ucc = dflt_some_one_el_ucc();
        quote! {
            #pg_crud_cmn
            #dflt_some_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudCmnDfltSomeOneElCall;
impl ToTokens for PgCrudCmnDfltSomeOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let path_dflt_some_one_el_call = path_dflt_some_one_el_call();
        quote! {
            #PgCrudCmnDfltSomeOneEl
            #path_dflt_some_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct CrateAllEnumVrtsArrDfltSomeOneEl;
impl ToTokens for CrateAllEnumVrtsArrDfltSomeOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let all_vrts_dflt_some_one_el_ucc = all_vrts_dflt_some_one_el_ucc();
        quote! {
            #crate_path_ts
            #all_vrts_dflt_some_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct CrateAllEnumVrtsArrDfltSomeOneElCall;
impl ToTokens for CrateAllEnumVrtsArrDfltSomeOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_all_vrts_dflt_some_one_el = CrateAllEnumVrtsArrDfltSomeOneEl;
        let path_all_vrts_dflt_some_one_el_call = path_all_vrts_dflt_some_one_el_call();
        quote! {
            #crate_all_vrts_dflt_some_one_el
            #path_all_vrts_dflt_some_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudAllEnumVrtsArrDfltSomeOneEl;
impl ToTokens for PgCrudAllEnumVrtsArrDfltSomeOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let all_vrts_dflt_some_one_el_ucc = all_vrts_dflt_some_one_el_ucc();
        quote! {
            #pg_crud
            #all_vrts_dflt_some_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudAllEnumVrtsArrDfltSomeOneElCall;
impl ToTokens for PgCrudAllEnumVrtsArrDfltSomeOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_all_vrts_dflt_some_one_el = PgCrudAllEnumVrtsArrDfltSomeOneEl;
        let path_all_vrts_dflt_some_one_el_call = path_all_vrts_dflt_some_one_el_call();
        quote! {
            #pg_crud_all_vrts_dflt_some_one_el
            #path_all_vrts_dflt_some_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudCmnAllEnumVrtsArrDfltSomeOneEl;
impl ToTokens for PgCrudCmnAllEnumVrtsArrDfltSomeOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_cmn = pg_crud_cmn();
        let all_vrts_dflt_some_one_el_ucc = all_vrts_dflt_some_one_el_ucc();
        quote! {
            #pg_crud_cmn
            #all_vrts_dflt_some_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudCmnAllEnumVrtsArrDfltSomeOneElCall;
impl ToTokens for PgCrudCmnAllEnumVrtsArrDfltSomeOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_cmn_all_vrts_dflt_some_one_el = PgCrudCmnAllEnumVrtsArrDfltSomeOneEl;
        let path_all_vrts_dflt_some_one_el_call = path_all_vrts_dflt_some_one_el_call();
        quote! {
            #pg_crud_cmn_all_vrts_dflt_some_one_el
            #path_all_vrts_dflt_some_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct CrateDfltSomeOneElMaxPageSize;
impl ToTokens for CrateDfltSomeOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let dflt_some_one_el_max_page_size_ucc = dflt_some_one_el_max_page_size_ucc();
        quote! {
            #crate_path_ts
            #dflt_some_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct CrateDfltSomeOneElMaxPageSizeCall;
impl ToTokens for CrateDfltSomeOneElMaxPageSizeCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let dflt_some_one_el_max_page_size_ucc = dflt_some_one_el_max_page_size_ucc();
        let path_dflt_some_one_el_max_page_size_call = path_dflt_some_one_el_max_page_size_call();
        quote! {
            #crate_path_ts
            #dflt_some_one_el_max_page_size_ucc
            #path_dflt_some_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudDfltSomeOneElMaxPageSize;
impl ToTokens for PgCrudDfltSomeOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let dflt_some_one_el_max_page_size_ucc = dflt_some_one_el_max_page_size_ucc();
        quote! {
            #pg_crud
            #dflt_some_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudDfltSomeOneElMaxPageSizeCall;
impl ToTokens for PgCrudDfltSomeOneElMaxPageSizeCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_dflt_some_one_el_max_page_size = PgCrudDfltSomeOneElMaxPageSize;
        let path_dflt_some_one_el_max_page_size_call = path_dflt_some_one_el_max_page_size_call();
        quote! {
            #pg_crud_dflt_some_one_el_max_page_size
            #path_dflt_some_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudCmnDfltSomeOneElMaxPageSize;
impl ToTokens for PgCrudCmnDfltSomeOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_cmn = pg_crud_cmn();
        let dflt_some_one_el_max_page_size_ucc = dflt_some_one_el_max_page_size_ucc();
        quote! {
            #pg_crud_cmn
            #dflt_some_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudCmnDfltSomeOneElMaxPageSizeCall;
impl ToTokens for PgCrudCmnDfltSomeOneElMaxPageSizeCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_cmn_dflt_some_one_el_max_page_size = PgCrudCmnDfltSomeOneElMaxPageSize;
        let path_dflt_some_one_el_max_page_size_call = path_dflt_some_one_el_max_page_size_call();
        quote! {
            #pg_crud_cmn_dflt_some_one_el_max_page_size
            #path_dflt_some_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct CrateAllEnumVrtsArrDfltSomeOneElMaxPageSize;
impl ToTokens for CrateAllEnumVrtsArrDfltSomeOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let all_vrts_dflt_some_one_el_max_page_size_ucc =
            all_vrts_dflt_some_one_el_max_page_size_ucc();
        quote! {
            #crate_path_ts
            #all_vrts_dflt_some_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct CrateAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize;
impl ToTokens for CrateAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_all_vrts_dflt_some_one_el_max_page_size =
            CrateAllEnumVrtsArrDfltSomeOneElMaxPageSize;
        let path_all_vrts_dflt_some_one_el_max_page_size_call =
            path_all_vrts_dflt_some_one_el_max_page_size_call();
        quote! {
            #crate_all_vrts_dflt_some_one_el_max_page_size
            #path_all_vrts_dflt_some_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudAllEnumVrtsArrDfltSomeOneElMaxPageSize;
impl ToTokens for PgCrudAllEnumVrtsArrDfltSomeOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let all_vrts_dflt_some_one_el_max_page_size_ucc =
            all_vrts_dflt_some_one_el_max_page_size_ucc();
        quote! {
            #pg_crud
            #all_vrts_dflt_some_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize;
impl ToTokens for PgCrudAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_all_vrts_dflt_some_one_el_max_page_size =
            PgCrudAllEnumVrtsArrDfltSomeOneElMaxPageSize;
        let path_all_vrts_dflt_some_one_el_max_page_size_call =
            path_all_vrts_dflt_some_one_el_max_page_size_call();
        quote! {
            #pg_crud_all_vrts_dflt_some_one_el_max_page_size
            #path_all_vrts_dflt_some_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudCmnAllEnumVrtsArrDfltSomeOneElMaxPageSize;
impl ToTokens for PgCrudCmnAllEnumVrtsArrDfltSomeOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_cmn = pg_crud_cmn();
        let all_vrts_dflt_some_one_el_max_page_size_ucc =
            all_vrts_dflt_some_one_el_max_page_size_ucc();
        quote! {
            #pg_crud_cmn
            #all_vrts_dflt_some_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct PgCrudCmnAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize;
impl ToTokens for PgCrudCmnAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_cmn_all_vrts_dflt_some_one_el_max_page_size =
            PgCrudCmnAllEnumVrtsArrDfltSomeOneElMaxPageSize;
        let path_all_vrts_dflt_some_one_el_max_page_size_call =
            path_all_vrts_dflt_some_one_el_max_page_size_call();
        quote! {
            #pg_crud_cmn_all_vrts_dflt_some_one_el_max_page_size
            #path_all_vrts_dflt_some_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct MustUse;
impl ToTokens for MustUse {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[must_use]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct AllowClippyArbitrarySrcItemOrdering;
impl ToTokens for AllowClippyArbitrarySrcItemOrdering {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[allow(clippy::arbitrary_source_item_ordering)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub struct NoneTs;
impl ToTokens for NoneTs {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {None}.to_tokens(tokens);
    }
}
//todo better reuse
fn path_all_vrts_dflt_some_one_el_max_page_size_call() -> Ts2 {
    quote! {::all_vrts_dflt_some_one_el_max_page_size()}
}
fn dflt_some_one_el_max_page_size_ucc() -> Ts2 {
    quote! {DfltSomeOneElMaxPageSize}
}
fn crate_path_ts() -> Ts2 {
    quote! {crate::}
}
fn pg_crud() -> Ts2 {
    quote! {pg_crud::}
}
fn pg_crud_cmn() -> Ts2 {
    quote! {pg_crud_cmn::}
}
fn dflt_some_one_el_ucc() -> Ts2 {
    quote! {DfltSomeOneEl}
}
fn all_vrts_dflt_some_one_el_ucc() -> Ts2 {
    quote! {AllEnumVrtsArrDfltSomeOneEl}
}
fn path_dflt_some_one_el_call() -> Ts2 {
    quote! {::dflt_some_one_el()}
}
fn path_dflt_some_one_el_max_page_size_call() -> Ts2 {
    quote! {::dflt_some_one_el_max_page_size()}
}
fn all_vrts_dflt_some_one_el_max_page_size_ucc() -> Ts2 {
    quote! {AllEnumVrtsArrDfltSomeOneElMaxPageSize}
}
fn path_all_vrts_dflt_some_one_el_call() -> Ts2 {
    quote! {::all_vrts_dflt_some_one_el()}
}
