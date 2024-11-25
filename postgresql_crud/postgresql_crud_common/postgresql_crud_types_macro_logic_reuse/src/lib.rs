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
        let value = format!("{}{ident}", naming_conventions::WhereUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!("{}{ident}", naming_conventions::StdOptionOptionUpperCamelCase);
        value
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!("{}{}{ident}", naming_conventions::WhereUpperCamelCase, naming_conventions::StdOptionOptionUpperCamelCase);
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

fn generate_impl_postgresql_json_type_token_stream(input: proc_macro::TokenStream, variant: StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant) -> proc_macro::TokenStream {
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
    let ident_to_create_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_to_create_upper_camel_case, &ident);
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
    let ident_options_to_read_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_options_to_read_upper_camel_case, &ident);
    let ident_option_to_update_upper_camel_case = naming_conventions::SelfOptionToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_option_to_update_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&ident_option_to_update_upper_camel_case, &ident);
    let ident_option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case = naming_conventions::SelfOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamedUpperCamelCase::from_dyn_quote_to_tokens(&ident);

    let checked_add_upper_camel_case = naming_conventions::CheckedAddUpperCamelCase;

    let ident_option_to_update_try_generate_bind_increments_error_named_token_stream = {
        quote::quote!{
            #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            pub enum #ident_option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case {
                #checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
            }
        }
    };
    let impl_crate_generate_postgresql_query_part_postgresql_json_type_for_ident_token_stream = postgresql_crud_macros_common::generate_postgresql_json_type_token_stream(
        &quote::quote!{crate::generate_postgresql_query_part::},
        &ident,
        &ident_to_create_upper_camel_case,
        &{
            let crate_generate_postgresql_query_part_postgresql_json_type_try_generate_postgresql_query_part_to_create_error_named_token_stream = quote::quote!{
                crate::generate_postgresql_query_part::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed
            };
            quote::quote!{
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        Ok(format!("${increment}"))
                    }
                    None => Err(#crate_generate_postgresql_query_part_postgresql_json_type_try_generate_postgresql_query_part_to_create_error_named_token_stream::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!()
                    }),
                }
            }
        },
        &{
            let to_create_snake_case = naming_conventions::ToCreateSnakeCase;
            quote::quote!{
                query = query.bind(sqlx::types::Json(#to_create_snake_case.0));
                query
            }
        },
        &ident_field_reader_upper_camel_case,
        &ident_options_to_read_upper_camel_case,
        &{
            let field_reader_snake_case = naming_conventions::FieldReaderSnakeCase;
            let postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream = |format_handle_token_stream: &dyn quote::ToTokens| {
                let pagination_start_end_initialization_token_stream = macros_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream(&field_reader_snake_case);
                quote::quote! {
                    #pagination_start_end_initialization_token_stream
                    format!(#format_handle_token_stream)
                }
            };
            let column_name_and_maybe_field_getter_snake_case = naming_conventions::ColumnNameAndMaybeFieldGetterSnakeCase;
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
        &ident_option_to_update_upper_camel_case,
        &ident_option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case,
        &{
            let jsonb_set_accumulator_snake_case = naming_conventions::JsonbSetAccumulatorSnakeCase;
            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                &format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{jsonb_set_path}}}}}}',${{increment}})")
            );
            let option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case = naming_conventions::OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamedUpperCamelCase;
            quote::quote!{
                match increment.checked_add(1) {
                    Some(value) => {
                        *increment = value;
                        Ok(format!(#format_handle_token_stream))
                    }
                    None => Err(Self::#option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
        },
        &{
            let option_to_update_snake_case = naming_conventions::OptionToUpdateSnakeCase;
            quote::quote!{
                query = query.bind(sqlx::types::Json(#option_to_update_snake_case.0));
                query
            }
        }
    );
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
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath)
}
#[proc_macro_derive(GenerateImplPostgresqlJsonTypeStdOptionOptionFullTypePath)]
pub fn generate_impl_postgresql_json_type_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateImplPostgresqlJsonTypeStdVecVecFullTypePath)]
pub fn generate_impl_postgresql_json_type_std_vec_vec_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateImplPostgresqlJsonTypeStdOptionOptionStdVecVecFullTypePath)]
pub fn generate_impl_postgresql_json_type_std_option_option_std_vec_vec_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath)
}
#[proc_macro_derive(GenerateImplPostgresqlJsonTypeStdVecVecStdOptionOptionFullTypePath)]
pub fn generate_impl_postgresql_json_type_std_vec_vec_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath)
}
#[proc_macro_derive(GenerateImplPostgresqlJsonTypeStdOptionOptionStdVecVecStdOptionOptionFullTypePath)]
pub fn generate_impl_postgresql_json_type_std_option_option_std_vec_vec_std_option_option_full_type_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl_postgresql_json_type_token_stream(input, StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath)
}


