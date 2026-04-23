use gen_quotes::dq_ts;
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use std::iter::repeat_n;
use syn::{
    Data, DeriveInput, Field, Fields, GenericParam, Ident, Lifetime, parse, punctuated::Punctuated,
    token::Comma, visit_mut::VisitMut,
};
struct ReplaceLts;
impl VisitMut for ReplaceLts {
    fn visit_lifetime_mut(&mut self, i: &mut Lifetime) {
        i.ident = Ident::new("static", i.ident.span());
    }
}
#[allow(clippy::single_call_fn)] // isolated helper keeps lifetime rewrite reusable when alignment logic grows
fn field_ty_with_static_lts(field: &Field) -> syn::Type {
    let mut ft = field.ty.clone();
    let mut visitor = ReplaceLts;
    visitor.visit_type_mut(&mut ft);
    ft
}
#[allow(clippy::single_call_fn)] // isolated helper keeps align token generation reusable and explicit
fn gen_align_of_ts(field: &Field) -> Ts2 {
    let ft = field_ty_with_static_lts(field);
    quote! {align_of::<#ft>()}
}
#[proc_macro_derive(Optml)]
pub fn optml(input_ts: Ts) -> Ts {
    let gen_alignments_ident_ts =
        |i: usize| format!("alignments_{i}").parse::<Ts2>().expect("5a0bb723");
    let di: DeriveInput = parse(input_ts).expect("a1d306de");
    let ident = &di.ident;
    let gen_fi = |i: usize| Ident::new(&format!("field_{i}"), ident.span());
    let gen_assertions_ts = |fields: &Punctuated<Field, Comma>,
                             alignments_ts: &dyn ToTokens,
                             kind_name: &'static str,
                             variant: Option<&Ident>|
     -> Option<Ts2> {
        let fields_len = fields.len();
        if fields_len <= 1 {
            return None;
        }
        let align_of_ts = fields.iter().map(&gen_align_of_ts);
        let variant_info = variant.map_or_else(String::new, |variant_ident| {
            format!("variant '{variant_ident}' ")
        });
        let gen_or_copy_ident = |field: &Field, idx: usize| {
            field
                .ident
                .as_ref()
                .map_or_else(|| gen_fi(idx), Clone::clone)
        };
        let assertions_ts = fields
            .iter()
            .zip(fields.iter().skip(1))
            .enumerate()
            .map(|(i, (field, next_field))| {
            let i_plus_one = i.saturating_add(1);
            let fi = gen_or_copy_ident(field, i);
            let fi_next = gen_or_copy_ident(next_field, i_plus_one);
            let msg_ts = dq_ts(&format!(
                "In {kind_name} '{ident}' {variant_info}align_of field '{fi}' < align_of field '{fi_next}'. Field '{fi_next}' must be placed before '{fi}' for better memory alignment",
            ));
            quote!{
                assert!(
                    #alignments_ts[#i] >= #alignments_ts[#i_plus_one],
                    #msg_ts,
                );
            }
        });
        Some(quote! {
            let #alignments_ts: [usize; #fields_len] = [#(#align_of_ts),*];
            #(#assertions_ts)*
        })
    };
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
            if fields_len <= 1 {
                return Ts::new();
            }
            match gen_assertions_ts(fields, &quote! {alignments}, "struct", None) {
                Some(v) => v,
                None => {
                    return Ts::new();
                }
            }
        }
        Data::Enum(data_enum) => {
            let mut vars_ts = Vec::new();
            for (i, var) in data_enum.variants.iter().enumerate() {
                let var_ident = &var.ident;
                let fields = match &var.fields {
                    Fields::Named(fields) => &fields.named,
                    Fields::Unnamed(fields) => &fields.unnamed,
                    Fields::Unit => continue,
                };
                let fields_len = fields.len();
                if fields_len <= 1 {
                    continue;
                }
                if let Some(v) =
                    gen_assertions_ts(fields, &gen_alignments_ident_ts(i), "enum", Some(var_ident))
                {
                    vars_ts.push(v);
                }
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
    let generics = &di.generics;
    let (impl_generics, ty_generics, wh_clause) = generics.split_for_impl();
    let has_only_lts = generics
        .params
        .iter()
        .all(|p| matches!(p, GenericParam::Lifetime(_)));
    let (impl_ts, ty_ts) = if has_only_lts && !generics.params.is_empty() {
        let lts_count = generics.params.len();
        let undrscrs = repeat_n(quote! {'_}, lts_count);
        let new_ty_generics = quote! {<#(#undrscrs),*>};
        (quote! {}, new_ty_generics)
    } else {
        (quote! { #impl_generics }, quote! { #ty_generics })
    };
    let const_name_ts = quote! {_OPTIMAL_PACK_CHECK};
    let impl_check_ts = quote! {
        #[allow(unused_qualifications)]
        impl #impl_ts #ident #ty_ts #wh_clause {
            const #const_name_ts: () = {
                #ts
            };
        }
    };
    let has_type_prms = generics
        .params
        .iter()
        .any(|p| matches!(p, GenericParam::Type(_) | GenericParam::Const(_)));
    let generated = if has_type_prms {
        quote! {#impl_check_ts}
    } else {
        quote! {
            #impl_check_ts
            const _: () = #ident::#const_name_ts;
        }
    };
    generated.into()
}
