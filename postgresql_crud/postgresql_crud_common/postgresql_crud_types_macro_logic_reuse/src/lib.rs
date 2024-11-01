#[proc_macro_derive(CommonWithEqImpl)] //todo check on postgresql max length value of type
pub fn common_with_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    common_handle(input, "CommonWithEqImpl", true, true, true)
}
#[proc_macro_derive(CommonWithoutEqImpl)] //todo check on postgresql max length value of type
pub fn common_without_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    common_handle(input, "CommonWithoutEqImpl", false, false, false)
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
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    // println!("{:#?}", syn_derive_input.data);
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            match fields_unnamed.unnamed.len() {
                1 => &fields_unnamed.unnamed[0],
                _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Unnamed with one field"),
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
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!("{}{ident}", naming_conventions::StdOptionOptionUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!("{}{}{ident}", naming_conventions::WhereUpperCamelCase, naming_conventions::StdOptionOptionUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    //
    let where_ident_should_implement_eq_token_stream = if where_ident_should_implement_eq {
        quote::quote! {Eq,}
    } else {
        proc_macro2::TokenStream::new()
    };
    let std_option_option_ident_upper_camel_case_should_implement_eq_token_stream = if std_option_option_ident_upper_camel_case_should_implement_eq {
        quote::quote! {Eq,}
    } else {
        proc_macro2::TokenStream::new()
    };
    let where_std_option_option_ident_upper_camel_case_should_implement_eq_token_stream = if where_std_option_option_ident_upper_camel_case_should_implement_eq {
        quote::quote! {Eq,}
    } else {
        proc_macro2::TokenStream::new()
    };
    let try_generate_bind_increments_error_named_upper_camel_case = naming_conventions::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let checked_add_upper_camel_case = naming_conventions::CheckedAddUpperCamelCase;
    let generated = quote::quote! {
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
        impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #where_ident_token_stream {
            fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                Self {
                    value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                    conjuctive_operator: ConjunctiveOperator::default(),
                }
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
        impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #std_option_option_ident_upper_camel_case_token_stream {
            fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                Self(Some(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()))
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
        impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #where_std_option_option_ident_upper_camel_case_token_stream {
            fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                Self {
                    value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                    conjuctive_operator: ConjunctiveOperator::default(),
                }
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
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    // println!("{:#?}", syn_derive_input.data);
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let _field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            match fields_unnamed.unnamed.len() {
                1 => &fields_unnamed.unnamed[0],
                _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Unnamed with one field"),
            }
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Unnamed");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let generated = quote::quote! {
        impl CheckSupportedRustAndPostgresqlColumnType for #ident {
            fn check_supported_rust_and_postgresql_column_type() {}
        }
    };
    generated.into()
}

#[derive(strum_macros::Display)]
enum StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant {
    FullTypePath,
    StdOptionOptionFullTypePath,
    StdVecVecFullTypePath,
    StdOptionOptionStdVecVecFullTypePath,
    StdVecVecStdOptionOptionFullTypePath,
    StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
}
impl StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(&self) -> proc_macro2::TokenStream {
        match &self {
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath => quote::quote! {
                ::core::default::Default::default()
            },
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath => quote::quote! {
                Some(::core::default::Default::default())
            },
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath => quote::quote! {
                vec![::core::default::Default::default()]
            },
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath => quote::quote! {
                Some(vec![::core::default::Default::default()])
            },
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath => quote::quote! {
                vec![Some(::core::default::Default::default())]
            },
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote! {
                Some(vec![Some(::core::default::Default::default())])
            },
        }
    }
}
fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_handle_token_stream(
    struct_ident_token_stream: &impl quote::ToTokens,
    variant: StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant
) -> proc_macro::TokenStream {
    let inner_content = variant.std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element();
    let generated = quote::quote! {
        impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #struct_ident_token_stream {
            #[inline]
            fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                Self(#inner_content)
            }
        }
    };
    generated.into()
}
fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(
    struct_ident_token_stream: &impl quote::ToTokens,
    variant: StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant
) -> proc_macro::TokenStream {
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_handle_token_stream(
        &struct_ident_token_stream,
        variant
    )
}
fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_to_create_token_stream(
    struct_ident_token_stream: &impl quote::ToTokens,
    variant: StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant
) -> proc_macro::TokenStream {
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_handle_token_stream(
        &struct_ident_token_stream,
        variant
    )
}
//
fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_options_to_read_token_stream(
    struct_ident_token_stream: &impl quote::ToTokens,
    variant: StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant
) -> proc_macro::TokenStream {
    let inner_content = variant.std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element();
    let generated = quote::quote! {
        impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #struct_ident_token_stream {
            #[inline]
            fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                Self(#inner_content)
            }
        }
    };
    generated.into()
}
//
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath)
}

