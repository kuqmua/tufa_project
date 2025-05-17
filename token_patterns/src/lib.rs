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
pub struct RefStdPrimitiveStr;
impl quote::ToTokens for RefStdPrimitiveStr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {&std::primitive::str}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdStringString;
impl quote::ToTokens for StdStringString {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::string::String}.to_tokens(tokens);
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
        quote::quote! {#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]}.to_tokens(tokens);
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
        quote::quote! {#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]}.to_tokens(tokens);
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
        quote::quote! {#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]}.to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveStrSqlxColumnIndex;
impl quote::ToTokens for StdPrimitiveStrSqlxColumnIndex {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {&'a std::primitive::str: sqlx::ColumnIndex<R>,}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SqlxDecodeDecodeDatabase;
impl quote::ToTokens for SqlxDecodeDecodeDatabase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {sqlx::decode::Decode<'a, R::Database>}.to_tokens(tokens);
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
impl quote::ToTokens for CodeOccurenceSnakeCaseDoubleDotSpaceErrorOccurenceLibCodeOccurenceCodeOccurence {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {code_occurence: error_occurence_lib::code_occurence::CodeOccurence}.to_tokens(tokens);
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
        quote::quote! {std::primitive::bool}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveU8;
impl quote::ToTokens for StdPrimitiveU8 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::primitive::u8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveIU6;
impl quote::ToTokens for StdPrimitiveIU6 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::primitive::u16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveU32;
impl quote::ToTokens for StdPrimitiveU32 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::primitive::u32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveU64;
impl quote::ToTokens for StdPrimitiveU64 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::primitive::u64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI8;
impl quote::ToTokens for StdPrimitiveI8 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::primitive::i8}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI16;
impl quote::ToTokens for StdPrimitiveI16 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::primitive::i16}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI32;
impl quote::ToTokens for StdPrimitiveI32 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::primitive::i32}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdPrimitiveI64;
impl quote::ToTokens for StdPrimitiveI64 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::primitive::i64}.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StdFmtDisplay;
impl quote::ToTokens for StdFmtDisplay {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::quote! {std::fmt::Display}.to_tokens(tokens);
    }
}

fn crate_path_token_stream() -> proc_macro2::TokenStream {
    quote::quote! {crate::}
}
fn postgresql_crud() -> proc_macro2::TokenStream {
    quote::quote! {postgresql_crud::}
}
fn default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case() -> proc_macro2::TokenStream {
    quote::quote! {DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement}
}
fn all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case() -> proc_macro2::TokenStream {
    quote::quote! {AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement}
}
fn path_default_but_option_is_always_some_and_vec_always_contains_one_element_call() -> proc_macro2::TokenStream {
    quote::quote! {::default_but_option_is_always_some_and_vec_always_contains_one_element()}
}
fn path_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call() -> proc_macro2::TokenStream {
    quote::quote! {::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()}
}
pub struct CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
impl quote::ToTokens for CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_path_token_stream = crate_path_token_stream();
        let default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case = default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case();
        quote::quote! {
            #crate_path_token_stream
            #default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
pub struct CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
impl quote::ToTokens for CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_path_token_stream = crate_path_token_stream();
        let default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case = default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case();
        let path_default_but_option_is_always_some_and_vec_always_contains_one_element_call = path_default_but_option_is_always_some_and_vec_always_contains_one_element_call();
        quote::quote! {
            #crate_path_token_stream
            #default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case
            #path_default_but_option_is_always_some_and_vec_always_contains_one_element_call
        }
        .to_tokens(tokens);
    }
}
pub struct PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
impl quote::ToTokens for PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud = postgresql_crud();
        let default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case = default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case();
        quote::quote! {
            #postgresql_crud
            #default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
pub struct PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
impl quote::ToTokens for PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element = PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
        let path_default_but_option_is_always_some_and_vec_always_contains_one_element_call = path_default_but_option_is_always_some_and_vec_always_contains_one_element_call();
        quote::quote! {
            #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element
            #path_default_but_option_is_always_some_and_vec_always_contains_one_element_call
        }
        .to_tokens(tokens);
    }
}
pub struct CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
impl quote::ToTokens for CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_path_token_stream = crate_path_token_stream();
        let all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case = all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case();
        quote::quote! {
            #crate_path_token_stream
            #all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
pub struct CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
impl quote::ToTokens for CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element = CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
        let path_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call = path_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call();
        quote::quote! {
            #crate_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element
            #path_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call
        }
        .to_tokens(tokens);
    }
}
pub struct PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
impl quote::ToTokens for PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud = postgresql_crud();
        let all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case = all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case();
        quote::quote! {
            #postgresql_crud
            #all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case
        }
        .to_tokens(tokens);
    }
}
pub struct PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
impl quote::ToTokens for PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element = PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
        let path_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call = path_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call();
        quote::quote! {
            #postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element
            #path_all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call
        }
        .to_tokens(tokens);
    }
}