///////////////////////////////
#[proc_macro_derive(PostgresqlCrudBaseTypeTokens)] //todo check on postgresql max length value of type
pub fn postgresql_crud_base_type_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
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
    let std_option_option_field_type_token_stream = quote::quote!{std::option::Option<#field_type>};
    let ident_where_token_stream = naming_conventions::SelfWhereUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let std_option_option_ident_upper_camel_case_token_stream = naming_conventions::StdOptionOptionSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let where_std_option_option_ident_upper_camel_case_token_stream = naming_conventions::WhereStdOptionOptionSelfUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let try_generate_bind_increments_error_named_upper_camel_case = naming_conventions::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let checked_add_upper_camel_case = naming_conventions::CheckedAddUpperCamelCase;
    let (
        impl_sqlx_type_sqlx_postgres_for_ident_token_stream,
        impl_sqlx_type_sqlx_postgres_for_std_option_option_ident_token_stream
    ) = {
        let generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream = |
            ident_token_stream: &dyn quote::ToTokens,
            field_type_token_stream: &dyn quote::ToTokens
        |{
            let field_type_as_sqlx_type_sqlx_postgres_token_stream = quote::quote!{<#field_type_token_stream as sqlx::Type<sqlx::Postgres>>::};
            quote::quote! {
                impl sqlx::Type<sqlx::Postgres> for #ident_token_stream {
                    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                       #field_type_as_sqlx_type_sqlx_postgres_token_stream type_info()
                    }
                    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
                       #field_type_as_sqlx_type_sqlx_postgres_token_stream compatible(ty)
                    }
                }
            }
        };
        (
            generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
                &ident,
                &field_type
            ),
            generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
                &std_option_option_ident_upper_camel_case_token_stream,
                &std_option_option_field_type_token_stream
            )
        )
    };
    let (
        impl_sqlx_encode_sqlx_postgres_for_ident_token_stream,
        impl_sqlx_encode_sqlx_postgres_for_std_option_option_ident_token_stream
    ) = {
        let generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream = |ident_token_stream: &dyn quote::ToTokens|{
            quote::quote! {
                impl sqlx::Encode<'_, sqlx::Postgres> for #ident_token_stream {
                    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
                        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
                    }
                }
            }
        };
        (
            generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(&ident),
            generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(&std_option_option_ident_upper_camel_case_token_stream)
        )
    };
    let (
        impl_sqlx_decode_sqlx_postgres_for_ident_token_stream,
        impl_sqlx_decode_sqlx_postgres_for_std_option_option_ident_token_stream,
    ) = {
        let generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream = |
            ident_token_stream: &dyn quote::ToTokens,
            field_type_token_stream: &dyn quote::ToTokens
        |{
            quote::quote! {
                impl sqlx::Decode<'_, sqlx::Postgres> for #ident_token_stream {
                    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                        match <#field_type_token_stream as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                            Ok(value) => Ok(Self(value)),
                            Err(error) => Err(error)
                        }
                    }
                }
            }
        };
        (
            generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
                &ident,
                &field_type
            ),
            generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
                &std_option_option_ident_upper_camel_case_token_stream,
                &quote::quote! {std::option::Option<#ident>}
            )
        )
    };
    let (
        impl_sqlx_postgres_pg_has_array_type_for_ident_token_stream,
        impl_sqlx_postgres_pg_has_array_type_for_std_option_option_ident_token_stream,
    ) = {
        let generate_impl_sqlx_postgres_pg_has_array_type_for_tokens_token_stream = |
            ident_token_stream: &dyn quote::ToTokens,
            field_type_token_stream: &dyn quote::ToTokens
        |{
            quote::quote!{
                impl sqlx::postgres::PgHasArrayType for #ident_token_stream {
                    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                        <#field_type_token_stream as sqlx::postgres::PgHasArrayType>::array_type_info()
                    }
                }
            }
        };
        (
            generate_impl_sqlx_postgres_pg_has_array_type_for_tokens_token_stream(
                &ident,
                &field_type
            ),
            generate_impl_sqlx_postgres_pg_has_array_type_for_tokens_token_stream(
                &std_option_option_ident_upper_camel_case_token_stream,
                &std_option_option_field_type_token_stream
            )
        )
    };
    let (
        impl_crate_bind_query_for_ident_token_stream,
        impl_crate_bind_query_for_std_option_option_ident_token_stream,
    ) = {
        let generate_impl_crate_bind_query_for_tokens_token_stream = |
            ident_token_stream: &dyn quote::ToTokens,
            try_generate_bind_increments_token_stream: &dyn quote::ToTokens,
            bind_value_to_query_token_stream: &dyn quote::ToTokens,
        |{
            quote::quote!{
                impl crate::BindQuerySecond<'_> for #ident_token_stream {
                    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::#try_generate_bind_increments_error_named_upper_camel_case> {
                        #try_generate_bind_increments_token_stream
                    }
                    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        #bind_value_to_query_token_stream
                    }
                }
            }
        };
        (
            generate_impl_crate_bind_query_for_tokens_token_stream(
                &ident,
                &quote::quote! {
                    let mut increments = std::string::String::default();
                    match increment.checked_add(1) {
                        Some(incr) => {
                            *increment = incr;
                            increments.push_str(&format!("${increment}"));
                        }
                        None => {
                            return Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                    Ok(increments)
                },
                &quote::quote! {
                    query = query.bind(self.0);
                    query
                }
            ),
            generate_impl_crate_bind_query_for_tokens_token_stream(
                &std_option_option_ident_upper_camel_case_token_stream,
                &quote::quote! {
                    let mut increments = std::string::String::default();
                    match increment.checked_add(1) {
                        Some(incr) => {
                            *increment = incr;
                            increments.push_str(&format!("${increment}"));
                        }
                        None => {
                            return Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                    Ok(increments)
                },
                &quote::quote! {
                    query = query.bind(match self.0 {
                        Some(value) => Some(value.0),
                        None => None
                    });
                    query
                }
            )
        )
    };
    let pub_crate_struct_std_option_option_ident_token_stream = quote::quote!{
        #[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
        pub(crate) struct #std_option_option_ident_upper_camel_case_token_stream(pub std::option::Option<#ident>);
    };
    let (
        impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream,
        impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_ident_token_stream
    ) = {
        let crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
            let generate_postgresql_query_part_snake_case = naming_conventions::GeneratePostgresqlQueryPartSnakeCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming_conventions::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
            quote::quote! {crate::#generate_postgresql_query_part_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case}
        };
        let default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming_conventions::DefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
        let generate_impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream = |
            ident_token_stream: &dyn quote::ToTokens,
            self_content_token_stream: &dyn quote::ToTokens,
        |{
            quote::quote! {
                impl #crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream for #ident_token_stream {
                    #[inline]
                    fn #default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case() -> Self {
                        Self(#self_content_token_stream)
                    }
                }
            }
        };
        (
            generate_impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &ident,
                &quote::quote!{::core::default::Default::default()},
            ),
            generate_impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &std_option_option_ident_upper_camel_case_token_stream,
                &quote::quote!{Some(#crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream::#default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case())},
            )
        )
    };
    let generated = quote::quote! {
        #impl_sqlx_type_sqlx_postgres_for_ident_token_stream
        #impl_sqlx_encode_sqlx_postgres_for_ident_token_stream
        #impl_sqlx_decode_sqlx_postgres_for_ident_token_stream
        #impl_sqlx_postgres_pg_has_array_type_for_ident_token_stream
        #impl_crate_bind_query_for_ident_token_stream
        #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream
        //////////
        #pub_crate_struct_std_option_option_ident_token_stream
        #impl_sqlx_type_sqlx_postgres_for_std_option_option_ident_token_stream
        #impl_sqlx_encode_sqlx_postgres_for_std_option_option_ident_token_stream
        #impl_sqlx_decode_sqlx_postgres_for_std_option_option_ident_token_stream
        #impl_sqlx_postgres_pg_has_array_type_for_std_option_option_ident_token_stream
        #impl_crate_bind_query_for_std_option_option_ident_token_stream
        #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_std_option_option_ident_token_stream
    };
    generated.into()
}

#[proc_macro_derive(PostgresqlCrudBaseWrapTypeTokens)] //todo check on postgresql max length value of type
pub fn postgresql_crud_base_wrap_type_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
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
    let ident_column_upper_camel_case = naming_conventions::SelfColumnUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let impl_std_fmt_display_for_ident_token_stream = {
        quote::quote!{
            impl std::fmt::Display for #ident {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(formatter, "{:?}", self.0)
                }
            }
        }
    };
    let impl_error_occurence_lib_to_std_string_string_for_ident_token_stream = {
        let error_occurence_lib_snake_case = naming_conventions::ErrorOccurenceLibSnakeCase;
        let to_std_string_string_upper_camel_case = naming_conventions::ToStdStringStringUpperCamelCase;
        let to_std_string_string_snake_case = naming_conventions::ToStdStringStringSnakeCase;
        let std_string_string_token_stream = token_patterns::StdStringString;
        quote::quote!{
            impl #error_occurence_lib_snake_case::#to_std_string_string_upper_camel_case for #ident {
                fn #to_std_string_string_snake_case(&self) -> #std_string_string_token_stream {
                    format!("{self}")
                }
            }
        }
    };
    let crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
        let generate_postgresql_query_part_snake_case = naming_conventions::GeneratePostgresqlQueryPartSnakeCase;
        let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming_conventions::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
        quote::quote!{
            crate::#generate_postgresql_query_part_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case
        }
    };
    let default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming_conventions::DefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
    let generate_impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream = |
        ident_token_stream: &dyn quote::ToTokens,
        content_token_stream: &dyn quote::ToTokens,
    |{
        quote::quote!{
            impl #crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream for #ident_token_stream {
                fn #default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case() -> Self {
                    Self(#content_token_stream)
                }
            }
        }
    };
    let impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream = generate_impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream(
        &ident,
        &quote::quote!{#crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream::#default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()}
    );
    let generate_impl_crate_bind_query_for_tokens_token_stream = |
        ident_token_stream: &dyn quote::ToTokens,
        try_generate_bind_increments_token_stream: &dyn quote::ToTokens,
        bind_value_to_query_token_stream: &dyn quote::ToTokens,
    |{
        let bind_query_upper_camel_case = naming_conventions::BindQueryUpperCamelCase;
        let try_generate_bind_increments_snake_case = naming_conventions::TryGenerateBindIncrementsSnakeCase;
        let bind_value_to_query_snake_case = naming_conventions::BindValueToQuerySnakeCase;
        let std_string_string_token_stream = token_patterns::StdStringString;
        let try_generate_bind_increments_error_named_upper_camel_case = naming_conventions::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        quote::quote!{
            impl crate::BindQuerySecond<'_> for #ident_token_stream {
                fn #try_generate_bind_increments_snake_case(&self, increment: &mut std::primitive::u64) -> Result<#std_string_string_token_stream, crate::#try_generate_bind_increments_error_named_upper_camel_case> {
                    #try_generate_bind_increments_token_stream
                }
                fn #bind_value_to_query_snake_case(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    #bind_value_to_query_token_stream
                }
            }
        }
    };
    //todo maybe not need it, maybe refactor later
    let impl_crate_bind_query_for_ident_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
        &ident,
        &quote::quote!{crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)},
        &quote::quote!{crate::BindQuerySecond::bind_value_to_query(self.0, query)},
    );
    let pub_struct_ident_column_token_stream = {
        quote::quote! {
            #[derive(
                Debug,
                Clone,
                Copy,
                PartialEq,
                serde::Serialize,
                serde::Deserialize,
            )]
            pub struct #ident_column_upper_camel_case;
        }
    };
    let impl_crate_generate_postgresql_query_part_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_column_token_stream = {
        let generate_postgresql_query_part_snake_case = naming_conventions::GeneratePostgresqlQueryPartSnakeCase;
        let all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming_conventions::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
        let all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming_conventions::AllEnumVariantsArrayDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
        quote::quote! {
            impl crate::#generate_postgresql_query_part_snake_case::#all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case for #ident_column_upper_camel_case {
                fn #all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case() -> std::vec::Vec<Self> {
                    vec![]
                }
            }
        }
    };
    let ident_to_create_upper_camel_case = naming_conventions::SelfToCreateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_to_create_token_stream = {
        quote::quote!{
            #[derive(
                Debug,
                Clone,
                serde::Serialize,
                serde::Deserialize,
            )]
            pub struct #ident_to_create_upper_camel_case(#field_type);
        }
    };
    let impl_crate_bind_query_for_ident_to_create_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
        &ident_to_create_upper_camel_case,
        &quote::quote!{crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)},
        &quote::quote!{crate::BindQuerySecond::bind_value_to_query(self.0, query)},
    );
    let impl_sqlx_decode_sqlx_postgres_for_ident_to_create_token_stream = {
        quote::quote!{
            impl sqlx::Decode<'_, sqlx::Postgres> for #ident_to_create_upper_camel_case {
                fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                    match <#field_type as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                        Ok(value) => Ok(Self(value)),
                        Err(error) => Err(error)
                    }
                }
            }
        }
    };
    let impl_sqlx_type_sqlx_postgres_for_ident_to_create_token_stream = {
        quote::quote!{
            impl sqlx::Type<sqlx::Postgres> for #ident_to_create_upper_camel_case {
                fn type_info() -> sqlx::postgres::PgTypeInfo {
                    <#field_type as sqlx::Type<sqlx::Postgres>>::type_info()
                }
            }
        }
    };
    let impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_token_stream = generate_impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream(
        &ident_to_create_upper_camel_case,
        &quote::quote!{#crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream::#default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()}
    );
    let ident_to_read_upper_camel_case = naming_conventions::SelfToReadUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_to_read_token_stream = {
        quote::quote!{
            #[derive(
                Debug,
                Clone,
                serde::Serialize,
                serde::Deserialize,
            )]
            pub struct #ident_to_read_upper_camel_case(#field_type);
        }
    };
    let impl_crate_bind_query_for_ident_to_read_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
        &ident_to_read_upper_camel_case,
        &quote::quote!{crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)},
        &quote::quote!{crate::BindQuerySecond::bind_value_to_query(self.0, query)},
    );
    let impl_sqlx_decode_sqlx_postgres_for_ident_to_read_token_stream = {
        quote::quote!{
            impl sqlx::Decode<'_, sqlx::Postgres> for #ident_to_read_upper_camel_case {
                fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                    match <#field_type as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                        Ok(value) => Ok(Self(value)),
                        Err(error) => Err(error)
                    }
                }
            }
        }
    };
    let impl_sqlx_type_sqlx_postgres_for_ident_to_read_token_stream = {
        quote::quote!{
            impl sqlx::Type<sqlx::Postgres> for #ident_to_read_upper_camel_case {
                fn type_info() -> sqlx::postgres::PgTypeInfo {
                    <#field_type as sqlx::Type<sqlx::Postgres>>::type_info()
                }
            }
        }
    };
    let impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_read_token_stream = generate_impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream(
        &ident_to_read_upper_camel_case,
        &quote::quote!{#crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream::#default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()}
    );
    let ident_to_update_upper_camel_case = naming_conventions::SelfToUpdateUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_to_update_token_stream = {
        quote::quote!{
            #[derive(
                Debug,
                Clone,
                PartialEq,
                serde::Serialize,
                serde::Deserialize,
            )]
            pub struct #ident_to_update_upper_camel_case(#field_type);
        }
    };
    let impl_crate_bind_query_for_ident_to_update_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
        &ident_to_update_upper_camel_case,
        &quote::quote!{crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)},
        &quote::quote!{crate::BindQuerySecond::bind_value_to_query(self.0, query)},
    );
    let impl_std_fmt_display_for_ident_to_update_token_stream = {
        quote::quote!{
            impl std::fmt::Display for #ident_to_update_upper_camel_case {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(formatter, "{:?}", self.0)
                }
            }
        }
    };
    let impl_error_occurence_lib_to_std_string_string_for_ident_to_update_token_stream = {
        let error_occurence_lib_snake_case = naming_conventions::ErrorOccurenceLibSnakeCase;
        let to_std_string_string_upper_camel_case = naming_conventions::ToStdStringStringUpperCamelCase;
        let to_std_string_string_snake_case = naming_conventions::ToStdStringStringSnakeCase;
        let std_string_string_token_stream = token_patterns::StdStringString;
        quote::quote!{
            impl #error_occurence_lib_snake_case::#to_std_string_string_upper_camel_case for #ident_to_update_upper_camel_case {
                fn #to_std_string_string_snake_case(&self) -> #std_string_string_token_stream {
                    format!("{self}")
                }
            }
        }
    };
    let impl_sqlx_encode_sqlx_postgres_for_ident_to_update_token_stream = {
        quote::quote!{
            impl sqlx::Encode<'_, sqlx::Postgres> for #ident_to_update_upper_camel_case {
                fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
                    sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
                }
            }
        }
    };
    let impl_sqlx_decode_sqlx_postgres_for_ident_to_update_token_stream = {
        quote::quote!{
            impl sqlx::Decode<'_, sqlx::Postgres> for #ident_to_update_upper_camel_case {
                fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                    match <#field_type as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                        Ok(value) => Ok(Self(value)),
                        Err(error) => Err(error)
                    }
                }
            }
        }
    };
    let impl_sqlx_type_sqlx_postgres_for_ident_to_update_token_stream = {
        quote::quote!{
            impl sqlx::Type<sqlx::Postgres> for #ident_to_update_upper_camel_case {
                fn type_info() -> sqlx::postgres::PgTypeInfo {
                    <#field_type as sqlx::Type<sqlx::Postgres>>::type_info()
                }
            }
        }
    };
    let impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_update_token_stream = generate_impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream(
        &ident_to_update_upper_camel_case,
        &quote::quote!{#crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream::#default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()}
    );
    let ident_to_delete_upper_camel_case = naming_conventions::SelfToDeleteUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let ident_to_delete_token_stream = {
        quote::quote!{
            #[derive(
                Debug,
                Clone,
                PartialEq,
                serde::Serialize,
                serde::Deserialize,
            )]
            pub struct #ident_to_delete_upper_camel_case(#field_type);
        }
    };
    let impl_crate_bind_query_for_ident_to_delete_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
        &ident_to_delete_upper_camel_case,
        &quote::quote!{crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)},
        &quote::quote!{crate::BindQuerySecond::bind_value_to_query(self.0, query)},
    );
    let impl_std_fmt_display_for_ident_to_delete_token_stream = {
        quote::quote!{
            impl std::fmt::Display for #ident_to_delete_upper_camel_case {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(formatter, "{:?}", self.0)
                }
            }
        }
    };
    let impl_error_occurence_lib_to_std_string_string_for_ident_to_delete_token_stream = {
        let error_occurence_lib_snake_case = naming_conventions::ErrorOccurenceLibSnakeCase;
        let to_std_string_string_upper_camel_case = naming_conventions::ToStdStringStringUpperCamelCase;
        let to_std_string_string_snake_case = naming_conventions::ToStdStringStringSnakeCase;
        let std_string_string_token_stream = token_patterns::StdStringString;
        quote::quote!{
            impl #error_occurence_lib_snake_case::#to_std_string_string_upper_camel_case for #ident_to_delete_upper_camel_case {
                fn #to_std_string_string_snake_case(&self) -> #std_string_string_token_stream {
                    format!("{self}")
                }
            }
        }
    };
    //todo some implementations only for primary key types. maybe write 2 traits: 1 for typical type and 1 for primary key
    let generated = quote::quote! {
        #impl_std_fmt_display_for_ident_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_token_stream
        #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream
        #impl_crate_bind_query_for_ident_token_stream
        #pub_struct_ident_column_token_stream
        #impl_crate_generate_postgresql_query_part_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_column_token_stream
        #ident_to_create_token_stream
        #impl_crate_bind_query_for_ident_to_create_token_stream
        #impl_sqlx_decode_sqlx_postgres_for_ident_to_create_token_stream
        #impl_sqlx_type_sqlx_postgres_for_ident_to_create_token_stream
        #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_create_token_stream
        #ident_to_read_token_stream
        #impl_crate_bind_query_for_ident_to_read_token_stream
        #impl_sqlx_decode_sqlx_postgres_for_ident_to_read_token_stream
        #impl_sqlx_type_sqlx_postgres_for_ident_to_read_token_stream
        #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_read_token_stream
        #ident_to_update_token_stream
        #impl_crate_bind_query_for_ident_to_update_token_stream
        #impl_std_fmt_display_for_ident_to_update_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_to_update_token_stream
        #impl_sqlx_encode_sqlx_postgres_for_ident_to_update_token_stream
        #impl_sqlx_decode_sqlx_postgres_for_ident_to_update_token_stream
        #impl_sqlx_type_sqlx_postgres_for_ident_to_update_token_stream
        #impl_crate_generate_postgresql_query_part_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_to_update_token_stream
        #ident_to_delete_token_stream
        #impl_crate_bind_query_for_ident_to_delete_token_stream
        #impl_std_fmt_display_for_ident_to_delete_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_to_delete_token_stream
    };
    generated.into()
}