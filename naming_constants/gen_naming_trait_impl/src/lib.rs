// #[proc_macro_derive(GenNamingTraitImpl)]
// pub fn gen_naming_trait_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
//     proc_macro_common::panic_location::panic_location();
//     let proc_macro_name_upper_camel_case = "GenNamingTraitImpl";
//     let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
//         panic!(
//             "{proc_macro_name_upper_camel_case} {}: {e}",
//             proc_macro_common::global_variables::hardcode::AST_PARSE_FAILED
//         )
//     });
//     let ident = &ast.ident;
//     let gen = quote::quote!{

//     };
//     // if ident == "" {
//     //     println!("{gen}");
//     //     println!("----------");//todo for some reason gen duplicates for few times - find out why and fix
//     // }
//     gen.into()
// }