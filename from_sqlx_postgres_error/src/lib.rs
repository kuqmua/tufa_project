#[proc_macro_derive(FromSqlxPostgresError)]
pub fn from_sqlx_postgres_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "FromSqlxPostgresError";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|_| {
        panic!(
            "{}",
            proc_macro_common::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified =
        format!("{proc_macro_name_upper_camel_case} {ident}");

    // let sqlx_postgres_error_named_syn_variants = proc_macro_helpers::enum_variants::sqlx_postgres_error_named_syn_variants(&proc_macro_name_upper_camel_case_ident_stringified);
    // sqlx_postgres_error_named_syn_variants.iter().map(|element|{
    //     let field_ident = field.ident;
    //     let field_ty =
    //     quote::quote! {}
    // }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();

    let field_code_occurence_new_d7be05e4_ebc4_47bc_a99c_d1143d5e4dae_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_d5e60d03_63d8_44be_be72_86f900b51836_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_7e40e82a_3043_41f3_974c_973a53d221de_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_279a8e42_c552_42db_b137_c8acb9973f92_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_e5c5b92a_40b4_429b_a29f_2e077ec23c04_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_bd227744_1167_4a95_8a83_27ea50261d2d_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_353cb4ca_0abb_4ba9_a78f_c89ea961a260_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_d032743c_7ee4_4a40_b46c_6cc5d13dc8cb_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_d3992828_c626_477d_b14a_9146037ab416_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_81a0d2c3_5246_477c_8c67_10a5940c95c1_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_89ccb64c_050b_45bb_ae7b_1ade4f90ada5_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_e0ca61d3_f936_4414_a82c_8a87c60f02c5_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_f235ca0a_319b_411b_b6b4_a022261725ae_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_75e32177_f3b7_46f3_81f3_f6f56e8f2308_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_b281dabf_8137_4f23_ad97_daa8b2347670_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_24cd6c76_3a1f_42bb_9769_fb8d1b3e93f9_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let gen = quote::quote! {
        impl From<sqlx::Error> for #ident {
            fn from(val: sqlx::Error) -> Self {
                match val {
                    sqlx::Error::Configuration(value) => {
                        Self::Configuration {
                            configuration: value.to_string(),
                            #field_code_occurence_new_d7be05e4_ebc4_47bc_a99c_d1143d5e4dae_token_stream
                        }
                    }
                    sqlx::Error::Database(database) => {
                        Self::Database {
                            database: database.message().to_string(),
                            #field_code_occurence_new_d5e60d03_63d8_44be_be72_86f900b51836_token_stream,
                        }
                    }
                    sqlx::Error::Io(io) => Self::Io {
                        io,
                        #field_code_occurence_new_7e40e82a_3043_41f3_974c_973a53d221de_token_stream,
                    },
                    sqlx::Error::Tls(value) => Self::Tls {
                        tls: value.to_string(),
                        #field_code_occurence_new_279a8e42_c552_42db_b137_c8acb9973f92_token_stream,
                    },
                    sqlx::Error::Protocol(string) => Self::Protocol {
                        protocol: string,
                        #field_code_occurence_new_e5c5b92a_40b4_429b_a29f_2e077ec23c04_token_stream,
                    },
                    sqlx::Error::RowNotFound => Self::RowNotFound {
                        row_not_found: std::string::String::from("row_not_found"),
                        #field_code_occurence_new_bd227744_1167_4a95_8a83_27ea50261d2d_token_stream,
                    },
                    sqlx::Error::TypeNotFound { type_name } => Self::TypeNotFound {
                        type_not_found: type_name,
                        #field_code_occurence_new_353cb4ca_0abb_4ba9_a78f_c89ea961a260_token_stream,
                    },
                    sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
                        Self::ColumnIndexOutOfBounds {
                            column_index_out_of_bounds: index,
                            len,
                            #field_code_occurence_new_d032743c_7ee4_4a40_b46c_6cc5d13dc8cb_token_stream,
                        }
                    }
                    sqlx::Error::ColumnNotFound(column_not_found) => {
                        Self::ColumnNotFound {
                            column_not_found,
                            #field_code_occurence_new_d3992828_c626_477d_b14a_9146037ab416_token_stream,
                        }
                    }
                    sqlx::Error::ColumnDecode { index, source } => {
                        Self::ColumnDecode {
                            column_decode_index: index,
                            source_handle: source.to_string(),
                            #field_code_occurence_new_81a0d2c3_5246_477c_8c67_10a5940c95c1_token_stream,
                        }
                    }
                    sqlx::Error::Decode(value) => Self::Decode {
                        decode: value.to_string(),
                        #field_code_occurence_new_89ccb64c_050b_45bb_ae7b_1ade4f90ada5_token_stream,
                    },
                    sqlx::Error::PoolTimedOut => Self::PoolTimedOut {
                        pool_timed_out: std::string::String::from("pool timed out"),
                        #field_code_occurence_new_e0ca61d3_f936_4414_a82c_8a87c60f02c5_token_stream,
                    },
                    sqlx::Error::PoolClosed => Self::PoolClosed {
                        pool_closed: std::string::String::from("pool closed"),
                        #field_code_occurence_new_f235ca0a_319b_411b_b6b4_a022261725ae_token_stream,
                    },
                    sqlx::Error::WorkerCrashed => Self::WorkerCrashed {
                        worker_crashed: std::string::String::from("worker crashed"),
                        #field_code_occurence_new_75e32177_f3b7_46f3_81f3_f6f56e8f2308_token_stream,
                    },
                    sqlx::Error::Migrate(migrate_error) => Self::Migrate {
                        migrate: *migrate_error,
                        #field_code_occurence_new_b281dabf_8137_4f23_ad97_daa8b2347670_token_stream,
                    },
                    _ => Self::UnexpectedCase {
                        unexpected_case: std::string::String::from("unexpected_case"),
                        #field_code_occurence_new_24cd6c76_3a1f_42bb_9769_fb8d1b3e93f9_token_stream,
                    },
                }
            }
        }
    };
    //println!("{gen}");
    gen.into()
}
