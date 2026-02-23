use gen_quotes::dq_ts;
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, parse};
use naming::ToTokensToScStr;
#[proc_macro_derive(OptimalPack)]
pub fn optimal_pack(input_ts: Ts) -> Ts {
    let di: DeriveInput = parse(input_ts).expect("a1d306de");
    let ident = &di.ident;
    let fields = match &di.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(fields) => &fields.unnamed,
            Fields::Unit => {
                return Ts::new();
            }
        },
        Data::Enum(_) | Data::Union(_) => {
            return Ts::new();
        }
    };
    let ident_test_optimal_pack = format!("{}_test_optimal_pack", ToTokensToScStr::case(ident))
        .parse::<Ts2>()
        .expect("63213aaa");
    let mut field_types = Vec::new();
    let mut field_idents = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        field_types.push(&field.ty);
        field_idents.push(dq_ts(&field.ident.as_ref().map_or_else(
            || Ident::new(&format!("field_{i}"), ident.span()),
            Clone::clone,
        )));
    }
    let generated = quote! {
        #[cfg(test)]
        mod #ident_test_optimal_pack {
            use super::*;
            #[test]
            fn #ident_test_optimal_pack() {
                let alignments = [#(std::mem::align_of::<#field_types>()),*];
                let field_names = [#(#field_idents),*];
                for i in 1..alignments.len() {
                    let alignment_i = alignments.get(i).expect("ba86f1dd");
                    let alignment_i_minus_one = alignments.get(i-1).expect("f8eef54f");
                    assert!(
                        alignment_i <=alignment_i_minus_one,
                        "Fields in `{}` are not optimally packed by alignment:\n\
                         field `{}` (alignment={}) > previous field `{}` (alignment={})",
                        stringify!(#ident),
                        field_names.get(i).expect("65637984"), alignment_i,
                        field_names.get(i-1).expect("2e45585a"), alignment_i_minus_one
                    );
                }
            }
        }
    };
    println!("{generated}");
    generated.into()
}
