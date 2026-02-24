use gen_quotes::dq_ts;
use proc_macro::TokenStream as Ts;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericParam, Ident, Lifetime, parse, visit_mut::VisitMut};
#[proc_macro_derive(OptimalPack)]
pub fn optimal_pack(input_ts: Ts) -> Ts {
    let di: DeriveInput = parse(input_ts).expect("a1d306de");
    let ident = &di.ident;
    // let ident_dq_ts = dq_ts(&ident);
    // let gen_unnamed_ts = |first_ts: &dyn ToTokens, second_ts: &dyn ToTokens| quote! {(#first_ts, align_of::<#second_ts>())};
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
            let fields_len = fields.len();
            if fields_len == 1 {
                return Ts::new();
            }
            let align_of_ts = fields.iter().map(|field| {
                struct ReplaceLifetimes;
                impl VisitMut for ReplaceLifetimes {
                    fn visit_lifetime_mut(&mut self, i: &mut Lifetime) {
                        i.ident = Ident::new("static", i.ident.span());
                    }
                }
                let mut ft = field.ty.clone();
                let mut visitor = ReplaceLifetimes;
                visitor.visit_type_mut(&mut ft);
                quote!{align_of::<#ft>()}
            });
            let assertions_ts = fields.iter().enumerate().take(fields.len().checked_sub(1).expect("14b7aa69")).map(|(i, field)| {
                let i_plus_one = i.checked_add(1).expect("941a5489");
                let fi = &field.ident.as_ref().map_or_else(|| gen_fi(i), Clone::clone);
                let fi_next = &fields.get(i_plus_one).expect("ae113a45").ident.as_ref().map_or_else(|| gen_fi(i_plus_one), Clone::clone);
                let message_ts = dq_ts(&format!("In struct `{ident}` align_of field '{fi}' < align_of field '{fi_next}'. Field '{fi_next}' must be placed before '{fi}' for better memory alignment"));
                quote!{
                    assert!(
                        ALIGNMENTS[#i] >= ALIGNMENTS[#i_plus_one],
                        #message_ts,
                    );
                }
            });
            quote! {
                const ALIGNMENTS: [usize; #fields_len] = [#(#align_of_ts),*];
                #(#assertions_ts)*
            }
        }
        Data::Enum(_) | //data_enum
        //  => {
        //     let mut vars_ts = Vec::new();
        //     for var in &data_enum.variants {
        //         let var_ident = &var.ident;
        //         let var_ident_dq_ts = dq_ts(&var_ident);
        //         let fields = match &var.fields {
        //             Fields::Named(fields) => &fields.named,
        //             Fields::Unnamed(fields) => &fields.unnamed,
        //             Fields::Unit => continue,
        //         };
        //         if fields.is_empty() {
        //             continue;
        //         }
        //         let fields_ts = fields.iter().enumerate().map(|(i, field)| {
        //             gen_unnamed_ts(
        //                 &dq_ts(&field.ident.as_ref().map_or_else(|| gen_fi(i), Clone::clone)),
        //                 &field.ty,
        //             )
        //         });
        //         vars_ts.push(quote! {{
        //             let fields = [#(#fields_ts),*];
        //             for i in 1..fields.len() {
        //                 let (prev_name, prev_align) = fields.get(i-1).expect("2011de63");
        //                 let (curr_name, curr_align) = fields.get(i).expect("1272b618");
        //                 assert!(
        //                     curr_align <= prev_align,
        //                     "In enum `{}`, variant `{}`: field `{}` (alignment={}) > previous field `{}` (alignment={})",
        //                     #ident_dq_ts,
        //                     #var_ident_dq_ts,
        //                     curr_name, curr_align,
        //                     prev_name, prev_align
        //                 );
        //             }
        //         }});
        //     }
        //     if vars_ts.is_empty() {
        //         return Ts::new();
        //     }
        //     quote! {#(#vars_ts)*}
        // }
        Data::Union(_) => {
            return Ts::new();
        }
    };
    let generics = &di.generics;
    let has_type_params = generics
        .params
        .iter()
        .any(|p| matches!(p, GenericParam::Type(_) | GenericParam::Const(_)));
    if has_type_params {
        return Ts::new();
    }
    let generated = quote! {
        #[allow(unused_qualifications)]
        const _: () = {
            #ts
        };
    };
    // if ident == "" {
    //     println!("{generated}");
    // }
    generated.into()
}
