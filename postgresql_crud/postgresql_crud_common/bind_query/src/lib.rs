#[proc_macro_derive(BindQueryForRustSqlxPostgresqlWrapperType)] //todo check on postgresql max length value of type
pub fn bind_query_for_rust_sqlx_postgresql_wrapper_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "BindQuery";
    let proc_macro_name_snake_case = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
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

#[proc_macro_derive(BindQueryForWhere)]
pub fn bind_query_for_where(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "BindQueryForWhere";
    let proc_macro_name_snake_case = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
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
                    },
                    None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                        checked_add: std::string::String::from("checked_add is None"),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                }
            }
            fn try_generate_bind_increments(&self, increment: &mut u64) -> Result<
                std::string::String,
                TryGenerateBindIncrementsErrorNamed,
            > {
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        Ok(format!("${increment}"))
                    },
                    None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                        checked_add: std::string::String::from("checked_add is None"),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                }
            }
            fn bind_value_to_query(self, mut query: sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
                query = query.bind(self.value.0);
                query
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresqlTypeLogicUsingFromSerializeDeserialize)]
pub fn std_convert_from_where_ident_serialize_deserialize_for_where_ident(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    println!("{input}");
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "PostgresqlTypeLogicUsingFromSerializeDeserialize";
    let proc_macro_name_snake_case = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &proc_macro_name_upper_camel_case,
    );
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {e}",
            proc_macro_common::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let field = if let syn::Data::Struct(data_struct) = &ast.data {
        if let syn::Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            match fields_unnamed.unnamed.len() {
                1 => &fields_unnamed.unnamed[0],
                _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Unnamed with one field")
            }
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Unnamed");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    println!("{:#?}", field);
    let field_type = &field.ty;
    let ident_with_serialize_deserialize_token_stream = {
        let value_stringified = format!("{ident}{}", proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified());
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let gen = quote::quote!{
        //todo maybe some of them will not be needed in the future
        //
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
        pub struct #ident_with_serialize_deserialize_token_stream(#field_type);
        impl std::convert::From<#ident_with_serialize_deserialize_token_stream> for #ident {
            fn from(value: #ident_with_serialize_deserialize_token_stream) -> Self {
                Self(value.0)
            }
        }
        // impl std::convert::From<StdPrimitiveBool> for StdPrimitiveBoolWithSerializeDeserialize {
        //     fn from(value: StdPrimitiveBool) -> Self {
        //         Self(value.0)
        //     }
        // }
        // impl StdPrimitiveBool {
        //     pub fn into_inner(self) -> std::primitive::bool {
        //         self.0
        //     }
        // }
        // impl std::convert::From<StdPrimitiveBool> for std::primitive::bool {
        //     fn from(value: StdPrimitiveBool) -> Self {
        //         value.0
        //     }
        // }
        // impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBool {
        //     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        //         <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::type_info()
        //     }
        //     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        //         <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        //     }
        // }
        // // impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveBool {
        // //     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        // //         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
        // //     }
        // //     fn encode(
        // //         self,
        // //         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
        // //     ) -> sqlx::encode::IsNull
        // //     where
        // //         Self: Sized,
        // //     {
        // //         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
        // //     }
        // //     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        // //         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
        // //     }
        // //     fn size_hint(&self) -> std::primitive::usize {
        // //         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
        // //     }
        // // }
        // // impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBool {
        // //     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        // //         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
        // //             Ok(value) => Ok(Self(value)),
        // //             Err(e) => Err(e),
        // //         }
        // //     }
        // // }
        // impl CheckSupportedPostgresqlColumnType for StdPrimitiveBool {
        //     fn check_supported_postgresql_column_type() {}
        // }
        // impl AsPostgresqlBool for StdPrimitiveBool {}
        // impl PostgresqlOrder for StdPrimitiveBool {}
        // impl std::convert::From<StdPrimitiveBool> for SupportedSqlxPostgresType {
        //     fn from(_value: StdPrimitiveBool) -> Self {
        //         SupportedSqlxPostgresType::StdPrimitiveBool
        //     }
        // }
        // impl std::convert::From<StdPrimitiveBoolAsPostgresqlBool> for StdPrimitiveBool {
        //     fn from(value: StdPrimitiveBoolAsPostgresqlBool) -> Self {
        //         value.0
        //     }
        // }
        // impl StdPrimitiveBool {
        //     pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::bool> {
        //         value
        //             .into_iter()
        //             .map(|element| element.into_inner())
        //             .collect()
        //     }
        // }
        // // impl std::convert::From<> for {
        // //     fn from(value: ) -> Self {
        // //         value.0
        // //     }
        // // }
        // #[derive(Debug, PartialEq, bind_query::BindQueryForWhere)]
        // pub struct WhereStdPrimitiveBool {
        //     pub value: StdPrimitiveBool,
        //     pub conjuctive_operator: ConjunctiveOperator,
        // }
        // #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        // pub struct WhereStdPrimitiveBoolWithSerializeDeserialize {
        //     pub value: StdPrimitiveBoolWithSerializeDeserialize,
        //     pub conjuctive_operator: ConjunctiveOperator,
        // }
        // impl std::convert::From<WhereStdPrimitiveBoolWithSerializeDeserialize> for WhereStdPrimitiveBool {
        //     fn from(value: WhereStdPrimitiveBoolWithSerializeDeserialize) -> Self {
        //         Self {
        //             value: StdPrimitiveBool::from(value.value),
        //             conjuctive_operator: value.conjuctive_operator
        //         }
        //     }
        // }
        //
    };
    gen.into()
}