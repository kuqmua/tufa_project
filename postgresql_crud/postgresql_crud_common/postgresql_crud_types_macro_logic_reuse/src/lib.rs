
#[proc_macro_derive(CommonWithEqImpl)] //todo check on postgresql max length value of type
pub fn common_with_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    common_handle(
        input,
        "CommonWithEqImpl",
        true,
        true,
        true,
    )
}

#[proc_macro_derive(CommonWithoutEqImpl)] //todo check on postgresql max length value of type
pub fn common_without_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    common_handle(
        input,
        "CommonWithoutEqImpl",
        false,
        false,
        false,
    )
}

fn common_handle(
    input: proc_macro::TokenStream,
    proc_macro_name_upper_camel_case: &str,
    where_ident_should_implement_eq: std::primitive::bool,
    std_option_option_ident_upper_camel_case_should_implement_eq: std::primitive::bool,
    where_std_option_option_ident_upper_camel_case_should_implement_eq: std::primitive::bool,
) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", syn_derive_input.data);
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
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
    let field_type = &field.ty;
    let where_ident_token_stream = {
        let value = format!("{}{ident}", naming_conventions::WhereUpperCamelCase);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!(
            "{}{ident}",
            naming_conventions::StdOptionOptionUpperCamelCase
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!(
            "{}{}{ident}",
            naming_conventions::WhereUpperCamelCase,
            naming_conventions::StdOptionOptionUpperCamelCase
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    //
    let where_ident_should_implement_eq_token_stream = if where_ident_should_implement_eq {
        quote::quote!{Eq,}
    }
    else {
        proc_macro2::TokenStream::new()
    };
    let std_option_option_ident_upper_camel_case_should_implement_eq_token_stream = if std_option_option_ident_upper_camel_case_should_implement_eq {
        quote::quote!{Eq,}
    }
    else {
        proc_macro2::TokenStream::new()
    };
    let where_std_option_option_ident_upper_camel_case_should_implement_eq_token_stream = if where_std_option_option_ident_upper_camel_case_should_implement_eq {
        quote::quote!{Eq,}
    }
    else {
        proc_macro2::TokenStream::new()
    };
    let try_generate_bind_increments_error_named_upper_camel_case = naming_conventions::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let checked_add_upper_camel_case = naming_conventions::CheckedAddUpperCamelCase;
    let gen = quote::quote!{
        impl std::fmt::Display for #ident {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{:?}", self.0)
            }
        }
        impl error_occurence_lib::ToStdStringString for #ident {
            fn to_std_string_string(&self) -> std::string::String {
                format!("{self}")
            }
        }
        impl #ident {
            pub fn into_inner(self) -> #field_type {
                self.0
            }
        }
        impl std::convert::From<#ident> for #field_type {
            fn from(value: #ident) -> Self {
                value.0
            }
        }
        impl sqlx::Type<sqlx::Postgres> for #ident {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                <#field_type as sqlx::Type<sqlx::Postgres>>::type_info()
            }
            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
                <#field_type as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }
        // impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
        //     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        //         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
        //     }
        //     fn encode(
        //         self,
        //         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
        //     ) -> sqlx::encode::IsNull
        //     where
        //         Self: Sized,
        //     {
        //         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
        //     }
        //     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        //         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
        //     }
        //     fn size_hint(&self) -> std::primitive::usize {
        //         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
        //     }
        // }
        // impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
        //     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        //         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
        //             Ok(value) => Ok(Self(value)),
        //             Err(error) => Err(error),
        //         }
        //     }
        // }
        impl CheckSupportedPostgresqlColumnType for #ident {
            fn check_supported_postgresql_column_type() {}
        }
        // impl AsPostgresqlInt4Range for SqlxPostgresTypesPgRangeStdPrimitiveI32 {}
        impl std::convert::From<#ident> for SupportedSqlxPostgresType {
            fn from(_value: #ident) -> Self {
                Self::#ident
            }
        }
        impl #ident {
            pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<#field_type> {
                value
                    .into_iter()
                    .map(Self::into_inner)
                    .collect()
            }
        }
        impl BindQuery<'_> for #ident {
            fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), #try_generate_bind_increments_error_named_upper_camel_case> {
                increment.checked_add(1).map_or_else(|| Err(#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                    code_occurence: error_occurence_lib::code_occurence!(),
                }), |incr| {
                    *increment = incr;
                    Ok(())
                })
            }
            fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, #try_generate_bind_increments_error_named_upper_camel_case> {
                let mut increments = std::string::String::default();
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        increments.push_str(&format!("${increment}"));
                    }
                    None => {
                        return Err(#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
                Ok(increments)
            }
            fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                query = query.bind(self.0);
                query
            }
        }
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, #where_ident_should_implement_eq_token_stream)]
        pub struct #where_ident_token_stream {
            pub value: #ident,
            pub conjuctive_operator: ConjunctiveOperator,
        }
        impl std::fmt::Display for #where_ident_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
            }
        }
        impl BindQuery<'_> for #where_ident_token_stream {
            fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), #try_generate_bind_increments_error_named_upper_camel_case> {
                increment.checked_add(1).map_or_else(|| Err(#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                    code_occurence: error_occurence_lib::code_occurence!(),
                }), |incr| {
                    *increment = incr;
                    Ok(())
                })
            }
            fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<
                std::string::String,
                #try_generate_bind_increments_error_named_upper_camel_case,
            > {
                increment.checked_add(1).map_or_else(|| Err(#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                    code_occurence: error_occurence_lib::code_occurence!(),
                }), |incr| {
                    *increment = incr;
                    Ok(format!("${increment}"))
                })
            }
            fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                query = query.bind(self.value.0);
                query
            }
        }
        ///////////////////////////////////////
        #[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, #std_option_option_ident_upper_camel_case_should_implement_eq_token_stream)]
        pub struct #std_option_option_ident_upper_camel_case_token_stream(pub std::option::Option<#ident>);//#field_type
        ////////////////////////////
        impl std::fmt::Display for #std_option_option_ident_upper_camel_case_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    formatter, "{:?}", 
                    match &self.0 {
                        Some(value) => Some(&value.0),
                        None => None
                    }
                )
            }
        }
        impl #std_option_option_ident_upper_camel_case_token_stream {
            pub fn into_inner(self) -> std::option::Option<#field_type> {
                match self.0 {
                    Some(value) => Some(value.0),
                    None => None
                }
            }
        }
        impl std::convert::From<#std_option_option_ident_upper_camel_case_token_stream> for std::option::Option<#field_type> {
            fn from(value: #std_option_option_ident_upper_camel_case_token_stream) -> Self {
                match value.0 {
                    Some(value) => Some(value.0),
                    None => None
                }
            }
        }
        impl sqlx::Type<sqlx::Postgres> for #std_option_option_ident_upper_camel_case_token_stream {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                <std::option::Option<#field_type> as sqlx::Type<sqlx::Postgres>>::type_info()
            }
            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
                <std::option::Option<#field_type> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }
        impl CheckSupportedPostgresqlColumnType for #std_option_option_ident_upper_camel_case_token_stream {
            fn check_supported_postgresql_column_type() {}
        }
        impl std::convert::From<#std_option_option_ident_upper_camel_case_token_stream> for SupportedSqlxPostgresType {
            fn from(_value: #std_option_option_ident_upper_camel_case_token_stream) -> Self {
                SupportedSqlxPostgresType::#ident
            }
        }
        impl #std_option_option_ident_upper_camel_case_token_stream {
            pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::option::Option<#field_type>> {
                value
                    .into_iter()
                    .map(|element| element.into_inner())
                    .collect()
            }
        }
        impl BindQuery<'_> for #std_option_option_ident_upper_camel_case_token_stream {
            fn try_increment(
                &self,
                increment: &mut std::primitive::u64,
            ) -> Result<(), #try_generate_bind_increments_error_named_upper_camel_case> {
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        Ok(())
                    }
                    None => Err(#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                }
            }
            fn try_generate_bind_increments(
                &self,
                increment: &mut std::primitive::u64,
            ) -> Result<std::string::String, #try_generate_bind_increments_error_named_upper_camel_case> {
                let mut increments = std::string::String::default();
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        increments.push_str(&format!("${increment}"));
                    }
                    None => {
                        return Err(#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
                Ok(increments)
            }
            fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                query = query.bind(match self.0 {
                    Some(value) => Some(value.0),
                    None => None
                });
                query
            }
        }
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, #where_std_option_option_ident_upper_camel_case_should_implement_eq_token_stream)]
        pub struct #where_std_option_option_ident_upper_camel_case_token_stream {
            pub value: #std_option_option_ident_upper_camel_case_token_stream ,
            pub conjuctive_operator: ConjunctiveOperator,
        }
        impl std::fmt::Display for #where_std_option_option_ident_upper_camel_case_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    formatter,
                    "value: {}, conjuctive_operator: {}",
                    self.value, self.conjuctive_operator
                )
            }
        }
        impl BindQuery<'_> for #where_std_option_option_ident_upper_camel_case_token_stream {
            fn try_increment(
                &self,
                increment: &mut std::primitive::u64,
            ) -> Result<(), #try_generate_bind_increments_error_named_upper_camel_case> {
                increment.checked_add(1).map_or_else(|| Err(#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                    code_occurence: error_occurence_lib::code_occurence!(),
                }), |incr| {
                    *increment = incr;
                    Ok(())
                })
            }
            fn try_generate_bind_increments(
                &self,
                increment: &mut std::primitive::u64,
            ) -> Result<std::string::String, #try_generate_bind_increments_error_named_upper_camel_case> {
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        Ok(format!("${increment}"))
                    }
                    None => Err(#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                }
            }
            fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                query = query.bind(match self.value.0 {
                    Some(value) => Some(value.0),
                    None => None
                });
                query
            }
        }
    };
    // if ident == "StdPrimitiveBool" {
    //     println!("{gen}");
    //     // println!("----------");//todo for some reason gen duplicates for few times - find out why and fix
    // }
    gen.into()
}
///////////////

