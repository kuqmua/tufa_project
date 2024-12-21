fn common_handle(
    input: proc_macro::TokenStream,
    where_ident_should_implement_eq: std::primitive::bool,
    std_option_option_ident_upper_camel_case_should_implement_eq: std::primitive::bool,
    where_std_option_option_ident_upper_camel_case_should_implement_eq: std::primitive::bool,
) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    // println!("{:#?}", syn_derive_input.data);
    let ident = &syn_derive_input.ident;
    let field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            match fields_unnamed.unnamed.len() {
                1 => &fields_unnamed.unnamed[0],
                _ => panic!("supports only syn::Fields::Unnamed with one field"),
            }
        } else {
            panic!("supports only syn::Fields::Unnamed");
        }
    } else {
        panic!("does work only on structs!");
    };
    let field_type = &field.ty;
    let where_ident_token_stream = {
        let value = format!("{}{ident}", naming::WhereUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!("{}{ident}", naming::StdOptionOptionUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!("{}{}{ident}", naming::WhereUpperCamelCase, naming::StdOptionOptionUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
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
            pub logical_operator: LogicalOperator,
        }
        impl std::fmt::Display for #where_ident_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "value: {}, logical_operator: {}", self.value, self.logical_operator)
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
        impl crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case for #where_ident_token_stream {
            fn #std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case() -> Self {
                Self {
                    logical_operator: crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case(),
                    value: crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case(),
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
        impl crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case for #std_option_option_ident_upper_camel_case_token_stream {
            fn #std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case() -> Self {
                Self(Some(crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()))
            }
        }
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, #where_std_option_option_ident_upper_camel_case_should_implement_eq_token_stream)]
        pub struct #where_std_option_option_ident_upper_camel_case_token_stream {
            pub value: #std_option_option_ident_upper_camel_case_token_stream ,
            pub logical_operator: LogicalOperator,
        }
        impl std::fmt::Display for #where_std_option_option_ident_upper_camel_case_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    formatter,
                    "value: {}, logical_operator: {}",
                    self.value, self.logical_operator
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
        impl crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case for #where_std_option_option_ident_upper_camel_case_token_stream {
            fn #std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case() -> Self {
                Self {
                    value: crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case(),
                    logical_operator: LogicalOperator::default(),
                }
            }
        }
    };
    generated.into()
}

#[proc_macro_derive(CommonWithEqImpl)] //todo check on postgresql max length value of type
pub fn common_with_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    common_handle(input, true, true, true)
}
#[proc_macro_derive(CommonWithoutEqImpl)] //todo check on postgresql max length value of type
pub fn common_without_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    common_handle(input, false, false, false)
}

#[proc_macro_derive(AsPostgresqlCommon)] //todo check on postgresql max length value of type
pub fn as_postgresql_common(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    // println!("{:#?}", syn_derive_input.data);
    let ident = &syn_derive_input.ident;
    let _field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            match fields_unnamed.unnamed.len() {
                1 => &fields_unnamed.unnamed[0],
                _ => panic!("supports only syn::Fields::Unnamed with one field"),
            }
        } else {
            panic!("supports only syn::Fields::Unnamed");
        }
    } else {
        panic!("does work only on structs!");
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
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let generated = quote::quote! {
        impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident {
            #[inline]
            fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
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

fn generate_postgresql_json_type_token_stream(input: proc_macro::TokenStream, variant: StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;

    let data_struct = match syn_derive_input.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("only works on Struct"),
    };
    let fields_unnamed = match data_struct.fields {
        syn::Fields::Unnamed(value) => value.unnamed,
        syn::Fields::Named(_) | syn::Fields::Unit => panic!("only works with syn::Fields::Unnamed"),
    };
    assert!(fields_unnamed.len() == 1, "fields_unnamed !== 1");

    let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream = {
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
        let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
        let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
        let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
        quote::quote!{
            impl crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case_case for #ident {
                #[inline]
                fn #std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case() -> Self {
                    Self(#content_token_stream)
                }
            }
        }
    };
    let postgresql_json_type_ident_to_create_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfToCreateUpperCamelCase::from_tokens(&ident);
    let postgresql_json_type_ident_to_create_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_json_type_ident_to_create_upper_camel_case, &ident);
    let postgresql_json_type_ident_field_reader_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfFieldReaderUpperCamelCase::from_tokens(&ident);
    let (
        postgresql_json_type_ident_field_reader_token_stream,
        impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_field_reader_token_stream
    ) = {
        let postgresql_json_type_ident_field_reader_token_stream = {
            let content_token_stream = match &variant {
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath | StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath => quote::quote!{{}},
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote!{{ pagination: crate::generate_postgresql_json_type::Pagination }},
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
                pub struct #postgresql_json_type_ident_field_reader_upper_camel_case #content_token_stream
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_field_reader_token_stream = {
            let content_token_stream = match &variant {
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath | StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath => quote::quote! {
                    ::core::default::Default::default()
                },
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => {
                    let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
                    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
                    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
                    quote::quote! {
                        Self {
                            pagination: crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case(),
                        }
                    }
                },
            };
            quote::quote!{
                impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_json_type_ident_field_reader_upper_camel_case {
                    #[inline]
                    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        #content_token_stream
                    }
                }
            }
        };
        (
            postgresql_json_type_ident_field_reader_token_stream,
            impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_field_reader_token_stream
        )
    };
    let postgresql_json_type_ident_options_to_read_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfOptionsToReadUpperCamelCase::from_tokens(&ident);
    let postgresql_json_type_ident_options_to_read_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_json_type_ident_options_to_read_upper_camel_case, &ident);
    let postgresql_json_type_ident_option_to_update_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfOptionToUpdateUpperCamelCase::from_tokens(&ident);
    let postgresql_json_type_ident_option_to_update_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_json_type_ident_option_to_update_upper_camel_case, &ident);
    let postgresql_json_type_ident_option_to_update_try_generate_postgresql_json_type_error_named_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamedUpperCamelCase::from_tokens(&ident);

    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;

    let postgresql_json_type_ident_option_to_update_try_generate_bind_increments_error_named_token_stream = {
        quote::quote!{
            #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            pub enum #postgresql_json_type_ident_option_to_update_try_generate_postgresql_json_type_error_named_upper_camel_case {
                #checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
            }
        }
    };
    let impl_crate_generate_postgresql_json_type_postgresql_json_type_for_ident_token_stream = postgresql_crud_macros_common::generate_postgresql_json_type_token_stream(
        &quote::quote!{crate::postgresql_json_type::postgresql_json_type_trait::},
        &ident,
        &postgresql_json_type_ident_to_create_upper_camel_case,
        &{
            let crate_json_types_postgresql_json_type_try_generate_postgresql_json_type_to_create_error_named_token_stream = quote::quote!{
                crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed
            };
            quote::quote!{
                match increment.checked_add(1) {
                    Some(value) => {
                        *increment = value;
                        Ok(format!("${increment}"))
                    }
                    None => Err(#crate_json_types_postgresql_json_type_try_generate_postgresql_json_type_to_create_error_named_token_stream::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!()
                    }),
                }
            }
        },
        &{
            let postgresql_json_type_self_to_create_snake_case = naming::PostgresqlJsonTypeSelfToCreateSnakeCase;
            quote::quote!{
                query = query.bind(sqlx::types::Json(#postgresql_json_type_self_to_create_snake_case.0));
                query
            }
        },
        &postgresql_json_type_ident_field_reader_upper_camel_case,
        &postgresql_json_type_ident_options_to_read_upper_camel_case,
        &{
            let postgresql_json_type_self_field_reader_snake_case = naming::PostgresqlJsonTypeSelfFieldReaderSnakeCase;
            let postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream = |format_handle_token_stream: &dyn quote::ToTokens| {
                let pagination_start_end_initialization_token_stream = macros_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream(&postgresql_json_type_self_field_reader_snake_case);
                quote::quote! {
                    #pagination_start_end_initialization_token_stream
                    format!(#format_handle_token_stream)
                }
            };
            let column_name_and_maybe_field_getter_snake_case = naming::ColumnNameAndMaybeFieldGetterSnakeCase;
            match &variant {
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath => {
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                        &format!("jsonb_build_object('{{field_ident}}', jsonb_build_object('value', {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}'))")
                    );
                    quote::quote! {
                        format!(#format_handle_token_stream)
                    }
                },
                 //different order
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                    &generate_quotes::double_quotes_token_stream(
                        &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))")
                    )
                ),
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath |
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                    &generate_quotes::double_quotes_token_stream(
                        &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value', case when jsonb_typeof({{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}') = 'array' then (select jsonb_agg(value) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}}) else null end))")
                    )
                ),
            }
        },
        &postgresql_json_type_ident_option_to_update_upper_camel_case,
        &postgresql_json_type_ident_option_to_update_try_generate_postgresql_json_type_error_named_upper_camel_case,
        &{
            let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                &format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{jsonb_set_path}}}}}}',${{increment}})")
            );
            let postgresql_json_type_self_option_to_update_try_generate_postgresql_json_type_error_named_upper_camel_case = naming::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamedUpperCamelCase;
            quote::quote!{
                match increment.checked_add(1) {
                    Some(value) => {
                        *increment = value;
                        Ok(format!(#format_handle_token_stream))
                    }
                    None => Err(Self::#postgresql_json_type_self_option_to_update_try_generate_postgresql_json_type_error_named_upper_camel_case::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
        },
        &{
            let postgresql_json_type_self_option_to_update_snake_case = naming::PostgresqlJsonTypeSelfOptionToUpdateSnakeCase;
            quote::quote!{
                query = query.bind(sqlx::types::Json(#postgresql_json_type_self_option_to_update_snake_case.0));
                query
            }
        }
    );
    let impl_crate_bind_query_for_token_stream = {
        quote::quote!{
            impl crate::BindQuerySecond<'_> for #ident {
                fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!("${increment}"))
                        }
                        None => Err(crate::TryGenerateBindIncrementsErrorNamed::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!()
                        }),
                    }
                }
                fn bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    query = query.bind(sqlx::types::Json(self.0));
                    query
                }
            }
        }
    };
    //todo maybe impl Encode instead of just wrap into sqlx::types::Json
    let generated = quote::quote!{
        #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream

        #postgresql_json_type_ident_to_create_alias_token_stream
        #postgresql_json_type_ident_field_reader_token_stream
        #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_field_reader_token_stream
        #postgresql_json_type_ident_options_to_read_alias_token_stream
        #postgresql_json_type_ident_option_to_update_alias_token_stream
        #postgresql_json_type_ident_option_to_update_try_generate_bind_increments_error_named_token_stream
        #impl_crate_generate_postgresql_json_type_postgresql_json_type_for_ident_token_stream

        #impl_crate_bind_query_for_token_stream
    };
    // if ident == "" {
    //     println!("{generated}");
    //     println!("-------");
    // }
    generated.into()
}

#[proc_macro_derive(GeneratePostgresqlJsonTypeFullTypePath)]
pub fn generate_postgresql_json_type_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath)
}
#[proc_macro_derive(GeneratePostgresqlJsonTypeStdOptionOptionFullTypePath)]
pub fn generate_postgresql_json_type_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath)
}
#[proc_macro_derive(GeneratePostgresqlJsonTypeStdVecVecFullTypePath)]
pub fn generate_postgresql_json_type_std_vec_vec_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath)
}
#[proc_macro_derive(GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath)]
pub fn generate_postgresql_json_type_std_option_option_std_vec_vec_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath)
}
#[proc_macro_derive(GeneratePostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath)]
pub fn generate_postgresql_json_type_std_vec_vec_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath)
}
#[proc_macro_derive(GeneratePostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub fn generate_postgresql_json_type_std_option_option_std_vec_vec_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath)
}


