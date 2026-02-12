use quote::quote;
use std::fmt::Display;
//todo maybe add another generic - trait casing. and ToUccString and others would implement it like .to_case::<UpperCamel>()
pub trait AsRefStrToUccStringified {
    fn case(&self) -> String;
}
impl<T: Sized> AsRefStrToUccStringified for T
where
    String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> String {
        to_ucc_stringified(self)
    }
}
pub trait AsRefStrToUccTs {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> AsRefStrToUccTs for T
where
    T: Sized + AsRefStrToUccStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_ts_or_panic(&AsRefStrToUccStringified::case(self))
    }
}
pub trait AsRefStrToScStringified {
    fn case(&self) -> String;
}
impl<T: Sized> AsRefStrToScStringified for T
where
    String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> String {
        to_sc_stringified(self)
    }
}
pub trait AsRefStrToScTs {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> AsRefStrToScTs for T
where
    T: Sized + AsRefStrToScStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_ts_or_panic(&AsRefStrToScStringified::case(self))
    }
}
pub trait AsRefStrToUpperScStringified {
    fn case(&self) -> String;
}
impl<T: Sized> AsRefStrToUpperScStringified for T
where
    String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> String {
        to_upper_sc_stringified(self)
    }
}
pub trait AsRefStrToUpperScTs {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> AsRefStrToUpperScTs for T
where
    T: Sized + AsRefStrToUpperScStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_ts_or_panic(&AsRefStrToUpperScStringified::case(self))
    }
}
pub trait DisplayToUccStringified {
    fn case(&self) -> String;
}
impl<T> DisplayToUccStringified for T
where
    T: Display,
{
    fn case(&self) -> String {
        to_ucc_stringified(&self.to_string())
    }
}
pub trait DisplayToUccTs {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> DisplayToUccTs for T
where
    T: DisplayToUccStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_ts_or_panic(&DisplayToUccStringified::case(self))
    }
}
pub trait DisplayToScStringified {
    fn case(&self) -> String;
}
impl<T> DisplayToScStringified for T
where
    T: Display,
{
    fn case(&self) -> String {
        to_sc_stringified(&self.to_string())
    }
}
pub trait DisplayToScTs {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> DisplayToScTs for T
where
    T: DisplayToScStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_ts_or_panic(&DisplayToScStringified::case(self))
    }
}
pub trait DisplayToUpperScStringified {
    fn case(&self) -> String;
}
impl<T> DisplayToUpperScStringified for T
where
    T: Display,
{
    fn case(&self) -> String {
        to_upper_sc_stringified(&self.to_string())
    }
}
pub trait DisplayToUpperScTs {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> DisplayToUpperScTs for T
where
    T: DisplayToUpperScStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_ts_or_panic(&DisplayToUpperScStringified::case(self))
    }
}
pub trait ToTokensToUccStringified {
    fn case(&self) -> String;
}
impl<T> ToTokensToUccStringified for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> String {
        to_ucc_stringified(&quote! {#self}.to_string())
    }
}
pub trait ToTokensToUccTs {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> ToTokensToUccTs for T
where
    T: ToTokensToUccStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_ts_or_panic(&ToTokensToUccStringified::case(self))
    }
}
pub trait ToTokensToScStringified {
    fn case(&self) -> String;
}
impl<T> ToTokensToScStringified for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> String {
        to_sc_stringified(&quote! {#self}.to_string())
    }
}
pub trait ToTokensToScTs {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> ToTokensToScTs for T
where
    T: ToTokensToScStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_ts_or_panic(&ToTokensToScStringified::case(self))
    }
}
pub trait ToTokensToUpperScStringified {
    fn case(&self) -> String;
}
impl<T> ToTokensToUpperScStringified for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> String {
        to_upper_sc_stringified(&quote! {#self}.to_string())
    }
}
pub trait ToTokensToUpperScTs {
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}
impl<T> ToTokensToUpperScTs for T
where
    T: ToTokensToUpperScStringified,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        to_ts_or_panic(&ToTokensToUpperScStringified::case(self))
    }
}
fn to_ts_or_panic(value: &dyn Display) -> proc_macro2::TokenStream {
    value
        .to_string()
        .parse::<proc_macro2::TokenStream>()
        .expect("753ce6dd-aa0f-4836-8e74-20a7c4f88f60")
}
fn to_ucc_stringified<T: AsRef<str>>(value: &T) -> String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::UpperCamel)
}
fn to_sc_stringified<T: AsRef<str>>(value: &T) -> String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::Snake)
}
fn to_upper_sc_stringified<T: AsRef<str>>(value: &T) -> String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::UpperSnake)
}
