#[proc_macro_derive(SvgComponent)]
pub fn svg_component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).expect("syn::parse(input) failed");
    let ident = &syn_derive_input.ident;
    // let get_html_variants: TokenStream;
    // let get_class_variants: TokenStream;
    //to tired to think how to do it without .clone()
    let get_html_variants = match syn_derive_input.data.clone() {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| {
            let variant_ident = v.ident;
            let module = syn::Ident::new(
                &generate_quotes::naming_conventions::ToSnakeCaseStringified::new(
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
        _ => panic!("works only on enums"),
    };
    let get_class_variants = match syn_derive_input.data {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| {
            let variant_ident = v.ident;
            let class = &format!(
                "anticon-{}",
                generate_quotes::naming_conventions::ToSnakeCaseStringified::new(
                    &variant_ident.to_string()
                )
            );
            quote::quote! {
                SvgType::#variant_ident(_) => AttrValue::Static(#class)
            }
        }),
        _ => panic!("works only on enums"),
    };
    let generated = quote::quote! {
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
    generated.into()
}
