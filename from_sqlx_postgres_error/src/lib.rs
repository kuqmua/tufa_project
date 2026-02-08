// #[proc_macro_derive(FromSqlxPostgresError)]
// pub fn from_sqlx_postgres_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     panic_location::panic_location();
//     let proc_macro_name_upper_camel_case = "FromSqlxPostgresError";
//     let syn_derive_input: syn::DeriveInput = syn::parse(input).expect("cd5c05b1-9cdc-49d8-83ef-f12c067cc021");
//     let ident = &syn_derive_input.ident;
//     let proc_macro_name_upper_camel_case_ident_stringified =
//         format!("{proc_macro_name_upper_camel_case} {ident}");

//     // let sqlx_postgres_error_named_syn_variants = macros_helpers::enum_variants::sqlx_postgres_error_named_syn_variants(&proc_macro_name_upper_camel_case_ident_stringified);
//     // sqlx_postgres_error_named_syn_variants.iter().map(|el_ea7f6dd1|{
//     //     let field_ident = field.ident;
//     //     let field_ty =
//     //     quote::quote! {}
//     // }).collect::<Vec<proc_macro2::TokenStream>>();

//     let field_code_occurence_new_d7be05e4_ebc4_47bc_a99c_d1143d5e4dae_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_d5e60d03_63d8_44be_be72_86f900b51836_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_7e40e82a_3043_41f3_974c_973a53d221de_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_279a8e42_c552_42db_b137_c8acb9973f92_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_e5c5b92a_40b4_429b_a29f_2e077ec23c04_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_bd227744_1167_4a95_8a83_27ea50261d2d_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_353cb4ca_0abb_4ba9_a78f_c89ea961a260_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_d032743c_7ee4_4a40_b46c_6cc5d13dc8cb_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_d3992828_c626_477d_b14a_9146037ab416_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_81a0d2c3_5246_477c_8c67_10a5940c95c1_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_89ccb64c_050b_45bb_ae7b_1ade4f90ada5_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_e0ca61d3_f936_4414_a82c_8a87c60f02c5_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_f235ca0a_319b_411b_b6b4_a022261725ae_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_75e32177_f3b7_46f3_81f3_f6f56e8f2308_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_b281dabf_8137_4f23_ad97_daa8b2347670_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let field_code_occurence_new_24cd6c76_3a1f_42bb_9769_fb8d1b3e93f9_ts = macros_helpers::generate_field_code_occurence_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let generated = quote::quote! {
//         impl From<sqlx::Error> for #ident {
//             fn from(val: sqlx::Error) -> Self {
//                 match val {
//                     sqlx::Error::Configuration(value) => {
//                         Self::Configuration {
//                             configuration: value.to_string(),
//                             #field_code_occurence_new_d7be05e4_ebc4_47bc_a99c_d1143d5e4dae_ts
//                         }
//                     }
//                     sqlx::Error::Database(database) => {
//                         Self::Database {
//                             database: database.message().to_owned(),
//                             #field_code_occurence_new_d5e60d03_63d8_44be_be72_86f900b51836_ts,
//                         }
//                     }
//                     sqlx::Error::Io(io) => Self::Io {
//                         io,
//                         #field_code_occurence_new_7e40e82a_3043_41f3_974c_973a53d221de_ts,
//                     },
//                     sqlx::Error::Tls(value) => Self::Tls {
//                         tls: value.to_string(),
//                         #field_code_occurence_new_279a8e42_c552_42db_b137_c8acb9973f92_ts,
//                     },
//                     sqlx::Error::Protocol(string) => Self::Protocol {
//                         protocol: string,
//                         #field_code_occurence_new_e5c5b92a_40b4_429b_a29f_2e077ec23c04_ts,
//                     },
//                     sqlx::Error::RowNotFound => Self::RowNotFound {
//                         row_not_found: String::from("row_not_found"),
//                         #field_code_occurence_new_bd227744_1167_4a95_8a83_27ea50261d2d_ts,
//                     },
//                     sqlx::Error::TypeNotFound { type_name } => Self::TypeNotFound {
//                         type_not_found: type_name,
//                         #field_code_occurence_new_353cb4ca_0abb_4ba9_a78f_c89ea961a260_ts,
//                     },
//                     sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
//                         Self::ColumnIndexOutOfBounds {
//                             column_index_out_of_bounds: index,
//                             len,
//                             #field_code_occurence_new_d032743c_7ee4_4a40_b46c_6cc5d13dc8cb_ts,
//                         }
//                     }
//                     sqlx::Error::ColumnNotFound(column_not_found) => {
//                         Self::ColumnNotFound {
//                             column_not_found,
//                             #field_code_occurence_new_d3992828_c626_477d_b14a_9146037ab416_ts,
//                         }
//                     }
//                     sqlx::Error::ColumnDecode { index, source } => {
//                         Self::ColumnDecode {
//                             column_decode_index: index,
//                             source_handle: source.to_string(),
//                             #field_code_occurence_new_81a0d2c3_5246_477c_8c67_10a5940c95c1_ts,
//                         }
//                     }
//                     sqlx::Error::Decode(value) => Self::Decode {
//                         decode: value.to_string(),
//                         #field_code_occurence_new_89ccb64c_050b_45bb_ae7b_1ade4f90ada5_ts,
//                     },
//                     sqlx::Error::PoolTimedOut => Self::PoolTimedOut {
//                         pool_timed_out: String::from("pool timed out"),
//                         #field_code_occurence_new_e0ca61d3_f936_4414_a82c_8a87c60f02c5_ts,
//                     },
//                     sqlx::Error::PoolClosed => Self::PoolClosed {
//                         pool_closed: String::from("pool closed"),
//                         #field_code_occurence_new_f235ca0a_319b_411b_b6b4_a022261725ae_ts,
//                     },
//                     sqlx::Error::WorkerCrashed => Self::WorkerCrashed {
//                         worker_crashed: String::from("worker crashed"),
//                         #field_code_occurence_new_75e32177_f3b7_46f3_81f3_f6f56e8f2308_ts,
//                     },
//                     sqlx::Error::Migrate(migrate_error) => Self::Migrate {
//                         migrate: *migrate_error,
//                         #field_code_occurence_new_b281dabf_8137_4f23_ad97_daa8b2347670_ts,
//                     },
//                     _ => Self::UnexpectedCase {
//                         unexpected_case: String::from("unexpected_case"),
//                         #field_code_occurence_new_24cd6c76_3a1f_42bb_9769_fb8d1b3e93f9_ts,
//                     },
//                 }
//             }
//         }
//     };
//     // println!("{gen}");
//     gen.into()
// }
