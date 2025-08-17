#[derive(Debug)]
pub enum HashMapValueType {
    Path { value_segments_stringified: std::string::String, value_vec_lifetime: Vec<crate::error_occurence::lifetime::Lifetime> },
    Reference { value_reference_ident: proc_macro2::Ident, value_lifetime_ident: proc_macro2::Ident },
}
