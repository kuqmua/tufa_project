use proc_macro2::TokenStream as Ts2;
use quote::ToTokens;
#[must_use]
pub fn get_macro_attribute<'attributes_litime>(
    attributes: &'attributes_litime [syn::Attribute],
    attribute_path: &String,
) -> &'attributes_litime syn::Attribute {
    let option_attribute = attributes.iter().find(|attr| {
        *attribute_path == {
            let mut stringified_path = ToTokens::to_token_stream(&attr.path()).to_string();
            stringified_path.retain(|value_3b43b8ea| !value_3b43b8ea.is_whitespace());
            stringified_path
        }
    });
    option_attribute.expect("68acaa15")
}

#[must_use]
pub fn get_macro_attribute_meta_list_ts<'attributes_lifetime>(
    attributes: &'attributes_lifetime [syn::Attribute],
    attribute_path: &String,
) -> &'attributes_lifetime Ts2 {
    let option_attribute = attributes
        .iter()
        .find(|attr| {
            *attribute_path == {
                let mut stringified_path = ToTokens::to_token_stream(&attr.path()).to_string();
                stringified_path.retain(|value_e5eda357| !value_e5eda357.is_whitespace());
                stringified_path
            }
        })
        .clone();
    let attribute = option_attribute.expect("9d057161");
    if let syn::Meta::List(value) = &attribute.meta {
        &value.tokens
    } else {
        panic!("985dc2d5")
    }
}
