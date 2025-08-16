pub fn panic_if_not_str(
    reference_ident: &proc_macro2::Ident,
    str_stringified: &str,
    proc_macro_name_ident_stringified: &str,
    supports_only_stringified: &str,
    attribute: &crate::error_occurence::ErrorOccurenceFieldAttribute,
) {
    assert!(
        reference_ident == str_stringified,
        "{proc_macro_name_ident_stringified} {} {supports_only_stringified} {str_stringified}, but got {reference_ident}",
        crate::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(
            attribute
        )
    );
}
