#[proc_macro]
pub fn server_port_try_from_u16(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let valid_port = match input.to_string().parse::<std::primitive::u16>() {
        Err(e) => panic!("failed to parse input into u16, error: {e}"),
        Ok(possible_port) => {
            if possible_port < constants::SERVER_PORT_MIN_VALUE {
                panic!("failed to server_port_try_from_u16!(), reason: system port range 0-1023");
            } else if possible_port <= constants::SERVER_PORT_MAX_VALUE {
                possible_port
            } else {
                panic!(
                    "failed to server_port_try_from_u16!(), reason: ephemeral port range 49152-65535"
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
        ServerPort::try_from(#valid_port_token_stream).unwrap()
    };
    // println!("{gen}");
    gen.into()
}
