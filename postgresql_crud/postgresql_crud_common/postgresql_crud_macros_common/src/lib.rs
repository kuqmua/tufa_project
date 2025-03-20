pub fn generate_postgresql_json_type_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    ident: &dyn quote::ToTokens,
    postgresql_json_type_ident_to_create_token_stream: &dyn quote::ToTokens,
    create_query_part_token_stream: &dyn quote::ToTokens,
    create_query_bind_token_stream: &dyn quote::ToTokens,
    postgresql_json_type_ident_select: &dyn quote::ToTokens,
    postgresql_json_type_ident_read: &dyn quote::ToTokens,
    select_query_part_token_stream: &dyn quote::ToTokens,
    postgresql_json_type_ident_where_element_token_stream: &dyn quote::ToTokens,
    postgresql_json_type_ident_option_to_update: &dyn quote::ToTokens,
    postgresql_json_type_ident_option_to_update_try_generate_postgresql_json_type_error_named: &dyn quote::ToTokens,
    update_query_part_token_stream: &dyn quote::ToTokens,
    update_query_bind_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let value_snake_case = naming::ValueSnakeCase;
    let select_upper_camel_case = naming::SelectUpperCamelCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    let where_element_upper_camel_case = naming::WhereElementUpperCamelCase;
    let update_upper_camel_case = naming::UpdateUpperCamelCase;
    let update_query_part_error_named_upper_camel_case = naming::UpdateQueryPartErrorNamedUpperCamelCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
    let query_snake_case = naming::QuerySnakeCase;
    let field_ident_snake_case = naming::FieldIdentSnakeCase;
    let column_name_and_maybe_field_getter_snake_case = naming::ColumnNameAndMaybeFieldGetterSnakeCase;
    let column_name_and_maybe_field_getter_for_error_message_snake_case = naming::ColumnNameAndMaybeFieldGetterForErrorMessageSnakeCase;
    let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
    let jsonb_set_target_snake_case = naming::JsonbSetTargetSnakeCase;
    let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;
    let create_query_part_error_named_upper_camel_case = naming::CreateQueryPartErrorNamedUpperCamelCase;
    let create_query_part_snake_case = naming::CreateQueryPartSnakeCase;
    let create_query_bind_snake_case = naming::CreateQueryBindSnakeCase;
    let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
    let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
    let reference_std_primitive_str_token_stream = quote::quote! {&std::primitive::str};
    let reference_mut_std_primitive_u64_token_stream = quote::quote! {&mut std::primitive::u64};
    let mut_query_sqlx_query_postgres_arguments_token_stream = quote::quote! {mut #query_snake_case: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let query_postgres_arguments_token_stream = quote::quote! {sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let std_string_string_token_stream = token_patterns::StdStringString;
    //todo maybe reexport sqlx?
    quote::quote! {
        impl #path_token_stream #postgresql_json_type_upper_camel_case for #ident {
            type #create_upper_camel_case<'a> = #postgresql_json_type_ident_to_create_token_stream;
            fn #create_query_part_snake_case(
                #value_snake_case: &Self::#create_upper_camel_case<'_>,
                #increment_snake_case: #reference_mut_std_primitive_u64_token_stream
            ) -> Result<#std_string_string_token_stream, #path_token_stream #create_query_part_error_named_upper_camel_case> {
                #create_query_part_token_stream
            }
            fn #create_query_bind_snake_case<'a>(
                #value_snake_case: Self::#create_upper_camel_case<'a>,
                #mut_query_sqlx_query_postgres_arguments_token_stream
            ) -> #query_postgres_arguments_token_stream {
                #create_query_bind_token_stream
            }
            type #select_upper_camel_case<'a> = #postgresql_json_type_ident_select;
            fn #select_query_part_snake_case(
                #value_snake_case: &Self::#select_upper_camel_case<'_>,
                #field_ident_snake_case: #reference_std_primitive_str_token_stream,
                #column_name_and_maybe_field_getter_snake_case: #reference_std_primitive_str_token_stream,
                #column_name_and_maybe_field_getter_for_error_message_snake_case: #reference_std_primitive_str_token_stream,
                is_postgresql_type: std::primitive::bool,
            ) -> #std_string_string_token_stream {
                #select_query_part_token_stream
            }
            type #where_element_upper_camel_case<'a> = #postgresql_json_type_ident_where_element_token_stream;
            type #read_upper_camel_case<'a> = #postgresql_json_type_ident_read;
            type #update_upper_camel_case<'a> = #postgresql_json_type_ident_option_to_update;
            type #update_query_part_error_named_upper_camel_case = #postgresql_json_type_ident_option_to_update_try_generate_postgresql_json_type_error_named;
            fn #update_query_part_snake_case(
                #value_snake_case: &Self::#update_upper_camel_case<'_>,
                #jsonb_set_accumulator_snake_case: #reference_std_primitive_str_token_stream,
                #jsonb_set_target_snake_case: #reference_std_primitive_str_token_stream,
                #jsonb_set_path_snake_case: #reference_std_primitive_str_token_stream,
                #increment_snake_case: #reference_mut_std_primitive_u64_token_stream,
            ) -> Result<#std_string_string_token_stream, Self::#update_query_part_error_named_upper_camel_case> {
                #update_query_part_token_stream
            }
            fn #update_query_bind_snake_case<'a>(
                #value_snake_case: Self::#update_upper_camel_case<'_>,
                #mut_query_sqlx_query_postgres_arguments_token_stream
            ) -> #query_postgres_arguments_token_stream {
                #update_query_bind_token_stream
            }
        }
    }
}