#[proc_macro_derive(GenerateIdentToCreate)]
pub fn generate_generate_ident_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateIdentToCreate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let ident_to_create_upper_camel_case = naming_conventions::SelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_to_create_token_stream = {
        let data_struct = match syn_derive_input.data {
            syn::Data::Struct(value) => value,
            syn::Data::Enum(_) | syn::Data::Union(_) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} only works on Struct"),
        };
        let fields_unnamed = match data_struct.fields {
            syn::Fields::Unnamed(value) => value.unnamed,
            syn::Fields::Named(_) | syn::Fields::Unit => panic!("{proc_macro_name_upper_camel_case_ident_stringified} only works with syn::Fields::Unnamed"),
        };
        assert!(fields_unnamed.len() == 1, "{proc_macro_name_upper_camel_case_ident_stringified} fields_unnamed !== 1");
        let first_field_unnamed = fields_unnamed.iter().next().map_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} fields_unnamed.iter().nth(0) is None"), |value| value);
        let first_field_unnamed_type = &first_field_unnamed.ty;
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]//Eq does not implemented for float//Copy does not implemented for String
            pub struct #ident_to_create_upper_camel_case(pub #first_field_unnamed_type);
        }
    };
    let impl_json_create_bind_query_for_ident_to_create_token_stream = {
        //maybe its not correct to bind array of json. maybe should use bing each element of array instead
        quote::quote! {
            impl<'a> JsonCreateBindQuery<'a> for #ident_to_create_upper_camel_case {
                fn json_create_try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, JsonCreateTryGenerateBindIncrementsErrorNamed> {
                    match increment.checked_add(1) {
                        Some(incr) => {
                            *increment = incr;
                            Ok(format!("${increment}"))
                        }
                        None => Err(JsonCreateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() })
                    }
                }
                fn json_create_bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    query = query.bind(sqlx::types::Json(self.0));
                    query
                }
            }
        }
    };
    let generated = quote::quote!{
        #ident_to_create_token_stream
        #impl_json_create_bind_query_for_ident_to_create_token_stream
    };
    generated.into()
}

#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_full_type_path_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case = naming_conventions::SelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_to_create_token_stream(&ident_to_create_upper_camel_case, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_full_type_path_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case = naming_conventions::SelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_to_create_token_stream(&ident_to_create_upper_camel_case, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_full_type_path_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case = naming_conventions::SelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_to_create_token_stream(&ident_to_create_upper_camel_case, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_full_type_path_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case = naming_conventions::SelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_to_create_token_stream(&ident_to_create_upper_camel_case, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_std_option_option_full_type_path_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case = naming_conventions::SelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_to_create_token_stream(&ident_to_create_upper_camel_case, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_to_create_std_option_option_std_vec_vec_std_option_option_full_type_path_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case = naming_conventions::SelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_to_create_token_stream(&ident_to_create_upper_camel_case, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath)
}

fn generate_pub_struct_ident_field_reader_token_stream(ident: &syn::Ident, content_token_stream: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ident_field_reader_upper_camel_case = naming_conventions::SelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    quote::quote!{
        #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
        pub struct #ident_field_reader_upper_camel_case #content_token_stream
    }
}
fn empty_content_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{{}}
}
fn pagination_content_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{{ pagination: Pagination }}
}

    // let impl_std_convert_from_ident_for_ident_options_to_read_token_stream = {
    //      quote::quote!{
    //         impl std::convert::From<#ident> for #ident_options_to_read_upper_camel_case_token_stream {
    //             fn from(value: #ident) -> Self {
    //                 Self(value.0)
    //             }
    //         }
    //      }
    // };

