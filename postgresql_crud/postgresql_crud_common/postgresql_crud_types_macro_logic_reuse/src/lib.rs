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
fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(
    struct_ident_token_stream: &impl quote::ToTokens,
    variant: StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant
) -> proc_macro::TokenStream {
    let inner_content = match variant {
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
    };
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
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath");
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePath");
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePath");
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePath");
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePath");
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePath");
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
    let ident_to_create_token_stream = {
        let ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&ident);
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
            pub struct #ident_to_create_upper_camel_case_token_stream(pub #first_field_unnamed_type);
        }
    };
    let impl_json_create_bind_query_for_ident_to_create_token_stream = {
        let ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&ident);
        //maybe its not correct to bind array of json. maybe should use bing each element of array instead
        quote::quote! {
            impl<'a> JsonCreateBindQuery<'a> for #ident_to_create_upper_camel_case_token_stream {
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
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePathToCreate");
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident_to_create_upper_camel_case_token_stream, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_full_type_path_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionFullTypePathToCreate");
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident_to_create_upper_camel_case_token_stream, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_full_type_path_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecFullTypePathToCreate");
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident_to_create_upper_camel_case_token_stream, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_full_type_path_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecFullTypePathToCreate");
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident_to_create_upper_camel_case_token_stream, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_vec_vec_std_option_option_full_type_path_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdVecVecStdOptionOptionFullTypePathToCreate");
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident_to_create_upper_camel_case_token_stream, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_option_option_std_vec_vec_std_option_option_full_type_path_to_create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = format!("GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementStdOptionOptionStdVecVecStdOptionOptionFullTypePathToCreate");
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_to_create_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfToCreateUpperCamelCaseTokenStream::impl_quote_to_tokens_self_to_create_upper_camel_case_token_stream(&ident);
    generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream(&ident_to_create_upper_camel_case_token_stream, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath)
}

fn generate_pub_struct_ident_field_reader_token_stream(ident: &syn::Ident, content_token_stream: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ident_field_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfFieldReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_self_field_reader_upper_camel_case_token_stream(&ident);
    quote::quote!{
        #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
        pub struct #ident_field_reader_upper_camel_case_token_stream #content_token_stream
    }
}
fn empty_content_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{{}}
}
fn pagination_content_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{{ pagination: Pagination }}
}