pub enum PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement {
    CrateGeneratePostgresqlJsonType,
    PostgresqlCrud,
}
impl PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::CrateGeneratePostgresqlJsonType => &token_patterns::CrateGeneratePostgresqlJsonTypeDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
            PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::PostgresqlCrud => &token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
        }
    }
    fn all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::CrateGeneratePostgresqlJsonType => &token_patterns::CrateGeneratePostgresqlJsonTypeAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
            PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::PostgresqlCrud => &token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
        }
    }
}
fn generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    path_default_but_option_is_always_some_and_vec_always_contains_one_element: &PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
    ident: &dyn quote::ToTokens,
    lifetime_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let path_trait_token_stream = path_default_but_option_is_always_some_and_vec_always_contains_one_element.default_but_option_is_always_some_and_vec_always_contains_one_element();
    let default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementSnakeCase;
    quote::quote! {
        impl #path_trait_token_stream for #ident #lifetime_token_stream {
            fn #default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case() -> Self {
                #content_token_stream
            }
        }
    }
}
fn generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    path_default_but_option_is_always_some_and_vec_always_contains_one_element: &PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let path_trait_token_stream = path_default_but_option_is_always_some_and_vec_always_contains_one_element.all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element();
    let all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::AllEnumVariantsArrayDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
    quote::quote! {
        impl #path_trait_token_stream for #ident {
            fn #all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case() -> std::vec::Vec<Self> {
                #content_token_stream
            }
        }
    }
}

pub fn generate_impl_crate_generate_postgresql_json_type_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
        &PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::CrateGeneratePostgresqlJsonType,
        ident,
        &proc_macro2::TokenStream::new(),
        content_token_stream
    )
}
pub fn generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    lifetime_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
        &PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::PostgresqlCrud,
        ident,
        lifetime_token_stream,
        content_token_stream
    )
}
pub fn generate_impl_crate_generate_postgresql_json_type_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::CrateGeneratePostgresqlJsonType, ident, content_token_stream)
}
pub fn generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::PostgresqlCrud, ident, content_token_stream)
}

pub enum PostgresqlTypeSelfWhereFilterPath {
    Crate,
    PostgresqlCrud,
}
impl quote::ToTokens for PostgresqlTypeSelfWhereFilterPath  {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::Crate => quote::quote!{crate},
            Self::PostgresqlCrud => quote::quote!{postgresql_crud},
        }.to_tokens(tokens)
    }
}
pub fn impl_postgresql_type_self_where_filter_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    where_query_part_content_token_stream: &dyn quote::ToTokens,
    where_query_bind_content_token_stream: &dyn quote::ToTokens,
    postgresql_type_self_where_filter_path: &PostgresqlTypeSelfWhereFilterPath,
) -> proc_macro2::TokenStream {
    let increment_snake_case = naming::IncrementSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let std_primitive_u64_token_stream = token_patterns::StdPrimitiveU64;
    let std_fmt_display_token_stream = token_patterns::StdFmtDisplay;
    let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;
    let std_string_string_token_stream = token_patterns::StdStringString;
    let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
    let where_query_part_snake_case = naming::WhereQueryPartSnakeCase;
    let where_query_bind_snake_case = naming::WhereQueryBindSnakeCase;
    let postgresql_type_self_where_filter_upper_camel_case = naming::PostgresqlTypeSelfWhereFilterUpperCamelCase;
    quote::quote!{
        impl #postgresql_type_self_where_filter_path ::postgresql_type::postgresql_type_trait::#postgresql_type_self_where_filter_upper_camel_case<'_> for #ident_token_stream {
            fn #where_query_part_snake_case(
                &self,
                #increment_snake_case: &mut #std_primitive_u64_token_stream,
                #column_snake_case: &dyn #std_fmt_display_token_stream,
                #is_need_to_add_logical_operator_snake_case: #std_primitive_bool_token_stream
            ) -> Result<#std_string_string_token_stream, #postgresql_type_self_where_filter_path::#query_part_error_named_upper_camel_case> {
                #where_query_part_content_token_stream
            }
            fn #where_query_bind_snake_case<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #where_query_bind_content_token_stream
            }
        }
    }
}