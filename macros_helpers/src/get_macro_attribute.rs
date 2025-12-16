pub fn get_macro_attribute<'attributes_litime>(attributes: &'attributes_litime [syn::Attribute], attribute_path: &String, proc_macro_name_ident_stringified: &String) -> &'attributes_litime syn::Attribute {
    let option_attribute = attributes.iter().find(|attr| {
        *attribute_path == {
            let mut stringified_path = quote::ToTokens::to_token_stream(&attr.path()).to_string();
            stringified_path.retain(|value| !value.is_whitespace());
            stringified_path
        }
    });
    option_attribute.unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} no {attribute_path}"))
}

pub fn get_macro_attribute_meta_list_token_stream<'attributes_lifetime>(attributes: &'attributes_lifetime [syn::Attribute], attribute_path: &String) -> &'attributes_lifetime proc_macro2::TokenStream {
    let option_attribute = attributes.iter().find(|attr| {
        *attribute_path == {
            let mut stringified_path = quote::ToTokens::to_token_stream(&attr.path()).to_string();
            stringified_path.retain(|value| !value.is_whitespace());
            stringified_path
        }
    });
    let attribute = option_attribute.unwrap_or_else(|| panic!("no {attribute_path}"));
    if let syn::Meta::List(value) = &attribute.meta { &value.tokens } else { panic!("&attribute.meta is not syn::Meta::List(value)") }
}