#[proc_macro_derive(GenerateJsonPostgresqlPrimitiveFieldReader)]
pub fn generate_json_postgresql_primitive_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateJsonPostgresqlPrimitiveFieldReader";
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
    let pub_struct_ident_field_reader_token_stream = generate_pub_struct_ident_field_reader_token_stream(&ident, &empty_content_token_stream());

    let ident_options_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_options_to_read_upper_camel_case_token_stream(&ident);
    let ident_options_to_read_token_stream = {
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]//Eq does not implemented for float//Copy does not implemented for String
            pub struct #ident_options_to_read_upper_camel_case_token_stream(pub #first_field_unnamed_type);
        }
    };
    // let impl_std_convert_from_ident_for_ident_options_to_read_token_stream = {
    //      quote::quote!{
    //         impl std::convert::From<#ident> for #ident_options_to_read_upper_camel_case_token_stream {
    //             fn from(value: #ident) -> Self {
    //                 Self(value)
    //             }
    //         }
    //      }
    // };
    let impl_serde_deserialize_for_ident_options_to_read_token_stream = {
        let ident_options_to_read_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseStringified::impl_quote_to_tokens_self_options_to_read_upper_camel_case_stringified(&ident);
        let ident_options_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_options_to_read_upper_camel_case_token_stream(&ident);
        let tuple_struct_ident_options_to_read_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!("tuple struct {ident_options_to_read_upper_camel_case_stringified}"),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let tuple_struct_ident_options_to_read_with_1_element_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!("tuple struct {ident_options_to_read_upper_camel_case_stringified} with 1 element"),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let ident_options_to_read_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
            &ident_options_to_read_upper_camel_case_stringified,
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        quote::quote!{
            impl<'de> serde::Deserialize<'de> for #ident_options_to_read_upper_camel_case_token_stream {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> serde::__private::Result<Self, __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                {
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: serde::__private::PhantomData<#ident_options_to_read_upper_camel_case_token_stream>,
                        lifetime: serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #ident_options_to_read_upper_camel_case_token_stream;
                        fn expecting(
                            &self,
                            __formatter: &mut serde::__private::Formatter<'_>,
                        ) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(
                                __formatter,
                                #tuple_struct_ident_options_to_read_double_quotes_token_stream,
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
                            let __field0: std::result::Result<
                                #first_field_unnamed_type,
                                std::string::String,
                            > = <std::result::Result<
                                #first_field_unnamed_type,
                                std::string::String,
                            > as serde::Deserialize>::deserialize(__e)?;
                            serde::__private::Ok(#ident_options_to_read_upper_camel_case_token_stream(match __field0 {
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
                                std::result::Result<#first_field_unnamed_type, std::string::String>,
                            >(&mut __seq)? {
                                serde::__private::Some(__value) => __value,
                                serde::__private::None => {
                                    return serde::__private::Err(
                                        serde::de::Error::invalid_length(
                                            0usize,
                                            &#tuple_struct_ident_options_to_read_with_1_element_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                            serde::__private::Ok(#ident_options_to_read_upper_camel_case_token_stream(match __field0 {
                                Ok(value) => value,
                                Err(error) => {
                                    return Err(serde::de::Error::custom(error));
                                }
                            }))
                        }
                    }
                    serde::Deserializer::deserialize_newtype_struct(
                        __deserializer,
                        #ident_options_to_read_upper_camel_case_double_quotes_token_stream,
                        __Visitor {
                            marker: serde::__private::PhantomData::<
                                #ident_options_to_read_upper_camel_case_token_stream,
                            >,
                            lifetime: serde::__private::PhantomData,
                        },
                    )
                }
            }
        }
    };
    let generated = quote::quote!{
        #pub_struct_ident_field_reader_token_stream

        #ident_options_to_read_token_stream
        // #impl_std_convert_from_ident_for_ident_options_to_read_token_stream
        #impl_serde_deserialize_for_ident_options_to_read_token_stream
    };
    generated.into()
}

