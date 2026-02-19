use crate::gen_simple_syn_punct::gen_simple_syn_punct;
use syn::{Field, FieldMutability, Ident, Path, Type, TypePath, Visibility, token::Colon};
#[must_use]
pub fn loc_syn_field() -> Field {
    Field {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(Ident::new("loc", proc_macro2::Span::call_site())),
        colon_token: Some(Colon {
            spans: [proc_macro2::Span::call_site()],
        }),
        ty: Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: gen_simple_syn_punct(&["location_lib", "loc", "Loc"]),
            },
        }),
    }
}
