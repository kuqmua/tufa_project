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
        ["primary", "key", "from", "row", "and", "failed", "rollback"]
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
        ["self", "payload", "try", "from", "self", "payload", "with", "serialize", "deserialize", "error", "named"]
        // ["", "", "", "", "", "", "", "", "", ""],
        // ["", "", "", "", "", "", "", "", "", ""],
        // ["", "", "", "", "", "", "", "", "", ""],
        // ["", "", "", "", "", "", "", "", "", ""],
        // ["", "", "", "", "", "", "", "", "", ""],
        // ["", "", "", "", "", "", "", "", "", ""],
        // ["", "", "", "", "", "", "", "", "", ""],
        // ["", "", "", "", "", "", "", "", "", ""],
        // ["", "", "", "", "", "", "", "", "", ""],
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

pub trait PayloadWithSerializeDeserializeTryFromPayloadErrorNamedUpperCamelCaseStringified {
    fn payload_with_serialize_deserialize_try_from_payload_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> PayloadWithSerializeDeserializeTryFromPayloadErrorNamedUpperCamelCaseStringified for T
where
    T: SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseStringified,
{
    fn payload_with_serialize_deserialize_try_from_payload_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}",
            self.self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_stringified(),
            <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait PayloadWithSerializeDeserializeTryFromPayloadErrorNamedUpperCamelCaseTokenStream {
    fn payload_with_serialize_deserialize_try_from_payload_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> PayloadWithSerializeDeserializeTryFromPayloadErrorNamedUpperCamelCaseTokenStream for T
where
    T: PayloadWithSerializeDeserializeTryFromPayloadErrorNamedUpperCamelCaseStringified,
{
    fn payload_with_serialize_deserialize_try_from_payload_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = self.payload_with_serialize_deserialize_try_from_payload_error_named_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfWithSerializeDeserializeUpperCamelCaseStringified {
    fn try_self_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> TrySelfWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::With as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Serialize as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Deserialize as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn try_self_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: TrySelfWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn try_self_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value =
            self.try_self_with_serialize_deserialize_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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

pub trait TrySelfWithSerializeDeserializeStringified {
    fn try_self_with_serialize_deserialize_stringified(&self) -> std::string::String;
}

impl<T> TrySelfWithSerializeDeserializeStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_with_serialize_deserialize_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::With as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Serialize as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Deserialize as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfWithSerializeDeserializeTokenStream {
    fn try_self_with_serialize_deserialize_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfWithSerializeDeserializeTokenStream for T
where
    T: TrySelfWithSerializeDeserializeStringified,
{
    fn try_self_with_serialize_deserialize_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_with_serialize_deserialize_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TvfrrExtractionLogicTrySelfSnakeCaseStringified {
    fn tvfrr_extraction_logic_try_self_snake_case_stringified(&self) -> std::string::String;
}

impl<T> TvfrrExtractionLogicTrySelfSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn tvfrr_extraction_logic_try_self_snake_case_stringified(&self) -> std::string::String {
        format!(
            "{}_{}_{}",
            tvfrr_extraction_logic_snake_case_stringified(),
            <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
            self.to_snake_case_stringified()
        )
    }
}

pub trait TvfrrExtractionLogicTrySelfSnakeCaseTokenStream {
    fn tvfrr_extraction_logic_try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TvfrrExtractionLogicTrySelfSnakeCaseTokenStream for T
where
    T: TvfrrExtractionLogicTrySelfSnakeCaseStringified,
{
    fn tvfrr_extraction_logic_try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.tvfrr_extraction_logic_try_self_snake_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

//
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

//
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
//

pub trait TrySelfGeneratedRouteLogicErrorNamedUpperCamelCaseStringified {
    fn try_self_generated_route_logic_error_named_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfGeneratedRouteLogicErrorNamedUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_generated_route_logic_error_named_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Generated as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Route as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Logic as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfGeneratedRouteLogicErrorNamedUpperCamelCaseTokenStream {
    fn try_self_generated_route_logic_error_named_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfGeneratedRouteLogicErrorNamedUpperCamelCaseTokenStream for T
where
    T: TrySelfGeneratedRouteLogicErrorNamedUpperCamelCaseStringified,
{
    fn try_self_generated_route_logic_error_named_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_generated_route_logic_error_named_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfGeneratedRouteLogicDesirableUpperCamelCaseStringified {
    fn try_self_generated_route_logic_desirable_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfGeneratedRouteLogicDesirableUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_generated_route_logic_desirable_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Generated as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Route as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Logic as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Desirable as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfGeneratedRouteLogicDesirableUpperCamelCaseTokenStream {
    fn try_self_generated_route_logic_desirable_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfGeneratedRouteLogicDesirableUpperCamelCaseTokenStream for T
where
    T: TrySelfGeneratedRouteLogicDesirableUpperCamelCaseStringified,
{
    fn try_self_generated_route_logic_desirable_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_generated_route_logic_desirable_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfRouteLogicSnakeCaseStringified {
    fn try_self_route_logic_snake_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfRouteLogicSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn try_self_route_logic_snake_case_stringified(&self) -> std::string::String {
        format!(
            "{}_{}_{}_{}",
            <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
            self.to_snake_case_stringified(),
            <naming_constants::Route as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Logic as naming_constants::Naming>::snake_case_stringified(),
        )
    }
}

pub trait TrySelfRouteLogicSnakeCaseTokenStream {
    fn try_self_route_logic_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfRouteLogicSnakeCaseTokenStream for T
where
    T: TrySelfRouteLogicSnakeCaseStringified,
{
    fn try_self_route_logic_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_route_logic_snake_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfRouteLogicResponseUpperCamelCaseStringified {
    fn try_self_route_logic_response_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfRouteLogicResponseUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_route_logic_response_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Route as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Logic as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Response as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfRouteLogicResponseUpperCamelCaseTokenStream {
    fn try_self_route_logic_response_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfRouteLogicResponseUpperCamelCaseTokenStream for T
where
    T: TrySelfRouteLogicResponseUpperCamelCaseStringified,
{
    fn try_self_route_logic_response_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_route_logic_response_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfRouteLogicResponseVariantsUpperCamelCaseStringified {
    fn try_self_route_logic_response_variants_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfRouteLogicResponseVariantsUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_route_logic_response_variants_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Route as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Logic as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Response as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Variants as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream {
    fn try_self_route_logic_response_variants_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream for T
where
    T: TrySelfRouteLogicResponseVariantsUpperCamelCaseStringified,
{
    fn try_self_route_logic_response_variants_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_route_logic_response_variants_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfRouteLogicErrorNamedUpperCamelCaseStringified {
    fn try_self_route_logic_error_named_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfRouteLogicErrorNamedUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_route_logic_error_named_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Route as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Logic as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfRouteLogicErrorNamedUpperCamelCaseTokenStream {
    fn try_self_route_logic_error_named_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfRouteLogicErrorNamedUpperCamelCaseTokenStream for T
where
    T: TrySelfRouteLogicErrorNamedUpperCamelCaseStringified,
{
    fn try_self_route_logic_error_named_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_route_logic_error_named_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseStringified {
    fn try_self_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Route as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Logic as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::With as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Serialize as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Deserialize as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn try_self_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn try_self_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCaseStringified {
    fn try_self_route_logic_error_named_with_serialize_deserialize_snake_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn try_self_route_logic_error_named_with_serialize_deserialize_snake_case_stringified(&self) -> std::string::String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}_{}_{}",
            <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
            self.to_snake_case_stringified(),
            <naming_constants::Route as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Logic as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Error as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Named as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::With as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Serialize as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Deserialize as naming_constants::Naming>::snake_case_stringified(),
        )
    }
}

pub trait TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCaseTokenStream {
    fn try_self_route_logic_error_named_with_serialize_deserialize_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCaseTokenStream for T
where
    T: TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCaseStringified,
{
    fn try_self_route_logic_error_named_with_serialize_deserialize_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_route_logic_error_named_with_serialize_deserialize_snake_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfGeneratedRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseStringified {
    fn try_self_generated_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfGeneratedRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_generated_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Generated as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Route as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Logic as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::With as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Serialize as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Deserialize as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfGeneratedRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn try_self_generated_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfGeneratedRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: TrySelfGeneratedRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn try_self_generated_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_generated_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}