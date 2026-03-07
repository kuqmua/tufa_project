use optimal_pack::OptimalPack;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct SqlxAcquire;
impl ToTokens for SqlxAcquire {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::Acquire}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct AxumExtractRejectionJsonRejection;
impl ToTokens for AxumExtractRejectionJsonRejection {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {axum::extract::rejection::JsonRejection}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct AxumResIntoRes;
impl ToTokens for AxumResIntoRes {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {axum::response::IntoResponse}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct ReqwestEr;
impl ToTokens for ReqwestEr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {reqwest::Error}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct ReqwestHeaderHeaderMap;
impl ToTokens for ReqwestHeaderHeaderMap {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {reqwest::header::HeaderMap}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct HttpStatusCode;
impl ToTokens for HttpStatusCode {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {http::StatusCode}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct SqlxRow;
impl ToTokens for SqlxRow {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::Row}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct SerdeSerialize;
impl ToTokens for SerdeSerialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {serde::Serialize}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct SerdeDeserialize;
impl ToTokens for SerdeDeserialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {serde::Deserialize}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct UtoipaToSchema;
impl ToTokens for UtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {utoipa::ToSchema}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct SchemarsJsonSchema;
impl ToTokens for SchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {schemars::JsonSchema}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct LocationLibLocation;
impl ToTokens for LocationLibLocation {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {location_lib::Location}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct ThiserrorError;
impl ToTokens for ThiserrorError {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {thiserror::Error}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct Char;
impl ToTokens for Char {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {char}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct RefStr;
impl ToTokens for RefStr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {&str}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct StringTs;
impl ToTokens for StringTs {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {String}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct DeriveDebug;
impl ToTokens for DeriveDebug {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, OptimalPack)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct DeriveDebugThiserrorLocation;
impl ToTokens for DeriveDebugThiserrorLocation {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, thiserror::Error, location_lib::Location, OptimalPack)]}
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct DeriveDebugUtoipaToSchema;
impl ToTokens for DeriveDebugUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, utoipa::ToSchema, OptimalPack)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct DeriveDebugSerdeSerializeSerdeDeserialize;
impl ToTokens for DeriveDebugSerdeSerializeSerdeDeserialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, serde::Serialize, serde::Deserialize, OptimalPack)]}
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
impl ToTokens for DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, OptimalPack)]}
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct DeriveDebugCloneCopy;
impl ToTokens for DeriveDebugCloneCopy {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, Clone, Copy, OptimalPack)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct StrSqlxColumnIndex;
impl ToTokens for StrSqlxColumnIndex {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {&'lt str: sqlx::ColumnIndex<R>,}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct SqlxDecodeDecodeDatabase;
impl ToTokens for SqlxDecodeDecodeDatabase {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::decode::Decode<'lt, R::Database>}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct SqlxTypesTypeDatabase;
impl ToTokens for SqlxTypesTypeDatabase {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::Type<R::Database>}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct LocationLibLocLoc;
impl ToTokens for LocationLibLocLoc {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {location_lib::loc::Loc}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct LocScDoubleDotSpaceLocationLibLocLoc;
impl ToTokens for LocScDoubleDotSpaceLocationLibLocLoc {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {loc: location_lib::loc::Loc}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct CoreDefault;
impl ToTokens for CoreDefault {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {::core::default::Default::default()}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct SqlxTypesTimeTimeMidnight;
impl ToTokens for SqlxTypesTimeTimeMidnight {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::time::Time::MIDNIGHT}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct SqlxTypesTimeOffsetDateTimeUnixEpoch;
impl ToTokens for SqlxTypesTimeOffsetDateTimeUnixEpoch {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::time::OffsetDateTime::UNIX_EPOCH}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct Er0;
impl ToTokens for Er0 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_0}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct Er1;
impl ToTokens for Er1 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_1}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct Er2;
impl ToTokens for Er2 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_2}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct Er3;
impl ToTokens for Er3 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {er_3}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct FieldAttrSerdeSkipSerializingIfOptIsNone;
impl ToTokens for FieldAttrSerdeSkipSerializingIfOptIsNone {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[serde(skip_serializing_if = "Option::is_none")]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct Bool;
impl ToTokens for Bool {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {bool}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct U8;
impl ToTokens for U8 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct U16;
impl ToTokens for U16 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct U32;
impl ToTokens for U32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct U64;
impl ToTokens for U64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct I8;
impl ToTokens for I8 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct I16;
impl ToTokens for I16 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct I32;
impl ToTokens for I32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct I64;
impl ToTokens for I64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct F32;
impl ToTokens for F32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {f32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct F64;
impl ToTokens for F64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {f64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct UuidUuid;
impl ToTokens for UuidUuid {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {uuid::Uuid}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct StdFmtDisplay;
impl ToTokens for StdFmtDisplay {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {std::fmt::Display}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct CrateDfltOptSomeVecOneEl;
impl ToTokens for CrateDfltOptSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let dflt_opt_some_vec_one_el_ucc = dflt_opt_some_vec_one_el_ucc();
        quote! {
            #crate_path_ts
            #dflt_opt_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct CrateDfltOptSomeVecOneElCall;
impl ToTokens for CrateDfltOptSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let dflt_opt_some_vec_one_el_ucc = dflt_opt_some_vec_one_el_ucc();
        let path_dflt_opt_some_vec_one_el_call = path_dflt_opt_some_vec_one_el_call();
        quote! {
            #crate_path_ts
            #dflt_opt_some_vec_one_el_ucc
            #path_dflt_opt_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudDfltOptSomeVecOneEl;
impl ToTokens for PgCrudDfltOptSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let dflt_opt_some_vec_one_el_ucc = dflt_opt_some_vec_one_el_ucc();
        quote! {
            #pg_crud
            #dflt_opt_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudDfltOptSomeVecOneElCall;
impl ToTokens for PgCrudDfltOptSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let path_dflt_opt_some_vec_one_el_call = path_dflt_opt_some_vec_one_el_call();
        quote! {
            #PgCrudDfltOptSomeVecOneEl
            #path_dflt_opt_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudCommonDfltOptSomeVecOneEl;
impl ToTokens for PgCrudCommonDfltOptSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common = pg_crud_common();
        let dflt_opt_some_vec_one_el_ucc = dflt_opt_some_vec_one_el_ucc();
        quote! {
            #pg_crud_common
            #dflt_opt_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudCommonDfltOptSomeVecOneElCall;
impl ToTokens for PgCrudCommonDfltOptSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let path_dflt_opt_some_vec_one_el_call = path_dflt_opt_some_vec_one_el_call();
        quote! {
            #PgCrudCommonDfltOptSomeVecOneEl
            #path_dflt_opt_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct CrateAllEnumVrtsArrDfltOptSomeVecOneEl;
impl ToTokens for CrateAllEnumVrtsArrDfltOptSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let all_vrts_dflt_opt_some_vec_one_el_ucc = all_vrts_dflt_opt_some_vec_one_el_ucc();
        quote! {
            #crate_path_ts
            #all_vrts_dflt_opt_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct CrateAllEnumVrtsArrDfltOptSomeVecOneElCall;
impl ToTokens for CrateAllEnumVrtsArrDfltOptSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_all_vrts_dflt_opt_some_vec_one_el = CrateAllEnumVrtsArrDfltOptSomeVecOneEl;
        let path_all_vrts_dflt_opt_some_vec_one_el_call =
            path_all_vrts_dflt_opt_some_vec_one_el_call();
        quote! {
            #crate_all_vrts_dflt_opt_some_vec_one_el
            #path_all_vrts_dflt_opt_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudAllEnumVrtsArrDfltOptSomeVecOneEl;
impl ToTokens for PgCrudAllEnumVrtsArrDfltOptSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let all_vrts_dflt_opt_some_vec_one_el_ucc = all_vrts_dflt_opt_some_vec_one_el_ucc();
        quote! {
            #pg_crud
            #all_vrts_dflt_opt_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudAllEnumVrtsArrDfltOptSomeVecOneElCall;
impl ToTokens for PgCrudAllEnumVrtsArrDfltOptSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_all_vrts_dflt_opt_some_vec_one_el = PgCrudAllEnumVrtsArrDfltOptSomeVecOneEl;
        let path_all_vrts_dflt_opt_some_vec_one_el_call =
            path_all_vrts_dflt_opt_some_vec_one_el_call();
        quote! {
            #pg_crud_all_vrts_dflt_opt_some_vec_one_el
            #path_all_vrts_dflt_opt_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneEl;
impl ToTokens for PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common = pg_crud_common();
        let all_vrts_dflt_opt_some_vec_one_el_ucc = all_vrts_dflt_opt_some_vec_one_el_ucc();
        quote! {
            #pg_crud_common
            #all_vrts_dflt_opt_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneElCall;
impl ToTokens for PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common_all_vrts_dflt_opt_some_vec_one_el =
            PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneEl;
        let path_all_vrts_dflt_opt_some_vec_one_el_call =
            path_all_vrts_dflt_opt_some_vec_one_el_call();
        quote! {
            #pg_crud_common_all_vrts_dflt_opt_some_vec_one_el
            #path_all_vrts_dflt_opt_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct CrateDfltOptSomeVecOneElMaxPageSize;
impl ToTokens for CrateDfltOptSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let dflt_opt_some_vec_one_el_max_page_size_ucc =
            dflt_opt_some_vec_one_el_max_page_size_ucc();
        quote! {
            #crate_path_ts
            #dflt_opt_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct CrateDfltOptSomeVecOneElMaxPageSizeCall;
impl ToTokens for CrateDfltOptSomeVecOneElMaxPageSizeCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let dflt_opt_some_vec_one_el_max_page_size_ucc =
            dflt_opt_some_vec_one_el_max_page_size_ucc();
        let path_dflt_opt_some_vec_one_el_max_page_size_call =
            path_dflt_opt_some_vec_one_el_max_page_size_call();
        quote! {
            #crate_path_ts
            #dflt_opt_some_vec_one_el_max_page_size_ucc
            #path_dflt_opt_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudDfltOptSomeVecOneElMaxPageSize;
impl ToTokens for PgCrudDfltOptSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let dflt_opt_some_vec_one_el_max_page_size_ucc =
            dflt_opt_some_vec_one_el_max_page_size_ucc();
        quote! {
            #pg_crud
            #dflt_opt_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudDfltOptSomeVecOneElMaxPageSizeCall;
impl ToTokens for PgCrudDfltOptSomeVecOneElMaxPageSizeCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_dflt_opt_some_vec_one_el_max_page_size = PgCrudDfltOptSomeVecOneElMaxPageSize;
        let path_dflt_opt_some_vec_one_el_max_page_size_call =
            path_dflt_opt_some_vec_one_el_max_page_size_call();
        quote! {
            #pg_crud_dflt_opt_some_vec_one_el_max_page_size
            #path_dflt_opt_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudCommonDfltOptSomeVecOneElMaxPageSize;
impl ToTokens for PgCrudCommonDfltOptSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common = pg_crud_common();
        let dflt_opt_some_vec_one_el_max_page_size_ucc =
            dflt_opt_some_vec_one_el_max_page_size_ucc();
        quote! {
            #pg_crud_common
            #dflt_opt_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudCommonDfltOptSomeVecOneElMaxPageSizeCall;
impl ToTokens for PgCrudCommonDfltOptSomeVecOneElMaxPageSizeCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common_dflt_opt_some_vec_one_el_max_page_size =
            PgCrudCommonDfltOptSomeVecOneElMaxPageSize;
        let path_dflt_opt_some_vec_one_el_max_page_size_call =
            path_dflt_opt_some_vec_one_el_max_page_size_call();
        quote! {
            #pg_crud_common_dflt_opt_some_vec_one_el_max_page_size
            #path_dflt_opt_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct CrateAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize;
impl ToTokens for CrateAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let all_vrts_dflt_opt_some_vec_one_el_max_page_size_ucc =
            all_vrts_dflt_opt_some_vec_one_el_max_page_size_ucc();
        quote! {
            #crate_path_ts
            #all_vrts_dflt_opt_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct CrateAllEnumVrtsArrDfltOptSomeVecOneElCallWithMaxPageSize;
impl ToTokens for CrateAllEnumVrtsArrDfltOptSomeVecOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_all_vrts_dflt_opt_some_vec_one_el_max_page_size =
            CrateAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize;
        let path_all_vrts_dflt_opt_some_vec_one_el_max_page_size_call =
            path_all_vrts_dflt_opt_some_vec_one_el_max_page_size_call();
        quote! {
            #crate_all_vrts_dflt_opt_some_vec_one_el_max_page_size
            #path_all_vrts_dflt_opt_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize;
impl ToTokens for PgCrudAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud = pg_crud();
        let all_vrts_dflt_opt_some_vec_one_el_max_page_size_ucc =
            all_vrts_dflt_opt_some_vec_one_el_max_page_size_ucc();
        quote! {
            #pg_crud
            #all_vrts_dflt_opt_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudAllEnumVrtsArrDfltOptSomeVecOneElCallWithMaxPageSize;
impl ToTokens for PgCrudAllEnumVrtsArrDfltOptSomeVecOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_all_vrts_dflt_opt_some_vec_one_el_max_page_size =
            PgCrudAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize;
        let path_all_vrts_dflt_opt_some_vec_one_el_max_page_size_call =
            path_all_vrts_dflt_opt_some_vec_one_el_max_page_size_call();
        quote! {
            #pg_crud_all_vrts_dflt_opt_some_vec_one_el_max_page_size
            #path_all_vrts_dflt_opt_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize;
impl ToTokens for PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common = pg_crud_common();
        let all_vrts_dflt_opt_some_vec_one_el_max_page_size_ucc =
            all_vrts_dflt_opt_some_vec_one_el_max_page_size_ucc();
        quote! {
            #pg_crud_common
            #all_vrts_dflt_opt_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneElCallWithMaxPageSize;
impl ToTokens for PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let pg_crud_common_all_vrts_dflt_opt_some_vec_one_el_max_page_size =
            PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize;
        let path_all_vrts_dflt_opt_some_vec_one_el_max_page_size_call =
            path_all_vrts_dflt_opt_some_vec_one_el_max_page_size_call();
        quote! {
            #pg_crud_common_all_vrts_dflt_opt_some_vec_one_el_max_page_size
            #path_all_vrts_dflt_opt_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct MustUse;
impl ToTokens for MustUse {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[must_use]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct AllowClippyArbitrarySourceItemOrdering;
impl ToTokens for AllowClippyArbitrarySourceItemOrdering {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[allow(clippy::arbitrary_source_item_ordering)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub struct NoneTs;
impl ToTokens for NoneTs {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {None}.to_tokens(tokens);
    }
}
//todo better reuse
fn path_all_vrts_dflt_opt_some_vec_one_el_max_page_size_call() -> Ts2 {
    quote! {::all_vrts_dflt_opt_some_vec_one_el_max_page_size()}
}
fn dflt_opt_some_vec_one_el_max_page_size_ucc() -> Ts2 {
    quote! {DfltOptSomeVecOneElMaxPageSize}
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
fn dflt_opt_some_vec_one_el_ucc() -> Ts2 {
    quote! {DfltOptSomeVecOneEl}
}
fn all_vrts_dflt_opt_some_vec_one_el_ucc() -> Ts2 {
    quote! {AllEnumVrtsArrDfltOptSomeVecOneEl}
}
fn path_dflt_opt_some_vec_one_el_call() -> Ts2 {
    quote! {::dflt_opt_some_vec_one_el()}
}
fn path_dflt_opt_some_vec_one_el_max_page_size_call() -> Ts2 {
    quote! {::dflt_opt_some_vec_one_el_max_page_size()}
}
fn all_vrts_dflt_opt_some_vec_one_el_max_page_size_ucc() -> Ts2 {
    quote! {AllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize}
}
fn path_all_vrts_dflt_opt_some_vec_one_el_call() -> Ts2 {
    quote! {::all_vrts_dflt_opt_some_vec_one_el()}
}
