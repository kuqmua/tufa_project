#[proc_macro_derive(BindQueryForRustSqlxPostgresqlWrapperType)] //todo check on postgresql max length value of type
pub fn bind_query_for_rust_sqlx_postgresql_wrapper_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "BindQuery";
    let proc_macro_name_snake_case =
        proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &proc_macro_name_upper_camel_case,
        );
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {e}",
            proc_macro_common::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", ast.data);
    let ident = &ast.ident;
    let gen = quote::quote!{
        impl BindQuery for #ident {
            fn try_increment(&self, increment: &mut u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        Ok(())
                    }
                    None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                        checked_add: std::string::String::from(CHECKED_ADD_IS_NONE),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            }
            fn try_generate_bind_increments(&self, increment: &mut u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
                let mut increments = std::string::String::default();
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        increments.push_str(&format!("${increment}"));
                    }
                    None => {
                        return Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                            checked_add: std::string::String::from(CHECKED_ADD_IS_NONE),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
                Ok(increments)
            }
            fn bind_value_to_query(self, mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
                query = query.bind(self.0);
                query
            }
        }
    };
    gen.into()
}