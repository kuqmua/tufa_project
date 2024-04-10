#[proc_macro_derive(GenNamingTraitImpl)]
pub fn gen_naming_trait_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenNamingTraitImpl";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {e}",
            proc_macro_common::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let ident_stringified = ident.to_string();
    let (ident_upper_camel_case_quotes_token_stream, ident_upper_camel_case_token_stream) = {
        let ident_upper_camel_case_stringified = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&ident_stringified);
        let ident_upper_camel_case_quotes_token_stream = {
            let value = format!("\"{ident_upper_camel_case_stringified}\"");
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let ident_upper_camel_case_token_stream = {
            ident_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{ident_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        (
            ident_upper_camel_case_quotes_token_stream,
            ident_upper_camel_case_token_stream
        )
    };
    let (ident_snake_case_quotes_token_stream, ident_snake_case_token_stream) = {
        let ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&ident_stringified);
        let ident_snake_case_quotes_token_stream = {
            let value = format!("\"{ident_snake_case_stringified}\"");
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let ident_snake_case_token_stream = {
            ident_snake_case_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{ident_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        (
            ident_snake_case_quotes_token_stream,
            ident_snake_case_token_stream
        )
    };
    let gen = quote::quote!{
        impl Naming for #ident {
            fn upper_camel_case_stringified() -> &'static str {
                #ident_upper_camel_case_quotes_token_stream
            }
            fn upper_camel_case_token_stream() -> proc_macro2::TokenStream {
                quote::quote!{#ident_upper_camel_case_token_stream}
            }
            fn snake_case_stringified() -> &'static str {
                #ident_snake_case_quotes_token_stream
            }
            fn snake_case_token_stream() -> proc_macro2::TokenStream {
                quote::quote!{#ident_snake_case_token_stream}
            }
        }
    };
    // if ident == "" {
    //     println!("{gen}");
    // }
    gen.into()
}

#[proc_macro]
pub fn gen_naming_trait_impl_vec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = match syn::parse::Parser::parse2(
        syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated, 
        proc_macro2::TokenStream::from(input)
    ) {
        Ok(value) => value,
        Err(e) => {
            return e.into_compile_error().into();//todo expirement with .into_compile_error()
        }
    };
    for element in parsed_input {
        println!("1");
    }
    quote::quote!{fn answer() -> u32 { 42 }}.into()
}