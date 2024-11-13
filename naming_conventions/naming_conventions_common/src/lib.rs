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
        let value = ToUpperCamelCaseStringified::new(self);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}


pub trait ToSnakeCaseStringified {
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
        let value = ToSnakeCaseStringified::new(self);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
        let value = ToScreamingSnakeCaseStringified::new(self);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}


////////////
pub trait DisplayToUpperCamelCaseStringified {
    fn new(&self) -> std::string::String;
}
impl<T> DisplayToUpperCamelCaseStringified for T
where T: std::fmt::Display,
{
    fn new(&self) -> std::string::String {
        convert_case::Casing::to_case(&self.to_string(), convert_case::Case::UpperCamel)
    }
}

pub trait DisplayToUpperCamelCaseTokenStream {
    fn new_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> DisplayToUpperCamelCaseTokenStream for T
where T: DisplayToUpperCamelCaseStringified,
{
    fn new_or_panic(&self) -> proc_macro2::TokenStream {
        let value = DisplayToUpperCamelCaseStringified::new(self);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}


pub trait DisplayToSnakeCaseStringified {
    fn new(&self) -> std::string::String;
}
impl<T> DisplayToSnakeCaseStringified for T
where T: std::fmt::Display,
{
    fn new(&self) -> std::string::String {
        convert_case::Casing::to_case(&self.to_string(), convert_case::Case::Snake)
    }
}

pub trait DisplayToSnakeCaseTokenStream {
    fn new_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> DisplayToSnakeCaseTokenStream for T
where T: DisplayToSnakeCaseStringified,
{
    fn new_or_panic(&self) -> proc_macro2::TokenStream {
        let value = DisplayToSnakeCaseStringified::new(self);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}


pub trait DisplayToScreamingSnakeCaseStringified {
    fn new(&self) -> std::string::String;
}
impl<T> DisplayToScreamingSnakeCaseStringified for T
where T: std::fmt::Display,
{
    fn new(&self) -> std::string::String {
        convert_case::Casing::to_case(&self.to_string(), convert_case::Case::ScreamingSnake)
    }
}

pub trait DisplayToScreamingSnakeCaseTokenStream {
    fn new_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> DisplayToScreamingSnakeCaseTokenStream for T
where T: DisplayToScreamingSnakeCaseStringified,
{
    fn new_or_panic(&self) -> proc_macro2::TokenStream {
        let value = DisplayToScreamingSnakeCaseStringified::new(self);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}
////////////

pub trait ToTokensToUpperCamelCaseStringified {
    fn new(&self) -> std::string::String;
}
impl<T> ToTokensToUpperCamelCaseStringified for T
where T: quote::ToTokens
{
    fn new(&self) -> std::string::String {
        convert_case::Casing::to_case(&quote::quote!{#self}.to_string(), convert_case::Case::UpperCamel)
    }
}

pub trait ToTokensToUpperCamelCaseTokenStream {
    fn new_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> ToTokensToUpperCamelCaseTokenStream for T
where T: ToTokensToUpperCamelCaseStringified,
{
    fn new_or_panic(&self) -> proc_macro2::TokenStream {
        let value = ToTokensToUpperCamelCaseStringified::new(self);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}


pub trait ToTokensToSnakeCaseStringified {
    fn new(&self) -> std::string::String;
}
impl<T> ToTokensToSnakeCaseStringified for T
where T: quote::ToTokens
{
    fn new(&self) -> std::string::String {
        convert_case::Casing::to_case(&quote::quote!{#self}.to_string(), convert_case::Case::Snake)
    }
}

pub trait ToTokensToSnakeCaseTokenStream {
    fn new_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> ToTokensToSnakeCaseTokenStream for T
where T: ToTokensToSnakeCaseStringified,
{
    fn new_or_panic(&self) -> proc_macro2::TokenStream {
        let value = ToTokensToSnakeCaseStringified::new(self);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}


pub trait ToTokensToScreamingSnakeCaseStringified {
    fn new(&self) -> std::string::String;
}
impl<T> ToTokensToScreamingSnakeCaseStringified for T
where T: quote::ToTokens,
{
    fn new(&self) -> std::string::String {
        convert_case::Casing::to_case(&quote::quote!{#self}.to_string(), convert_case::Case::ScreamingSnake)
    }
}

pub trait ToTokensToScreamingSnakeCaseTokenStream {
    fn new_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> ToTokensToScreamingSnakeCaseTokenStream for T
where T: ToTokensToScreamingSnakeCaseStringified,
{
    fn new_or_panic(&self) -> proc_macro2::TokenStream {
        let value = ToTokensToScreamingSnakeCaseStringified::new(self);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}



pub fn display_to_upper_camel_case_stringified(value: &dyn std::fmt::Display) -> std::string::String {
    convert_case::Casing::to_case(&value.to_string(), convert_case::Case::UpperCamel)
}
pub fn display_to_upper_camel_case_token_stream(value: &dyn std::fmt::Display) -> proc_macro2::TokenStream {
    let value = display_to_upper_camel_case_stringified(value);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn display_to_snake_case_stringified(value: &dyn std::fmt::Display) -> std::string::String {
    convert_case::Casing::to_case(&value.to_string(), convert_case::Case::Snake)
}
pub fn display_to_snake_case_token_stream(value: &dyn std::fmt::Display) -> proc_macro2::TokenStream {
    let value = display_to_snake_case_stringified(value);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn display_to_screaming_snake_case_stringified(value: &dyn std::fmt::Display) -> std::string::String {
    convert_case::Casing::to_case(&value.to_string(), convert_case::Case::ScreamingSnake)
}
pub fn display_to_screaming_snake_case_token_stream(value: &dyn std::fmt::Display) -> proc_macro2::TokenStream {
    let value = display_to_screaming_snake_case_stringified(value);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}

pub fn tokens_to_upper_camel_case_stringified(value: &dyn quote::ToTokens) -> std::string::String {
    convert_case::Casing::to_case(&quote::quote!{#value}.to_string(), convert_case::Case::UpperCamel)
}
pub fn tokens_to_upper_camel_case_token_stream(value: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value = tokens_to_upper_camel_case_stringified(value);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn tokens_to_snake_case_stringified(value: &dyn quote::ToTokens) -> std::string::String {
    convert_case::Casing::to_case(&quote::quote!{#value}.to_string(), convert_case::Case::Snake)
}
pub fn tokens_to_snake_case_token_stream(value: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value = tokens_to_snake_case_stringified(value);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn tokens_to_screaming_snake_case_stringified(value: &dyn quote::ToTokens) -> std::string::String {
    convert_case::Casing::to_case(&quote::quote!{#value}.to_string(), convert_case::Case::ScreamingSnake)
}
pub fn tokens_to_screaming_snake_case_token_stream(value: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value = tokens_to_screaming_snake_case_stringified(value);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}