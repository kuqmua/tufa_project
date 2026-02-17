use syn::{Ident, Type, Visibility};
#[derive(Debug, Clone)]
pub struct SynFieldWrapper {
    pub field_ident: Ident,
    pub field_type: Type,
    pub field_visibility: Visibility,
}
