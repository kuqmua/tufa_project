#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuportedEnumVariant {
    Named,
    Unnamed,
}

pub fn create(
    data_enum: &syn::DataEnum,
    proc_macro_name_ident_stringified: &std::string::String,
) -> SuportedEnumVariant {
    let mut all_equal: Option<SuportedEnumVariant> = None;
    assert!(!data_enum.variants.is_empty(), "{proc_macro_name_ident_stringified} enum variants are empty");
    let error_message = format!("{proc_macro_name_ident_stringified} {} enums where all variants are {}::{} or all variants are {}::{}",
        naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        naming_constants::SYN_FIELDS,
        naming_constants::SYN_FIELDS,
        naming_constants::NamedUpperCamelCase,
        naming_constants::UnnamedUpperCamelCase,
    );
    data_enum
        .variants
        .iter()
        .for_each(|variant| match &variant.fields {
            syn::Fields::Named(_) => match &all_equal {
                Some(supported_variant) => {
                    assert!(!(*supported_variant == SuportedEnumVariant::Unnamed), "{error_message}");
                }
                None => {
                    all_equal = Some(SuportedEnumVariant::Named);
                }
            },
            syn::Fields::Unnamed(_) => match &all_equal {
                Some(supported_variant) => {
                    assert!(!(*supported_variant == SuportedEnumVariant::Named), "{error_message}");
                }
                None => {
                    all_equal = Some(SuportedEnumVariant::Unnamed);
                }
            },
            syn::Fields::Unit => panic!("{error_message}"),
        });
    all_equal.map_or_else(|| {
        panic!("{proc_macro_name_ident_stringified} {} with enums where all variants are named or unnamed", naming_constants::SUPPORTS_ONLY_STRINGIFIED);
    }, |supported_enum_variant| supported_enum_variant)
}
