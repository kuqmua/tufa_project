use proc_macro2::TokenStream as Ts2;
use quote::quote;
#[proc_macro]
pub fn server_port_try_from_u16(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let valid_port_ts = {
        let possible_port = input.to_string().parse::<u16>().expect("310a948a");
        if possible_port < server_port_common::SERVER_PORT_MIN_VALUE {
            panic!("8d1c94bc");
        } else if possible_port <= server_port_common::SERVER_PORT_MAX_VALUE {
            possible_port
        } else {
            panic!("bfb64c71");
        }
    }
    .to_string()
    .parse::<Ts2>()
    .expect("dda46f3a");
    let generated = quote! {
        ServerPort::try_from(#valid_port_ts).expect("575a501d")
    };
    // println!("{generated}");
    generated.into()
}
