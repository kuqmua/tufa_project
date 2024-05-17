naming_macros::generate_upper_camel_and_snake_case_stringified_and_token_stream_from_naming_constants!(
    [
        ["primary", "key"],
        ["serialize", "deserialize"],
        ["with", "serialize", "deserialize"],
        ["error", "occurence"],
        ["code", "occurence"],
        ["is", "none"],
        ["error", "named"],
        ["try", "from"],
        ["from", "str"],
        ["column", "not", "found"],
        ["request", "error"],
        ["row", "not", "found"],
        ["type", "not", "found"],
        ["column", "index", "out", "of", "bounds"],
        ["column", "decode"],
        ["pool", "timed", "out"],
        ["pool", "closed"],
        ["worker", "crashed"],
        ["json", "data", "error"],
        ["json", "syntax", "error"],
        ["missing", "json", "content", "type"],
        ["bytes", "rejection"],
        ["unexpected", "case"],
        ["expected", "type"],
        ["unexpected", "status", "code"],
        ["failed", "to", "get", "response", "text"],
        ["deserialize", "response"],
        ["status", "code"],
        ["response", "text"],
        ["response", "text", "result"],
        ["std", "option", "option"],
        ["wrapper", "vec", "column"],
        ["order", "by"],
        ["primary", "keys"],
        ["not", "unique", "primary", "keys"],
        ["response", "variants"],
        ["tvfrr", "extraction", "logic"],
        ["bind", "query"],
        ["into", "serialize", "deserialize", "version"],
        ["checked", "add"],
        ["query", "and", "rollback", "failed"],
        ["primary", "key", "from", "row", "and", "failed", "rollback"],
        ["non", "existing", "primary", "keys"],
        ["non", "existing", "primary", "keys", "and", "failed", "rollback"]
    ]
);

naming_macros::generate_self_upper_camel_and_snake_case_stringified_and_token_stream_from_naming_constants!(
    [
        ["self", "parameters"],
        ["self", "payload"],
        ["self", "payload", "with", "serialize", "deserialize"],
        ["self", "payload", "try", "from", "self", "payload", "with", "serialize", "deserialize"],
        ["self", "payload", "with", "serialize", "deserialize", "try", "from", "self", "payload"],
        ["self", "payload", "with", "serialize", "deserialize", "try", "from", "self", "payload", "error", "named"],
        ["try", "self"],
        ["try", "self", "response", "variants"],
        ["self", "payload", "element", "with", "serialize", "deserialize"],
        ["self", "payload", "element"],
        ["self", "payload", "element", "try", "from", "self", "payload", "with", "serialize", "deserialize"],
        ["self", "payload", "element", "try", "from", "self", "payload", "element", "with", "serialize", "deserialize", "error", "named"],
        ["try", "self", "error", "named"],
        ["try", "self", "request", "error"],
        ["self", "payload", "try", "from", "self", "payload", "with", "serialize", "deserialize", "error", "named"],
        ["try", "self", "with", "serialize", "deserialize"],
        ["tvfrr", "extraction", "logic", "try", "self"],
        ["try", "self", "generated", "route", "logic", "error", "named"],
        ["try", "self", "generated", "route", "logic", "desirable"],
        ["try", "self", "route", "logic"],
        ["try", "self", "route", "logic", "response"],
        ["try", "self", "route", "logic", "response", "variants"],
        ["try", "self", "route", "logic", "error", "named"],
        ["try", "self", "route", "logic", "error", "named", "with", "serialize", "deserialize"],
        ["try", "self", "generated", "route", "logic", "error", "named", "with", "serialize", "deserialize"]
    ]
);

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep> {
        let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
        handle.push_value(
            syn::PathSegment {
                ident: proc_macro2::Ident::new(
                    &format!(
                        "{}{}{}",
                        self.self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(),
                        <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
                        <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
                    ),
                    proc_macro2::Span::call_site()
                ),
                arguments: syn::PathArguments::None,
            }
        );
        handle
    }
}

pub trait TrySelfResponseVariantsStatusCodeStringified {
    fn try_self_response_variants_status_code_stringified(
        &self,
        status_code: &crate::status_code::StatusCode,
    ) -> std::string::String;
}

impl<T> TrySelfResponseVariantsStatusCodeStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_response_variants_status_code_stringified(
        &self,
        status_code: &crate::status_code::StatusCode,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Response as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Variants as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(status_code),
        )
    }
}

