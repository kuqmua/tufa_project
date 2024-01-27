#[proc_macro]
pub fn user_port_try_from_u16(
    possible_u16_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let valid_port = match possible_u16_token_stream.to_string().parse::<u16>() {
        Err(e) => panic!("failed to parse input into u16, error: {e}"),
        Ok(possible_port) => {
            if possible_port < 1024 {
                panic!("failed to user_port_try_from_u16!(), reason: system port range 0-1023");
            } else if possible_port < 49152 {
                possible_port
            } else {
                panic!(
                    "failed to user_port_try_from_u16!(), reason: ephemeral port range 49152-65535"
                );
            }
        }
    };
    let valid_port_token_stream = valid_port
        .to_string()
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| {
            panic!("failed to parse valid u16 user port to proc_macro2::TokenStream")
        });
    let gen = quote::quote! {
        UserPort::try_from(#valid_port_token_stream).unwrap()
        // UserPort {
        //     port: #valid_port_token_stream
        // }
    };
    // println!("{gen}");
    gen.into()
}
