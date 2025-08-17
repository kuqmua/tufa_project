fn to_upper_camel_case_stringified<T: AsRef<str>>(value: &T) -> std::string::String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::UpperCamel)
}
fn to_snake_case_stringified<T: AsRef<str>>(value: &T) -> std::string::String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::Snake)
}
fn to_screaming_snake_case_stringified<T: AsRef<str>>(value: &T) -> std::string::String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::ScreamingSnake)
}
fn to_token_stream_or_panic(value: &dyn std::fmt::Display) -> proc_macro2::TokenStream {
    value.to_string().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}

//todo maybe add another generic - trait casing. and ToUpperCamelCaseString and others would implement it like .to_case::<UpperCamel>()
pub trait AsRefStrToUpperCamelCaseStringified {
    fn case(&self) -> std::string::String;
}
impl<T: Sized> AsRefStrToUpperCamelCaseStringified for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> std::string::String {
        to_upper_camel_case_stringified(self)
    }
}

pub trait AsRefStrToUpperCamelCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T: Sized> AsRefStrToUpperCamelCaseTokenStream for T
where
    T: AsRefStrToUpperCamelCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&AsRefStrToUpperCamelCaseStringified::case(self))
    }
}

pub trait AsRefStrToSnakeCaseStringified {
    fn case(&self) -> std::string::String;
}
impl<T: Sized> AsRefStrToSnakeCaseStringified for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> std::string::String {
        to_snake_case_stringified(self)
    }
}

pub trait AsRefStrToSnakeCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T: Sized> AsRefStrToSnakeCaseTokenStream for T
where
    T: AsRefStrToSnakeCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&AsRefStrToSnakeCaseStringified::case(self))
    }
}

pub trait AsRefStrToScreamingSnakeCaseStringified {
    fn case(&self) -> std::string::String;
}
impl<T: Sized> AsRefStrToScreamingSnakeCaseStringified for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> std::string::String {
        to_screaming_snake_case_stringified(self)
    }
}

pub trait AsRefStrToScreamingSnakeCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T: Sized> AsRefStrToScreamingSnakeCaseTokenStream for T
where
    T: AsRefStrToScreamingSnakeCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&AsRefStrToScreamingSnakeCaseStringified::case(self))
    }
}

////////////
pub trait DisplayToUpperCamelCaseStringified {
    fn case(&self) -> std::string::String;
}
impl<T> DisplayToUpperCamelCaseStringified for T
where
    T: std::fmt::Display,
{
    fn case(&self) -> std::string::String {
        to_upper_camel_case_stringified(&self.to_string())
    }
}

pub trait DisplayToUpperCamelCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> DisplayToUpperCamelCaseTokenStream for T
where
    T: DisplayToUpperCamelCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&DisplayToUpperCamelCaseStringified::case(self))
    }
}

pub trait DisplayToSnakeCaseStringified {
    fn case(&self) -> std::string::String;
}
impl<T> DisplayToSnakeCaseStringified for T
where
    T: std::fmt::Display,
{
    fn case(&self) -> std::string::String {
        to_snake_case_stringified(&self.to_string())
    }
}

pub trait DisplayToSnakeCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> DisplayToSnakeCaseTokenStream for T
where
    T: DisplayToSnakeCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&DisplayToSnakeCaseStringified::case(self))
    }
}

pub trait DisplayToScreamingSnakeCaseStringified {
    fn case(&self) -> std::string::String;
}
impl<T> DisplayToScreamingSnakeCaseStringified for T
where
    T: std::fmt::Display,
{
    fn case(&self) -> std::string::String {
        to_screaming_snake_case_stringified(&self.to_string())
    }
}

pub trait DisplayToScreamingSnakeCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> DisplayToScreamingSnakeCaseTokenStream for T
where
    T: DisplayToScreamingSnakeCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&DisplayToScreamingSnakeCaseStringified::case(self))
    }
}
////////////

pub trait ToTokensToUpperCamelCaseStringified {
    fn case(&self) -> std::string::String;
}
impl<T> ToTokensToUpperCamelCaseStringified for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> std::string::String {
        to_upper_camel_case_stringified(&quote::quote! {#self}.to_string())
    }
}

pub trait ToTokensToUpperCamelCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> ToTokensToUpperCamelCaseTokenStream for T
where
    T: ToTokensToUpperCamelCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&ToTokensToUpperCamelCaseStringified::case(self))
    }
}

pub trait ToTokensToSnakeCaseStringified {
    fn case(&self) -> std::string::String;
}
impl<T> ToTokensToSnakeCaseStringified for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> std::string::String {
        to_snake_case_stringified(&quote::quote! {#self}.to_string().to_string())
    }
}

pub trait ToTokensToSnakeCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> ToTokensToSnakeCaseTokenStream for T
where
    T: ToTokensToSnakeCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&ToTokensToSnakeCaseStringified::case(self))
    }
}

pub trait ToTokensToScreamingSnakeCaseStringified {
    fn case(&self) -> std::string::String;
}
impl<T> ToTokensToScreamingSnakeCaseStringified for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> std::string::String {
        to_screaming_snake_case_stringified(&quote::quote! {#self}.to_string())
    }
}

pub trait ToTokensToScreamingSnakeCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> ToTokensToScreamingSnakeCaseTokenStream for T
where
    T: ToTokensToScreamingSnakeCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&ToTokensToScreamingSnakeCaseStringified::case(self))
    }
}
