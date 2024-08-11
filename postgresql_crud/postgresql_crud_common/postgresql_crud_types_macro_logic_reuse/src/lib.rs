
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
    let generated = quote::quote!{
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
    generated.into()
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
    let generated = quote::quote!{
        impl CheckSupportedRustAndPostgresqlColumnType for #ident {
            fn check_supported_rust_and_postgresql_column_type() {}
        }
    };
    generated.into()
}

///////////////
#[derive(Debug)]
enum SupportedPredefinedType {
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

    Generic(syn::TypePath), 
    StdOptionOptionGeneric(syn::TypePath), 
    StdVecVecGeneric(syn::TypePath), 
    StdOptionOptionStdVecVecGeneric(syn::TypePath), 
    StdVecVecStdOptionOptionGeneric(syn::TypePath), 
    StdOptionOptionStdVecVecStdOptionOptionGeneric(syn::TypePath), 
}
#[derive(Debug)]
enum SupportedPredefinedTypeTryFromSynField {
    TypePathPathSegmentsIsNotSynTypePath,
    TypePathPathSegmentsLastIsNone,
    PathSegmentArgumentsIsNotSynPathArgumentsNone,
    PathSegmentArgumentsIsNotSynPathArgumentsAngleBracketed,
    UnsupportedPredefinedTypeWrapper,
}
impl std::convert::TryFrom<&syn::Field> for SupportedPredefinedType {
    type Error = SupportedPredefinedTypeTryFromSynField;
    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        match &value.ty {
            syn::Type::Path(type_path) => match type_path.path.segments.last() {
                Some(path_segment) => {
                    let try_generate_generic_ident_upper_camel_case_token_stream = |path_segment: &syn::PathSegment|{
                        let proc_macro_name_upper_camel_case_ident_stringified = "123";//todo
                        match &path_segment.arguments {
                            syn::PathArguments::AngleBracketed(value) => {
                                if value.args.len() != 1 {
                                    panic!(
                                        "{proc_macro_name_upper_camel_case_ident_stringified} AngleBracketedGenericArguments args len != 1",
                                    );
                                }
                                match value.args.first() {
                                    Some(value) => if let syn::GenericArgument::Type(value) = value {
                                        match &value {
                                            syn::Type::Path(type_path) => {
                                                for element in &type_path.path.segments {
                                                    if let syn::PathArguments::None = element.arguments {}
                                                    else {
                                                        panic!(
                                                            "{proc_macro_name_upper_camel_case_ident_stringified} PathArguments::None",
                                                        );
                                                    }
                                                }
                                                type_path.clone()
                                                // println!("---{}---", quote::quote!{#type_path}.to-string());
                                                // match type_path.path.segments.last() {
                                                //     Some(path_segment) => {
                                                //         path_segment.ident.clone()
                                                //     }
                                                //     None => panic!(
                                                //         "{proc_macro_name_upper_camel_case_ident_stringified} GenericArgument::Type syn::Type::Path last is None",
                                                //     ),
                                                // }
                                            }
                                            _ => panic!(
                                                "{proc_macro_name_upper_camel_case_ident_stringified} GenericArgument::Type value is not syn::Type::Path",
                                            )
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_upper_camel_case_ident_stringified} GenericArgument is not GenericArgument::Type",
                                        );
                                    },
                                    None => panic!(
                                        "{proc_macro_name_upper_camel_case_ident_stringified} AngleBracketedGenericArguments args first is None",
                                    )
                                }
                            },
                            _ => {
                                panic!(
                                    "{proc_macro_name_upper_camel_case_ident_stringified} syn::PathArguments is not AngleBracketed",
                                )
                            }
                        }
                    };
                    let supported_predefined_type = match path_segment.ident.to_string().as_str() {
                        "StdPrimitiveI8" => Self::StdPrimitiveI16,
                        "StdPrimitiveI32" => Self::StdPrimitiveI32,
                        "StdPrimitiveI64" => Self::StdPrimitiveI64,
                        "StdPrimitiveI128" => Self::StdPrimitiveI128,
                        "StdPrimitiveU8" => Self::StdPrimitiveU8,
                        "StdPrimitiveU16" => Self::StdPrimitiveU16,
                        "StdPrimitiveU32" => Self::StdPrimitiveU32,
                        "StdPrimitiveU64" => Self::StdPrimitiveU64,
                        "StdPrimitiveU128" => Self::StdPrimitiveU128,
                        "StdPrimitiveF32" => Self::StdPrimitiveF32,
                        "StdPrimitiveF64" => Self::StdPrimitiveF64,
                        "StdPrimitiveBool" => Self::StdPrimitiveBool,
                        "StdStringString" => Self::StdStringString,

                        "StdOptionOptionStdPrimitiveI8" => Self::StdOptionOptionStdPrimitiveI8,
                        "StdOptionOptionStdPrimitiveI16" => Self::StdOptionOptionStdPrimitiveI16,
                        "StdOptionOptionStdPrimitiveI32" => Self::StdOptionOptionStdPrimitiveI32,
                        "StdOptionOptionStdPrimitiveI64" => Self::StdOptionOptionStdPrimitiveI64,
                        "StdOptionOptionStdPrimitiveI128" => Self::StdOptionOptionStdPrimitiveI128,
                        "StdOptionOptionStdPrimitiveU8" => Self::StdOptionOptionStdPrimitiveU8,
                        "StdOptionOptionStdPrimitiveU16" => Self::StdOptionOptionStdPrimitiveU16,
                        "StdOptionOptionStdPrimitiveU32" => Self::StdOptionOptionStdPrimitiveU32,
                        "StdOptionOptionStdPrimitiveU64" => Self::StdOptionOptionStdPrimitiveU64,
                        "StdOptionOptionStdPrimitiveU128" => Self::StdOptionOptionStdPrimitiveU128,
                        "StdOptionOptionStdPrimitiveF32" => Self::StdOptionOptionStdPrimitiveF32,
                        "StdOptionOptionStdPrimitiveF64" => Self::StdOptionOptionStdPrimitiveF64,
                        "StdOptionOptionStdPrimitiveBool" => Self::StdOptionOptionStdPrimitiveBool,
                        "StdOptionOptionStdStringString" => Self::StdOptionOptionStdStringString,

                        "StdVecVecStdPrimitiveI8" => Self::StdVecVecStdPrimitiveI8,
                        "StdVecVecStdPrimitiveI16" => Self::StdVecVecStdPrimitiveI16,
                        "StdVecVecStdPrimitiveI32" => Self::StdVecVecStdPrimitiveI32,
                        "StdVecVecStdPrimitiveI64" => Self::StdVecVecStdPrimitiveI64,
                        "StdVecVecStdPrimitiveI128" => Self::StdVecVecStdPrimitiveI128,
                        "StdVecVecStdPrimitiveU8" => Self::StdVecVecStdPrimitiveU8,
                        "StdVecVecStdPrimitiveU16" => Self::StdVecVecStdPrimitiveU16,
                        "StdVecVecStdPrimitiveU32" => Self::StdVecVecStdPrimitiveU32,
                        "StdVecVecStdPrimitiveU64" => Self::StdVecVecStdPrimitiveU64,
                        "StdVecVecStdPrimitiveU128" => Self::StdVecVecStdPrimitiveU128,
                        "StdVecVecStdPrimitiveF32" => Self::StdVecVecStdPrimitiveF32,
                        "StdVecVecStdPrimitiveF64" => Self::StdVecVecStdPrimitiveF64,
                        "StdVecVecStdPrimitiveBool" => Self::StdVecVecStdPrimitiveBool,
                        "StdVecVecStdStringString" => Self::StdVecVecStdStringString,

                        "StdOptionOptionStdVecVecStdPrimitiveI8" => Self::StdOptionOptionStdVecVecStdPrimitiveI8,
                        "StdOptionOptionStdVecVecStdPrimitiveI16" => Self::StdOptionOptionStdVecVecStdPrimitiveI16,
                        "StdOptionOptionStdVecVecStdPrimitiveI32" => Self::StdOptionOptionStdVecVecStdPrimitiveI32,
                        "StdOptionOptionStdVecVecStdPrimitiveI64" => Self::StdOptionOptionStdVecVecStdPrimitiveI64,
                        "StdOptionOptionStdVecVecStdPrimitiveI128" => Self::StdOptionOptionStdVecVecStdPrimitiveI128,
                        "StdOptionOptionStdVecVecStdPrimitiveU8" => Self::StdOptionOptionStdVecVecStdPrimitiveU8,
                        "StdOptionOptionStdVecVecStdPrimitiveU16" => Self::StdOptionOptionStdVecVecStdPrimitiveU16,
                        "StdOptionOptionStdVecVecStdPrimitiveU32" => Self::StdOptionOptionStdVecVecStdPrimitiveU32,
                        "StdOptionOptionStdVecVecStdPrimitiveU64" => Self::StdOptionOptionStdVecVecStdPrimitiveU64,
                        "StdOptionOptionStdVecVecStdPrimitiveU128" => Self::StdOptionOptionStdVecVecStdPrimitiveU128,
                        "StdOptionOptionStdVecVecStdPrimitiveF32" => Self::StdOptionOptionStdVecVecStdPrimitiveF32,
                        "StdOptionOptionStdVecVecStdPrimitiveF64" => Self::StdOptionOptionStdVecVecStdPrimitiveF64,
                        "StdOptionOptionStdVecVecStdPrimitiveBool" => Self::StdOptionOptionStdVecVecStdPrimitiveBool,
                        "StdOptionOptionStdVecVecStdStringString" => Self::StdOptionOptionStdVecVecStdStringString,

                        "StdVecVecStdOptionOptionStdPrimitiveI8" => Self::StdVecVecStdOptionOptionStdPrimitiveI8,
                        "StdVecVecStdOptionOptionStdPrimitiveI16" => Self::StdVecVecStdOptionOptionStdPrimitiveI16,
                        "StdVecVecStdOptionOptionStdPrimitiveI32" => Self::StdVecVecStdOptionOptionStdPrimitiveI32,
                        "StdVecVecStdOptionOptionStdPrimitiveI64" => Self::StdVecVecStdOptionOptionStdPrimitiveI64,
                        "StdVecVecStdOptionOptionStdPrimitiveI128" => Self::StdVecVecStdOptionOptionStdPrimitiveI128,
                        "StdVecVecStdOptionOptionStdPrimitiveU8" => Self::StdVecVecStdOptionOptionStdPrimitiveU8,
                        "StdVecVecStdOptionOptionStdPrimitiveU16" => Self::StdVecVecStdOptionOptionStdPrimitiveU16,
                        "StdVecVecStdOptionOptionStdPrimitiveU32" => Self::StdVecVecStdOptionOptionStdPrimitiveU32,
                        "StdVecVecStdOptionOptionStdPrimitiveU64" => Self::StdVecVecStdOptionOptionStdPrimitiveU64,
                        "StdVecVecStdOptionOptionStdPrimitiveU128" => Self::StdVecVecStdOptionOptionStdPrimitiveU128,
                        "StdVecVecStdOptionOptionStdPrimitiveF32" => Self::StdVecVecStdOptionOptionStdPrimitiveF32,
                        "StdVecVecStdOptionOptionStdPrimitiveF64" => Self::StdVecVecStdOptionOptionStdPrimitiveF64,
                        "StdVecVecStdOptionOptionStdPrimitiveBool" => Self::StdVecVecStdOptionOptionStdPrimitiveBool,
                        "StdVecVecStdOptionOptionStdStringString" => Self::StdVecVecStdOptionOptionStdStringString,

                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
                        "StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool" => Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
                        "StdOptionOptionStdVecVecStdOptionOptionStdStringString" => Self::StdOptionOptionStdVecVecStdOptionOptionStdStringString,

                        "Generic" => Self::Generic(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)),
                        "StdOptionOptionGeneric" => Self::StdOptionOptionGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)),
                        "StdVecVecGeneric" => Self::StdVecVecGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)),
                        "StdOptionOptionStdVecVecGeneric" => Self::StdOptionOptionStdVecVecGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)),
                        "StdVecVecStdOptionOptionGeneric" => Self::StdVecVecStdOptionOptionGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)),
                        "StdOptionOptionStdVecVecStdOptionOptionGeneric" => Self::StdOptionOptionStdVecVecStdOptionOptionGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)),
                        _ => {
                            return Err(Self::Error::UnsupportedPredefinedTypeWrapper);
                        }
                    };
                    match &supported_predefined_type {
                        SupportedPredefinedType::StdPrimitiveI8 |
                        SupportedPredefinedType::StdPrimitiveI16 |
                        SupportedPredefinedType::StdPrimitiveI32 |
                        SupportedPredefinedType::StdPrimitiveI64 |
                        SupportedPredefinedType::StdPrimitiveI128 |
                        SupportedPredefinedType::StdPrimitiveU8 |
                        SupportedPredefinedType::StdPrimitiveU16 |
                        SupportedPredefinedType::StdPrimitiveU32 |
                        SupportedPredefinedType::StdPrimitiveU64 |
                        SupportedPredefinedType::StdPrimitiveU128 |
                        SupportedPredefinedType::StdPrimitiveF32 |
                        SupportedPredefinedType::StdPrimitiveF64 |
                        SupportedPredefinedType::StdPrimitiveBool |
                        SupportedPredefinedType::StdStringString |

                        SupportedPredefinedType::StdOptionOptionStdPrimitiveI8 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveI16 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveI32 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveI64 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveI128 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveU8 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveU16 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveU32 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveU64 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveU128 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveF32 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveF64 |
                        SupportedPredefinedType::StdOptionOptionStdPrimitiveBool |
                        SupportedPredefinedType::StdOptionOptionStdStringString |

                        SupportedPredefinedType::StdVecVecStdPrimitiveI8 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveI16 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveI32 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveI64 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveI128 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveU8 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveU16 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveU32 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveU64 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveU128 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveF32 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveF64 |
                        SupportedPredefinedType::StdVecVecStdPrimitiveBool |
                        SupportedPredefinedType::StdVecVecStdStringString |

                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI8 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI16 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI32 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI64 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI128 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU8 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU16 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU32 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU64 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU128 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF32 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF64 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveBool |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdStringString |

                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI8 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI16 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI32 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI64 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI128 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU8 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU16 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU32 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU64 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU128 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF32 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF64 |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveBool |
                        SupportedPredefinedType::StdVecVecStdOptionOptionStdStringString |

                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                        SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdStringString
                        => {
                            match path_segment.arguments{
                                syn::PathArguments::None => (),
                                _ => {
                                    return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                                },
                            }
                        }
                        _ => (),
                    }
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
    let generate_ident_field_upper_camel_case_token_stream = |value: &std::primitive::str|{
        let value = format!("{value}{}", naming_conventions::FieldUpperCamelCase);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_ident_options_upper_camel_case_token_stream = |value: &std::primitive::str|{
        let value = format!("{value}{}", naming_conventions::OptionsUpperCamelCase);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_field_upper_camel_case_token_stream = generate_ident_field_upper_camel_case_token_stream(&ident.to_string());
    let options_upper_camel_case = naming_conventions::OptionsUpperCamelCase;
    let ident_options_upper_camel_case_token_stream = {
        let value = format!("{ident}{options_upper_camel_case}");
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let impl_std_fmt_display_for_ident_token_stream = {
        quote::quote!{
            impl std::fmt::Display for #ident {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(formatter, "{:?}", &self)
                }
            }
        }
    };
    let pub_enum_ident_field_token_stream = {
        let variants_token_stream = vec_syn_field.iter().map(|element|{
            let field_ident_stringified = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            }).to_string();
            let serialize_deserialize_field_ident_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
                &field_ident_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
            let maybe_additional_token_stream = {
                let supported_predefined_type = SupportedPredefinedType::try_from(*element)
                    .unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
                match supported_predefined_type {
                    SupportedPredefinedType::StdPrimitiveI8 |
                    SupportedPredefinedType::StdPrimitiveI16 |
                    SupportedPredefinedType::StdPrimitiveI32 |
                    SupportedPredefinedType::StdPrimitiveI64 |
                    SupportedPredefinedType::StdPrimitiveI128 |
                    SupportedPredefinedType::StdPrimitiveU8 |
                    SupportedPredefinedType::StdPrimitiveU16 |
                    SupportedPredefinedType::StdPrimitiveU32 |
                    SupportedPredefinedType::StdPrimitiveU64 |
                    SupportedPredefinedType::StdPrimitiveU128 |
                    SupportedPredefinedType::StdPrimitiveF32 |
                    SupportedPredefinedType::StdPrimitiveF64 |
                    SupportedPredefinedType::StdPrimitiveBool |
                    SupportedPredefinedType::StdStringString |

                    SupportedPredefinedType::StdOptionOptionStdPrimitiveI8 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveI16 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveI32 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveI64 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveI128 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveU8 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveU16 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveU32 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveU64 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveU128 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveF32 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveF64 |
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveBool |
                    SupportedPredefinedType::StdOptionOptionStdStringString
                    => proc_macro2::TokenStream::new(),

                    SupportedPredefinedType::StdVecVecStdPrimitiveI8 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveI16 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveI32 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveI64 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveI128 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveU8 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveU16 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveU32 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveU64 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveU128 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveF32 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveF64 |
                    SupportedPredefinedType::StdVecVecStdPrimitiveBool |
                    SupportedPredefinedType::StdVecVecStdStringString |

                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI8 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI16 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI32 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI64 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI128 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU8 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU16 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU32 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU64 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU128 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF32 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF64 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveBool |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdStringString |

                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI8 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI16 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI32 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI64 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI128 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU8 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU16 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU32 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU64 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU128 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF32 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF64 |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveBool |
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdStringString |

                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdStringString
                    => quote::quote!{
                        {
                            limit: std::primitive::u64,
                            offset: std::primitive::u64,
                        }
                    },

                    SupportedPredefinedType::Generic(type_path) => {
                        let generic_ident_field_upper_camel_case_token_stream = generate_ident_field_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            (std::vec::Vec<#generic_ident_field_upper_camel_case_token_stream>)
                        }
                    }
                    SupportedPredefinedType::StdOptionOptionGeneric(type_path) => {
                        let generic_ident_field_upper_camel_case_token_stream = generate_ident_field_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            (std::vec::Vec<#generic_ident_field_upper_camel_case_token_stream>)
                        }
                    }
                    SupportedPredefinedType::StdVecVecGeneric(type_path) => {
                        let generic_ident_field_upper_camel_case_token_stream = generate_ident_field_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            {
                                field_vec: std::vec::Vec<#generic_ident_field_upper_camel_case_token_stream>,
                                limit: std::primitive::u64,
                                offset: std::primitive::u64,
                            }
                        }
                    }
                    SupportedPredefinedType::StdOptionOptionStdVecVecGeneric(type_path) => {
                        let generic_ident_field_upper_camel_case_token_stream = generate_ident_field_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            {
                                field_vec: std::vec::Vec<#generic_ident_field_upper_camel_case_token_stream>,
                                limit: std::primitive::u64,
                                offset: std::primitive::u64,
                            }
                        }
                    }
                    SupportedPredefinedType::StdVecVecStdOptionOptionGeneric(type_path) => {
                        let generic_ident_field_upper_camel_case_token_stream = generate_ident_field_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            {
                                field_vec: std::vec::Vec<#generic_ident_field_upper_camel_case_token_stream>,
                                limit: std::primitive::u64,
                                offset: std::primitive::u64,
                            }
                        }
                    }
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                        let generic_ident_field_upper_camel_case_token_stream = generate_ident_field_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            {
                                field_vec: std::vec::Vec<#generic_ident_field_upper_camel_case_token_stream>,
                                limit: std::primitive::u64,
                                offset: std::primitive::u64,
                            }
                        }
                    }
                }
            };
            quote::quote!{
                #[serde(rename(
                   serialize = #serialize_deserialize_field_ident_quotes_token_stream,
                   deserialize = #serialize_deserialize_field_ident_quotes_token_stream
                ))]
                #variant_ident_upper_camel_case_token_stream #maybe_additional_token_stream
            }
        });
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
            pub enum #ident_field_upper_camel_case_token_stream {
                #(#variants_token_stream),*
            }
        }
    };
    let impl_error_occurence_lib_to_std_string_string_for_ident_field_token_stream = {
        quote::quote!{
            impl error_occurence_lib::ToStdStringString for #ident_field_upper_camel_case_token_stream {
                fn to_std_string_string(&self) -> std::string::String {
                    format!("{self:?}")
                }
            }
        }
    };
    let pub_enum_field_generate_postgresql_query_part_error_named_token_stream = {
        let ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream = {
            let value = format!("{ident}SomethingGeneratePostgresqlQueryPartErrorNamed");
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let ident_generate_postgresql_query_part_from_self_vec_error_named_token_stream = {
            let ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case = {
                let value = format!("{ident}GeneratePostgresqlQueryPartFromSelfVecErrorNamed");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case {
                    FieldsFilterIsEmpty {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    NotUniqueFieldFilter {
                        #[eo_to_std_string_string_serialize_deserialize]
                        field: #ident_field_upper_camel_case_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    GeneratePostgresqlQueryPart {
                        #[eo_error_occurence]
                        error: #ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                }
            }
        };
        let ident_generate_postgresql_query_part_error_named_token_stream = {
            //
            // let f = match element {
            //     StdPrimitiveI8, 
            //     StdPrimitiveI16, 
            //     StdPrimitiveI32, 
            //     StdPrimitiveI64, 
            //     StdPrimitiveI128,
            //     StdPrimitiveU8, 
            //     StdPrimitiveU16, 
            //     StdPrimitiveU32, 
            //     StdPrimitiveU64, 
            //     StdPrimitiveU128, 
            //     StdPrimitiveF32, 
            //     StdPrimitiveF64, 
            //     StdPrimitiveBool, 
            //     StdStringString,            

            //     StdOptionOptionStdPrimitiveI8, 
            //     StdOptionOptionStdPrimitiveI16, 
            //     StdOptionOptionStdPrimitiveI32, 
            //     StdOptionOptionStdPrimitiveI64, 
            //     StdOptionOptionStdPrimitiveI128, 
            //     StdOptionOptionStdPrimitiveU8, 
            //     StdOptionOptionStdPrimitiveU16, 
            //     StdOptionOptionStdPrimitiveU32, 
            //     StdOptionOptionStdPrimitiveU64, 
            //     StdOptionOptionStdPrimitiveU128, 
            //     StdOptionOptionStdPrimitiveF32, 
            //     StdOptionOptionStdPrimitiveF64, 
            //     StdOptionOptionStdPrimitiveBool, 
            //     StdOptionOptionStdStringString,             

            //     StdVecVecStdPrimitiveI8, 
            //     StdVecVecStdPrimitiveI16, 
            //     StdVecVecStdPrimitiveI32, 
            //     StdVecVecStdPrimitiveI64, 
            //     StdVecVecStdPrimitiveI128, 
            //     StdVecVecStdPrimitiveU8, 
            //     StdVecVecStdPrimitiveU16, 
            //     StdVecVecStdPrimitiveU32, 
            //     StdVecVecStdPrimitiveU64, 
            //     StdVecVecStdPrimitiveU128, 
            //     StdVecVecStdPrimitiveF32, 
            //     StdVecVecStdPrimitiveF64, 
            //     StdVecVecStdPrimitiveBool, 
            //     StdVecVecStdStringString,           

            //     StdOptionOptionStdVecVecStdPrimitiveI8, 
            //     StdOptionOptionStdVecVecStdPrimitiveI16, 
            //     StdOptionOptionStdVecVecStdPrimitiveI32, 
            //     StdOptionOptionStdVecVecStdPrimitiveI64, 
            //     StdOptionOptionStdVecVecStdPrimitiveI128, 
            //     StdOptionOptionStdVecVecStdPrimitiveU8, 
            //     StdOptionOptionStdVecVecStdPrimitiveU16, 
            //     StdOptionOptionStdVecVecStdPrimitiveU32, 
            //     StdOptionOptionStdVecVecStdPrimitiveU64, 
            //     StdOptionOptionStdVecVecStdPrimitiveU128, 
            //     StdOptionOptionStdVecVecStdPrimitiveF32, 
            //     StdOptionOptionStdVecVecStdPrimitiveF64, 
            //     StdOptionOptionStdVecVecStdPrimitiveBool, 
            //     StdOptionOptionStdVecVecStdStringString,            

            //     StdVecVecStdOptionOptionStdPrimitiveI8, 
            //     StdVecVecStdOptionOptionStdPrimitiveI16, 
            //     StdVecVecStdOptionOptionStdPrimitiveI32, 
            //     StdVecVecStdOptionOptionStdPrimitiveI64, 
            //     StdVecVecStdOptionOptionStdPrimitiveI128, 
            //     StdVecVecStdOptionOptionStdPrimitiveU8, 
            //     StdVecVecStdOptionOptionStdPrimitiveU16, 
            //     StdVecVecStdOptionOptionStdPrimitiveU32, 
            //     StdVecVecStdOptionOptionStdPrimitiveU64, 
            //     StdVecVecStdOptionOptionStdPrimitiveU128, 
            //     StdVecVecStdOptionOptionStdPrimitiveF32, 
            //     StdVecVecStdOptionOptionStdPrimitiveF64, 
            //     StdVecVecStdOptionOptionStdPrimitiveBool, 
            //     StdVecVecStdOptionOptionStdStringString,            

            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool, 
            //     StdOptionOptionStdVecVecStdOptionOptionStdStringString,             

            //     Generic(proc_macro2::TokenStream), 
            //     StdOptionOptionGeneric(proc_macro2::TokenStream), 
            //     StdVecVecGeneric(proc_macro2::TokenStream), 
            //     StdOptionOptionStdVecVecGeneric(proc_macro2::TokenStream), 
            //     StdVecVecStdOptionOptionGeneric(proc_macro2::TokenStream), 
            //     StdOptionOptionStdVecVecStdOptionOptionGeneric(proc_macro2::TokenStream), 
            // };
            //
            quote::quote!{
                #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream {
                    OffsetPlusLimitIsIntOverflow {
                        #[eo_to_std_string_string_serialize_deserialize]
                        limit: std::primitive::u64,
                        #[eo_to_std_string_string_serialize_deserialize]
                        offset: std::primitive::u64,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    FieldsFilterIsEmpty {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    NotUniqueStdOptionOptionGenericFieldFilter {
                        #[eo_to_std_string_string_serialize_deserialize]
                        field: #ident_field_upper_camel_case_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    DoggieGeneratePostgresqlQueryPartFromSelfVec {
                        #[eo_error_occurence]
                        field: DoggieGeneratePostgresqlQueryPartFromSelfVecErrorNamed,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                }
            }
        };
        quote::quote!{
            #ident_generate_postgresql_query_part_from_self_vec_error_named_token_stream
            #ident_generate_postgresql_query_part_error_named_token_stream
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
        let fields_token_stream = vec_syn_field.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let type_token_stream = match SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) 
            {
                SupportedPredefinedType::StdPrimitiveI8 => quote::quote!{std::primitive::bool},
                SupportedPredefinedType::StdPrimitiveI16 => quote::quote!{std::primitive::i16},
                SupportedPredefinedType::StdPrimitiveI32 => quote::quote!{std::primitive::i32},
                SupportedPredefinedType::StdPrimitiveI64 => quote::quote!{std::primitive::i64},
                SupportedPredefinedType::StdPrimitiveI128 => quote::quote!{std::primitive::i128},
                SupportedPredefinedType::StdPrimitiveU8 => quote::quote!{std::primitive::u8},
                SupportedPredefinedType::StdPrimitiveU16 => quote::quote!{std::primitive::u16},
                SupportedPredefinedType::StdPrimitiveU32 => quote::quote!{std::primitive::u32},
                SupportedPredefinedType::StdPrimitiveU64 => quote::quote!{std::primitive::u64},
                SupportedPredefinedType::StdPrimitiveU128 => quote::quote!{std::primitive::u128},
                SupportedPredefinedType::StdPrimitiveF32 => quote::quote!{std::primitive::f32},
                SupportedPredefinedType::StdPrimitiveF64 => quote::quote!{std::primitive::f64},
                SupportedPredefinedType::StdPrimitiveBool => quote::quote!{std::primitive::bool},
                SupportedPredefinedType::StdStringString => quote::quote!{std::string::String},

                SupportedPredefinedType::StdOptionOptionStdPrimitiveI8 => quote::quote!{std::option::Option<std::primitive::i8>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveI16 => quote::quote!{std::option::Option<std::primitive::i16>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveI32 => quote::quote!{std::option::Option<std::primitive::i32>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveI64 => quote::quote!{std::option::Option<std::primitive::i64>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveI128 => quote::quote!{std::option::Option<std::primitive::i128>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveU8 => quote::quote!{std::option::Option<std::primitive::u8>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveU16 => quote::quote!{std::option::Option<std::primitive::u16>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveU32 => quote::quote!{std::option::Option<std::primitive::u32>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveU64 => quote::quote!{std::option::Option<std::primitive::u64>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveU128 => quote::quote!{std::option::Option<std::primitive::u128>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveF32 => quote::quote!{std::option::Option<std::primitive::f32>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveF64 => quote::quote!{std::option::Option<std::primitive::f64>},
                SupportedPredefinedType::StdOptionOptionStdPrimitiveBool => quote::quote!{std::option::Option<std::primitive::bool>},
                SupportedPredefinedType::StdOptionOptionStdStringString => quote::quote!{std::option::Option<std::string::String>},

                SupportedPredefinedType::StdVecVecStdPrimitiveI8 => quote::quote!{std::vec::Vec<std::primitive::i8>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI16 => quote::quote!{std::vec::Vec<std::primitive::i16>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI32 => quote::quote!{std::vec::Vec<std::primitive::i32>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI64 => quote::quote!{std::vec::Vec<std::primitive::i64>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI128 => quote::quote!{std::vec::Vec<std::primitive::i128>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU8 => quote::quote!{std::vec::Vec<std::primitive::u8>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU16 => quote::quote!{std::vec::Vec<std::primitive::u16>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU32 => quote::quote!{std::vec::Vec<std::primitive::u32>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU64 => quote::quote!{std::vec::Vec<std::primitive::u64>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU128 => quote::quote!{std::vec::Vec<std::primitive::u128>},
                SupportedPredefinedType::StdVecVecStdPrimitiveF32 => quote::quote!{std::vec::Vec<std::primitive::f32>},
                SupportedPredefinedType::StdVecVecStdPrimitiveF64 => quote::quote!{std::vec::Vec<std::primitive::f64>},
                SupportedPredefinedType::StdVecVecStdPrimitiveBool => quote::quote!{std::vec::Vec<std::primitive::bool>},
                SupportedPredefinedType::StdVecVecStdStringString => quote::quote!{std::vec::Vec<std::string::String>},

                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI8 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::i8>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI16 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::i16>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI32 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::i32>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI64 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::i64>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI128 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::i128>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU8 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::u8>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU16 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::u16>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU32 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::u32>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU64 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::u64>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU128 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::u128>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF32 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::f32>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF64 => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::f64>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveBool => quote::quote!{std::option::Option<std::vec::Vec<std::primitive::bool>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdStringString => quote::quote!{std::option::Option<std::vec::Vec<std::string::String>>},

                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI8 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::i8>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI16 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::i16>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI32 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::i32>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI64 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::i64>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI128 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::i128>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU8 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::u8>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU16 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::u16>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU32 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::u32>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU64 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::u64>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU128 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::u128>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF32 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::f32>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF64 => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::f64>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveBool => quote::quote!{std::vec::Vec<std::option::Option<std::primitive::bool>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdStringString => quote::quote!{std::vec::Vec<std::option::Option<std::string::String>>},

                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i128>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u128>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdStringString => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>},

                SupportedPredefinedType::Generic(type_path) => generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string()),
                SupportedPredefinedType::StdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<#generic_ident_options_upper_camel_case_token_stream>}
                }
                SupportedPredefinedType::StdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::vec::Vec<#generic_ident_options_upper_camel_case_token_stream>}
                }
                SupportedPredefinedType::StdOptionOptionStdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<std::vec::Vec<#generic_ident_options_upper_camel_case_token_stream>>}
                }
                SupportedPredefinedType::StdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::vec::Vec<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>>}
                }
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>>>}
                }
            };
            quote::quote!{
                #element_ident: std::option::Option<crate::value::Value<#type_token_stream>>
            }
        });
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
            pub struct #ident_options_upper_camel_case_token_stream {
                #(#fields_token_stream),*

                // std_string_string: std::option::Option<crate::value::Value<std::string::String>>,
                // std_vec_vec_std_primitive_bool: std::option::Option<crate::value::Value<std::vec::Vec<std::primitive::bool>>>,
                // generic: std::option::Option<crate::value::Value<DoggieOptions>>,
                // std_option_option_generic: std::option::Option<crate::value::Value<std::option::Option<DoggieOptions>>>,
                // std_vec_vec_generic: std::option::Option<crate::value::Value<std::vec::Vec<DoggieOptions>>>,
                // std_option_option_std_vec_vec_generic: std::option::Option<crate::value::Value<std::option::Option<std::vec::Vec<DoggieOptions>>>>,
                // std_vec_vec_std_option_option_generic: std::option::Option<crate::value::Value<std::vec::Vec<std::option::Option<DoggieOptions>>>>,
                // std_option_option_std_vec_vec_std_option_option_generic: std::option::Option<crate::value::Value<std::option::Option<std::vec::Vec<std::option::Option<DoggieOptions>>>>>,

                // std_string_string: std::option::Option<std::result::Result<std::string::String, std::string::String>>,
                // std_vec_vec_std_primitive_bool: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>, std::string::String>>,
                // generic: std::option::Option<std::result::Result<DoggieOptions,std::string::String>>,
                // std_option_option_generic: std::option::Option<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>,
                // std_vec_vec_generic: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>,std::string::String>>,
                // std_option_option_std_vec_vec_generic: std::option::Option<std::result::Result<
                //     std::option::Option<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>>,
                //     std::string::String
                // >>,
                // std_vec_vec_std_option_option_generic: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>,std::string::String>>,
                // std_option_option_std_vec_vec_std_option_option_generic: std::option::Option<
                //     std::result::Result<
                //         std::option::Option<
                //             std::vec::Vec<
                //                 std::result::Result<
                //                     std::option::Option<DoggieOptions>,
                //                     std::string::String
                //                 >
                //             >
                //         >,
                //         std::string::String
                //     >
                // >,
            }
        }
    };
    let impl_std_convert_from_ident_for_ident_options_token_stream = {
        let fields_token_stream = vec_syn_field.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let conversion_token_stream = match SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) {
                SupportedPredefinedType::StdPrimitiveI8 |
                SupportedPredefinedType::StdPrimitiveI16 |
                SupportedPredefinedType::StdPrimitiveI32 |
                SupportedPredefinedType::StdPrimitiveI64 |
                SupportedPredefinedType::StdPrimitiveI128 |
                SupportedPredefinedType::StdPrimitiveU8 |
                SupportedPredefinedType::StdPrimitiveU16 |
                SupportedPredefinedType::StdPrimitiveU32 |
                SupportedPredefinedType::StdPrimitiveU64 |
                SupportedPredefinedType::StdPrimitiveU128 |
                SupportedPredefinedType::StdPrimitiveF32 |
                SupportedPredefinedType::StdPrimitiveF64 |
                SupportedPredefinedType::StdPrimitiveBool |
                SupportedPredefinedType::StdStringString
                => quote::quote!{value.#element_ident.0},

                SupportedPredefinedType::StdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::StdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::StdOptionOptionStdStringString
                => quote::quote!{
                    match value.#element_ident.0 {
                        Some(value) => Some(value),
                        None => None,
                    }
                },

                SupportedPredefinedType::StdVecVecStdPrimitiveI8 |
                SupportedPredefinedType::StdVecVecStdPrimitiveI16 |
                SupportedPredefinedType::StdVecVecStdPrimitiveI32 |
                SupportedPredefinedType::StdVecVecStdPrimitiveI64 |
                SupportedPredefinedType::StdVecVecStdPrimitiveI128 |
                SupportedPredefinedType::StdVecVecStdPrimitiveU8 |
                SupportedPredefinedType::StdVecVecStdPrimitiveU16 |
                SupportedPredefinedType::StdVecVecStdPrimitiveU32 |
                SupportedPredefinedType::StdVecVecStdPrimitiveU64 |
                SupportedPredefinedType::StdVecVecStdPrimitiveU128 |
                SupportedPredefinedType::StdVecVecStdPrimitiveF32 |
                SupportedPredefinedType::StdVecVecStdPrimitiveF64 |
                SupportedPredefinedType::StdVecVecStdPrimitiveBool |
                SupportedPredefinedType::StdVecVecStdStringString
                => quote::quote!{value.#element_ident.0},

                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI8 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI16 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI32 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI64 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI128 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU8 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU16 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU32 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU64 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU128 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF32 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF64 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveBool |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdStringString
                => quote::quote!{
                    match value.#element_ident.0 {
                        Some(value) => Some(value),
                        None => None
                    }
                },

                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::StdVecVecStdOptionOptionStdStringString
                => quote::quote!{
                    value.#element_ident.0.into_iter().map(|element|match element {
                        Some(value) => Some(value),
                        None => None
                    }).collect()
                },

                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdStringString
                => quote::quote!{
                    match value.#element_ident.0 {
                        Some(value) => Some(value.into_iter().map(|element|match element {
                            Some(value) => Some(value),
                            None => None
                        }).collect()),
                        None => None
                    }
                },

                SupportedPredefinedType::Generic(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        #generic_ident_options_upper_camel_case_token_stream::from(value.#element_ident.0)
                    }
                }
                SupportedPredefinedType::StdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        match value.#element_ident.0 {
                            Some(value) => Some(#generic_ident_options_upper_camel_case_token_stream::from(value)),
                            None => None,
                        }
                    }
                }
                SupportedPredefinedType::StdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        value.#element_ident.0.into_iter().map(|element|#generic_ident_options_upper_camel_case_token_stream::from(element)).collect::<std::vec::Vec<#generic_ident_options_upper_camel_case_token_stream>>()
                    }
                }
                SupportedPredefinedType::StdOptionOptionStdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        match value.#element_ident.0 {
                            Some(value) => Some(value.into_iter().map(|element|#generic_ident_options_upper_camel_case_token_stream::from(element)).collect::<std::vec::Vec<#generic_ident_options_upper_camel_case_token_stream>>()),
                            None => None
                        }
                    }
                }
                SupportedPredefinedType::StdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        value.#element_ident.0.into_iter().map(|element|match element {
                            Some(value) => Some(#generic_ident_options_upper_camel_case_token_stream::from(value)),
                            None => None
                        }).collect::<std::vec::Vec<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>>>()
                    }
                }
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        match value.#element_ident.0 {
                            Some(value) => Some(value.into_iter().map(|element|match element {
                                Some(value) => Some(#generic_ident_options_upper_camel_case_token_stream::from(value)),
                                None => None
                            }).collect::<std::vec::Vec<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>>>()),
                            None => None
                        }
                    }
                }
            };
            quote::quote!{
                #element_ident: Some(crate::value::Value{ value: #conversion_token_stream })
            }
        });
        quote::quote!{
            impl std::convert::From<#ident> for #ident_options_upper_camel_case_token_stream {
                fn from(value: #ident) -> Self {
                    Self {
                        #(#fields_token_stream),*

                        //just for case if must return result impl
                        // // std_string_string: Some(std::result::Result::Ok(value.std_string_string.0)),
                        // // std_vec_vec_std_primitive_bool: Some(std::result::Result::Ok(
                        // //     value.std_vec_vec_std_primitive_bool.0.into_iter().map(|element|
                        // //         std::result::Result::Ok(element)
                        // //     ).collect::<std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>>()
                        // // )),
                        // // generic: Some(std::result::Result::Ok(DoggieOptions::from(value.generic.0))),
                        // // //todo rewrite to from or try from impl
                        // // std_option_option_generic: Some(std::result::Result::Ok(Some(match value.std_option_option_generic.0 {
                        // //     Some(value) => DoggieOptions {
                        // //         std_string_string: Some(crate::value::Value{ value: value.std_string_string.0 }),
                        // //     },
                        // //     None => DoggieOptions {
                        // //         std_string_string: None,
                        // //     },
                        // // }))),
                        // // std_vec_vec_generic: Some(std::result::Result::Ok(value.std_vec_vec_generic.0.into_iter().map(|element|std::result::Result::Ok(DoggieOptions::from(element))).collect::<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>>())),
                        // // std_option_option_std_vec_vec_generic: Some(std::result::Result::Ok(match value.std_option_option_std_vec_vec_generic.0 {
                        // //         Some(value) => Some(value.into_iter().map(|element|std::result::Result::Ok(DoggieOptions::from(element))).collect::<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>>()),
                        // //         None => None
                        // // })),
                        // // std_vec_vec_std_option_option_generic: Some(std::result::Result::Ok(value.std_vec_vec_std_option_option_generic.0.into_iter().map(|element|std::result::Result::Ok(match element {
                        // //     Some(value) => Some(DoggieOptions::from(value)),
                        // //     None => None
                        // // })).collect::<std::vec::Vec<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>>())),
                        // // std_option_option_std_vec_vec_std_option_option_generic: Some(
                        // //     std::result::Result::Ok(
                        // //         match value.std_option_option_std_vec_vec_std_option_option_generic.0 {
                        // //             Some(value) => Some(value.into_iter().map(|element|std::result::Result::Ok(match element {
                        // //                 Some(value) => Some(DoggieOptions::from(value)),
                        // //                 None => None
                        // //             })).collect::<std::vec::Vec<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>>()),
                        // //             None => None
                        // //         }
                        // //     )
                        // // ),
                    }
                }
            }
        }
    };
    let generated = quote::quote!{
        #impl_std_fmt_display_for_ident_token_stream
        #pub_enum_ident_field_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_field_token_stream
        // #pub_enum_field_generate_postgresql_query_part_error_named_token_stream
        #impl_generate_postgresql_query_part_field_generate_postgresql_query_part_error_named_for_ident_field_token_stream
        #pub_struct_ident_options_token_stream
        #impl_std_convert_from_ident_for_ident_options_token_stream
    };
    generated.into()
}