///////////////////////////////
fn generate_impl_crate_bind_query_for_tokens_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    try_generate_bind_increments_token_stream: &dyn quote::ToTokens,
    bind_value_to_query_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let std_string_string_token_stream = token_patterns::StdStringString;
    let self_snake_case = naming::SelfSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let crate_bind_query_token_stream = quote::quote!{crate::BindQuerySecond::};
    let try_generate_bind_increments_snake_case = naming::TryGenerateBindIncrementsSnakeCase;
    let bind_value_to_query_snake_case = naming::BindValueToQuerySnakeCase;
    quote::quote!{
        impl #crate_bind_query_token_stream<'_> for #ident_token_stream {
            fn #try_generate_bind_increments_snake_case(&#self_snake_case, #increment_snake_case: &mut std::primitive::u64) -> Result<#std_string_string_token_stream, crate::#try_generate_bind_increments_error_named_upper_camel_case> {
                #try_generate_bind_increments_token_stream
            }
            fn #bind_value_to_query_snake_case(#self_snake_case, mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #bind_value_to_query_token_stream
            }
        }
    }
}
fn generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    self_content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
        let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
        let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
        quote::quote! {crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case}
    };
    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
    quote::quote!{
        impl #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream for #ident_token_stream {
            fn #std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case() -> Self {
                Self #self_content_token_stream
            }
        }
    }
}
fn generate_impl_std_fmt_display_for_tokens_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens
) -> proc_macro2::TokenStream {
    let self_snake_case = naming::SelfSnakeCase;
    quote::quote!{
        impl std::fmt::Display for #ident_token_stream {
            fn fmt(&#self_snake_case, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, #content_token_stream)
            }
        }
    }
}
fn generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    field_type_token_stream: &dyn quote::ToTokens
) -> proc_macro2::TokenStream {
    let field_type_as_sqlx_type_sqlx_postgres_token_stream = quote::quote!{};
    quote::quote! {
        impl sqlx::Type<sqlx::Postgres> for #ident_token_stream {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
               <#field_type_token_stream as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }
    }
}
fn generate_impl_sqlx_encode_sqlx_postgres_for_tokens_token_stream(ident_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream{
    let self_snake_case = naming::SelfSnakeCase;
    quote::quote! {
        impl sqlx::Encode<'_, sqlx::Postgres> for #ident_token_stream {
            fn encode_by_ref(&#self_snake_case, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#self_snake_case.0, buf)
            }
        }
    }
}
fn generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    field_type_token_stream: &dyn quote::ToTokens
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let error_snake_case = naming::ErrorSnakeCase;
    quote::quote! {
        impl sqlx::Decode<'_, sqlx::Postgres> for #ident_token_stream {
            fn decode(#value_snake_case: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                match <#field_type_token_stream as sqlx::Decode<sqlx::Postgres>>::decode(#value_snake_case) {
                    Ok(#value_snake_case) => Ok(Self(#value_snake_case)),
                    Err(#error_snake_case) => Err(#error_snake_case)
                }
            }
        }
    }
}
enum Visibility {
    Pub,
    PubCrate,
    Private
}
impl quote::ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let pub_snake_case = naming::PubSnakeCase;
        let crate_snake_case = naming::CrateSnakeCase;
        match &self {
            Visibility::Pub => naming::PubSnakeCase.to_tokens(tokens),
            Visibility::PubCrate => quote::quote!{#pub_snake_case(#crate_snake_case)}.to_tokens(tokens),
            Visibility::Private => (),
        }
        
    }
}
fn generate_pub_struct_tokens_token_stream(
    visibility: Visibility,
    ident_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
    impl_default: std::primitive::bool,
    impl_deserialize: std::primitive::bool,
) -> proc_macro2::TokenStream {
    let maybe_impl_default_token_stream = if impl_default {
        quote::quote! {Default,}
    }
    else {
        proc_macro2::TokenStream::new()
    };
    let maybe_impl_serde_deserialize_token_stream = if impl_deserialize {
        quote::quote! {serde::Deserialize,}
    }
    else {
        proc_macro2::TokenStream::new()
    };
    quote::quote! {
        #[derive(
            Debug,
            #maybe_impl_default_token_stream
            Clone,
            PartialEq,
            serde::Serialize,
            #maybe_impl_serde_deserialize_token_stream
        )]
        #visibility struct #ident_token_stream #content_token_stream
    }
}
fn generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(ident_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let error_occurence_lib_snake_case = naming::ErrorOccurenceLibSnakeCase;
    let to_std_string_string_upper_camel_case = naming::ToStdStringStringUpperCamelCase;
    let to_std_string_string_snake_case = naming::ToStdStringStringSnakeCase;
    let std_string_string_token_stream = token_patterns::StdStringString;
    let self_snake_case = naming::SelfSnakeCase;
    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{{self_snake_case}}}"));
    quote::quote!{
        impl #error_occurence_lib_snake_case::#to_std_string_string_upper_camel_case for #ident_token_stream {
            fn #to_std_string_string_snake_case(&#self_snake_case) -> #std_string_string_token_stream {
                format!(#format_handle_token_stream)
            }
        }
    }
}
fn extract_first_syn_type_from_unnamed_struct<'a>(syn_derive_input: &'a syn::DeriveInput) -> &'a syn::Type {
    if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            match fields_unnamed.unnamed.len() {
                1 => &fields_unnamed.unnamed[0].ty,
                _ => panic!("supports only syn::Fields::Unnamed with one field"),
            }
        } else {
            panic!("supports only syn::Fields::Unnamed");
        }
    } else {
        panic!("does work only on structs!");
    }
}

#[proc_macro_derive(PostgresqlBaseTypeTokens)]
pub fn postgresql_base_type_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let std_option_option_field_type_token_stream = quote::quote!{std::option::Option<#field_type>};
    let std_option_option_ident_upper_camel_case = naming::parameter::StdOptionOptionSelfUpperCamelCase::from_tokens(&ident);
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let impl_error_occurence_lib_to_std_string_string_for_ident_token_stream = {
        quote::quote!{
            impl error_occurence_lib::ToStdStringString for #ident {
                fn to_std_string_string(&self) -> std::string::String {
                    format!("{self:#?}")
                }
            }
        }
    };
    let impl_sqlx_type_sqlx_postgres_for_ident_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
        &ident,
        &field_type
    );
    let impl_sqlx_type_sqlx_postgres_for_std_option_option_ident_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
        &std_option_option_ident_upper_camel_case,
        &std_option_option_field_type_token_stream
    );
    let impl_sqlx_decode_sqlx_postgres_for_ident_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
        &ident,
        &field_type
    );
    let impl_sqlx_decode_sqlx_postgres_for_std_option_option_ident_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
        &std_option_option_ident_upper_camel_case,
        &quote::quote! {std::option::Option<#ident>}
    );
    let self_zero_token_stream = {
        let self_snake_case = naming::SelfSnakeCase;
        quote::quote!{#self_snake_case.0}
    };
    let query_snake_case = naming::QuerySnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let try_generate_bind_increments_token_stream = {
        let increment_snake_case = naming::IncrementSnakeCase;
        let acc_snake_case = naming::AccSnakeCase;
        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("${{{increment_snake_case}}}"));
        quote::quote! {
            let mut #acc_snake_case = std::string::String::default();
            match #increment_snake_case.checked_add(1) {
                Some(#value_snake_case) => {
                    *#increment_snake_case = #value_snake_case;
                    #acc_snake_case.push_str(&format!(#format_handle_token_stream));
                }
                None => {
                    return Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
            Ok(#acc_snake_case)
        }
    };
    let impl_crate_bind_query_for_ident_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
        &ident,
        &try_generate_bind_increments_token_stream,
        &quote::quote! {
            #query_snake_case = #query_snake_case.bind(#self_zero_token_stream);
            #query_snake_case
        }
    );
    let impl_crate_bind_query_for_std_option_option_ident_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
        &std_option_option_ident_upper_camel_case,
        &try_generate_bind_increments_token_stream,
        &quote::quote! {
            #query_snake_case = #query_snake_case.bind(match #self_zero_token_stream {
                Some(#value_snake_case) => Some(#value_snake_case.0),
                None => None
            });
            #query_snake_case
        }
    );
    let pub_crate_struct_std_option_option_ident_token_stream = generate_pub_struct_tokens_token_stream(
        Visibility::PubCrate,
        &std_option_option_ident_upper_camel_case,
        &quote::quote!{(pub std::option::Option<#ident>);},
        false,
        true,
    );
    let (
        impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream,
        impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_ident_token_stream
    ) = {
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
            let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
            quote::quote! {crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case}
        };
        let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
        (
            generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &ident,
                &quote::quote!{(::core::default::Default::default())},
            ),
            generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &std_option_option_ident_upper_camel_case,
                &quote::quote!{(
                    Some(
                        #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
                    )
                )},
            )
        )
    };
    let (
        impl_crate_create_table_query_part_for_ident_token_stream,
        impl_crate_create_table_query_part_for_std_option_option_ident_token_stream
    ) = {
        let generate_impl_crate_create_table_query_part_for_tokens_token_stream = |
            ident_token_stream: &dyn quote::ToTokens,
            is_not_null: std::primitive::bool,
        |{
            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!(
                "{{{value_snake_case}}}{}",
                if is_not_null {
                    " NOT NULL"
                }
                else {
                    ""
                }
            ));
            quote::quote!{
                impl #ident_token_stream {
                    pub fn create_table_query_part_handle(value: &dyn std::fmt::Display) -> impl std::fmt::Display {
                        format!(#format_handle_token_stream)
                    }
                }
            }
        };
        (
            generate_impl_crate_create_table_query_part_for_tokens_token_stream(
                &ident,
                true,
            ),
            generate_impl_crate_create_table_query_part_for_tokens_token_stream(
                &std_option_option_ident_upper_camel_case,
                false,
            )
        )
    };
    let impl_postgresql_crud_base_type_self_type_for_ident_token_stream = {
        let postgresql_base_type_self_traits_upper_camel_case = naming::PostgresqlBaseTypeSelfTraitsUpperCamelCase;
        quote::quote!{
            impl crate::postgresql_type::postgresql_base_type_trait:: #postgresql_base_type_self_traits_upper_camel_case<'_> for #ident {}
        }
    };
    let impl_postgresql_base_type_for_ident_token_stream = {
        let postgresql_base_type_upper_camel_case = naming::PostgresqlBaseTypeUpperCamelCase;
        let postgresql_base_type_self_upper_camel_case = naming::PostgresqlBaseTypeSelfUpperCamelCase;
        let postgresql_base_type_std_option_option_self_upper_camel_case = naming::PostgresqlBaseTypeStdOptionOptionSelfUpperCamelCase;
        quote::quote! {
            impl crate::postgresql_type::postgresql_base_type_trait:: #postgresql_base_type_upper_camel_case<'_> for #ident {
                type #postgresql_base_type_self_upper_camel_case = Self;
                type #postgresql_base_type_std_option_option_self_upper_camel_case = #std_option_option_ident_upper_camel_case;
            }
        }
    };
    let generated = quote::quote! {
        #impl_error_occurence_lib_to_std_string_string_for_ident_token_stream
        #impl_sqlx_type_sqlx_postgres_for_ident_token_stream
        #impl_sqlx_decode_sqlx_postgres_for_ident_token_stream
        #impl_crate_bind_query_for_ident_token_stream
        #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream
        #impl_crate_create_table_query_part_for_ident_token_stream

        #pub_crate_struct_std_option_option_ident_token_stream
        #impl_sqlx_type_sqlx_postgres_for_std_option_option_ident_token_stream
        #impl_sqlx_decode_sqlx_postgres_for_std_option_option_ident_token_stream
        #impl_crate_bind_query_for_std_option_option_ident_token_stream
        #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_ident_token_stream
        #impl_crate_create_table_query_part_for_std_option_option_ident_token_stream
        
        #impl_postgresql_crud_base_type_self_type_for_ident_token_stream
        #impl_postgresql_base_type_for_ident_token_stream
    };
    // if ident == "" {
    //     println!("{generated}");
    // }
    generated.into()
}

