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
    generated.into()
}

#[proc_macro_derive(CommonWithEqImpl)] //todo check on postgresql max length value of type
pub fn common_with_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    common_handle(input, "CommonWithEqImpl", true, true, true)
}
#[proc_macro_derive(CommonWithoutEqImpl)] //todo check on postgresql max length value of type
pub fn common_without_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    common_handle(input, "CommonWithoutEqImpl", false, false, false)
}

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

#[proc_macro_derive(GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath)]
pub fn generate_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case} {}: {error}", proc_macro_common::constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let generated = quote::quote! {
        impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident {
            #[inline]
            fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                Self(::core::default::Default::default())
            }
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

fn generate_impl_postgresql_json_type_token_stream(input: proc_macro::TokenStream, variant: StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant, proc_macro_name_upper_camel_case: &std::primitive::str) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
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

    let impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream = {
        let content_token_stream = match &variant {
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
        quote::quote!{
            impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident {
                #[inline]
                fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                    Self(#content_token_stream)
                }
            }
        }
    };
    let ident_to_create_upper_camel_case = naming_conventions::SelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_to_create_alias_token_stream = proc_macro_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_to_create_upper_camel_case, &ident);
    let ident_field_reader_upper_camel_case = naming_conventions::SelfFieldReaderUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let (
        ident_field_reader_token_stream,
        impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_reader_token_stream
    ) = {
        let ident_field_reader_token_stream = {
            let content_token_stream = match &variant {
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath | StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath => quote::quote!{{}},
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote!{{ pagination: crate::generate_postgresql_query_part::Pagination }},
            };
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
            let content_token_stream = match &variant {
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath | StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath => quote::quote! {
                    ::core::default::Default::default()
                },
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote! {
                    Self {
                        pagination: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                    }
                },
            };
            quote::quote!{
                impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_field_reader_upper_camel_case {
                    #[inline]
                    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        #content_token_stream
                    }
                }
            }
        };
        (
            ident_field_reader_token_stream,
            impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_reader_token_stream
        )
    };
    let ident_options_to_read_upper_camel_case = naming_conventions::SelfOptionsToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_options_to_read_alias_token_stream = proc_macro_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_options_to_read_upper_camel_case, &ident);
    let ident_option_to_update_upper_camel_case = naming_conventions::SelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_option_to_update_alias_token_stream = proc_macro_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_option_to_update_upper_camel_case, &ident);
    let ident_option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case = naming_conventions::SelfOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_option_to_update_try_generate_bind_increments_error_named_token_stream = {
        quote::quote!{
            #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
            pub enum #ident_option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case {
                CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
            }
        }
    };
    let impl_crate_generate_postgresql_query_part_postgresql_json_type_for_ident_token_stream = {
        let (
            to_create_token_stream,
            try_generate_postgresql_query_part_to_create_token_stream,
            bind_value_to_postgresql_query_part_to_create_token_stream
        ) = {
            let to_create_upper_camel_case = naming_conventions::ToCreateUpperCamelCase;
            let to_create_token_stream = {
                quote::quote!{
                    type #to_create_upper_camel_case<'a> = #ident_to_create_upper_camel_case;
                }
            };
            //todo maybe rename later
            let try_generate_postgresql_query_part_to_create_token_stream = {
                quote::quote!{
                    fn try_generate_postgresql_query_part_to_create(
                        self_to_create: &Self::#to_create_upper_camel_case<'_>,
                        increment: &mut std::primitive::u64
                    ) -> Result<std::string::String, crate::generate_postgresql_query_part::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
                        match increment.checked_add(1) {
                            Some(incr) => {
                                *increment = incr;
                                Ok(format!("${increment}"))
                            }
                            None => Err(crate::generate_postgresql_query_part::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                        }
                    }
                }
            };
            let bind_value_to_postgresql_query_part_to_create_token_stream = {
                quote::quote!{
                    fn bind_value_to_postgresql_query_part_to_create<'a>(
                        self_to_create: Self::#to_create_upper_camel_case<'a>,
                        mut query: sqlx::query::Query<'a,
                        sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        query = query.bind(sqlx::types::Json(self_to_create.0));
                        query
                    }
                }
            };
            (
                to_create_token_stream,
                try_generate_postgresql_query_part_to_create_token_stream,
                bind_value_to_postgresql_query_part_to_create_token_stream
            )
        };
        let field_reader_upper_camel_case = naming_conventions::FieldReaderUpperCamelCase;
        let field_reader_token_stream = {
            quote::quote!{
                type #field_reader_upper_camel_case<'a> = #ident_field_reader_upper_camel_case;
            }
        };
        let (
            options_to_read_token_stream,
            generate_postgresql_query_part_to_read_token_stream
        ) = {
            let options_to_read_upper_camel_case = naming_conventions::OptionsToReadUpperCamelCase;
            let options_to_read_token_stream = {
                quote::quote!{
                    type #options_to_read_upper_camel_case<'a> = #ident_options_to_read_upper_camel_case;
                }
            };
            //todo maybe rename later
            let generate_postgresql_query_part_to_read_token_stream = {
                let field_reader_snake_case = naming_conventions::FieldReaderSnakeCase;
                let postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream = |format_handle_token_stream: &proc_macro2::TokenStream| {
                    let pagination_start_end_initialization_token_stream = proc_macro_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream(&field_reader_snake_case);
                    quote::quote! {
                        #pagination_start_end_initialization_token_stream
                        format!(#format_handle_token_stream)
                    }
                };
                let content_token_stream = match &variant {
                    StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath |
                    StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath => quote::quote! {
                        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter}->'{field_ident}'))")
                    },
                     //different order
                    StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath |
                    StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                        &proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        )
                    ),
                    StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath |
                    StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                        &proc_macro_common::generate_quotes::double_quotes_token_stream(
                            &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value', case when jsonb_typeof({{column_name_and_maybe_field_getter}}->'{{field_ident}}') = 'array' then (select jsonb_agg(value) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}}) else null end))"),
                            &proc_macro_name_upper_camel_case_ident_stringified
                        )
                    ),
                };
                quote::quote!{
                    fn generate_postgresql_query_part_to_read(
                        #field_reader_snake_case: &Self::#field_reader_upper_camel_case<'_>,
                        field_ident: &std::primitive::str,
                        column_name_and_maybe_field_getter: &std::primitive::str,
                        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str
                    ) -> std::string::String {
                        #content_token_stream
                    }
                }
            };
            (
                options_to_read_token_stream,
                generate_postgresql_query_part_to_read_token_stream
            )
        };
        let (
            option_to_update_token_stream,
            option_to_update_try_generate_bind_increments_error_named_token_stream,
            try_generate_bind_increments_token_stream,
            bind_value_to_query_token_stream
        ) = {
            let option_to_update_upper_camel_case = naming_conventions::OptionToUpdateUpperCamelCase;
            let option_to_update_token_stream = {
                quote::quote!{
                    type #option_to_update_upper_camel_case<'a>: = #ident_option_to_update_upper_camel_case;
                }
            };
            let option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case = naming_conventions::OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamedUpperCamelCase;
            let option_to_update_try_generate_postgresql_query_part_error_named_token_stream = {
                quote::quote!{
                    type #option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case = #ident_option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case;
                }
            };
            //todo maybe rename later
            let try_generate_bind_increments_token_stream = {
                quote::quote!{
                    fn try_generate_postgresql_query_part_to_update(
                        option_to_update: &Self::#option_to_update_upper_camel_case<'_>,
                        jsonb_set_accumulator: &std::primitive::str,
                        jsonb_set_target: &std::primitive::str,
                        jsonb_set_path: &std::primitive::str,
                        increment: &mut std::primitive::u64,
                    ) -> Result<std::string::String, Self::#option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case> {
                        match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                            }
                            None => Err(Self::#option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                        }
                    }
                }
            };
            //todo maybe rename later
            let bind_value_to_query_token_stream = {
                quote::quote!{
                    fn bind_value_to_postgresql_query_part_to_update<'a>(
                        option_to_update: Self::#option_to_update_upper_camel_case<'_>,
                        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        query = query.bind(sqlx::types::Json(option_to_update.0));
                        query
                    }
                }
            };
            (
                option_to_update_token_stream,
                option_to_update_try_generate_postgresql_query_part_error_named_token_stream,
                try_generate_bind_increments_token_stream,
                bind_value_to_query_token_stream
            )
        };
        quote::quote!{
            impl crate::generate_postgresql_query_part::PostgresqlJsonType for #ident {
                #to_create_token_stream
                #try_generate_postgresql_query_part_to_create_token_stream
                #bind_value_to_postgresql_query_part_to_create_token_stream
                #field_reader_token_stream
                #options_to_read_token_stream
                #generate_postgresql_query_part_to_read_token_stream
                #option_to_update_token_stream
                #option_to_update_try_generate_bind_increments_error_named_token_stream
                #try_generate_bind_increments_token_stream
                #bind_value_to_query_token_stream
            }
        }
    };
    let generated = quote::quote!{
        #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream

        #ident_to_create_alias_token_stream
        #ident_field_reader_token_stream
        #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_field_reader_token_stream
        #ident_options_to_read_alias_token_stream
        #ident_option_to_update_alias_token_stream
        #ident_option_to_update_try_generate_bind_increments_error_named_token_stream
        #impl_crate_generate_postgresql_query_part_postgresql_json_type_for_ident_token_stream
    };
    generated.into()
}

#[proc_macro_derive(GenerateImplPostgresqlJsonTypeFullTypePath)]
pub fn generate_impl_postgresql_json_type_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath, "GenerateImplPostgresqlJsonTypeFullTypePath")
}
#[proc_macro_derive(GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath)]
pub fn generate_impl_postgresql_json_type_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath, "GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath")
}
#[proc_macro_derive(GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath)]
pub fn generate_impl_postgresql_json_type_std_vec_vec_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath, "GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath")
}
#[proc_macro_derive(GenerateImplPostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath)]
pub fn generate_impl_postgresql_json_type_std_option_option_std_vec_vec_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath, "GenerateImplPostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath")
}
#[proc_macro_derive(GenerateImplPostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath)]
pub fn generate_impl_postgresql_json_type_std_vec_vec_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath, "GenerateImplPostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath")
}
#[proc_macro_derive(GenerateImplPostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub fn generate_impl_postgresql_json_type_std_option_option_std_vec_vec_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath, "GenerateImplPostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath")
}