#[proc_macro_derive(AsPostgresqlCommon)] //todo check on postgresql max length value of type
pub fn as_postgresql_common(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "AsPostgresqlCommon";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", syn_derive_input.data);
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let _field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
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
    let gen = quote::quote!{
        impl CheckSupportedRustAndPostgresqlColumnType for #ident {
            fn check_supported_rust_and_postgresql_column_type() {}
        }
    };
    gen.into()
}

///////////////
#[derive(Debug)]
enum SupportedPredefinedType<'a> {
    StdPrimitiveI8Json, 
    StdPrimitiveI16Json, 
    StdPrimitiveI32Json, 
    StdPrimitiveI64Json, 
    StdPrimitiveI128Json,
    StdPrimitiveU8Json, 
    StdPrimitiveU16Json, 
    StdPrimitiveU32Json, 
    StdPrimitiveU64Json, 
    StdPrimitiveU128Json, 
    StdPrimitiveF32Json, 
    StdPrimitiveF64Json, 
    StdPrimitiveBoolJson, 
    StdStringStringJson, 

    StdOptionOptionStdPrimitiveI8Json, 
    StdOptionOptionStdPrimitiveI16Json, 
    StdOptionOptionStdPrimitiveI32Json, 
    StdOptionOptionStdPrimitiveI64Json, 
    StdOptionOptionStdPrimitiveI128Json, 
    StdOptionOptionStdPrimitiveU8Json, 
    StdOptionOptionStdPrimitiveU16Json, 
    StdOptionOptionStdPrimitiveU32Json, 
    StdOptionOptionStdPrimitiveU64Json, 
    StdOptionOptionStdPrimitiveU128Json, 
    StdOptionOptionStdPrimitiveF32Json, 
    StdOptionOptionStdPrimitiveF64Json, 
    StdOptionOptionStdPrimitiveBoolJson, 
    StdOptionOptionStdStringStringJson, 

    StdVecVecStdPrimitiveI8Json, 
    StdVecVecStdPrimitiveI16Json, 
    StdVecVecStdPrimitiveI32Json, 
    StdVecVecStdPrimitiveI64Json, 
    StdVecVecStdPrimitiveI128Json, 
    StdVecVecStdPrimitiveU8Json, 
    StdVecVecStdPrimitiveU16Json, 
    StdVecVecStdPrimitiveU32Json, 
    StdVecVecStdPrimitiveU64Json, 
    StdVecVecStdPrimitiveU128Json, 
    StdVecVecStdPrimitiveF32Json, 
    StdVecVecStdPrimitiveF64Json, 
    StdVecVecStdPrimitiveBoolJson, 
    StdVecVecStdStringStringJson, 

    StdOptionOptionStdVecVecStdPrimitiveI8Json, 
    StdOptionOptionStdVecVecStdPrimitiveI16Json, 
    StdOptionOptionStdVecVecStdPrimitiveI32Json, 
    StdOptionOptionStdVecVecStdPrimitiveI64Json, 
    StdOptionOptionStdVecVecStdPrimitiveI128Json, 
    StdOptionOptionStdVecVecStdPrimitiveU8Json, 
    StdOptionOptionStdVecVecStdPrimitiveU16Json, 
    StdOptionOptionStdVecVecStdPrimitiveU32Json, 
    StdOptionOptionStdVecVecStdPrimitiveU64Json, 
    StdOptionOptionStdVecVecStdPrimitiveU128Json, 
    StdOptionOptionStdVecVecStdPrimitiveF32Json, 
    StdOptionOptionStdVecVecStdPrimitiveF64Json, 
    StdOptionOptionStdVecVecStdPrimitiveBoolJson, 
    StdOptionOptionStdVecVecStdStringStringJson, 

    StdVecVecStdOptionOptionStdPrimitiveI8Json, 
    StdVecVecStdOptionOptionStdPrimitiveI16Json, 
    StdVecVecStdOptionOptionStdPrimitiveI32Json, 
    StdVecVecStdOptionOptionStdPrimitiveI64Json, 
    StdVecVecStdOptionOptionStdPrimitiveI128Json, 
    StdVecVecStdOptionOptionStdPrimitiveU8Json, 
    StdVecVecStdOptionOptionStdPrimitiveU16Json, 
    StdVecVecStdOptionOptionStdPrimitiveU32Json, 
    StdVecVecStdOptionOptionStdPrimitiveU64Json, 
    StdVecVecStdOptionOptionStdPrimitiveU128Json, 
    StdVecVecStdOptionOptionStdPrimitiveF32Json, 
    StdVecVecStdOptionOptionStdPrimitiveF64Json, 
    StdVecVecStdOptionOptionStdPrimitiveBoolJson, 
    StdVecVecStdOptionOptionStdStringStringJson, 

    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64Json, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBoolJson, 
    StdOptionOptionStdVecVecStdOptionOptionStdStringStringJson, 

    GenericJson(&'a syn::PathArguments), 
    StdOptionOptionGenericJson(&'a syn::PathArguments), 
    StdVecVecGenericJson(&'a syn::PathArguments), 
    StdOptionOptionStdVecVecGenericJson(&'a syn::PathArguments), 
    StdVecVecStdOptionOptionStdGenericJson(&'a syn::PathArguments), 
    StdOptionOptionStdVecVecStdOptionOptionStdGenericJson(&'a syn::PathArguments), 
}
#[proc_macro_derive(GeneratePostgresqlQueryPart)]
pub fn generate_postgresql_query_part(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GeneratePostgresqlQueryPart";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", syn_derive_input.data);
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
//
    let vec_syn_field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            fields_named.named.iter().map(|element|element).collect::<std::vec::Vec<&syn::Field>>()
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Named");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    // println!("{vec_syn_field:#?}");
//
    let field_upper_camel_case = naming_conventions::FieldUpperCamelCase;
    let ident_field_upper_camel_case = {
        let value =  format!("{ident}{field_upper_camel_case}");
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let pub_enum_ident_field_token_stream = {
        let variants_token_stream = vec_syn_field.iter().map(|element|{
            let field_ident_stringified = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                )
            }).to_string();
            let serialize_deserialize_field_ident_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
                &field_ident_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
            let maybe_additional_token_stream = {
                let element_ty = &element.ty;
                match &element.ty {
                    syn::Type::Path(type_path) => match type_path.path.segments.last() {
                        Some(path_segment) => {
                            let segment_ident_stringified = path_segment.ident.to_string();
                            // let syn_angle_bracketed_generic_arguments = match &path_segment.arguments {
                            
                            // };
                            
                            // if let syn::PathArguments::AngleBracketed(value) = &path_segment.arguments {
                            //     value
                            // }
                            // else {
                            //     panic!("{proc_macro_name_upper_camel_case_ident_stringified} unsupported predefined type wrapper: {segment_ident_stringified}");
                            // };
                            //todo use later
                            // {
                            //         if syn::PathArguments::None != path_segment.arguments {
                            //             panic!("{proc_macro_name_upper_camel_case_ident_stringified} unsupported predefined type wrapper: {segment_ident_stringified} syn::PathArguments::None != path_segment.arguments");
                            //         };
                            //         SupportedPredefinedType::StdPrimitiveI8Json
                            //     }
                            let supported_predefined_type = match segment_ident_stringified.as_str() {
                                "StdPrimitiveI8Json" => "StdPrimitiveI16Json",
                                "StdPrimitiveI32Json" => "",
                                "StdPrimitiveI64Json" => "",
                                "StdPrimitiveI128Json" => "",
                                "StdPrimitiveU8Json" => "",
                                "StdPrimitiveU16Json" => "",
                                "StdPrimitiveU32Json" => "",
                                "StdPrimitiveU64Json" => "",
                                "StdPrimitiveU128Json" => "",
                                "StdPrimitiveF32Json" => "",
                                "StdPrimitiveF64Json" => "",
                                "StdPrimitiveBoolJson" => "",
                                "StdStringStringJson" => "",

                                "StdOptionOptionStdPrimitiveI8Json" => "",
                                "StdOptionOptionStdPrimitiveI16Json" => "",
                                "StdOptionOptionStdPrimitiveI32Json" => "",
                                "StdOptionOptionStdPrimitiveI64Json" => "",
                                "StdOptionOptionStdPrimitiveI128Json" => "",
                                "StdOptionOptionStdPrimitiveU8Json" => "",
                                "StdOptionOptionStdPrimitiveU16Json" => "",
                                "StdOptionOptionStdPrimitiveU32Json" => "",
                                "StdOptionOptionStdPrimitiveU64Json" => "",
                                "StdOptionOptionStdPrimitiveU128Json" => "",
                                "StdOptionOptionStdPrimitiveF32Json" => "",
                                "StdOptionOptionStdPrimitiveF64Json" => "",
                                "StdOptionOptionStdPrimitiveBoolJson" => "",
                                "StdOptionOptionStdStringStringJson" => "",

                                "StdVecVecStdPrimitiveI8Json" => "",
                                "StdVecVecStdPrimitiveI16Json" => "",
                                "StdVecVecStdPrimitiveI32Json" => "",
                                "StdVecVecStdPrimitiveI64Json" => "",
                                "StdVecVecStdPrimitiveI128Json" => "",
                                "StdVecVecStdPrimitiveU8Json" => "",
                                "StdVecVecStdPrimitiveU16Json" => "",
                                "StdVecVecStdPrimitiveU32Json" => "",
                                "StdVecVecStdPrimitiveU64Json" => "",
                                "StdVecVecStdPrimitiveU128Json" => "",
                                "StdVecVecStdPrimitiveF32Json" => "",
                                "StdVecVecStdPrimitiveF64Json" => "",
                                "StdVecVecStdPrimitiveBoolJson" => "",
                                "StdVecVecStdStringStringJson" => "",

                                "StdOptionOptionStdVecVecStdPrimitiveI8Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveI16Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveI32Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveI64Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveI128Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveU8Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveU16Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveU32Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveU64Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveU128Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveF32Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveF64Json" => "",
                                "StdOptionOptionStdVecVecStdPrimitiveBoolJson" => "",
                                "StdOptionOptionStdVecVecStdStringStringJson" => "",

                                "StdVecVecStdOptionOptionStdPrimitiveI8Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveI16Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveI32Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveI64Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveI128Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveU8Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveU16Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveU32Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveU64Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveU128Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveF32Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveF64Json" => "",
                                "StdVecVecStdOptionOptionStdPrimitiveBoolJson" => "",
                                "StdVecVecStdOptionOptionStdStringStringJson" => "",

                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64Json" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBoolJson" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdStringStringJson" => "",

                                "GenericJson" => "",
                                "StdOptionOptionGenericJson" => "",
                                "StdVecVecGenericJson" => "",
                                "StdOptionOptionStdVecVecGenericJson" => "",
                                "StdVecVecStdOptionOptionStdGenericJson" => "",
                                "StdOptionOptionStdVecVecStdOptionOptionStdGenericJson" => "",
                                _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} unsupported predefined type wrapper: {segment_ident_stringified}")
                                //
                            };
                        }
                        None => panic!("{proc_macro_name_upper_camel_case_ident_stringified} field_type syn::Type::Path last path segment is None")
                    }
                    _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} field_type is not syn::Type::Path")
                }
                // proc_macro2::TokenStream::new()
                quote::quote!{

                }
            };
            quote::quote!{
                #[serde(rename(
                   serialize = #serialize_deserialize_field_ident_quotes_token_stream,
                   deserialize = #serialize_deserialize_field_ident_quotes_token_stream
                ))]
                #variant_ident_upper_camel_case_token_stream #maybe_additional_token_stream,
            }
        });
        let f = quote::quote!{#(#variants_token_stream),*};
        quote::quote!{
            // #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
            // pub enum #ident_field_upper_camel_case {
            //     #[serde(rename(
            //         serialize = "meow",
            //         deserialize = "meow"
            //     ))]
            //     Meow,
            //     #[serde(rename(
            //         serialize = "one",
            //         deserialize = "one"
            //     ))]
            //     One
            // }

    // #[serde(rename(
    //     serialize = "something",
    //     deserialize = "something"
    // ))]
    // Something,
    // #[serde(rename(
    //     serialize = "omega",
    //     deserialize = "omega"
    // ))]
    // Omega {
    //     limit: std::primitive::u64,
    //     offset: std::primitive::u64,
    // },
    // #[serde(rename(
    //     serialize = "doggie",
    //     deserialize = "doggie"
    // ))]
    // Doggie(std::vec::Vec<DoggieField>),
    // #[serde(rename(
    //     serialize = "cats",
    //     deserialize = "cats"
    // ))]
    // Cats {
    //     reader_vec: std::vec::Vec<CatField>,
    //     limit: std::primitive::u64,
    //     offset: std::primitive::u64,
    // }
        }
    };
    let impl_error_occurence_lib_to_std_string_string_for_ident_field_token_stream = {
        quote::quote!{
            // impl error_occurence_lib::ToStdStringString for #ident_field_upper_camel_case {
            //     fn to_std_string_string(&self) -> std::string::String {
            //         format!("{self:?}")
            //     }
            // }
        }
    };
    let pub_enum_field_generate_postgresql_query_part_error_named_token_stream = {
        quote::quote!{
            // #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            // pub enum CatGeneratePostgresqlQueryPartErrorNamed {
            //     OffsetPlusLimitIsIntOverflow {
            //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
            //     },
            //     FieldsFilterIsEmpty {
            //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
            //     },
            //     NotUniqueFieldFilter {
            //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
            //     }
            // }
        }
    };
    let impl_generate_postgresql_query_part_field_generate_postgresql_query_part_error_named_for_ident_field_token_stream = {
        quote::quote!{
            // impl GeneratePostgresqlQueryPart<CatGeneratePostgresqlQueryPartErrorNamed> for CatField {
            //     fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, CatGeneratePostgresqlQueryPartErrorNamed> {
            //         match self {
            //             Self::Meow => Ok(format!("'meow',{column_name_and_maybe_field_getter}->'meow'")),
            //             Self::One => Ok(format!("'one',{column_name_and_maybe_field_getter}->'one'")),
            //         }
            //     }
            // }
        }
    };
    let pub_struct_ident_options_token_stream = {
        quote::quote!{
            // #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
            // pub struct CatOptions {
            //     meow: std::option::Option<StdStringStringJson>,
            //     one: std::option::Option<StdStringStringJson>,
            // }
        }
    };
    let impl_std_convert_from_ident_for_ident_options_token_stream = {
        quote::quote!{
            // impl std::convert::From<Cat> for CatOptions {
            //     fn from(value: Cat) -> Self {
            //         Self {
            //             meow: Some(value.meow),
            //             one: Some(value.one),
            //         }
            //     }
            // }
        }
    };
    let gen = quote::quote!{
        // #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
        // pub struct Cat {
        //     pub meow: StdStringStringJson,
        //     pub one: StdStringStringJson,
        // }
        #pub_enum_ident_field_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_field_token_stream
        #pub_enum_field_generate_postgresql_query_part_error_named_token_stream
        #impl_generate_postgresql_query_part_field_generate_postgresql_query_part_error_named_for_ident_field_token_stream
        #pub_struct_ident_options_token_stream
        #impl_std_convert_from_ident_for_ident_options_token_stream
    };
    gen.into()
}