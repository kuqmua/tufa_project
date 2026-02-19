use gen_quotes::dq_ts;
use naming::LocSc;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
#[must_use]
pub fn gen_field_loc_new_ts(file: &'static str, line: u32, column: u32) -> Ts2 {
    let loc_new_ts = {
        let file_ts = dq_ts(&file);
        let line_ts = {
            let line_str = line.to_string();
            line_str.parse::<Ts2>().expect("1d6812d7")
        };
        let column_ts = {
            let column_str = column.to_string();
            column_str.parse::<Ts2>().expect("105a4e62")
        };
        quote! {
            location_lib::loc::Loc::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(location_lib::loc::MacroOccurence {
                    file: String::from(#file_ts),
                    line: #line_ts,
                    column: #column_ts,
                })
            )
        }
    };
    quote! {#LocSc: #loc_new_ts}
}
