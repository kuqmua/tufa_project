
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
    StdPrimitiveI8, 
    StdPrimitiveI16, 
    StdPrimitiveI32, 
    StdPrimitiveI64, 
    StdPrimitiveI128,
    StdPrimitiveU8, 
    StdPrimitiveU16, 
    StdPrimitiveU32, 
    StdPrimitiveU64, 
    StdPrimitiveU128, 
    StdPrimitiveF32, 
    StdPrimitiveF64, 
    StdPrimitiveBool, 
    StdStringString, 

    StdOptionOptionStdPrimitiveI8, 
    StdOptionOptionStdPrimitiveI16, 
    StdOptionOptionStdPrimitiveI32, 
    StdOptionOptionStdPrimitiveI64, 
    StdOptionOptionStdPrimitiveI128, 
    StdOptionOptionStdPrimitiveU8, 
    StdOptionOptionStdPrimitiveU16, 
    StdOptionOptionStdPrimitiveU32, 
    StdOptionOptionStdPrimitiveU64, 
    StdOptionOptionStdPrimitiveU128, 
    StdOptionOptionStdPrimitiveF32, 
    StdOptionOptionStdPrimitiveF64, 
    StdOptionOptionStdPrimitiveBool, 
    StdOptionOptionStdStringString, 

    StdVecVecStdPrimitiveI8, 
    StdVecVecStdPrimitiveI16, 
    StdVecVecStdPrimitiveI32, 
    StdVecVecStdPrimitiveI64, 
    StdVecVecStdPrimitiveI128, 
    StdVecVecStdPrimitiveU8, 
    StdVecVecStdPrimitiveU16, 
    StdVecVecStdPrimitiveU32, 
    StdVecVecStdPrimitiveU64, 
    StdVecVecStdPrimitiveU128, 
    StdVecVecStdPrimitiveF32, 
    StdVecVecStdPrimitiveF64, 
    StdVecVecStdPrimitiveBool, 
    StdVecVecStdStringString, 

    StdOptionOptionStdVecVecStdPrimitiveI8, 
    StdOptionOptionStdVecVecStdPrimitiveI16, 
    StdOptionOptionStdVecVecStdPrimitiveI32, 
    StdOptionOptionStdVecVecStdPrimitiveI64, 
    StdOptionOptionStdVecVecStdPrimitiveI128, 
    StdOptionOptionStdVecVecStdPrimitiveU8, 
    StdOptionOptionStdVecVecStdPrimitiveU16, 
    StdOptionOptionStdVecVecStdPrimitiveU32, 
    StdOptionOptionStdVecVecStdPrimitiveU64, 
    StdOptionOptionStdVecVecStdPrimitiveU128, 
    StdOptionOptionStdVecVecStdPrimitiveF32, 
    StdOptionOptionStdVecVecStdPrimitiveF64, 
    StdOptionOptionStdVecVecStdPrimitiveBool, 
    StdOptionOptionStdVecVecStdStringString, 

    StdVecVecStdOptionOptionStdPrimitiveI8, 
    StdVecVecStdOptionOptionStdPrimitiveI16, 
    StdVecVecStdOptionOptionStdPrimitiveI32, 
    StdVecVecStdOptionOptionStdPrimitiveI64, 
    StdVecVecStdOptionOptionStdPrimitiveI128, 
    StdVecVecStdOptionOptionStdPrimitiveU8, 
    StdVecVecStdOptionOptionStdPrimitiveU16, 
    StdVecVecStdOptionOptionStdPrimitiveU32, 
    StdVecVecStdOptionOptionStdPrimitiveU64, 
    StdVecVecStdOptionOptionStdPrimitiveU128, 
    StdVecVecStdOptionOptionStdPrimitiveF32, 
    StdVecVecStdOptionOptionStdPrimitiveF64, 
    StdVecVecStdOptionOptionStdPrimitiveBool, 
    StdVecVecStdOptionOptionStdStringString, 

    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64, 
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool, 
    StdOptionOptionStdVecVecStdOptionOptionStdStringString, 

    Generic(&'a syn::AngleBracketedGenericArguments), 
    StdOptionOptionGeneric(&'a syn::AngleBracketedGenericArguments), 
    StdVecVecGeneric(&'a syn::AngleBracketedGenericArguments), 
    StdOptionOptionStdVecVecGeneric(&'a syn::AngleBracketedGenericArguments), 
    StdVecVecStdOptionOptionStdGeneric(&'a syn::AngleBracketedGenericArguments), 
    StdOptionOptionStdVecVecStdOptionOptionStdGeneric(&'a syn::AngleBracketedGenericArguments), 
}
#[derive(Debug)]
enum SupportedPredefinedTypeTryFromSynField {
    TypePathPathSegmentsIsNotSynTypePath,
    TypePathPathSegmentsLastIsNone,
    PathSegmentArgumentsIsNotSynPathArgumentsNone,
    PathSegmentArgumentsIsNotSynPathArgumentsAngleBracketed,
    UnsupportedPredefinedTypeWrapper,
}
impl<'a> std::convert::TryFrom<&'a syn::Field> for SupportedPredefinedType<'a> {
    type Error = SupportedPredefinedTypeTryFromSynField;
    fn try_from(value: &'a syn::Field) -> Result<Self, Self::Error> {
        match &value.ty {
            syn::Type::Path(type_path) => match type_path.path.segments.last() {
                Some(path_segment) => {
                    let supported_predefined_type = match path_segment.ident.to_string().as_str() {
                        "StdPrimitiveI8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveI16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdPrimitiveI32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveI32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdPrimitiveI64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveI64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdPrimitiveI128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveI128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdPrimitiveU8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveU8,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdPrimitiveU16" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveU16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdPrimitiveU32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveU32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdPrimitiveU64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveU64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdPrimitiveU128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveU128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdPrimitiveF32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveF32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdPrimitiveF64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveF64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdPrimitiveBool" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdPrimitiveBool,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdStringString" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdStringString,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },

                        "StdOptionOptionStdPrimitiveI8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveI8,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveI16" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveI16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveI32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveI32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveI64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveI64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveI128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveI128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveU8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveU8,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveU16" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveU16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveU32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveU32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveU64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveU64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveU128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveU128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveF32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveF32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveF64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveF64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdPrimitiveBool" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdPrimitiveBool,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdStringString" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdStringString,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },

                        "StdVecVecStdPrimitiveI8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveI8,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveI16" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveI16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveI32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveI32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveI64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveI64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveI128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveI128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveU8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveU8,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveU16" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveU16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveU32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveU32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveU64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveU64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveU128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveU128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveF32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveF32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveF64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveF64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdPrimitiveBool" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdPrimitiveBool,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdStringString" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdStringString,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },

                        "StdOptionOptionStdVecVecStdPrimitiveI8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveI8,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveI16" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveI16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveI32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveI32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveI64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveI64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveI128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveI128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveU8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveU8,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveU16" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveU16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveU32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveU32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveU64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveU64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveU128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveU128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveF32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveF32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveF64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveF64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdPrimitiveBool" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdPrimitiveBool,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdStringString" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdStringString,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },

                        "StdVecVecStdOptionOptionStdPrimitiveI8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveI8,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveI16" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveI16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveI32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveI32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveI64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveI64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveI128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveI128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveU8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveU8,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveU16" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveU16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveU32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveU32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveU64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveU64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveU128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveU128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveF32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveF32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveF64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveF64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdPrimitiveBool" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdPrimitiveBool,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdVecVecStdOptionOptionStdStringString" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdVecVecStdOptionOptionStdStringString,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },

                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },
                        "StdOptionOptionStdVecVecStdOptionOptionStdStringString" => match path_segment.arguments{
                            syn::PathArguments::None => Self::StdOptionOptionStdVecVecStdOptionOptionStdStringString,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                            },
                        },

                        "Generic" => Self::Generic(match &path_segment.arguments {
                            syn::PathArguments::AngleBracketed(value) => value,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsAngleBracketed);
                            },
                        }),
                        "StdOptionOptionGeneric" => Self::StdOptionOptionGeneric(match &path_segment.arguments {
                            syn::PathArguments::AngleBracketed(value) => value,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsAngleBracketed);
                            },
                        }),
                        "StdVecVecGeneric" => Self::StdVecVecGeneric(match &path_segment.arguments {
                            syn::PathArguments::AngleBracketed(value) => value,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsAngleBracketed);
                            },
                        }),
                        "StdOptionOptionStdVecVecGeneric" => Self::StdOptionOptionStdVecVecGeneric(match &path_segment.arguments {
                            syn::PathArguments::AngleBracketed(value) => value,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsAngleBracketed);
                            },
                        }),
                        "StdVecVecStdOptionOptionStdGeneric" => Self::StdVecVecStdOptionOptionStdGeneric(match &path_segment.arguments {
                            syn::PathArguments::AngleBracketed(value) => value,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsAngleBracketed);
                            },
                        }),
                        "StdOptionOptionStdVecVecStdOptionOptionStdGeneric" => Self::StdOptionOptionStdVecVecStdOptionOptionStdGeneric(match &path_segment.arguments {
                            syn::PathArguments::AngleBracketed(value) => value,
                            _ => {
                                return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsAngleBracketed);
                            },
                        }),
                        _ => {
                            return Err(Self::Error::UnsupportedPredefinedTypeWrapper);
                        }
                    };
                    Ok(supported_predefined_type)
                }
                None => Err(Self::Error::TypePathPathSegmentsLastIsNone),
            }
            _ => Err(Self::Error::TypePathPathSegmentsIsNotSynTypePath)
        }
    }
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
                let supported_predefined_type = SupportedPredefinedType::try_from(*element)
                    .unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
                println!("{supported_predefined_type:#?}");
                match supported_predefined_type {
                    SupportedPredefinedType::Generic(value) => {
                        quote::quote!{
                            // (std::vec::Vec<DoggieField>)
                        }
                    }, 
                    SupportedPredefinedType::StdOptionOptionGeneric(value) => {
                        quote::quote!{

                        }
                    }, 
                    SupportedPredefinedType::StdVecVecGeneric(value) => {
                        quote::quote!{

                        }
                    }, 
                    SupportedPredefinedType::StdOptionOptionStdVecVecGeneric(value) => {
                        quote::quote!{

                        }
                    }, 
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdGeneric(value) => {
                        quote::quote!{

                        }
                    }, 
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdGeneric(value) => {
                        quote::quote!{

                        }
                    },
                    _ => proc_macro2::TokenStream::new()
                };
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
            //     meow: std::option::Option<StdStringString>,
            //     one: std::option::Option<StdStringString>,
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
        //     pub meow: StdStringString,
        //     pub one: StdStringString,
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



