use optml::Optml;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
macro_rules! tp {
    ($name:ident, $($tt:tt)*) => {
        #[derive(Debug, Clone, Copy, Optml)]
        pub struct $name;
        impl ToTokens for $name {
            fn to_tokens(&self, tokens: &mut Ts2) {
                quote! {$($tt)*}.to_tokens(tokens);
            }
        }
    };
}
macro_rules! tp_parts {
    ($name:ident, $($part:expr),+) => {
        #[derive(Debug, Clone, Copy, Optml)]
        pub struct $name;
        impl ToTokens for $name {
            fn to_tokens(&self, tokens: &mut Ts2) {
                $($part.to_tokens(tokens);)+
            }
        }
    };
}
tp!(SqlxAcquire, sqlx::Acquire);
tp!(
    AxumExtractRejectionJsonRejection,
    axum::extract::rejection::JsonRejection
);
tp!(AxumResIntoRes, axum::response::IntoResponse);
tp!(ReqwestEr, reqwest::Error);
tp!(ReqwestHeaderHeaderMap, reqwest::header::HeaderMap);
tp!(HttpStatusCode, http::StatusCode);
tp!(SqlxRow, sqlx::Row);
tp!(SerdeSerialize, serde::Serialize);
tp!(SerdeDeserialize, serde::Deserialize);
tp!(UtoipaToSchema, utoipa::ToSchema);
tp!(SchemarsJsonSchema, schemars::JsonSchema);
tp!(LocLibLoc, loc_lib::Location);
tp!(ThiserrorError, thiserror::Error);
tp!(Char, char);
tp!(RefStr, &str);
tp!(StringTs, String);
tp!(DeriveDebug, #[derive(Debug, Optml)]);
tp!(DeriveDebugThiserrorLoc, #[derive(Debug, thiserror::Error, loc_lib::Location, Optml)]);
tp!(DeriveDebugUtoipaToSchema, #[derive(Debug, utoipa::ToSchema, Optml)]);
tp!(DeriveDebugSerdeSerializeSerdeDeserialize, #[derive(Debug, serde::Serialize, serde::Deserialize, Optml)]);
tp!(DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema, #[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Optml)]);
tp!(DeriveDebugCloneCopy, #[derive(Debug, Clone, Copy, Optml)]);
tp!(StrSqlxColIndex, &'lt str: sqlx::ColumnIndex<R>,);
tp!(
    SqlxDecodeDecodeDatabase,
    sqlx::decode::Decode<'lt, R::Database>
);
tp!(SqlxTypesTypeDatabase, sqlx::types::Type<R::Database>);
tp!(LocLibLocLoc, loc_lib::loc::Loc);
tp!(LocScDoubleDotSpaceLocLibLocLoc, loc: loc_lib::loc::Loc);
tp!(CoreDefault, ::core::default::Default::default());
tp!(SqlxTypesTimeTimeMidnight, sqlx::types::time::Time::MIDNIGHT);
tp!(
    SqlxTypesTimeOffsetDateTimeUnixEpoch,
    sqlx::types::time::OffsetDateTime::UNIX_EPOCH
);
tp!(Er0, er_0);
tp!(Er1, er_1);
tp!(Er2, er_2);
tp!(Er3, er_3);
tp!(FieldAttrSerdeSkipSerializingIfOptIsNone, #[serde(skip_serializing_if = "Option::is_none")]);
tp!(Bool, bool);
tp!(U8, u8);
tp!(U16, u16);
tp!(U32, u32);
tp!(U64, u64);
tp!(I8, i8);
tp!(I16, i16);
tp!(I32, i32);
tp!(I64, i64);
tp!(F32, f32);
tp!(F64, f64);
tp!(UuidUuid, uuid::Uuid);
tp!(StdFmtDisplay, std::fmt::Display);
tp_parts!(CrateDfltSomeOneEl, crate_path_ts(), dflt_some_one_el_ucc());
tp_parts!(
    CrateDfltSomeOneElCall,
    crate_path_ts(),
    dflt_some_one_el_ucc(),
    path_dflt_some_one_el_call()
);
tp_parts!(PgCrudDfltSomeOneEl, pg_crud(), dflt_some_one_el_ucc());
tp_parts!(
    PgCrudDfltSomeOneElCall,
    PgCrudDfltSomeOneEl,
    path_dflt_some_one_el_call()
);
tp_parts!(
    PgCrudCmnDfltSomeOneEl,
    pg_crud_cmn(),
    dflt_some_one_el_ucc()
);
tp_parts!(
    PgCrudCmnDfltSomeOneElCall,
    PgCrudCmnDfltSomeOneEl,
    path_dflt_some_one_el_call()
);
tp_parts!(
    CrateAllEnumVrtsArrDfltSomeOneEl,
    crate_path_ts(),
    all_vrts_dflt_some_one_el_ucc()
);
tp_parts!(
    CrateAllEnumVrtsArrDfltSomeOneElCall,
    CrateAllEnumVrtsArrDfltSomeOneEl,
    path_all_vrts_dflt_some_one_el_call()
);
tp_parts!(
    PgCrudAllEnumVrtsArrDfltSomeOneEl,
    pg_crud(),
    all_vrts_dflt_some_one_el_ucc()
);
tp_parts!(
    PgCrudAllEnumVrtsArrDfltSomeOneElCall,
    PgCrudAllEnumVrtsArrDfltSomeOneEl,
    path_all_vrts_dflt_some_one_el_call()
);
tp_parts!(
    PgCrudCmnAllEnumVrtsArrDfltSomeOneEl,
    pg_crud_cmn(),
    all_vrts_dflt_some_one_el_ucc()
);
tp_parts!(
    PgCrudCmnAllEnumVrtsArrDfltSomeOneElCall,
    PgCrudCmnAllEnumVrtsArrDfltSomeOneEl,
    path_all_vrts_dflt_some_one_el_call()
);
tp_parts!(
    CrateDfltSomeOneElMaxPageSize,
    crate_path_ts(),
    dflt_some_one_el_max_page_size_ucc()
);
tp_parts!(
    CrateDfltSomeOneElMaxPageSizeCall,
    crate_path_ts(),
    dflt_some_one_el_max_page_size_ucc(),
    path_dflt_some_one_el_max_page_size_call()
);
tp_parts!(
    PgCrudDfltSomeOneElMaxPageSize,
    pg_crud(),
    dflt_some_one_el_max_page_size_ucc()
);
tp_parts!(
    PgCrudDfltSomeOneElMaxPageSizeCall,
    PgCrudDfltSomeOneElMaxPageSize,
    path_dflt_some_one_el_max_page_size_call()
);
tp_parts!(
    PgCrudCmnDfltSomeOneElMaxPageSize,
    pg_crud_cmn(),
    dflt_some_one_el_max_page_size_ucc()
);
tp_parts!(
    PgCrudCmnDfltSomeOneElMaxPageSizeCall,
    PgCrudCmnDfltSomeOneElMaxPageSize,
    path_dflt_some_one_el_max_page_size_call()
);
tp_parts!(
    CrateAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    crate_path_ts(),
    all_vrts_dflt_some_one_el_max_page_size_ucc()
);
tp_parts!(
    CrateAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize,
    CrateAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    path_all_vrts_dflt_some_one_el_max_page_size_call()
);
tp_parts!(
    PgCrudAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    pg_crud(),
    all_vrts_dflt_some_one_el_max_page_size_ucc()
);
tp_parts!(
    PgCrudAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize,
    PgCrudAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    path_all_vrts_dflt_some_one_el_max_page_size_call()
);
tp_parts!(
    PgCrudCmnAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    pg_crud_cmn(),
    all_vrts_dflt_some_one_el_max_page_size_ucc()
);
tp_parts!(
    PgCrudCmnAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize,
    PgCrudCmnAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    path_all_vrts_dflt_some_one_el_max_page_size_call()
);
tp!(MustUse, #[must_use]);
tp!(AllowClippyArbitrarySrcItemOrdering, #[allow(clippy::arbitrary_source_item_ordering)]);
tp!(NoneTs, None);
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
