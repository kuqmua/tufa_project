#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy)]
pub enum ErrorOccurenceFieldAttribute {
    EoToStdStringString,
    EoToStdStringStringSerializeDeserialize,
    EoErrorOccurence,
    EoVecToStdStringString,
    EoVecToStdStringStringSerializeDeserialize,
    EoVecErrorOccurence,
    EoHashMapKeyStdStringStringValueToStdStringString,
    EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize,
    EoHashMapKeyStdStringStringValueErrorOccurence,
}
impl std::str::FromStr for ErrorOccurenceFieldAttribute {
    type Err = ();
    fn from_str(value: &std::primitive::str) -> Result<Self, Self::Err> {
        if value == "eo_to_std_string_string" {
            Ok(Self::EoToStdStringString)
        }
        else if value == "eo_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoToStdStringStringSerializeDeserialize)
        }
        else if value == "eo_error_occurence" {
            Ok(Self::EoErrorOccurence)
        }
        else if value == "eo_vec_to_std_string_string" {
            Ok(Self::EoVecToStdStringString)
        }
        else if value == "eo_vec_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoVecToStdStringStringSerializeDeserialize)
        }
        else if value == "eo_vec_error_occurence" {
            Ok(Self::EoVecErrorOccurence)
        }
        else if value == "eo_hashmap_key_std_string_string_value_to_std_string_string" {
            Ok(Self::EoHashMapKeyStdStringStringValueToStdStringString)
        }
        else if value == "eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize)
        }
        else if value == "eo_hashmap_key_std_string_string_value_error_occurence" {
            Ok(Self::EoHashMapKeyStdStringStringValueErrorOccurence)
        }
        else {
            Err(())
        }
    }
}
impl std::convert::From<&syn::Field> for ErrorOccurenceFieldAttribute {
    fn from(value: &syn::Field) -> Self {
        let mut option_attribute = None;
        for attr in &value.attrs {
            if attr.path().segments.len() == 1 {
                let first_segment_ident = &attr.path().segments.first().expect("no first value in punctuated").ident;
                if let Ok(value) = {
                    use std::str::FromStr;
                    ErrorOccurenceFieldAttribute::from_str(&first_segment_ident.to_string())
                } {
                    if option_attribute.is_some() {
                        panic!("two or more supported attributes!");
                    }
                    else {
                        option_attribute = Some(value);
                    }
                }
            }//other attributes are not for this proc_macro
        };
        option_attribute.unwrap_or_else(|| panic!(
            "option attribute {}",
            naming_constants::IS_NONE_STRINGIFIED
        ))
    }
}
impl proc_macro_common::attribute_ident_stringified::AttributeIdentStringified for ErrorOccurenceFieldAttribute {
    fn attribute_ident_stringified(&self) -> &str {
        match self {
            Self::EoToStdStringString => "eo_to_std_string_string",
            Self::EoToStdStringStringSerializeDeserialize => "eo_to_std_string_string_serialize_deserialize",
            Self::EoErrorOccurence => "eo_error_occurence",
            Self::EoVecToStdStringString => "eo_vec_to_std_string_string",
            Self::EoVecToStdStringStringSerializeDeserialize => "eo_vec_to_std_string_string_serialize_deserialize",
            Self::EoVecErrorOccurence => "eo_vec_error_occurence",
            Self::EoHashMapKeyStdStringStringValueToStdStringString => "eo_hashmap_key_std_string_string_value_to_std_string_string",
            Self::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => "eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize",
            Self::EoHashMapKeyStdStringStringValueErrorOccurence => "eo_hashmap_key_std_string_string_value_error_occurence",
        }
    }
}
impl ErrorOccurenceFieldAttribute {
    pub fn to_attribute_view_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!("#[{}]", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(self));
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}
pub fn attribute_view(attribute: &str) -> std::string::String {
    format!("attribute #[{attribute}]")
}