use convert_case::{Case, Casing};
use gen_quotes::dq_ts;
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Fields, Ident, parse};
#[proc_macro_derive(OptimalPack)]
pub fn optimal_pack(input_ts: Ts) -> Ts {
    let di: DeriveInput = parse(input_ts).expect("a1d306de");
    let ident = &di.ident;
    let ident_dq_ts = dq_ts(&ident);
    let ident_test_optimal_pack = format!(
        "{}_test_optimal_pack",
        Casing::to_case(&ident.to_string(), Case::Snake)
    )
    .parse::<Ts2>()
    .expect("63213aaa");
    let gen_unnamed_ts = |first_ts: &dyn ToTokens, second_ts: &dyn ToTokens| quote! {(#first_ts, std::mem::align_of::<#second_ts>())};
    let gen_fi = |i: usize| Ident::new(&format!("field_{i}"), ident.span());
    let ts = match &di.data {
        Data::Struct(data) => {
            let fields = match &data.fields {
                Fields::Named(fields) => &fields.named,
                Fields::Unnamed(fields) => &fields.unnamed,
                Fields::Unit => {
                    return Ts::new();
                }
            };
            let ts = fields.iter().enumerate().map(|(i, field)| {
                gen_unnamed_ts(
                    &dq_ts(&field.ident.as_ref().map_or_else(|| gen_fi(i), Clone::clone)),
                    &field.ty,
                )
            });
            quote! {
                let fields = [#(#ts),*];
                for i in 1..fields.len() {
                    let (prev_name, prev_align) = fields.get(i - 1).expect("ba86f1dd");
                    let (curr_name, curr_align) = fields.get(i).expect("ba86f1dd");
                    assert!(
                        curr_align <= prev_align,
                        "Fields in `{}` are not optimally packed by alignment:\n\
                         field `{}` (alignment={}) > previous field `{}` (alignment={})",
                        #ident_dq_ts,
                        curr_name, curr_align,
                        prev_name, prev_align
                    );
                }
            }
        }
        Data::Enum(data_enum) => {
            let mut vars_ts = Vec::new();
            for var in &data_enum.variants {
                let var_ident = &var.ident;
                let var_ident_dq_ts = dq_ts(&var_ident);
                let fields = match &var.fields {
                    Fields::Named(fields) => &fields.named,
                    Fields::Unnamed(fields) => &fields.unnamed,
                    Fields::Unit => continue,
                };
                if fields.is_empty() {
                    continue;
                }
                let fields_ts = fields.iter().enumerate().map(|(i, field)| {
                    gen_unnamed_ts(
                        &dq_ts(&field.ident.as_ref().map_or_else(|| gen_fi(i), Clone::clone)),
                        &field.ty,
                    )
                });
                vars_ts.push(quote! {{
                    let fields = [#(#fields_ts),*];
                    for i in 1..fields.len() {
                        let (prev_name, prev_align) = fields.get(i-1).expect("2011de63");
                        let (curr_name, curr_align) = fields.get(i).expect("1272b618");
                        assert!(
                            curr_align <= prev_align,
                            "In enum `{}`, variant `{}`: field `{}` (alignment={}) > previous field `{}` (alignment={})",
                            #ident_dq_ts,
                            #var_ident_dq_ts,
                            curr_name, curr_align,
                            prev_name, prev_align
                        );
                    }
                }});
            }
            if vars_ts.is_empty() {
                return Ts::new();
            }
            quote! {#(#vars_ts)*}
        }
        Data::Union(_) => {
            return Ts::new();
        }
    };
    let generated = quote! {
        #[cfg(test)]
        mod #ident_test_optimal_pack {
            use super::*;
            #[test]
            fn #ident_test_optimal_pack() {
                #ts
            }
        }
    };
    // if ident == "" {
    //     println!("{generated}");
    // }
    generated.into()
}
