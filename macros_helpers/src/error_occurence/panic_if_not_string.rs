pub fn panic_if_not_string(
    segments_stringified: &str,
    std_string_string_stringified: &str,
    proc_macro_name_ident_stringified: &str,
    supports_only_stringified: &str,
    as_std_collections_hashmap_key_type_stringified: &str,
    attribute: &crate::error_occurence::ErrorOccurenceFieldAttribute,
) {
    assert!(
        segments_stringified == std_string_string_stringified,
        "{proc_macro_name_ident_stringified} {} {supports_only_stringified} {std_string_string_stringified} {as_std_collections_hashmap_key_type_stringified} (hashmap key must be string for deserialization)",
        crate::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(
            attribute
        )
    );
}
