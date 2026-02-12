use proc_macro2::TokenStream as Ts2;
use quote::quote;
use std::fmt::Display;
//todo maybe add another generic - trait casing. and ToUccString and others would implement it like .to_case::<UpperCamel>()
pub trait AsRefStrToUccStr {
    fn case(&self) -> String;
}
impl<T: Sized> AsRefStrToUccStr for T
where
    String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> String {
        to_ucc_str(self)
    }
}
pub trait AsRefStrToUccTs {
    fn case_or_panic(&self) -> Ts2;
}
impl<T> AsRefStrToUccTs for T
where
    T: Sized + AsRefStrToUccStr,
{
    fn case_or_panic(&self) -> Ts2 {
        to_ts_or_panic(&AsRefStrToUccStr::case(self))
    }
}
pub trait AsRefStrToScStr {
    fn case(&self) -> String;
}
impl<T: Sized> AsRefStrToScStr for T
where
    String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> String {
        to_sc_str(self)
    }
}
pub trait AsRefStrToScTs {
    fn case_or_panic(&self) -> Ts2;
}
impl<T> AsRefStrToScTs for T
where
    T: Sized + AsRefStrToScStr,
{
    fn case_or_panic(&self) -> Ts2 {
        to_ts_or_panic(&AsRefStrToScStr::case(self))
    }
}
pub trait AsRefStrToUpperScStr {
    fn case(&self) -> String;
}
impl<T: Sized> AsRefStrToUpperScStr for T
where
    String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn case(&self) -> String {
        to_upper_sc_str(self)
    }
}
pub trait AsRefStrToUpperScTs {
    fn case_or_panic(&self) -> Ts2;
}
impl<T> AsRefStrToUpperScTs for T
where
    T: Sized + AsRefStrToUpperScStr,
{
    fn case_or_panic(&self) -> Ts2 {
        to_ts_or_panic(&AsRefStrToUpperScStr::case(self))
    }
}
pub trait DisplayToUccStr {
    fn case(&self) -> String;
}
impl<T> DisplayToUccStr for T
where
    T: Display,
{
    fn case(&self) -> String {
        to_ucc_str(&self.to_string())
    }
}
pub trait DisplayToUccTs {
    fn case_or_panic(&self) -> Ts2;
}
impl<T> DisplayToUccTs for T
where
    T: DisplayToUccStr,
{
    fn case_or_panic(&self) -> Ts2 {
        to_ts_or_panic(&DisplayToUccStr::case(self))
    }
}
pub trait DisplayToScStr {
    fn case(&self) -> String;
}
impl<T> DisplayToScStr for T
where
    T: Display,
{
    fn case(&self) -> String {
        to_sc_str(&self.to_string())
    }
}
pub trait DisplayToScTs {
    fn case_or_panic(&self) -> Ts2;
}
impl<T> DisplayToScTs for T
where
    T: DisplayToScStr,
{
    fn case_or_panic(&self) -> Ts2 {
        to_ts_or_panic(&DisplayToScStr::case(self))
    }
}
pub trait DisplayToUpperScStr {
    fn case(&self) -> String;
}
impl<T> DisplayToUpperScStr for T
where
    T: Display,
{
    fn case(&self) -> String {
        to_upper_sc_str(&self.to_string())
    }
}
pub trait DisplayToUpperScTs {
    fn case_or_panic(&self) -> Ts2;
}
impl<T> DisplayToUpperScTs for T
where
    T: DisplayToUpperScStr,
{
    fn case_or_panic(&self) -> Ts2 {
        to_ts_or_panic(&DisplayToUpperScStr::case(self))
    }
}
pub trait ToTokensToUccStr {
    fn case(&self) -> String;
}
impl<T> ToTokensToUccStr for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> String {
        to_ucc_str(&quote! {#self}.to_string())
    }
}
pub trait ToTokensToUccTs {
    fn case_or_panic(&self) -> Ts2;
}
impl<T> ToTokensToUccTs for T
where
    T: ToTokensToUccStr,
{
    fn case_or_panic(&self) -> Ts2 {
        to_ts_or_panic(&ToTokensToUccStr::case(self))
    }
}
pub trait ToTokensToScStr {
    fn case(&self) -> String;
}
impl<T> ToTokensToScStr for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> String {
        to_sc_str(&quote! {#self}.to_string())
    }
}
pub trait ToTokensToScTs {
    fn case_or_panic(&self) -> Ts2;
}
impl<T> ToTokensToScTs for T
where
    T: ToTokensToScStr,
{
    fn case_or_panic(&self) -> Ts2 {
        to_ts_or_panic(&ToTokensToScStr::case(self))
    }
}
pub trait ToTokensToUpperScStr {
    fn case(&self) -> String;
}
impl<T> ToTokensToUpperScStr for T
where
    T: quote::ToTokens,
{
    fn case(&self) -> String {
        to_upper_sc_str(&quote! {#self}.to_string())
    }
}
pub trait ToTokensToUpperScTs {
    fn case_or_panic(&self) -> Ts2;
}
impl<T> ToTokensToUpperScTs for T
where
    T: ToTokensToUpperScStr,
{
    fn case_or_panic(&self) -> Ts2 {
        to_ts_or_panic(&ToTokensToUpperScStr::case(self))
    }
}
fn to_ts_or_panic(value: &dyn Display) -> Ts2 {
    value
        .to_string()
        .parse::<Ts2>()
        .expect("753ce6dd-aa0f-4836-8e74-20a7c4f88f60")
}
fn to_ucc_str<T: AsRef<str>>(value: &T) -> String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::UpperCamel)
}
fn to_sc_str<T: AsRef<str>>(value: &T) -> String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::Snake)
}
fn to_upper_sc_str<T: AsRef<str>>(value: &T) -> String
where
    String: PartialEq<T>,
{
    convert_case::Casing::to_case(value, convert_case::Case::UpperSnake)
}
