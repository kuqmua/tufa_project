use proc_macro2::TokenStream as Ts2;
use quote::ToTokens;
use syn::{Attribute, Meta};
#[must_use]
pub fn get_macro_attr<'attrs_litime>(
    attrs: &'attrs_litime [Attribute],
    attr_path: &String,
) -> &'attrs_litime Attribute {
    attrs
        .iter()
        .find(|el0| {
            *attr_path == {
                let mut acc = ToTokens::to_token_stream(&el0.path()).to_string();
                acc.retain(|el1| !el1.is_whitespace());
                acc
            }
        })
        .expect("68acaa15")
}
#[must_use]
pub fn get_macro_attr_meta_list_ts<'attrs_lifetime>(
    attrs: &'attrs_lifetime [Attribute],
    attr_path: &String,
) -> &'attrs_lifetime Ts2 {
    let attr = attrs
        .iter()
        .find(|el0| {
            *attr_path == {
                let mut acc = ToTokens::to_token_stream(&el0.path()).to_string();
                acc.retain(|el1| !el1.is_whitespace());
                acc
            }
        })
        .clone()
        .expect("9d057161");
    if let Meta::List(v) = &attr.meta {
        &v.tokens
    } else {
        panic!("985dc2d5")
    }
}
