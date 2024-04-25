#[allow(clippy::enum_variant_names)]
// #[derive(strum_macros::EnumIter, strum_macros::Display)]
#[derive(Debug, Clone, Copy)]
pub enum NamedAttribute {
    EoDisplay,
    EoDisplayWithSerializeDeserialize,
    EoDisplayForeignType,
    EoDisplayForeignTypeWithSerializeDeserialize,
    EoErrorOccurence,
    EoVecDisplay,
    EoVecDisplayWithSerializeDeserialize,
    EoVecDisplayForeignType,
    EoVecDisplayForeignTypeWithSerializeDeserialize,
    EoVecErrorOccurence,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize,
    EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence,
    EoHashMapKeyDisplayForeignTypeValueDisplay,
    EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize,
    EoHashMapKeyDisplayForeignTypeValueDisplayForeignType,
    EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize,
    EoHashMapKeyDisplayForeignTypeValueErrorOccurence,
}

impl std::str::FromStr for NamedAttribute {
    type Err = std::string::String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "eo_display" => Ok(Self::EoDisplay),
            "eo_display_with_serialize_deserialize" => Ok(Self::EoDisplayWithSerializeDeserialize),
            "eo_display_foreign_type" => Ok(Self::EoDisplayForeignType),
            "eo_display_foreign_type_with_serialize_deserialize" => Ok(Self::EoDisplayForeignTypeWithSerializeDeserialize),
            "eo_error_occurence" => Ok(Self::EoErrorOccurence),
            "eo_vec_display" => Ok(Self::EoVecDisplay),
            "eo_vec_display_with_serialize_deserialize" => Ok(Self::EoVecDisplayWithSerializeDeserialize),
            "eo_vec_display_foreign_type" => Ok(Self::EoVecDisplayForeignType),
            "eo_vec_display_foreign_type_with_serialize_deserialize" => Ok(Self::EoVecDisplayForeignTypeWithSerializeDeserialize),
            "eo_vec_error_occurence" => Ok(Self::EoVecErrorOccurence),
            "eo_hashmap_key_display_with_serialize_deserialize_value_display" => Ok(Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay),
            "eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize" => Ok(Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize),
            "eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type" => Ok(Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType),
            "eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize" => Ok(Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize),
            "eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence" => Ok(Self::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence),
            "eo_hashmap_key_display_foreign_type_value_display" => Ok(Self::EoHashMapKeyDisplayForeignTypeValueDisplay),
            "eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize" => Ok(Self::EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize),
            "eo_hashmap_key_display_foreign_type_value_display_foreign_type" => Ok(Self::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType),
            "eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize" => Ok(Self::EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize),
            "eo_hashmap_key_display_foreign_type_value_error_occurence" => Ok(Self::EoHashMapKeyDisplayForeignTypeValueErrorOccurence),
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
                                        value.to_string()
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
                                        value.to_string()
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

impl std::string::ToString for NamedAttribute {
    fn to_string(&self) -> std::string::String {
        match self {
            Self::EoDisplay => std::string::String::from("eo_display"),
            Self::EoDisplayWithSerializeDeserialize => std::string::String::from("eo_display_with_serialize_deserialize"),
            Self::EoDisplayForeignType => std::string::String::from("eo_display_foreign_type"),
            Self::EoDisplayForeignTypeWithSerializeDeserialize => std::string::String::from("eo_display_foreign_type_with_serialize_deserialize"),
            Self::EoErrorOccurence => std::string::String::from("eo_error_occurence"),
            Self::EoVecDisplay => std::string::String::from("eo_vec_display"),
            Self::EoVecDisplayWithSerializeDeserialize => std::string::String::from("eo_vec_display_with_serialize_deserialize"),
            Self::EoVecDisplayForeignType => std::string::String::from("eo_vec_display_foreign_type"),
            Self::EoVecDisplayForeignTypeWithSerializeDeserialize => std::string::String::from("eo_vec_display_foreign_type_with_serialize_deserialize"),
            Self::EoVecErrorOccurence => std::string::String::from("eo_vec_error_occurence"),
            Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay => std::string::String::from("eo_hashmap_key_display_with_serialize_deserialize_value_display"),
            Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize => std::string::String::from("eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize"),
            Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType => std::string::String::from("eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type"),
            Self::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize => std::string::String::from("eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize"),
            Self::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence => std::string::String::from("eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence"),
            Self::EoHashMapKeyDisplayForeignTypeValueDisplay => std::string::String::from("eo_hashmap_key_display_foreign_type_value_display"),
            Self::EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize => std::string::String::from("eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize"),
            Self::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType => std::string::String::from("eo_hashmap_key_display_foreign_type_value_display_foreign_type"),
            Self::EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize => std::string::String::from("eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize"),
            Self::EoHashMapKeyDisplayForeignTypeValueErrorOccurence => std::string::String::from("eo_hashmap_key_display_foreign_type_value_error_occurence"),
        }
    }
}

impl NamedAttribute {
    pub fn to_attribute_view_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!("#[{}]", self.to_string());
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
    pub fn attribute_view_stringified(&self) -> std::string::String {
        self.to_string()
    }
}

pub fn attribute_view(attribute: &str) -> std::string::String {
    format!("attribute #[{attribute}]")
}
