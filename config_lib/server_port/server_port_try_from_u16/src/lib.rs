#[proc_macro]
pub fn server_port_try_from_u16(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let valid_port = match input.to_string().parse::<std::primitive::u16>() {
        Err(error) => panic!("failed to parse input into u16, error: {error}"),
        Ok(possible_port) => {
            if possible_port < server_port_common::SERVER_PORT_MIN_VALUE {
                panic!("{}", server_port_common::SERVER_PORT_IN_SYSTEM_PORT_RANGE_ERROR_MESSAGE);
            } else if possible_port <= server_port_common::SERVER_PORT_MAX_VALUE {
                possible_port
            } else {
                panic!("{}", server_port_common::SERVER_PORT_IN_EPHEMERAL_PORT_RANGE_ERROR_MESSAGE);
            }
        }
    };
    let valid_port_token_stream = valid_port.to_string().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("failed to parse valid u16 user port to proc_macro2::TokenStream"));
    let generated = quote::quote! {
        ServerPort::try_from(#valid_port_token_stream).unwrap()
    };
    // println!("{generated}");
    generated.into()
}
