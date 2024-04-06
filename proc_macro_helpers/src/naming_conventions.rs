// todo maybe use struct like struct Parameters<'a>(&'a str) and impl ToUpperCamelCaseString for it ?
// todo maybe reqwrite it this way
// trait InnerValue<'a> {
//     fn inner_value() -> &'a str;
// }

// pub struct Named;
// impl<'a> InnerValue<'a> for Named {
//     fn inner_value() -> &'a str {
//         "named"
//     }
// }
// impl ToUpperCamelCaseString for Named {
//     fn to_upper_camel_case_string(&self) -> std::string::String {
//         convert_case::Casing::to_case(&Self::inner_value(), convert_case::Case::UpperCamel)
//     }
// }
pub fn named_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::NAMED)
}
pub fn named_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::NAMED,
    )
}
pub fn unnamed_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::UNNAMED)
}
pub fn unnamed_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::UNNAMED,
    )
}
pub fn error_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::ERROR)
}
pub fn error_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::ERROR,
    )
}
pub fn occurence_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::OCCURENCE)
}
pub fn occurence_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::OCCURENCE,
    )
}
pub fn string_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::STRING)
}
pub fn string_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::STRING,
    )
}
pub fn parameters_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::PARAMETERS)
}
pub fn parameters_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::PARAMETERS,
    )
}
pub fn parameters_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = parameters_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn payload_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::PAYLOAD)
}
pub fn payload_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::PAYLOAD,
    )
}
pub fn payload_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = payload_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn element_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::ELEMENT)
}
pub fn element_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::ELEMENT,
    )
}
pub fn element_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = element_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn try_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::TRY)
}
pub fn try_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::TRY,
    )
}
pub fn from_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::FROM)
}
pub fn from_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::FROM,
    )
}
pub fn from_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = from_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn response_variants_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::RESPONSE_VARIANTS)
}
// pub fn response_variants_snake_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&naming_constants::RESPONSE_VARIANTS)
// }
pub fn path_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::PATH)
}
pub fn path_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::PATH,
    )
}
pub fn path_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = path_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn key_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::KEY)
}
pub fn key_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::KEY,
    )
}
pub fn value_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::VALUE)
}
pub fn value_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::VALUE,
    )
}
pub fn vec_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::VEC)
}
pub fn vec_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::VEC,
    )
}
pub fn hashmap_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::HASHMAP)
}
pub fn hashmap_snake_case_stringified() -> std::string::String {
    //naming exception -
    std::string::String::from("hashmap")
}
pub fn reference_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::REFERENCE)
}
pub fn reference_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::REFERENCE,
    )
}
pub fn with_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::WITH)
}
pub fn with_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::WITH,
    )
}
pub fn serialize_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::SERIALIZE)
}
pub fn serialize_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::SERIALIZE,
    )
}
pub fn deserialize_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::DESERIALIZE)
}
pub fn deserialize_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::DESERIALIZE,
    )
}
pub fn request_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::REQUEST)
}
pub fn request_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::REQUEST,
    )
}
pub fn response_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::RESPONSE)
}
pub fn response_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::RESPONSE,
    )
}
pub fn variants_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::VARIANTS)
}
pub fn variants_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::VARIANTS,
    )
}
// pub fn tvfrr_extraction_logic_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::TVFRR_EXTRACTION_LOGIC)
// }
pub fn tvfrr_extraction_logic_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::TVFRR_EXTRACTION_LOGIC,
    )
}
pub fn options_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::OPTIONS)
}
// pub fn options_snake_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&naming_constants::OPTIONS)
// }
pub fn code_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::CODE)
}
pub fn code_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::CODE,
    )
}
pub fn config_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::CONFIG)
}
pub fn config_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::CONFIG,
    )
}
pub fn is_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::IS)
}
pub fn is_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::IS,
    )
}
pub fn none_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::NONE)
}
pub fn none_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::NONE,
    )
}
pub fn str_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::STR)
}
pub fn str_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::STR,
    )
}
pub fn uuid_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::UUID)
}
pub fn uuid_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::UUID,
    )
}
pub fn wrapper_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::WRAPPER)
}
pub fn wrapper_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::WRAPPER,
    )
}
pub fn possible_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::POSSIBLE)
}
pub fn possible_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::POSSIBLE,
    )
}
pub fn source_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::SOURCE)
}
pub fn source_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::SOURCE,
    )
}
pub fn display_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::DISPLAY)
}
pub fn display_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::DISPLAY,
    )
}
pub fn foreign_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::FOREIGN)
}
pub fn foreign_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::FOREIGN,
    )
}
pub fn type_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::TYPE)
}
pub fn type_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::TYPE,
    )
}
pub fn into_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::INTO)
}
pub fn into_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::INTO,
    )
}
pub fn get_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::GET)
}
pub fn get_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::GET,
    )
}
pub fn column_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::COLUMN)
}
pub fn column_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::COLUMN,
    )
}
pub fn column_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = column_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn select_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::SELECT)
}
pub fn select_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::SELECT,
    )
}
pub fn select_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = select_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn order_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::ORDER)
}
pub fn order_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::ORDER,
    )
}
pub fn order_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = order_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn by_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::BY)
}
pub fn by_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::BY,
    )
}
pub fn not_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::NOT)
}
pub fn not_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::NOT,
    )
}
pub fn found_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::FOUND)
}
pub fn found_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::FOUND,
    )
}
pub fn desirable_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::DESIRABLE)
}
pub fn desirable_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::DESIRABLE,
    )
}
pub fn desirable_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = desirable_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn desirable_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = desirable_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
// pub fn rollback_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::ROLLBACK)
// }
pub fn rollback_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::ROLLBACK,
    )
}
pub fn rollback_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = rollback_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
// pub fn limit_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::LIMIT)
// }
pub fn limit_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::LIMIT,
    )
}
pub fn limit_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = limit_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
// pub fn offset_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::OFFSET)
// }
pub fn offset_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::OFFSET,
    )
}
pub fn offset_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = offset_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn in_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::IN)
}
pub fn in_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::IN,
    )
}
pub fn client_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::CLIENT)
}
pub fn client_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::CLIENT,
    )
}
pub fn server_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::SERVER)
}
pub fn server_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::SERVER,
    )
}
pub fn no_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::NO)
}
pub fn fields_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::FIELDS)
}
// pub fn commit_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::COMMIT)
// }
pub fn commit_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::COMMIT,
    )
}
pub fn commit_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = commit_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
// pub fn begin_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::BEGIN)
// }
pub fn begin_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::BEGIN,
    )
}
pub fn begin_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = begin_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
// pub fn acc_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::ACC)
// }
pub fn acc_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::ACC,
    )
}
pub fn acc_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = acc_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
// pub fn query_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::QUERY)
// }
pub fn query_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::QUERY,
    )
}
pub fn query_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = query_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
// pub fn update_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::UPDATE)
// }
pub fn update_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::UPDATE,
    )
}
// pub fn as_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::AS)
// }
pub fn as_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::AS,
    )
}
// pub fn set_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::SET)
// }
pub fn set_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::SET,
    )
}
// pub fn insert_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::INSERT)
// }
pub fn insert_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::INSERT,
    )
}
// pub fn values_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::VALUES)
// }
pub fn values_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::VALUES,
    )
}
// pub fn delete_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::DELETE)
// }
pub fn delete_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::DELETE,
    )
}
pub fn where_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::WHERE)
}
pub fn where_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::WHERE,
    )
}
// pub fn and_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::AND)
// }
pub fn and_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::AND,
    )
}
// pub fn unnest_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::UNNEST)
// }
pub fn unnest_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::UNNEST,
    )
}
pub fn configuration_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::CONFIGURATION)
}
pub fn database_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::DATABASE)
}
pub fn io_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::IO)
}
pub fn tls_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::TLS)
}
pub fn protocol_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::PROTOCOL)
}
pub fn row_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::ROW)
}
pub fn index_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::INDEX)
}
pub fn out_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::OUT)
}
pub fn of_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::OF)
}
pub fn bounds_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::BOUNDS)
}
pub fn decode_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::DECODE)
}
pub fn pool_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::POOL)
}
pub fn timed_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::TIMED)
}
pub fn closed_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::CLOSED)
}
pub fn worker_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::WORKER)
}
pub fn crashed_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::CRASHED)
}
pub fn migrate_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::MIGRATE)
}
pub fn json_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::JSON)
}
pub fn data_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::DATA)
}
pub fn syntax_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::SYNTAX)
}
pub fn missing_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::MISSING)
}
pub fn content_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::CONTENT)
}
pub fn bytes_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::BYTES)
}
pub fn rejection_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::REJECTION)
}
pub fn unexpected_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::UNEXPECTED)
}
pub fn case_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::CASE)
}
pub fn expected_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::EXPECTED)
}
pub fn expected_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::EXPECTED,
    )
}
pub fn status_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::STATUS)
}
pub fn status_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::STATUS,
    )
}
pub fn failed_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::FAILED)
}
pub fn to_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::TO)
}
pub fn text_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::TEXT)
}
pub fn text_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::TEXT,
    )
}
pub fn reqwest_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::REQWEST)
}
pub fn reqwest_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = reqwest_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn reqwest_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::REQWEST,
    )
}
pub fn reqwest_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = reqwest_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn headers_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::HEADERS,
    )
}
pub fn headers_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = headers_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn result_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::RESULT,
    )
}
pub fn serde_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::SERDE,
    )
}
pub fn serde_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = serde_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn debug_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::DEBUG)
}
pub fn debug_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = debug_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn unique_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::UNIQUE)
}
pub fn unique_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::UNIQUE,
    )
}
pub fn many_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::MANY)
}
pub fn many_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &naming_constants::MANY,
    )
}
pub fn std_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::STD)
}
pub fn option_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&naming_constants::OPTION)
}
//////////////////////////////////////////////////////////////

