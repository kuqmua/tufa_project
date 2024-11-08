#[derive(Debug)]
pub enum VecElementType {
    Path { element_path: std::string::String, vec_lifetime: Vec<crate::error_occurence::lifetime::Lifetime> },
    Reference { reference_ident: proc_macro2::Ident, lifetime_ident: proc_macro2::Ident },
}
