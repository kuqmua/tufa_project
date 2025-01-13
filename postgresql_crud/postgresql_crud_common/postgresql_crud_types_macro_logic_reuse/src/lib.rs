fn generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let error_occurence_lib_snake_case = naming::ErrorOccurenceLibSnakeCase;
    let to_std_string_string_upper_camel_case = naming::ToStdStringStringUpperCamelCase;
    let to_std_string_string_snake_case = naming::ToStdStringStringSnakeCase;
    let std_string_string_token_stream = token_patterns::StdStringString;
    let self_snake_case = naming::SelfSnakeCase;
    quote::quote!{
        impl #error_occurence_lib_snake_case::#to_std_string_string_upper_camel_case for #ident {
            fn #to_std_string_string_snake_case(&#self_snake_case) -> #std_string_string_token_stream {
                #content_token_stream
            }
        }
    }
}
fn generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
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
                #content_token_stream
            }
        }
    }
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
        let content_token_stream = {
            let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
            match &variant {
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath => quote::quote! {
                    #core_default_default_default
                },
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath => quote::quote! {
                    Some(#core_default_default_default)
                },
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecFullTypePath => quote::quote! {
                    vec![#core_default_default_default]
                },
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecFullTypePath => quote::quote! {
                    Some(vec![#core_default_default_default])
                },
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdVecVecStdOptionOptionFullTypePath => quote::quote! {
                    vec![Some(#core_default_default_default)]
                },
                StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote! {
                    Some(vec![Some(#core_default_default_default)])
                },
            }
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
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_field_reader_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &postgresql_json_type_ident_field_reader_upper_camel_case,
            &{
                let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                match &variant {
                    StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::FullTypePath | StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementVariant::StdOptionOptionFullTypePath => quote::quote! {
                        #core_default_default_default
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
                }
            },
        );
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
fn generate_postgresql_base_type_tokens(
    input: proc_macro::TokenStream,
    impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_content_token_stream: &dyn quote::ToTokens,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let std_option_option_field_type_token_stream = quote::quote!{std::option::Option<#field_type>};
    let std_option_option_ident_upper_camel_case = naming::parameter::StdOptionOptionSelfUpperCamelCase::from_tokens(&ident);
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let impl_error_occurence_lib_to_std_string_string_for_ident_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(
        &ident,
        &quote::quote!{format!("{self:#?}")},
    );
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
                &quote::quote!{Self(#impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_content_token_stream)},
            ),
            generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &std_option_option_ident_upper_camel_case,
                &quote::quote!{Self(
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
#[proc_macro_derive(PostgresqlBaseTypeTokens)]
pub fn postgresql_base_type_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &token_patterns::CoreDefaultDefaultDefault
    )
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
fn generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    postgresql_type_self_where_try_generate_bind_increments_token_stream: &dyn quote::ToTokens,
    postgresql_type_self_where_bind_value_to_query_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let increment_snake_case = naming::IncrementSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    quote::quote!{
        impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #ident {
            fn postgresql_type_self_where_try_generate_bind_increments(
                &self,
                #increment_snake_case: &mut std::primitive::u64,
                #column_snake_case : &dyn std::fmt::Display,
                #is_need_to_add_logical_operator_snake_case: std::primitive::bool,
            ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                #postgresql_type_self_where_try_generate_bind_increments_token_stream
            }
            fn postgresql_type_self_where_bind_value_to_query<'a>(
                self,
                mut #query_snake_case: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #postgresql_type_self_where_bind_value_to_query_token_stream
            }
        }
    }
}
fn generate_impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote!{
        impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident {
            fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
                #content_token_stream
            }
        }
    }
}
fn generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_element_traits_for_tokens_token_stream(ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for #ident {}}
}

enum PostgresqlTypeKind {
    Nullable,
    NotNull
}
impl PostgresqlTypeKind {
    fn postgresql_type_field_type_where_element_upper_camel_case(&self, field_type: &syn::Type) -> proc_macro2::TokenStream {
        let value: &dyn quote::ToTokens = match &self {
            Self::Nullable => &naming::parameter::PostgresqlTypeStdOptionOptionSelfWhereElementUpperCamelCase::from_type_last_segment(&field_type),
            Self::NotNull => &naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_type_last_segment(&field_type),
        };
        quote::quote!{#value}
    }
    fn ident_handle(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let value: &dyn quote::ToTokens = match &self {
            Self::Nullable => &naming::parameter::SelfNullableUpperCamelCase::from_tokens(&ident),
            Self::NotNull => &naming::parameter::SelfNotNullUpperCamelCase::from_tokens(&ident),
        };
        quote::quote!{#value}
    }
    fn field_type_handle(&self, field_type: &syn::Type) -> proc_macro2::TokenStream {
        let value: &dyn quote::ToTokens = match &self {
            Self::Nullable => &naming::parameter::StdOptionOptionSelfUpperCamelCase::from_type_last_segment(&field_type),
            Self::NotNull => &field_type,
        };
        quote::quote!{#value}
    }
}
#[proc_macro_derive(PostgresqlTypeTokens)]
pub fn postgresql_type_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);

    let generate_postgresql_type_nulllable_or_not_null = |postgresql_type_kind: &PostgresqlTypeKind| -> proc_macro2::TokenStream {
        let postgresql_type_field_type_where_element_upper_camel_case: &dyn quote::ToTokens = &postgresql_type_kind.postgresql_type_field_type_where_element_upper_camel_case(&field_type);
        let ident_handle: &dyn quote::ToTokens = &postgresql_type_kind.ident_handle(&ident);
        let field_type_handle: &dyn quote::ToTokens = &postgresql_type_kind.field_type_handle(&field_type);
        let postgresql_type_ident_column_upper_camel_case = naming::parameter::PostgresqlTypeSelfColumnUpperCamelCase::from_tokens(&ident_handle);

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
        let self_braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            Self(#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
        };
        let self_token_stream = {
            let ident_token_stream = {
                quote::quote!{
                    #[derive(
                        Debug,
                        Clone,
                        PartialEq,
                        serde::Serialize,
                        serde::Deserialize,
                    )]
                    pub struct #ident_handle(#field_type_handle);
                }
            };
            let impl_std_fmt_display_for_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(
                &ident_handle,
                &impl_std_fmt_display_for_tokens_self_zero_content_token_stream
            );
            let impl_error_occurence_lib_to_std_string_string_for_ident_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(
                &ident_handle,
                &quote::quote!{format!("{self}")}
            );
            let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &ident_handle,
                &self_braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            );
            //todo maybe not need it, maybe refactor later
            let impl_crate_bind_query_for_ident_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
                &ident_handle,
                &crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream,
                &crate_bind_query_bind_value_to_query_self_zero_query_token_stream,
            );
            let impl_ident_create_table_query_part_handle_token_stream = {
                quote::quote!{
                    impl #ident_handle {
                        pub fn create_table_query_part_handle(value: &dyn std::fmt::Display) -> impl std::fmt::Display {
                            #field_type_handle::create_table_query_part_handle(value)
                        }
                    }
                }
            };
            quote::quote!{
                #ident_token_stream
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
            let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_column_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &postgresql_type_ident_column_upper_camel_case,
                &token_patterns::CoreDefaultDefaultDefault,
            );
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
        let field_type_struct_content_token_stream = quote::quote!{(#field_type_handle);};
        let postgresql_type_ident_to_create_upper_camel_case = naming::parameter::PostgresqlTypeSelfToCreateUpperCamelCase::from_tokens(&ident_handle);
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
                &self_braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
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
        let postgresql_type_ident_to_read_upper_camel_case = naming::parameter::PostgresqlTypeSelfToReadUpperCamelCase::from_tokens(&ident_handle);
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
                &field_type_handle
            );
            let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_read_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
                &postgresql_type_ident_to_read_upper_camel_case,
                &field_type_handle
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
        let postgresql_type_ident_to_update_upper_camel_case = naming::parameter::PostgresqlTypeSelfToUpdateUpperCamelCase::from_tokens(&ident_handle);
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
                &self_braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
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

        let postgresql_type_ident_to_update_query_part_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfToUpdateQueryPartErrorNamedUpperCamelCase::from_tokens(&ident_handle);
        let postgresql_type_ident_to_update_query_part_error_named_token_stream = {
            quote::quote!{
                #[derive(
                    Debug,
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

        let postgresql_type_ident_to_delete_upper_camel_case = naming::parameter::PostgresqlTypeSelfToDeleteUpperCamelCase::from_tokens(&ident_handle);
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
                &self_braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            );
            let impl_std_fmt_display_for_postgresql_type_ident_to_delete_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(
                &postgresql_type_ident_to_delete_upper_camel_case,
                &impl_std_fmt_display_for_tokens_self_zero_content_token_stream
            );
            let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_to_delete_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(
                &postgresql_type_ident_to_delete_upper_camel_case,
                &quote::quote!{format!("{self}")}
            );
            let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_to_delete_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
                &postgresql_type_ident_to_delete_upper_camel_case,
                &field_type_handle
            );
            let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_to_delete_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
                &postgresql_type_ident_to_delete_upper_camel_case,
                &field_type_handle
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
        let postgresql_type_ident_where_element_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident_handle);
        let postgresql_type_ident_where_element_token_stream = {
            let postgresql_type_ident_where_element_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident_handle);
            let postgresql_type_ident_where_element_token_stream = {
                quote::quote! {
                    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                    pub struct #postgresql_type_ident_where_element_upper_camel_case(pub #postgresql_type_field_type_where_element_upper_camel_case);
                }
            };
            let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_token_stream = generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_token_stream(
                &postgresql_type_ident_where_element_upper_camel_case,
                &{
                    let column_snake_case = naming::ColumnSnakeCase;
                    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
                    quote::quote!{
                        crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                            &self.0,
                            #increment_snake_case,
                            #column_snake_case,
                            #is_need_to_add_logical_operator_snake_case,
                        )
                    }
                },
                &quote::quote!{
                    crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                        self.0,
                        #query_snake_case
                    )
                }
            );
            let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_element_traits_for_postgresql_type_ident_where_element_token_stream = generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_element_traits_for_tokens_token_stream(
                &postgresql_type_ident_where_element_upper_camel_case
            );
            let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_where_element_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(
                &postgresql_type_ident_where_element_upper_camel_case,
                &quote::quote!{format!("{self:#?}")},
            );
            let impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_token_stream = generate_impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &postgresql_type_ident_where_element_upper_camel_case,
                &quote::quote!{
                    <#postgresql_type_field_type_where_element_upper_camel_case as crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().into_iter().map(|element| Self(element)).collect()
                },
            );
            quote::quote! {
                #postgresql_type_ident_where_element_token_stream
                #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_token_stream
                #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_element_traits_for_postgresql_type_ident_where_element_token_stream
                #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_where_element_token_stream
                #impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_token_stream
            }
        };
        let postgresql_type_ident_where_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereUpperCamelCase::from_tokens(&ident_handle);
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
            let postgresql_type_ident_where_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereTryNewErrorNamedUpperCamelCase::from_tokens(&ident_handle);
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
                            #logical_operator_snake_case: crate::#logical_operator_upper_camel_case,
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
                                #logical_operator_snake_case,
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
                &quote::quote!{Self{
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
                impl crate::postgresql_type::postgresql_type_trait:: #postgresql_type_upper_camel_case<'_> for #ident_handle {
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

            #postgresql_type_ident_where_element_token_stream

            #postgresql_type_ident_where_token_stream

            #impl_postgresql_type_for_ident_token_stream
        };
        generated.into()
    };
    let ident_nullable_token_stream = generate_postgresql_type_nulllable_or_not_null(&PostgresqlTypeKind::Nullable);
    let ident_not_null_token_stream = generate_postgresql_type_nulllable_or_not_null(&PostgresqlTypeKind::NotNull);
    let generated = quote::quote!{
        #ident_nullable_token_stream
        #ident_not_null_token_stream
    };
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlTypeTokens",
    //         &generated,
    //     );
    // }
    generated.into()
}

enum IsPrimaryKey {
    True,
    False,
}
fn generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
    postgresql_type_kind: &PostgresqlTypeKind,
    ident: &syn::Ident,
    field_type: &syn::Type,
    is_primary_key: &IsPrimaryKey,
) -> proc_macro2::TokenStream {
    let ident_handle: &dyn quote::ToTokens = &postgresql_type_kind.ident_handle(&ident);
    let ident_as_crate_create_table_column_query_part_create_table_column_query_part_column_is_primary_key_token_stream = quote::quote!{
        <#ident as crate::CreateTableColumnQueryPart>::create_table_column_query_part(column, is_primary_key)
    };
    let content_token_stream: &dyn quote::ToTokens = match &postgresql_type_kind {
        PostgresqlTypeKind::Nullable => &ident_as_crate_create_table_column_query_part_create_table_column_query_part_column_is_primary_key_token_stream,
        PostgresqlTypeKind::NotNull => &match is_primary_key {
            IsPrimaryKey::True => quote::quote!{
                format!("{} NOT NULL {}", #ident_as_crate_create_table_column_query_part_create_table_column_query_part_column_is_primary_key_token_stream, crate::maybe_primary_key(is_primary_key))
            },
            IsPrimaryKey::False => quote::quote!{
                format!("{} NOT NULL", #ident_as_crate_create_table_column_query_part_create_table_column_query_part_column_is_primary_key_token_stream)
            }
        },
    };
    quote::quote!{
        impl crate::CreateTableColumnQueryPart for #ident_handle {
            fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
                #content_token_stream
            }
        }
    }
}
#[proc_macro_derive(PostgresqlTypeCreateTableColumnQueryPartTokens)]
pub fn postgresql_type_create_table_column_query_part_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let is_primary_key = IsPrimaryKey::False;
    let impl_crate_create_table_column_query_part_for_ident_nullable_token_stream = generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
       &PostgresqlTypeKind::Nullable,
        &ident,
        &field_type,
        &is_primary_key,
    );
    let impl_crate_create_table_column_query_part_for_ident_not_null_token_stream = generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
       &PostgresqlTypeKind::NotNull,
        &ident,
        &field_type,
        &is_primary_key,
    );
    let generated = quote::quote!{
        #impl_crate_create_table_column_query_part_for_ident_nullable_token_stream
        #impl_crate_create_table_column_query_part_for_ident_not_null_token_stream
    };
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlTypeCreateTableColumnQueryPartTokens",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlTypeCreateTableColumnQueryPartPrimaryKeyTokens)]
pub fn postgresql_type_create_table_column_query_part_primary_key_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let is_primary_key = IsPrimaryKey::True;
    let impl_crate_create_table_column_query_part_for_ident_nullable_token_stream = generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
       &PostgresqlTypeKind::Nullable,
        &ident,
        &field_type,
        &is_primary_key,
    );
    let impl_crate_create_table_column_query_part_for_ident_not_null_token_stream = generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
       &PostgresqlTypeKind::NotNull,
        &ident,
        &field_type,
        &is_primary_key,
    );
    let generated = quote::quote!{
        #impl_crate_create_table_column_query_part_for_ident_nullable_token_stream
        #impl_crate_create_table_column_query_part_for_ident_not_null_token_stream
    };
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlTypeCreateTableColumnQueryPartPrimaryKeyTokens",
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
    let self_braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        Self(#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
    };
    let field_type_struct_content_token_stream = quote::quote!{(#field_type);};
    let postgresql_type_ident_not_null_to_create_upper_camel_case = naming::parameter::PostgresqlTypeSelfNotNullToCreateUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_not_null_to_create_token_stream = {
        let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_not_null_to_create_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_create_upper_camel_case,
            &field_type
        );
        let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_create_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_create_upper_camel_case,
            &field_type
        );
        quote::quote! {
            #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_not_null_to_create_token_stream
            #impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_create_token_stream
        }
    };
    let postgresql_type_ident_not_null_to_read_upper_camel_case = naming::parameter::PostgresqlTypeSelfNotNullToReadUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_not_null_to_read_token_stream = {
        let impl_crate_bind_query_for_postgresql_type_ident_not_null_to_read_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_read_upper_camel_case,
            &crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream,
            &crate_bind_query_bind_value_to_query_self_zero_query_token_stream,
        );
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_not_null_to_read_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_read_upper_camel_case,
            &self_braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
        );
        quote::quote! {
            #impl_crate_bind_query_for_postgresql_type_ident_not_null_to_read_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_not_null_to_read_token_stream
        }
    };
    let postgresql_type_ident_not_null_to_update_upper_camel_case = naming::parameter::PostgresqlTypeSelfNotNullToUpdateUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_not_null_to_update_token_stream = {
        let impl_std_fmt_display_for_postgresql_type_ident_not_null_to_update_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_update_upper_camel_case,
            &impl_std_fmt_display_for_tokens_self_zero_content_token_stream
        );
        let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_not_null_to_update_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_update_upper_camel_case,
            &quote::quote!{format!("{self}")}
        );
        let impl_sqlx_encode_sqlx_postgres_for_postgresql_type_ident_not_null_to_update_token_stream = generate_impl_sqlx_encode_sqlx_postgres_for_tokens_token_stream(&postgresql_type_ident_not_null_to_update_upper_camel_case);
        let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_not_null_to_update_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_update_upper_camel_case,
            &field_type
        );
        let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_update_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_update_upper_camel_case,
            &field_type
        );
        quote::quote! {
            #impl_std_fmt_display_for_postgresql_type_ident_not_null_to_update_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_not_null_to_update_token_stream
            #impl_sqlx_encode_sqlx_postgres_for_postgresql_type_ident_not_null_to_update_token_stream
            #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_not_null_to_update_token_stream
            #impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_update_token_stream
        }
    };
    let postgresql_type_ident_not_null_to_delete_upper_camel_case = naming::parameter::SelfNotNullToDeleteUpperCamelCase::from_tokens(&ident);
    let postgresql_type_ident_not_null_to_delete_token_stream = {
        let postgresql_type_ident_not_null_to_delete_token_stream = generate_pub_struct_tokens_token_stream(
            Visibility::Pub,
            &postgresql_type_ident_not_null_to_delete_upper_camel_case,
            &field_type_struct_content_token_stream,
            false,
            true,
        );
        let impl_crate_bind_query_for_postgresql_type_ident_not_null_to_delete_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_delete_upper_camel_case,
            &crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream,
            &crate_bind_query_bind_value_to_query_self_zero_query_token_stream,
        );
        let impl_std_fmt_display_for_postgresql_type_ident_not_null_to_delete_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_delete_upper_camel_case,
            &impl_std_fmt_display_for_tokens_self_zero_content_token_stream
        );
        let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_not_null_to_delete_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_delete_upper_camel_case,
            &quote::quote!{format!("{self}")}
        );
        let impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_not_null_to_delete_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_delete_upper_camel_case,
            &field_type
        );
        let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_delete_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_delete_upper_camel_case,
            &field_type
        );
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_not_null_to_delete_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_delete_upper_camel_case,
            &self_braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
        );
        quote::quote!{
            #postgresql_type_ident_not_null_to_delete_token_stream
            #impl_crate_bind_query_for_postgresql_type_ident_not_null_to_delete_token_stream
            #impl_std_fmt_display_for_postgresql_type_ident_not_null_to_delete_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_not_null_to_delete_token_stream
            #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_not_null_to_delete_token_stream
            #impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_delete_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_not_null_to_delete_token_stream
        }
    };
    let impl_postgresql_crud_base_wrap_type_primary_key_for_ident_not_null_token_stream = {
        let ident_not_null = naming::parameter::SelfNotNullUpperCamelCase::from_tokens(&ident);
        quote::quote!{
            impl crate::postgresql_type::postgresql_type_trait:: PostgresqlTypePrimaryKey<'_> for #ident_not_null {
                type PostgresqlTypeSelfToCreate = #postgresql_type_ident_not_null_to_create_upper_camel_case;
                type PostgresqlTypeSelfToRead = #postgresql_type_ident_not_null_to_read_upper_camel_case;
                type PostgresqlTypeSelfToUpdate = #postgresql_type_ident_not_null_to_update_upper_camel_case;
                type PostgresqlTypeSelfToDelete = #postgresql_type_ident_not_null_to_delete_upper_camel_case;
            }
        }
    };
    //todo some implementations only for primary key types. maybe write 2 traits: 1 for typical type and 1 for primary key
    let generated = quote::quote! {
        #postgresql_type_ident_not_null_to_create_token_stream

        #postgresql_type_ident_not_null_to_read_token_stream

        #postgresql_type_ident_not_null_to_update_token_stream

        #postgresql_type_ident_not_null_to_delete_token_stream

        #impl_postgresql_crud_base_wrap_type_primary_key_for_ident_not_null_token_stream
    };
    // if ident == "" {
    //     println!("{generated}");
    //     println!("----------------------");
    // }
    generated.into()
}
fn generate_pub_enum_postgresql_type_tokens_where_element_token_stream(ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub enum #ident {
            #content_token_stream
        }
    }
}
enum IsNullable {
    True,
    False,
}
impl IsNullable {
    fn maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        match &self  {
            Self::True => {
                let column_snake_case = naming::ColumnSnakeCase;
                let query_snake_case = naming::QuerySnakeCase;
                let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
                    crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
                };
                let postgresql_type_std_option_option_ident_where_element_is_null_upper_camel_case = naming::parameter::PostgresqlTypeStdOptionOptionSelfWhereElementIsNullUpperCamelCase::from_tokens(&ident);
                let postgresql_type_std_option_option_ident_where_element_is_null_token_stream = generate_postgresql_type_tokens_where_element_tokens_token_stream(
                    &postgresql_type_std_option_option_ident_where_element_is_null_upper_camel_case,
                    &ShouldWhereElementFieldsBePublic::True,
                    &proc_macro2::TokenStream::new()
                );
                let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_std_option_option_ident_where_element_is_null_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                    &postgresql_type_std_option_option_ident_where_element_is_null_upper_camel_case,
                    &quote::quote! {Self {
                        logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    }},
                );
                let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_std_option_option_ident_where_element_is_null_token_stream = generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_token_stream(
                    &postgresql_type_std_option_option_ident_where_element_is_null_upper_camel_case,
                    &quote::quote! {
                        Ok(format!(
                            "{}({} is null)",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                        ))
                    },
                    &query_snake_case
                );
                quote::quote! {
                    #postgresql_type_std_option_option_ident_where_element_is_null_token_stream
                    #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_std_option_option_ident_where_element_is_null_token_stream
                    #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_std_option_option_ident_where_element_is_null_token_stream
                }
            },
            Self::False => proc_macro2::TokenStream::new(),
        }
    }
    fn maybe_add_is_null_variant<'a>(&self, variants: &'a std::vec::Vec<&'a dyn quote::ToTokens>) -> std::vec::Vec<&'a dyn quote::ToTokens> {
        if let Self::True = &self {
            let mut variants_cloned = variants.clone();
            variants_cloned.push(&naming::IsNullUpperCamelCase);
            variants_cloned
        }
        else {
            variants.clone()
        }
    }
}
enum ShouldWhereElementFieldsBePublic<'a> {
    True,
    False {
        ident: &'a dyn quote::ToTokens,
        postfix: &'a dyn naming::StdFmtDisplayPlusQuoteToTokens,
        try_new_error_named_variants_token_stream: &'a dyn quote::ToTokens,
        try_new_additional_input_parameters_token_stream: &'a dyn quote::ToTokens,
        try_new_content_token_stream: &'a dyn quote::ToTokens,
        impl_deserialize_token_stream: &'a dyn quote::ToTokens,
    }
}
impl ShouldWhereElementFieldsBePublic<'_> {
    fn maybe_generate_try_new_error_named_and_try_new_and_deserialize_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            Self::True => proc_macro2::TokenStream::new(),
            Self::False {
                ident,
                postfix,
                try_new_error_named_variants_token_stream,
                try_new_additional_input_parameters_token_stream,
                try_new_content_token_stream,
                impl_deserialize_token_stream,
            } => {
                let postgresql_type_ident_where_element_tokens_try_new_error_named_upper_camel_case = {
                    let value = format!(
                        "{}{postfix}{}",
                        naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident),
                        naming::TryNewErrorNamedUpperCamelCase
                    );
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let postgresql_type_ident_where_element_tokens_try_new_error_named_token_stream = {
                    quote::quote! {
                        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                        pub enum #postgresql_type_ident_where_element_tokens_try_new_error_named_upper_camel_case {
                            #try_new_error_named_variants_token_stream
                        }
                    }
                };
                let postgresql_type_ident_where_element_tokens_upper_camel_case = {
                    let value = format!(
                        "{}{postfix}",
                        naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident),
                    );
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let impl_postgresql_type_ident_where_element_tokens_try_new_token_stream = {
                    quote::quote! {
                        impl #postgresql_type_ident_where_element_tokens_upper_camel_case {
                            fn try_new(
                                logical_operator: crate::LogicalOperator,
                                #try_new_additional_input_parameters_token_stream
                            ) -> Result<Self, #postgresql_type_ident_where_element_tokens_try_new_error_named_upper_camel_case> {
                                #try_new_content_token_stream
                            }
                        }
                    }
                };
                quote::quote!{
                    #postgresql_type_ident_where_element_tokens_try_new_error_named_token_stream
                    #impl_postgresql_type_ident_where_element_tokens_try_new_token_stream
                    #impl_deserialize_token_stream
                }
            }
        }
    }
}
fn generate_serde_deserialize_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens, length: std::primitive::u64, postfix: &dyn naming::StdFmtDisplayPlusQuoteToTokens) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream
) {
    let struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
        &format!(
            "struct {postgresql_type_ident_where_element_tokens_upper_camel_case}"
        )
    );
    let struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
        &format!(
            "struct {postgresql_type_ident_where_element_tokens_upper_camel_case} with {length} elements"
        )
    );
    let postgresql_type_ident_where_element_tokens_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
        &postgresql_type_ident_where_element_tokens_upper_camel_case
    );
    (
        struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream,
        struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream,
        postgresql_type_ident_where_element_tokens_double_quotes_token_stream
    )
}
fn generate_postgresql_type_tokens_where_element_variant_token_stream(
    ident: &dyn quote::ToTokens,
    postfix: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    is_nullable: &IsNullable,
    should_where_element_fields_be_public: ShouldWhereElementFieldsBePublic,
    additional_type_declaration_token_stream: &dyn quote::ToTokens,
    additional_default_initialization_token_stream: &dyn quote::ToTokens,
    postgresql_type_self_where_try_generate_bind_increments_token_stream: &dyn quote::ToTokens,
    postgresql_type_self_where_bind_value_to_query_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let generate_postgresql_type_ident_where_element_tokens_upper_camel_case = |prefix: &dyn std::fmt::Display|{
        let value = format!("{prefix}{postfix}");
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let postgresql_type_ident_where_element_tokens_upper_camel_case = generate_postgresql_type_ident_where_element_tokens_upper_camel_case(
        &naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident)
    );
    match &is_nullable {
        IsNullable::True => macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(
            &generate_postgresql_type_ident_where_element_tokens_upper_camel_case(
                &naming::parameter::PostgresqlTypeStdOptionOptionSelfWhereElementUpperCamelCase::from_tokens(&ident)
            ),
            &postgresql_type_ident_where_element_tokens_upper_camel_case
        ),
        IsNullable::False => {
            let postgresql_type_ident_where_element_tokens_token_stream = generate_postgresql_type_tokens_where_element_tokens_token_stream(
                &postgresql_type_ident_where_element_tokens_upper_camel_case,
                &should_where_element_fields_be_public,
                &additional_type_declaration_token_stream,
            );
            let maybe_try_new_error_named_and_try_new_and_deserialize_token_stream = should_where_element_fields_be_public.maybe_generate_try_new_error_named_and_try_new_and_deserialize_token_stream();
            let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_tokens_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &postgresql_type_ident_where_element_tokens_upper_camel_case,
                &{
                    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
                        crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
                    };
                    quote::quote! {Self {
                        logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                        #additional_default_initialization_token_stream
                    }}
                },
            );
            let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_tokens_token_stream = generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_token_stream(
                &postgresql_type_ident_where_element_tokens_upper_camel_case,
                &postgresql_type_self_where_try_generate_bind_increments_token_stream,
                &postgresql_type_self_where_bind_value_to_query_token_stream
            );
            quote::quote! {
                #postgresql_type_ident_where_element_tokens_token_stream
                #maybe_try_new_error_named_and_try_new_and_deserialize_token_stream
                #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_tokens_token_stream
                #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_ident_where_element_tokens_token_stream
            }
        }
    }
}
fn generate_postgresql_type_tokens_where_element_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    should_where_element_fields_be_public: &ShouldWhereElementFieldsBePublic,
    additional_type_declaration_token_stream: &dyn quote::ToTokens
) -> proc_macro2::TokenStream {
    let maybe_pub_token_stream: &dyn quote::ToTokens = match should_where_element_fields_be_public {
        ShouldWhereElementFieldsBePublic::True => &naming::PubSnakeCase,
        ShouldWhereElementFieldsBePublic::False {
            ident: _,
            postfix: _,
            try_new_error_named_variants_token_stream: _,
            try_new_additional_input_parameters_token_stream: _,
            try_new_content_token_stream: _,
            impl_deserialize_token_stream: _,
        } => &proc_macro2::TokenStream::new()
    };
    let maybe_impl_serde_deserialize_token_stream = match should_where_element_fields_be_public {
        ShouldWhereElementFieldsBePublic::True => quote::quote! {serde::Deserialize, },
        ShouldWhereElementFieldsBePublic::False {
            ident: _,
            postfix: _,
            try_new_error_named_variants_token_stream: _,
            try_new_additional_input_parameters_token_stream: _,
            try_new_content_token_stream: _,
            impl_deserialize_token_stream: _,
        } => proc_macro2::TokenStream::new()
    };
    let logical_operator_snake_case = naming::LogicalOperatorSnakeCase;
    let logical_operator_upper_camel_case = naming::LogicalOperatorUpperCamelCase;
    quote::quote! {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, #maybe_impl_serde_deserialize_token_stream)]
        pub struct #ident {
            #maybe_pub_token_stream #logical_operator_snake_case: crate::#logical_operator_upper_camel_case,
            #additional_type_declaration_token_stream
        }
    }
}
fn generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
    is_nullable: IsNullable,
    ident: &dyn quote::ToTokens,
    variants_original: &std::vec::Vec<&dyn quote::ToTokens>
) -> proc_macro2::TokenStream {
    let variants = is_nullable.maybe_add_is_null_variant(variants_original);
    let value_snake_case = naming::ValueSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    };
    let postgresql_type_tokens_where_element_upper_camel_case: &dyn quote::ToTokens = match &is_nullable {
        IsNullable::True => &naming::parameter::PostgresqlTypeStdOptionOptionSelfWhereElementUpperCamelCase::from_tokens(&ident),
        IsNullable::False => &naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident)
    };
    let postgresql_type_tokens_where_element_token_stream = generate_pub_enum_postgresql_type_tokens_where_element_token_stream(
        &postgresql_type_tokens_where_element_upper_camel_case,
        &{
            let variants_token_stream = variants.iter().map(|element|{
                let postgresql_type_tokens_where_element_upper_camel_case: &dyn std::fmt::Display = match &is_nullable {
                    IsNullable::True => &naming::parameter::PostgresqlTypeStdOptionOptionSelfWhereElementUpperCamelCase::from_tokens(&ident),
                    IsNullable::False => &naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident)
                };
                let postgresql_type_tokens_where_element_equal_upper_camel_case = {
                    let value = format!("{postgresql_type_tokens_where_element_upper_camel_case}{}", quote::quote!{#element});
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote!{#element(#postgresql_type_tokens_where_element_equal_upper_camel_case)}
            });
            quote::quote!{#(#variants_token_stream),*}
        }
    );
    let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_tokens_where_element_token_stream = generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_token_stream(
        &postgresql_type_tokens_where_element_upper_camel_case,
        &{
            let variants_token_stream = variants.iter().map(|element|quote::quote!{
                Self::#element(#value_snake_case) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                    #value_snake_case,
                    #increment_snake_case,
                    #column_snake_case,
                    #is_need_to_add_logical_operator_snake_case,
                )
            });
            quote::quote!{
                match &self {
                    #(#variants_token_stream),*
                }
            }
        },
        &{
            let variants_token_stream = variants.iter().map(|element|quote::quote!{
                Self::#element(#value_snake_case) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                    #value_snake_case,
                    #query_snake_case
                )
            });
            quote::quote!{
                match self {
                    #(#variants_token_stream),*
                }
            }
        }
    );
    let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_element_traits_for_postgresql_type_tokens_where_element_token_stream = generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_element_traits_for_tokens_token_stream(
        &postgresql_type_tokens_where_element_upper_camel_case
    );
    let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_element_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(
        &postgresql_type_tokens_where_element_upper_camel_case,
        &quote::quote!{format!("{self:#?}")},
    );
    let impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_tokens_where_element_token_stream = generate_impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
        &postgresql_type_tokens_where_element_upper_camel_case,
        &{
            let variants_token_stream = variants.iter().map(|element|quote::quote!{
                Self::#element(
                    #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                )
            });
            quote::quote!{vec![#(#variants_token_stream),*]}
        },
    );
    quote::quote! {
        #postgresql_type_tokens_where_element_token_stream
        #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_tokens_where_element_token_stream
        #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_element_traits_for_postgresql_type_tokens_where_element_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_element_token_stream
        #impl_crate_generate_postgresql_json_type_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_tokens_where_element_token_stream
    }
}
fn generate_nullable_and_not_nullable_token_stream<F>(generate_postgresql_type_ident_where_element_token_stream: F) -> proc_macro2::TokenStream 
where
    F: Fn(IsNullable) -> proc_macro2::TokenStream
{
    let postgresql_type_ident_where_element_token_stream = generate_postgresql_type_ident_where_element_token_stream(IsNullable::False);
    let postgresql_type_std_option_option_ident_where_element_token_stream = generate_postgresql_type_ident_where_element_token_stream(IsNullable::True);
    quote::quote! {
        #postgresql_type_ident_where_element_token_stream
        #postgresql_type_std_option_option_ident_where_element_token_stream
    }
}
enum IsValueTypePub {
    True,
    False,
}
impl IsValueTypePub {
    fn maybe_pub_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            Self::True => {
                let value = naming::PubSnakeCase;
                quote::quote!{#value}
            },
            Self::False => proc_macro2::TokenStream::new()
        }
    }
}
trait WhereOperatorName {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens;
}
enum WhereOperatorType<'a> {
    Ident(&'a syn::Ident),
    FieldType {
        field_type: &'a syn::Type,
        default_initialization_token_stream: &'a dyn quote::ToTokens,
    },
}
impl WhereOperatorType<'_> {
    fn type_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(value) => quote::quote!{#value},
            WhereOperatorType::FieldType {
                field_type,
                ..
            } => quote::quote!{#field_type},
        }
    }
    fn additional_bind_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(_) => quote::quote!{.0},
            WhereOperatorType::FieldType { .. } => proc_macro2::TokenStream::new(),
        }
    }
    fn default_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(_) => quote::quote!{
                crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
            },
            WhereOperatorType::FieldType {
                field_type: _,
                default_initialization_token_stream
            } => quote::quote!{#default_initialization_token_stream},
        }
    }
}
struct Equal;
impl WhereOperatorName for Equal {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::EqualUpperCamelCase
    }
}
impl Equal {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
        where_operator_type: &WhereOperatorType,
    ) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let where_operator_type_type_token_stream = where_operator_type.type_token_stream();
        let where_operator_type_additional_bind_token_stream = where_operator_type.additional_bind_token_stream();
        let default_initialization_token_stream = where_operator_type.default_initialization_token_stream();
        generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            Self::upper_camel_case(),
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #where_operator_type_type_token_stream},
            &quote::quote!{#value_snake_case: #default_initialization_token_stream},
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} = ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case #where_operator_type_additional_bind_token_stream);
                #query_snake_case
            }
        )
    }
}
struct GreaterThan;
impl WhereOperatorName for GreaterThan {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::GreaterThanUpperCamelCase
    }
}
impl GreaterThan {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
        where_operator_type: &WhereOperatorType,
    ) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let where_operator_type_type_token_stream = where_operator_type.type_token_stream();
        let where_operator_type_additional_bind_token_stream = where_operator_type.additional_bind_token_stream();
        let default_initialization_token_stream = where_operator_type.default_initialization_token_stream();
        generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            Self::upper_camel_case(),
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #where_operator_type_type_token_stream},
            &quote::quote!{#value_snake_case: #default_initialization_token_stream},
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} > ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case #where_operator_type_additional_bind_token_stream);
                #query_snake_case
            }
        )
    }
}
enum BetweenTryNewErrorType {
    StartMoreOrEqualToEnd,
    StartIsEqualToEnd
}
impl BetweenTryNewErrorType {
    fn try_new_error_named_upper_camel_case_token_stream(&self) -> &'static dyn quote::ToTokens {
        match self {
            Self::StartMoreOrEqualToEnd => &naming::StartMoreOrEqualToEndUpperCamelCase,
            Self::StartIsEqualToEnd => &naming::StartIsEqualToEndUpperCamelCase,
        }
    }
    fn try_new_error_named_compare_symbol_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::StartMoreOrEqualToEnd => quote::quote!{<},
            Self::StartIsEqualToEnd => quote::quote!{!=},
        }
    }
}
enum ShouldAddDotZero {
    True,
    False,
}
impl ShouldAddDotZero {
    fn maybe_dot_zero_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            Self::True => quote::quote!{.0},
            Self::False => proc_macro2::TokenStream::new(),
        }
    }
}
struct Between;
impl WhereOperatorName for Between {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::BetweenUpperCamelCase
    }
}
impl Between {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
        where_operator_type: &WhereOperatorType,
        between_try_new_error_type: &BetweenTryNewErrorType,
        should_add_dot_zero: &ShouldAddDotZero,
    ) -> proc_macro2::TokenStream {
        let increment_snake_case = naming::IncrementSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let start_more_or_equal_to_end_upper_camel_case = naming::StartMoreOrEqualToEndUpperCamelCase;
        let start_snake_case = naming::StartSnakeCase;
        let end_snake_case = naming::EndSnakeCase;
        let where_operator_type_type_token_stream = where_operator_type.type_token_stream();
        let where_operator_type_additional_bind_token_stream = where_operator_type.additional_bind_token_stream();
        let default_initialization_token_stream = where_operator_type.default_initialization_token_stream();
        let self_upper_camel_case = Self::upper_camel_case();
        let try_new_error_named_upper_camel_case_token_stream = between_try_new_error_type.try_new_error_named_upper_camel_case_token_stream();
        let try_new_error_named_compare_symbol_token_stream = between_try_new_error_type.try_new_error_named_compare_symbol_token_stream();
        let maybe_dot_zero_token_stream = should_add_dot_zero.maybe_dot_zero_token_stream();
        generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &self_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::False {
                ident: &ident,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &quote::quote!{
                    #try_new_error_named_upper_camel_case_token_stream {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #start_snake_case: #where_operator_type_type_token_stream,
                        #[eo_to_std_string_string_serialize_deserialize]
                        #end_snake_case: #where_operator_type_type_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                },
                try_new_additional_input_parameters_token_stream: &quote::quote!{
                    #start_snake_case: #where_operator_type_type_token_stream,
                    #end_snake_case: #where_operator_type_type_token_stream
                },
                try_new_content_token_stream: &{
                    let postgresql_type_ident_where_element_between_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementBetweenTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
                    quote::quote!{
                        if 
                            #start_snake_case
                            #where_operator_type_additional_bind_token_stream
                            #maybe_dot_zero_token_stream
                            #try_new_error_named_compare_symbol_token_stream
                            #end_snake_case
                            #where_operator_type_additional_bind_token_stream
                            #maybe_dot_zero_token_stream
                        {
                            Ok(Self {
                                logical_operator,
                                #start_snake_case,
                                #end_snake_case
                            })
                        }
                        else {
                            Err(#postgresql_type_ident_where_element_between_try_new_error_named_upper_camel_case::#try_new_error_named_upper_camel_case_token_stream {
                                #start_snake_case,
                                #end_snake_case,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            })
                        }
                    }
                },
                impl_deserialize_token_stream: &{
                    let postgresql_type_ident_where_element_between_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementBetweenUpperCamelCase::from_tokens(&ident);
                    let (
                        struct_postgresql_type_ident_where_element_between_double_quotes_token_stream,
                        struct_postgresql_type_ident_where_element_between_with_3_elements_double_quotes_token_stream,
                        postgresql_type_ident_where_element_between_double_quotes_token_stream
                    ) = generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_ident_where_element_between_upper_camel_case, 3, &self_upper_camel_case);
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
                                                #where_operator_type_type_token_stream,
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
                                                #where_operator_type_type_token_stream,
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
                                            let mut __field1: _serde::__private::Option<#where_operator_type_type_token_stream> = _serde::__private::None;
                                            let mut __field2: _serde::__private::Option<#where_operator_type_type_token_stream> = _serde::__private::None;
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
                                                                #where_operator_type_type_token_stream,
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
                                                                #where_operator_type_type_token_stream,
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
                },
            },
            &quote::quote!{
                #start_snake_case: #where_operator_type_type_token_stream,
                #end_snake_case: #where_operator_type_type_token_stream
            },
            &quote::quote!{
                #start_snake_case: #default_initialization_token_stream,
                #end_snake_case: #default_initialization_token_stream,
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(first_value) => {
                        *#increment_snake_case = first_value;
                        match #increment_snake_case.checked_add(1) {
                            Some(second_value) => {
                                *#increment_snake_case = second_value;
                                let between_snake_case = naming::BetweenSnakeCase;
                                let and_snake_case = naming::AndSnakeCase;
                                Ok(format!("{}({column} {between_snake_case} ${first_value} {and_snake_case} ${second_value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
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
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#start_snake_case #where_operator_type_additional_bind_token_stream);
                #query_snake_case = #query_snake_case.bind(self.#end_snake_case #where_operator_type_additional_bind_token_stream);
                #query_snake_case
            }
        )
    }
}
struct In;
impl WhereOperatorName for In {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::InUpperCamelCase
    }
}
impl In {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
        where_operator_type: &WhereOperatorType,
    ) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let is_empty_upper_camel_case = naming::IsEmptyUpperCamelCase;
        let not_unique_upper_camel_case = naming::NotUniqueUpperCamelCase;
        let element_snake_case = naming::ElementSnakeCase;
        let acc_snake_case = naming::AccSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let query_snake_case = naming::QuerySnakeCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let self_upper_camel_case = Self::upper_camel_case();
        let where_operator_type_type_token_stream = where_operator_type.type_token_stream();
        let where_operator_type_additional_bind_token_stream = where_operator_type.additional_bind_token_stream();
        let default_initialization_token_stream = where_operator_type.default_initialization_token_stream();
        generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &self_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::False {
                ident: &ident,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &quote::quote!{
                    #is_empty_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #not_unique_upper_camel_case {
                        #[eo_to_std_string_string_serialize_deserialize]
                        #value_snake_case: #where_operator_type_type_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                },
                try_new_additional_input_parameters_token_stream: &quote::quote!{
                    #value_snake_case: std::vec::Vec<#where_operator_type_type_token_stream>
                },
                try_new_content_token_stream: &{
                    let postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementInTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
                    quote::quote!{
                        if #value_snake_case.is_empty() {
                            return Err(#postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case::#is_empty_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        {
                            let mut acc = vec![];
                            for element in &#value_snake_case {
                                if !acc.contains(&element) {
                                    acc.push(element);
                                } else {
                                    return Err(#postgresql_type_ident_where_element_in_try_new_error_named_upper_camel_case::#not_unique_upper_camel_case {
                                        #value_snake_case: element.clone(),
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
                },
                impl_deserialize_token_stream: &{
                    let postgresql_type_ident_where_element_in_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementInUpperCamelCase::from_tokens(&ident);
                    let (
                        struct_postgresql_type_ident_where_element_in_double_quotes_token_stream,
                        struct_postgresql_type_ident_where_element_in_with_2_elements_double_quotes_token_stream,
                        postgresql_type_ident_where_element_in_double_quotes_token_stream
                    ) = generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_ident_where_element_in_upper_camel_case, 2, &self_upper_camel_case);
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
                                                std::vec::Vec<#where_operator_type_type_token_stream>,
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
                                                std::vec::Vec<#where_operator_type_type_token_stream>,
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
                                                                std::vec::Vec<#where_operator_type_type_token_stream>,
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
                },
            },
            &quote::quote!{
                value: std::vec::Vec<#where_operator_type_type_token_stream>
            },
            &quote::quote!{
                value: vec![#default_initialization_token_stream]
            },
            &quote::quote!{
                let mut #acc_snake_case = std::string::String::default();
                for #element_snake_case in &self.#value_snake_case {
                    match #increment_snake_case.checked_add(1) {
                        Some(#value_snake_case) => {
                            *#increment_snake_case = #value_snake_case;
                            #acc_snake_case.push_str(&format!("${},", #value_snake_case));
                        },
                        None => {
                            return Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                }
                let _ = #acc_snake_case.pop();
                let in_snake_case = naming::InSnakeCase;
                Ok(format!(
                    "{}({} {in_snake_case} ({}))",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                    #acc_snake_case
                ))
            },
            &quote::quote!{
                for #element_snake_case in self.#value_snake_case {
                    #query_snake_case = #query_snake_case.bind(#element_snake_case #where_operator_type_additional_bind_token_stream);
                }
                #query_snake_case
            }
        )
    }
}
enum RegularExpression {
    CaseSensitive,
    CaseInsensitive
}
impl RegularExpression {
    fn stringified(&self) -> &'static str {
        match &self {
            RegularExpression::CaseSensitive => "",
            RegularExpression::CaseInsensitive => "*",
        }
    }
}
fn generate_regular_expression_postgresql_type_tokens_where_element_variant_handle_token_stream(
    ident: &dyn quote::ToTokens,
    is_nullable: &IsNullable,
    regular_expression: &RegularExpression,
    self_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let query_snake_case = naming::QuerySnakeCase;
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let case_stringified = regular_expression.stringified();
    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}({{}} ~{case_stringified} ${{}})"));
    generate_postgresql_type_tokens_where_element_variant_token_stream(
        &ident,
        &self_upper_camel_case,
        &is_nullable,
        ShouldWhereElementFieldsBePublic::True,
        &quote::quote!{pub value: std::string::String},
        &{
            let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
            quote::quote!{value: #core_default_default_default}
        },
        &quote::quote!{
            match #increment_snake_case.checked_add(1) {
                Some(#value_snake_case) => {
                    *#increment_snake_case = #value_snake_case;
                    Ok(format!(
                        #format_handle_token_stream,
                        &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                        #column_snake_case,
                        #increment_snake_case
                    ))
                },
                None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            }
        },
        &quote::quote!{
            #query_snake_case = #query_snake_case.bind(self.#value_snake_case);
            #query_snake_case
        }
    )
}
struct CaseSensitiveRegularExpression;
impl WhereOperatorName for CaseSensitiveRegularExpression {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::CaseSensitiveRegularExpressionUpperCamelCase
    }
}
impl CaseSensitiveRegularExpression {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        generate_regular_expression_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &RegularExpression::CaseSensitive,
            Self::upper_camel_case(),
        )
    }
}
struct CaseInsensitiveRegularExpression;
impl WhereOperatorName for CaseInsensitiveRegularExpression {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::CaseInsensitiveRegularExpressionUpperCamelCase
    }
}
impl CaseInsensitiveRegularExpression {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        generate_regular_expression_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &RegularExpression::CaseInsensitive,
            Self::upper_camel_case(),
        )
    }
}
struct Before;
impl WhereOperatorName for Before {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::BeforeUpperCamelCase
    }
}
impl Before {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let query_snake_case = naming::QuerySnakeCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            Self::upper_camel_case(),
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} < ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        )
    }
}
struct CurrentDate;
impl WhereOperatorName for CurrentDate {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::CurrentDateUpperCamelCase
    }
}
impl CurrentDate {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            Self::upper_camel_case(),
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} = current_date)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &quote::quote!{
                #query_snake_case
            }
        )
    }
}
struct GreaterThanCurrentDate;
impl WhereOperatorName for GreaterThanCurrentDate {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::GreaterThanCurrentDateUpperCamelCase
    }
}
impl GreaterThanCurrentDate {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            Self::upper_camel_case(),
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} > current_date)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &quote::quote!{
                #query_snake_case
            }
        )
    }
}
struct CurrentTimestamp;
impl WhereOperatorName for CurrentTimestamp {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::CurrentTimestampUpperCamelCase
    }
}
impl CurrentTimestamp {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            Self::upper_camel_case(),
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} = current_timestamp)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &quote::quote!{
                #query_snake_case
            }
        )
    }
}
struct GreaterThanCurrentTimestamp;
impl WhereOperatorName for GreaterThanCurrentTimestamp {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::GreaterThanCurrentTimestampUpperCamelCase
    }
}
impl GreaterThanCurrentTimestamp {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            Self::upper_camel_case(),
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} > current_timestamp)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &quote::quote!{
                #query_snake_case
            }
        )
    }
}
struct CurrentTime;
impl WhereOperatorName for CurrentTime {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::CurrentTimeUpperCamelCase
    }
}
impl CurrentTime {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            Self::upper_camel_case(),
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} = current_time)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &quote::quote!{
                #query_snake_case
            }
        )
    }
}
struct GreaterThanCurrentTime;
impl WhereOperatorName for GreaterThanCurrentTime {
    fn upper_camel_case() -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::GreaterThanCurrentTimeUpperCamelCase
    }
}
impl GreaterThanCurrentTime {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            Self::upper_camel_case(),
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{},
            &quote::quote!{},
            &quote::quote!{
                Ok(format!(
                    "{}({} > current_time)",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    #column_snake_case,
                ))
            },
            &quote::quote!{
                #query_snake_case
            }
        )
    }
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementNumber)]
pub fn postgresql_base_type_tokens_where_element_number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
        };
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_in_token_stream = In::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &Between::upper_camel_case(),
                &In::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_in_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementNumber",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxPostgresTypesPgMoney)]
pub fn postgresql_base_type_tokens_sqlx_postgres_types_pg_money(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &{
            let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
            quote::quote!{sqlx::postgres::types::PgMoney(#core_default_default_default)}
        }
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgMoney)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_money(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::True,
        );
        let postgresql_type_tokens_where_element_in_token_stream = In::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &Between::upper_camel_case(),
                &In::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_in_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgMoney",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesDecimal)]
pub fn postgresql_base_type_tokens_sqlx_types_decimal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &token_patterns::CoreDefaultDefaultDefault
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesDecimal)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_decimal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
        };
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &Between::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesDecimal",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesBigDecimal)]
pub fn postgresql_base_type_tokens_sqlx_types_big_decimal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &token_patterns::CoreDefaultDefaultDefault
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesBigDecimal)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_big_decimal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &Between::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesBigDecimal",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementBool)]
pub fn postgresql_base_type_tokens_where_element_bool(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &WhereOperatorType::FieldType {
                field_type: &field_type,
                default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
            },
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementBool",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementStdStringString)]
pub fn postgresql_base_type_tokens_where_element_std_string_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = CaseSensitiveRegularExpression::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = CaseInsensitiveRegularExpression::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &CaseSensitiveRegularExpression::upper_camel_case(),
                &CaseInsensitiveRegularExpression::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream
            #postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementStdStringString",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensStdVecVecStdPrimitiveU8)]
pub fn postgresql_base_type_tokens_std_vec_vec_std_primitive_u8(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &{
            let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
            quote::quote!{vec![#core_default_default_default]}
        }
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementStdVecVecStdPrimitiveU8)]
pub fn postgresql_base_type_tokens_where_element_std_vec_vec_std_primitive_u8(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let increment_snake_case = naming::IncrementSnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &WhereOperatorType::FieldType {
                field_type: &field_type,
                default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
            },
        );
        let length_more_than_upper_camel_case = naming::LengthMoreThanUpperCamelCase;
        let postgresql_type_tokens_where_element_length_more_than_token_stream = {
            let length_is_negative_upper_camel_case = naming::LengthIsNegativeUpperCamelCase;
            let std_primitive_i64_token_stream = quote::quote!{std::primitive::i64};
            generate_postgresql_type_tokens_where_element_variant_token_stream(
                &ident,
                &length_more_than_upper_camel_case,
                &is_nullable,
                ShouldWhereElementFieldsBePublic::False {
                    ident: &ident,
                    postfix: &length_more_than_upper_camel_case,
                    try_new_error_named_variants_token_stream: &quote::quote!{
                        #length_is_negative_upper_camel_case {
                            #[eo_to_std_string_string_serialize_deserialize]
                            #value_snake_case: #std_primitive_i64_token_stream,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        },
                    },
                    try_new_additional_input_parameters_token_stream: &quote::quote!{
                        length_more_than: #std_primitive_i64_token_stream,
                    },
                    try_new_content_token_stream: &{
                        let postgresql_type_ident_where_element_length_more_than_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementLengthMoreThanTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
                        quote::quote!{
                            if length_more_than > 0 {
                                Ok(Self{
                                    logical_operator,
                                    length_more_than
                                })
                            }
                            else {
                                Err(#postgresql_type_ident_where_element_length_more_than_try_new_error_named_upper_camel_case::#length_is_negative_upper_camel_case {
                                    #value_snake_case: length_more_than,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                })
                            }
                        }
                    },
                    impl_deserialize_token_stream: &{
                        let postgresql_type_ident_where_element_length_more_than_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementLengthMoreThanUpperCamelCase::from_tokens(&ident);
                        let (
                            struct_postgresql_type_ident_where_element_length_more_than_double_quotes_token_stream,
                            struct_postgresql_type_ident_where_element_length_more_than_with_2_elements_double_quotes_token_stream,
                            postgresql_type_ident_where_element_length_more_than_double_quotes_token_stream
                        ) = generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_ident_where_element_length_more_than_upper_camel_case, 2, &length_more_than_upper_camel_case);
                        quote::quote! {
                            const _: () = {
                                #[allow(unused_extern_crates, clippy::useless_attribute)]
                                extern crate serde as _serde;
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for #postgresql_type_ident_where_element_length_more_than_upper_camel_case {
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
                                                    "length_more_than" => _serde::__private::Ok(__Field::__field1),
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
                                                    b"length_more_than" => _serde::__private::Ok(__Field::__field1),
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
                                                #postgresql_type_ident_where_element_length_more_than_upper_camel_case,
                                            >,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = #postgresql_type_ident_where_element_length_more_than_upper_camel_case;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::__private::Formatter<'_>,
                                            ) -> _serde::__private::fmt::Result {
                                                _serde::__private::Formatter::write_str(
                                                    __formatter,
                                                    #struct_postgresql_type_ident_where_element_length_more_than_double_quotes_token_stream,
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
                                                                &#struct_postgresql_type_ident_where_element_length_more_than_with_2_elements_double_quotes_token_stream,
                                                            ),
                                                        );
                                                    }
                                                };
                                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                                    #std_primitive_i64_token_stream,
                                                >(&mut __seq)? {
                                                    _serde::__private::Some(__value) => __value,
                                                    _serde::__private::None => {
                                                        return _serde::__private::Err(
                                                            _serde::de::Error::invalid_length(
                                                                1usize,
                                                                &#struct_postgresql_type_ident_where_element_length_more_than_with_2_elements_double_quotes_token_stream,
                                                            ),
                                                        );
                                                    }
                                                };
                                                match #postgresql_type_ident_where_element_length_more_than_upper_camel_case::try_new(__field0, __field1) {
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
                                                let mut __field1: _serde::__private::Option<#std_primitive_i64_token_stream> = _serde::__private::None;
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
                                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                                        "length_more_than",
                                                                    ),
                                                                );
                                                            }
                                                            __field1 = _serde::__private::Some(
                                                                _serde::de::MapAccess::next_value::<
                                                                    #std_primitive_i64_token_stream,
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
                                                        _serde::__private::de::missing_field("length_more_than")?
                                                    }
                                                };
                                                match #postgresql_type_ident_where_element_length_more_than_upper_camel_case::try_new(__field0, __field1) {
                                                    Ok(value) => _serde::__private::Ok(value),
                                                    Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                                }
                                            }
                                        }
                                        #[doc(hidden)]
                                        const FIELDS: &'static [&'static str] = &[
                                            "logical_operator",
                                            "length_more_than",
                                        ];
                                        _serde::Deserializer::deserialize_struct(
                                            __deserializer,
                                            #postgresql_type_ident_where_element_length_more_than_double_quotes_token_stream,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::__private::PhantomData::<
                                                    #postgresql_type_ident_where_element_length_more_than_upper_camel_case,
                                                >,
                                                lifetime: _serde::__private::PhantomData,
                                            },
                                        )
                                    }
                                }
                            };
                        }
                    },
                },
                &quote::quote!{length_more_than: #std_primitive_i64_token_stream,},
                &{
                    let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                    quote::quote!{length_more_than: #core_default_default_default}
                },
                &quote::quote!{
                    match #increment_snake_case.checked_add(1) {
                        Some(#value_snake_case) => {
                            *#increment_snake_case = #value_snake_case;
                            Ok(format!("{}(length({}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), #column_snake_case, #increment_snake_case))
                        }
                        None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
                    }
                },
                &quote::quote!{
                    #query_snake_case = #query_snake_case.bind(self.length_more_than);
                    #query_snake_case
                }
            )
        };
        let equal_to_encoded_string_representation_upper_camel_case = naming::EqualToEncodedStringRepresentationUpperCamelCase;
        let postgresql_type_tokens_where_element_equal_to_encoded_string_representation_token_stream = generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &equal_to_encoded_string_representation_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{
                pub encode_format: EncodeFormat,
                pub encoded_string_representation: std::string::String,
            },
            &{
                let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                quote::quote!{
                    encode_format: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    encoded_string_representation: #core_default_default_default,
                }
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!("{}(encode({}, '{}') = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), #column_snake_case, &self.encode_format, #increment_snake_case))
                    }
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.encoded_string_representation);
                #query_snake_case
            }
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &length_more_than_upper_camel_case,
                &equal_to_encoded_string_representation_upper_camel_case,
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_length_more_than_token_stream
            #postgresql_type_tokens_where_element_equal_to_encoded_string_representation_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementStdVecVecStdPrimitiveU8",
    //         &generated,
    //     );
    // }
    generated.into()
}
fn sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream() -> proc_macro2::TokenStream {
    let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
    quote::quote!{
        sqlx::types::time::Date::from_ordinal_date(
            #core_default_default_default,
            1,
        ).unwrap()//todo 
    }
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesTimeDate)]
pub fn postgresql_base_type_tokens_sqlx_types_time_date(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream()
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeDate)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_time_date(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_current_date_token_stream = CurrentDate::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_greater_than_current_date_token_stream = GreaterThanCurrentDate::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &Between::upper_camel_case(),
                &CurrentDate::upper_camel_case(),
                &GreaterThanCurrentDate::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_current_date_token_stream
            #postgresql_type_tokens_where_element_greater_than_current_date_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeDate",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesChronoNaiveDate)]
pub fn postgresql_base_type_tokens_sqlx_types_chrono_naive_date(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &token_patterns::CoreDefaultDefaultDefault
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesChronoNaiveDate)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_chrono_naive_date(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
        };
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_current_date_token_stream = CurrentDate::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_greater_than_current_date_token_stream = GreaterThanCurrentDate::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &Between::upper_camel_case(),
                &CurrentDate::upper_camel_case(),
                &GreaterThanCurrentDate::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_current_date_token_stream
            #postgresql_type_tokens_where_element_greater_than_current_date_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesChronoNaiveDate",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesChronoNaiveTime)]
pub fn postgresql_base_type_tokens_sqlx_types_chrono_naive_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &token_patterns::CoreDefaultDefaultDefault
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesChronoNaiveTime)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_chrono_naive_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
        };
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_current_time_token_stream = CurrentTime::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_greater_than_current_time_token_stream = GreaterThanCurrentTime::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &Between::upper_camel_case(),
                &CurrentTime::upper_camel_case(),
                &GreaterThanCurrentTime::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_current_time_token_stream
            #postgresql_type_tokens_where_element_greater_than_current_time_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesChronoNaiveTime",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesTimeTime)]
