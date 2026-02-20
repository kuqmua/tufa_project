use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
#[derive(Debug, Clone, Copy)]
pub struct SqlxAcquire;
impl ToTokens for SqlxAcquire {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::Acquire}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AxumExtractRejectionJsonRejection;
impl ToTokens for AxumExtractRejectionJsonRejection {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {axum::extract::rejection::JsonRejection}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AxumResIntoRes;
impl ToTokens for AxumResIntoRes {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {axum::response::IntoResponse}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ReqwestEr;
impl ToTokens for ReqwestEr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {reqwest::Error}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ReqwestHeaderHeaderMap;
impl ToTokens for ReqwestHeaderHeaderMap {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {reqwest::header::HeaderMap}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HttpStatusCode;
impl ToTokens for HttpStatusCode {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {http::StatusCode}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxRow;
impl ToTokens for SqlxRow {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::Row}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SerdeSerialize;
impl ToTokens for SerdeSerialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {serde::Serialize}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SerdeDeserialize;
impl ToTokens for SerdeDeserialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {serde::Deserialize}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UtoipaToSchema;
impl ToTokens for UtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {utoipa::ToSchema}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SchemarsJsonSchema;
impl ToTokens for SchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {schemars::JsonSchema}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LocationLibLocation;
impl ToTokens for LocationLibLocation {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {location_lib::Location}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ThiserrorError;
impl ToTokens for ThiserrorError {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {thiserror::Error}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Char;
impl ToTokens for Char {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {char}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RefStr;
impl ToTokens for RefStr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {&str}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StringTs;
impl ToTokens for StringTs {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {String}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebug;
impl ToTokens for DeriveDebug {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugThiserrorErrorOccurence;
impl ToTokens for DeriveDebugThiserrorErrorOccurence {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, thiserror::Error, location_lib::Location)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugUtoipaToSchema;
impl ToTokens for DeriveDebugUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, utoipa::ToSchema)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugSerdeSerializeSerdeDeserialize;
impl ToTokens for DeriveDebugSerdeSerializeSerdeDeserialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, serde::Serialize, serde::Deserialize)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
impl ToTokens for DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]}
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugCloneCopy;
impl ToTokens for DeriveDebugCloneCopy {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, Clone, Copy)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StrSqlxColumnIndex;
impl ToTokens for StrSqlxColumnIndex {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {&'lifetime str: sqlx::ColumnIndex<R>,}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxDecodeDecodeDatabase;
impl ToTokens for SqlxDecodeDecodeDatabase {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::decode::Decode<'lifetime, R::Database>}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxTypesTypeDatabase;
impl ToTokens for SqlxTypesTypeDatabase {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::Type<R::Database>}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LocationLibLocLoc;
impl ToTokens for LocationLibLocLoc {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {location_lib::loc::Loc}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct LocScDoubleDotSpaceLocationLibLocLoc;
impl ToTokens for LocScDoubleDotSpaceLocationLibLocLoc {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {loc: location_lib::loc::Loc}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CoreDefault;
impl ToTokens for CoreDefault {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {::core::default::Default::default()}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxTypesTimeTimeMidnight;
impl ToTokens for SqlxTypesTimeTimeMidnight {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::time::Time::MIDNIGHT}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxTypesTimeOffsetDateTimeUnixEpoch;
impl ToTokens for SqlxTypesTimeOffsetDateTimeUnixEpoch {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::time::OffsetDateTime::UNIX_EPOCH}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Er0;
impl ToTokens for Er0 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_0}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Er1;
impl ToTokens for Er1 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_1}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Er2;
impl ToTokens for Er2 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_2}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Er3;
impl ToTokens for Er3 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_3}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FieldAttrSerdeSkipSerializingIfOptionIsNone;
impl ToTokens for FieldAttrSerdeSkipSerializingIfOptionIsNone {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[serde(skip_serializing_if = "Option::is_none")]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Bool;
impl ToTokens for Bool {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {bool}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct U8;
impl ToTokens for U8 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct U16;
impl ToTokens for U16 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct U32;
impl ToTokens for U32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct U64;
impl ToTokens for U64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct I8;
impl ToTokens for I8 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct I16;
impl ToTokens for I16 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct I32;
impl ToTokens for I32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct I64;
impl ToTokens for I64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct F32;
impl ToTokens for F32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {f32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct F64;
impl ToTokens for F64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {f64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UuidUuid;
impl ToTokens for UuidUuid {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {uuid::Uuid}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdFmtDisplay;
impl ToTokens for StdFmtDisplay {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {std::fmt::Display}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateDefaultOptionSomeVecOneEl;
impl ToTokens for CrateDefaultOptionSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let default_option_some_vec_one_el_ucc = default_option_some_vec_one_el_ucc();
        quote! {
            #crate_path_ts
            #default_option_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateDefaultOptionSomeVecOneElCall;
impl ToTokens for CrateDefaultOptionSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let default_option_some_vec_one_el_ucc = default_option_some_vec_one_el_ucc();
        let path_default_option_some_vec_one_el_call = path_default_option_some_vec_one_el_call();
        quote! {
            #crate_path_ts
            #default_option_some_vec_one_el_ucc
            #path_default_option_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudDefaultOptionSomeVecOneEl;
impl ToTokens for PgCrudDefaultOptionSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let default_option_some_vec_one_el_ucc = default_option_some_vec_one_el_ucc();
        quote! {
            #pg_crud
            #default_option_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudDefaultOptionSomeVecOneElCall;
impl ToTokens for PgCrudDefaultOptionSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_default_option_some_vec_one_el = PgCrudDefaultOptionSomeVecOneEl;
        let path_default_option_some_vec_one_el_call = path_default_option_some_vec_one_el_call();
        quote! {
            #pg_crud_default_option_some_vec_one_el
            #path_default_option_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudCommonDefaultOptionSomeVecOneEl;
impl ToTokens for PgCrudCommonDefaultOptionSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common = pg_crud_common();
        let default_option_some_vec_one_el_ucc = default_option_some_vec_one_el_ucc();
        quote! {
            #pg_crud_common
            #default_option_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudCommonDefaultOptionSomeVecOneElCall;
impl ToTokens for PgCrudCommonDefaultOptionSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common_default_option_some_vec_one_el = PgCrudCommonDefaultOptionSomeVecOneEl;
        let path_default_option_some_vec_one_el_call = path_default_option_some_vec_one_el_call();
        quote! {
            #pg_crud_common_default_option_some_vec_one_el
            #path_default_option_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVrtsArrayDefaultOptionSomeVecOneEl;
impl ToTokens for CrateAllEnumVrtsArrayDefaultOptionSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let all_vrts_default_option_some_vec_one_el_ucc =
            all_vrts_default_option_some_vec_one_el_ucc();
        quote! {
            #crate_path_ts
            #all_vrts_default_option_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVrtsArrayDefaultOptionSomeVecOneElCall;
impl ToTokens for CrateAllEnumVrtsArrayDefaultOptionSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_all_vrts_default_option_some_vec_one_el =
            CrateAllEnumVrtsArrayDefaultOptionSomeVecOneEl;
        let path_all_vrts_default_option_some_vec_one_el_call =
            path_all_vrts_default_option_some_vec_one_el_call();
        quote! {
            #crate_all_vrts_default_option_some_vec_one_el
            #path_all_vrts_default_option_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudAllEnumVrtsArrayDefaultOptionSomeVecOneEl;
impl ToTokens for PgCrudAllEnumVrtsArrayDefaultOptionSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let all_vrts_default_option_some_vec_one_el_ucc =
            all_vrts_default_option_some_vec_one_el_ucc();
        quote! {
            #pg_crud
            #all_vrts_default_option_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudAllEnumVrtsArrayDefaultOptionSomeVecOneElCall;
impl ToTokens for PgCrudAllEnumVrtsArrayDefaultOptionSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_all_vrts_default_option_some_vec_one_el =
            PgCrudAllEnumVrtsArrayDefaultOptionSomeVecOneEl;
        let path_all_vrts_default_option_some_vec_one_el_call =
            path_all_vrts_default_option_some_vec_one_el_call();
        quote! {
            #pg_crud_all_vrts_default_option_some_vec_one_el
            #path_all_vrts_default_option_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudCommonAllEnumVrtsArrayDefaultOptionSomeVecOneEl;
impl ToTokens for PgCrudCommonAllEnumVrtsArrayDefaultOptionSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common = pg_crud_common();
        let all_vrts_default_option_some_vec_one_el_ucc =
            all_vrts_default_option_some_vec_one_el_ucc();
        quote! {
            #pg_crud_common
            #all_vrts_default_option_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudCommonAllEnumVrtsArrayDefaultOptionSomeVecOneElCall;
impl ToTokens for PgCrudCommonAllEnumVrtsArrayDefaultOptionSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common_all_vrts_default_option_some_vec_one_el =
            PgCrudCommonAllEnumVrtsArrayDefaultOptionSomeVecOneEl;
        let path_all_vrts_default_option_some_vec_one_el_call =
            path_all_vrts_default_option_some_vec_one_el_call();
        quote! {
            #pg_crud_common_all_vrts_default_option_some_vec_one_el
            #path_all_vrts_default_option_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateDefaultOptionSomeVecOneElMaxPageSize;
impl ToTokens for CrateDefaultOptionSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let default_option_some_vec_one_el_max_page_size_ucc =
            default_option_some_vec_one_el_max_page_size_ucc();
        quote! {
            #crate_path_ts
            #default_option_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateDefaultOptionSomeVecOneElMaxPageSizeCall;
impl ToTokens for CrateDefaultOptionSomeVecOneElMaxPageSizeCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let default_option_some_vec_one_el_max_page_size_ucc =
            default_option_some_vec_one_el_max_page_size_ucc();
        let path_default_option_some_vec_one_el_max_page_size_call =
            path_default_option_some_vec_one_el_max_page_size_call();
        quote! {
            #crate_path_ts
            #default_option_some_vec_one_el_max_page_size_ucc
            #path_default_option_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudDefaultOptionSomeVecOneElMaxPageSize;
impl ToTokens for PgCrudDefaultOptionSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let default_option_some_vec_one_el_max_page_size_ucc =
            default_option_some_vec_one_el_max_page_size_ucc();
        quote! {
            #pg_crud
            #default_option_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudDefaultOptionSomeVecOneElMaxPageSizeCall;
impl ToTokens for PgCrudDefaultOptionSomeVecOneElMaxPageSizeCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_default_option_some_vec_one_el_max_page_size =
            PgCrudDefaultOptionSomeVecOneElMaxPageSize;
        let path_default_option_some_vec_one_el_max_page_size_call =
            path_default_option_some_vec_one_el_max_page_size_call();
        quote! {
            #pg_crud_default_option_some_vec_one_el_max_page_size
            #path_default_option_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudCommonDefaultOptionSomeVecOneElMaxPageSize;
impl ToTokens for PgCrudCommonDefaultOptionSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common = pg_crud_common();
        let default_option_some_vec_one_el_max_page_size_ucc =
            default_option_some_vec_one_el_max_page_size_ucc();
        quote! {
            #pg_crud_common
            #default_option_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudCommonDefaultOptionSomeVecOneElMaxPageSizeCall;
impl ToTokens for PgCrudCommonDefaultOptionSomeVecOneElMaxPageSizeCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common_default_option_some_vec_one_el_max_page_size =
            PgCrudCommonDefaultOptionSomeVecOneElMaxPageSize;
        let path_default_option_some_vec_one_el_max_page_size_call =
            path_default_option_some_vec_one_el_max_page_size_call();
        quote! {
            #pg_crud_common_default_option_some_vec_one_el_max_page_size
            #path_default_option_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVrtsArrayDefaultOptionSomeVecOneElMaxPageSize;
impl ToTokens for CrateAllEnumVrtsArrayDefaultOptionSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let all_vrts_default_option_some_vec_one_el_max_page_size_ucc =
            all_vrts_default_option_some_vec_one_el_max_page_size_ucc();
        quote! {
            #crate_path_ts
            #all_vrts_default_option_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVrtsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize;
impl ToTokens for CrateAllEnumVrtsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_all_vrts_default_option_some_vec_one_el_max_page_size =
            CrateAllEnumVrtsArrayDefaultOptionSomeVecOneElMaxPageSize;
        let path_all_vrts_default_option_some_vec_one_el_max_page_size_call =
            path_all_vrts_default_option_some_vec_one_el_max_page_size_call();
        quote! {
            #crate_all_vrts_default_option_some_vec_one_el_max_page_size
            #path_all_vrts_default_option_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudAllEnumVrtsArrayDefaultOptionSomeVecOneElMaxPageSize;
impl ToTokens for PgCrudAllEnumVrtsArrayDefaultOptionSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let all_vrts_default_option_some_vec_one_el_max_page_size_ucc =
            all_vrts_default_option_some_vec_one_el_max_page_size_ucc();
        quote! {
            #pg_crud
            #all_vrts_default_option_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudAllEnumVrtsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize;
impl ToTokens for PgCrudAllEnumVrtsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_all_vrts_default_option_some_vec_one_el_max_page_size =
            PgCrudAllEnumVrtsArrayDefaultOptionSomeVecOneElMaxPageSize;
        let path_all_vrts_default_option_some_vec_one_el_max_page_size_call =
            path_all_vrts_default_option_some_vec_one_el_max_page_size_call();
        quote! {
            #pg_crud_all_vrts_default_option_some_vec_one_el_max_page_size
            #path_all_vrts_default_option_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudCommonAllEnumVrtsArrayDefaultOptionSomeVecOneElMaxPageSize;
impl ToTokens for PgCrudCommonAllEnumVrtsArrayDefaultOptionSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common = pg_crud_common();
        let all_vrts_default_option_some_vec_one_el_max_page_size_ucc =
            all_vrts_default_option_some_vec_one_el_max_page_size_ucc();
        quote! {
            #pg_crud_common
            #all_vrts_default_option_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgCrudCommonAllEnumVrtsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize;
impl ToTokens for PgCrudCommonAllEnumVrtsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common_all_vrts_default_option_some_vec_one_el_max_page_size =
            PgCrudCommonAllEnumVrtsArrayDefaultOptionSomeVecOneElMaxPageSize;
        let path_all_vrts_default_option_some_vec_one_el_max_page_size_call =
            path_all_vrts_default_option_some_vec_one_el_max_page_size_call();
        quote! {
            #pg_crud_common_all_vrts_default_option_some_vec_one_el_max_page_size
            #path_all_vrts_default_option_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MustUse;
impl ToTokens for MustUse {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[must_use]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AllowClippyArbitrarySourceItemOrdering;
impl ToTokens for AllowClippyArbitrarySourceItemOrdering {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[allow(clippy::arbitrary_source_item_ordering)]}.to_tokens(tokens);
    }
}
fn path_all_vrts_default_option_some_vec_one_el_max_page_size_call() -> Ts2 {
    quote! {::all_vrts_default_option_some_vec_one_el_max_page_size()}
}
fn default_option_some_vec_one_el_max_page_size_ucc() -> Ts2 {
    quote! {DefaultOptionSomeVecOneElMaxPageSize}
}
fn crate_path_ts() -> Ts2 {
    quote! {crate::}
}
fn pg_crud() -> Ts2 {
    quote! {pg_crud::}
}
fn pg_crud_common() -> Ts2 {
    quote! {pg_crud_common::}
}
fn default_option_some_vec_one_el_ucc() -> Ts2 {
    quote! {DefaultOptionSomeVecOneEl}
}
fn all_vrts_default_option_some_vec_one_el_ucc() -> Ts2 {
    quote! {AllEnumVrtsArrayDefaultOptionSomeVecOneEl}
}
fn path_default_option_some_vec_one_el_call() -> Ts2 {
    quote! {::default_option_some_vec_one_el()}
}
fn path_default_option_some_vec_one_el_max_page_size_call() -> Ts2 {
    quote! {::default_option_some_vec_one_el_max_page_size()}
}
fn all_vrts_default_option_some_vec_one_el_max_page_size_ucc() -> Ts2 {
    quote! {AllEnumVrtsArrayDefaultOptionSomeVecOneElMaxPageSize}
}
fn path_all_vrts_default_option_some_vec_one_el_call() -> Ts2 {
    quote! {::all_vrts_default_option_some_vec_one_el()}
}
