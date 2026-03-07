use optml::Optml;
use syn::{Ident, Type, Visibility};
#[derive(Debug, Clone, Optml)]
pub struct SynField {
    pub ident: Ident,
    pub type0: Type,
    pub vis: Visibility,
}