#[proc_macro_derive(GenerateFullTypePathFieldReader)]
pub fn generate_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let generated = generate_pub_struct_ident_field_reader_token_stream(&ident, &empty_content_token_stream());
    generated.into()
}
#[proc_macro_derive(GenerateStdOptionOptionFullTypePathFieldReader)]
pub fn generate_std_option_option_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdOptionOptionFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let generated = generate_pub_struct_ident_field_reader_token_stream(&ident, &empty_content_token_stream());
    generated.into()
}
#[proc_macro_derive(GenerateStdVecVecFullTypePathFieldReader)]
pub fn generate_std_vec_vec_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdVecVecFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let generated = generate_pub_struct_ident_field_reader_token_stream(&ident, &pagination_content_token_stream());
    generated.into()
}
#[proc_macro_derive(GenerateStdOptionOptionStdVecVecFullTypePathFieldReader)]
pub fn generate_std_option_option_std_vec_vec_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdOptionOptionStdVecVecFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let generated = generate_pub_struct_ident_field_reader_token_stream(&ident, &pagination_content_token_stream());
    generated.into()
}
#[proc_macro_derive(GenerateStdVecVecStdOptionOptionFullTypePathFieldReader)]
pub fn generate_std_vec_vec_std_option_option_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdVecVecStdOptionOptionFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let generated = generate_pub_struct_ident_field_reader_token_stream(&ident, &pagination_content_token_stream());
    generated.into()
}
#[proc_macro_derive(GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader)]
pub fn generate_std_option_option_std_vec_vec_std_option_option_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let generated = generate_pub_struct_ident_field_reader_token_stream(&ident, &pagination_content_token_stream());
    generated.into()
}


fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_field_reader_token_stream(
    ident: &syn::Ident,
    variant: StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant
) -> proc_macro::TokenStream {
    let ident_field_reader_upper_camel_case = naming_conventions::SelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let inner_content = match variant {
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath => quote::quote! {
            ::core::default::Default::default()
        },
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath => quote::quote! {
            ::core::default::Default::default()
        },
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath => quote::quote! {
            Self {
                pagination: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }
        },
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath => quote::quote! {
            Self {
                pagination: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }
        },
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath => quote::quote! {
            Self {
                pagination: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }
        },
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote! {
            Self {
                pagination: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }
        },
    };
    let generated = quote::quote! {
        impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_field_reader_upper_camel_case {
            #[inline]
            fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                #inner_content
            }
        }
    };
    generated.into()
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_field_reader_token_stream(
        &&syn_derive_input.ident,
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath
    )
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_field_reader_token_stream(
        &syn_derive_input.ident,
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath
    )
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_field_reader_token_stream(
        &syn_derive_input.ident,
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath
    )
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_field_reader_token_stream(
        &&syn_derive_input.ident,
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath
    )
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_std_option_option_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_field_reader_token_stream(
        &&syn_derive_input.ident,
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath
    )
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_std_option_option_full_type_path_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_field_reader_token_stream(
        &&syn_derive_input.ident,
        StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath
    )
}


#[proc_macro_derive(GenerateIdentOptionsToRead)]
pub fn generate_ident_options_to_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateIdentOptionsToRead";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let data_struct = match syn_derive_input.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} only works on Struct"),
    };
    let fields_unnamed = match data_struct.fields {
        syn::Fields::Unnamed(value) => value.unnamed,
        syn::Fields::Named(_) | syn::Fields::Unit => panic!("{proc_macro_name_upper_camel_case_ident_stringified} only works with syn::Fields::Unnamed"),
    };
    assert!(fields_unnamed.len() == 1, "{proc_macro_name_upper_camel_case_ident_stringified} fields_unnamed !== 1");
    let first_field_unnamed = fields_unnamed.iter().next().map_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} fields_unnamed.iter().nth(0) is None"), |value| value);
    let first_field_unnamed_type = &first_field_unnamed.ty;
    let ident_options_to_read_upper_camel_case = naming_conventions::SelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let generated = quote::quote!{
        #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]//Eq does not implemented for float//Copy does not implemented for String
        pub struct #ident_options_to_read_upper_camel_case(pub #first_field_unnamed_type);
    };
    generated.into()
}

