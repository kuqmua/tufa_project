pub mod code_occurence_syn_field;
pub mod construct_syn_variant;
pub mod enum_variants;
pub mod error_occurence;
pub mod generate_field_code_occurence_new_token_stream;
pub mod generate_simple_syn_punctuated_punctuated;
pub mod get_macro_attribute;
// pub mod naming_conventions;
pub mod status_code;
pub mod type_variants_from_request_response;
pub mod wrap_derive;
pub mod write_token_stream_into_file;
pub mod generate_impl_std_convert_from_token_stream;

// impl OperationHttpMethod {
//     fn to_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value_snake_case_stringified = proc_macro_helpers::naming_conventions::ToSnakeCase::to_snake_case(self);
//         value_snake_case_stringified.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("{value_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }

// trait ParametersUpperCamelCaseTokenStream {
//     fn parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
// }

// impl<Generic> ParametersUpperCamelCaseTokenStream for Generic
//     where Generic: proc_macro_helpers::naming_conventions::ToUpperCamelCase
// {
//     fn parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = format!(
//             "{}Parameters",
//             self.to_upper_camel_case()
//         );
//         value.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }

#[derive(Debug, Clone, Copy, proc_macro_assistants::ToSnakeCaseStringified)]
pub enum TestOperationPrintlnInfo {
    Start,
    End,
}

// impl std::fmt::Display for TestOperationPrintlnInfo {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             Self::Start => write!(f, "start"),
//             Self::End => write!(f, "end")
//         }
//     }
// }

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
            naming_constants::TrySnakeCase,
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
            self.try_self_snake_case_println_token_stream(&TestOperationPrintlnInfo::Start);
        let end_println_token_stream =
            self.try_self_snake_case_println_token_stream(&TestOperationPrintlnInfo::End);
        quote::quote! {
            #start_println_token_stream
            #test_content_token_stream
            #end_println_token_stream
        }
    }
}