#[proc_macro_derive(GenerateJsonPostgresqlOptionPrimitiveFieldReader)]
pub fn generate_json_postgresql_option_primitive_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateJsonPostgresqlOptionPrimitiveFieldReader";
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
    let pub_struct_ident_field_reader_token_stream = generate_pub_struct_ident_field_reader_token_stream(&ident, &empty_content_token_stream());
    let ident_options_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_options_to_read_upper_camel_case_token_stream(&ident);
    let ident_options_to_read_token_stream = {
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]//Eq does not implemented for float//Copy does not implemented for String
            pub struct #ident_options_to_read_upper_camel_case_token_stream(pub std::option::Option<#first_field_unnamed_type>);
        }
    };
    // let impl_serde_deserialize_for_ident_options_to_read_token_stream = {
    //     // let ident_options_to_read_upper_camel_case_stringified = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseStringified::impl_quote_to_tokens_self_options_to_read_upper_camel_case_stringified(&ident);
    //     // let ident_options_to_read_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfOptionsToReadUpperCamelCaseTokenStream::impl_quote_to_tokens_self_options_to_read_upper_camel_case_token_stream(&ident);
    //     // let tuple_struct_ident_options_to_read_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //     //     &format!("tuple struct {ident_options_to_read_upper_camel_case_stringified}"),
    //     //     &proc_macro_name_upper_camel_case_ident_stringified
    //     // );
    //     // let tuple_struct_ident_options_to_read_with_1_element_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //     //     &format!("tuple struct {ident_options_to_read_upper_camel_case_stringified} with 1 element"),
    //     //     &proc_macro_name_upper_camel_case_ident_stringified
    //     // );
    //     // let ident_options_to_read_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
    //     //     &ident_options_to_read_upper_camel_case_stringified,
    //     //     &proc_macro_name_upper_camel_case_ident_stringified
    //     // );
    //     if ident == "JsonStdOptionOptionStdPrimitiveI8" {
    //     quote::quote!{
    //         impl<'de> serde::Deserialize<'de> for #ident_options_to_read_upper_camel_case_token_stream {
    //             fn deserialize<__D>(
    //                 __deserializer: __D,
    //             ) -> serde::__private::Result<Self, __D::Error>
    //             where
    //                 __D: serde::Deserializer<'de>,
    //             {
    //                 #[doc(hidden)]
    //                 struct __Visitor<'de> {
    //                     marker: serde::__private::PhantomData<
    //                         #ident_options_to_read_upper_camel_case_token_stream,
    //                     >,
    //                     lifetime: serde::__private::PhantomData<&'de ()>,
    //                 }
    //                 impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
    //                     type Value = #ident_options_to_read_upper_camel_case_token_stream;
    //                     fn expecting(
    //                         &self,
    //                         __formatter: &mut serde::__private::Formatter<'_>,
    //                     ) -> serde::__private::fmt::Result {
    //                         serde::__private::Formatter::write_str(
    //                             __formatter,
    //                             "tuple struct JsonStdOptionOptionStdPrimitiveI8OptionsToRead",
    //                         )
    //                     }
    //                     #[inline]
    //                     fn visit_newtype_struct<__E>(
    //                         self,
    //                         __e: __E,
    //                     ) -> serde::__private::Result<Self::Value, __E::Error>
    //                     where
    //                         __E: serde::Deserializer<'de>,
    //                     {
    //                         let __field0: std::result::Result<
    //                             std::option::Option<JsonStdPrimitiveI8OptionsToRead>,
    //                             std::string::String,
    //                         > = <std::result::Result<
    //                             std::option::Option<JsonStdPrimitiveI8OptionsToRead>,
    //                             std::string::String,
    //                         > as serde::Deserialize>::deserialize(__e)?;
    //                         serde::__private::Ok(
    //                             #ident_options_to_read_upper_camel_case_token_stream(match __field0 {
    //                                 Ok(value) => value,
    //                                 Err(error) => {
    //                                     return Err(serde::de::Error::custom(error));
    //                                 }
    //                             }),
    //                         )
    //                     }
    //                     #[inline]
    //                     fn visit_seq<__A>(
    //                         self,
    //                         mut __seq: __A,
    //                     ) -> serde::__private::Result<Self::Value, __A::Error>
    //                     where
    //                         __A: serde::de::SeqAccess<'de>,
    //                     {
    //                         let __field0 = match serde::de::SeqAccess::next_element::<
    //                             std::result::Result<
    //                                 std::option::Option<JsonStdPrimitiveI8OptionsToRead>,
    //                                 std::string::String,
    //                             >,
    //                         >(&mut __seq)? {
    //                             serde::__private::Some(__value) => __value,
    //                             serde::__private::None => {
    //                                 return serde::__private::Err(
    //                                     serde::de::Error::invalid_length(
    //                                         0usize,
    //                                         &"tuple struct JsonStdOptionOptionStdPrimitiveI8OptionsToRead with 1 element",
    //                                     ),
    //                                 );
    //                             }
    //                         };
    //                         serde::__private::Ok(
    //                             #ident_options_to_read_upper_camel_case_token_stream(match __field0 {
    //                                 Ok(value) => value,
    //                                 Err(error) => {
    //                                     return Err(serde::de::Error::custom(error));
    //                                 }
    //                             }),
    //                         )
    //                     }
    //                 }
    //                 serde::Deserializer::deserialize_newtype_struct(
    //                     __deserializer,
    //                     "JsonStdOptionOptionStdPrimitiveI8OptionsToRead",
    //                     __Visitor {
    //                         marker: serde::__private::PhantomData::<
    //                             #ident_options_to_read_upper_camel_case_token_stream,
    //                         >,
    //                         lifetime: serde::__private::PhantomData,
    //                     },
    //                 )
    //             }
    //         }
    //     }
    //     }
    //     else {
    //         proc_macro2::TokenStream::new()
    //     }
    // };
    let generated = quote::quote!{
        #pub_struct_ident_field_reader_token_stream

        #ident_options_to_read_token_stream
        // #impl_serde_deserialize_for_ident_options_to_read_token_stream
    };
    generated.into()
}

