#[derive(Debug)]
pub enum ErrorFieldOrCodeOccurence {
    ErrorField {
        attribute: crate::error_occurence::error_occurence_field_attribute::ErrorOccurenceFieldAttribute,
        supported_container: crate::error_occurence::supported_container::SupportedContainer,
    },
    CodeOccurence {
        field_type: std::string::String,
        vec_lifetime: Vec<crate::error_occurence::lifetime::Lifetime>,
    },
}
