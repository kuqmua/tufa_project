fn to_upper_camel_case_stringified<T: AsRef<str>>(value: &T) -> String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::UpperCamel)
}
fn to_snake_case_stringified<T: AsRef<str>>(value: &T) -> String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::Snake)
}
fn to_upper_snake_case_stringified<T: AsRef<str>>(value: &T) -> String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::UpperSnake)
}
fn to_token_stream_or_panic(value: &dyn std::fmt::Display) -> proc_macro2::TokenStream {
    value
        .to_string()
        .parse::<proc_macro2::TokenStream>()
        .expect("753ce6dd-aa0f-4836-8e74-20a7c4f88f60")
}

//todo maybe add another generic - trait casing. and ToUpperCamelCaseString and others would implement it like .to_case::<UpperCamel>()
pub trait AsRefStrToUpperCamelCaseStringified {
    fn case(&self) -> String;
}
impl<T: Sized> AsRefStrToUpperCamelCaseStringified for T
where
    String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> String {
        to_upper_camel_case_stringified(self)
    }
}

pub trait AsRefStrToUpperCamelCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> AsRefStrToUpperCamelCaseTokenStream for T
where
    T: Sized + AsRefStrToUpperCamelCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&AsRefStrToUpperCamelCaseStringified::case(self))
    }
}

pub trait AsRefStrToSnakeCaseStringified {
    fn case(&self) -> String;
}
impl<T: Sized> AsRefStrToSnakeCaseStringified for T
where
    String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> String {
        to_snake_case_stringified(self)
    }
}

pub trait AsRefStrToSnakeCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> AsRefStrToSnakeCaseTokenStream for T
where
    T: Sized + AsRefStrToSnakeCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&AsRefStrToSnakeCaseStringified::case(self))
    }
}

pub trait AsRefStrToUpperSnakeCaseStringified {
    fn case(&self) -> String;
}
impl<T: Sized> AsRefStrToUpperSnakeCaseStringified for T
where
    String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> String {
        to_upper_snake_case_stringified(self)
    }
}

pub trait AsRefStrToUpperSnakeCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> AsRefStrToUpperSnakeCaseTokenStream for T
where
    T: Sized + AsRefStrToUpperSnakeCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&AsRefStrToUpperSnakeCaseStringified::case(self))
    }
}

////////////
pub trait DisplayToUpperCamelCaseStringified {
    fn case(&self) -> String;
}
impl<T> DisplayToUpperCamelCaseStringified for T
where
    T: std::fmt::Display,
{
    fn case(&self) -> String {
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
    fn case(&self) -> String;
}
impl<T> DisplayToSnakeCaseStringified for T
where
    T: std::fmt::Display,
{
    fn case(&self) -> String {
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

pub trait DisplayToUpperSnakeCaseStringified {
    fn case(&self) -> String;
}
impl<T> DisplayToUpperSnakeCaseStringified for T
where
    T: std::fmt::Display,
{
    fn case(&self) -> String {
        to_upper_snake_case_stringified(&self.to_string())
    }
}

pub trait DisplayToUpperSnakeCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> DisplayToUpperSnakeCaseTokenStream for T
where
    T: DisplayToUpperSnakeCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&DisplayToUpperSnakeCaseStringified::case(self))
    }
}
////////////

pub trait ToTokensToUpperCamelCaseStringified {
    fn case(&self) -> String;
}
impl<T> ToTokensToUpperCamelCaseStringified for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> String {
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
    fn case(&self) -> String;
}
impl<T> ToTokensToSnakeCaseStringified for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> String {
        to_snake_case_stringified(&quote::quote! {#self}.to_string())
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

pub trait ToTokensToUpperSnakeCaseStringified {
    fn case(&self) -> String;
}
impl<T> ToTokensToUpperSnakeCaseStringified for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> String {
        to_upper_snake_case_stringified(&quote::quote! {#self}.to_string())
    }
}

pub trait ToTokensToUpperSnakeCaseTokenStream {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> ToTokensToUpperSnakeCaseTokenStream for T
where
    T: ToTokensToUpperSnakeCaseStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_token_stream_or_panic(&ToTokensToUpperSnakeCaseStringified::case(self))
    }
}