#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_full_type_path_options_to_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionsToRead";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_options_to_read_upper_camel_case_token_stream = naming_conventions::SelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_options_to_read_token_stream(&ident_options_to_read_upper_camel_case_token_stream, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_full_type_path_options_to_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionsToRead";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_options_to_read_upper_camel_case_token_stream = naming_conventions::SelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_options_to_read_token_stream(&ident_options_to_read_upper_camel_case_token_stream, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_full_type_path_options_to_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionsToRead";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_options_to_read_upper_camel_case = naming_conventions::SelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_options_to_read_token_stream(&ident_options_to_read_upper_camel_case, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_full_type_path_options_to_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionsToRead";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_options_to_read_upper_camel_case_token_stream = naming_conventions::SelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_options_to_read_token_stream(&ident_options_to_read_upper_camel_case_token_stream, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_std_option_option_full_type_path_options_to_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionsToRead";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_options_to_read_upper_camel_case_token_stream = naming_conventions::SelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_options_to_read_token_stream(&ident_options_to_read_upper_camel_case_token_stream, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_std_option_option_full_type_path_options_to_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionsToRead";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_options_to_read_upper_camel_case_token_stream = naming_conventions::SelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_options_to_read_token_stream(&ident_options_to_read_upper_camel_case_token_stream, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath)
}


fn impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
    ident: &syn::Ident,
    content_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let ident_field_reader_upper_camel_case = naming_conventions::SelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    quote::quote!{
        impl GeneratePostgresqlQueryPartFieldToRead for #ident_field_reader_upper_camel_case {
            fn generate_postgresql_query_part_field_to_read(
                &self,
                field_ident: &std::primitive::str,
                column_name_and_maybe_field_getter: &std::primitive::str,
                column_name_and_maybe_field_getter_for_error_message: &std::primitive::str
            ) -> std::string::String {
                #content_token_stream
            }
        }
    }
}
fn generate_primitive_postgresql_part_field_to_read_query_format(format_handle_token_stream: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote::quote!{format!(#format_handle_token_stream)}
}
//todo refactor it to pagination struct with custom Deserialize and try_new check
fn postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(format_handle_token_stream: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pagination_start_end_initialization_token_stream = proc_macro_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream();
    quote::quote! {
        #pagination_start_end_initialization_token_stream
        format!(#format_handle_token_stream)
    }
}


fn generate_primitive_postgresql_part_field_to_read_query(proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str) -> proc_macro2::TokenStream {
    proc_macro_common::generate_quotes::double_quotes_token_stream(
        &format!("jsonb_build_object('{{field_ident}}', jsonb_build_object('value', {{column_name_and_maybe_field_getter}}->'{{field_ident}}'))"),
        &proc_macro_name_upper_camel_case_ident_stringified
    )
}
fn generate_array_primitive_postgresql_part_field_to_read_query(proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str) -> proc_macro2::TokenStream {
    proc_macro_common::generate_quotes::double_quotes_token_stream(
        &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))"),
        &proc_macro_name_upper_camel_case_ident_stringified
    )
}
fn generate_nullable_array_primitive_postgresql_part_field_to_read_query(proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str) -> proc_macro2::TokenStream {
    proc_macro_common::generate_quotes::double_quotes_token_stream(
        &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value', case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 'array' then (select jsonb_agg(value) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}}) else null end))"),
        &proc_macro_name_upper_camel_case_ident_stringified
    )
}
///
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderBoolean)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderString)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_nullable_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableBoolean)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_nullable_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableString)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_nullable_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_array_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayBoolean)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_array_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayString)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_array_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_nullable_array_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_nullable_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayBoolean)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_nullable_array_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_nullable_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayString)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_nullable_array_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_nullable_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_array_nullable_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableBoolean)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_array_nullable_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableString)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_array_nullable_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderArrayNullableString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_nullable_array_nullable_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_nullable_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableBoolean)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_nullable_array_nullable_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_nullable_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableString)]
pub fn generate_impl_generate_postgresql_query_part_field_to_read_for_ident_field_reader_nullable_array_nullable_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartFieldToReadForIdentFieldReaderNullableArrayNullableString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_nullable_array_primitive_postgresql_part_field_to_read_query(&proc_macro_name_upper_camel_case_ident_stringified))
    );
    let generated = quote::quote!{
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}

