#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

pub mod error_occurence;
pub mod status_code;
pub mod type_variants_from_request_response;
pub mod write_token_stream_into_file;
pub mod get_macro_attribute;
pub mod naming_conventions;

// impl OperationHttpMethod {
//     fn to_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value_snake_case_stringified = proc_macro_helpers::naming_conventions::ToSnakeCase::to_snake_case(self);
//         value_snake_case_stringified.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("{value_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
//         .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }

#[derive(
    proc_macro_assistants::ToSnakeCaseStringified,
)]
pub enum TestOperationPrintlnInfo {
    Start,
    End
}

// impl std::fmt::Display for TestOperationPrintlnInfo {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             Self::Start => write!(f, "start"),
//             Self::End => write!(f, "end")
//         }
//     }
// }