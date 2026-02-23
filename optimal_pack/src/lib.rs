use gen_quotes::dq_ts;
use naming::ToTokensToScStr;
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, parse};
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
    let ts = fields.iter().enumerate().map(|(i, field)| {
        let fi_dq_ts = dq_ts(&field.ident.as_ref().map_or_else(
            || Ident::new(&format!("field_{i}"), ident.span()),
            Clone::clone,
        ));
        let ft = &field.ty;
        quote! {
            (#fi_dq_ts, std::mem::align_of::<#ft>())
        }
    });
    let generated = quote! {
        #[cfg(test)]
        mod #ident_test_optimal_pack {
            use super::*;
            #[test]
            fn #ident_test_optimal_pack() {
                let fields = [#(#ts),*];
                for i in 1..fields.len() {
                    let (prev_name, prev_align) = fields.get(i - 1).expect("ba86f1dd");
                    let (curr_name, curr_align) = fields.get(i).expect("ba86f1dd");
                    assert!(
                        curr_align <= prev_align,
                        "Fields in `{}` are not optimally packed by alignment:\n\
                         field `{}` (alignment={}) > previous field `{}` (alignment={})",
                        stringify!(ServerPortEr),
                        curr_name, curr_align,
                        prev_name, prev_align
                    );
                }
            }
        }
    };
    // println!("{generated}");
    generated.into()
}
