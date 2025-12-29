#[derive(Debug, Clone)]
pub struct SynFieldWrapper {
    pub field_visibility: syn::Visibility,
    pub field_ident: syn::Ident,
    pub field_type: syn::Type,
}
