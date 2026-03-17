use convert_case::{Case, Casing};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use std::fmt::Display;
//todo mb add another generic - trait casing. and ToUccString and others would implement it like .to_case::<UpperCamel>()
macro_rules! case_str_trait_as_ref {
    ($trait_name:ident, $conv_fn:ident) => {
        pub trait $trait_name {
            fn case(&self) -> String;
        }
        impl<T: Sized> $trait_name for T
        where
            String: PartialEq<T>,
            Self: AsRef<str>,
        {
            fn case(&self) -> String {
                $conv_fn(self)
            }
        }
    };
}
macro_rules! case_str_trait_display {
    ($trait_name:ident, $conv_fn:ident) => {
        pub trait $trait_name {
            fn case(&self) -> String;
        }
        impl<T> $trait_name for T
        where
            T: Display,
        {
            fn case(&self) -> String {
                $conv_fn(&self.to_string())
            }
        }
    };
}
macro_rules! case_str_trait_to_tokens {
    ($trait_name:ident, $conv_fn:ident) => {
        pub trait $trait_name {
            fn case(&self) -> String;
        }
        impl<T> $trait_name for T
        where
            T: ToTokens,
        {
            fn case(&self) -> String {
                $conv_fn(&quote! {#self}.to_string())
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
case_str_trait_as_ref!(AsRefStrToUccStr, to_ucc_str);
case_ts_trait!(AsRefStrToUccTs, AsRefStrToUccStr);
case_str_trait_as_ref!(AsRefStrToScStr, to_sc_str);
case_ts_trait!(AsRefStrToScTs, AsRefStrToScStr);
case_str_trait_as_ref!(AsRefStrToUpperScStr, to_upper_sc_str);
case_ts_trait!(AsRefStrToUpperScTs, AsRefStrToUpperScStr);
case_str_trait_display!(DisplayToUccStr, to_ucc_str);
case_ts_trait!(DisplayToUccTs, DisplayToUccStr);
case_str_trait_display!(DisplayToScStr, to_sc_str);
case_ts_trait!(DisplayToScTs, DisplayToScStr);
case_str_trait_display!(DisplayToUpperScStr, to_upper_sc_str);
case_ts_trait!(DisplayToUpperScTs, DisplayToUpperScStr);
case_str_trait_to_tokens!(ToTokensToUccStr, to_ucc_str);
case_ts_trait!(ToTokensToUccTs, ToTokensToUccStr);
case_str_trait_to_tokens!(ToTokensToScStr, to_sc_str);
case_ts_trait!(ToTokensToScTs, ToTokensToScStr);
case_str_trait_to_tokens!(ToTokensToUpperScStr, to_upper_sc_str);
case_ts_trait!(ToTokensToUpperScTs, ToTokensToUpperScStr);
fn to_ts_or_panic(v: &dyn Display) -> Ts2 {
    v.to_string().parse::<Ts2>().expect("753ce6dd")
}
fn to_ucc_str<T: AsRef<str>>(v: &T) -> String
where
    String: PartialEq<T>,
{
    Casing::to_case(v, Case::UpperCamel)
}
fn to_sc_str<T: AsRef<str>>(v: &T) -> String
where
    String: PartialEq<T>,
{
    Casing::to_case(v, Case::Snake)
}
fn to_upper_sc_str<T: AsRef<str>>(v: &T) -> String
where
    String: PartialEq<T>,
{
    Casing::to_case(v, Case::UpperSnake)
}
