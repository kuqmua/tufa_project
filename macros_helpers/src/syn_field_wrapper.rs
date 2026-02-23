use optimal_pack::OptimalPack;
use syn::{Ident, Type, Visibility};
#[derive(Debug, Clone, OptimalPack)]
pub struct SynFieldWrapper {
    pub ident: Ident,
    pub type0: Type,
    pub vis: Visibility,
}