pub fn postgresql_base_type_tokens_sqlx_types_time_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &token_patterns::SqlxTypesTimeTimeMidnight,
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeTime)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_time_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::SqlxTypesTimeTimeMidnight,
        };
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_current_time_token_stream = CurrentTime::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_greater_than_current_time_token_stream = GreaterThanCurrentTime::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &Between::upper_camel_case(),
                &CurrentTime::upper_camel_case(),
                &GreaterThanCurrentTime::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_current_time_token_stream
            #postgresql_type_tokens_where_element_greater_than_current_time_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeTime",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxPostgresTypesPgInterval)]
pub fn postgresql_base_type_tokens_sqlx_postgres_types_pg_interval(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &{
            let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
            quote::quote!{sqlx::postgres::types::PgInterval {
                months: #core_default_default_default,
                days: #core_default_default_default,
                microseconds: #core_default_default_default,
            }}
        }
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgInterval)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_interval(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &BetweenTryNewErrorType::StartIsEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &Between::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgInterval",
    //         &generated,
    //     );
    // }
    generated.into()
}
fn generate_sqlx_postgres_types_pg_range_token_steram(
    start_token_stream: &dyn quote::ToTokens,
    end_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote!{sqlx::postgres::types::PgRange {
        start: std::ops::Bound::Included(#start_token_stream),
        end: std::ops::Bound::Excluded(#end_token_stream),
    }}
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxPostgresTypesPgRangeDefaultInitialization)]
pub fn postgresql_base_type_tokens_sqlx_postgres_types_pg_range(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &{
            let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
            generate_sqlx_postgres_types_pg_range_token_steram(
                &core_default_default_default,
                &core_default_default_default,
            )
        }
    )
}
enum RangeType {
    I32,
    I64,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
    SqlxPostgresTypesPgRangeSqlxTypesDecimal,
    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
    SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
}
impl RangeType {
    fn type_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            Self::I32 => quote::quote!{std::primitive::i32},
            Self::I64 => quote::quote!{std::primitive::i64},
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc => quote::quote!{sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>},
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal => quote::quote!{sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>},
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => quote::quote!{sqlx::types::chrono::NaiveDateTime},
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => quote::quote!{sqlx::types::chrono::NaiveDate},
            Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal => quote::quote!{sqlx::types::Decimal},
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => quote::quote!{SqlxTypesTimeOffsetDateTime},
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => quote::quote!{SqlxTypesTimePrimitiveDateTime},
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => quote::quote!{SqlxTypesTimeDate},
            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => quote::quote!{SqlxTypesBigDecimal},
        }
    }
    fn should_impl_range_length(&self) -> ShouldImplRangeLength {
        match &self {
            Self::I32 => ShouldImplRangeLength::True,
            Self::I64 => ShouldImplRangeLength::True,
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate |
            Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate |
            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => ShouldImplRangeLength::False,
        }
    }
    fn default_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            Self::I32 |
            Self::I64 |
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate |
            Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal => {
                let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                quote::quote!{#core_default_default_default}
            },
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate |
            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => quote::quote!{
                crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
            },
        }
    }
    fn postgresql_type_self_where_bind_value_to_query_parameter_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            Self::I32 |
            Self::I64 |
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc |
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate |
            Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal => proc_macro2::TokenStream::new(),
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime |
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate |
            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => quote::quote!{.0},
        }
    }
}
enum ShouldImplRangeLength {
    True,
    False
}
fn generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
    input: proc_macro::TokenStream,
    range_type: RangeType,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let range_type_token_stream = range_type.type_token_stream();
        let range_type_should_impl_range_length = range_type.should_impl_range_length();
        let range_type_default_initialization_token_stream = range_type.default_initialization_token_stream();
        let range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream = range_type.postgresql_type_self_where_bind_value_to_query_parameter_token_stream();
        let increment_snake_case = naming::IncrementSnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &WhereOperatorType::Ident(&ident),
        );
        let value_is_contained_within_range_upper_camel_case = naming::ValueIsContainedWithinRangeUpperCamelCase;
        let postgresql_type_tokens_where_element_value_is_contained_within_range_token_stream = generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &value_is_contained_within_range_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #range_type_token_stream},
            &quote::quote!{#value_snake_case: #range_type_default_initialization_token_stream},
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} @> ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case #range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream);
                #query_snake_case
            }
        );
        let contains_another_range_upper_camel_case = naming::ContainsAnotherRangeUpperCamelCase;
        let postgresql_type_tokens_where_element_contains_another_range_token_stream = generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &contains_another_range_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} @> ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        );
        let strictly_to_left_of_range_upper_camel_case = naming::StrictlyToLeftOfRangeUpperCamelCase;
        let postgresql_type_tokens_where_element_strictly_to_left_of_range_token_stream = generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &strictly_to_left_of_range_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} &< ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        );
        let strictly_to_right_of_range_upper_camel_case = naming::StrictlyToRightOfRangeUpperCamelCase;
        let postgresql_type_tokens_where_element_strictly_to_right_of_range_token_stream = generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &strictly_to_right_of_range_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} &> ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        );
        let included_lower_bound_upper_camel_case = naming::IncludedLowerBoundUpperCamelCase;
        let postgresql_type_tokens_where_element_included_lower_bound_token_stream = generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &included_lower_bound_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #range_type_token_stream},
            &quote::quote!{#value_snake_case: #range_type_default_initialization_token_stream},
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}(lower({}) = ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case #range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream);
                #query_snake_case
            }
        );
        let excluded_upper_bound_upper_camel_case = naming::ExcludedUpperBoundUpperCamelCase;
        let postgresql_type_tokens_where_element_excluded_upper_bound_token_stream = generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &excluded_upper_bound_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #range_type_token_stream},
            &quote::quote!{#value_snake_case: #range_type_default_initialization_token_stream},
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}(upper({}) = ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case #range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream);
                #query_snake_case
            }
        );
        let greater_than_lower_bound_upper_camel_case = naming::GreaterThanLowerBoundUpperCamelCase;
        let postgresql_type_tokens_where_element_greater_than_lower_bound_token_stream = generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &greater_than_lower_bound_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} > ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        );
        let overlap_with_range_upper_camel_case = naming::OverlapWithRangeUpperCamelCase;
        let postgresql_type_tokens_where_element_overlap_with_range_token_stream = generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &overlap_with_range_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} && ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        );
        let adjacent_with_range_upper_camel_case = naming::AdjacentWithRangeUpperCamelCase;
        let postgresql_type_tokens_where_element_adjacent_with_range_token_stream = generate_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &adjacent_with_range_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::True,
            &quote::quote!{pub #value_snake_case: #ident},
            &quote::quote!{
                #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
            },
            &quote::quote!{
                match #increment_snake_case.checked_add(1) {
                    Some(#value_snake_case) => {
                        *#increment_snake_case = #value_snake_case;
                        Ok(format!(
                            "{}({} -|- ${})",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                            #increment_snake_case
                        ))
                    },
                    None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            &quote::quote!{
                #query_snake_case = #query_snake_case.bind(self.#value_snake_case.0);
                #query_snake_case
            }
        );
        //todo find out maximum length of range(INT8RANGE, INT4RANGE) in postgresql
        let range_length_upper_camel_case = naming::RangeLengthUpperCamelCase;
        let maybe_postgresql_type_tokens_where_element_range_length_token_stream = match &range_type_should_impl_range_length {
            ShouldImplRangeLength::True => {
                let length_is_negative_or_zero_upper_camel_case = naming::LengthIsNegativeOrZeroUpperCamelCase;
                let std_primitive_i64_token_stream = quote::quote!{std::primitive::i64};
                generate_postgresql_type_tokens_where_element_variant_token_stream(
                    &ident,
                    &range_length_upper_camel_case,
                    &is_nullable,
                    ShouldWhereElementFieldsBePublic::False {
                        ident: &ident,
                        postfix: &range_length_upper_camel_case,
                        try_new_error_named_variants_token_stream: &quote::quote!{
                            #length_is_negative_or_zero_upper_camel_case {
                                #[eo_to_std_string_string_serialize_deserialize]
                                #value_snake_case: #std_primitive_i64_token_stream,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                        },
                        try_new_additional_input_parameters_token_stream: &quote::quote!{
                            #value_snake_case: #std_primitive_i64_token_stream
                        },
                        try_new_content_token_stream: &{
                            let postgresql_type_ident_where_element_range_length_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementRangeLengthTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
                            quote::quote!{
                                if #value_snake_case > 0 {
                                    Ok(Self {
                                        logical_operator,
                                        #value_snake_case,
                                    })
                                }
                                else {
                                    Err(#postgresql_type_ident_where_element_range_length_try_new_error_named_upper_camel_case::#length_is_negative_or_zero_upper_camel_case {
                                        #value_snake_case,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    })
                                }
                            }
                        },
                        impl_deserialize_token_stream: &{
                            let postgresql_type_ident_where_element_range_length_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementRangeLengthUpperCamelCase::from_tokens(&ident);
                            let (
                                struct_postgresql_type_ident_where_element_range_length_double_quotes_token_stream,
                                struct_postgresql_type_ident_where_element_range_length_with_2_elements_double_quotes_token_stream,
                                postgresql_type_ident_where_element_range_length_double_quotes_token_stream
                            ) = generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_ident_where_element_range_length_upper_camel_case, 2, &range_length_upper_camel_case);
                            quote::quote! {
                                const _: () = {
                                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                                    extern crate serde as _serde;
                                    #[automatically_derived]
                                    impl<'de> _serde::Deserialize<'de> for #postgresql_type_ident_where_element_range_length_upper_camel_case {
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
                                                    #postgresql_type_ident_where_element_range_length_upper_camel_case,
                                                >,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = #postgresql_type_ident_where_element_range_length_upper_camel_case;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter,
                                                ) -> _serde::__private::fmt::Result {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        #struct_postgresql_type_ident_where_element_range_length_double_quotes_token_stream,
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
                                                                    &#struct_postgresql_type_ident_where_element_range_length_with_2_elements_double_quotes_token_stream,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                                        #std_primitive_i64_token_stream,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    1usize,
                                                                    &#struct_postgresql_type_ident_where_element_range_length_with_2_elements_double_quotes_token_stream,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    match #postgresql_type_ident_where_element_range_length_upper_camel_case::try_new(__field0, __field1) {
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
                                                    let mut __field1: _serde::__private::Option<#std_primitive_i64_token_stream> = _serde::__private::None;
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
                                                                        #std_primitive_i64_token_stream,
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
                                                    match #postgresql_type_ident_where_element_range_length_upper_camel_case::try_new(__field0, __field1) {
                                                        Ok(value) => _serde::__private::Ok(value),
                                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                                    }
                                                }
                                            }
                                            #[doc(hidden)]
                                            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
                                            _serde::Deserializer::deserialize_struct(
                                                __deserializer,
                                                #postgresql_type_ident_where_element_range_length_double_quotes_token_stream,
                                                FIELDS,
                                                __Visitor {
                                                    marker: _serde::__private::PhantomData::<
                                                        #postgresql_type_ident_where_element_range_length_upper_camel_case,
                                                    >,
                                                    lifetime: _serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                };
                            }
                        },
                    },
                    &quote::quote!{#value_snake_case: #std_primitive_i64_token_stream},//todo try_new - check length > 0
                    &{
                        let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                        quote::quote!{#value_snake_case: #core_default_default_default}
                    },
                    &quote::quote!{
                        match #increment_snake_case.checked_add(1) {
                            Some(#value_snake_case) => {
                                *#increment_snake_case = #value_snake_case;
                                Ok(format!(
                                    "{}(upper({}) - lower({}) = ${})",
                                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                    #column_snake_case,
                                    #column_snake_case,
                                    #increment_snake_case
                                ))
                            },
                            None => Err(crate::#try_generate_bind_increments_error_named_upper_camel_case::#checked_add_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            })
                        }
                    },
                    &quote::quote!{
                        #query_snake_case = #query_snake_case.bind(self.#value_snake_case);
                        #query_snake_case
                    }
                )
            },
            ShouldImplRangeLength::False => proc_macro2::TokenStream::new(), 
        };
        let equal_upper_camel_case = Equal::upper_camel_case();
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &{
                let mut value: std::vec::Vec<&dyn quote::ToTokens> = vec![
                    &equal_upper_camel_case,
                    &value_is_contained_within_range_upper_camel_case,
                    &contains_another_range_upper_camel_case,
                    &strictly_to_left_of_range_upper_camel_case,
                    &strictly_to_right_of_range_upper_camel_case,
                    &included_lower_bound_upper_camel_case,
                    &excluded_upper_bound_upper_camel_case,
                    &greater_than_lower_bound_upper_camel_case,
                    &overlap_with_range_upper_camel_case,
                    &adjacent_with_range_upper_camel_case,
                ];
                if let ShouldImplRangeLength::True = &range_type_should_impl_range_length {
                    value.push(&range_length_upper_camel_case);
                }
                value
            }
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_value_is_contained_within_range_token_stream
            #postgresql_type_tokens_where_element_contains_another_range_token_stream
            #postgresql_type_tokens_where_element_strictly_to_left_of_range_token_stream
            #postgresql_type_tokens_where_element_strictly_to_right_of_range_token_stream
            #postgresql_type_tokens_where_element_included_lower_bound_token_stream
            #postgresql_type_tokens_where_element_excluded_upper_bound_token_stream
            #postgresql_type_tokens_where_element_greater_than_lower_bound_token_stream
            #postgresql_type_tokens_where_element_overlap_with_range_token_stream
            #postgresql_type_tokens_where_element_adjacent_with_range_token_stream
            #maybe_postgresql_type_tokens_where_element_range_length_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeStdPrimitiveI32OrI64",
    //         &generated,
    //     );
    // }
    generated.into()
} 
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeStdPrimitiveI32)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_std_primitive_i32(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
        input,
        RangeType::I32,
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeStdPrimitiveI64)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_std_primitive_i64(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
        input,
        RangeType::I64,
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
        input,
        RangeType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
    )
}
fn sqlx_types_time_primitive_date_time_new_token_stream() -> proc_macro2::TokenStream {
    let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
    let sqlx_types_time_time_midnight = token_patterns::SqlxTypesTimeTimeMidnight;
    let sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream = sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream();
    quote::quote!{sqlx::types::time::PrimitiveDateTime::new(
        #sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
        #sqlx_types_time_time_midnight,
    )}
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime)]
pub fn postgresql_base_type_tokens_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &{
            let sqlx_types_time_primitive_date_time_new_token_stream = sqlx_types_time_primitive_date_time_new_token_stream();
            generate_sqlx_postgres_types_pg_range_token_steram(
                &sqlx_types_time_primitive_date_time_new_token_stream,
                &sqlx_types_time_primitive_date_time_new_token_stream,
            )
        }
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
        input,
        RangeType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
        input,
        RangeType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
        input,
        RangeType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime)]
