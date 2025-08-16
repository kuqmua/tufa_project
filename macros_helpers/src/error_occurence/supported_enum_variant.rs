#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuportedEnumVariant {
    Named,
    Unnamed,
}
impl SuportedEnumVariant {
    pub fn new_or_panic(data_enum: &syn::DataEnum) -> Self {
        let mut all_equal: Option<Self> = None;
        assert!(!data_enum.variants.is_empty(), "enum variants are empty");
        let error_message = format!(
            "{} enums where all variants are {}::{} or all variants are {}::{}",
            naming::SUPPORTS_ONLY_STRINGIFIED,
            naming::SYN_FIELDS,
            naming::SYN_FIELDS,
            naming::NamedUpperCamelCase,
            naming::UnnamedUpperCamelCase,
        );
        data_enum
            .variants
            .iter()
            .for_each(|variant| match &variant.fields {
                syn::Fields::Named(_) => match &all_equal {
                    Some(supported_variant) => {
                        assert!(!(*supported_variant == Self::Unnamed), "{error_message}");
                    }
                    None => {
                        all_equal = Some(Self::Named);
                    }
                },
                syn::Fields::Unnamed(_) => match &all_equal {
                    Some(supported_variant) => {
                        assert!(!(*supported_variant == Self::Named), "{error_message}");
                    }
                    None => {
                        all_equal = Some(Self::Unnamed);
                    }
                },
                syn::Fields::Unit => panic!("{error_message}"),
            });
        all_equal.map_or_else(
            || {
                panic!(
                    "{} with enums where all variants are named or unnamed",
                    naming::SUPPORTS_ONLY_STRINGIFIED
                );
            },
            |supported_enum_variant| supported_enum_variant,
        )
    }
}
