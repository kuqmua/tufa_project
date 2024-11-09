#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuportedEnumVariant {
    Named,
    Unnamed,
}
impl SuportedEnumVariant {
    pub fn new_or_panic(data_enum: &syn::DataEnum) -> SuportedEnumVariant {
        let mut all_equal: Option<SuportedEnumVariant> = None;
        assert!(!data_enum.variants.is_empty(), "enum variants are empty");
        let error_message = format!(
            "{} enums where all variants are {}::{} or all variants are {}::{}",
            naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
            naming_conventions::SYN_FIELDS,
            naming_conventions::SYN_FIELDS,
            naming_conventions::NamedUpperCamelCase,
            naming_conventions::UnnamedUpperCamelCase,
        );
        data_enum.variants.iter().for_each(|variant| match &variant.fields {
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
        all_equal.map_or_else(
            || {
                panic!("{} with enums where all variants are named or unnamed", naming_conventions::SUPPORTS_ONLY_STRINGIFIED);
            },
            |supported_enum_variant| supported_enum_variant,
        )
    }
}