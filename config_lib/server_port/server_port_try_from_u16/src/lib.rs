use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
const SERVER_PORT_MIN_VALUE: u16 = 1024;
const SERVER_PORT_MAX_VALUE: u16 = 49151;
#[proc_macro]
pub fn server_port_try_from_u16(v: Ts) -> Ts {
    panic_loc::panic_loc();
    let valid_port_ts = {
        let possible_port = v.to_string().parse::<u16>().expect("310a948a");
        if possible_port < SERVER_PORT_MIN_VALUE {
            panic!("8d1c94bc");
        } else if possible_port <= SERVER_PORT_MAX_VALUE {
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
