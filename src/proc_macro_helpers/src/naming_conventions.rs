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
pub const SUPPORTS_ONLY_STRINGIFIED: &str = "supports only";
pub const SYN_FIELDS: &str = "syn::Fields";
pub const SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT: &str =
    "proc_macro_helpers::error_occurence::supported_container::SupportedContainer::";
pub const SUPPORTED_ENUM_VARIANT_STRINGIFIED: &str =
    "proc_macro_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant";
pub const SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED: &str = "syn::GenericArgument::Type";
pub const IS_NONE_STRINGIFIED: &str = "is None";
pub const STD_STRINGIFIED: &str = "std";
pub const SQLX_TYPES_UUID_STRINGIFIED: &str = "sqlx::types::Uuid";
pub const FIELD_IDENT_IS_NONE: &str = "field.ident is None";

const NAMED: &str = "named";
pub fn named_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&NAMED)
}
pub fn named_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&NAMED)
}
const ERROR: &str = "error";
pub fn error_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&ERROR)
}
pub fn error_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&ERROR)
}
const OCCURENCE: &str = "occurence";
pub fn occurence_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&OCCURENCE)
}
pub fn occurence_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&OCCURENCE)
}
const STRING: &str = "string";
pub fn string_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&STRING)
}
pub fn string_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&STRING)
}
const PARAMETERS: &str = "parameters";
pub fn parameters_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&PARAMETERS)
}
// pub fn parameters_snake_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&PARAMETERS)
// }
const PAYLOAD: &str = "payload";
pub fn payload_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&PAYLOAD)
}
pub fn payload_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&PAYLOAD)
}
const ELEMENT: &str = "element";
pub fn element_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&ELEMENT)
}
pub fn element_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&ELEMENT)
}
const TRY: &str = "try";
pub fn try_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&TRY)
}
pub fn try_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&TRY)
}
const FROM: &str = "from";
pub fn from_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&FROM)
}
pub fn from_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&FROM)
}
const RESPONSE_VARIANTS: &str = "response_variants";
pub fn response_variants_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&RESPONSE_VARIANTS)
}
// pub fn response_variants_snake_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&RESPONSE_VARIANTS)
// }
const PATH: &str = "path";
pub fn path_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&PATH)
}
pub fn path_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&PATH)
}
const KEY: &str = "key";
pub fn key_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&KEY)
}
pub fn key_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&KEY)
}
const VALUE: &str = "value";
pub fn value_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&VALUE)
}
pub fn value_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&VALUE)
}
const VEC: &str = "vec";
pub fn vec_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&VEC)
}
pub fn vec_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&VEC)
}
const HASHMAP: &str = "HashMap";
pub fn hashmap_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&HASHMAP)
}
pub fn hashmap_snake_case_stringified() -> std::string::String {
    //naming exception - 
    std::string::String::from("hashmap")
}
const REFERENCE: &str = "reference";
pub fn reference_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&REFERENCE)
}
pub fn reference_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&REFERENCE)
}
const WITH: &str = "with";
pub fn with_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&WITH)
}
pub fn with_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&WITH)
}
const SERIALIZE: &str = "serialize";
pub fn serialize_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&SERIALIZE)
}
pub fn serialize_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&SERIALIZE)
}
const DESERIALIZE: &str = "deserialize";
pub fn deserialize_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&DESERIALIZE)
}
pub fn deserialize_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&DESERIALIZE)
}
const REQUEST: &str = "request";
pub fn request_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&REQUEST)
}
pub fn request_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&REQUEST)
}
const RESPONSE: &str = "response";
pub fn response_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&RESPONSE)
}
pub fn response_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&RESPONSE)
}
const VARIANTS: &str = "variants";
pub fn variants_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&VARIANTS)
}
pub fn variants_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&VARIANTS)
}
const TVFRR_EXTRACTION_LOGIC: &str = "tvfrr_extraction_logic";
// pub fn tvfrr_extraction_logic_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&TVFRR_EXTRACTION_LOGIC)
// }
pub fn tvfrr_extraction_logic_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&TVFRR_EXTRACTION_LOGIC)
}
const OPTIONS: &str = "options";
pub fn options_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&OPTIONS)
}
// pub fn options_snake_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&OPTIONS)
// }
const CODE: &str = "code";
pub fn code_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&CODE)
}
pub fn code_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&CODE)
}
const CONFIG: &str = "config";
pub fn config_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&CONFIG)
}
pub fn config_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&CONFIG)
}
const IS: &str = "is";
pub fn is_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&IS)
}
pub fn is_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&IS)
}
const NONE: &str = "none";
pub fn none_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&NONE)
}
pub fn none_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&NONE)
}
const STR: &str = "str";
pub fn str_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&STR)
}
pub fn str_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&STR)
}
const UUID: &str = "uuid";
pub fn uuid_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&UUID)
}
pub fn uuid_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&UUID)
}
const WRAPPER: &str = "wrapper";
pub fn wrapper_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&WRAPPER)
}
pub fn wrapper_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&WRAPPER)
}
const POSSIBLE: &str = "possible";
pub fn possible_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&POSSIBLE)
}
pub fn possible_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&POSSIBLE)
}
const SOURCE: &str = "source";
pub fn source_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&SOURCE)
}
pub fn source_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&SOURCE)
}
const DISPLAY: &str = "display";
pub fn display_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&DISPLAY)
}
pub fn display_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&DISPLAY)
}
const FOREIGN: &str = "foreign";
pub fn foreign_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&FOREIGN)
}
pub fn foreign_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&FOREIGN)
}
const TYPE: &str = "type";
pub fn type_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&TYPE)
}
pub fn type_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&TYPE)
}
const INTO: &str = "into";
pub fn into_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&INTO)
}
pub fn into_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&INTO)
}
const GET: &str = "get";
pub fn get_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&GET)
}
pub fn get_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&GET)
}
const COLUMN: &str = "column";
pub fn column_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&COLUMN)
}
pub fn column_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&COLUMN)
}
const SELECT: &str = "select";
pub fn select_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&SELECT)
}
pub fn select_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&SELECT)
}
const ORDER: &str = "order";
pub fn order_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&ORDER)
}
pub fn order_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&ORDER)
}
const BY: &str = "by";
pub fn by_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&BY)
}
pub fn by_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&BY)
}
const NOT: &str = "not";
pub fn not_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&NOT)
}
pub fn not_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&NOT)
}
const FOUND: &str = "found";
pub fn found_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&FOUND)
}
pub fn found_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&FOUND)
}







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
pub fn unnamed_upper_camel_case_stringified() -> std::string::String {
    format!("Un{}", named_snake_case_stringified())
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
        SUPPORTS_ONLY_STRINGIFIED, SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT
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
pub fn from_str_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}", 
        from_upper_camel_case_stringified(), 
        str_upper_camel_case_stringified()
    )
}
pub fn column_not_found_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}", 
        column_upper_camel_case_stringified(), 
        not_upper_camel_case_stringified(),
        found_upper_camel_case_stringified(),
    )
}







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
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(&self) -> std::string::String {
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

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2> {
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
        let value_stringified =
            self.self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified();
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
    fn try_self_response_variants_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfResponseVariantsUpperCamelCaseTokenStream for T
where
    T: TrySelfResponseVariantsUpperCamelCaseStringified,
{
    fn try_self_response_variants_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
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
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> SelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(&self) -> std::string::String {
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
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: SelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_element_with_serialize_deserialize_upper_camel_case_stringified();
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

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified for T
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

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream for T
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

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCasePunctuated {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCasePunctuated for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2> {
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

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseStringified for T
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

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseTokenStream for T
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

pub trait TrySelfErrorNamedUpperCamelCaseTokenStream
{
    fn try_self_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T>
    TrySelfErrorNamedUpperCamelCaseTokenStream
    for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
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
    fn try_self_request_error_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> TrySelfRequestErrorUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_request_error_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
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
    fn try_self_request_error_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfRequestErrorUpperCamelCaseTokenStream for T
where
    T: TrySelfRequestErrorUpperCamelCaseStringified,
{
    fn try_self_request_error_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
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
        let value_stringified = self.try_self_with_serialize_deserialize_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfResponseVariantsStatusCodeStringified {
    fn try_self_response_variants_status_code_stringified(
        &self,
        status_code: &crate::status_code::StatusCode
    ) -> std::string::String;
}

impl<T> TrySelfResponseVariantsStatusCodeStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_response_variants_status_code_stringified(
        &self,
        status_code: &crate::status_code::StatusCode
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
        status_code: &crate::status_code::StatusCode
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfResponseVariantsStatusCodeTokenStream for T
where
    T: TrySelfResponseVariantsStatusCodeStringified,
{
    fn try_self_response_variants_status_code_token_stream(
        &self,
        status_code: &crate::status_code::StatusCode
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_response_variants_status_code_stringified(status_code);
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
        let value_stringified = self.try_self_snake_case_println_stringified(test_operation_print_in_info);
        let value_token_stream = value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
        quote::quote!{println!(#value_token_stream);}
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
        let start_println_token_stream = self.try_self_snake_case_println_token_stream(&crate::TestOperationPrintlnInfo::Start);
        let end_println_token_stream = self.try_self_snake_case_println_token_stream(&crate::TestOperationPrintlnInfo::End);
        quote::quote!{
            #start_println_token_stream
            #test_content_token_stream
            #end_println_token_stream
        }
    }
}

pub trait SwaggerUrlPathSelfQuotesStringified {
    fn swagger_url_path_self_quotes_stringified(&self, table_name_stringified: &str) -> std::string::String;
}

impl<T> SwaggerUrlPathSelfQuotesStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn swagger_url_path_self_quotes_stringified(&self, table_name_stringified: &str) -> std::string::String {
        proc_macro_common::generate_quotes::generate_quotes_stringified(&format!(
            "/{}/{}",
            table_name_stringified,
            self.to_snake_case_stringified(),
        ))
    }
}

pub trait SwaggerUrlPathSelfQuotesTokenStream {
    fn swagger_url_path_self_quotes_token_stream(&self, table_name_stringified: &str) -> proc_macro2::TokenStream;
}

impl<T> SwaggerUrlPathSelfQuotesTokenStream for T
where
    T: SwaggerUrlPathSelfQuotesStringified,
{
    fn swagger_url_path_self_quotes_token_stream(&self, table_name_stringified: &str) -> proc_macro2::TokenStream {
        let value_stringified = self.swagger_url_path_self_quotes_stringified(table_name_stringified);
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
        let value_stringified =
            self.url_handle_self_snake_case_stringified(table_name_stringified);
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