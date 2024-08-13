
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
enum JsonType {
    Boolean,
    Number,
    String,
    Array,
    Object,
    Null
}
impl std::fmt::Display for JsonType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Boolean => write!(f, "boolean"),
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),
            Self::Array => write!(f, "array"),
            Self::Object => write!(f, "object"),
            Self::Null => write!(f, "null"),
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
    let ident_options_upper_camel_case_stringified = format!("{ident}{}", naming_conventions::OptionsUpperCamelCase);
    let ident_options_upper_camel_case_token_stream = {
        ident_options_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_options_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_options_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
        &ident_options_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let ident_wrapper_upper_camel_case_stringified = format!("{ident}{}", naming_conventions::WrapperUpperCamelCase);
    let ident_wrapper_upper_camel_case_token_stream = {
        ident_wrapper_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_wrapper_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream = {
        let value = format!("{ident}GeneratePostgresqlQueryPartErrorNamed");
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream = {
        let value = format!("{ident}GeneratePostgresqlQueryPartFromSelfVecErrorNamed");
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = |value: &std::primitive::str|{
        let value = format!("{value}GeneratePostgresqlQueryPartFromSelfVec");
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
            let serialize_deserialize_field_ident_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
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
                   serialize = #serialize_deserialize_field_ident_double_quotes_token_stream,
                   deserialize = #serialize_deserialize_field_ident_double_quotes_token_stream
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
        let ident_generate_postgresql_query_part_from_self_vec_error_named_token_stream = {
            quote::quote!{
                #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream {
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
    let impl_serde_deserialize_for_ident_options_token_stream = {
        let field_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, _)|{
            let value = format!(
                "__{}{index}",
                naming_conventions::FieldSnakeCase,
            );
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        });
        let visit_u64_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, _)|{
            let index_u64_token_stream = {
                let value = format!("{index}u64");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let field_index_token_stream = {
                let value = format!("__field{index}");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #index_u64_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)
            }
        });
        let visit_str_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_name_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
                &{
                     element.ident.as_ref().unwrap_or_else(|| {
                        panic!(
                            "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                            naming_conventions::FIELD_IDENT_IS_NONE
                        );
                    }).to_string()
                },
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let field_index_token_stream = {
                let value = format!("__field{index}");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #field_name_double_quotes_token_stream=> serde::__private::Ok(__Field::#field_index_token_stream)
            }
        });
        let visit_bytes_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let b_field_name_double_quotes_token_stream= {
                let element_ident_double_quotes_stringified = proc_macro_common::generate_quotes::double_quotes_stringified(
                    &element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    }).to_string()
                );
                let value = format!("b{element_ident_double_quotes_stringified}");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let field_index_token_stream = {
                let value = format!("__field{index}");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #b_field_name_double_quotes_token_stream=> serde::__private::Ok(__Field::#field_index_token_stream)
            }
        });
        let struct_ident_options_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!("struct {ident_options_upper_camel_case_stringified}"),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let struct_ident_options_with_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!(
                "struct {ident_options_upper_camel_case_stringified} with {} elements",
                vec_syn_field.len()
            ),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let visit_seq_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_index_token_stream = {
                let value = format!("__field{index}");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let index_usize_token_stream = {
                let value = format!("{index}usize");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
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

                SupportedPredefinedType::StdVecVecStdPrimitiveI8 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i8,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI16 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i16,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI32 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i32,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI64 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i64,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI128 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i128,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU8 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u8,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU16 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u16,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU32 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u32,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU64 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u64,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU128 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u128,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveF32 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::f32,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveF64 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::f64,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveBool => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdStringString => quote::quote!{std::vec::Vec<std::result::Result<std::string::String,std::string::String>>},

                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i8,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i16,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i32,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i64,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i128,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u8,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u16,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u32,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u64,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u128,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::f32,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::f64,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveBool => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdStringString => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::string::String,std::string::String>>>},

                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI8 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI16 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i16>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i32>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i64>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI128 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i128>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU8 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u8>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU16 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u16>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u32>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u64>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU128 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u128>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f32>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f64>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveBool => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::bool>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdStringString => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::string::String>,std::string::String>>},

                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i16>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i32>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i64>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i128>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u8>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u16>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u32>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u64>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u128>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f32>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f64>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::bool>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdStringString => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::string::String>,std::string::String>>},

                SupportedPredefinedType::Generic(type_path) => generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string()),
                SupportedPredefinedType::StdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<#generic_ident_options_upper_camel_case_token_stream>}
                }
                SupportedPredefinedType::StdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::vec::Vec<std::result::Result<#generic_ident_options_upper_camel_case_token_stream,std::string::String>>}
                }
                SupportedPredefinedType::StdOptionOptionStdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<#generic_ident_options_upper_camel_case_token_stream,std::string::String>>>}
                }
                SupportedPredefinedType::StdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>,std::string::String>>}
                }
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>,std::string::String>>>}
                }
            };
            quote::quote!{
                let #field_index_token_stream = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<#type_token_stream, std::string::String>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                #index_usize_token_stream,
                                &#struct_ident_options_with_double_quotes_token_stream,
                            ),
                        );
                    }
                };
            }
        });
        let visit_seq_fields_assignment_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let field_index_token_stream = {
                let value = format!("__field{index}");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let index_usize_token_stream = {
                let value = format!("{index}usize");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let conversion_logic_token_stream = match SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) 
            {
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
                => quote::quote!{value},

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
                => quote::quote!{value},

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
                => quote::quote!{
                    {
                        let mut acc = vec![];
                        for element in value {
                            match element {
                                Ok(value) => {
                                    acc.push(value);
                                }
                                Err(error) => {
                                    return Err(serde::de::Error::custom(error));
                                }
                            }
                        }
                        acc
                    }
                },

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
                    match value {
                        Some(value) => {
                            let mut acc = vec![];
                            for element in value {
                                match element {
                                    Ok(value) => {
                                        acc.push(value);
                                    }
                                    Err(error) => {
                                        return Err(serde::de::Error::custom(error));
                                    }
                                }
                            }
                            Some(acc)
                        }
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
                    {
                        let mut acc = vec![];
                        for element in value {
                            match element {
                                Ok(value) => {
                                    acc.push(value);
                                }
                                Err(error) => {
                                    return Err(serde::de::Error::custom(error));
                                }
                            }
                        }
                        acc
                    }
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
                    match value {
                        Some(value) => {
                            let mut acc = vec![];
                            for element in value {
                                match element {
                                    Ok(value) => {
                                        acc.push(value);
                                    }
                                    Err(error) => {
                                        return Err(serde::de::Error::custom(error));
                                    }
                                }
                            }
                            Some(acc)
                        }
                        None => None
                    }
                },

                SupportedPredefinedType::Generic(_) => quote::quote!{value},
                SupportedPredefinedType::StdOptionOptionGeneric(_) => quote::quote!{value},
                SupportedPredefinedType::StdVecVecGeneric(_) => quote::quote!{
                    {
                        let mut acc = vec![];
                        for element in value {
                            match element {
                                Ok(value) => {
                                    acc.push(value);
                                }
                                Err(error) => {
                                    return Err(serde::de::Error::custom(error));
                                }
                            }
                        }
                        acc
                    }
                },
                SupportedPredefinedType::StdOptionOptionStdVecVecGeneric(_) => quote::quote!{
                    match value {
                        Some(value) => {
                            let mut acc = vec![];
                            for element in value {
                                match element {
                                    Ok(value) => {
                                        acc.push(value);
                                    }
                                    Err(error) => {
                                        return Err(serde::de::Error::custom(error));
                                    }
                                }
                            }
                            Some(acc)
                        }
                        None => None
                    }
                },
                SupportedPredefinedType::StdVecVecStdOptionOptionGeneric(_) => quote::quote!{
                    {
                        let mut acc = vec![];
                        for element in value {
                            match element {
                                Ok(value) => {
                                    acc.push(value);
                                }
                                Err(error) => {
                                    return Err(serde::de::Error::custom(error));
                                }
                            }
                        }
                        acc
                    }
                },
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionGeneric(_) => quote::quote!{
                    match value {
                        Some(value) => {
                            let mut acc = vec![];
                            for element in value {
                                match element {
                                    Ok(value) => {
                                        acc.push(value);
                                    }
                                    Err(error) => {
                                        return Err(serde::de::Error::custom(error));
                                    }
                                }
                            }
                            Some(acc)
                        }
                        None => None
                    }
                }
            };
            quote::quote!{
                #field_ident: match #field_index_token_stream {
                    Some(value) => match value {
                        Ok(value) => Some(crate::value::Value{ value: #conversion_logic_token_stream }),
                        Err(error) => {
                            return Err(serde::de::Error::custom(error));
                        }
                    },
                    None => None
                }
            }
        });
        //its just #(#visit_seq_fields_assignment_token_stream),* reusage making move error 
        let visit_seq_fields_assignment_handle_token_stream = quote::quote!{#(#visit_seq_fields_assignment_token_stream),*};
        let visit_map_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_index_token_stream = {
                let value = format!("__field{index}");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
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

                SupportedPredefinedType::StdVecVecStdPrimitiveI8 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i8,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI16 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i16,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI32 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i32,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI64 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i64,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI128 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i128,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU8 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u8,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU16 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u16,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU32 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u32,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU64 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u64,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU128 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u128,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveF32 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::f32,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveF64 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::f64,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveBool => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdStringString => quote::quote!{std::vec::Vec<std::result::Result<std::string::String,std::string::String>>},

                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i8,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i16,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i32,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i64,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i128,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u8,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u16,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u32,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u64,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u128,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::f32,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::f64,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveBool => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdStringString => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::string::String,std::string::String>>>},

                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI8 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI16 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i16>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i32>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i64>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI128 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i128>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU8 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u8>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU16 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u16>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u32>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u64>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU128 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u128>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f32>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f64>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveBool => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::bool>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdStringString => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::string::String>,std::string::String>>},

                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i16>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i32>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i64>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i128>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u8>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u16>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u32>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u64>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u128>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f32>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f64>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::bool>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdStringString => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::string::String>,std::string::String>>},

                SupportedPredefinedType::Generic(type_path) => generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string()),
                SupportedPredefinedType::StdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<#generic_ident_options_upper_camel_case_token_stream>}
                }
                SupportedPredefinedType::StdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::vec::Vec<std::result::Result<#generic_ident_options_upper_camel_case_token_stream,std::string::String>>}
                }
                SupportedPredefinedType::StdOptionOptionStdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<#generic_ident_options_upper_camel_case_token_stream,std::string::String>>>}
                }
                SupportedPredefinedType::StdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>,std::string::String>>}
                }
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>,std::string::String>>>}
                }
            };
            quote::quote!{
                let mut #field_index_token_stream: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<#type_token_stream, std::string::String>,
                    >,
                > = serde::__private::None;
            }
        });
        let visit_map_match_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_index_token_stream = {
                let value = format!("__field{index}");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let field_ident_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
                &element.ident.as_ref().unwrap_or_else(|| {
                    panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                        naming_conventions::FIELD_IDENT_IS_NONE
                    );
                }).to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
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

                SupportedPredefinedType::StdVecVecStdPrimitiveI8 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i8,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI16 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i16,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI32 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i32,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI64 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i64,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveI128 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::i128,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU8 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u8,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU16 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u16,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU32 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u32,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU64 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u64,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveU128 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::u128,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveF32 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::f32,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveF64 => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::f64,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdPrimitiveBool => quote::quote!{std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdStringString => quote::quote!{std::vec::Vec<std::result::Result<std::string::String,std::string::String>>},

                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i8,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i16,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i32,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i64,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveI128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i128,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u8,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u16,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u32,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u64,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveU128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::u128,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::f32,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::f64,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveBool => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdStringString => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::string::String,std::string::String>>>},

                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI8 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI16 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i16>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i32>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i64>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI128 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i128>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU8 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u8>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU16 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u16>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u32>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u64>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU128 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u128>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f32>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f64>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveBool => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::bool>,std::string::String>>},
                SupportedPredefinedType::StdVecVecStdOptionOptionStdStringString => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::string::String>,std::string::String>>},

                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i16>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i32>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i64>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i128>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u8>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u16>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u32>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u64>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u128>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f32>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f64>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::bool>,std::string::String>>},
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdStringString => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::string::String>,std::string::String>>},

                SupportedPredefinedType::Generic(type_path) => generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string()),
                SupportedPredefinedType::StdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<#generic_ident_options_upper_camel_case_token_stream>}
                }
                SupportedPredefinedType::StdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::vec::Vec<std::result::Result<#generic_ident_options_upper_camel_case_token_stream,std::string::String>>}
                }
                SupportedPredefinedType::StdOptionOptionStdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<#generic_ident_options_upper_camel_case_token_stream,std::string::String>>>}
                }
                SupportedPredefinedType::StdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>,std::string::String>>}
                }
                SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>,std::string::String>>>}
                }
            };
            quote::quote!{
                __Field::#field_index_token_stream => {
                    if serde::__private::Option::is_some(&#field_index_token_stream) {
                        return serde::__private::Err(
                            <__A::Error as serde::de::Error>::duplicate_field(
                                #field_ident_double_quotes_token_stream,
                            ),
                        );
                    }
                    #field_index_token_stream = serde::__private::Some(
                        serde::de::MapAccess::next_value::<
                            std::option::Option<
                                std::result::Result<
                                    #type_token_stream,
                                    std::string::String,
                                >,
                            >,
                        >(&mut __map)?,
                    );
                }
            }
        });
        let visit_map_missing_fields_check_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_index_token_stream = {
                let value = format!("__field{index}");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let field_ident_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
                &element.ident.as_ref().unwrap_or_else(|| {
                    panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                        naming_conventions::FIELD_IDENT_IS_NONE
                    );
                }).to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            quote::quote!{
                let #field_index_token_stream = match #field_index_token_stream {
                    serde::__private::Some(#field_index_token_stream) => #field_index_token_stream,
                    serde::__private::None => {
                        serde::__private::de::missing_field(#field_ident_double_quotes_token_stream)?
                    }
                };
            }
        });
        let fields_array_elements_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            proc_macro_common::generate_quotes::double_quotes_token_stream(
                &element.ident.as_ref().unwrap_or_else(|| {
                    panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                        naming_conventions::FIELD_IDENT_IS_NONE
                    );
                }).to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        });
        quote::quote!{
            impl<'de> serde::Deserialize<'de> for #ident_options_upper_camel_case_token_stream {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> serde::__private::Result<Self, __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        #(#field_enum_variants_token_stream),*,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl serde::de::Visitor<'_> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut serde::__private::Formatter<'_>,
                        ) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> serde::__private::Result<Self::Value, __E>
                        where
                            __E: serde::de::Error,
                        {
                            match __value {
                                #(#visit_u64_value_enum_variants_token_stream),*,
                                _ => serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> serde::__private::Result<Self::Value, __E>
                        where
                            __E: serde::de::Error,
                        {
                            match __value {
                                #(#visit_str_value_enum_variants_token_stream),*,
                                _ => serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> serde::__private::Result<Self::Value, __E>
                        where
                            __E: serde::de::Error,
                        {
                            match __value {
                                #(#visit_bytes_value_enum_variants_token_stream),*,
                                _ => serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: serde::__private::PhantomData<#ident_options_upper_camel_case_token_stream>,
                        lifetime: serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #ident_options_upper_camel_case_token_stream;
                        fn expecting(
                            &self,
                            __formatter: &mut serde::__private::Formatter<'_>,
                        ) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(
                                __formatter,
                                #struct_ident_options_double_quotes_token_stream,
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: serde::de::SeqAccess<'de>,
                        {
                            #(#visit_seq_fields_initialization_token_stream)*
                            serde::__private::Ok(#ident_options_upper_camel_case_token_stream {
                                #visit_seq_fields_assignment_handle_token_stream
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: serde::de::MapAccess<'de>,
                        {
                            #(#visit_map_fields_initialization_token_stream)*
                            while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    #(#visit_map_match_variants_token_stream),*
                                    _ => {
                                        let _ = serde::de::MapAccess::next_value::<
                                            serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            #(#visit_map_missing_fields_check_token_stream)*
                            serde::__private::Ok(#ident_options_upper_camel_case_token_stream {
                                #visit_seq_fields_assignment_handle_token_stream
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        #(#fields_array_elements_token_stream),*
                    ];
                    serde::Deserializer::deserialize_struct(
                        __deserializer,
                        #ident_options_double_quotes_token_stream,
                        FIELDS,
                        __Visitor {
                            marker: serde::__private::PhantomData::<#ident_options_upper_camel_case_token_stream>,
                            lifetime: serde::__private::PhantomData,
                        },
                    )
                }
            }
        }
    };
    let ident_wrapper_token_stream = {
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait //, serde::Deserialize
            pub struct #ident_wrapper_upper_camel_case_token_stream(pub #ident_options_upper_camel_case_token_stream);//pub Result<SomethingOptions,std::string::String>
        }
    };
    let impl_serde_deserialize_for_ident_wrapper_token_stream = {
        let tuple_struct_ident_wrapper_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!("tuple struct {ident_wrapper_upper_camel_case_stringified}"),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let tuple_struct_ident_wrapper_with_1_element_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!("tuple struct {ident_wrapper_upper_camel_case_stringified} with 1 element"),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let ident_wrapper_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
            &ident_wrapper_upper_camel_case_stringified,
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        quote::quote!{
            impl<'de> serde::Deserialize<'de> for #ident_wrapper_upper_camel_case_token_stream {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> serde::__private::Result<Self, __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                {
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: serde::__private::PhantomData<#ident_wrapper_upper_camel_case_token_stream>,
                        lifetime: serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #ident_wrapper_upper_camel_case_token_stream;
                        fn expecting(
                            &self,
                            __formatter: &mut serde::__private::Formatter<'_>,
                        ) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(
                                __formatter,
                                #tuple_struct_ident_wrapper_double_quotes_token_stream,
                            )
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(
                            self,
                            __e: __E,
                        ) -> serde::__private::Result<Self::Value, __E::Error>
                        where
                            __E: serde::Deserializer<'de>,
                        {
                            let __field0: Result<#ident_options_upper_camel_case_token_stream, std::string::String> = <Result<
                                #ident_options_upper_camel_case_token_stream,
                                std::string::String,
                            > as serde::Deserialize>::deserialize(__e)?;
                            serde::__private::Ok(#ident_wrapper_upper_camel_case_token_stream(match __field0 {
                                Ok(value) => value,
                                Err(error) => {
                                    return Err(serde::de::Error::custom(error));
                                }
                            }))
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match serde::de::SeqAccess::next_element::<
                                Result<#ident_options_upper_camel_case_token_stream, std::string::String>,
                            >(&mut __seq)? {
                                serde::__private::Some(__value) => __value,
                                serde::__private::None => {
                                    return serde::__private::Err(
                                        serde::de::Error::invalid_length(
                                            0usize,
                                            &#tuple_struct_ident_wrapper_with_1_element_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                            serde::__private::Ok(#ident_wrapper_upper_camel_case_token_stream(match __field0 {
                                Ok(value) => value,
                                Err(error) => {
                                    return Err(serde::de::Error::custom(error));
                                }
                            }))
                        }
                    }
                    serde::Deserializer::deserialize_newtype_struct(
                        __deserializer,
                        #ident_wrapper_double_quotes_token_stream,
                        __Visitor {
                            marker: serde::__private::PhantomData::<#ident_wrapper_upper_camel_case_token_stream>,
                            lifetime: serde::__private::PhantomData,
                        },
                    )
                }
            }
        }
    };
    let impl_generate_postgresql_query_part_for_ident_field_token_stream = {
        let generate_postgresql_query_part_match_variants_token_steram = vec_syn_field.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
               panic!(
                   "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                   naming_conventions::FIELD_IDENT_IS_NONE
               );
            });
            let el_ident_str = element_ident.to_string();
            let element_ident_upper_camel_case_token_stream = {
                let value = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&el_ident_str);
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let query_part_logic_token_stream = {
                let generate_simple_json_type = |json_type: JsonType|{
                    let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!(
"'{el_ident_str}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = '{json_type}' then jsonb_build_object('Ok',{{column_name_and_maybe_field_getter}}->'{el_ident_str}') else jsonb_build_object('Err','todo this must be error message') end "
                        ),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        Ok(format!(#query_string_token_stream))
                    }
                };
                let generate_optional_simple_json_type = |json_type: JsonType|{
                    let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!(
"'{el_ident_str}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = '{json_type}' then jsonb_build_object('Ok',{{column_name_and_maybe_field_getter}}->'{el_ident_str}') when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object('Err','todo this must be error message') end "
                        ),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        Ok(format!(#query_string_token_stream))
                    }
                };
                let generate_vec_simple_json_type = |json_type: JsonType|{
                    let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!(
"'{el_ident_str}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = 'array' then jsonb_build_object('Ok',(select jsonb_agg(case when jsonb_typeof(value) = '{json_type}' then jsonb_build_object('Ok', value) else jsonb_build_object('Err','todo error message') end) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{el_ident_str}')) with ordinality where ordinality between {{start}} and {{end}})) else jsonb_build_object('Err','todo this must be error message') end"
                        ),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        {
                            let start = offset;
                            let end = match offset.checked_add(*limit) {
                                Some(value) => value,
                                None => {
                                    return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::OffsetPlusLimitIsIntOverflow {
                                        limit: *limit,
                                        offset: *offset,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            };
                            Ok(format!(#query_string_token_stream))
                        }
                    }
                };
                let generate_optional_vec_simple_json_type = |json_type: JsonType|{
                    let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!(
"'{el_ident_str}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = 'array' then jsonb_build_object('Ok',(select jsonb_agg(case when jsonb_typeof(value) = '{json_type}' then jsonb_build_object('Ok', value) else jsonb_build_object('Err','todo error message') end) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{el_ident_str}')) with ordinality where ordinality between {{start}} and {{end}})) when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object('Err','todo this must be error message') end"
                        ),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        {
                            let start = offset;
                            let end = match offset.checked_add(*limit) {
                                Some(value) => value,
                                None => {
                                    return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::OffsetPlusLimitIsIntOverflow {
                                        limit: *limit,
                                        offset: *offset,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            };
                            Ok(format!(#query_string_token_stream))
                        }
                    }
                };
                let type_token_stream = match SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) 
                {
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
                    SupportedPredefinedType::StdPrimitiveF64 => {
                        let query_part_token_stream = generate_simple_json_type(JsonType::Number);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream => #query_part_token_stream
                        }
                    },
                    SupportedPredefinedType::StdPrimitiveBool => {
                        let query_part_token_stream = generate_simple_json_type(JsonType::Boolean);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream => #query_part_token_stream
                        }
                    },
                    SupportedPredefinedType::StdStringString => {
                        let query_part_token_stream = generate_simple_json_type(JsonType::String);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream => #query_part_token_stream
                        }
                    },

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
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveF64 => {
                        let query_part_token_stream = generate_optional_simple_json_type(JsonType::Number);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream => #query_part_token_stream
                        }
                    },
                    SupportedPredefinedType::StdOptionOptionStdPrimitiveBool => {
                        let query_part_token_stream = generate_optional_simple_json_type(JsonType::Boolean);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream => #query_part_token_stream
                        }
                    },
                    SupportedPredefinedType::StdOptionOptionStdStringString => {
                        let query_part_token_stream = generate_optional_simple_json_type(JsonType::String);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream => #query_part_token_stream
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
                    SupportedPredefinedType::StdVecVecStdPrimitiveF64 => {
                        let query_part_token_stream = generate_vec_simple_json_type(JsonType::Number);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream {
                                limit,
                                offset
                            } => #query_part_token_stream
                        }
                    },
                    // generate_vec_imple_json_types
                    SupportedPredefinedType::StdVecVecStdPrimitiveBool => {
                        let query_part_token_stream = generate_vec_simple_json_type(JsonType::Boolean);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream {
                                limit,
                                offset
                            } => #query_part_token_stream
                        }
                    },
                    SupportedPredefinedType::StdVecVecStdStringString => {
                        let query_part_token_stream = generate_vec_simple_json_type(JsonType::String);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream {
                                limit,
                                offset
                            } => #query_part_token_stream
                        }
                    },

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
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveF64 => {
                        let query_part_token_stream = generate_optional_vec_simple_json_type(JsonType::Number);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream {
                                limit,
                                offset
                            } => #query_part_token_stream
                        }
                    },
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdPrimitiveBool => {
                        let query_part_token_stream = generate_optional_vec_simple_json_type(JsonType::Boolean);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream {
                                limit,
                                offset
                            } => #query_part_token_stream
                        }
                    },
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdStringString => {
                        let query_part_token_stream = generate_optional_vec_simple_json_type(JsonType::String);
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream {
                                limit,
                                offset
                            } => #query_part_token_stream
                        }
                    },

                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI8 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI16 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i16>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i32>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i64>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveI128 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i128>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU8 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u8>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU16 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u16>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u32>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u64>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveU128 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u128>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF32 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f32>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveF64 => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f64>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdPrimitiveBool => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::primitive::bool>,std::string::String>>},
                    SupportedPredefinedType::StdVecVecStdOptionOptionStdStringString => quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<std::string::String>,std::string::String>>},

                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i16>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i32>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i64>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i128>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u8>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u16>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u32>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u64>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::u128>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f32>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::f64>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::bool>,std::string::String>>},
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionStdStringString => quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::string::String>,std::string::String>>},

                    SupportedPredefinedType::Generic(type_path) => {
                        let first_query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("{{column_name_and_maybe_field_getter}}->'{el_ident_str}'"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let second_query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("'{el_ident_str}',{{value}}"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            //todo add path to GeneratePostgresqlQueryPart trait?
                            Self::#element_ident_upper_camel_case_token_stream(fields_vec) => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                                fields_vec,
                                &format!(#first_query_string_token_stream),
                                false
                            ) {
                                Ok(value) => Ok(format!(#second_query_string_token_stream)),
                                Err(error) => {
                                    return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::#ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream  {
                                        field: error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            },
                        }
                    },
                    SupportedPredefinedType::StdOptionOptionGeneric(type_path) => {
                        let first_query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("{{column_name_and_maybe_field_getter}}->'{el_ident_str}'"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let second_query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("'{el_ident_str}',{{value}}"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream(fields_vec) => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                                fields_vec,
                                &format!(#first_query_string_token_stream),
                                true
                            ) {
                                Ok(value) => Ok(format!(#second_query_string_token_stream)),
                                Err(error) => {
                                    return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::#ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream {
                                        field: error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            },
                        }
                    }
                    SupportedPredefinedType::StdVecVecGeneric(type_path) => {
                        let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!(
"'{el_ident_str}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = 'array' then jsonb_build_object('Ok',(select jsonb_agg({{value}}) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{el_ident_str}')) with ordinality where ordinality between {{start}} and {{end}})) else jsonb_build_object('Err','todo error message') end"
                            ),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream {
                                field_vec,
                                limit,
                                offset
                            } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                                field_vec,
                                &format!("value"),
                                false
                            ) {
                                Ok(value) => {
                                    let start = offset;
                                    let end = match offset.checked_add(*limit) {
                                        Some(value) => value,
                                        None => {
                                            return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::OffsetPlusLimitIsIntOverflow {
                                                limit: *limit,
                                                offset: *offset,
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                    };
                                    Ok(format!(#query_string_token_stream))
                                },
                                Err(error) => {
                                    return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::#ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream {
                                        field: error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            },
                        }
                    }
                    SupportedPredefinedType::StdOptionOptionStdVecVecGeneric(type_path) => {
                        let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!(
"'{el_ident_str}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = 'array' then jsonb_build_object('Ok',(select jsonb_agg({{value}}) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{el_ident_str}')) with ordinality where ordinality between {{start}} and {{end}})) when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object('Err','todo error message') end"
                            ),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream {
                                field_vec,
                                limit,
                                offset
                            } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                                field_vec,
                                &format!("value"),
                                false
                            ) {
                                Ok(value) => {
                                    let start = offset;
                                    let end = match offset.checked_add(*limit) {
                                        Some(value) => value,
                                        None => {
                                            return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::OffsetPlusLimitIsIntOverflow {
                                                limit: *limit,
                                                offset: *offset,
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                    };
                                    Ok(format!(#query_string_token_stream))
                                },
                                Err(error) => {
                                    return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::#ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream {
                                        field: error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                    }
                    SupportedPredefinedType::StdVecVecStdOptionOptionGeneric(type_path) => {
                        let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!(
"'{el_ident_str}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = 'array' then jsonb_build_object('Ok',(select jsonb_agg({{value}}) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{el_ident_str}')) with ordinality where ordinality between {{start}} and {{end}})) else jsonb_build_object('Err','todo error message') end"
                            ),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream {
                                field_vec,
                                limit,
                                offset
                            } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                                field_vec,
                                &format!("value"),
                                true
                            ) {
                                Ok(value) => {
                                    let start = offset;
                                    let end = match offset.checked_add(*limit) {
                                        Some(value) => value,
                                        None => {
                                            return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::OffsetPlusLimitIsIntOverflow {
                                                limit: *limit,
                                                offset: *offset,
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                    };
                                    Ok(format!(#query_string_token_stream))
                                },
                                Err(error) => {
                                    return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::#ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream {
                                        field: error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                    }
                    SupportedPredefinedType::StdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                        let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!(
"'{el_ident_str}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = 'array' then jsonb_build_object('Ok',(select jsonb_agg({{value}}) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{el_ident_str}')) with ordinality where ordinality between {{start}} and {{end}})) when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{el_ident_str}') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object('Err','todo error message') end"
                            ),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        );
                        let ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                        quote::quote!{
                            Self::#element_ident_upper_camel_case_token_stream {
                                field_vec,
                                limit,
                                offset
                            } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                                field_vec,
                                &format!("value"),
                                true
                            ) {
                                Ok(value) => {
                                    let start = offset;
                                    let end = match offset.checked_add(*limit) {
                                        Some(value) => value,
                                        None => {
                                            return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::OffsetPlusLimitIsIntOverflow {
                                                limit: *limit,
                                                offset: *offset,
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                    };
                                    Ok(format!(#query_string_token_stream))
                                },
                                Err(error) => {
                                    return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::#ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream {
                                        field: error,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            },
                        }
                    }
                };
                //
                quote::quote!{

                }
            };
            quote::quote!{
                
            }
        });
        quote::quote!{
            impl GeneratePostgresqlQueryPart<#ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream, #ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream> for SomethingField {
                fn generate_postgresql_query_part_from_self_vec(
                    value: &std::vec::Vec<Self>,
                    column_name_and_maybe_field_getter: &std::primitive::str,
                    is_optional: std::primitive::bool,
                ) -> Result<std::string::String, #ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream> {
                    if value.is_empty() {
                        return Err(#ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream::FieldsFilterIsEmpty {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                    let mut unique = vec![];
                    for element in value {
                        if unique.contains(&element) {
                            return Err(#ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream::NotUniqueFieldFilter {
                                field: element.clone(),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        else {
                            unique.push(&element);
                        }
                    }
                    let mut acc = std::string::String::default();
                    for element in value {
                        match element.generate_postgresql_query_part(column_name_and_maybe_field_getter) {
                            Ok(value) => {
                                acc.push_str(&format!("{value},"));
                            }
                            Err(error) => {
                                return Err(#ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream::GeneratePostgresqlQueryPart {
                                    error,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        }
                    }
                    let _ = acc.pop();
                    let is_optional_query_part = match is_optional {
                        true => format!(r#"
                            when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then
                                jsonb_build_object(
                                    'Ok',
                                    null
                                )
                        "#),
                        false => std::string::String::default()
                    };
                    Ok(format!(r#"
                        case 
                            when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then 
                                jsonb_build_object(
                                    'Ok',
                                    jsonb_build_object({acc})
                                )
                            {is_optional_query_part}
                            else 
                                jsonb_build_object(
                                    'Err',
                                    'todo error message'
                                ) 
                        end
                    "#))
                }
                fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, #ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream> {
                    match self {
                        Self::StdStringString => Ok(format!(r#"
                            'std_string_string',
                            case 
                                when jsonb_typeof({column_name_and_maybe_field_getter}->'std_string_string') = 'string' then
                                    jsonb_build_object(
                                        'Ok',
                                        {column_name_and_maybe_field_getter}->'std_string_string'
                                    )
                                else 
                                    jsonb_build_object('Err','todo this must be error message')
                            end 
                        "#)),
                        Self::StdVecVecStdPrimitiveBool {
                            limit,
                            offset
                        } => {
                            let start = offset;
                            let end = match offset.checked_add(*limit) {
                                Some(value) => value,
                                None => {
                                    return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::OffsetPlusLimitIsIntOverflow {
                                        limit: *limit,
                                        offset: *offset,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            };
                            Ok(format!(r#"
                                'std_vec_vec_std_primitive_bool',
                                case 
                                    when jsonb_typeof({column_name_and_maybe_field_getter}->'std_vec_vec_std_primitive_bool') = 'array' then 
                                        jsonb_build_object(
                                            'Ok',
                                            (
                                                select jsonb_agg(
                                                    case 
                                                        when jsonb_typeof(value) = 'boolean' then 
                                                            jsonb_build_object(
                                                              'Ok', 
                                                              value
                                                            ) 
                                                      else 
                                                        jsonb_build_object(
                                                            'Err', 
                                                            'todo error message'
                                                        ) 
                                                    end
                                                ) 
                                                from jsonb_array_elements(
                                                    (select {column_name_and_maybe_field_getter}->'std_vec_vec_std_primitive_bool')
                                                )
                                                with ordinality 
                                                where ordinality between {start} and {end}
                                            )
                                        )
                                    else 
                                        jsonb_build_object(
                                            'Err', 
                                            'todo this must be error message'
                                        ) 
                                end
                            "#))
                        },
                        Self::Generic(fields_vec) => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                            fields_vec,
                            &format!("{column_name_and_maybe_field_getter}->'generic'"),
                            false
                        ) {
                            Ok(value) => Ok(format!("'generic',{value}")),
                            Err(error) => {
                                return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                                    field: error,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        },
                        Self::StdOptionOptionGeneric(fields_vec) => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                            fields_vec,
                            &format!("{column_name_and_maybe_field_getter}->'std_option_option_generic'"),
                            true
                        ) {
                            Ok(value) => Ok(format!("'std_option_option_generic',{value}")),
                            Err(error) => {
                                return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                                    field: error,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        },
                        Self::StdVecVecGeneric {
                            field_vec,
                            limit,
                            offset
                        } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                            field_vec,
                            &format!("value"),
                            false
                        ) {
                            Ok(value) => {
                                let start = offset;
                                let end = match offset.checked_add(*limit) {
                                    Some(value) => value,
                                    None => {
                                        return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::OffsetPlusLimitIsIntOverflow {
                                            limit: *limit,
                                            offset: *offset,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                };
                                Ok(format!(r#"
                                    'std_vec_vec_generic',
                                    case 
                                        when jsonb_typeof({column_name_and_maybe_field_getter}->'std_vec_vec_generic') = 'array' then
                                            jsonb_build_object(
                                                'Ok',
                                                (
                                                    select jsonb_agg({value}) 
                                                    from jsonb_array_elements(
                                                        (select {column_name_and_maybe_field_getter}->'std_vec_vec_generic')
                                                    ) 
                                                    with ordinality 
                                                    where ordinality between {start} and {end}
                                                )
                                            )
                                        else 
                                            jsonb_build_object(
                                                'Err',
                                                'todo error message'
                                            )
                                    end
                                "#))
                            },
                            Err(error) => {
                                return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                                    field: error,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        },
                        Self::StdOptionOptionStdVecVecGeneric {
                            field_vec,
                            limit,
                            offset
                        } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                            field_vec,
                            &format!("value"),
                            false
                        ) {
                            Ok(value) => {
                                let start = offset;
                                let end = match offset.checked_add(*limit) {
                                    Some(value) => value,
                                    None => {
                                        return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::OffsetPlusLimitIsIntOverflow {
                                            limit: *limit,
                                            offset: *offset,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                };
                                Ok(format!(r#"
                                    'std_option_option_std_vec_vec_generic',
                                    case 
                                        when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic') = 'array' then
                                            jsonb_build_object(
                                                'Ok',
                                                (
                                                    select jsonb_agg({value}) 
                                                    from jsonb_array_elements(
                                                        (select {column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic')
                                                    ) 
                                                    with ordinality 
                                                    where ordinality between {start} and {end}
                                                )
                                            )
                                        when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic') = 'null' then
                                            jsonb_build_object(
                                                'Ok',
                                                null
                                            )
                                        else 
                                            jsonb_build_object(
                                                'Err',
                                                'todo error message'
                                            )
                                    end
                                "#))
                            },
                            Err(error) => {
                                return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                                    field: error,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        },
                        Self::StdVecVecStdOptionOptionGeneric {
                            field_vec,
                            limit,
                            offset
                        } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                            field_vec,
                            &format!("value"),
                            true
                        ) {
                            Ok(value) => {
                                let start = offset;
                                let end = match offset.checked_add(*limit) {
                                    Some(value) => value,
                                    None => {
                                        return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::OffsetPlusLimitIsIntOverflow {
                                            limit: *limit,
                                            offset: *offset,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                };
                                Ok(format!(r#"
                                    'std_vec_vec_std_option_option_generic',
                                    case 
                                    	when jsonb_typeof({column_name_and_maybe_field_getter}->'std_vec_vec_std_option_option_generic') = 'array' then 
                                    		jsonb_build_object(
                                    			'Ok',
                                    			(
                                    				select jsonb_agg({value}) 
                                    				from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_vec_vec_std_option_option_generic')) 
                                    				with ordinality 
                                    				where ordinality between {start} and {end}
                                    			)
                                    		)
                                    	else 
                                    		jsonb_build_object(
                                    			'Err',
                                    			'todo error message'
                                    		) 
                                    end
                                "#))
                            },
                            Err(error) => {
                                return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                                    field: error,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        },
                        Self::StdOptionOptionStdVecVecStdOptionOptionGeneric {
                            field_vec,
                            limit,
                            offset
                        } => match GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                            field_vec,
                            &format!("value"),
                            true
                        ) {
                            Ok(value) => {
                                let start = offset;
                                let end = match offset.checked_add(*limit) {
                                    Some(value) => value,
                                    None => {
                                        return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::OffsetPlusLimitIsIntOverflow {
                                            limit: *limit,
                                            offset: *offset,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                };
                                Ok(format!(r#"
                                    'std_option_option_std_vec_vec_std_option_option_generic',
                                    case 
                                    	when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_option_option_generic') = 'array' then 
                                    		jsonb_build_object(
                                    			'Ok',
                                    			(
                                    				select jsonb_agg({value}) 
                                    				from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_option_option_generic')) 
                                    				with ordinality 
                                    				where ordinality between {start} and {end}
                                    			)
                                    		)
                                        when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_option_option_generic') = 'null' then
                                        	jsonb_build_object(
                                        		'Ok',
                                        		null
                                        	)
                                    	else 
                                    		jsonb_build_object(
                                    			'Err',
                                    			'todo error message'
                                    		) 
                                    end
                                "#))
                            },
                            Err(error) => {
                                return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::DoggieGeneratePostgresqlQueryPartFromSelfVec {
                                    field: error,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        },
                    }
                }
            }
        }
    };
    let impl_generate_postgresql_query_part_for_ident_field_token_stream = if ident.to_string() == "Something" {
        impl_generate_postgresql_query_part_for_ident_field_token_stream   
    }
    else {
        proc_macro2::TokenStream::new()
    };
    let generated = quote::quote!{
        #impl_std_fmt_display_for_ident_token_stream
        #pub_enum_ident_field_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_field_token_stream
        // #pub_enum_field_generate_postgresql_query_part_error_named_token_stream
        #impl_generate_postgresql_query_part_field_generate_postgresql_query_part_error_named_for_ident_field_token_stream
        #pub_struct_ident_options_token_stream
        #impl_std_convert_from_ident_for_ident_options_token_stream
        #impl_serde_deserialize_for_ident_options_token_stream
        #ident_wrapper_token_stream
        #impl_serde_deserialize_for_ident_wrapper_token_stream
        #impl_generate_postgresql_query_part_for_ident_field_token_stream
    };
    generated.into()
}