#[proc_macro_derive(GenerateJsonPostgresqlVecPrimitiveFieldReader)]
pub fn generate_json_postgresql_vec_primitive_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateJsonPostgresqlVecPrimitiveFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    // let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let pub_struct_ident_field_reader_token_stream = generate_pub_struct_ident_field_reader_token_stream(&ident, &pagination_content_token_stream());
    let generated = quote::quote!{
        #pub_struct_ident_field_reader_token_stream
    };
    generated.into()
}

#[proc_macro_derive(GenerateJsonPostgresqlOptionVecPrimitiveFieldReader)]
pub fn generate_json_postgresql_option_vec_primitive_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateJsonPostgresqlOptionVecPrimitiveFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    // let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let pub_struct_ident_field_reader_token_stream = generate_pub_struct_ident_field_reader_token_stream(&ident, &pagination_content_token_stream());
    let generated = quote::quote!{
        #pub_struct_ident_field_reader_token_stream
    };
    generated.into()
}

#[proc_macro_derive(GenerateJsonPostgresqlVecOptionPrimitiveFieldReader)]
pub fn generate_json_postgresql_vec_option_primitive_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateJsonPostgresqlVecOptionPrimitiveFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    // let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let pub_struct_ident_field_reader_token_stream = generate_pub_struct_ident_field_reader_token_stream(&ident, &pagination_content_token_stream());
    let generated = quote::quote!{
        #pub_struct_ident_field_reader_token_stream
    };
    generated.into()
}

#[proc_macro_derive(GenerateJsonPostgresqlOptionVecOptionPrimitiveFieldReader)]
pub fn generate_json_postgresql_option_vec_option_primitive_field_reader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateJsonPostgresqlOptionVecOptionPrimitiveFieldReader";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    // let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let pub_struct_ident_field_reader_token_stream = generate_pub_struct_ident_field_reader_token_stream(&ident, &pagination_content_token_stream());
    let generated = quote::quote!{
        #pub_struct_ident_field_reader_token_stream
    };
    generated.into()
}
///
fn impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
    ident: &syn::Ident,
    content_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let ident_field_reader_upper_camel_case_token_stream = naming_conventions::ImplQuoteToTokensSelfFieldReaderUpperCamelCaseTokenStream::impl_quote_to_tokens_self_field_reader_upper_camel_case_token_stream(&ident);
    quote::quote!{
        impl GeneratePostgresqlQueryPartFieldToRead for #ident_field_reader_upper_camel_case_token_stream {
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

#[derive(Debug)]
enum PrimitiveJsonType {
    Number,
    Boolean,
    String
}
impl std::fmt::Display for PrimitiveJsonType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Number => write!(f, "number"),
            Self::Boolean => write!(f, "boolean"),
            Self::String => write!(f, "string"),
        }
    }
}

