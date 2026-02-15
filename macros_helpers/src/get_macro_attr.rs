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
                let mut value = ToTokens::to_token_stream(&el0.path()).to_string();
                value.retain(|el1| !el1.is_whitespace());
                value
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
                let mut value = ToTokens::to_token_stream(&el0.path()).to_string();
                value.retain(|el1| !el1.is_whitespace());
                value
            }
        })
        .clone()
        .expect("9d057161");
    if let Meta::List(value) = &attr.meta {
        &value.tokens
    } else {
        panic!("985dc2d5")
    }
}
