#[must_use]
pub fn get_macro_attribute<'attributes_litime>(
    attributes: &'attributes_litime [syn::Attribute],
    attribute_path: &String,
) -> &'attributes_litime syn::Attribute {
    let option_attribute = attributes.iter().find(|attr| {
        *attribute_path == {
            let mut stringified_path = quote::ToTokens::to_token_stream(&attr.path()).to_string();
            stringified_path.retain(|value_3b43b8ea| !value_3b43b8ea.is_whitespace());
            stringified_path
        }
    });
    option_attribute.expect("68acaa15-abdc-4ede-b38a-4cf744512136")
}

#[must_use]
pub fn get_macro_attribute_meta_list_ts<'attributes_lifetime>(
    attributes: &'attributes_lifetime [syn::Attribute],
    attribute_path: &String,
) -> &'attributes_lifetime proc_macro2::TokenStream {
    let option_attribute = attributes
        .iter()
        .find(|attr| {
            *attribute_path == {
                let mut stringified_path =
                    quote::ToTokens::to_token_stream(&attr.path()).to_string();
                stringified_path.retain(|value_e5eda357| !value_e5eda357.is_whitespace());
                stringified_path
            }
        })
        .clone();
    let attribute = option_attribute.expect("9d057161-4c8e-4cf5-bee3-12c0020d5b95");
    if let syn::Meta::List(value) = &attribute.meta {
        &value.tokens
    } else {
        panic!("985dc2d5-f743-4f1e-a6f3-25c96c2af323")
    }
}