pub fn postgresql_base_type_tokens_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &{
            let sqlx_types_time_offset_date_time_unix_epoch = token_patterns::SqlxTypesTimeOffsetDateTimeUnixEpoch;
            generate_sqlx_postgres_types_pg_range_token_steram(
                &sqlx_types_time_offset_date_time_unix_epoch,
                &sqlx_types_time_offset_date_time_unix_epoch,
            )
        }
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
        input,
        RangeType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
        input,
        RangeType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxPostgresTypesPgRangeSqlxTypesTimeDate)]
pub fn postgresql_base_type_tokens_sqlx_postgres_types_pg_range_sqlx_types_time_date(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &{
            let sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream = sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream();
            generate_sqlx_postgres_types_pg_range_token_steram(
                &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
            )
        }
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesTimeDate)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_sqlx_types_time_date(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
        input,
        RangeType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesDecimal)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_sqlx_types_decimal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
        input,
        RangeType::SqlxPostgresTypesPgRangeSqlxTypesDecimal,
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesBigDecimal)]
pub fn postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_sqlx_types_big_decimal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens_where_element_sqlx_postgres_types_pg_range_tokens(
        input,
        RangeType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesChronoNaiveDateTime)]
pub fn postgresql_base_type_tokens_sqlx_types_chrono_naive_date_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &token_patterns::CoreDefaultDefaultDefault
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesChronoNaiveDateTime)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_chrono_naive_date_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
        };
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_current_timestamp_token_stream = CurrentTimestamp::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream = GreaterThanCurrentTimestamp::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &Between::upper_camel_case(),
                &CurrentTimestamp::upper_camel_case(),
                &GreaterThanCurrentTimestamp::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_current_timestamp_token_stream
            #postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesChronoNaiveDateTime",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesTimePrimitiveDateTime)]
