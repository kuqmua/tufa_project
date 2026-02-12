use proc_macro2::TokenStream as Ts2;
use quote::quote;
#[derive(Debug, Clone, Copy)]
pub struct SqlxAcquire;
impl quote::ToTokens for SqlxAcquire {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::Acquire}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AxumExtractRejectionJsonRejection;
impl quote::ToTokens for AxumExtractRejectionJsonRejection {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {axum::extract::rejection::JsonRejection}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AxumResponseIntoResponse;
impl quote::ToTokens for AxumResponseIntoResponse {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {axum::response::IntoResponse}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ReqwestError;
impl quote::ToTokens for ReqwestError {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {reqwest::Error}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ReqwestHeaderHeaderMap;
impl quote::ToTokens for ReqwestHeaderHeaderMap {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {reqwest::header::HeaderMap}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct HttpStatusCode;
impl quote::ToTokens for HttpStatusCode {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {http::StatusCode}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxRow;
impl quote::ToTokens for SqlxRow {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::Row}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SerdeSerialize;
impl quote::ToTokens for SerdeSerialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {serde::Serialize}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SerdeDeserialize;
impl quote::ToTokens for SerdeDeserialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {serde::Deserialize}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UtoipaToSchema;
impl quote::ToTokens for UtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {utoipa::ToSchema}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SchemarsJsonSchema;
impl quote::ToTokens for SchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {schemars::JsonSchema}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ErrorOccurenceLibErrorOccurence;
impl quote::ToTokens for ErrorOccurenceLibErrorOccurence {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {error_occurence_lib::ErrorOccurence}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ThiserrorError;
impl quote::ToTokens for ThiserrorError {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {thiserror::Error}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveChar;
impl quote::ToTokens for StdPrimitiveChar {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {char}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RefStdPrimitiveStr;
impl quote::ToTokens for RefStdPrimitiveStr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {&str}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdStringString;
impl quote::ToTokens for StdStringString {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {String}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebug;
impl quote::ToTokens for DeriveDebug {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugThiserrorErrorOccurence;
impl quote::ToTokens for DeriveDebugThiserrorErrorOccurence {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]}
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugUtoipaToSchema;
impl quote::ToTokens for DeriveDebugUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, utoipa::ToSchema)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugSerdeSerializeSerdeDeserialize;
impl quote::ToTokens for DeriveDebugSerdeSerializeSerdeDeserialize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, serde::Serialize, serde::Deserialize)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
impl quote::ToTokens for DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]}
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugCloneCopy;
impl quote::ToTokens for DeriveDebugCloneCopy {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, Clone, Copy)]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeriveDebugThisErrorErrorOccurence;
impl quote::ToTokens for DeriveDebugThisErrorErrorOccurence {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]}
            .to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveStrSqlxColumnIndex;
impl quote::ToTokens for StdPrimitiveStrSqlxColumnIndex {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {&'lifetime str: sqlx::ColumnIndex<R>,}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxDecodeDecodeDatabase;
impl quote::ToTokens for SqlxDecodeDecodeDatabase {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::decode::Decode<'lifetime, R::Database>}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxTypesTypeDatabase;
impl quote::ToTokens for SqlxTypesTypeDatabase {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::Type<R::Database>}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ErrorOccurenceLibCodeOccurenceCodeOccurence;
impl quote::ToTokens for ErrorOccurenceLibCodeOccurenceCodeOccurence {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {error_occurence_lib::code_occurence::CodeOccurence}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CodeOccurenceScDoubleDotSpaceErrorOccurenceLibCodeOccurenceCodeOccurence;
impl quote::ToTokens for CodeOccurenceScDoubleDotSpaceErrorOccurenceLibCodeOccurenceCodeOccurence {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {code_occurence: error_occurence_lib::code_occurence::CodeOccurence}
            .to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CoreDefaultDefaultDefault;
impl quote::ToTokens for CoreDefaultDefaultDefault {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {::core::default::Default::default()}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxTypesTimeTimeMidnight;
impl quote::ToTokens for SqlxTypesTimeTimeMidnight {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::time::Time::MIDNIGHT}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxTypesTimeOffsetDateTimeUnixEpoch;
impl quote::ToTokens for SqlxTypesTimeOffsetDateTimeUnixEpoch {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {sqlx::types::time::OffsetDateTime::UNIX_EPOCH}.to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Error0;
impl quote::ToTokens for Error0 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {error_0}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Error1;
impl quote::ToTokens for Error1 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {error_1}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Error2;
impl quote::ToTokens for Error2 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {error_2}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Error3;
impl quote::ToTokens for Error3 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {error_3}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct FieldAttributeSerdeSkipSerializingIfOptionIsNone;
impl quote::ToTokens for FieldAttributeSerdeSkipSerializingIfOptionIsNone {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[serde(skip_serializing_if = "Option::is_none")]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveBool;
impl quote::ToTokens for StdPrimitiveBool {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {bool}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveU8;
impl quote::ToTokens for StdPrimitiveU8 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveU16;
impl quote::ToTokens for StdPrimitiveU16 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveU32;
impl quote::ToTokens for StdPrimitiveU32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveU64;
impl quote::ToTokens for StdPrimitiveU64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {u64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI8;
impl quote::ToTokens for StdPrimitiveI8 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI16;
impl quote::ToTokens for StdPrimitiveI16 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI32;
impl quote::ToTokens for StdPrimitiveI32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI64;
impl quote::ToTokens for StdPrimitiveI64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {i64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveF32;
impl quote::ToTokens for StdPrimitiveF32 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {f32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveF64;
impl quote::ToTokens for StdPrimitiveF64 {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {f64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UuidUuid;
impl quote::ToTokens for UuidUuid {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {uuid::Uuid}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdFmtDisplay;
impl quote::ToTokens for StdFmtDisplay {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {std::fmt::Display}.to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CrateDefaultOptionSomeVecOneEl;
impl quote::ToTokens for CrateDefaultOptionSomeVecOneEl {
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
impl quote::ToTokens for CrateDefaultOptionSomeVecOneElCall {
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
pub struct PostgresCrudDefaultOptionSomeVecOneEl;
impl quote::ToTokens for PostgresCrudDefaultOptionSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud = postgres_crud();
        let default_option_some_vec_one_el_ucc = default_option_some_vec_one_el_ucc();
        quote! {
            #postgres_crud
            #default_option_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudDefaultOptionSomeVecOneElCall;
impl quote::ToTokens for PostgresCrudDefaultOptionSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_default_option_some_vec_one_el = PostgresCrudDefaultOptionSomeVecOneEl;
        let path_default_option_some_vec_one_el_call = path_default_option_some_vec_one_el_call();
        quote! {
            #postgres_crud_default_option_some_vec_one_el
            #path_default_option_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudCommonDefaultOptionSomeVecOneEl;
impl quote::ToTokens for PostgresCrudCommonDefaultOptionSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_common = postgres_crud_common();
        let default_option_some_vec_one_el_ucc = default_option_some_vec_one_el_ucc();
        quote! {
            #postgres_crud_common
            #default_option_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudCommonDefaultOptionSomeVecOneElCall;
impl quote::ToTokens for PostgresCrudCommonDefaultOptionSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_common_default_option_some_vec_one_el =
            PostgresCrudCommonDefaultOptionSomeVecOneEl;
        let path_default_option_some_vec_one_el_call = path_default_option_some_vec_one_el_call();
        quote! {
            #postgres_crud_common_default_option_some_vec_one_el
            #path_default_option_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVariantsArrayDefaultOptionSomeVecOneEl;
impl quote::ToTokens for CrateAllEnumVariantsArrayDefaultOptionSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let all_variants_default_option_some_vec_one_el_ucc =
            all_variants_default_option_some_vec_one_el_ucc();
        quote! {
            #crate_path_ts
            #all_variants_default_option_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVariantsArrayDefaultOptionSomeVecOneElCall;
impl quote::ToTokens for CrateAllEnumVariantsArrayDefaultOptionSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_all_variants_default_option_some_vec_one_el =
            CrateAllEnumVariantsArrayDefaultOptionSomeVecOneEl;
        let path_all_variants_default_option_some_vec_one_el_call =
            path_all_variants_default_option_some_vec_one_el_call();
        quote! {
            #crate_all_variants_default_option_some_vec_one_el
            #path_all_variants_default_option_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudAllEnumVariantsArrayDefaultOptionSomeVecOneEl;
impl quote::ToTokens for PostgresCrudAllEnumVariantsArrayDefaultOptionSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud = postgres_crud();
        let all_variants_default_option_some_vec_one_el_ucc =
            all_variants_default_option_some_vec_one_el_ucc();
        quote! {
            #postgres_crud
            #all_variants_default_option_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudAllEnumVariantsArrayDefaultOptionSomeVecOneElCall;
impl quote::ToTokens for PostgresCrudAllEnumVariantsArrayDefaultOptionSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_all_variants_default_option_some_vec_one_el =
            PostgresCrudAllEnumVariantsArrayDefaultOptionSomeVecOneEl;
        let path_all_variants_default_option_some_vec_one_el_call =
            path_all_variants_default_option_some_vec_one_el_call();
        quote! {
            #postgres_crud_all_variants_default_option_some_vec_one_el
            #path_all_variants_default_option_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneEl;
impl quote::ToTokens for PostgresCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneEl {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_common = postgres_crud_common();
        let all_variants_default_option_some_vec_one_el_ucc =
            all_variants_default_option_some_vec_one_el_ucc();
        quote! {
            #postgres_crud_common
            #all_variants_default_option_some_vec_one_el_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneElCall;
impl quote::ToTokens for PostgresCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneElCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_common_all_variants_default_option_some_vec_one_el =
            PostgresCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneEl;
        let path_all_variants_default_option_some_vec_one_el_call =
            path_all_variants_default_option_some_vec_one_el_call();
        quote! {
            #postgres_crud_common_all_variants_default_option_some_vec_one_el
            #path_all_variants_default_option_some_vec_one_el_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateDefaultOptionSomeVecOneElMaxPageSize;
impl quote::ToTokens for CrateDefaultOptionSomeVecOneElMaxPageSize {
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
impl quote::ToTokens for CrateDefaultOptionSomeVecOneElMaxPageSizeCall {
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
pub struct PostgresCrudDefaultOptionSomeVecOneElMaxPageSize;
impl quote::ToTokens for PostgresCrudDefaultOptionSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud = postgres_crud();
        let default_option_some_vec_one_el_max_page_size_ucc =
            default_option_some_vec_one_el_max_page_size_ucc();
        quote! {
            #postgres_crud
            #default_option_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudDefaultOptionSomeVecOneElMaxPageSizeCall;
impl quote::ToTokens for PostgresCrudDefaultOptionSomeVecOneElMaxPageSizeCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_default_option_some_vec_one_el_max_page_size =
            PostgresCrudDefaultOptionSomeVecOneElMaxPageSize;
        let path_default_option_some_vec_one_el_max_page_size_call =
            path_default_option_some_vec_one_el_max_page_size_call();
        quote! {
            #postgres_crud_default_option_some_vec_one_el_max_page_size
            #path_default_option_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudCommonDefaultOptionSomeVecOneElMaxPageSize;
impl quote::ToTokens for PostgresCrudCommonDefaultOptionSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_common = postgres_crud_common();
        let default_option_some_vec_one_el_max_page_size_ucc =
            default_option_some_vec_one_el_max_page_size_ucc();
        quote! {
            #postgres_crud_common
            #default_option_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudCommonDefaultOptionSomeVecOneElMaxPageSizeCall;
impl quote::ToTokens for PostgresCrudCommonDefaultOptionSomeVecOneElMaxPageSizeCall {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_common_default_option_some_vec_one_el_max_page_size =
            PostgresCrudCommonDefaultOptionSomeVecOneElMaxPageSize;
        let path_default_option_some_vec_one_el_max_page_size_call =
            path_default_option_some_vec_one_el_max_page_size_call();
        quote! {
            #postgres_crud_common_default_option_some_vec_one_el_max_page_size
            #path_default_option_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize;
impl quote::ToTokens for CrateAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_path_ts = crate_path_ts();
        let all_variants_default_option_some_vec_one_el_max_page_size_ucc =
            all_variants_default_option_some_vec_one_el_max_page_size_ucc();
        quote! {
            #crate_path_ts
            #all_variants_default_option_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CrateAllEnumVariantsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize;
impl quote::ToTokens for CrateAllEnumVariantsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let crate_all_variants_default_option_some_vec_one_el_max_page_size =
            CrateAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize;
        let path_all_variants_default_option_some_vec_one_el_max_page_size_call =
            path_all_variants_default_option_some_vec_one_el_max_page_size_call();
        quote! {
            #crate_all_variants_default_option_some_vec_one_el_max_page_size
            #path_all_variants_default_option_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize;
impl quote::ToTokens for PostgresCrudAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize {
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud = postgres_crud();
        let all_variants_default_option_some_vec_one_el_max_page_size_ucc =
            all_variants_default_option_some_vec_one_el_max_page_size_ucc();
        quote! {
            #postgres_crud
            #all_variants_default_option_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudAllEnumVariantsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize;
impl quote::ToTokens
    for PostgresCrudAllEnumVariantsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize
{
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_all_variants_default_option_some_vec_one_el_max_page_size =
            PostgresCrudAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize;
        let path_all_variants_default_option_some_vec_one_el_max_page_size_call =
            path_all_variants_default_option_some_vec_one_el_max_page_size_call();
        quote! {
            #postgres_crud_all_variants_default_option_some_vec_one_el_max_page_size
            #path_all_variants_default_option_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize;
impl quote::ToTokens
    for PostgresCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize
{
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_common = postgres_crud_common();
        let all_variants_default_option_some_vec_one_el_max_page_size_ucc =
            all_variants_default_option_some_vec_one_el_max_page_size_ucc();
        quote! {
            #postgres_crud_common
            #all_variants_default_option_some_vec_one_el_max_page_size_ucc
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PostgresCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize;
impl quote::ToTokens
    for PostgresCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneElCallWithMaxPageSize
{
    fn to_tokens(&self, tokens: &mut Ts2) {
        let postgres_crud_common_all_variants_default_option_some_vec_one_el_max_page_size =
            PostgresCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize;
        let path_all_variants_default_option_some_vec_one_el_max_page_size_call =
            path_all_variants_default_option_some_vec_one_el_max_page_size_call();
        quote! {
            #postgres_crud_common_all_variants_default_option_some_vec_one_el_max_page_size
            #path_all_variants_default_option_some_vec_one_el_max_page_size_call
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MustUse;
impl quote::ToTokens for MustUse {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[must_use]}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AllowClippyArbitrarySourceItemOrdering;
impl quote::ToTokens for AllowClippyArbitrarySourceItemOrdering {
    fn to_tokens(&self, tokens: &mut Ts2) {
        quote! {#[allow(clippy::arbitrary_source_item_ordering)]}.to_tokens(tokens);
    }
}
fn path_all_variants_default_option_some_vec_one_el_max_page_size_call() -> Ts2 {
    quote! {::all_variants_default_option_some_vec_one_el_max_page_size()}
}
fn default_option_some_vec_one_el_max_page_size_ucc() -> Ts2 {
    quote! {DefaultOptionSomeVecOneElMaxPageSize}
}
fn crate_path_ts() -> Ts2 {
    quote! {crate::}
}
fn postgres_crud() -> Ts2 {
    quote! {postgres_crud::}
}
fn postgres_crud_common() -> Ts2 {
    quote! {postgres_crud_common::}
}
fn default_option_some_vec_one_el_ucc() -> Ts2 {
    quote! {DefaultOptionSomeVecOneEl}
}
fn all_variants_default_option_some_vec_one_el_ucc() -> Ts2 {
    quote! {AllEnumVariantsArrayDefaultOptionSomeVecOneEl}
}
fn path_default_option_some_vec_one_el_call() -> Ts2 {
    quote! {::default_option_some_vec_one_el()}
}
fn path_default_option_some_vec_one_el_max_page_size_call() -> Ts2 {
    quote! {::default_option_some_vec_one_el_max_page_size()}
}
fn all_variants_default_option_some_vec_one_el_max_page_size_ucc() -> Ts2 {
    quote! {AllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize}
}
fn path_all_variants_default_option_some_vec_one_el_call() -> Ts2 {
    quote! {::all_variants_default_option_some_vec_one_el()}
}
