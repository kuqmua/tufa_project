use syn::{Ident, Type, Visibility};
#[derive(Debug, Clone)]
pub struct SynFieldWrapper {
    pub ident: Ident,
    pub type0: Type,
    pub vis: Visibility,
}
