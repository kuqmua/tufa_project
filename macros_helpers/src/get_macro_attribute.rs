pub fn get_macro_attribute<'a>(attrs: &'a [syn::Attribute], attribute_path: &std::string::String, proc_macro_name_ident_stringified: &std::string::String) -> &'a syn::Attribute {
    let option_attribute = attrs.iter().find(|attr| {
        *attribute_path == {
            let mut stringified_path = quote::ToTokens::to_token_stream(&attr.path()).to_string();
            stringified_path.retain(|value| !value.is_whitespace());
            stringified_path
        }
    });
    option_attribute.map_or_else(
        || {
            panic!("{proc_macro_name_ident_stringified} no {attribute_path}");
        },
        |attribute| attribute,
    )
}

pub fn get_macro_attribute_meta_list_token_stream<'a>(attrs: &'a [syn::Attribute], attribute_path: &std::string::String) -> &'a proc_macro2::TokenStream {
    let option_attribute = attrs.iter().find(|attr| {
        *attribute_path == {
            let mut stringified_path = quote::ToTokens::to_token_stream(&attr.path()).to_string();
            stringified_path.retain(|value| !value.is_whitespace());
            stringified_path
        }
    });
    let attribute = option_attribute.map_or_else(
        || {
            panic!("no {attribute_path}");
        },
        |attribute| attribute,
    );
    if let syn::Meta::List(value) = &attribute.meta { &value.tokens } else { panic!("&attribute.meta is not syn::Meta::List(value)") }
}
