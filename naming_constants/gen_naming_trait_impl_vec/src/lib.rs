#[proc_macro]
pub fn gen_naming_trait_impl_vec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let impls_token_stream = match syn::parse::Parser::parse2(
        syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated, 
        proc_macro2::TokenStream::from(input)
    ) {
        Ok(value) => value,
        Err(error) => {
            return error.into_compile_error().into();//todo expirement with .into_compile_error() https://docs.rs/syn/1.0.109/syn/parse/struct.Error.html#method.into_compile_error
        }
    }.into_iter().map(|element|{
        let ident = if let syn::Expr::Lit(value) = &element {
            if let syn::Lit::Str(value) = &value.lit {
                value.value()
            }
            else {
                panic!("052d9f61-2209-45d6-82af-cf7221e5a762");
            }
        }
        else {
            panic!("04ae46d6-1450-4006-901f-f56181711340");
        };
        let (ident_upper_camel_case_quotes_token_stream, ident_upper_camel_case_token_stream) = {
            let ident_upper_camel_case_stringified = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&ident.as_str());
            let ident_upper_camel_case_quotes_token_stream = {
                let value = format!("\"{ident_upper_camel_case_stringified}\"");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let ident_upper_camel_case_token_stream = {
                ident_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{ident_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            (
                ident_upper_camel_case_quotes_token_stream,
                ident_upper_camel_case_token_stream
            )
        };
        let (ident_snake_case_quotes_token_stream, ident_snake_case_token_stream) = {
            let ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&ident.as_str());
            let ident_snake_case_quotes_token_stream = {
                let value = format!("\"{ident_snake_case_stringified}\"");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let ident_snake_case_token_stream = {
                ident_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{ident_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            (
                ident_snake_case_quotes_token_stream,
                ident_snake_case_token_stream
            )
        };
        let ident_upper_camel_case_upper_camel_case_token_stream = {
            let value = format!(
                "{}UpperCamelCase",
                proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&ident.as_str())
            );
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let ident_snake_case_upper_camel_case_token_stream = {
            let value = format!(
                "{}SnakeCase",
                proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&ident.as_str())
            );
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        quote::quote!{
            #[derive(Debug, Clone, Copy)]
            pub struct #ident_upper_camel_case_upper_camel_case_token_stream;
            impl std::fmt::Display for #ident_upper_camel_case_upper_camel_case_token_stream {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(formatter, #ident_upper_camel_case_quotes_token_stream)
                }
            }
            impl quote::ToTokens for #ident_upper_camel_case_upper_camel_case_token_stream {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    quote::quote!{#ident_upper_camel_case_token_stream}.to_tokens(tokens)
                }
            }
            #[derive(Debug, Clone, Copy)]
            pub struct #ident_snake_case_upper_camel_case_token_stream;
            impl std::fmt::Display for #ident_snake_case_upper_camel_case_token_stream {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(formatter, #ident_snake_case_quotes_token_stream)
                }
            }
            impl quote::ToTokens for #ident_snake_case_upper_camel_case_token_stream {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    quote::quote!{#ident_snake_case_token_stream}.to_tokens(tokens)
                }
            }
        }
    });
    let generated = quote::quote!{#(#impls_token_stream)*};
    // println!("{generated}");
    generated.into()
}