//todo maybe add another generic - trait casing. and ToUpperCamelCaseString and others would implement it like .to_case::<UpperCamel>()
pub trait ToUpperCamelCaseStringified {
    fn new(&self) -> std::string::String;
}

impl<T: Sized> ToUpperCamelCaseStringified for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn new(&self) -> std::string::String {
        convert_case::Casing::to_case(self, convert_case::Case::UpperCamel)
    }
}

pub trait ToUpperCamelCaseTokenStream {
    fn new_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T: Sized> ToUpperCamelCaseTokenStream for T
where
    T: ToUpperCamelCaseStringified,
{
    fn new_or_panic(&self) -> proc_macro2::TokenStream {
        let value_upper_camel_case_stringified = ToUpperCamelCaseStringified::new(self);
        value_upper_camel_case_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value_upper_camel_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait ToSnakeCaseStringified {
    //todo rename as just snake case and all variable names
    fn new(&self) -> std::string::String;
}

impl<T: Sized> ToSnakeCaseStringified for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn new(&self) -> std::string::String {
        convert_case::Casing::to_case(self, convert_case::Case::Snake)
    }
}

pub trait ToSnakeCaseTokenStream {
    fn new_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T: Sized> ToSnakeCaseTokenStream for T
where
    T: ToSnakeCaseStringified,
{
    fn new_or_panic(&self) -> proc_macro2::TokenStream {
        let value_snake_case_stringified = ToSnakeCaseStringified::new(self);
        value_snake_case_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value_snake_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait ToScreamingSnakeCaseStringified {
    fn new(&self) -> std::string::String;
}

impl<T: Sized> ToScreamingSnakeCaseStringified for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn new(&self) -> std::string::String {
        convert_case::Casing::to_case(self, convert_case::Case::ScreamingSnake)
    }
}

pub trait ToScreamingSnakeCaseTokenStream {
    fn new_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T: Sized> ToScreamingSnakeCaseTokenStream for T
where
    T: ToScreamingSnakeCaseStringified,
{
    fn new_or_panic(&self) -> proc_macro2::TokenStream {
        let value_screaming_snake_case_stringified = ToScreamingSnakeCaseStringified::new(self);
        value_screaming_snake_case_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value_screaming_snake_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub fn display_to_upper_camel_case_stringified(value: &dyn std::fmt::Display) -> std::string::String {
    convert_case::Casing::to_case(&value.to_string(), convert_case::Case::UpperCamel)
}
pub fn display_to_upper_camel_case_token_stream(value: &dyn std::fmt::Display) -> proc_macro2::TokenStream {
    let value_upper_camel_case_stringified = display_to_upper_camel_case_stringified(value);
    value_upper_camel_case_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value_upper_camel_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn display_to_snake_case_stringified(value: &dyn std::fmt::Display) -> std::string::String {
    convert_case::Casing::to_case(&value.to_string(), convert_case::Case::Snake)
}
pub fn display_to_snake_case_token_stream(value: &dyn std::fmt::Display) -> proc_macro2::TokenStream {
    let value_snake_case_stringified = display_to_snake_case_stringified(value);
    value_snake_case_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value_snake_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn display_to_screaming_snake_case_stringified(value: &dyn std::fmt::Display) -> std::string::String {
    convert_case::Casing::to_case(&value.to_string(), convert_case::Case::ScreamingSnake)
}
pub fn display_to_screaming_snake_case_token_stream(value: &dyn std::fmt::Display) -> proc_macro2::TokenStream {
    let value_screaming_snake_case_stringified = display_to_screaming_snake_case_stringified(value);
    value_screaming_snake_case_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value_screaming_snake_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}

pub fn tokens_to_upper_camel_case_stringified(value: &dyn quote::ToTokens) -> std::string::String {
    convert_case::Casing::to_case(&quote::quote!{#value}.to_string(), convert_case::Case::UpperCamel)
}
pub fn tokens_to_upper_camel_case_token_stream(value: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value_upper_camel_case_stringified = tokens_to_upper_camel_case_stringified(value);
    value_upper_camel_case_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value_upper_camel_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn tokens_to_snake_case_stringified(value: &dyn quote::ToTokens) -> std::string::String {
    convert_case::Casing::to_case(&quote::quote!{#value}.to_string(), convert_case::Case::Snake)
}
pub fn tokens_to_snake_case_token_stream(value: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value_snake_case_stringified = tokens_to_snake_case_stringified(value);
    value_snake_case_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value_snake_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn tokens_to_screaming_snake_case_stringified(value: &dyn quote::ToTokens) -> std::string::String {
    convert_case::Casing::to_case(&quote::quote!{#value}.to_string(), convert_case::Case::ScreamingSnake)
}
pub fn tokens_to_screaming_snake_case_token_stream(value: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value_screaming_snake_case_stringified = tokens_to_screaming_snake_case_stringified(value);
    value_screaming_snake_case_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value_screaming_snake_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}