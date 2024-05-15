#[proc_macro]
pub fn generate_upper_camel_and_snake_case_stringified_and_token_stream_from_naming_constants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let implementations_token_stream = serde_json::from_str::<std::vec::Vec<std::vec::Vec<std::string::String>>>(&input.to_string())
    .expect("failed to convert tokens input into valid json string[][] pattern")
    .into_iter()
    .map(|element| {
        let prefix_upper_camel_case = "_upper_camel_case";
        let prefix_snake_case = "_snake_case";
        let prefix_stringified = "_stringified";
        let prefix_token_stream = "_token_stream";
        let prefix_upper_camel_case_stringified = format!("{prefix_upper_camel_case}{prefix_stringified}");
        let prefix_snake_case_stringified = format!("{prefix_snake_case}{prefix_stringified}");
        let prefix_upper_camel_case_token_stream = format!("{prefix_upper_camel_case}{prefix_token_stream}");
        let prefix_snake_case_token_stream = format!("{prefix_snake_case}{prefix_token_stream}");
        //
// proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&tvfrr_status_code)
        //
        quote::quote!{
            // pub fn primary_key_from_row_and_failed_rollback_upper_camel_case_stringified() -> std::string::String {
            //     format!(
            //         "{}{}{}{}{}{}{}",
            //         <naming_constants::Primary as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::Key as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::From as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::Row as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::And as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::Failed as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::Rollback as naming_constants::Naming>::upper_camel_case_stringified(),
            //     )
            // }
            // pub fn primary_key_from_row_and_failed_rollback_snake_case_stringified() -> std::string::String {
            //     format!(
            //         "{}_{}_{}_{}_{}_{}_{}",
            //         <naming_constants::Primary as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::Key as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::From as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::Row as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::And as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::Failed as naming_constants::Naming>::upper_camel_case_stringified(),
            //         <naming_constants::Rollback as naming_constants::Naming>::upper_camel_case_stringified(),
            //     )
            // }
            // pub fn primary_key_from_row_and_failed_rollback_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
            //     let value = primary_key_from_row_and_failed_rollback_upper_camel_case_stringified();
            //     value.parse::<proc_macro2::TokenStream>()
            //     .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            // }
            // pub fn primary_key_from_row_and_failed_rollback_snake_case_token_stream() -> proc_macro2::TokenStream {
            //     let value = primary_key_from_row_and_failed_rollback_snake_case_stringified();
            //     value.parse::<proc_macro2::TokenStream>()
            //     .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            // }
        }
    });
    let gen = quote::quote!{#(#implementations_token_stream)*};
    // println!("{gen}");
    gen.into()
}