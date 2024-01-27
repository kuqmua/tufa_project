pub fn panic_if_not_str(
    reference_ident: &proc_macro2::Ident,
    str_stringified: &str,
    proc_macro_name_ident_stringified: &str,
    supports_only_stringified: &str,
    attribute: &crate::error_occurence::named_attribute::NamedAttribute,
) {
    if let false = reference_ident == str_stringified {
        panic!("{proc_macro_name_ident_stringified} {} {supports_only_stringified} {str_stringified}, but got {reference_ident}", attribute.attribute_view_stringified());
    }
}
