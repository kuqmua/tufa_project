use convert_case::{Case, Casing as _};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use std::fmt::Display;
//todo mb add another generic - trait casing. and ToUccString and others would implement it like .to_case::<UpperCamel>()
macro_rules! case_str_trait {
    ($trait_name:ident, $bound:path, |$slf:ident| $body:expr) => {
        pub trait $trait_name {
            fn case(&self) -> String;
        }
        impl<T> $trait_name for T
        where
            T: $bound,
        {
            fn case(&self) -> String {
                let $slf = self;
                $body
            }
        }
    };
}
macro_rules! case_ts_trait {
    ($ts_trait:ident, $str_trait:ident) => {
        pub trait $ts_trait {
            fn case_or_panic(&self) -> Ts2;
        }
        impl<T> $ts_trait for T
        where
            T: $str_trait,
        {
            fn case_or_panic(&self) -> Ts2 {
                to_ts_or_panic(&$str_trait::case(self))
            }
        }
    };
}
case_str_trait!(AsRefStrToUccStr, AsRef<str>, |self_ref| str_case(
    self_ref.as_ref(),
    Case::UpperCamel
));
case_ts_trait!(AsRefStrToUccTs, AsRefStrToUccStr);
case_str_trait!(AsRefStrToScStr, AsRef<str>, |self_ref| str_case(
    self_ref.as_ref(),
    Case::Snake
));
case_ts_trait!(AsRefStrToScTs, AsRefStrToScStr);
case_str_trait!(AsRefStrToUpperScStr, AsRef<str>, |self_ref| str_case(
    self_ref.as_ref(),
    Case::UpperSnake
));
case_ts_trait!(AsRefStrToUpperScTs, AsRefStrToUpperScStr);
case_str_trait!(DisplayToUccStr, Display, |self_ref| display_case_str(
    self_ref,
    Case::UpperCamel
));
case_ts_trait!(DisplayToUccTs, DisplayToUccStr);
case_str_trait!(DisplayToScStr, Display, |self_ref| display_case_str(
    self_ref,
    Case::Snake
));
case_ts_trait!(DisplayToScTs, DisplayToScStr);
case_str_trait!(DisplayToUpperScStr, Display, |self_ref| display_case_str(
    self_ref,
    Case::UpperSnake
));
case_ts_trait!(DisplayToUpperScTs, DisplayToUpperScStr);
case_str_trait!(ToTokensToUccStr, ToTokens, |self_ref| tokenized_case_str(
    self_ref,
    Case::UpperCamel
));
case_ts_trait!(ToTokensToUccTs, ToTokensToUccStr);
case_str_trait!(ToTokensToScStr, ToTokens, |self_ref| tokenized_case_str(
    self_ref,
    Case::Snake
));
case_ts_trait!(ToTokensToScTs, ToTokensToScStr);
case_str_trait!(ToTokensToUpperScStr, ToTokens, |self_ref| {
    tokenized_case_str(self_ref, Case::UpperSnake)
});
case_ts_trait!(ToTokensToUpperScTs, ToTokensToUpperScStr);
fn to_ts_or_panic<T>(v: &T) -> Ts2
where
    T: Display + ?Sized,
{
    v.to_string().parse::<Ts2>().expect("753ce6dd")
}
fn display_case_str<T: Display>(v: &T, case: Case<'_>) -> String {
    let stringified = v.to_string();
    str_case(&stringified, case)
}
fn tokenized_case_str<T: ToTokens>(v: &T, case: Case<'_>) -> String {
    let tokenized = quote! {#v}.to_string();
    str_case(&tokenized, case)
}
fn str_case(v: &str, case: Case<'_>) -> String {
    v.to_case(case)
}
#[cfg(test)]
mod tests {
    use super::{
        AsRefStrToScStr, AsRefStrToScTs, AsRefStrToUccStr, AsRefStrToUccTs, AsRefStrToUpperScStr,
        AsRefStrToUpperScTs, DisplayToScStr, DisplayToScTs, DisplayToUccStr, DisplayToUccTs,
        DisplayToUpperScStr, DisplayToUpperScTs, ToTokensToScStr, ToTokensToScTs, ToTokensToUccStr,
        ToTokensToUccTs, ToTokensToUpperScStr, ToTokensToUpperScTs,
    };
    use quote::quote;
    #[test]
    fn as_ref_case_conversions_are_expected() {
        assert_eq!(AsRefStrToUccStr::case(&"hello_world"), "HelloWorld");
        assert_eq!(AsRefStrToScStr::case(&"HelloWorld"), "hello_world");
        assert_eq!(AsRefStrToUpperScStr::case(&"helloWorld"), "HELLO_WORLD");
    }
    #[test]
    fn ts_case_conversions_are_expected() {
        assert_eq!(
            AsRefStrToUccTs::case_or_panic(&"hello_world").to_string(),
            "HelloWorld"
        );
        assert_eq!(
            AsRefStrToScTs::case_or_panic(&"HelloWorld").to_string(),
            "hello_world"
        );
        assert_eq!(
            AsRefStrToUpperScTs::case_or_panic(&"helloWorld").to_string(),
            "HELLO_WORLD"
        );
    }
    #[test]
    fn display_and_tokens_conversion_are_expected() {
        assert_eq!(DisplayToUccStr::case(&"hello_world"), "HelloWorld");
        assert_eq!(DisplayToScStr::case(&"HelloWorld"), "hello_world");
        assert_eq!(DisplayToUpperScStr::case(&"helloWorld"), "HELLO_WORLD");
        assert_eq!(ToTokensToUccStr::case(&quote! {hello_world}), "HelloWorld");
        assert_eq!(ToTokensToScStr::case(&quote! {HelloWorld}), "hello_world");
        assert_eq!(
            ToTokensToUpperScStr::case(&quote! {helloWorld}),
            "HELLO_WORLD"
        );
    }
    #[test]
    fn display_and_tokens_ts_conversion_are_expected() {
        assert_eq!(
            DisplayToUccTs::case_or_panic(&"hello_world").to_string(),
            "HelloWorld"
        );
        assert_eq!(
            DisplayToScTs::case_or_panic(&"HelloWorld").to_string(),
            "hello_world"
        );
        assert_eq!(
            DisplayToUpperScTs::case_or_panic(&"helloWorld").to_string(),
            "HELLO_WORLD"
        );
        assert_eq!(
            ToTokensToUccTs::case_or_panic(&quote! {hello_world}).to_string(),
            "HelloWorld"
        );
        assert_eq!(
            ToTokensToScTs::case_or_panic(&quote! {HelloWorld}).to_string(),
            "hello_world"
        );
        assert_eq!(
            ToTokensToUpperScTs::case_or_panic(&quote! {helloWorld}).to_string(),
            "HELLO_WORLD"
        );
    }
}