#[proc_macro_derive(PostgresqlBaseTypePrimaryKeyTokens)]
pub fn postgresql_base_type_primary_key_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let impl_sqlx_encode_sqlx_postgres_for_ident_token_stream = generate_impl_sqlx_encode_sqlx_postgres_for_tokens_token_stream(&ident);
    let impl_sqlx_postgres_pg_has_array_type_for_ident_token_stream = quote::quote!{
        impl sqlx::postgres::PgHasArrayType for #ident {
            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                <#field_type as sqlx::postgres::PgHasArrayType>::array_type_info()
            }
        }
    };
    let impl_postgresql_crud_base_type_primary_key_for_ident_token_stream = {
        let postgresql_base_type_primary_key_upper_camel_case = naming::PostgresqlBaseTypePrimaryKeyUpperCamelCase;
        let postgresql_base_type_self_upper_camel_case = naming::PostgresqlBaseTypeSelfUpperCamelCase;
        quote::quote! {
            impl crate::postgresql_type::postgresql_base_type_trait:: #postgresql_base_type_primary_key_upper_camel_case<'_> for #ident {
                type #postgresql_base_type_self_upper_camel_case = Self;
            }
        }
    };
    let generated = quote::quote! {
        #impl_sqlx_encode_sqlx_postgres_for_ident_token_stream
        #impl_sqlx_postgres_pg_has_array_type_for_ident_token_stream
        #impl_postgresql_crud_base_type_primary_key_for_ident_token_stream
    };
    // if ident == "" {
    //     println!("{generated}");
    // }
    generated.into()
}

