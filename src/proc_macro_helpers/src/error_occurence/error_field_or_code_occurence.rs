pub enum ErrorFieldOrCodeOccurence {
    ErrorField {
        attribute: crate::error_occurence::named_attribute::NamedAttribute,
        supported_container: crate::error_occurence::supported_container::SupportedContainer,
    },
    CodeOccurence {
        field_type: std::string::String,
        vec_lifetime: Vec<crate::error_occurence::lifetime::Lifetime>,
    },
}