pub trait TrySelfResponseVariantsStatusCodeTokenStream {
    fn try_self_response_variants_status_code_token_stream(
        &self,
        status_code: &crate::status_code::StatusCode,
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfResponseVariantsStatusCodeTokenStream for T
where
    T: TrySelfResponseVariantsStatusCodeStringified,
{
    fn try_self_response_variants_status_code_token_stream(
        &self,
        status_code: &crate::status_code::StatusCode,
    ) -> proc_macro2::TokenStream {
        let value =
            self.try_self_response_variants_status_code_stringified(status_code);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfSnakeCasePrintlnStringified {
    fn try_self_snake_case_println_stringified(
        &self,
        test_operation_print_in_info: &crate::TestOperationPrintlnInfo,
    ) -> std::string::String;
}

impl<T> TrySelfSnakeCasePrintlnStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn try_self_snake_case_println_stringified(
        &self,
        test_operation_print_in_info: &crate::TestOperationPrintlnInfo,
    ) -> std::string::String {
        let slashes = "-------";
        format!(
            "\"{}{}{} {}{}\"",
            slashes,
            <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
            self.to_snake_case_stringified(),
            proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(test_operation_print_in_info),
            slashes,
        )
    }
}

pub trait TrySelfSnakeCasePrintlnTokenStream {
    fn try_self_snake_case_println_token_stream(
        &self,
        test_operation_print_in_info: &crate::TestOperationPrintlnInfo,
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfSnakeCasePrintlnTokenStream for T
where
    T: TrySelfSnakeCasePrintlnStringified,
{
    fn try_self_snake_case_println_token_stream(
        &self,
        test_operation_print_in_info: &crate::TestOperationPrintlnInfo,
    ) -> proc_macro2::TokenStream {
        let value =
            self.try_self_snake_case_println_stringified(test_operation_print_in_info);
        let value_token_stream = value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
        quote::quote! {println!(#value_token_stream);}
    }
}

pub trait WrapIntoStartEndPrintlnSelfTokenStream {
    fn wrap_into_start_end_println_self_token_stream(
        &self,
        test_content_token_stream: &proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream;
}

impl<T> WrapIntoStartEndPrintlnSelfTokenStream for T
where
    T: TrySelfSnakeCasePrintlnTokenStream,
{
    fn wrap_into_start_end_println_self_token_stream(
        &self,
        test_content_token_stream: &proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let start_println_token_stream =
            self.try_self_snake_case_println_token_stream(&crate::TestOperationPrintlnInfo::Start);
        let end_println_token_stream =
            self.try_self_snake_case_println_token_stream(&crate::TestOperationPrintlnInfo::End);
        quote::quote! {
            #start_println_token_stream
            #test_content_token_stream
            #end_println_token_stream
        }
    }
}

pub trait SwaggerUrlPathSelfQuotesStringified {
    fn swagger_url_path_self_quotes_stringified(
        &self,
        table_name_stringified: &str,
    ) -> std::string::String;
}

impl<T> SwaggerUrlPathSelfQuotesStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn swagger_url_path_self_quotes_stringified(
        &self,
        table_name_stringified: &str,
    ) -> std::string::String {
        proc_macro_common::generate_quotes::stringified(&format!(
            "/{}/{}",
            table_name_stringified,
            self.to_snake_case_stringified(),
        ))
    }
}

pub trait SwaggerUrlPathSelfQuotesTokenStream {
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        table_name_stringified: &str,
    ) -> proc_macro2::TokenStream;
}

impl<T> SwaggerUrlPathSelfQuotesTokenStream for T
where
    T: SwaggerUrlPathSelfQuotesStringified,
{
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        table_name_stringified: &str,
    ) -> proc_macro2::TokenStream {
        let value =
            self.swagger_url_path_self_quotes_stringified(table_name_stringified);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait UrlHandleSelfSnakeCaseStringified {
    fn url_handle_self_snake_case_stringified(
        &self,
        table_name_stringified: &str,
    ) -> std::string::String;
}

impl<T> UrlHandleSelfSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn url_handle_self_snake_case_stringified(
        &self,
        table_name_stringified: &str,
    ) -> std::string::String {
        format!(
            "\"{{}}/{}/{}\"",
            table_name_stringified,
            self.to_snake_case_stringified()
        )
    }
}

pub trait UrlHandleSelfSnakeCaseTokenStream {
    fn url_handle_self_snake_case_token_stream(
        &self,
        table_name_stringified: &str,
    ) -> proc_macro2::TokenStream;
}

impl<T> UrlHandleSelfSnakeCaseTokenStream for T
where
    T: UrlHandleSelfSnakeCaseStringified,
{
    fn url_handle_self_snake_case_token_stream(
        &self,
        table_name_stringified: &str,
    ) -> proc_macro2::TokenStream {
        let value = self.url_handle_self_snake_case_stringified(table_name_stringified);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}
// fn generate_url_handle_token_stream(
//     table_name_stringified: &str,
//     operation_name_snake_case_stringified: &str,
//     proc_macro_name_upper_camel_case_ident_stringified: &str,
// ) -> proc_macro2::TokenStream {
//     let url_handle_stringified = format!("\"{{}}/{table_name_stringified}/{operation_name_snake_case_stringified}\"");
//     url_handle_stringified.parse::<proc_macro2::TokenStream>()
//     .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {url_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
// }