#[proc_macro_derive(PostgresqlTypeTokens)]
pub fn postgresql_type_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let postgresql_type_ident_column_upper_camel_case = naming::parameter::PostgresqlTypeSelfColumnUpperCamelCase::from_tokens(&ident);

    let try_generate_bind_increments_snake_case = naming::TryGenerateBindIncrementsSnakeCase;
    let bind_value_to_query_snake_case = naming::BindValueToQuerySnakeCase;
    let crate_bind_query_token_stream = quote::quote!{crate::BindQuerySecond::};

    let crate_bind_query_try_generate_bind_increments_token_stream = quote::quote!{#crate_bind_query_token_stream #try_generate_bind_increments_snake_case};
    let crate_bind_query_bind_value_to_query_token_stream = quote::quote!{#crate_bind_query_token_stream #bind_value_to_query_snake_case};

    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let self_snake_case = naming::SelfSnakeCase;
    let self_dot_zero_token_stream = quote::quote!{#self_snake_case.0};
    let crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream = quote::quote!{#crate_bind_query_try_generate_bind_increments_token_stream(&#self_dot_zero_token_stream, #increment_snake_case)};
    let crate_bind_query_bind_value_to_query_self_zero_query_token_stream = quote::quote!{#crate_bind_query_bind_value_to_query_token_stream(#self_dot_zero_token_stream, #query_snake_case)};

    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
        let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
        let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
        quote::quote!{
            crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case
        }
    };
    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
    let impl_std_fmt_display_for_tokens_self_zero_content_token_stream = quote::quote!{"{:?}", #self_dot_zero_token_stream};
    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
    };
    let braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        (#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
    };
    let self_token_stream = {
        let impl_std_fmt_display_for_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(
            &ident,
            &impl_std_fmt_display_for_tokens_self_zero_content_token_stream
        );
        let impl_error_occurence_lib_to_std_string_string_for_ident_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(&ident);
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &ident,
            &braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
        );
        //todo maybe not need it, maybe refactor later
        let impl_crate_bind_query_for_ident_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
            &ident,
            &crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream,
            &crate_bind_query_bind_value_to_query_self_zero_query_token_stream,
        );
        let impl_ident_create_table_query_part_handle_token_stream = {
            quote::quote!{
                impl #ident {
                    pub fn create_table_query_part_handle(value: &dyn std::fmt::Display) -> impl std::fmt::Display {
                        #field_type::create_table_query_part_handle(value)
                    }
                }
            }
        };
        quote::quote!{
            #impl_std_fmt_display_for_ident_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_ident_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream
            #impl_crate_bind_query_for_ident_token_stream
            #impl_ident_create_table_query_part_handle_token_stream
        }
    };
    let pub_snake_case = naming::PubSnakeCase;
    let postgresql_type_ident_column_token_stream = {
        let pub_struct_postgresql_type_ident_column_token_stream = generate_pub_struct_tokens_token_stream(
            Visibility::Pub,
            &postgresql_type_ident_column_upper_camel_case,
            &quote::quote!{;},
            true,
            true,
        );
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_column_token_stream = {
            quote::quote! {
                impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_type_ident_column_upper_camel_case {
                    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        ::core::default::Default::default()
                    }
                }
            }
        };
        quote::quote! {
            #pub_struct_postgresql_type_ident_column_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_column_token_stream
        }
    };
    let postgresql_type_self_column_upper_camel_case = naming::PostgresqlTypeSelfColumnUpperCamelCase;
    let postgresql_type_self_column_query_part_token_stream = {
        let postgresql_type_self_column_snake_case = naming::PostgresqlTypeSelfColumnSnakeCase;
        quote::quote!{
            fn postgresql_type_self_column_query_part(
                #postgresql_type_self_column_snake_case: &Self::#postgresql_type_self_column_upper_camel_case,
                column: &std::primitive::str,
            ) -> std::string::String {
                column.to_string()
            }
        }
    };
    let value_snake_case = naming::ValueSnakeCase;
    let field_type_struct_content_token_stream = quote::quote!{(#field_type);};
    let postgresql_type_ident_to_create_upper_camel_case = naming::parameter::PostgresqlTypeSelfToCreateUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_to_create_token_stream = {
        let postgresql_type_ident_to_create_token_stream = generate_pub_struct_tokens_token_stream(
            Visibility::Pub,
            &postgresql_type_ident_to_create_upper_camel_case,
            &field_type_struct_content_token_stream,
            false,
            true,
        );
        let impl_crate_bind_query_for_postgresql_type_ident_to_create_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
            &postgresql_type_ident_to_create_upper_camel_case,
            &crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream,
            &crate_bind_query_bind_value_to_query_self_zero_query_token_stream,
        );
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_to_create_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &postgresql_type_ident_to_create_upper_camel_case,
            &braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
        );
        let impl_postgresql_type_self_to_create_traits_for_postgresql_type_ident_to_create_token_stream = {
            let postgresql_type_self_to_create_traits_upper_camel_case = naming::PostgresqlTypeSelfToCreateTraitsUpperCamelCase;
            quote::quote!{
                impl crate::postgresql_type::postgresql_type_trait:: #postgresql_type_self_to_create_traits_upper_camel_case<'_> for #postgresql_type_ident_to_create_upper_camel_case {}
            }
        };
        quote::quote! {
            #postgresql_type_ident_to_create_token_stream
            #impl_crate_bind_query_for_postgresql_type_ident_to_create_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_to_create_token_stream
            #impl_postgresql_type_self_to_create_traits_for_postgresql_type_ident_to_create_token_stream
        }
    };
    let postgresql_type_ident_to_read_upper_camel_case = naming::parameter::PostgresqlTypeSelfToReadUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_to_read_token_stream = {
        let postgresql_type_ident_to_read_token_stream = generate_pub_struct_tokens_token_stream(
            Visibility::Pub,
            &postgresql_type_ident_to_read_upper_camel_case,
            &field_type_struct_content_token_stream,
            false,
            true,
        );
        let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_to_read_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_to_read_upper_camel_case,
            &field_type
        );
        let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_read_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_to_read_upper_camel_case,
            &field_type
        );
        let impl_postgresql_type_self_to_read_traits_for_postgresql_type_ident_to_read_token_stream = {
            let postgresql_type_self_to_read_traits_upper_camel_case = naming::PostgresqlTypeSelfToReadTraitsUpperCamelCase;
            quote::quote!{
                impl crate::postgresql_type::postgresql_type_trait:: #postgresql_type_self_to_read_traits_upper_camel_case<'_> for #postgresql_type_ident_to_read_upper_camel_case {}
            }
        };
        quote::quote! {
            #postgresql_type_ident_to_read_token_stream
            #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_to_read_token_stream
            #impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_read_token_stream
            #impl_postgresql_type_self_to_read_traits_for_postgresql_type_ident_to_read_token_stream
        }
    };
    let postgresql_type_ident_to_update_upper_camel_case = naming::parameter::PostgresqlTypeSelfToUpdateUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_to_update_token_stream = {
        let postgresql_type_ident_to_update_token_stream = generate_pub_struct_tokens_token_stream(
            Visibility::Pub,
            &postgresql_type_ident_to_update_upper_camel_case,
            &field_type_struct_content_token_stream,
            false,
            true,
        );
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_to_update_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &postgresql_type_ident_to_update_upper_camel_case,
            &braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
        );
        let impl_postgresql_type_self_to_update_traits_for_postgresql_type_ident_to_update_token_stream = {
            let postgresql_type_self_to_update_traits_upper_camel_case = naming::PostgresqlTypeSelfToUpdateTraitsUpperCamelCase;
            quote::quote!{
                impl crate::postgresql_type::postgresql_type_trait:: #postgresql_type_self_to_update_traits_upper_camel_case<'_> for #postgresql_type_ident_to_update_upper_camel_case {}
            }
        };
        quote::quote! {
            #postgresql_type_ident_to_update_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_to_update_token_stream
            #impl_postgresql_type_self_to_update_traits_for_postgresql_type_ident_to_update_token_stream
        }
    };

    let postgresql_type_ident_to_update_query_part_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfToUpdateQueryPartErrorNamedUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_to_update_query_part_error_named_token_stream = {
        quote::quote!{
            #[derive(
                Debug,
                // Default,
                Clone,
                PartialEq,
                serde::Serialize,
                serde::Deserialize,
            )]
            pub enum #postgresql_type_ident_to_update_query_part_error_named_upper_camel_case {
                Todo//todo
            }
        }
    };
    let postgresql_type_self_to_update_upper_camel_case = naming::PostgresqlTypeSelfToUpdateUpperCamelCase;
    let postgresql_type_self_to_update_query_part_error_named_upper_camel_case = naming::PostgresqlTypeSelfToUpdateQueryPartErrorNamedUpperCamelCase;
    let postgresql_type_self_to_update_query_part_token_stream = {
        let postgresql_type_self_to_update_query_part_snake_case = naming::PostgresqlTypeSelfToUpdateQueryPartSnakeCase;
        let postgresql_type_self_to_update_snake_case = naming::PostgresqlTypeSelfToUpdateSnakeCase;
        //todo remove jsonb_ prefix (coz it can be json, jsonb, json not null, jsonb not null)
        let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
        let jsonb_set_target_snake_case = naming::JsonbSetTargetSnakeCase;
        let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        quote::quote!{
            fn #postgresql_type_self_to_update_query_part_snake_case(
                //few parameters usefull only with json types. maybe refactor it later
                #postgresql_type_self_to_update_snake_case: &Self::#postgresql_type_self_to_update_upper_camel_case,
                #jsonb_set_accumulator_snake_case: &std::primitive::str,
                #jsonb_set_target_snake_case: &std::primitive::str,
                #jsonb_set_path_snake_case: &std::primitive::str,
                #increment_snake_case: &mut std::primitive::u64
            ) -> Result<std::string::String, Self::#postgresql_type_self_to_update_query_part_error_named_upper_camel_case> {
                //todo remove .unwrap()
                Ok(#crate_bind_query_try_generate_bind_increments_token_stream(&#postgresql_type_self_to_update_snake_case.0, #increment_snake_case).unwrap())
            }
        }
    };
    let postgresql_type_self_to_update_bind_query_part_token_stream = {
        let postgresql_type_self_to_update_bind_query_part = naming::PostgresqlTypeSelfToUpdateBindQueryPartSnakeCase;
        let postgresql_type_self_to_update_snake_case = naming::PostgresqlTypeSelfToUpdateSnakeCase;
        quote::quote!{
            fn #postgresql_type_self_to_update_bind_query_part<'a>(
                #postgresql_type_self_to_update_snake_case: Self::#postgresql_type_self_to_update_upper_camel_case,
                query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #crate_bind_query_bind_value_to_query_token_stream(#postgresql_type_self_to_update_snake_case.0, #query_snake_case)
            }
        }
    };

    let postgresql_type_ident_to_delete_upper_camel_case = naming::parameter::PostgresqlTypeSelfToDeleteUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_to_delete_token_stream = {
        let postgresql_type_ident_to_delete_token_stream = generate_pub_struct_tokens_token_stream(
            Visibility::Pub,
            &postgresql_type_ident_to_delete_upper_camel_case,
            &field_type_struct_content_token_stream,
            false,
            true,
        );
        let impl_crate_bind_query_for_postgresql_type_ident_to_delete_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
            &postgresql_type_ident_to_delete_upper_camel_case,
            &crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream,
            &crate_bind_query_bind_value_to_query_self_zero_query_token_stream,
        );
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_to_delete_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &postgresql_type_ident_to_delete_upper_camel_case,
            &braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
        );
        let impl_std_fmt_display_for_postgresql_type_ident_to_delete_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(
            &postgresql_type_ident_to_delete_upper_camel_case,
            &impl_std_fmt_display_for_tokens_self_zero_content_token_stream
        );
        let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_to_delete_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(&postgresql_type_ident_to_delete_upper_camel_case);
        let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_to_delete_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_to_delete_upper_camel_case,
            &field_type
        );
        let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_delete_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_to_delete_upper_camel_case,
            &field_type
        );
        quote::quote!{
            #postgresql_type_ident_to_delete_token_stream
            #impl_std_fmt_display_for_postgresql_type_ident_to_delete_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_to_delete_token_stream
            #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_to_delete_token_stream
            #impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_delete_token_stream
            #impl_crate_bind_query_for_postgresql_type_ident_to_delete_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_to_delete_token_stream
        }
    };
    let logical_operator_upper_camel_case = naming::LogicalOperatorUpperCamelCase;
    let postgresql_type_ident_where_element_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_token_stream = {
        let logical_operator_snake_case = naming::LogicalOperatorSnakeCase;
        let postgresql_type_ident_where_token_stream = generate_pub_struct_tokens_token_stream(
            Visibility::Pub,
            &postgresql_type_ident_where_upper_camel_case,
            &quote::quote!{{
                #logical_operator_snake_case: crate::#logical_operator_upper_camel_case,
                #value_snake_case: std::vec::Vec<#postgresql_type_ident_where_element_upper_camel_case>
            }},
            false,
            false,
        );
        let postgresql_type_ident_where_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
        let postgresql_type_ident_where_try_new_error_named_token_stream = {
            quote::quote!{
                #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #postgresql_type_ident_where_try_new_error_named_upper_camel_case {
                    IsEmpty {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    NotUnique {
                        #[eo_to_std_string_string_serialize_deserialize]
                        value: #postgresql_type_ident_where_element_upper_camel_case,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                }
            }
        };
        let impl_postgresql_type_std_primitive_i32_as_postgresql_serial_not_null_where_fn_try_new_token_stream = {
            quote::quote!{
                impl #postgresql_type_ident_where_upper_camel_case {
                    fn try_new(
                        logical_operator: crate::LogicalOperator,
                        value: std::vec::Vec<#postgresql_type_ident_where_element_upper_camel_case>,
                    ) -> Result<Self, #postgresql_type_ident_where_try_new_error_named_upper_camel_case> {
                        if value.is_empty() {
                            return Err(#postgresql_type_ident_where_try_new_error_named_upper_camel_case::IsEmpty {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        {
                            //todo maybe not correct?
                            let mut acc = vec![];
                            for element in &value {
                                if !acc.contains(&element) {
                                    acc.push(element);
                                } else {
                                    return Err(#postgresql_type_ident_where_try_new_error_named_upper_camel_case::NotUnique {
                                        value: element.clone(),
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        Ok(Self {
                            logical_operator,
                            value,
                        })
                    }
                }
            }
        };
        let impl_serde_deserialize_for_postgresql_type_std_primitive_i32_as_postgresql_serial_not_null_where_token_stream = {
            let struct_postgresql_type_ident_where_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!(
                    "struct {postgresql_type_ident_where_upper_camel_case}"
                )
            );
            let struct_postgresql_type_ident_where_with_2_elements_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!(
                    "struct {postgresql_type_ident_where_upper_camel_case} with 2 elements"
                )
            );
            let postgresql_type_ident_where_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &postgresql_type_ident_where_upper_camel_case
            );
            quote::quote!{
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for #postgresql_type_ident_where_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __field1,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        1u64 => _serde::__private::Ok(__Field::__field1),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                                        "value" => _serde::__private::Ok(__Field::__field1),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                                        b"value" => _serde::__private::Ok(__Field::__field1),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private::PhantomData<
                                    #postgresql_type_ident_where_upper_camel_case,
                                >,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #postgresql_type_ident_where_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        #struct_postgresql_type_ident_where_double_quotes_token_stream,
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        crate::LogicalOperator,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#struct_postgresql_type_ident_where_with_2_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                        std::vec::Vec<
                                            #postgresql_type_ident_where_element_upper_camel_case,
                                        >,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &#struct_postgresql_type_ident_where_with_2_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    match #postgresql_type_ident_where_upper_camel_case::try_new(__field0, __field1) {
                                        Ok(value) => _serde::__private::Ok(value),
                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                    }
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<
                                        crate::LogicalOperator,
                                    > = _serde::__private::None;
                                    let mut __field1: _serde::__private::Option<
                                        std::vec::Vec<
                                            #postgresql_type_ident_where_element_upper_camel_case,
                                        >,
                                    > = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "logical_operator",
                                                        ),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        crate::LogicalOperator,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::__private::Option::is_some(&__field1) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                                    );
                                                }
                                                __field1 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        std::vec::Vec<
                                                            #postgresql_type_ident_where_element_upper_camel_case,
                                                        >,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            _ => {
                                                let _ = _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(&mut __map)?;
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("logical_operator")?
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::__private::Some(__field1) => __field1,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("value")?
                                        }
                                    };
                                    match #postgresql_type_ident_where_upper_camel_case::try_new(__field0, __field1) {
                                        Ok(value) => _serde::__private::Ok(value),
                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                    }
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                #postgresql_type_ident_where_double_quotes_token_stream,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<
                                        #postgresql_type_ident_where_upper_camel_case,
                                    >,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                };
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &postgresql_type_ident_where_upper_camel_case,
            &quote::quote!{{
                #logical_operator_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                #value_snake_case: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
            }}
        );
        quote::quote!{
            #postgresql_type_ident_where_token_stream
            #postgresql_type_ident_where_try_new_error_named_token_stream
            #impl_postgresql_type_std_primitive_i32_as_postgresql_serial_not_null_where_fn_try_new_token_stream
            #impl_serde_deserialize_for_postgresql_type_std_primitive_i32_as_postgresql_serial_not_null_where_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_token_stream
        }
    };
    let postgresql_type_self_where_element_upper_camel_case = naming::PostgresqlTypeSelfWhereElementUpperCamelCase;
    let postgresql_type_self_where_upper_camel_case = naming::PostgresqlTypeSelfWhereUpperCamelCase;
    let postgresql_type_self_where_snake_case = naming::PostgresqlTypeSelfWhereSnakeCase;
    let postgresql_type_self_where_try_generate_bind_increments_token_stream = {
        let postgresql_type_self_where_try_generate_bind_increments_snake_case = naming::PostgresqlTypeSelfWhereTryGenerateBindIncrementsSnakeCase;
        quote::quote!{
            fn #postgresql_type_self_where_try_generate_bind_increments_snake_case(
                #postgresql_type_self_where_snake_case: &Self::#postgresql_type_self_where_upper_camel_case,
                increment: &mut std::primitive::u64,
                column: &dyn std::fmt::Display,
                is_need_to_add_logical_operator: std::primitive::bool,
            ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                let mut acc = std::string::String::default();
                let mut is_need_to_add_logical_operator_inner_handle = false;
                for element in &#postgresql_type_self_where_snake_case.value {
                    match crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
                        Ok(value) => {
                            acc.push_str(&format!("{value} "));
                            is_need_to_add_logical_operator_inner_handle = true;
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                let _ = acc.pop();
                Ok(format!("{}({acc})", &#postgresql_type_self_where_snake_case.logical_operator.to_query_part(is_need_to_add_logical_operator)))
            }
        }
    };
    let postgresql_type_self_where_bind_value_to_query_token_stream = {
        let postgresql_type_self_where_bind_value_to_query_snake_case = naming::PostgresqlTypeSelfWhereBindValueToQuerySnakeCase;
        quote::quote!{
            fn #postgresql_type_self_where_bind_value_to_query_snake_case<'a>(
                #postgresql_type_self_where_snake_case: Self::#postgresql_type_self_where_upper_camel_case,
                mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                for element in postgresql_type_self_where.value {
                    query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
                }
                query
            }
        }
    };
    let impl_postgresql_type_for_ident_token_stream = {
        let postgresql_type_upper_camel_case = naming::PostgresqlTypeUpperCamelCase;
        let self_upper_camel_case = naming::SelfUpperCamelCase;
        let postgresql_type_self_upper_camel_case = naming::PostgresqlTypeSelfUpperCamelCase;
        let postgresql_type_self_to_create_upper_camel_case = naming::PostgresqlTypeSelfToCreateUpperCamelCase;
        let postgresql_type_self_to_read_upper_camel_case = naming::PostgresqlTypeSelfToReadUpperCamelCase;
        quote::quote!{
            impl crate::postgresql_type::postgresql_type_trait:: #postgresql_type_upper_camel_case<'_> for #ident {
                type #postgresql_type_self_upper_camel_case = #self_upper_camel_case;
                type #postgresql_type_self_column_upper_camel_case = #postgresql_type_ident_column_upper_camel_case;
                #postgresql_type_self_column_query_part_token_stream
                type #postgresql_type_self_to_create_upper_camel_case = #postgresql_type_ident_to_create_upper_camel_case;
                type #postgresql_type_self_to_read_upper_camel_case = #postgresql_type_ident_to_read_upper_camel_case;
                type #postgresql_type_self_to_update_upper_camel_case = #postgresql_type_ident_to_update_upper_camel_case;
                type #postgresql_type_self_to_update_query_part_error_named_upper_camel_case = #postgresql_type_ident_to_update_query_part_error_named_upper_camel_case;
                #postgresql_type_self_to_update_query_part_token_stream
                #postgresql_type_self_to_update_bind_query_part_token_stream
                type #postgresql_type_self_where_element_upper_camel_case = #postgresql_type_ident_where_element_upper_camel_case;
                type #postgresql_type_self_where_upper_camel_case = #postgresql_type_ident_where_upper_camel_case;
                #postgresql_type_self_where_try_generate_bind_increments_token_stream
                #postgresql_type_self_where_bind_value_to_query_token_stream
            }
        }
    };
    //todo some implementations only for primary key types. maybe write 2 traits: 1 for typical type and 1 for primary key
    let generated = quote::quote! {
        #self_token_stream

        #postgresql_type_ident_column_token_stream

        #postgresql_type_ident_to_create_token_stream

        #postgresql_type_ident_to_read_token_stream

        #postgresql_type_ident_to_update_token_stream

        #postgresql_type_ident_to_update_query_part_error_named_token_stream

        #postgresql_type_ident_to_delete_token_stream

        #postgresql_type_ident_where_token_stream

        #impl_postgresql_type_for_ident_token_stream
    };
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlTypeTokens",
    //         &generated,
    //     );
    // }
    generated.into()
}