// StdStringString
// StdVecVecStdPrimitiveBool
// Generic(
//     AngleBracketedGenericArguments {
//         colon2_token: None,
//         lt_token: Lt,
//         args: [
//             GenericArgument::Type(
//                 Type::Path {
//                     qself: None,
//                     path: Path {
//                         leading_colon: None,
//                         segments: [
//                             PathSegment {
//                                 ident: Ident {
//                                     ident: "Doggie",
//                                     span: #0 bytes(566941..566947),
//                                 },
//                                 arguments: PathArguments::None,
//                             },
//                         ],
//                     },
//                 },
//             ),
//         ],
//         gt_token: Gt,
//     },
// )
// StdVecVecGeneric(
//     AngleBracketedGenericArguments {
//         colon2_token: None,
//         lt_token: Lt,
//         args: [
//             GenericArgument::Type(
//                 Type::Path {
//                     qself: None,
//                     path: Path {
//                         leading_colon: None,
//                         segments: [
//                             PathSegment {
//                                 ident: Ident {
//                                     ident: "Cat",
//                                     span: #0 bytes(566985..566988),
//                                 },
//                                 arguments: PathArguments::None,
//                             },
//                         ],
//                     },
//                 },
//             ),
//         ],
//         gt_token: Gt,
//     },
// )
// StdStringString
// StdVecVecStdPrimitiveBool
// Generic(
//     AngleBracketedGenericArguments {
//         colon2_token: None,
//         lt_token: Lt,
//         args: [
//             GenericArgument::Type(
//                 Type::Path {
//                     qself: None,
//                     path: Path {
//                         leading_colon: None,
//                         segments: [
//                             PathSegment {
//                                 ident: Ident {
//                                     ident: "Doggie",
//                                     span: #0 bytes(566941..566947),
//                                 },
//                                 arguments: PathArguments::None,
//                             },
//                         ],
//                     },
//                 },
//             ),
//         ],
//         gt_token: Gt,
//     },
// )
// StdVecVecGeneric(
//     AngleBracketedGenericArguments {
//         colon2_token: None,
//         lt_token: Lt,
//         args: [
//             GenericArgument::Type(
//                 Type::Path {
//                     qself: None,
//                     path: Path {
//                         leading_colon: None,
//                         segments: [
//                             PathSegment {
//                                 ident: Ident {
//                                     ident: "Cat",
//                                     span: #0 bytes(566985..566988),
//                                 },
//                                 arguments: PathArguments::None,
//                             },
//                         ],
//                     },
//                 },
//             ),
//         ],
//         gt_token: Gt,
//     },
// )