#[derive(Debug)]
pub enum HashMapKeyType {
    Path { key_segments_stringified: std::string::String, key_vec_lifetime: Vec<crate::error_occurence::lifetime::Lifetime> },
    Reference { key_reference_ident: proc_macro2::Ident, key_lifetime_ident: proc_macro2::Ident },
}