fn generate_primitive_postgresql_part_field_to_read_query(
    primitive_json_type: PrimitiveJsonType,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    proc_macro_common::generate_quotes::double_quotes_token_stream(
        &format!("jsonb_build_object('{{field_ident}}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = '{primitive_json_type}' then jsonb_build_object('Ok',{{column_name_and_maybe_field_getter}}->'{{field_ident}}') else jsonb_build_object('Err', 'type of {{column_name_and_maybe_field_getter_for_error_message}}.{{field_ident}} is not {primitive_json_type}') end)"),
        &proc_macro_name_upper_camel_case_ident_stringified
    )
}
fn generate_nullable_primitive_postgresql_part_field_to_read_query(
    primitive_json_type: PrimitiveJsonType,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    proc_macro_common::generate_quotes::double_quotes_token_stream(
        &format!("jsonb_build_object('{{field_ident}}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = '{primitive_json_type }' then jsonb_build_object('Ok',{{column_name_and_maybe_field_getter}}->'{{field_ident}}') when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}}.{{field_ident}} is not {primitive_json_type } and not null') end)"),
        &proc_macro_name_upper_camel_case_ident_stringified
    )
}
fn generate_array_primitive_postgresql_part_field_to_read_query(
    primitive_json_type: PrimitiveJsonType,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    proc_macro_common::generate_quotes::double_quotes_token_stream(
        &format!("jsonb_build_object('{{field_ident}}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 'array' then jsonb_build_object('Ok',(select jsonb_agg(case when jsonb_typeof(value) = '{primitive_json_type}' then jsonb_build_object('Ok', value) else jsonb_build_object('Err', 'type of {{column_name_and_maybe_field_getter_for_error_message}}.{{field_ident}}[array element] is not {primitive_json_type}') end) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})) else jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}}.{{field_ident}} is not array') end)"),
        &proc_macro_name_upper_camel_case_ident_stringified
    )
}
fn generate_nullable_array_primitive_postgresql_part_field_to_read_query(
    primitive_json_type: PrimitiveJsonType,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    proc_macro_common::generate_quotes::double_quotes_token_stream(
        &format!("jsonb_build_object('{{field_ident}}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 'array' then jsonb_build_object('Ok',(select jsonb_agg(case when jsonb_typeof(value) = '{primitive_json_type}' then jsonb_build_object('Ok',value) else jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}}.{{field_ident}}[array element] is not {primitive_json_type}') end) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})) when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}}.{{field_ident}} is not array and not null') end)"),
        &proc_macro_name_upper_camel_case_ident_stringified
    )
}
fn generate_array_nullable_primitive_postgresql_part_field_to_read_query(
    primitive_json_type: PrimitiveJsonType,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    proc_macro_common::generate_quotes::double_quotes_token_stream(
        &format!("jsonb_build_object('{{field_ident}}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 'array' then jsonb_build_object('Ok',(select jsonb_agg(case when jsonb_typeof(value) = '{primitive_json_type}' then jsonb_build_object('Ok',value) when jsonb_typeof(value) = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}}.{{field_ident}}[array element] is not {primitive_json_type} and not null') end) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})) else jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}}.{{field_ident}} is not array') end)"),
        &proc_macro_name_upper_camel_case_ident_stringified
    )
}
fn generate_nullable_array_nullable_primitive_postgresql_part_field_to_read_query(
    primitive_json_type: PrimitiveJsonType,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    proc_macro_common::generate_quotes::double_quotes_token_stream(
        &format!("jsonb_build_object('{{field_ident}}',case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 'array' then jsonb_build_object('Ok',(select jsonb_agg(case when jsonb_typeof(value) = '{primitive_json_type}' then jsonb_build_object('Ok',value) when jsonb_typeof(value) = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}}.{{field_ident}}[array element] is not {primitive_json_type} and not null') end) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}}))when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}}.{{field_ident}} is not array and not null') end)"),
        &proc_macro_name_upper_camel_case_ident_stringified
    )
}
///
#[proc_macro_derive(GenerateGetJsonRepresentationNumber)]
pub fn generate_get_json_representation_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_primitive_postgresql_part_field_to_read_query(
            PrimitiveJsonType::Number,
            &proc_macro_name_upper_camel_case_ident_stringified
        ))
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::Number
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationBoolean)]
pub fn generate_get_json_representation_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_primitive_postgresql_part_field_to_read_query(
            PrimitiveJsonType::Boolean,
            &proc_macro_name_upper_camel_case_ident_stringified
        ))
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::Boolean
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationString)]
pub fn generate_get_json_representation_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_primitive_postgresql_part_field_to_read_query(
            PrimitiveJsonType::String,
            &proc_macro_name_upper_camel_case_ident_stringified
        ))
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::String
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationNullableNumber)]
pub fn generate_get_json_representation_nullable_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationNullableNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_nullable_primitive_postgresql_part_field_to_read_query(
            PrimitiveJsonType::Number,
            &proc_macro_name_upper_camel_case_ident_stringified
        ))
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::NullableNumber
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationNullableBoolean)]
pub fn generate_get_json_representation_nullable_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationNullableBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_nullable_primitive_postgresql_part_field_to_read_query(
            PrimitiveJsonType::Boolean,
            &proc_macro_name_upper_camel_case_ident_stringified
        ))
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::NullableBoolean
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationNullableString)]
pub fn generate_get_json_representation_nullable_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationNullableString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &generate_primitive_postgresql_part_field_to_read_query_format(&generate_nullable_primitive_postgresql_part_field_to_read_query(
            PrimitiveJsonType::String,
            &proc_macro_name_upper_camel_case_ident_stringified
        ))
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::NullableString
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationArrayNumber)]
pub fn generate_get_json_representation_array_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationArrayNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_array_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::Number,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::ArrayNumber
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationArrayBoolean)]
pub fn generate_get_json_representation_array_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationArrayBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_array_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::Boolean,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::ArrayBoolean
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationArrayString)]
pub fn generate_get_json_representation_array_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationArrayString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_array_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::String,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::ArrayString
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationNullableArrayNumber)]
pub fn generate_get_json_representation_nullable_array_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationNullableArrayNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_nullable_array_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::Number,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::NullableArrayNumber
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationNullableArrayBoolean)]
pub fn generate_get_json_representation_nullable_array_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationNullableArrayBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_nullable_array_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::Boolean,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::NullableArrayBoolean
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationNullableArrayString)]
pub fn generate_get_json_representation_nullable_array_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationNullableArrayString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_nullable_array_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::String,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::NullableArrayString
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationArrayNullableNumber)]
pub fn generate_get_json_representation_array_nullable_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationArrayNullableNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_array_nullable_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::Number,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::ArrayNullableNumber
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationArrayNullableBoolean)]
pub fn generate_get_json_representation_array_nullable_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationArrayNullableBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_array_nullable_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::Boolean,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::ArrayNullableBoolean
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationArrayNullableString)]
pub fn generate_get_json_representation_array_nullable_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationArrayNullableString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_array_nullable_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::String,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::ArrayNullableString
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationNullableArrayNullableNumber)]
pub fn generate_get_json_representation_nullable_array_nullable_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationNullableArrayNullableNumber";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_nullable_array_nullable_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::Number,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::NullableArrayNullableNumber
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationNullableArrayNullableBoolean)]
pub fn generate_get_json_representation_nullable_array_nullable_boolean(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationNullableArrayNullableBoolean";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_nullable_array_nullable_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::Boolean,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::NullableArrayNullableBoolean
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}
#[proc_macro_derive(GenerateGetJsonRepresentationNullableArrayNullableString)]
pub fn generate_get_json_representation_nullable_array_nullable_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateGetJsonRepresentationNullableArrayNullableString";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream = impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream(
        &ident,
        &postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
            &generate_nullable_array_nullable_primitive_postgresql_part_field_to_read_query(
                PrimitiveJsonType::String,
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        )
    );
    let generated = quote::quote!{
        impl GetJsonRepresentation for #ident {
            fn get_json_representation() -> JsonRepresentation {
                JsonRepresentation::NullableArrayNullableString
            }
        }
        #impl_generate_postgresql_query_part_field_to_read_for_ident_token_stream
    };
    generated.into()
}