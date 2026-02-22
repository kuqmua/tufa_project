// #[proc_macro_derive(FromSqlxPgEr)]
// pub fn from_sqlx_pg_er(input: Ts) -> Ts {
//     panic_location::panic_location();
//     let proc_macro_name_ucc = "FromSqlxPgEr";
//     let di: DeriveInput = parse(input).expect("cd5c05b1");
//     let ident = &di.ident;
//     let proc_macro_name_ucc_ident_str =
//         format!("{proc_macro_name_ucc} {ident}");
//     // let sqlx_pg_er_syn_vrts = enum_vrts::sqlx_pg_er_syn_vrts(&proc_macro_name_ucc_ident_str);
//     // sqlx_pg_er_syn_vrts.iter().map(|el|{
//     //     let field_ident = field.ident;
//     //     let field_ty =
//     //     quote! {}
//     // }).collect::<Vec<Ts2>>();
//     let field_loc_new_d7be05e4_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_d5e60d03_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_7e40e82a_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_279a8e42_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_e5c5b92a_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_bd227744_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_353cb4ca_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_d032743c_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_d3992828_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_81a0d2c3_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_89ccb64c_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_e0ca61d3_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_f235ca0a_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_75e32177_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_b281dabf_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let field_loc_new_24cd6c76_ts = gen_field_loc_new_ts(
//         file!(),
//         line!(),
//         column!(),
//         &proc_macro_name_ucc_ident_str,
//     );
//     let generated = quote! {
//         impl From<sqlx::Error> for #ident {
//             fn from(val: sqlx::Error) -> Self {
//                 match val {
//                     sqlx::Error::Configuration(v) => {
//                         Self::Configuration {
//                             configuration: v.to_string(),
//                             #field_loc_new_d7be05e4_ts
//                         }
//                     }
//                     sqlx::Error::Database(database) => {
//                         Self::Database {
//                             database: database.message().to_owned(),
//                             #field_loc_new_d5e60d03_ts,
//                         }
//                     }
//                     sqlx::Error::Io(io) => Self::Io {
//                         io,
//                         #field_loc_new_7e40e82a_ts,
//                     },
//                     sqlx::Error::Tls(v) => Self::Tls {
//                         tls: v.to_string(),
//                         #field_loc_new_279a8e42_ts,
//                     },
//                     sqlx::Error::Protocol(string) => Self::Protocol {
//                         protocol: string,
//                         #field_loc_new_e5c5b92a_ts,
//                     },
//                     sqlx::Error::RowNotFound => Self::RowNotFound {
//                         row_not_found: String::from("row_not_found"),
//                         #field_loc_new_bd227744_ts,
//                     },
//                     sqlx::Error::TypeNotFound { type_name } => Self::TypeNotFound {
//                         type_not_found: type_name,
//                         #field_loc_new_353cb4ca_ts,
//                     },
//                     sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
//                         Self::ColumnIndexOutOfBounds {
//                             column_index_out_of_bounds: index,
//                             len,
//                             #field_loc_new_d032743c_ts,
//                         }
//                     }
//                     sqlx::Error::ColumnNotFound(column_not_found) => {
//                         Self::ColumnNotFound {
//                             column_not_found,
//                             #field_loc_new_d3992828_ts,
//                         }
//                     }
//                     sqlx::Error::ColumnDecode { index, source } => {
//                         Self::ColumnDecode {
//                             column_decode_index: index,
//                             source_handle: source.to_string(),
//                             #field_loc_new_81a0d2c3_ts,
//                         }
//                     }
//                     sqlx::Error::Decode(v) => Self::Decode {
//                         decode: v.to_string(),
//                         #field_loc_new_89ccb64c_ts,
//                     },
//                     sqlx::Error::PoolTimedOut => Self::PoolTimedOut {
//                         pool_timed_out: String::from("pool timed out"),
//                         #field_loc_new_e0ca61d3_ts,
//                     },
//                     sqlx::Error::PoolClosed => Self::PoolClosed {
//                         pool_closed: String::from("pool closed"),
//                         #field_loc_new_f235ca0a_ts,
//                     },
//                     sqlx::Error::WorkerCrashed => Self::WorkerCrashed {
//                         worker_crashed: String::from("worker crashed"),
//                         #field_loc_new_75e32177_ts,
//                     },
//                     sqlx::Error::Migrate(migrate_er) => Self::Migrate {
//                         migrate: *migrate_er,
//                         #field_loc_new_b281dabf_ts,
//                     },
//                     _ => Self::UnexpectedCase {
//                         unexpected_case: String::from("unexpected_case"),
//                         #field_loc_new_24cd6c76_ts,
//                     },
//                 }
//             }
//         }
//     };
//     // println!("{gen}");
//     gen.into()
// }