#[proc_macro_derive(GenerateIdentOptionToUpdate)]
pub fn generate_ident_option_to_update(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateIdentOptionToUpdate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let data_struct = match syn_derive_input.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} only works on Struct"),
    };
    let fields_unnamed = match data_struct.fields {
        syn::Fields::Unnamed(value) => value.unnamed,
        syn::Fields::Named(_) | syn::Fields::Unit => panic!("{proc_macro_name_upper_camel_case_ident_stringified} only works with syn::Fields::Unnamed"),
    };
    assert!(fields_unnamed.len() == 1, "{proc_macro_name_upper_camel_case_ident_stringified} fields_unnamed !== 1");
    let first_field_unnamed = fields_unnamed.iter().next().map_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} fields_unnamed.iter().nth(0) is None"), |value| value);
    let first_field_unnamed_type = &first_field_unnamed.ty;
    let ident_option_to_update_upper_camel_case = naming_conventions::SelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let generated = quote::quote!{
        #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]//Eq does not implemented for float//Copy does not implemented for String
        pub struct #ident_option_to_update_upper_camel_case(pub #first_field_unnamed_type);
    };
    generated.into()
}

fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_option_to_update_token_stream(
    ident: &syn::Ident,
    variant: StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant
) -> proc_macro::TokenStream {
    let ident_option_to_update_upper_camel_case = naming_conventions::SelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let inner_content = variant.std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element();
    let generated = quote::quote! {
        impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_option_to_update_upper_camel_case {
            #[inline]
            fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                Self(#inner_content)
            }
        }
    };
    generated.into()
}

#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_full_type_path_option_to_update(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathOptionToUpdate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_option_to_update_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_full_type_path_option_to_update(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathOptionToUpdate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_option_to_update_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_full_type_path_option_to_update(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathOptionToUpdate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_option_to_update_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_full_type_path_option_to_update(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathOptionToUpdate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_option_to_update_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_std_option_option_full_type_path_option_to_update(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathOptionToUpdate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_option_to_update_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_option_to_update_std_option_option_std_vec_vec_std_option_option_full_type_path_option_to_update(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathOptionToUpdate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_option_to_update_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath)
}

#[proc_macro_derive(GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate)]
pub fn generate_impl_generate_postgresql_query_part_to_update_for_ident_option_to_update(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplGeneratePostgresqlQueryPartToUpdateForIdentOptionToUpdate";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case = naming_conventions::SelfOptionToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_option_to_update_try_generate_bind_increments_error_named_token_stream = {
        quote::quote!{
            #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            pub enum #ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case {
                CheckedAdd {
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
            }
        }
    };
    let impl_postgresql_crud_generate_postgresql_query_part_to_update_ident_option_to_update_try_generate_bind_increments_error_named_for_ident_option_to_update_token_stream = {
        let ident_option_to_update_upper_camel_case = naming_conventions::SelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
        quote::quote!{
            impl GeneratePostgresqlQueryPartToUpdate<#ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case> for #ident_option_to_update_upper_camel_case {
                fn try_generate_bind_increments(
                    &self,
                    jsonb_set_accumulator: &std::primitive::str,
                    jsonb_set_target: &std::primitive::str,
                    jsonb_set_path: &std::primitive::str,
                    increment: &mut std::primitive::u64,
                ) -> Result<std::string::String, #ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case> {
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                        },
                        None => Err(#ident_option_to_update_try_generate_bind_increments_error_named_upper_camel_case::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() })
                    }
                }
                fn bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    query = query.bind(sqlx::types::Json(self.0));
                    query
                }
            }
        }
    };

    let generated = quote::quote!{
        #ident_option_to_update_try_generate_bind_increments_error_named_token_stream
        #impl_postgresql_crud_generate_postgresql_query_part_to_update_ident_option_to_update_try_generate_bind_increments_error_named_for_ident_option_to_update_token_stream
    };
    generated.into()
}