#[proc_macro_derive(PostgresqlTypePrimaryKeyTokens)]
pub fn postgresql_type_primary_key_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let try_generate_bind_increments_snake_case = naming::TryGenerateBindIncrementsSnakeCase;
    let bind_value_to_query_snake_case = naming::BindValueToQuerySnakeCase;
    let crate_bind_query_token_stream = quote::quote!{crate::BindQuerySecond::};

    let crate_bind_query_try_generate_bind_increments_token_stream = quote::quote!{#crate_bind_query_token_stream #try_generate_bind_increments_snake_case};
    let crate_bind_query_bind_value_to_query_token_stream = quote::quote!{#crate_bind_query_token_stream #bind_value_to_query_snake_case};

    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let self_snake_case = naming::SelfSnakeCase;
    let self_dot_zero_token_stream = quote::quote!{#self_snake_case.0};
    let crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream = quote::quote!{#crate_bind_query_try_generate_bind_increments_token_stream(&#self_dot_zero_token_stream, #increment_snake_case)};
    let crate_bind_query_bind_value_to_query_self_zero_query_token_stream = quote::quote!{#crate_bind_query_bind_value_to_query_token_stream(#self_dot_zero_token_stream, #query_snake_case)};

    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
        let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
        let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
        quote::quote!{
            crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case
        }
    };
    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
    let impl_std_fmt_display_for_tokens_self_zero_content_token_stream = quote::quote!{"{:?}", #self_dot_zero_token_stream};
    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
    };
    let braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        (#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
    };
    let value_snake_case = naming::ValueSnakeCase;
    let field_type_struct_content_token_stream = quote::quote!{(#field_type);};
    let postgresql_type_ident_to_create_upper_camel_case = naming::parameter::PostgresqlTypeSelfToCreateUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_to_create_token_stream = {
        let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_to_create_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_to_create_upper_camel_case,
            &field_type
        );
        let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_create_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_to_create_upper_camel_case,
            &field_type
        );
        quote::quote! {
            #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_to_create_token_stream
            #impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_create_token_stream
        }
    };
    let postgresql_type_ident_to_read_upper_camel_case = naming::parameter::PostgresqlTypeSelfToReadUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_to_read_token_stream = {
        let impl_crate_bind_query_for_postgresql_type_ident_to_read_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
            &postgresql_type_ident_to_read_upper_camel_case,
            &crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream,
            &crate_bind_query_bind_value_to_query_self_zero_query_token_stream,
        );
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_to_read_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &postgresql_type_ident_to_read_upper_camel_case,
            &braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
        );
        quote::quote! {
            #impl_crate_bind_query_for_postgresql_type_ident_to_read_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_to_read_token_stream
        }
    };
    let postgresql_type_ident_to_update_upper_camel_case = naming::parameter::PostgresqlTypeSelfToUpdateUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_to_update_token_stream = {
        let impl_std_fmt_display_for_postgresql_type_ident_to_update_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(
            &postgresql_type_ident_to_update_upper_camel_case,
            &impl_std_fmt_display_for_tokens_self_zero_content_token_stream
        );
        let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_to_update_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(&postgresql_type_ident_to_update_upper_camel_case);
        let impl_sqlx_encode_sqlx_postgres_for_postgresql_type_ident_to_update_token_stream = generate_impl_sqlx_encode_sqlx_postgres_for_tokens_token_stream(&postgresql_type_ident_to_update_upper_camel_case);
        let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_to_update_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_to_update_upper_camel_case,
            &field_type
        );
        let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_update_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_to_update_upper_camel_case,
            &field_type
        );
        quote::quote! {
            #impl_std_fmt_display_for_postgresql_type_ident_to_update_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_to_update_token_stream
            #impl_sqlx_encode_sqlx_postgres_for_postgresql_type_ident_to_update_token_stream
            #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_to_update_token_stream
            #impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_update_token_stream
        }
    };
    let postgresql_type_ident_to_delete_upper_camel_case = naming::parameter::SelfToDeleteUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_to_delete_token_stream = {
        let postgresql_type_ident_to_delete_token_stream = generate_pub_struct_tokens_token_stream(
            Visibility::Pub,
            &postgresql_type_ident_to_delete_upper_camel_case,
            &field_type_struct_content_token_stream,
            false,
            true,
        );
        let impl_crate_bind_query_for_postgresql_type_ident_to_delete_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
            &postgresql_type_ident_to_delete_upper_camel_case,
            &crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream,
            &crate_bind_query_bind_value_to_query_self_zero_query_token_stream,
        );
        let impl_std_fmt_display_for_postgresql_type_ident_to_delete_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(
            &postgresql_type_ident_to_delete_upper_camel_case,
            &impl_std_fmt_display_for_tokens_self_zero_content_token_stream
        );
        let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_to_delete_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(&postgresql_type_ident_to_delete_upper_camel_case);
        let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_to_delete_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_to_delete_upper_camel_case,
            &field_type
        );
        let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_delete_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_to_delete_upper_camel_case,
            &field_type
        );
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_to_delete_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &postgresql_type_ident_to_delete_upper_camel_case,
            &braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
        );
        quote::quote!{
            #postgresql_type_ident_to_delete_token_stream
            #impl_crate_bind_query_for_postgresql_type_ident_to_delete_token_stream
            #impl_std_fmt_display_for_postgresql_type_ident_to_delete_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_to_delete_token_stream
            #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_to_delete_token_stream
            #impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_delete_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_to_delete_token_stream
        }
    };
    let impl_postgresql_crud_base_wrap_type_primary_key_for_ident_token_stream = {
        quote::quote!{
            impl crate::postgresql_type::postgresql_type_trait:: PostgresqlTypePrimaryKey<'_> for #ident {
                type PostgresqlTypeSelfToCreate = #postgresql_type_ident_to_create_upper_camel_case;
                type PostgresqlTypeSelfToRead = #postgresql_type_ident_to_read_upper_camel_case;
                type PostgresqlTypeSelfToUpdate = #postgresql_type_ident_to_update_upper_camel_case;
                type PostgresqlTypeSelfToDelete = #postgresql_type_ident_to_delete_upper_camel_case;
            }
        }
    };
    //todo some implementations only for primary key types. maybe write 2 traits: 1 for typical type and 1 for primary key
    let generated = quote::quote! {
        #postgresql_type_ident_to_create_token_stream

        #postgresql_type_ident_to_read_token_stream

        #postgresql_type_ident_to_update_token_stream

        #postgresql_type_ident_to_delete_token_stream

        #impl_postgresql_crud_base_wrap_type_primary_key_for_ident_token_stream
    };
    // if ident == "" {
    //     println!("{generated}");
    //     println!("----------------------");
    // }
    generated.into()
}

