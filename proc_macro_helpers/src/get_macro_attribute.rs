pub fn get_macro_attribute<'a>(
    attrs: &'a [syn::Attribute],
    attribute_path: &std::string::String,
    proc_macro_name_ident_stringified: &std::string::String,
) -> &'a syn::Attribute {
    let option_attribute = attrs.iter().find(|attr| {
        *attribute_path == {
            let mut stringified_path = quote::ToTokens::to_token_stream(&attr.path()).to_string();
            stringified_path.retain(|value| !value.is_whitespace());
            stringified_path
        }
    });
    option_attribute.map_or_else(|| {
        panic!("{proc_macro_name_ident_stringified} no {attribute_path}");
    }, |attribute| attribute)
}
