#[proc_macro_derive(FromSqlxPostgresError)]
pub fn from_sqlx_postgres_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location(); //panic_location function from https://github.com/kuqmua/proc_macro_helpers
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|_| {
        panic!(
            "{}",
            proc_macro_common::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let gen = quote::quote! {
        impl From<sqlx::Error> for #ident {
            fn from(val: sqlx::Error) -> Self {
                match val {
                    sqlx::Error::Configuration(value) => {
                        Self::Configuration {
                            configuration: value.to_string(),
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }
                    }
                    sqlx::Error::Database(database) => {
                        Self::Database {
                            database: database.message().to_string(),
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }
                    }
                    sqlx::Error::Io(io) => Self::Io {
                        io,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                    sqlx::Error::Tls(value) => Self::Tls {
                        tls: value.to_string(),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                    sqlx::Error::Protocol(string) => Self::Protocol {
                        protocol: string,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                    sqlx::Error::RowNotFound => Self::RowNotFound {
                        row_not_found: std::string::String::from("row_not_found"),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                    sqlx::Error::TypeNotFound { type_name } => Self::TypeNotFound {
                        type_not_found: type_name,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                    sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
                        Self::ColumnIndexOutOfBounds {
                            column_index_out_of_bounds: index,
                            len,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }
                    }
                    sqlx::Error::ColumnNotFound(column_not_found) => {
                        Self::ColumnNotFound {
                            column_not_found,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }
                    }
                    sqlx::Error::ColumnDecode { index, source } => {
                        Self::ColumnDecode {
                            column_decode_index: index,
                            source_handle: source.to_string(),
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }
                    }
                    sqlx::Error::Decode(value) => Self::Decode {
                        decode: value.to_string(),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                    sqlx::Error::PoolTimedOut => Self::PoolTimedOut {
                        pool_timed_out: std::string::String::from("pool timed out"),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                    sqlx::Error::PoolClosed => Self::PoolClosed {
                        pool_closed: std::string::String::from("pool closed"),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                    sqlx::Error::WorkerCrashed => Self::WorkerCrashed {
                        worker_crashed: std::string::String::from("worker crashed"),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                    sqlx::Error::Migrate(migrate_error) => Self::Migrate {
                        migrate: *migrate_error,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                    _ => Self::UnexpectedCase {
                        unexpected_case: std::string::String::from("unexpected_case"),
                        code_occurence: crate::code_occurence_tufa_common!(),
                    },
                }
            }
        }
    };
    //println!("{gen}");
    gen.into()
}