#[proc_macro_derive(PostgresqlTypeTokensNotNullWhereElementInt)]
pub fn postgresql_type_tokens_not_null_where_element_int(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    };
    let postgresql_type_ident_where_element_equal_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementEqualUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_element_equal_token_stream = {
        let postgresql_type_ident_where_element_equal_token_stream = {
            quote::quote!{
                #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
                pub struct #postgresql_type_ident_where_element_equal_upper_camel_case {
                    pub logical_operator: crate::LogicalOperator,
                    pub value: #field_type
                }
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_equal_token_stream = {
            quote::quote!{
                impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_type_ident_where_element_equal_upper_camel_case {
                    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        Self {
                            logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                            value: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                        }
                    }
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_equal_token_stream = {
            quote::quote!{
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_ident_where_element_equal_upper_camel_case {
                    fn postgresql_type_self_where_try_generate_bind_increments(
                        &self,
                        increment: &mut std::primitive::u64,
                        column: &dyn std::fmt::Display,
                        is_need_to_add_logical_operator: std::primitive::bool,
                    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                        match crate::BindQuerySecond::try_generate_bind_increments(&self.value, increment) {
                            Ok(value) => Ok(format!("{}({column} = {value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator))),
                            Err(error) => Err(error),//todo another checked add? 
                        }
                    }
                    fn postgresql_type_self_where_bind_value_to_query<'a>(
                        self,
                        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        crate::BindQuerySecond::bind_value_to_query(self.value, query)
                    }
                }
            }
        };
        quote::quote!{
            #postgresql_type_ident_where_element_equal_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_equal_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_equal_token_stream
        }
    };
    let postgresql_type_ident_where_element_greater_than_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementGreaterThanUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_element_greater_than_token_stream = {
        let postgresql_type_ident_where_element_greater_than_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
                pub struct #postgresql_type_ident_where_element_greater_than_upper_camel_case {
                    pub logical_operator: crate::LogicalOperator,
                    pub value: #field_type
                }
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_greater_than_token_stream = {
            quote::quote! {
                impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_type_ident_where_element_greater_than_upper_camel_case {
                    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        Self {
                            logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                            value: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                        }
                    }
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_greater_than_token_stream = {
            quote::quote! {
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_ident_where_element_greater_than_upper_camel_case {
                    fn postgresql_type_self_where_try_generate_bind_increments(
                        &self,
                        increment: &mut std::primitive::u64,
                        column: &dyn std::fmt::Display,
                        is_need_to_add_logical_operator: std::primitive::bool,
                    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                        match crate::BindQuerySecond::try_generate_bind_increments(&self.value, increment) {
                            Ok(value) => {
                                Ok(format!("{}({column} > {value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                            }
                            Err(error) => Err(error),//todo another checked add? 
                        }
                    }
                    fn postgresql_type_self_where_bind_value_to_query<'a>(
                        self,
                        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        crate::BindQuerySecond::bind_value_to_query(self.value, query)
                    }
                }
            }
        };
        quote::quote! {
            #postgresql_type_ident_where_element_greater_than_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_greater_than_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_greater_than_token_stream
        }
    };
    let postgresql_type_ident_where_element_between_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementBetweenUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_element_between_token_stream = {
        let postgresql_type_ident_where_element_between_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde::Serialize)]
                pub struct #postgresql_type_ident_where_element_between_upper_camel_case {
                    logical_operator: crate::LogicalOperator,
                    start: #field_type,
                    end: #field_type
                }
            }
        };
        let postgresql_type_ident_where_element_between_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementBetweenTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
        let postgresql_type_ident_where_element_between_try_new_error_named_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #postgresql_type_ident_where_element_between_try_new_error_named_upper_camel_case {
                    StartMoreOrEqualToEnd {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                }
            }
        };
        let impl_postgresql_type_ident_where_element_between_try_new_token_stream = {
            quote::quote! {
                impl #postgresql_type_ident_where_element_between_upper_camel_case {
                    fn try_new(
                        logical_operator: crate::LogicalOperator,
                        start: #field_type,
                        end: #field_type
                    ) -> Result<Self, #postgresql_type_ident_where_element_between_try_new_error_named_upper_camel_case> {
                        if start.0 < end.0 {
                            Ok(Self {
                                logical_operator,
                                start,
                                end
                            })
                        }
                        else {
                            Err(#postgresql_type_ident_where_element_between_try_new_error_named_upper_camel_case::StartMoreOrEqualToEnd {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            })
                        }
                    }
                }
            }
        };
        let impl_serde_deserialize_for_postgresql_type_ident_where_element_between_token_stream = {
            let struct_postgresql_type_ident_where_element_between_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!(
                    "struct {postgresql_type_ident_where_element_between_upper_camel_case}"
                )
            );
            let struct_postgresql_type_ident_where_element_between_with_3_elements_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!(
                    "struct {postgresql_type_ident_where_element_between_upper_camel_case} with 3 elements"
                )
            );
            let postgresql_type_ident_where_element_between_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &postgresql_type_ident_where_element_between_upper_camel_case
            );
            quote::quote! {
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for #postgresql_type_ident_where_element_between_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        1u64 => _serde::__private::Ok(__Field::__field1),
                                        2u64 => _serde::__private::Ok(__Field::__field2),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                                        "start" => _serde::__private::Ok(__Field::__field1),
                                        "end" => _serde::__private::Ok(__Field::__field2),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                                        b"start" => _serde::__private::Ok(__Field::__field1),
                                        b"end" => _serde::__private::Ok(__Field::__field2),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private::PhantomData<
                                    #postgresql_type_ident_where_element_between_upper_camel_case,
                                >,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #postgresql_type_ident_where_element_between_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        #struct_postgresql_type_ident_where_element_between_double_quotes_token_stream,
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        crate::LogicalOperator,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#struct_postgresql_type_ident_where_element_between_with_3_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                        #field_type,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &#struct_postgresql_type_ident_where_element_between_with_3_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match _serde::de::SeqAccess::next_element::<
                                        #field_type,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &#struct_postgresql_type_ident_where_element_between_with_3_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    match #postgresql_type_ident_where_element_between_upper_camel_case::try_new(__field0, __field1, __field2) {
                                        Ok(value) => _serde::__private::Ok(value),
                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                    }
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<
                                        crate::LogicalOperator,
                                    > = _serde::__private::None;
                                    let mut __field1: _serde::__private::Option<
                                        #field_type,
                                    > = _serde::__private::None;
                                    let mut __field2: _serde::__private::Option<
                                        #field_type,
                                    > = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "logical_operator",
                                                        ),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        crate::LogicalOperator,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::__private::Option::is_some(&__field1) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                                    );
                                                }
                                                __field1 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        #field_type,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::__private::Option::is_some(&__field2) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                                    );
                                                }
                                                __field2 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        #field_type,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            _ => {
                                                let _ = _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(&mut __map)?;
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("logical_operator")?
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::__private::Some(__field1) => __field1,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("start")?
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::__private::Some(__field2) => __field2,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("end")?
                                        }
                                    };
                                    match #postgresql_type_ident_where_element_between_upper_camel_case::try_new(__field0, __field1, __field2) {
                                        Ok(value) => _serde::__private::Ok(value),
                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                    }
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &[
                                "logical_operator",
                                "start",
                                "end",
                            ];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                #postgresql_type_ident_where_element_between_double_quotes_token_stream,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<
                                        #postgresql_type_ident_where_element_between_upper_camel_case,
                                    >,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                };
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_between_token_stream = {
            quote::quote! {
                impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_type_ident_where_element_between_upper_camel_case {
                    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        Self {
                            logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                            start: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                            end: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                        }
                    }
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_between_token_stream = {
            quote::quote! {
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_ident_where_element_between_upper_camel_case {
                    fn postgresql_type_self_where_try_generate_bind_increments(
                        &self,
                        increment: &mut std::primitive::u64,
                        column: &dyn std::fmt::Display,
                        is_need_to_add_logical_operator: std::primitive::bool,
                    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                        match crate::BindQuerySecond::try_generate_bind_increments(&self.start, increment) {
                            Ok(first_value) => match crate::BindQuerySecond::try_generate_bind_increments(&self.end, increment) {
                                Ok(second_value) => {
                                    let between_snake_case = naming::BetweenSnakeCase;
                                    let and_snake_case = naming::AndSnakeCase;
                                    Ok(format!("{}({column} {between_snake_case} {first_value} {and_snake_case} {second_value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                                },
                                Err(error) => Err(error),
                            },
                            Err(error) => Err(error),//todo another checked add? 
                        }
                    }
                    fn postgresql_type_self_where_bind_value_to_query<'a>(
                        self,
                        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        query = crate::BindQuerySecond::bind_value_to_query(self.start, query);
                        query = crate::BindQuerySecond::bind_value_to_query(self.end, query);
                        query
                    }
                }
            }
        };
        quote::quote! {
            #postgresql_type_ident_where_element_between_token_stream
            #postgresql_type_ident_where_element_between_try_new_error_named_token_stream
            #impl_postgresql_type_ident_where_element_between_try_new_token_stream
            #impl_serde_deserialize_for_postgresql_type_ident_where_element_between_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_between_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_between_token_stream
        }
    };
    let postgresql_type_ident_where_element_in_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementInUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_element_in_token_stream = {
        let postgresql_type_ident_where_element_in_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde::Serialize)]
                pub struct #postgresql_type_ident_where_element_in_upper_camel_case {
                    logical_operator: crate::LogicalOperator,
                    value: std::vec::Vec<#field_type>
                }
            }
        };
        let postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementInTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
        let postgresql_type_ident_where_element_in_try_new_error_named_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case {
                    IsEmpty {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    NotUnique {
                        #[eo_to_std_string_string_serialize_deserialize]
                        value: #field_type,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                }
            }
        };
        let impl_postgresql_type_ident_where_element_in_try_new_token_stream = {
            quote::quote! {
                impl #postgresql_type_ident_where_element_in_upper_camel_case {
                    fn try_new(
                        logical_operator: crate::LogicalOperator,
                        value: std::vec::Vec<#field_type>
                    ) -> Result<Self, #postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case> {
                        if value.is_empty() {
                            return Err(#postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case::IsEmpty {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        {
                            let mut acc = vec![];
                            for element in &value {
                                if !acc.contains(&element) {
                                    acc.push(element);
                                } else {
                                    return Err(#postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case::NotUnique {
                                        value: element.clone(),
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        Ok(Self{
                            logical_operator,
                            value
                        })
                    }
                }
            }
        };
        let impl_serde_deserialize_for_postgresql_type_ident_where_element_in_token_stream = {
            let struct_postgresql_type_ident_where_element_in_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!(
                    "struct {postgresql_type_ident_where_element_in_upper_camel_case}"
                )
            );
            let struct_postgresql_type_ident_where_element_in_with_2_elements_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!(
                    "struct {postgresql_type_ident_where_element_in_upper_camel_case} with 2 elements"
                )
            );
            let postgresql_type_ident_where_element_in_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &postgresql_type_ident_where_element_in_upper_camel_case
            );
            quote::quote! {
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for #postgresql_type_ident_where_element_in_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __field1,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        1u64 => _serde::__private::Ok(__Field::__field1),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                                        "value" => _serde::__private::Ok(__Field::__field1),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                                        b"value" => _serde::__private::Ok(__Field::__field1),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private::PhantomData<
                                    #postgresql_type_ident_where_element_in_upper_camel_case,
                                >,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #postgresql_type_ident_where_element_in_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        #struct_postgresql_type_ident_where_element_in_double_quotes_token_stream,
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        crate::LogicalOperator,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#struct_postgresql_type_ident_where_element_in_with_2_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                        std::vec::Vec<
                                            #field_type,
                                        >,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &#struct_postgresql_type_ident_where_element_in_with_2_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    match #postgresql_type_ident_where_element_in_upper_camel_case::try_new(__field0, __field1) {
                                        Ok(value) => _serde::__private::Ok(value),
                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                    }
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<
                                        crate::LogicalOperator,
                                    > = _serde::__private::None;
                                    let mut __field1: _serde::__private::Option<
                                        std::vec::Vec<
                                            #field_type,
                                        >,
                                    > = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "logical_operator",
                                                        ),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        crate::LogicalOperator,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::__private::Option::is_some(&__field1) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                                    );
                                                }
                                                __field1 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        std::vec::Vec<
                                                            #field_type,
                                                        >,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            _ => {
                                                let _ = _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(&mut __map)?;
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("logical_operator")?
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::__private::Some(__field1) => __field1,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("value")?
                                        }
                                    };
                                    match #postgresql_type_ident_where_element_in_upper_camel_case::try_new(__field0, __field1) {
                                        Ok(value) => _serde::__private::Ok(value),
                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                    }
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                #postgresql_type_ident_where_element_in_double_quotes_token_stream,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<
                                        #postgresql_type_ident_where_element_in_upper_camel_case,
                                    >,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                };
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_in_token_stream = {
            quote::quote! {
                impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_type_ident_where_element_in_upper_camel_case {
                    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        Self {
                            logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                            value: vec![
                                #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            ]
                        }
                    }
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_in_token_stream = {
            quote::quote! {
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_ident_where_element_in_upper_camel_case {
                    fn postgresql_type_self_where_try_generate_bind_increments(
                        &self,
                        increment: &mut std::primitive::u64,
                        column: &dyn std::fmt::Display,
                        is_need_to_add_logical_operator: std::primitive::bool,
                    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                        let mut acc = std::string::String::default();
                        for element in &self.value {
                            match crate::BindQuerySecond::try_generate_bind_increments(element, increment) {
                                Ok(value) => {
                                    acc.push_str(&format!("{value},"));
                                }
                                Err(error) => {
                                    return Err(error);//todo another checked add? 
                                },
                            }
                        }
                        let _ = acc.pop();
                        let in_snake_case = naming::InSnakeCase;
                        Ok(format!("{}({column} {in_snake_case} ({acc}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                    }
                    fn postgresql_type_self_where_bind_value_to_query<'a>(
                        self,
                        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        for element in self.value {
                            query = crate::BindQuerySecond::bind_value_to_query(element, query);
                        }
                        query
                    }
                }
            }
        };
        quote::quote! {
            #postgresql_type_ident_where_element_in_token_stream
            #postgresql_type_ident_where_element_in_try_new_error_named_token_stream
            #impl_postgresql_type_ident_where_element_in_try_new_token_stream
            #impl_serde_deserialize_for_postgresql_type_ident_where_element_in_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_in_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_in_token_stream
        }
    };
    let postgresql_type_ident_where_element_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_element_token_stream = {
        let postgresql_type_ident_where_element_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                pub enum #postgresql_type_ident_where_element_upper_camel_case {
                    Equal(#postgresql_type_ident_where_element_equal_upper_camel_case),
                    GreaterThan(#postgresql_type_ident_where_element_greater_than_upper_camel_case),
                    Between(#postgresql_type_ident_where_element_between_upper_camel_case),
                    In(#postgresql_type_ident_where_element_in_upper_camel_case)
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_token_stream = {
            quote::quote! {
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_ident_where_element_upper_camel_case {
                    fn postgresql_type_self_where_try_generate_bind_increments(
                        &self,
                        increment: &mut std::primitive::u64,
                        column: &dyn std::fmt::Display,
                        is_need_to_add_logical_operator: std::primitive::bool,
                    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                        match &self {
                            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                                value,
                                increment,
                                column,
                                is_need_to_add_logical_operator,
                            ),
                            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                                value,
                                increment,
                                column,
                                is_need_to_add_logical_operator,
                            ),
                            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                                value,
                                increment,
                                column,
                                is_need_to_add_logical_operator,
                            ),
                            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                                value,
                                increment,
                                column,
                                is_need_to_add_logical_operator,
                            ),
                        }
                    }
                    fn postgresql_type_self_where_bind_value_to_query<'a>(
                        self,
                        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        match self {
                            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                                value,
                                query
                            ),
                            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                                value,
                                query
                            ),
                            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                                value,
                                query
                            ),
                            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                                value,
                                query
                            ),
                        }
                    }
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_element_traits_for_postgresql_type_ident_where_element_token_stream = {
            quote::quote!{
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for #postgresql_type_ident_where_element_upper_camel_case {}
            }
        };
        let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_where_element_token_stream = {
            quote::quote!{
                impl error_occurence_lib::ToStdStringString for #postgresql_type_ident_where_element_upper_camel_case {
                    fn to_std_string_string(&self) -> std::string::String {
                        format!("{self:#?}")
                    }
                }
            }
        };
        let impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_token_stream = {
            quote::quote!{
                impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_type_ident_where_element_upper_camel_case {
                    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
                        vec![
                            Self::Equal(
                                #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            ),
                            Self::GreaterThan(
                                #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            ),
                            Self::Between(
                                #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            ),
                            Self::In(
                                #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            ),
                        ]
                    }
                }
            }
        };
        quote::quote! {
            #postgresql_type_ident_where_element_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_element_traits_for_postgresql_type_ident_where_element_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_where_element_token_stream
            #impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_token_stream
        }
    };
    let generated = quote::quote! {
       #postgresql_type_ident_where_element_equal_token_stream
       #postgresql_type_ident_where_element_greater_than_token_stream
       #postgresql_type_ident_where_element_between_token_stream
       #postgresql_type_ident_where_element_in_token_stream
       #postgresql_type_ident_where_element_token_stream
    };
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlTypeTokensNotNullWhereElementInt",
    //         &generated,
    //     );
    // }
    generated.into()
}

#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementInt)]
pub fn postgresql_base_type_tokens_where_element_int(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    };
    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let postgresql_type_ident_where_element_equal_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementEqualUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_element_equal_token_stream = {
        let postgresql_type_ident_where_element_equal_token_stream = {
            quote::quote!{
                #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
                pub struct #postgresql_type_ident_where_element_equal_upper_camel_case {
                    pub logical_operator: crate::LogicalOperator,
                    pub value: #field_type
                }
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_equal_token_stream = {
            quote::quote!{
                impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_type_ident_where_element_equal_upper_camel_case {
                    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        Self {
                            logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                            value: ::core::default::Default::default()
                        }
                    }
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_equal_token_stream = {
            let postgresql_type_self_where_try_generate_bind_increments_token_stream = {
                let increment_snake_case = naming::IncrementSnakeCase;
                let acc_snake_case = naming::AccSnakeCase;
                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("${{{increment_snake_case}}}"));
                quote::quote! {
                    match #increment_snake_case.checked_add(1) {
                        Some(value) => {
                            *#increment_snake_case = value;
                            Ok(format!(
                                "{}({column} = ${increment})",
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            ))
                        },
                        None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            };
            let postgresql_type_self_where_bind_value_to_query_token_stream = {
                quote::quote!{
                    query = query.bind(self.value);
                    query
                }
            };
            quote::quote!{
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_ident_where_element_equal_upper_camel_case {
                    fn postgresql_type_self_where_try_generate_bind_increments(
                        &self,
                        increment: &mut std::primitive::u64,
                        column: &dyn std::fmt::Display,
                        is_need_to_add_logical_operator: std::primitive::bool,
                    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                        #postgresql_type_self_where_try_generate_bind_increments_token_stream
                    }
                    fn postgresql_type_self_where_bind_value_to_query<'a>(
                        self,
                        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        #postgresql_type_self_where_bind_value_to_query_token_stream
                    }
                }
            }
        };
        quote::quote!{
            #postgresql_type_ident_where_element_equal_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_equal_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_equal_token_stream
        }
    };
    let postgresql_type_ident_where_element_greater_than_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementGreaterThanUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_element_greater_than_token_stream = {
        let postgresql_type_ident_where_element_greater_than_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
                pub struct #postgresql_type_ident_where_element_greater_than_upper_camel_case {
                    pub logical_operator: crate::LogicalOperator,
                    pub value: #field_type
                }
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_greater_than_token_stream = {
            quote::quote! {
                impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_type_ident_where_element_greater_than_upper_camel_case {
                    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        Self {
                            logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                            value: ::core::default::Default::default(),
                        }
                    }
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_greater_than_token_stream = {
            let postgresql_type_self_where_try_generate_bind_increments_token_stream = {
                let increment_snake_case = naming::IncrementSnakeCase;
                let acc_snake_case = naming::AccSnakeCase;
                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("${{{increment_snake_case}}}"));
                quote::quote! {
                    match #increment_snake_case.checked_add(1) {
                        Some(value) => {
                            *#increment_snake_case = value;
                            Ok(format!(
                                "{}({column} > ${increment})",
                                &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            ))
                        },
                        None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            };
            let postgresql_type_self_where_bind_value_to_query_token_stream = {
                quote::quote!{
                    query = query.bind(self.value);
                    query
                }
            };
            quote::quote! {
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_ident_where_element_greater_than_upper_camel_case {
                    fn postgresql_type_self_where_try_generate_bind_increments(
                        &self,
                        increment: &mut std::primitive::u64,
                        column: &dyn std::fmt::Display,
                        is_need_to_add_logical_operator: std::primitive::bool,
                    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                        #postgresql_type_self_where_try_generate_bind_increments_token_stream
                    }
                    fn postgresql_type_self_where_bind_value_to_query<'a>(
                        self,
                        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        #postgresql_type_self_where_bind_value_to_query_token_stream
                    }
                }
            }
        };
        quote::quote! {
            #postgresql_type_ident_where_element_greater_than_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_greater_than_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_greater_than_token_stream
        }
    };
    let postgresql_type_ident_where_element_between_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementBetweenUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_element_between_token_stream = {
        let postgresql_type_ident_where_element_between_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde::Serialize)]
                pub struct #postgresql_type_ident_where_element_between_upper_camel_case {
                    logical_operator: crate::LogicalOperator,
                    start: #field_type,
                    end: #field_type
                }
            }
        };
        let postgresql_type_ident_where_element_between_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementBetweenTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
        let postgresql_type_ident_where_element_between_try_new_error_named_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #postgresql_type_ident_where_element_between_try_new_error_named_upper_camel_case {
                    StartMoreOrEqualToEnd {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                }
            }
        };
        let impl_postgresql_type_ident_where_element_between_try_new_token_stream = {
            quote::quote! {
                impl #postgresql_type_ident_where_element_between_upper_camel_case {
                    fn try_new(
                        logical_operator: crate::LogicalOperator,
                        start: #field_type,
                        end: #field_type
                    ) -> Result<Self, #postgresql_type_ident_where_element_between_try_new_error_named_upper_camel_case> {
                        if start < end {
                            Ok(Self {
                                logical_operator,
                                start,
                                end
                            })
                        }
                        else {
                            Err(#postgresql_type_ident_where_element_between_try_new_error_named_upper_camel_case::StartMoreOrEqualToEnd {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            })
                        }
                    }
                }
            }
        };
        let impl_serde_deserialize_for_postgresql_type_ident_where_element_between_token_stream = {
            let struct_postgresql_type_ident_where_element_between_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!(
                    "struct {postgresql_type_ident_where_element_between_upper_camel_case}"
                )
            );
            let struct_postgresql_type_ident_where_element_between_with_3_elements_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!(
                    "struct {postgresql_type_ident_where_element_between_upper_camel_case} with 3 elements"
                )
            );
            let postgresql_type_ident_where_element_between_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &postgresql_type_ident_where_element_between_upper_camel_case
            );
            quote::quote! {
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for #postgresql_type_ident_where_element_between_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        1u64 => _serde::__private::Ok(__Field::__field1),
                                        2u64 => _serde::__private::Ok(__Field::__field2),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                                        "start" => _serde::__private::Ok(__Field::__field1),
                                        "end" => _serde::__private::Ok(__Field::__field2),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                                        b"start" => _serde::__private::Ok(__Field::__field1),
                                        b"end" => _serde::__private::Ok(__Field::__field2),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private::PhantomData<
                                    #postgresql_type_ident_where_element_between_upper_camel_case,
                                >,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #postgresql_type_ident_where_element_between_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter<'_>,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        #struct_postgresql_type_ident_where_element_between_double_quotes_token_stream,
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        crate::LogicalOperator,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#struct_postgresql_type_ident_where_element_between_with_3_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                        #field_type,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &#struct_postgresql_type_ident_where_element_between_with_3_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match _serde::de::SeqAccess::next_element::<
                                        #field_type,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &#struct_postgresql_type_ident_where_element_between_with_3_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    match #postgresql_type_ident_where_element_between_upper_camel_case::try_new(__field0, __field1, __field2) {
                                        Ok(value) => _serde::__private::Ok(value),
                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                    }
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<
                                        crate::LogicalOperator,
                                    > = _serde::__private::None;
                                    let mut __field1: _serde::__private::Option<#field_type> = _serde::__private::None;
                                    let mut __field2: _serde::__private::Option<#field_type> = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "logical_operator",
                                                        ),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        crate::LogicalOperator,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::__private::Option::is_some(&__field1) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                                    );
                                                }
                                                __field1 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        #field_type,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::__private::Option::is_some(&__field2) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                                    );
                                                }
                                                __field2 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        #field_type,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            _ => {
                                                let _ = _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(&mut __map)?;
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("logical_operator")?
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::__private::Some(__field1) => __field1,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("start")?
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::__private::Some(__field2) => __field2,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("end")?
                                        }
                                    };
                                    match #postgresql_type_ident_where_element_between_upper_camel_case::try_new(__field0, __field1, __field2) {
                                        Ok(value) => _serde::__private::Ok(value),
                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                    }
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &[
                                "logical_operator",
                                "start",
                                "end",
                            ];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                #postgresql_type_ident_where_element_between_double_quotes_token_stream,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<
                                        #postgresql_type_ident_where_element_between_upper_camel_case,
                                    >,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                };
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_between_token_stream = {
            quote::quote! {
                impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_type_ident_where_element_between_upper_camel_case {
                    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        Self {
                            logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                            start: ::core::default::Default::default(),
                            end: ::core::default::Default::default(),
                        }
                    }
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_between_token_stream = {
            let postgresql_type_self_where_try_generate_bind_increments_token_stream = {
                let increment_snake_case = naming::IncrementSnakeCase;
                let acc_snake_case = naming::AccSnakeCase;
                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("${{{increment_snake_case}}}"));
                quote::quote! {
                    match #increment_snake_case.checked_add(1) {
                        Some(first_value) => {
                            *#increment_snake_case = first_value;
                            match #increment_snake_case.checked_add(1) {
                                Some(second_value) => {
                                    *#increment_snake_case = second_value;
                                    let between_snake_case = naming::BetweenSnakeCase;
                                    let and_snake_case = naming::AndSnakeCase;
                                    Ok(format!("{}({column} {between_snake_case} {first_value} {and_snake_case} {second_value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                                },
                                None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                })
                            }
                        },
                        None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
            };
            let postgresql_type_self_where_bind_value_to_query_token_stream = {
                quote::quote!{
                    query = query.bind(self.start);
                    query = query.bind(self.end);
                    query
                }
            };
            quote::quote! {
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_ident_where_element_between_upper_camel_case {
                    fn postgresql_type_self_where_try_generate_bind_increments(
                        &self,
                        increment: &mut std::primitive::u64,
                        column: &dyn std::fmt::Display,
                        is_need_to_add_logical_operator: std::primitive::bool,
                    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                        #postgresql_type_self_where_try_generate_bind_increments_token_stream
                    }
                    fn postgresql_type_self_where_bind_value_to_query<'a>(
                        self,
                        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        #postgresql_type_self_where_bind_value_to_query_token_stream
                    }
                }
            }
        };
        quote::quote! {
            #postgresql_type_ident_where_element_between_token_stream
            #postgresql_type_ident_where_element_between_try_new_error_named_token_stream
            #impl_postgresql_type_ident_where_element_between_try_new_token_stream
            #impl_serde_deserialize_for_postgresql_type_ident_where_element_between_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_between_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_between_token_stream
        }
    };
    let postgresql_type_ident_where_element_in_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementInUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_element_in_token_stream = {
        let postgresql_type_ident_where_element_in_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde::Serialize)]
                pub struct #postgresql_type_ident_where_element_in_upper_camel_case {
                    logical_operator: crate::LogicalOperator,
                    value: std::vec::Vec<#field_type>
                }
            }
        };
        let postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementInTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
        let postgresql_type_ident_where_element_in_try_new_error_named_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case {
                    IsEmpty {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    NotUnique {
                        #[eo_to_std_string_string_serialize_deserialize]
                        value: #field_type,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                }
            }
        };
        let impl_postgresql_type_ident_where_element_in_try_new_token_stream = {
            quote::quote! {
                impl #postgresql_type_ident_where_element_in_upper_camel_case {
                    fn try_new(
                        logical_operator: crate::LogicalOperator,
                        value: std::vec::Vec<#field_type>
                    ) -> Result<Self, #postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case> {
                        if value.is_empty() {
                            return Err(#postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case::IsEmpty {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        {
                            let mut acc = vec![];
                            for element in &value {
                                if !acc.contains(&element) {
                                    acc.push(element);
                                } else {
                                    return Err(#postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case::NotUnique {
                                        value: element.clone(),
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            }
                        }
                        Ok(Self{
                            logical_operator,
                            value
                        })
                    }
                }
            }
        };
        let impl_serde_deserialize_for_postgresql_type_ident_where_element_in_token_stream = {
            let struct_postgresql_type_ident_where_element_in_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!(
                    "struct {postgresql_type_ident_where_element_in_upper_camel_case}"
                )
            );
            let struct_postgresql_type_ident_where_element_in_with_2_elements_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!(
                    "struct {postgresql_type_ident_where_element_in_upper_camel_case} with 2 elements"
                )
            );
            let postgresql_type_ident_where_element_in_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &postgresql_type_ident_where_element_in_upper_camel_case
            );
            quote::quote! {
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for #postgresql_type_ident_where_element_in_upper_camel_case {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __field1,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        1u64 => _serde::__private::Ok(__Field::__field1),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                                        "value" => _serde::__private::Ok(__Field::__field1),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                                        b"value" => _serde::__private::Ok(__Field::__field1),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private::PhantomData<
                                    #postgresql_type_ident_where_element_in_upper_camel_case,
                                >,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = #postgresql_type_ident_where_element_in_upper_camel_case;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        #struct_postgresql_type_ident_where_element_in_double_quotes_token_stream,
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        crate::LogicalOperator,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &#struct_postgresql_type_ident_where_element_in_with_2_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                        std::vec::Vec<#field_type>,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &#struct_postgresql_type_ident_where_element_in_with_2_elements_double_quotes_token_stream,
                                                ),
                                            );
                                        }
                                    };
                                    match #postgresql_type_ident_where_element_in_upper_camel_case::try_new(__field0, __field1) {
                                        Ok(value) => _serde::__private::Ok(value),
                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                    }
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<
                                        crate::LogicalOperator,
                                    > = _serde::__private::None;
                                    let mut __field1: _serde::__private::Option<
                                        std::vec::Vec<#field_type>,
                                    > = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "logical_operator",
                                                        ),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        crate::LogicalOperator,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::__private::Option::is_some(&__field1) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                                    );
                                                }
                                                __field1 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        std::vec::Vec<#field_type>,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            _ => {
                                                let _ = _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(&mut __map)?;
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("logical_operator")?
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::__private::Some(__field1) => __field1,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("value")?
                                        }
                                    };
                                    match #postgresql_type_ident_where_element_in_upper_camel_case::try_new(__field0, __field1) {
                                        Ok(value) => _serde::__private::Ok(value),
                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                    }
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                #postgresql_type_ident_where_element_in_double_quotes_token_stream,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<
                                        #postgresql_type_ident_where_element_in_upper_camel_case,
                                    >,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                };
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_in_token_stream = {
            quote::quote! {
                impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_type_ident_where_element_in_upper_camel_case {
                    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                        Self {
                            logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                            value: vec![::core::default::Default::default()]
                        }
                    }
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_in_token_stream = {
            let postgresql_type_self_where_try_generate_bind_increments_token_stream = {
                let increment_snake_case = naming::IncrementSnakeCase;
                let acc_snake_case = naming::AccSnakeCase;
                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("${{{increment_snake_case}}}"));
                quote::quote! {
                    let mut acc = std::string::String::default();
                    for element in &self.value {
                        match #increment_snake_case.checked_add(1) {
                            Some(value) => {
                                acc.push_str(&format!("${value},"));
                            },
                            None => {
                                return Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        }
                    }
                    let _ = acc.pop();
                    let in_snake_case = naming::InSnakeCase;
                    Ok(format!("{}({column} {in_snake_case} ({acc}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                }
            };
            let postgresql_type_self_where_bind_value_to_query_token_stream = {
                quote::quote!{
                    for element in self.value {
                        query = query.bind(element);
                    }
                    query
                }
            };
            quote::quote! {
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_ident_where_element_in_upper_camel_case {
                    fn postgresql_type_self_where_try_generate_bind_increments(
                        &self,
                        increment: &mut std::primitive::u64,
                        column: &dyn std::fmt::Display,
                        is_need_to_add_logical_operator: std::primitive::bool,
                    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                        #postgresql_type_self_where_try_generate_bind_increments_token_stream
                    }
                    fn postgresql_type_self_where_bind_value_to_query<'a>(
                        self,
                        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        #postgresql_type_self_where_bind_value_to_query_token_stream
                    }
                }
            }
        };
        quote::quote! {
            #postgresql_type_ident_where_element_in_token_stream
            #postgresql_type_ident_where_element_in_try_new_error_named_token_stream
            #impl_postgresql_type_ident_where_element_in_try_new_token_stream
            #impl_serde_deserialize_for_postgresql_type_ident_where_element_in_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_in_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_in_token_stream
        }
    };
    let postgresql_type_ident_where_element_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_where_element_token_stream = {
        let postgresql_type_ident_where_element_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                pub enum #postgresql_type_ident_where_element_upper_camel_case {
                    Equal(#postgresql_type_ident_where_element_equal_upper_camel_case),
                    GreaterThan(#postgresql_type_ident_where_element_greater_than_upper_camel_case),
                    Between(#postgresql_type_ident_where_element_between_upper_camel_case),
                    In(#postgresql_type_ident_where_element_in_upper_camel_case)
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_token_stream = {
            quote::quote! {
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_ident_where_element_upper_camel_case {
                    fn postgresql_type_self_where_try_generate_bind_increments(
                        &self,
                        increment: &mut std::primitive::u64,
                        column: &dyn std::fmt::Display,
                        is_need_to_add_logical_operator: std::primitive::bool,
                    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                        match &self {
                            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                                value,
                                increment,
                                column,
                                is_need_to_add_logical_operator,
                            ),
                            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                                value,
                                increment,
                                column,
                                is_need_to_add_logical_operator,
                            ),
                            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                                value,
                                increment,
                                column,
                                is_need_to_add_logical_operator,
                            ),
                            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                                value,
                                increment,
                                column,
                                is_need_to_add_logical_operator,
                            ),
                        }
                    }
                    fn postgresql_type_self_where_bind_value_to_query<'a>(
                        self,
                        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
                    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        match self {
                            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                                value,
                                query
                            ),
                            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                                value,
                                query
                            ),
                            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                                value,
                                query
                            ),
                            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                                value,
                                query
                            ),
                        }
                    }
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_element_traits_for_postgresql_type_ident_where_element_token_stream = {
            quote::quote!{
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for #postgresql_type_ident_where_element_upper_camel_case {}
            }
        };
        let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_where_element_token_stream = {
            quote::quote!{
                impl error_occurence_lib::ToStdStringString for #postgresql_type_ident_where_element_upper_camel_case {
                    fn to_std_string_string(&self) -> std::string::String {
                        format!("{self:#?}")
                    }
                }
            }
        };
        let impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_token_stream = {
            quote::quote!{
                impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #postgresql_type_ident_where_element_upper_camel_case {
                    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
                        vec![
                            Self::Equal(
                                #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            ),
                            Self::GreaterThan(
                                #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            ),
                            Self::Between(
                                #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            ),
                            Self::In(
                                #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            ),
                        ]
                    }
                }
            }
        };
        quote::quote! {
            #postgresql_type_ident_where_element_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_element_traits_for_postgresql_type_ident_where_element_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_where_element_token_stream
            #impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_token_stream
        }
    };
    let generated = quote::quote! {
       #postgresql_type_ident_where_element_equal_token_stream
       #postgresql_type_ident_where_element_greater_than_token_stream
       #postgresql_type_ident_where_element_between_token_stream
       #postgresql_type_ident_where_element_in_token_stream
       #postgresql_type_ident_where_element_token_stream
    };
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlTypeTokensNotNullWhereElementInt",
    //         &generated,
    //     );
    // }
    generated.into()
}