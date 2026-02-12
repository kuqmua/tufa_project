use quote::quote;

#[proc_macro]
pub fn server_port_try_from_u16(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let valid_port_ts = {
        let possible_port = input
            .to_string()
            .parse::<u16>()
            .expect("310a948a-cfd7-4b2c-9e62-1c25cd61b891");
        if possible_port < server_port_common::SERVER_PORT_MIN_VALUE {
            panic!("8d1c94bc-89bf-4743-9ff8-0d838c1bbc79");
        } else if possible_port <= server_port_common::SERVER_PORT_MAX_VALUE {
            possible_port
        } else {
            panic!("bfb64c71-219e-414f-a65c-bd857a57a103");
        }
    }
    .to_string()
    .parse::<proc_macro2::TokenStream>()
    .expect("dda46f3a-0e3b-43e4-90e7-3d63977f226c");
    let gend = quote! {
        ServerPort::try_from(#valid_port_ts).expect("575a501d-fcca-4091-92c2-8ca5128bf314")
    };
    // println!("{gend}");
    gend.into()
}
