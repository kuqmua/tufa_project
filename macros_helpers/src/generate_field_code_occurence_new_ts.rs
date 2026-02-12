use proc_macro2::TokenStream;
use quote::quote;
#[must_use]
pub fn generate_field_code_occurence_new_ts(
    file: &'static str,
    line: u32,
    column: u32,
) -> TokenStream {
    use naming::CodeOccurenceSc;
    let code_occurence_new_ts = {
        let file_ts = generate_quotes::double_quotes_ts(&file);
        let line_ts = {
            let line_str = line.to_string();
            line_str
                .parse::<TokenStream>()
                .expect("1d6812d7-2296-4d38-b3ea-bff1e625eaf5")
        };
        let column_ts = {
            let column_str = column.to_string();
            column_str
                .parse::<TokenStream>()
                .expect("105a4e62-7574-4b1e-bd5f-eed440d72e89")
        };
        quote! {
            error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from(#file_ts),
                    line: #line_ts,
                    column: #column_ts,
                })
            )
        }
    };
    let code_occurence_sc = CodeOccurenceSc;
    quote! {#code_occurence_sc: #code_occurence_new_ts}
}