pub fn postgresql_base_type_tokens_sqlx_types_time_primitive_date_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &sqlx_types_time_primitive_date_time_new_token_stream()
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesTimePrimitiveDateTime)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_time_primitive_date_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let sqlx_types_time_time_midnight = token_patterns::SqlxTypesTimeTimeMidnight;
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &sqlx_types_time_primitive_date_time_new_token_stream(),
        };
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_current_timestamp_token_stream = CurrentTimestamp::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream = GreaterThanCurrentTimestamp::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &Between::upper_camel_case(),
                &CurrentTimestamp::upper_camel_case(),
                &GreaterThanCurrentTimestamp::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_current_timestamp_token_stream
            #postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesTimePrimitiveDateTime",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesTimeOffsetDateTime)]
pub fn postgresql_base_type_tokens_sqlx_types_time_offset_date_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &token_patterns::SqlxTypesTimeOffsetDateTimeUnixEpoch
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeOffsetDateTime)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_time_offset_date_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let postgresql_type_tokens_where_element_before_token_stream = Before::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        // todo
        // -- Values after the current timestamp
        // SELECT *
        // FROM your_table
        // WHERE your_timestamptz_column > CURRENT_TIMESTAMP;

        // -- Values equal to the current date (ignoring time)
        // SELECT *
        // FROM your_table
        // WHERE your_timestamptz_column::date = CURRENT_DATE;
        // 6. Time Zone Conversion
        // You can also use AT TIME ZONE to convert the TIMESTAMPTZ to a different time zone for comparison. This is useful when you want to perform comparisons based on different time zones.

        // -- Compare with a specific timestamp in another time zone
        // SELECT *
        // FROM your_table
        // WHERE your_timestamptz_column AT TIME ZONE 'UTC' = '2024-12-30 14:30:00+00';

        // -- Values after a timestamp in a different time zone
        // SELECT *
        // FROM your_table
        // WHERE your_timestamptz_column AT TIME ZONE 'America/New_York' > '2024-12-30 14:30:00';
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &Before::upper_camel_case(),
                &Between::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_before_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeOffsetDateTime",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesChronoDateTimeSqlxTypesChronoUtc)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_utc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let postgresql_type_tokens_where_element_before_token_stream = Before::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &Before::upper_camel_case(),
                &Between::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_before_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesChronoDateTimeSqlxTypesChronoUtc",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesChronoDateTimeSqlxTypesChronoLocal)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_local(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let postgresql_type_tokens_where_element_before_token_stream = Before::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_between_token_stream = Between::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &Before::upper_camel_case(),
                &Between::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_before_token_stream
            #postgresql_type_tokens_where_element_between_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesChronoDateTimeSqlxTypesChronoLocal",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesUuidUuid)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_uuid_uuid(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &WhereOperatorType::Ident(&ident),
        );
        let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = CaseSensitiveRegularExpression::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = CaseInsensitiveRegularExpression::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &CaseSensitiveRegularExpression::upper_camel_case(),
                &CaseInsensitiveRegularExpression::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream
            #postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesUuidUuid",
    //         &generated,
    //     );
    // }
    generated.into()
}
fn std_net_ip_addr_v4_std_net_ipv4_addr_unspecified_token_stream() -> proc_macro2::TokenStream {
    quote::quote! {std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED)}
}
#[proc_macro_derive(PostgresqlBaseTypeTokensStdNetIpAddr)]
pub fn postgresql_base_type_tokens_std_net_ip_addr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &std_net_ip_addr_v4_std_net_ipv4_addr_unspecified_token_stream()
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementStdNetIpAddr)]
pub fn postgresql_base_type_tokens_where_element_std_net_ip_addr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &WhereOperatorType::FieldType {
                field_type: &field_type,
                default_initialization_token_stream: &std_net_ip_addr_v4_std_net_ipv4_addr_unspecified_token_stream(),
            },
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementStdNetIpAddr",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesMacAddressMacAddress)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_mac_address_mac_address(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let postgresql_type_tokens_where_element_greater_than_token_stream = GreaterThan::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident
        );
        let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = CaseSensitiveRegularExpression::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = CaseInsensitiveRegularExpression::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &GreaterThan::upper_camel_case(),
                &CaseSensitiveRegularExpression::upper_camel_case(),
                &CaseInsensitiveRegularExpression::upper_camel_case(),
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_greater_than_token_stream
            #postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream
            #postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesMacAddressMacAddress",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesBitVec)]