pub fn serialize_deserialize_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        serialize_upper_camel_case_stringified(),
        deserialize_upper_camel_case_stringified()
    )
}
pub fn serialize_deserialize_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        serialize_snake_case_stringified(),
        deserialize_snake_case_stringified()
    )
}
pub fn with_serialize_deserialize_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        with_upper_camel_case_stringified(),
        serialize_upper_camel_case_stringified(),
        deserialize_upper_camel_case_stringified()
    )
}
// pub fn with_serialize_deserialize_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
//     let value = with_serialize_deserialize_upper_camel_case_stringified();
//     value.parse::<proc_macro2::TokenStream>()
//     .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
// }
pub fn with_serialize_deserialize_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}_{}",
        with_snake_case_stringified(),
        serialize_snake_case_stringified(),
        deserialize_snake_case_stringified()
    )
}
pub fn error_occurence_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        error_upper_camel_case_stringified(),
        occurence_upper_camel_case_stringified()
    )
}
pub fn error_occurence_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        error_snake_case_stringified(),
        occurence_snake_case_stringified()
    )
}
pub fn syn_type_path_stringified() -> std::string::String {
    format!("syn::Type::{}", path_upper_camel_case_stringified())
}
pub fn supports_only_supported_container_stringified() -> std::string::String {
    format!(
        "{} {}",
        naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        naming_constants::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT
    )
}
pub fn code_occurence_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        code_upper_camel_case_stringified(),
        occurence_upper_camel_case_stringified()
    )
}
pub fn code_occurence_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = code_occurence_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn code_occurence_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        code_snake_case_stringified(),
        occurence_snake_case_stringified()
    )
}
pub fn code_occurence_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = code_occurence_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn is_none_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        is_upper_camel_case_stringified(),
        none_upper_camel_case_stringified()
    )
}
pub fn is_none_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        is_snake_case_stringified(),
        none_snake_case_stringified()
    )
}
pub fn error_named_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        error_upper_camel_case_stringified(),
        named_upper_camel_case_stringified()
    )
}
pub fn try_from_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        try_upper_camel_case_stringified(),
        from_upper_camel_case_stringified()
    )
}
pub fn try_from_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        try_snake_case_stringified(),
        from_snake_case_stringified()
    )
}
pub fn try_from_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = try_from_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn from_str_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        from_upper_camel_case_stringified(),
        str_upper_camel_case_stringified()
    )
}
pub fn from_str_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = from_str_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn from_str_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        from_snake_case_stringified(),
        str_snake_case_stringified()
    )
}
pub fn from_str_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = from_str_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn column_not_found_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        column_upper_camel_case_stringified(),
        not_upper_camel_case_stringified(),
        found_upper_camel_case_stringified(),
    )
}
pub fn request_error_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        request_upper_camel_case_stringified(),
        error_upper_camel_case_stringified()
    )
}
pub fn row_not_found_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        row_upper_camel_case_stringified(),
        not_upper_camel_case_stringified(),
        found_upper_camel_case_stringified(),
    )
}
pub fn type_not_found_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        type_upper_camel_case_stringified(),
        not_upper_camel_case_stringified(),
        found_upper_camel_case_stringified(),
    )
}
pub fn column_index_out_of_bounds_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}{}{}",
        column_upper_camel_case_stringified(),
        index_upper_camel_case_stringified(),
        out_upper_camel_case_stringified(),
        of_upper_camel_case_stringified(),
        bounds_upper_camel_case_stringified(),
    )
}
pub fn column_decode_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        column_upper_camel_case_stringified(),
        decode_upper_camel_case_stringified(),
    )
}
pub fn pool_timed_out_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        pool_upper_camel_case_stringified(),
        timed_upper_camel_case_stringified(),
        out_upper_camel_case_stringified(),
    )
}
pub fn pool_closed_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        pool_upper_camel_case_stringified(),
        closed_upper_camel_case_stringified(),
    )
}
pub fn worker_crashed_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        worker_upper_camel_case_stringified(),
        crashed_upper_camel_case_stringified(),
    )
}
pub fn json_data_error_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        json_upper_camel_case_stringified(),
        data_upper_camel_case_stringified(),
        error_upper_camel_case_stringified()
    )
}
pub fn json_syntax_error_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        json_upper_camel_case_stringified(),
        syntax_upper_camel_case_stringified(),
        error_upper_camel_case_stringified()
    )
}
pub fn missing_json_content_type_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}{}",
        missing_upper_camel_case_stringified(),
        json_upper_camel_case_stringified(),
        content_upper_camel_case_stringified(),
        type_upper_camel_case_stringified()
    )
}
pub fn bytes_rejection_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        bytes_upper_camel_case_stringified(),
        rejection_upper_camel_case_stringified()
    )
}
pub fn unexpected_case_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        unexpected_upper_camel_case_stringified(),
        case_upper_camel_case_stringified()
    )
}
pub fn expected_type_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        expected_upper_camel_case_stringified(),
        type_upper_camel_case_stringified()
    )
}
pub fn expected_type_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = expected_type_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn expected_type_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        expected_snake_case_stringified(),
        type_snake_case_stringified()
    )
}
pub fn expected_type_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = expected_type_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn unexpected_status_code_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        unexpected_upper_camel_case_stringified(),
        status_upper_camel_case_stringified(),
        code_upper_camel_case_stringified(),
    )
}
pub fn unexpected_status_code_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = unexpected_status_code_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn failed_to_get_response_text_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}{}{}",
        failed_upper_camel_case_stringified(),
        to_upper_camel_case_stringified(),
        get_upper_camel_case_stringified(),
        response_upper_camel_case_stringified(),
        text_upper_camel_case_stringified(),
    )
}
pub fn failed_to_get_response_text_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = failed_to_get_response_text_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn deserialize_response_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        deserialize_upper_camel_case_stringified(),
        response_upper_camel_case_stringified()
    )
}
pub fn deserialize_response_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = deserialize_response_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn status_code_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        status_snake_case_stringified(),
        code_snake_case_stringified()
    )
}
pub fn status_code_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = status_code_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn response_text_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        response_snake_case_stringified(),
        text_snake_case_stringified(),
    )
}
pub fn response_text_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = response_text_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn response_text_result_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}_{}",
        response_snake_case_stringified(),
        text_snake_case_stringified(),
        result_snake_case_stringified(),
    )
}
pub fn response_text_result_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = response_text_result_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn std_option_option_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        std_upper_camel_case_stringified(),
        option_upper_camel_case_stringified(),
        option_upper_camel_case_stringified(),
    )
}
// pub fn std_option_option_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
//     let value = std_option_option_upper_camel_case_stringified();
//     value.parse::<proc_macro2::TokenStream>()
//     .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
// }
/////////////////////////////////////////////////////////////////////

