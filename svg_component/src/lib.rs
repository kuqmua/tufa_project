#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

#[proc_macro_derive(SvgComponent)]
pub fn svg_component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let ast: syn::DeriveInput = syn::parse(input).expect("SvgComponent syn::parse(input) failed");
    let ident = &ast.ident;
    // let get_html_variants: TokenStream;
    // let get_class_variants: TokenStream;
    //to tired to think how to do it without .clone()
    let get_html_variants = match ast.data.clone() {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| {
            let variant_ident = v.ident;
            let module = syn::Ident::new(
                &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
                    &variant_ident.to_string(),
                ),
                ident.span(),
            );
            quote::quote! {
                SvgType::#variant_ident(svg_props) => html! {
                    <crate::components::ant_design::svg::#module::#variant_ident
                      height={svg_props.height.clone()}
                      width={svg_props.width.clone()}
                      fill={svg_props.fill.clone()}
                      spin={svg_props.spin}
                      rotate={svg_props.rotate.clone()}
                      theme={svg_props.theme.clone()}
                    />
                }
            }
        }),
        _ => panic!("SvgComponent works only on enums"),
    };
    let get_class_variants = match ast.data {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| {
            let variant_ident = v.ident;
            let class = &format!(
                "anticon-{}",
                proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
                    &variant_ident.to_string()
                )
            );
            quote::quote! {
                SvgType::#variant_ident(_) => AttrValue::Static(#class)
            }
        }),
        _ => panic!("SvgComponent works only on enums"),
    };
    let gen = quote::quote! {
        impl SvgComponent for #ident {
            fn get_html(&self) -> Html {
                match self {
                    #(#get_html_variants),*
                }
            }
            fn get_class(&self) -> AttrValue {
                match *self {
                    #(#get_class_variants),*
                }
            }
        }
    };
    gen.into()
}
