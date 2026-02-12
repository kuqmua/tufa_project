use syn::{Field, FieldMutability, Ident, Path, Type, TypePath, Visibility, token::Colon};
#[must_use]
pub fn code_occurence_syn_field() -> Field {
    use crate::gen_simple_syn_punctuated_punctuated::gen_simple_syn_punctuated_punctuated;
    Field {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(Ident::new("code_occurence", proc_macro2::Span::call_site())),
        colon_token: Some(Colon {
            spans: [proc_macro2::Span::call_site()],
        }),
        ty: Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: gen_simple_syn_punctuated_punctuated(&[
                    "error_occurence_lib",
                    "code_occurence",
                    "CodeOccurence",
                ]),
            },
        }),
    }
}