pub trait SelfParametersUpperCamelCaseTokenStream {
    fn self_parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfParametersUpperCamelCaseTokenStream for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}",
            self.to_upper_camel_case_stringified(),
            parameters_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadUpperCamelCaseTokenStream {
    fn self_payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadUpperCamelCaseTokenStream for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            try_upper_camel_case_stringified(),
            from_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified()
        )
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseStringified {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified(),
            try_upper_camel_case_stringified(),
            from_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseTokenStream {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseTokenStream for T
where
    T: SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseStringified {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}",
            self.to_snake_case_stringified(),
            payload_snake_case_stringified(),
            with_serialize_deserialize_snake_case_stringified(),
            try_snake_case_stringified(),
            from_snake_case_stringified(),
            self.to_snake_case_stringified(),
            payload_snake_case_stringified(),
        )
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseTokenStream {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseTokenStream for T
where
    T: SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseStringified {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified(),
            try_upper_camel_case_stringified(),
            from_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            error_named_upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseTokenStream {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseTokenStream for T
where
    T: SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2> {
        let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();
        handle.push_value(
            syn::PathSegment {
                ident: proc_macro2::Ident::new(
                    &format!(
                        "{}{}{}",
                        self.self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(),
                        error_upper_camel_case_stringified(),
                        named_upper_camel_case_stringified(),
                    ),
                    proc_macro2::Span::call_site()
                ),
                arguments: syn::PathArguments::None,
            }
        );
        handle
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseStringified {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}",
            self.to_snake_case_stringified(),
            payload_snake_case_stringified(),
            try_snake_case_stringified(),
            from_snake_case_stringified(),
            self.to_snake_case_stringified(),
            payload_snake_case_stringified(),
            with_serialize_deserialize_snake_case_stringified()
        )
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self
            .self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfSnakeCaseStringified {
    fn try_self_snake_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn try_self_snake_case_stringified(&self) -> std::string::String {
        format!(
            "{}_{}",
            try_snake_case_stringified(),
            self.to_snake_case_stringified()
        )
    }
}

pub trait TrySelfSnakeCaseTokenStream {
    fn try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfSnakeCaseTokenStream for T
where
    T: TrySelfSnakeCaseStringified,
{
    fn try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_snake_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfResponseVariantsUpperCamelCaseStringified {
    fn try_self_response_variants_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfResponseVariantsUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_response_variants_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            response_variants_upper_camel_case_stringified()
        )
    }
}

pub trait TrySelfResponseVariantsUpperCamelCaseTokenStream {
    fn try_self_response_variants_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfResponseVariantsUpperCamelCaseTokenStream for T
where
    T: TrySelfResponseVariantsUpperCamelCaseStringified,
{
    fn try_self_response_variants_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_response_variants_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfUpperCamelCaseStringified {
    fn try_self_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfUpperCamelCaseTokenStream {
    fn try_self_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfUpperCamelCaseTokenStream for T
where
    T: TrySelfUpperCamelCaseStringified,
{
    fn try_self_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified {
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            with_upper_camel_case_stringified(),
            serialize_upper_camel_case_stringified(),
            deserialize_upper_camel_case_stringified()
        )
    }
}

pub trait SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: SelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified =
            self.self_payload_element_with_serialize_deserialize_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementUpperCamelCaseStringified {
    fn self_payload_element_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> SelfPayloadElementUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadElementUpperCamelCaseTokenStream {
    fn self_payload_element_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementUpperCamelCaseTokenStream for T
where
    T: SelfPayloadElementUpperCamelCaseStringified,
{
    fn self_payload_element_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_element_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified
    for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            try_upper_camel_case_stringified(),
            from_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream
    for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseStringified {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_stringified(
        &self,
    ) -> std::string::String;
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCasePunctuated
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCasePunctuated
    for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2> {
        let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();
        handle.push_value(
            syn::PathSegment {
                ident: proc_macro2::Ident::new(
                    &format!(
                        "{}{}{}",
                        self.self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(),
                        error_upper_camel_case_stringified(),
                        named_upper_camel_case_stringified(),
                    ),
                    proc_macro2::Span::call_site()
                ),
                arguments: syn::PathArguments::None,
            }
        );
        handle
    }
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseStringified
    for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}_{}_{}",
            self.to_snake_case_stringified(),
            payload_snake_case_stringified(),
            element_snake_case_stringified(),
            try_snake_case_stringified(),
            from_snake_case_stringified(),
            self.to_snake_case_stringified(),
            payload_snake_case_stringified(),
            element_snake_case_stringified(),
            with_serialize_deserialize_snake_case_stringified(),
        )
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseTokenStream {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseTokenStream
    for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self
            .self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_stringified(
            );
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseStringified
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseStringified
    for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            try_upper_camel_case_stringified(),
            from_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified(),
            error_upper_camel_case_stringified(),
            named_upper_camel_case_stringified()
        )
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T>
    SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream
    for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfErrorNamedUpperCamelCaseTokenStream {
    fn try_self_error_named_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfErrorNamedUpperCamelCaseTokenStream for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_error_named_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = format!(
            "{}{}{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            error_upper_camel_case_stringified(),
            named_upper_camel_case_stringified()
        );
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfRequestErrorUpperCamelCaseStringified {
    fn try_self_request_error_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfRequestErrorUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_request_error_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            request_upper_camel_case_stringified(),
            error_upper_camel_case_stringified()
        )
    }
}

pub trait TrySelfRequestErrorUpperCamelCaseTokenStream {
    fn try_self_request_error_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfRequestErrorUpperCamelCaseTokenStream for T
where
    T: TrySelfRequestErrorUpperCamelCaseStringified,
{
    fn try_self_request_error_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_request_error_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseStringified {
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseStringified for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}",
            self.self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(),
            error_upper_camel_case_stringified(),
            named_upper_camel_case_stringified()
        )
    }
}

pub trait PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream {
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream for T
where
    T: PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseStringified,
{
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
            error_upper_camel_case_stringified(),
            named_upper_camel_case_stringified()
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
        let value_stringified = self.payload_with_serialize_deserialize_try_from_payload_error_named_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            with_upper_camel_case_stringified(),
            serialize_upper_camel_case_stringified(),
            deserialize_upper_camel_case_stringified()
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
        let value_stringified =
            self.try_self_with_serialize_deserialize_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            response_upper_camel_case_stringified(),
            variants_upper_camel_case_stringified(),
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
        let value_stringified =
            self.try_self_response_variants_status_code_stringified(status_code);
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            with_upper_camel_case_stringified(),
            serialize_upper_camel_case_stringified(),
            deserialize_upper_camel_case_stringified()
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
        let value_stringified = self.try_self_with_serialize_deserialize_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
            try_snake_case_stringified(),
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
        let value_stringified = self.tvfrr_extraction_logic_try_self_snake_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
            try_snake_case_stringified(),
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
        let value_stringified =
            self.try_self_snake_case_println_stringified(test_operation_print_in_info);
        let value_token_stream = value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
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
        proc_macro_common::generate_quotes::generate_quotes_stringified(&format!(
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
        let value_stringified =
            self.swagger_url_path_self_quotes_stringified(table_name_stringified);
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
        let value_stringified = self.url_handle_self_snake_case_stringified(table_name_stringified);
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}
// fn generate_url_handle_token_stream(
//     table_name_stringified: &str,
//     operation_name_snake_case_stringified: &str,
//     proc_macro_name_upper_camel_case_ident_stringified: &str,
// ) -> proc_macro2::TokenStream {
//     let url_handle_stringified = format!("\"{{}}/{table_name_stringified}/{operation_name_snake_case_stringified}\"");
//     url_handle_stringified.parse::<proc_macro2::TokenStream>()
//     .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {url_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
// }
//
