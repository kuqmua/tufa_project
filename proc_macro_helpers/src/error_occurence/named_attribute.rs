#[allow(clippy::enum_variant_names)]
// #[derive(strum_macros::EnumIter, strum_macros::Display)]
#[derive(Debug, Clone, Copy)]
pub enum NamedAttribute {
    EoDisplay,
    EoDisplayWithSerializeDeserialize,
    EoToStdStringString,
    EoToStdStringStringWithSerializeDeserialize,
    EoErrorOccurence,
    EoVecDisplay,
    EoVecDisplayWithSerializeDeserialize,
    EoVecToStdStringString,
    EoVecToStdStringStringWithSerializeDeserialize,
    EoVecErrorOccurence,
    // EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay,
    // EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize,
    // EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringString,
    // EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringStringWithSerializeDeserialize,
    // EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence,
    // EoHashMapKeyToStdStringStringValueDisplay,
    // EoHashMapKeyToStdStringStringValueDisplayWithSerializeDeserialize,
    // EoHashMapKeyToStdStringStringValueToStdStringString,
    // EoHashMapKeyToStdStringStringValueToStdStringStringWithSerializeDeserialize,
    // EoHashMapKeyToStdStringStringValueErrorOccurence,
}

impl std::str::FromStr for NamedAttribute {
    type Err = std::string::String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "eo_display" => Ok(Self::EoDisplay),
            "eo_display_with_serialize_deserialize" => Ok(Self::EoDisplayWithSerializeDeserialize),
            "eo_to_std_string_string" => Ok(Self::EoToStdStringString),
            "eo_to_std_string_string_with_serialize_deserialize" => Ok(Self::EoToStdStringStringWithSerializeDeserialize),
            "eo_error_occurence" => Ok(Self::EoErrorOccurence),
            "eo_vec_display" => Ok(Self::EoVecDisplay),
            "eo_vec_display_with_serialize_deserialize" => Ok(Self::EoVecDisplayWithSerializeDeserialize),
            "eo_vec_to_std_string_string" => Ok(Self::EoVecToStdStringString),
            "eo_vec_to_std_string_string_with_serialize_deserialize" => Ok(Self::EoVecToStdStringStringWithSerializeDeserialize),
            "eo_vec_error_occurence" => Ok(Self::EoVecErrorOccurence),
            // "eo_hashmap_key_display_with_serialize_deserialize_value_display" => Ok(Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay),
            // "eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize" => Ok(Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize),
            // "eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string" => Ok(Self::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringString),
            // "eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string_with_serialize_deserialize" => Ok(Self::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringStringWithSerializeDeserialize),
            // "eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence" => Ok(Self::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence),
            // "eo_hashmap_key_to_std_string_string_value_display" => Ok(Self::EoHashMapKeyToStdStringStringValueDisplay),
            // "eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize" => Ok(Self::EoHashMapKeyToStdStringStringValueDisplayWithSerializeDeserialize),
            // "eo_hashmap_key_to_std_string_string_value_to_std_string_string" => Ok(Self::EoHashMapKeyToStdStringStringValueToStdStringString),
            // "eo_hashmap_key_to_std_string_string_value_to_std_string_string_with_serialize_deserialize" => Ok(Self::EoHashMapKeyToStdStringStringValueToStdStringStringWithSerializeDeserialize),
            // "eo_hashmap_key_to_std_string_string_value_error_occurence" => Ok(Self::EoHashMapKeyToStdStringStringValueErrorOccurence),
            _ => Err(format!("unsupported NamedAttribute: {value}"))
        }
    }
}