#[proc_macro_derive(GenerateImplPostgresqlJsonType)]
pub fn generate_impl_postgresql_json_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateImplPostgresqlJsonType";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let (
        ident_to_create_token_stream,
        impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_token_stream
    ) = {
        let ident_to_create_upper_camel_case = naming_conventions::SelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
        let ident_to_create_token_stream = {
            let data_struct = match syn_derive_input.data {
                syn::Data::Struct(value) => value,
                syn::Data::Enum(_) | syn::Data::Union(_) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} only works on Struct"),
            };
            let fields_unnamed = match data_struct.fields {
                syn::Fields::Unnamed(value) => value.unnamed,
                syn::Fields::Named(_) | syn::Fields::Unit => panic!("{proc_macro_name_upper_camel_case_ident_stringified} only works with syn::Fields::Unnamed"),
            };
            assert!(fields_unnamed.len() == 1, "{proc_macro_name_upper_camel_case_ident_stringified} fields_unnamed !== 1");
            let first_field_unnamed = fields_unnamed.iter().next().map_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} fields_unnamed.iter().nth(0) is None"), |value| value);
            let first_field_unnamed_type = &first_field_unnamed.ty;
            quote::quote!{
                #[derive(
                    Debug,
                    Clone,
                    PartialEq,
                    Default,
                    serde::Serialize,
                    serde::Deserialize,
                    utoipa::ToSchema,
                    schemars::JsonSchema,
                )]
                pub struct #ident_to_create_upper_camel_case(pub #first_field_unnamed_type);
            }
        };
        let impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_token_stream = {
            quote::quote!{
                impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_to_create_upper_camel_case {
                    #[inline]
                    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        Self(::core::default::Default::default())
                    }
                }
            }
        };
        (
            ident_to_create_token_stream,
            impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_token_stream
        )
    };
    let (
        ident_field_reader_token_stream,
        impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_reader_token_stream
    ) = {
        let ident_field_reader_upper_camel_case = naming_conventions::SelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
        let ident_field_reader_token_stream = {
            let content_token_stream = quote::quote!{{}};
            quote::quote!{
                #[derive(
                    Debug,
                    Clone,
                    PartialEq,
                    Default,
                    serde::Serialize,
                    serde::Deserialize,
                    utoipa::ToSchema,
                    schemars::JsonSchema,
                )]
                pub struct #ident_field_reader_upper_camel_case #content_token_stream
            }
        };
        let impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_reader_token_stream = {
            quote::quote!{
                impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_field_reader_upper_camel_case {
                    #[inline]
                    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        ::core::default::Default::default()
                    }
                }
            }
        };
        (
            ident_field_reader_token_stream,
            impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_reader_token_stream
        )
    };
    let ident_options_to_read_token_stream = {
        quote::quote!{
            #[derive(
                Debug,
                Clone,
                PartialEq,
                Default,
                serde::Serialize,
                serde::Deserialize,
                utoipa::ToSchema,
                schemars::JsonSchema,
            )]
            pub struct JsonStdPrimitiveI8OptionsToRead(pub std::primitive::i8);
        }
    };
    let impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_read_token_stream = {
        quote::quote!{
            impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for JsonStdPrimitiveI8OptionsToRead {
                #[inline]
                fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                    Self(::core::default::Default::default())
                }
            }
        }
    };
    let ident_option_to_update_token_stream = {
        quote::quote!{
            #[derive(
                Debug,
                Clone,
                PartialEq,
                Default,
                serde::Serialize,
                serde::Deserialize,
                utoipa::ToSchema,
                schemars::JsonSchema,
            )]
            pub struct JsonStdPrimitiveI8OptionToUpdate(pub std::primitive::i8);
        }
    };
    let impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_option_to_update_token_stream = {
        quote::quote!{
            impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for JsonStdPrimitiveI8OptionToUpdate {
                #[inline]
                fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                    Self(::core::default::Default::default())
                }
            }
        }
    };
    let ident_option_to_update_try_generate_bind_increments_error_named_token_stream = {
        quote::quote!{
            #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
            pub enum JsonStdPrimitiveI8OptionToUpdateTryGenerateBindIncrementsErrorNamed {
                CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
            }
        }
    };
    let impl_crate_generate_postgresql_query_part_postgresql_json_type_for_ident_token_stream = {
        let to_create_token_stream = {
            quote::quote!{
                type ToCreate<'a> = JsonStdPrimitiveI8ToCreate;
            }
        };
        //todo maybe rename later
        let json_create_try_generate_bind_increments_token_stream = {
            quote::quote!{
                fn json_create_try_generate_bind_increments(
                    self_to_create: &Self::ToCreate<'_>,
                    increment: &mut std::primitive::u64
                ) -> Result<std::string::String, JsonCreateTryGenerateBindIncrementsErrorNamed> {
                    match increment.checked_add(1) {
                        Some(incr) => {
                            *increment = incr;
                            Ok(format!("${increment}"))
                        }
                        None => Err(JsonCreateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                    }
                }
            }
        };
        let json_create_bind_value_to_query_token_stream = {
            quote::quote!{
                fn json_create_bind_value_to_query<'a>(
                    self_to_create: Self::ToCreate<'a>,
                    mut query: sqlx::query::Query<'a,
                    sqlx::Postgres, sqlx::postgres::PgArguments>
                ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    query = query.bind(sqlx::types::Json(self_to_create.0));
                    query
                }
            }
        };
        let field_reader_token_stream = {
            quote::quote!{
                type FieldReader<'a> = JsonStdPrimitiveI8FieldReader;
            }
        };
        let options_to_read_token_stream = {
            quote::quote!{
                type OptionsToRead<'a> = JsonStdPrimitiveI8OptionsToRead;
            }
        };
        //todo maybe rename later
        let generate_postgresql_query_part_field_to_read_token_stream = {
            quote::quote!{
                fn generate_postgresql_query_part_field_to_read(
                    options_to_read: &Self::OptionsToRead<'_>,
                    field_ident: &std::primitive::str,
                    column_name_and_maybe_field_getter: &std::primitive::str,
                    column_name_and_maybe_field_getter_for_error_message: &std::primitive::str
                ) -> std::string::String {
                    format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter}->'{field_ident}'))")
                }
            }
        };
        let option_to_update_token_stream = {
            quote::quote!{
                type OptionToUpdate<'a>: = JsonStdPrimitiveI8OptionToUpdate;
            }
        };
        let option_to_update_try_generate_bind_increments_error_named_token_stream = {
            quote::quote!{
                type OptionToUpdateTryGenerateBindIncrementsErrorNamed = JsonStdPrimitiveI8OptionToUpdateTryGenerateBindIncrementsErrorNamed;
            }
        };
        //todo maybe rename later
        let try_generate_bind_increments_token_stream = {
            quote::quote!{
                fn try_generate_bind_increments(
                    &self,
                    jsonb_set_accumulator: &std::primitive::str,
                    jsonb_set_target: &std::primitive::str,
                    jsonb_set_path: &std::primitive::str,
                    increment: &mut std::primitive::u64,
                ) -> Result<std::string::String, Self::OptionToUpdateTryGenerateBindIncrementsErrorNamed> {
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                        }
                        None => Err(Self::OptionToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                    }
                }
            }
        };
        //todo maybe rename later
        let bind_value_to_query_token_stream = {
            quote::quote!{
                fn bind_value_to_query<'a>(
                    self,
                    mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    query = query.bind(sqlx::types::Json(self.0));
                    query
                }
            }
        };
        quote::quote!{
            impl crate::generate_postgresql_query_part::PostgresqlJsonType for JsonStdPrimitiveI8 {
                #to_create_token_stream
                #json_create_try_generate_bind_increments_token_stream
                #json_create_bind_value_to_query_token_stream
                #field_reader_token_stream
                #options_to_read_token_stream
                #generate_postgresql_query_part_field_to_read_token_stream
                #option_to_update_token_stream
                #option_to_update_try_generate_bind_increments_error_named_token_stream
                #try_generate_bind_increments_token_stream
                #bind_value_to_query_token_stream

            }
        }
    };
    let generated = quote::quote!{
        #ident_to_create_token_stream
        #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_token_stream
        #ident_field_reader_token_stream
        #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_reader_token_stream
        // #ident_options_to_read_token_stream
        // #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_options_to_read_token_stream
        // #ident_option_to_update_token_stream
        // #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_option_to_update_token_stream
        // #ident_option_to_update_try_generate_bind_increments_error_named_token_stream
        // #impl_crate_generate_postgresql_query_part_postgresql_json_type_for_ident_token_stream
    };
    // println!("{generated}");
    generated.into()
}