pub fn postgresql_base_type_tokens_sqlx_types_bit_vec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &quote::quote!{{
            let mut value = sqlx::types::BitVec::new();
            value.push(false);
            value
        }}
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesBitVec)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_bit_vec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let increment_snake_case = naming::IncrementSnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
        let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &WhereOperatorType::Ident(&ident),
        );
        let position_equals_upper_camel_case = naming::PositionEqualsUpperCamelCase;
        let postgresql_type_tokens_where_element_position_equals_token_stream = {
            let std_primitive_bool_token_stream = quote::quote!{std::primitive::bool};
            let std_primitive_i32_token_stream = quote::quote!{std::primitive::i32};
            generate_postgresql_type_tokens_where_element_variant_token_stream(
                &ident,
                &position_equals_upper_camel_case,
                &is_nullable,
                {
                    let position_is_less_or_equal_zero_upper_camel_case = naming::PositionIsLessOrEqualZeroUpperCamelCase;
                    ShouldWhereElementFieldsBePublic::False {
                        ident: &ident,
                        postfix: &position_equals_upper_camel_case,
                        try_new_error_named_variants_token_stream: &quote::quote!{
                            #position_is_less_or_equal_zero_upper_camel_case {
                                #[eo_to_std_string_string_serialize_deserialize]
                                position: #std_primitive_i32_token_stream,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            },
                        },
                        try_new_additional_input_parameters_token_stream: &quote::quote!{
                            #value_snake_case: #std_primitive_bool_token_stream,
                            position: #std_primitive_i32_token_stream,
                        },
                        try_new_content_token_stream: &{
                            let postgresql_type_ident_where_element_position_equals_try_new_error_named_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementPositionEqualsTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
                            quote::quote!{
                                if position > 0 {
                                    Ok(Self {
                                        logical_operator,
                                        #value_snake_case,
                                        position,
                                    })
                                }
                                else {
                                    Err(#postgresql_type_ident_where_element_position_equals_try_new_error_named_upper_camel_case::#position_is_less_or_equal_zero_upper_camel_case {
                                        position,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    })
                                }
                            }
                        },
                        impl_deserialize_token_stream: &{
                            let postgresql_type_ident_where_element_position_equals_upper_camel_case = naming::parameter::PostgresqlTypeSelfWhereElementPositionEqualsUpperCamelCase::from_tokens(&ident);
                            let (
                                struct_postgresql_type_ident_where_element_position_equals_double_quotes_token_stream,
                                struct_postgresql_type_ident_where_element_position_equals_with_2_elements_double_quotes_token_stream,
                                postgresql_type_ident_where_element_position_equals_double_quotes_token_stream
                            ) = generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_ident_where_element_position_equals_upper_camel_case, 2, &position_equals_upper_camel_case);
                            quote::quote! {
                                const _: () = {
                                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                                    extern crate serde as _serde;
                                    #[automatically_derived]
                                    impl<'de> _serde::Deserialize<'de>
                                    for #postgresql_type_ident_where_element_position_equals_upper_camel_case {
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
                                                        "value" => _serde::__private::Ok(__Field::__field1),
                                                        "position" => _serde::__private::Ok(__Field::__field2),
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
                                                        b"position" => _serde::__private::Ok(__Field::__field2),
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
                                                    #postgresql_type_ident_where_element_position_equals_upper_camel_case,
                                                >,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = #postgresql_type_ident_where_element_position_equals_upper_camel_case;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private::Formatter,
                                                ) -> _serde::__private::fmt::Result {
                                                    _serde::__private::Formatter::write_str(
                                                        __formatter,
                                                        #struct_postgresql_type_ident_where_element_position_equals_double_quotes_token_stream,
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
                                                                    &#struct_postgresql_type_ident_where_element_position_equals_with_2_elements_double_quotes_token_stream,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                                        #std_primitive_bool_token_stream,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    1usize,
                                                                    &#struct_postgresql_type_ident_where_element_position_equals_with_2_elements_double_quotes_token_stream,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    let __field2 = match _serde::de::SeqAccess::next_element::<
                                                        #std_primitive_i32_token_stream,
                                                    >(&mut __seq)? {
                                                        _serde::__private::Some(__value) => __value,
                                                        _serde::__private::None => {
                                                            return _serde::__private::Err(
                                                                _serde::de::Error::invalid_length(
                                                                    2usize,
                                                                    &#struct_postgresql_type_ident_where_element_position_equals_with_2_elements_double_quotes_token_stream,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    match #postgresql_type_ident_where_element_position_equals_upper_camel_case::try_new(__field0, __field1, __field2) {
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
                                                    let mut __field1: _serde::__private::Option<#std_primitive_bool_token_stream> = _serde::__private::None;
                                                    let mut __field2: _serde::__private::Option<#std_primitive_i32_token_stream> = _serde::__private::None;
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
                                                                        #std_primitive_bool_token_stream,
                                                                    >(&mut __map)?,
                                                                );
                                                            }
                                                            __Field::__field2 => {
                                                                if _serde::__private::Option::is_some(&__field2) {
                                                                    return _serde::__private::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                                            "position",
                                                                        ),
                                                                    );
                                                                }
                                                                __field2 = _serde::__private::Some(
                                                                    _serde::de::MapAccess::next_value::<
                                                                        #std_primitive_i32_token_stream,
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
                                                    let __field2 = match __field2 {
                                                        _serde::__private::Some(__field2) => __field2,
                                                        _serde::__private::None => {
                                                            _serde::__private::de::missing_field("position")?
                                                        }
                                                    };
                                                    match #postgresql_type_ident_where_element_position_equals_upper_camel_case::try_new(__field0, __field1, __field2) {
                                                        Ok(value) => _serde::__private::Ok(value),
                                                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                                                    }
                                                }
                                            }
                                            #[doc(hidden)]
                                            const FIELDS: &'static [&'static str] = &[
                                                "logical_operator",
                                                "value",
                                                "position",
                                            ];
                                            _serde::Deserializer::deserialize_struct(
                                                __deserializer,
                                                #postgresql_type_ident_where_element_position_equals_double_quotes_token_stream,
                                                FIELDS,
                                                __Visitor {
                                                    marker: _serde::__private::PhantomData::<
                                                        #postgresql_type_ident_where_element_position_equals_upper_camel_case,
                                                    >,
                                                    lifetime: _serde::__private::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                };
                            }
                        },
                    }
                },
                &quote::quote!{
                    #value_snake_case: #std_primitive_bool_token_stream,
                    position: #std_primitive_i32_token_stream,
                },
                &{
                    let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                    quote::quote!{
                        #value_snake_case: #core_default_default_default,
                        position: #core_default_default_default,
                    }
                },
                &quote::quote!{
                    match #increment_snake_case.checked_add(1) {
                        Some(first_increment) => {
                            *#increment_snake_case = first_increment;
                            match #increment_snake_case.checked_add(1) {
                                Some(second_increment) => {
                                    *#increment_snake_case = second_increment;
                                    Ok(format!(
                                        "{}(substring({}::text from ${}::int4 for 1::int4) = ${})",
                                        &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                                        #column_snake_case,
                                        first_increment,
                                        second_increment,
                                    ))
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
                },
                &quote::quote!{
                    #query_snake_case = #query_snake_case.bind(self.position);
                    #query_snake_case = #query_snake_case.bind(if self.#value_snake_case {
                        "1"
                    }
                    else {
                        "0"
                    });
                    #query_snake_case
                }
            )
        };
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &Equal::upper_camel_case(),
                &position_equals_upper_camel_case,
            ]
        );
        quote::quote! {
            #maybe_postgresql_type_tokens_where_element_is_null_token_stream
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_position_equals_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesBitVec",
    //         &generated,
    //     );
    // }
    generated.into()
}
// fn sqlx_types_ipnetwork_ip_network_v4_token_stream() -> proc_macro2::TokenStream {
//     let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
//     quote::quote!{sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::new(core::net::Ipv4Addr::UNSPECIFIED, #core_default_default_default).unwrap())}
// }
// // todo mismatched types; Rust type `postgresql_crud_common::postgresql_type::postgresql_type::PostgresqlTypeSqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNullToRead` (as SQL type `INET`) is not compatible with SQL type `CIDR`
// #[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesIpnetworkIpNetwork)]
// pub fn postgresql_base_type_tokens_sqlx_types_ipnetwork_ip_network(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     generate_postgresql_base_type_tokens(
//         input,
//         &sqlx_types_ipnetwork_ip_network_v4_token_stream()
//     )
// }
// #[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesIpnetworkIpNetwork)]
// pub fn postgresql_base_type_tokens_where_element_sqlx_types_ipnetwork_ip_network(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     panic_location::panic_location();
//     let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
//     let ident = &syn_derive_input.ident;
//     let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
//     let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
//         let maybe_postgresql_type_tokens_where_element_is_null_token_stream = is_nullable.maybe_generate_postgresql_type_std_option_option_tokens_where_element_is_null_token_stream(&ident);
//         let postgresql_type_tokens_where_element_equal_token_stream = Equal::generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
//             &ident,
//             &is_nullable,
//             &WhereOperatorType::FieldType {
//                 field_type: &field_type,
//                 default_initialization_token_stream: &sqlx_types_ipnetwork_ip_network_v4_token_stream()
//             },
//         );
//         let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
//             is_nullable,
//             &ident,
//             &vec![
//                 &Equal::upper_camel_case(),
//             ]
//         );
//         quote::quote! {
//             #maybe_postgresql_type_tokens_where_element_is_null_token_stream
//             #postgresql_type_tokens_where_element_equal_token_stream
//             #postgresql_type_tokens_where_element_token_stream
//         }
//     });
//     // if ident == "" {
//     //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
//     //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesIpnetworkIpNetwork",
//     //         &generated,
//     //     );
//     // }
//     generated.into()
// }