impl TryFrom<&syn::Field> for NamedAttribute {
    type Error = std::string::String;
    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        let mut error_occurence_attribute: Option<Self> = None;
        for element in &value.attrs {
            if element.path().segments.len() == 1 {
                match element.path().segments.first() {
                    Some(value) => {
                        if let Ok(value) = {
                            use std::str::FromStr;
                            Self::from_str(&value.ident.to_string())
                        } {
                            match error_occurence_attribute {
                                Some(value) => {
                                    return Err(format!(
                                        "duplicated attributes {} are not supported",
                                        proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&value)
                                    ));
                                }
                                None => {
                                    error_occurence_attribute = Some(value);
                                }
                            }
                        }
                    }
                    None => {
                        return Err(std::string::String::from(
                            "element.path().segments.first() is None",
                        ));
                    }
                }
            }
        }
        error_occurence_attribute.map_or_else(|| Err(std::string::String::from("supported attribute not found")), |value| Ok(value))
    }
}

impl TryFrom<&&syn::Field> for NamedAttribute {
    type Error = std::string::String;
    fn try_from(value: &&syn::Field) -> Result<Self, Self::Error> {
        let mut error_occurence_attribute: Option<Self> = None;
        for element in &value.attrs {
            if element.path().segments.len() == 1 {
                match element.path().segments.first() {
                    Some(value) => {
                        if let Ok(value) = {
                            use std::str::FromStr;
                            Self::from_str(&value.ident.to_string())
                        } {
                            match error_occurence_attribute {
                                Some(value) => {
                                    return Err(format!(
                                        "duplicated attributes {} are not supported",
                                        proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&value)
                                    ));
                                }
                                None => {
                                    error_occurence_attribute = Some(value);
                                }
                            }
                        }
                    }
                    None => {
                        return Err(std::string::String::from(
                            "element.path().segments.first() is None",
                        ));
                    }
                }
            }
        }
        error_occurence_attribute.map_or_else(|| Err(std::string::String::from("supported attribute not found")), |value| Ok(value))
    }
}

impl proc_macro_common::attribute_ident_stringified::AttributeIdentStringified for NamedAttribute {
    fn attribute_ident_stringified(&self) -> &str {
        match self {
            Self::EoDisplay => "eo_display",
            Self::EoDisplayWithSerializeDeserialize => "eo_display_with_serialize_deserialize",
            Self::EoToStdStringString => "eo_to_std_string_string",
            Self::EoToStdStringStringWithSerializeDeserialize => "eo_to_std_string_string_with_serialize_deserialize",
            Self::EoErrorOccurence => "eo_error_occurence",
            Self::EoVecDisplay => "eo_vec_display",
            Self::EoVecDisplayWithSerializeDeserialize => "eo_vec_display_with_serialize_deserialize",
            Self::EoVecToStdStringString => "eo_vec_to_std_string_string",
            Self::EoVecToStdStringStringWithSerializeDeserialize => "eo_vec_to_std_string_string_with_serialize_deserialize",
            Self::EoVecErrorOccurence => "eo_vec_error_occurence",
            // Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay => "eo_hashmap_key_display_with_serialize_deserialize_value_display",
            // Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize => "eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize",
            // Self::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringString => "eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string",
            // Self::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringStringWithSerializeDeserialize => "eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string_with_serialize_deserialize",
            // Self::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence => "eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence",
            // Self::EoHashMapKeyToStdStringStringValueDisplay => "eo_hashmap_key_to_std_string_string_value_display",
            // Self::EoHashMapKeyToStdStringStringValueDisplayWithSerializeDeserialize => "eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize",
            // Self::EoHashMapKeyToStdStringStringValueToStdStringString => "eo_hashmap_key_to_std_string_string_value_to_std_string_string",
            // Self::EoHashMapKeyToStdStringStringValueToStdStringStringWithSerializeDeserialize => "eo_hashmap_key_to_std_string_string_value_to_std_string_string_with_serialize_deserialize",
            // Self::EoHashMapKeyToStdStringStringValueErrorOccurence => "eo_hashmap_key_to_std_string_string_value_error_occurence",
        }
    }
}

impl NamedAttribute {
    pub fn to_attribute_view_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!("#[{}]", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(self));
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub fn attribute_view(attribute: &str) -> std::string::String {
    format!("attribute #[{attribute}]")
}

/////////////////////////////////////
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