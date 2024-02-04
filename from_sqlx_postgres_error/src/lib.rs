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
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let field_code_occurence_new_d7be05e4_ebc4_47bc_a99c_d1143d5e4dae_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
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
                            code_occurence: crate::code_occurence!(),
                        }
                    }
                    sqlx::Error::Io(io) => Self::Io {
                        io,
                        code_occurence: crate::code_occurence!(),
                    },
                    sqlx::Error::Tls(value) => Self::Tls {
                        tls: value.to_string(),
                        code_occurence: crate::code_occurence!(),
                    },
                    sqlx::Error::Protocol(string) => Self::Protocol {
                        protocol: string,
                        code_occurence: crate::code_occurence!(),
                    },
                    sqlx::Error::RowNotFound => Self::RowNotFound {
                        row_not_found: std::string::String::from("row_not_foundcrrrr"),
                        code_occurence: crate::code_occurence!(),
                    },
                    sqlx::Error::TypeNotFound { type_name } => Self::TypeNotFound {
                        type_not_found: type_name,
                        code_occurence: crate::code_occurence!(),
                    },
                    sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
                        Self::ColumnIndexOutOfBounds {
                            column_index_out_of_bounds: index,
                            len,
                            code_occurence: crate::code_occurence!(),
                        }
                    }
                    sqlx::Error::ColumnNotFound(column_not_found) => {
                        Self::ColumnNotFound {
                            column_not_found,
                            code_occurence: crate::code_occurence!(),
                        }
                    }
                    sqlx::Error::ColumnDecode { index, source } => {
                        Self::ColumnDecode {
                            column_decode_index: index,
                            source_handle: source.to_string(),
                            code_occurence: crate::code_occurence!(),
                        }
                    }
                    sqlx::Error::Decode(value) => Self::Decode {
                        decode: value.to_string(),
                        code_occurence: crate::code_occurence!(),
                    },
                    sqlx::Error::PoolTimedOut => Self::PoolTimedOut {
                        pool_timed_out: std::string::String::from("pool timed out"),
                        code_occurence: crate::code_occurence!(),
                    },
                    sqlx::Error::PoolClosed => Self::PoolClosed {
                        pool_closed: std::string::String::from("pool closed"),
                        code_occurence: crate::code_occurence!(),
                    },
                    sqlx::Error::WorkerCrashed => Self::WorkerCrashed {
                        worker_crashed: std::string::String::from("worker crashed"),
                        code_occurence: crate::code_occurence!(),
                    },
                    sqlx::Error::Migrate(migrate_error) => Self::Migrate {
                        migrate: *migrate_error,
                        code_occurence: crate::code_occurence!(),
                    },
                    _ => Self::UnexpectedCase {
                        unexpected_case: std::string::String::from("unexpected_case"),
                        code_occurence: crate::code_occurence!(),
                    },
                }
            }
        }
    };
    //println!("{gen}");
    gen.into()
}
