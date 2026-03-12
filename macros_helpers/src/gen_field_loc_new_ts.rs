use gen_quotes::dq_ts;
use naming::LocSc;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
#[must_use]
pub fn gen_field_loc_new_ts(file: &'static str, line: u32, col: u32) -> Ts2 {
    let loc_new_ts = {
        let file_ts = dq_ts(&file);
        let line_ts = {
            let line_str = line.to_string();
            line_str.parse::<Ts2>().expect("1d6812d7")
        };
        let col_ts = {
            let col_str = col.to_string();
            col_str.parse::<Ts2>().expect("105a4e62")
        };
        quote! {
            loc_lib::loc::Loc::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(loc_lib::loc::Occr {
                    file: String::from(#file_ts),
                    line: #line_ts,
                    col: #col_ts,
                })
            )
        }
    };
    quote! {#LocSc: #loc_new_ts}
}
