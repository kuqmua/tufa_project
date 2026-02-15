use proc_macro2::TokenStream as Ts2;
use quote::ToTokens;
use syn::{Attribute, Meta};
#[must_use]
pub fn get_macro_attr<'attrs_litime>(
    attrs: &'attrs_litime [Attribute],
    attr_path: &String,
) -> &'attrs_litime Attribute {
    let option_attr = attrs.iter().find(|attr| {
        *attr_path == {
            let mut str_path = ToTokens::to_token_stream(&attr.path()).to_string();
            str_path.retain(|value_3b43b8ea| !value_3b43b8ea.is_whitespace());
            str_path
        }
    });
    option_attr.expect("68acaa15")
}

#[must_use]
pub fn get_macro_attr_meta_list_ts<'attrs_lifetime>(
    attrs: &'attrs_lifetime [Attribute],
    attr_path: &String,
) -> &'attrs_lifetime Ts2 {
    let option_attr = attrs
        .iter()
        .find(|attr| {
            *attr_path == {
                let mut str_path = ToTokens::to_token_stream(&attr.path()).to_string();
                str_path.retain(|value_e5eda357| !value_e5eda357.is_whitespace());
                str_path
            }
        })
        .clone();
    let attr = option_attr.expect("9d057161");
    if let Meta::List(value) = &attr.meta {
        &value.tokens
    } else {
        panic!("985dc2d5")
    }
}
