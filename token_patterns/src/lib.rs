#[derive(Debug, Clone, Copy)]
pub struct SqlxAcquire;
impl quote::ToTokens for SqlxAcquire {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {sqlx::Acquire}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AxumExtractRejectionJsonRejection;
impl quote::ToTokens for AxumExtractRejectionJsonRejection {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {axum::extract::rejection::JsonRejection}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AxumResponseIntoResponse;
impl quote::ToTokens for AxumResponseIntoResponse {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {axum::response::IntoResponse}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ReqwestError;
impl quote::ToTokens for ReqwestError {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {reqwest::Error}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ReqwestHeaderHeaderMap;
impl quote::ToTokens for ReqwestHeaderHeaderMap {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {reqwest::header::HeaderMap}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HttpStatusCode;
impl quote::ToTokens for HttpStatusCode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {http::StatusCode}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxRow;
impl quote::ToTokens for SqlxRow {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {sqlx::Row}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SerdeSerialize;
impl quote::ToTokens for SerdeSerialize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {serde::Serialize}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SerdeDeserialize;
impl quote::ToTokens for SerdeDeserialize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {serde::Deserialize}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UtoipaToSchema;
impl quote::ToTokens for UtoipaToSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {utoipa::ToSchema}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SchemarsJsonSchema;
impl quote::ToTokens for SchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {schemars::JsonSchema}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ErrorOccurenceLibErrorOccurence;
impl quote::ToTokens for ErrorOccurenceLibErrorOccurence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {error_occurence_lib::ErrorOccurence}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ThiserrorError;
impl quote::ToTokens for ThiserrorError {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {thiserror::Error}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveChar;
impl quote::ToTokens for StdPrimitiveChar {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {char}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RefStdPrimitiveStr;
impl quote::ToTokens for RefStdPrimitiveStr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {&str}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdStringString;
impl quote::ToTokens for StdStringString {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {String}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebug;
impl quote::ToTokens for DeriveDebug {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {#[derive(Debug)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugThiserrorErrorOccurence;
impl quote::ToTokens for DeriveDebugThiserrorErrorOccurence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]}
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugUtoipaToSchema;
impl quote::ToTokens for DeriveDebugUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {#[derive(Debug, utoipa::ToSchema)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugSerdeSerializeSerdeDeserialize;
impl quote::ToTokens for DeriveDebugSerdeSerializeSerdeDeserialize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {#[derive(Debug, serde::Serialize, serde::Deserialize)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
impl quote::ToTokens for DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]}
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugCloneCopy;
impl quote::ToTokens for DeriveDebugCloneCopy {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {#[derive(Debug, Clone, Copy)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugThisErrorErrorOccurence;
impl quote::ToTokens for DeriveDebugThisErrorErrorOccurence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]}
            .to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveStrSqlxColumnIndex;
impl quote::ToTokens for StdPrimitiveStrSqlxColumnIndex {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {&'lifetime str: sqlx::ColumnIndex<R>,}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxDecodeDecodeDatabase;
impl quote::ToTokens for SqlxDecodeDecodeDatabase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {sqlx::decode::Decode<'lifetime, R::Database>}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxTypesTypeDatabase;
impl quote::ToTokens for SqlxTypesTypeDatabase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {sqlx::types::Type<R::Database>}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ErrorOccurenceLibCodeOccurenceCodeOccurence;
impl quote::ToTokens for ErrorOccurenceLibCodeOccurenceCodeOccurence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {error_occurence_lib::code_occurence::CodeOccurence}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CodeOccurenceSnakeCaseDoubleDotSpaceErrorOccurenceLibCodeOccurenceCodeOccurence;
impl quote::ToTokens
    for CodeOccurenceSnakeCaseDoubleDotSpaceErrorOccurenceLibCodeOccurenceCodeOccurence
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {code_occurence: error_occurence_lib::code_occurence::CodeOccurence}
            .to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CoreDefaultDefaultDefault;
impl quote::ToTokens for CoreDefaultDefaultDefault {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {::core::default::Default::default()}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxTypesTimeTimeMidnight;
impl quote::ToTokens for SqlxTypesTimeTimeMidnight {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {sqlx::types::time::Time::MIDNIGHT}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxTypesTimeOffsetDateTimeUnixEpoch;
impl quote::ToTokens for SqlxTypesTimeOffsetDateTimeUnixEpoch {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {sqlx::types::time::OffsetDateTime::UNIX_EPOCH}.to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Error0;
impl quote::ToTokens for Error0 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {error_0}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Error1;
impl quote::ToTokens for Error1 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {error_1}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Error2;
impl quote::ToTokens for Error2 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {error_2}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Error3;
impl quote::ToTokens for Error3 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {error_3}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FieldAttributeSerdeSkipSerializingIfOptionIsNone;
impl quote::ToTokens for FieldAttributeSerdeSkipSerializingIfOptionIsNone {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {#[serde(skip_serializing_if = "Option::is_none")]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveBool;
impl quote::ToTokens for StdPrimitiveBool {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {bool}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveU8;
impl quote::ToTokens for StdPrimitiveU8 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {u8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveU16;
impl quote::ToTokens for StdPrimitiveU16 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {u16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveU32;
impl quote::ToTokens for StdPrimitiveU32 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {u32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveU64;
impl quote::ToTokens for StdPrimitiveU64 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {u64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI8;
impl quote::ToTokens for StdPrimitiveI8 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {i8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI16;
impl quote::ToTokens for StdPrimitiveI16 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {i16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI32;
impl quote::ToTokens for StdPrimitiveI32 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {i32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI64;
impl quote::ToTokens for StdPrimitiveI64 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {i64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveF32;
impl quote::ToTokens for StdPrimitiveF32 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {f32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveF64;
impl quote::ToTokens for StdPrimitiveF64 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {f64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UuidUuid;
impl quote::ToTokens for UuidUuid {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {uuid::Uuid}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdFmtDisplay;
impl quote::ToTokens for StdFmtDisplay {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::fmt::Display}.to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl;
impl quote::ToTokens for CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_path_ts = crate_path_ts();
        let default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case =
            default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case();
        quote::quote! {
            #crate_path_ts
            #default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall;
impl quote::ToTokens for CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_path_ts = crate_path_ts();
        let default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case =
            default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case();
        let path_default_but_option_is_always_some_and_vec_always_contains_one_el_call =
            path_default_but_option_is_always_some_and_vec_always_contains_one_el_call();
        quote::quote! {
            #crate_path_ts
            #default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case
            #path_default_but_option_is_always_some_and_vec_always_contains_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl;
impl quote::ToTokens for PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud = postgresql_crud();
        let default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case =
            default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case();
        quote::quote! {
            #postgresql_crud
            #default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall;
impl quote::ToTokens for PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_el =
            PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl;
        let path_default_but_option_is_always_some_and_vec_always_contains_one_el_call =
            path_default_but_option_is_always_some_and_vec_always_contains_one_el_call();
        quote::quote! {
            #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_el
            #path_default_but_option_is_always_some_and_vec_always_contains_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl;
impl quote::ToTokens for PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_common = postgresql_crud_common();
        let default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case =
            default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case();
        quote::quote! {
            #postgresql_crud_common
            #default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall;
impl quote::ToTokens
    for PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_el =
            PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl;
        let path_default_but_option_is_always_some_and_vec_always_contains_one_el_call =
            path_default_but_option_is_always_some_and_vec_always_contains_one_el_call();
        quote::quote! {
            #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_el
            #path_default_but_option_is_always_some_and_vec_always_contains_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl;
impl quote::ToTokens
    for CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_path_ts = crate_path_ts();
        let all_variants_array_default_but_option_is_some_and_vec_contains_one_el_upper_camel_case =
            all_variants_array_default_but_option_is_some_and_vec_contains_one_el_upper_camel_case();
        quote::quote! {
            #crate_path_ts
            #all_variants_array_default_but_option_is_some_and_vec_contains_one_el_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall;
impl quote::ToTokens
    for CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_all_variants_array_default_but_option_is_some_and_vec_contains_one_el =
            CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl;
        let path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_call =
            path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_call();
        quote::quote! {
            #crate_all_variants_array_default_but_option_is_some_and_vec_contains_one_el
            #path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl;
impl quote::ToTokens
    for PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud = postgresql_crud();
        let all_variants_array_default_but_option_is_some_and_vec_contains_one_el_upper_camel_case =
            all_variants_array_default_but_option_is_some_and_vec_contains_one_el_upper_camel_case();
        quote::quote! {
            #postgresql_crud
            #all_variants_array_default_but_option_is_some_and_vec_contains_one_el_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall;
impl quote::ToTokens
    for PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_all_variants_array_default_but_option_is_some_and_vec_contains_one_el =
            PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl;
        let path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_call =
            path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_call();
        quote::quote! {
            #postgresql_crud_all_variants_array_default_but_option_is_some_and_vec_contains_one_el
            #path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl;
impl quote::ToTokens for PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_common = postgresql_crud_common();
        let all_variants_array_default_but_option_is_some_and_vec_contains_one_el_upper_camel_case = all_variants_array_default_but_option_is_some_and_vec_contains_one_el_upper_camel_case();
        quote::quote! {
            #postgresql_crud_common
            #all_variants_array_default_but_option_is_some_and_vec_contains_one_el_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall;
impl quote::ToTokens for PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCall {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_common_all_variants_array_default_but_option_is_some_and_vec_contains_one_el = PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl;
        let path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_call = path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_call();
        quote::quote! {
            #postgresql_crud_common_all_variants_array_default_but_option_is_some_and_vec_contains_one_el
            #path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize;
impl quote::ToTokens for CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_path_ts = crate_path_ts();
        let default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case = default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case();
        quote::quote! {
            #crate_path_ts
            #default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSizeCall;
impl quote::ToTokens
    for CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSizeCall
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_path_ts = crate_path_ts();
        let default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case = default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case();
        let path_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_call = path_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_call();
        quote::quote! {
            #crate_path_ts
            #default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case
            #path_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize;
impl quote::ToTokens
    for PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud = postgresql_crud();
        let default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case = default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case();
        quote::quote! {
            #postgresql_crud
            #default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSizeCall;
impl quote::ToTokens
    for PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSizeCall
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size =
            PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize;
        let path_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_call = path_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_call();
        quote::quote! {
            #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size
            #path_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize;
impl quote::ToTokens
    for PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_common = postgresql_crud_common();
        let default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case = default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case();
        quote::quote! {
            #postgresql_crud_common
            #default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSizeCall;
impl quote::ToTokens
    for PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSizeCall
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size =
            PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize;
        let path_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_call = path_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_call();
        quote::quote! {
            #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size
            #path_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize;
impl quote::ToTokens for CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_path_ts = crate_path_ts();
        let all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_upper_camel_case = all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_upper_camel_case();
        quote::quote! {
            #crate_path_ts
            #all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCallWithMaxPageSize;
impl quote::ToTokens for CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size = CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize;
        let path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_call = path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_call();
        quote::quote! {
            #crate_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size
            #path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize;
impl quote::ToTokens for PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud = postgresql_crud();
        let all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_upper_camel_case = all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_upper_camel_case();
        quote::quote! {
            #postgresql_crud
            #all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCallWithMaxPageSize;
impl quote::ToTokens for PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size = PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize;
        let path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_call = path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_call();
        quote::quote! {
            #postgresql_crud_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size
            #path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize;
impl quote::ToTokens for PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_common = postgresql_crud_common();
        let all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_upper_camel_case = all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_upper_camel_case();
        quote::quote! {
            #postgresql_crud_common
            #all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCallWithMaxPageSize;
impl quote::ToTokens for PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_common_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size = PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize;
        let path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_call = path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_call();
        quote::quote! {
            #postgresql_crud_common_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size
            #path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MustUse;
impl quote::ToTokens for MustUse {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {#[must_use]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AllowClippyArbitrarySourceItemOrdering;
impl quote::ToTokens for AllowClippyArbitrarySourceItemOrdering {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {#[allow(clippy::arbitrary_source_item_ordering)]}.to_tokens(tokens);
    }
}
fn path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_call()
-> proc_macro2::TokenStream {
    quote::quote! {::all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size()}
}
fn default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_upper_camel_case()
-> proc_macro2::TokenStream {
    quote::quote! {DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize}
}
fn crate_path_ts() -> proc_macro2::TokenStream {
    quote::quote! {crate::}
}
fn postgresql_crud() -> proc_macro2::TokenStream {
    quote::quote! {postgresql_crud::}
}
fn postgresql_crud_common() -> proc_macro2::TokenStream {
    quote::quote! {postgresql_crud_common::}
}
fn default_but_option_is_always_some_and_vec_always_contains_one_el_upper_camel_case()
-> proc_macro2::TokenStream {
    quote::quote! {DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl}
}
fn all_variants_array_default_but_option_is_some_and_vec_contains_one_el_upper_camel_case()
-> proc_macro2::TokenStream {
    quote::quote! {AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneEl}
}
fn path_default_but_option_is_always_some_and_vec_always_contains_one_el_call()
-> proc_macro2::TokenStream {
    quote::quote! {::default_but_option_is_always_some_and_vec_always_contains_one_el()}
}
fn path_default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size_call()
-> proc_macro2::TokenStream {
    quote::quote! {::default_but_option_is_always_some_and_vec_always_contains_one_el_with_max_page_size()}
}
fn all_variants_array_default_but_option_is_some_and_vec_contains_one_el_with_max_page_size_upper_camel_case()
-> proc_macro2::TokenStream {
    quote::quote! {AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElWithMaxPageSize}
}
fn path_all_variants_array_default_but_option_is_some_and_vec_contains_one_el_call()
-> proc_macro2::TokenStream {
    quote::quote! {::all_variants_array_default_but_option_is_some_and_vec_contains_one_el()}
}
