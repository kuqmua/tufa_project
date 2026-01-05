mod filters;

pub use filters::*;

#[derive(Debug, Clone)]
pub enum DeriveOrImpl {
    Derive,
    Impl(proc_macro2::TokenStream),
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::Display,
    strum_macros::EnumIter,
    enum_extension_lib::EnumExtension,
)]
pub enum NotNullOrNullable {
    #[default]
    NotNull,
    Nullable,
}
impl NotNullOrNullable {
    pub fn rust(&self) -> &'static dyn std::fmt::Display {
        match &self {
            Self::NotNull => &"",
            Self::Nullable => &naming::OptionUpperCamelCase,
        }
    }
    pub fn maybe_option_wrap(
        &self,
        content_token_stream: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        match &self {
            Self::NotNull => content_token_stream,
            Self::Nullable => quote::quote! {Option<#content_token_stream>},
        }
    }
    pub fn maybe_some_wrap(
        &self,
        content_token_stream: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        match &self {
            Self::NotNull => content_token_stream,
            Self::Nullable => quote::quote! {Some(#content_token_stream)},
        }
    }
    //json
    pub fn prefix_stringified(&self) -> String {
        match &self {
            Self::NotNull => String::default(),
            Self::Nullable => String::from("StdOptionOption"),
        }
    }
}

pub fn generate_postgresql_type_where_token_stream(
    variants: &Vec<&dyn PostgresqlFilter>,
    prefix: &dyn quote::ToTokens,
    should_derive_utoipa_to_schema: &ShouldDeriveUtoipaToSchema,
    should_derive_schemars_json_schema: &ShouldDeriveSchemarsJsonSchema,
    is_query_bind_mutable: &IsQueryBindMutable,
) -> proc_macro2::TokenStream {
    let ident = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&prefix);
    let value_snake_case = naming::ValueSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let postgresql_type_tokens_where_token_stream = {
        let variants_token_stream = variants.iter().map(|element| {
            let element_upper_camel_case = element.upper_camel_case();
            let prefix_where_self_upper_camel_case = element.prefix_where_self_upper_camel_case();
            let option_type_token_stream: Option<proc_macro2::TokenStream> = element.maybe_generic();
            let type_token_stream = option_type_token_stream.map_or_else(proc_macro2::TokenStream::new, |value| quote::quote! {<#value>});
            quote::quote! {#element_upper_camel_case(where_filters::#prefix_where_self_upper_camel_case #type_token_stream)}
        });
        quote::quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_derive_utoipa_to_schema #should_derive_schemars_json_schema)]
            pub enum #ident {
                #(#variants_token_stream),*
            }
        }
    };
    let impl_postgresql_type_postgresql_type_where_filter_for_postgresql_type_tokens_where_token_stream =
        impl_postgresql_type_where_filter_for_ident_token_stream(
            &quote::quote! {<'lifetime>},
            &ident,
            &proc_macro2::TokenStream::new(),
            &IncrementParameterUnderscore::False,
            &ColumnParameterUnderscore::False,
            &IsNeedToAddLogicalOperatorUnderscore::False,
            &{
                let variants_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.upper_camel_case();
                quote::quote! {
                    Self::#element_upper_camel_case(#value_snake_case) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(
                        #value_snake_case,
                        #increment_snake_case,
                        #column_snake_case,
                        #is_need_to_add_logical_operator_snake_case,
                    )
                }
            });
                quote::quote! {
                    match &self {
                        #(#variants_token_stream),*
                    }
                }
            },
            is_query_bind_mutable,
            &{
                let variants_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.upper_camel_case();
                quote::quote! {
                    Self::#element_upper_camel_case(#value_snake_case) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(
                        #value_snake_case,
                        #query_snake_case
                    )
                }
            });
                quote::quote! {
                    match self {
                        #(#variants_token_stream),*
                    }
                }
            },
            &ImportPath::PostgresqlCrudCommon,
        );
    let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_token_stream =
        macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(
            &proc_macro2::TokenStream::new(),
            &ident,
            &proc_macro2::TokenStream::new(),
            &quote::quote! {format!("{self:#?}")},
        );
    let impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_tokens_where_token_stream = generate_impl_postgresql_crud_common_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident, &{
        let variants_token_stream = variants.iter().map(|element| {
            let element_upper_camel_case = element.upper_camel_case();
            let default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
            quote::quote! {
                Self::#element_upper_camel_case(#default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
            }
        });
        quote::quote! {vec![#(#variants_token_stream),*]}
    });
    quote::quote! {
        #postgresql_type_tokens_where_token_stream
        #impl_postgresql_type_postgresql_type_where_filter_for_postgresql_type_tokens_where_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_token_stream
        #impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_tokens_where_token_stream
    }
}

pub fn postgresql_crud_common_query_part_error_named_token_stream() -> proc_macro2::TokenStream {
    let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
    quote::quote! {postgresql_crud_common::#query_part_error_named_upper_camel_case}
}

pub fn generate_struct_ident_double_quotes_token_stream(
    value: &dyn std::fmt::Display,
) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_token_stream(&format!("struct {value}"))
}
pub fn generate_struct_ident_with_number_elements_double_quotes_token_stream(
    ident: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    length: usize,
) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_token_stream(&format!("struct {ident} with {length} elements"))
}
pub fn generate_tuple_struct_ident_double_quotes_token_stream(
    value: &dyn std::fmt::Display,
) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_token_stream(&format!("tuple struct {value}"))
}

pub fn generate_sqlx_types_json_type_declaration_token_stream(
    type_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {sqlx::types::Json<#type_token_stream>}
}
pub fn generate_std_option_option_tokens_declaration_token_stream(
    type_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {Option<#type_token_stream>}
}
pub fn generate_std_vec_vec_tokens_declaration_token_stream(
    type_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {Vec<#type_token_stream>}
}

pub fn generate_serde_deserialize_double_quotes_token_stream(
    ident: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    length: usize,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    let struct_postgresql_type_ident_where_tokens_double_quotes_token_stream =
        generate_struct_ident_double_quotes_token_stream(ident);
    let struct_postgresql_type_ident_where_tokens_with_number_elements_double_quotes_token_stream =
        generate_struct_ident_with_number_elements_double_quotes_token_stream(ident, length);
    let postgresql_type_ident_where_tokens_double_quotes_token_stream =
        generate_quotes::double_quotes_token_stream(&ident);
    (
        struct_postgresql_type_ident_where_tokens_double_quotes_token_stream,
        struct_postgresql_type_ident_where_tokens_with_number_elements_double_quotes_token_stream,
        postgresql_type_ident_where_tokens_double_quotes_token_stream,
    )
}

#[derive(Debug, Clone, Copy)]
pub enum ShouldDeriveSchemarsJsonSchema {
    True,
    False,
}
impl quote::ToTokens for ShouldDeriveSchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {, schemars::JsonSchema}.to_tokens(tokens),
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ShouldDeriveUtoipaToSchema {
    True,
    False,
}
impl quote::ToTokens for ShouldDeriveUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {, utoipa::ToSchema}.to_tokens(tokens),
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IsCreateQueryPartSelfCreateUsed {
    True,
    False,
}
impl quote::ToTokens for IsCreateQueryPartSelfCreateUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::ValueSnakeCase.to_tokens(tokens),
            Self::False => quote::quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsCreateQueryBindMutable {
    True,
    False,
}
impl quote::ToTokens for IsCreateQueryBindMutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::MutSnakeCase.to_tokens(tokens),
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectQueryPartSelfSelectUsed {
    True,
    False,
}
impl quote::ToTokens for IsSelectQueryPartSelfSelectUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::ValueSnakeCase.to_tokens(tokens),
            Self::False => quote::quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed {
    True,
    False,
}
impl quote::ToTokens for IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => {
                naming::ColumnNameAndMaybeFieldGetterForErrorMessageSnakeCase.to_tokens(tokens);
            }
            Self::False => quote::quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectQueryPartIsPostgresqlTypeUsed {
    True,
    False,
}
impl quote::ToTokens for IsSelectQueryPartIsPostgresqlTypeUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {is_postgresql_type}.to_tokens(tokens),
            Self::False => quote::quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsUpdateQueryPartSelfUpdateUsed {
    True,
    False,
}
impl quote::ToTokens for IsUpdateQueryPartSelfUpdateUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::ValueSnakeCase.to_tokens(tokens),
            Self::False => quote::quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsUpdateQueryPartJsonbSetTargetUsed {
    True,
    False,
}
impl quote::ToTokens for IsUpdateQueryPartJsonbSetTargetUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::JsonbSetTargetSnakeCase.to_tokens(tokens),
            Self::False => quote::quote! {_}.to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IsUpdateQueryBindMutable {
    True,
    False,
}
impl quote::ToTokens for IsUpdateQueryBindMutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::MutSnakeCase.to_tokens(tokens),
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IsSelectOnlyUpdatedIdsQueryBindMutable {
    True,
    False,
}
impl quote::ToTokens for IsSelectOnlyUpdatedIdsQueryBindMutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::MutSnakeCase.to_tokens(tokens),
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IsSelectOnlyCreatedIdsQueryBindMutable {
    True,
    False,
}
impl quote::ToTokens for IsSelectOnlyCreatedIdsQueryBindMutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::MutSnakeCase.to_tokens(tokens),
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
        }
    }
}
pub fn generate_impl_postgresql_json_type_token_stream(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    table_type_declaration_type_token_stream: &dyn quote::ToTokens,
    create_type_token_stream: &dyn quote::ToTokens,
    create_for_query_type_token_stream: &dyn quote::ToTokens,
    select_type_token_stream: &dyn quote::ToTokens,
    is_select_query_part_self_select_used: &IsSelectQueryPartSelfSelectUsed,
    is_select_query_part_column_name_and_maybe_field_getter_for_error_message_used: &IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed,
    is_select_query_part_is_postgresql_type_used: &IsSelectQueryPartIsPostgresqlTypeUsed,
    select_query_part_token_stream: &dyn quote::ToTokens,
    where_type_token_stream: &dyn quote::ToTokens,
    read_type_token_stream: &dyn quote::ToTokens,
    read_only_ids_type_token_stream: &dyn quote::ToTokens,
    select_only_ids_query_part_token_stream: &dyn quote::ToTokens,
    read_inner_type_token_stream: &dyn quote::ToTokens,
    into_inner_token_stream: &dyn quote::ToTokens,
    update_type_token_stream: &dyn quote::ToTokens,
    update_type_for_query_token_stream: &dyn quote::ToTokens,
    update_query_part_token_stream: &dyn quote::ToTokens,
    is_update_query_part_self_update_used: &IsUpdateQueryPartSelfUpdateUsed,
    is_update_query_part_jsonb_set_target_used: &IsUpdateQueryPartJsonbSetTargetUsed,
    is_update_query_bind_mutable: &IsUpdateQueryBindMutable,
    update_query_bind_token_stream: &dyn quote::ToTokens,
    select_only_updated_ids_query_part_token_stream: &dyn quote::ToTokens,
    is_select_only_updated_ids_query_bind_mutable: &IsSelectOnlyUpdatedIdsQueryBindMutable,
    select_only_updated_ids_query_bind_token_stream: &dyn quote::ToTokens,
    select_only_created_ids_query_part_token_stream: &dyn quote::ToTokens,
    is_select_only_created_ids_query_bind_mutable: &IsSelectOnlyCreatedIdsQueryBindMutable,
    select_only_created_ids_query_bind_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let path_token_stream = quote::quote! {#import_path ::};
    let table_type_declaration_upper_camel_case = naming::TableTypeDeclarationUpperCamelCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let create_for_query_upper_camel_case = naming::CreateForQueryUpperCamelCase;
    let value_snake_case = naming::ValueSnakeCase;
    let select_upper_camel_case = naming::SelectUpperCamelCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    let select_only_ids_query_part_snake_case = naming::SelectOnlyIdsQueryPartSnakeCase;
    let read_inner_upper_camel_case = naming::ReadInnerUpperCamelCase;
    let where_upper_camel_case = naming::WhereUpperCamelCase;
    let update_upper_camel_case = naming::UpdateUpperCamelCase;
    let update_for_query_upper_camel_case = naming::UpdateForQueryUpperCamelCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
    let query_snake_case = naming::QuerySnakeCase;
    let field_ident_snake_case = naming::FieldIdentSnakeCase;
    let column_name_and_maybe_field_getter_snake_case =
        naming::ColumnNameAndMaybeFieldGetterSnakeCase;
    let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
    let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;
    let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
    let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
    let select_only_updated_ids_query_part_snake_case =
        naming::SelectOnlyUpdatedIdsQueryPartSnakeCase;
    let select_only_updated_ids_query_bind_snake_case =
        naming::SelectOnlyUpdatedIdsQueryBindSnakeCase;
    let select_only_created_ids_query_part_snake_case =
        naming::SelectOnlyCreatedIdsQueryPartSnakeCase;
    let select_only_created_ids_query_bind_snake_case =
        naming::SelectOnlyCreatedIdsQueryBindSnakeCase;
    let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
    let reference_std_primitive_str_token_stream = token_patterns::RefStdPrimitiveStr;
    let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;
    let reference_mut_std_primitive_u64_token_stream = {
        let std_primitive_u64_token_stream = token_patterns::StdPrimitiveU64;
        quote::quote! {&mut #std_primitive_u64_token_stream}
    };
    let std_string_string_token_stream = token_patterns::StdStringString;
    let std_primitive_u64_token_stream = token_patterns::StdPrimitiveU64;
    let query_postgres_arguments_token_stream =
        quote::quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let query_lifetime_postgres_arguments_token_stream =
        quote::quote! {sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>};
    //todo maybe reexport sqlx?
    quote::quote! {
        impl #path_token_stream #postgresql_json_type_upper_camel_case for #ident {
            type #table_type_declaration_upper_camel_case = #table_type_declaration_type_token_stream;
            type #create_upper_camel_case = #create_type_token_stream;
            type #create_for_query_upper_camel_case = #create_for_query_type_token_stream;
            type #select_upper_camel_case = #select_type_token_stream;
            fn #select_query_part_snake_case(
                #is_select_query_part_self_select_used: &Self::#select_upper_camel_case,
                #field_ident_snake_case: #reference_std_primitive_str_token_stream,
                #column_name_and_maybe_field_getter_snake_case: #reference_std_primitive_str_token_stream,
                #is_select_query_part_column_name_and_maybe_field_getter_for_error_message_used: #reference_std_primitive_str_token_stream,
                #is_select_query_part_is_postgresql_type_used: #std_primitive_bool_token_stream,
            ) -> Result<#std_string_string_token_stream, #path_token_stream #query_part_error_named_upper_camel_case> {
                #select_query_part_token_stream
            }
            type #where_upper_camel_case = #where_type_token_stream;
            type #read_upper_camel_case = #read_type_token_stream;
            type #read_only_ids_upper_camel_case = #read_only_ids_type_token_stream;
            fn #select_only_ids_query_part_snake_case(
                #column_name_and_maybe_field_getter_snake_case: #reference_std_primitive_str_token_stream,
            ) -> Result<#std_string_string_token_stream, #import_path ::#query_part_error_named_upper_camel_case> {
                #select_only_ids_query_part_token_stream
            }
            type #read_inner_upper_camel_case = #read_inner_type_token_stream;
            fn into_inner(#value_snake_case: Self::#read_upper_camel_case) -> Self::#read_inner_upper_camel_case {
                #into_inner_token_stream
            }
            type #update_upper_camel_case = #update_type_token_stream;
            type #update_for_query_upper_camel_case = #update_type_for_query_token_stream;
            fn #update_query_part_snake_case(
                #is_update_query_part_self_update_used: &Self::#update_for_query_upper_camel_case,
                #jsonb_set_accumulator_snake_case: #reference_std_primitive_str_token_stream,
                #is_update_query_part_jsonb_set_target_used: #reference_std_primitive_str_token_stream,
                #jsonb_set_path_snake_case: #reference_std_primitive_str_token_stream,
                #increment_snake_case: #reference_mut_std_primitive_u64_token_stream,
            ) -> Result<#std_string_string_token_stream, #path_token_stream #query_part_error_named_upper_camel_case> {
                #update_query_part_token_stream
            }
            fn #update_query_bind_snake_case(
                #value_snake_case: Self::#update_for_query_upper_camel_case,
                #is_update_query_bind_mutable #query_snake_case: #query_postgres_arguments_token_stream
            ) -> Result<#query_postgres_arguments_token_stream, #std_string_string_token_stream> {
                #update_query_bind_token_stream
            }
            fn #select_only_updated_ids_query_part_snake_case(
                #value_snake_case: &Self::#update_for_query_upper_camel_case,
                #field_ident_snake_case: #reference_std_primitive_str_token_stream,
                #column_name_and_maybe_field_getter_snake_case: #reference_std_primitive_str_token_stream,
                #increment_snake_case: &mut #std_primitive_u64_token_stream
            ) -> Result<#std_string_string_token_stream, #import_path ::#query_part_error_named_upper_camel_case> {
                #select_only_updated_ids_query_part_token_stream
            }
            fn #select_only_updated_ids_query_bind_snake_case<'lifetime>(
                #value_snake_case: &'lifetime Self::#update_for_query_upper_camel_case,
                #is_select_only_updated_ids_query_bind_mutable #query_snake_case: #query_lifetime_postgres_arguments_token_stream
            ) -> Result<#query_lifetime_postgres_arguments_token_stream, #std_string_string_token_stream> {
                #select_only_updated_ids_query_bind_token_stream
            }

            fn #select_only_created_ids_query_part_snake_case(
                #value_snake_case: &Self::#create_for_query_upper_camel_case,
                #field_ident_snake_case: #reference_std_primitive_str_token_stream,
                #column_name_and_maybe_field_getter_snake_case: #reference_std_primitive_str_token_stream,
                #increment_snake_case: &mut #std_primitive_u64_token_stream
            ) -> Result<#std_string_string_token_stream, #import_path ::#query_part_error_named_upper_camel_case> {
                #select_only_created_ids_query_part_token_stream
            }
            fn #select_only_created_ids_query_bind_snake_case<'lifetime>(
                #value_snake_case: &'lifetime Self::#create_for_query_upper_camel_case,
                #is_select_only_created_ids_query_bind_mutable #query_snake_case: #query_lifetime_postgres_arguments_token_stream
            ) -> Result<#query_lifetime_postgres_arguments_token_stream, #std_string_string_token_stream> {
                #select_only_created_ids_query_bind_token_stream
            }
        }
    }
}

pub fn generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    ident_generic_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let path_trait_token_stream =
        import_path.default_but_option_is_always_some_and_vec_always_contains_one_element();
    let default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case =
        naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementSnakeCase;
    quote::quote! {
        impl #impl_generic_token_stream #path_trait_token_stream for #ident #ident_generic_token_stream {
            fn #default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case() -> Self {
                #content_token_stream
            }
        }
    }
}
pub fn generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let path_trait_token_stream = import_path.all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element();
    let all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::AllEnumVariantsArrayDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
    quote::quote! {
        impl #path_trait_token_stream for #ident {
            fn #all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case() -> Vec<Self> {
                #content_token_stream
            }
        }
    }
}
pub fn generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    ident_generic_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let path_trait_token_stream = import_path
        .default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size();
    let default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_snake_case =
        naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSizeSnakeCase;
    quote::quote! {
        impl #impl_generic_token_stream #path_trait_token_stream for #ident #ident_generic_token_stream {
            fn #default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_snake_case() -> Self {
                #content_token_stream
            }
        }
    }
}
pub fn generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let path_trait_token_stream = import_path.all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size();
    let all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_with_max_page_size_snake_case = naming::AllEnumVariantsArrayDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementWithMaxPageSizeSnakeCase;
    quote::quote! {
        impl #path_trait_token_stream for #ident {
            fn #all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_with_max_page_size_snake_case() -> Vec<Self> {
                #content_token_stream
            }
        }
    }
}
pub fn generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&proc_macro2::TokenStream::new(), &ImportPath::PostgresqlCrudCommon, ident, &proc_macro2::TokenStream::new(), content_token_stream)
}
pub fn generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    lifetime_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&proc_macro2::TokenStream::new(), &ImportPath::PostgresqlCrud, ident, lifetime_token_stream, content_token_stream)
}
pub fn generate_impl_postgresql_crud_common_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ImportPath::PostgresqlCrudCommon, ident, content_token_stream)
}
pub fn generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ImportPath::PostgresqlCrud, ident, content_token_stream)
}

pub fn generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(&proc_macro2::TokenStream::new(), &ImportPath::PostgresqlCrudCommon, ident, &proc_macro2::TokenStream::new(), content_token_stream)
}
pub fn generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    lifetime_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(&proc_macro2::TokenStream::new(), &ImportPath::PostgresqlCrud, ident, lifetime_token_stream, content_token_stream)
}
pub fn generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(&ImportPath::PostgresqlCrud, ident, content_token_stream)
}

#[derive(Debug, Clone, Copy)]
pub enum ImportPath {
    Crate,
    PostgresqlCrud,
    PostgresqlCrudCommon,
}
impl ImportPath {
    pub const fn snake_case_std_primitive_str(&self) -> &'static str {
        match &self {
            Self::Crate => "crate",
            Self::PostgresqlCrud => "postgresql_crud",
            Self::PostgresqlCrudCommon => "postgresql_crud_common",
        }
    }
    fn default_but_option_is_always_some_and_vec_always_contains_one_element(
        &self,
    ) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
            Self::PostgresqlCrud => &token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
            Self::PostgresqlCrudCommon => &token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
        }
    }
    fn all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element(
        &self,
    ) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
            Self::PostgresqlCrud => &token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
            Self::PostgresqlCrudCommon => &token_patterns::PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
        }
    }
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(
        &self,
    ) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize,
            Self::PostgresqlCrud => &token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize,
            Self::PostgresqlCrudCommon => &token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize,
        }
    }
    fn all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size(
        &self,
    ) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize,
            Self::PostgresqlCrud => &token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize,
            Self::PostgresqlCrudCommon => &token_patterns::PostgresqlCrudCommonAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize,
        }
    }
    pub const fn to_path(&self) -> &'static str {
        match &self {
            Self::Crate => "crate",
            Self::PostgresqlCrud => "postgresql_crud",
            Self::PostgresqlCrudCommon => "postgresql_crud_common",
        }
    }
}
impl quote::ToTokens for ImportPath {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.snake_case_std_primitive_str()
            .parse::<proc_macro2::TokenStream>()
            .expect("d8636ee5-942b-472d-a025-c6e0700e1b59")
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsQueryBindMutable {
    True,
    False,
}
impl quote::ToTokens for IsQueryBindMutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::MutSnakeCase.to_tokens(tokens),
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IncrementParameterUnderscore {
    True,
    False,
}
impl quote::ToTokens for IncrementParameterUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => naming::IncrementSnakeCase.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ColumnParameterUnderscore {
    True,
    False,
}
impl quote::ToTokens for ColumnParameterUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => naming::ColumnSnakeCase.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsNeedToAddLogicalOperatorUnderscore {
    True,
    False,
}
impl quote::ToTokens for IsNeedToAddLogicalOperatorUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => naming::IsNeedToAddLogicalOperatorSnakeCase.to_tokens(tokens),
        }
    }
}
pub fn impl_postgresql_type_where_filter_for_ident_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    ident_token_stream: &dyn quote::ToTokens,
    ident_generic_token_stream: &dyn quote::ToTokens,
    increment_parameter_underscore: &IncrementParameterUnderscore,
    column_parameter_underscore: &ColumnParameterUnderscore,
    is_need_to_add_logical_operator_underscore: &IsNeedToAddLogicalOperatorUnderscore,
    query_part_content_token_stream: &dyn quote::ToTokens,
    is_query_bind_mutable: &IsQueryBindMutable,
    query_bind_content_token_stream: &dyn quote::ToTokens,
    import_path: &ImportPath,
) -> proc_macro2::TokenStream {
    let std_primitive_u64_token_stream = token_patterns::StdPrimitiveU64;
    let std_fmt_display_token_stream = token_patterns::StdFmtDisplay;
    let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;
    let std_string_string_token_stream = token_patterns::StdStringString;
    let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
    let query_part_snake_case = naming::QueryPartSnakeCase;
    let query_bind_snake_case = naming::QueryBindSnakeCase;
    let postgresql_type_where_filter_upper_camel_case =
        naming::PostgresqlTypeWhereFilterUpperCamelCase;
    quote::quote! {
        impl #impl_generic_token_stream #import_path ::#postgresql_type_where_filter_upper_camel_case<'lifetime> for #ident_token_stream #ident_generic_token_stream {
            fn #query_part_snake_case(
                &self,
                #increment_parameter_underscore: &mut #std_primitive_u64_token_stream,
                #column_parameter_underscore: &dyn #std_fmt_display_token_stream,
                #is_need_to_add_logical_operator_underscore: #std_primitive_bool_token_stream
            ) -> Result<#std_string_string_token_stream, #import_path::#query_part_error_named_upper_camel_case> {
                #query_part_content_token_stream
            }
            fn #query_bind_snake_case(self, #is_query_bind_mutable query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
                sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
                String
            > {
                #query_bind_content_token_stream
            }
        }
    }
}

pub fn generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl sqlx::Encode<'_, sqlx::Postgres> for #ident_token_stream {
            fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#content_token_stream, buf)
            }
        }
    }
}
pub fn generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    type_token_stream: &dyn quote::ToTokens,
    ok_value_match_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    quote::quote! {
        impl sqlx::Decode<'_, sqlx::Postgres> for #ident_token_stream {
            fn decode(#value_snake_case: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                match <#type_token_stream as sqlx::Decode<sqlx::Postgres>>::decode(#value_snake_case) {
                    Ok(ok_value) => #ok_value_match_token_stream,
                    Err(error) => Err(error),
                }
            }
        }
    }
}
pub fn generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    type_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl sqlx::Type<sqlx::Postgres> for #ident_token_stream {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
               <#type_token_stream as sqlx::Type<sqlx::Postgres>>::type_info()
            }
            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
                <#type_token_stream as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CreateQueryPartValueUnderscore {
    True,
    False,
}
impl quote::ToTokens for CreateQueryPartValueUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => naming::ValueSnakeCase.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum CreateQueryPartIncrementUnderscore {
    True,
    False,
}
impl quote::ToTokens for CreateQueryPartIncrementUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => naming::IncrementSnakeCase.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum CreateQueryBindValueUnderscore {
    True,
    False,
}
impl quote::ToTokens for CreateQueryBindValueUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => naming::ValueSnakeCase.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum SelectQueryPartValueUnderscore {
    True,
    False,
}
impl quote::ToTokens for SelectQueryPartValueUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => naming::ValueSnakeCase.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum UpdateQueryPartValueUnderscore {
    True,
    False,
}
impl quote::ToTokens for UpdateQueryPartValueUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => naming::ValueSnakeCase.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum UpdateQueryPartJsonbSetAccumulatorUnderscore {
    True,
    False,
}
impl quote::ToTokens for UpdateQueryPartJsonbSetAccumulatorUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => quote::quote! {jsonb_set_accumulator}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum UpdateQueryPartJsonbSetTargetUnderscore {
    True,
    False,
}
impl quote::ToTokens for UpdateQueryPartJsonbSetTargetUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => quote::quote! {jsonb_set_target}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum UpdateQueryPartJsonbSetPathUnderscore {
    True,
    False,
}
impl quote::ToTokens for UpdateQueryPartJsonbSetPathUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => quote::quote! {jsonb_set_path}.to_tokens(tokens),
        }
    }
}
pub fn generate_impl_postgresql_type_token_stream(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    ident_table_type_declaration_upper_camel_case: &dyn quote::ToTokens,
    is_primary_key_underscore: &IsPrimaryKeyUnderscore,
    create_table_column_query_part_token_stream: &dyn quote::ToTokens,
    ident_create_upper_camel_case: &dyn quote::ToTokens,
    create_query_part_value_underscore: &CreateQueryPartValueUnderscore,
    create_query_part_increment_underscore: &CreateQueryPartIncrementUnderscore,
    create_query_part_content_token_stream: &dyn quote::ToTokens,
    create_query_bind_value_underscore: &CreateQueryBindValueUnderscore,
    is_create_query_bind_mutable: &IsCreateQueryBindMutable,
    create_query_bind_content_token_stream: &dyn quote::ToTokens,
    ident_select_upper_camel_case: &dyn quote::ToTokens,
    select_query_part_value_underscore: &SelectQueryPartValueUnderscore,
    select_query_part_content_token_stream: &dyn quote::ToTokens,
    ident_where_upper_camel_case: &dyn quote::ToTokens,
    ident_read_upper_camel_case: &dyn quote::ToTokens,
    normalize_token_stream: &dyn quote::ToTokens,
    read_only_ids_token_stream: &dyn quote::ToTokens,
    select_only_ids_query_part_token_stream: &dyn quote::ToTokens,
    ident_read_inner_upper_camel_case: &dyn quote::ToTokens,
    into_inner_token_stream: &dyn quote::ToTokens,
    ident_update_upper_camel_case: &dyn quote::ToTokens,
    ident_update_for_query_upper_camel_case: &dyn quote::ToTokens,
    update_query_part_value_underscore: &UpdateQueryPartValueUnderscore,
    update_query_part_jsonb_set_accumulator_underscore: &UpdateQueryPartJsonbSetAccumulatorUnderscore,
    update_query_part_jsonb_set_target_underscore: &UpdateQueryPartJsonbSetTargetUnderscore,
    update_query_part_jsonb_set_path_underscore: &UpdateQueryPartJsonbSetPathUnderscore,
    update_query_part_content_token_stream: &dyn quote::ToTokens,
    is_update_query_bind_mutable: &IsUpdateQueryBindMutable,
    update_query_bind_content_token_stream: &dyn quote::ToTokens,
    select_only_updated_ids_query_part_token_stream: &dyn quote::ToTokens,
    is_select_only_updated_ids_query_bind_mutable: &IsSelectOnlyUpdatedIdsQueryBindMutable,
    select_only_updated_ids_query_bind_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_upper_camel_case = naming::PostgresqlTypeUpperCamelCase;
    let table_type_declaration_upper_camel_case = naming::TableTypeDeclarationUpperCamelCase;
    let create_table_column_query_part_snake_case = naming::CreateTableColumnQueryPartSnakeCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let create_query_part_snake_case = naming::CreateQueryPartSnakeCase;
    let create_query_bind_snake_case = naming::CreateQueryBindSnakeCase;
    let select_upper_camel_case = naming::SelectUpperCamelCase;
    let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
    let where_upper_camel_case = naming::WhereUpperCamelCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    let select_only_ids_query_part_snake_case = naming::SelectOnlyIdsQueryPartSnakeCase;
    let normalize_snake_case = naming::NormalizeSnakeCase;
    let read_inner_upper_camel_case = naming::ReadInnerUpperCamelCase;
    let update_upper_camel_case = naming::UpdateUpperCamelCase;
    let update_for_query_upper_camel_case = naming::UpdateForQueryUpperCamelCase;
    let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let select_only_updated_ids_query_part_snake_case =
        naming::SelectOnlyUpdatedIdsQueryPartSnakeCase;
    let select_only_updated_ids_query_bind_snake_case =
        naming::SelectOnlyUpdatedIdsQueryBindSnakeCase;
    let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
    let std_string_string_token_stream = token_patterns::StdStringString;
    let std_primitive_u64_token_stream = token_patterns::StdPrimitiveU64;
    let reference_std_primitive_str_token_stream = token_patterns::RefStdPrimitiveStr;
    let query_postgres_arguments_token_stream =
        quote::quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let std_fmt_display_token_stream = token_patterns::StdFmtDisplay;
    let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;
    quote::quote! {
        impl #import_path :: #postgresql_type_upper_camel_case for #ident {
            type #table_type_declaration_upper_camel_case = #ident_table_type_declaration_upper_camel_case;
            fn #create_table_column_query_part_snake_case(#column_snake_case: &dyn #std_fmt_display_token_stream, #is_primary_key_underscore: #std_primitive_bool_token_stream) -> impl #std_fmt_display_token_stream {
                #create_table_column_query_part_token_stream
            }
            type #create_upper_camel_case = #ident_create_upper_camel_case;
            fn #create_query_part_snake_case(
                #create_query_part_value_underscore: &Self::#create_upper_camel_case,
                #create_query_part_increment_underscore: &mut #std_primitive_u64_token_stream
            ) -> Result<#std_string_string_token_stream, #import_path ::#query_part_error_named_upper_camel_case> {
                #create_query_part_content_token_stream
            }
            fn #create_query_bind_snake_case(
                #create_query_bind_value_underscore: Self::#create_upper_camel_case,
                #is_create_query_bind_mutable #query_snake_case: #query_postgres_arguments_token_stream
            ) -> Result<
                #query_postgres_arguments_token_stream,
                String
            > {
                #create_query_bind_content_token_stream
            }
            type #select_upper_camel_case = #ident_select_upper_camel_case;
            fn #select_query_part_snake_case(
                #select_query_part_value_underscore: &Self::#select_upper_camel_case,
                #column_snake_case: #reference_std_primitive_str_token_stream,
            ) -> Result<#std_string_string_token_stream, #import_path ::#query_part_error_named_upper_camel_case> {
                #select_query_part_content_token_stream
            }
            type #where_upper_camel_case = #ident_where_upper_camel_case;
            type #read_upper_camel_case = #ident_read_upper_camel_case;
            fn #normalize_snake_case(#value_snake_case: Self::#read_upper_camel_case) -> Self::#read_upper_camel_case {
                #normalize_token_stream
            }
            type #read_only_ids_upper_camel_case = #read_only_ids_token_stream;
            fn #select_only_ids_query_part_snake_case(
                #column_snake_case: #reference_std_primitive_str_token_stream
            ) -> Result<#std_string_string_token_stream, #import_path ::#query_part_error_named_upper_camel_case> {
                #select_only_ids_query_part_token_stream
            }
            type #read_inner_upper_camel_case = #ident_read_inner_upper_camel_case;
            fn into_inner(#value_snake_case: Self::#read_upper_camel_case) -> Self::#read_inner_upper_camel_case {
                #into_inner_token_stream
            }
            type #update_upper_camel_case = #ident_update_upper_camel_case;
            type #update_for_query_upper_camel_case = #ident_update_for_query_upper_camel_case;
            fn #update_query_part_snake_case(
                #update_query_part_value_underscore: &Self::#update_for_query_upper_camel_case,
                #update_query_part_jsonb_set_accumulator_underscore: #reference_std_primitive_str_token_stream,
                #update_query_part_jsonb_set_target_underscore: #reference_std_primitive_str_token_stream,
                #update_query_part_jsonb_set_path_underscore: #reference_std_primitive_str_token_stream,
                #increment_snake_case: &mut #std_primitive_u64_token_stream
            ) -> Result<#std_string_string_token_stream, #import_path ::#query_part_error_named_upper_camel_case> {
                #update_query_part_content_token_stream
            }
            fn #update_query_bind_snake_case(
                #value_snake_case: Self::#update_for_query_upper_camel_case,
                #is_update_query_bind_mutable #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> Result<
                sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                String
            > {
                #update_query_bind_content_token_stream
            }
            fn #select_only_updated_ids_query_part_snake_case(
                #value_snake_case: &Self::#update_for_query_upper_camel_case,
                #column_snake_case: #reference_std_primitive_str_token_stream,
                #increment_snake_case: &mut #std_primitive_u64_token_stream,
            ) -> Result<#std_string_string_token_stream, #import_path ::#query_part_error_named_upper_camel_case> {
                #select_only_updated_ids_query_part_token_stream
            }
            fn #select_only_updated_ids_query_bind_snake_case<'lifetime>(
                #value_snake_case: &'lifetime Self::#update_for_query_upper_camel_case,
                #is_select_only_updated_ids_query_bind_mutable #query_snake_case: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
                #select_only_updated_ids_query_bind_token_stream
            }
        }
    }
}

pub fn generate_impl_postgresql_type_not_primary_key_for_ident_token_stream(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_not_primary_key_upper_camel_case =
        naming::PostgresqlTypeNotPrimaryKeyUpperCamelCase;
    let postgresql_type_upper_camel_case = naming::PostgresqlTypeUpperCamelCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let ident_create_upper_camel_case =
        naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
    quote::quote! {
        impl #import_path::#postgresql_type_not_primary_key_upper_camel_case for #ident {
            type #postgresql_type_upper_camel_case = Self;
            type #create_upper_camel_case = #ident_create_upper_camel_case;
        }
    }
}

// fn generate_read_only_ids_merged_with_create_into_where_method_token_stream(
//     import_path: &ImportPath,
//     method_name_token_stream: &dyn quote::ToTokens,
//     content_token_stream: &dyn quote::ToTokens,
//     postgresql_type_or_postgresql_json_type: &PostgresqlTypeOrPostgresqlJsonType,
// ) -> proc_macro2::TokenStream {
//     let self_upper_camel_case = naming::SelfUpperCamelCase;
//     let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
//     let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
//     let create_snake_case = naming::CreateSnakeCase;
//     let create_upper_camel_case = naming::CreateUpperCamelCase;
//     let where_upper_camel_case = naming::WhereUpperCamelCase;
//     let self_postgresql_type_or_postgresql_json_type_as_postgresql_json_type_token_stream = {
//         let postgresql_type_or_postgresql_json_type_token_stream: &dyn quote::ToTokens = match &postgresql_type_or_postgresql_json_type {
//             PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => &naming::PostgresqlTypeUpperCamelCase,
//             PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => &naming::PostgresqlJsonTypeUpperCamelCase,
//         };
//         quote::quote! {
//             <#self_upper_camel_case::#postgresql_type_or_postgresql_json_type_token_stream as #import_path::#postgresql_type_or_postgresql_json_type_token_stream>
//         }
//     };
//     quote::quote!{
//         fn #method_name_token_stream(
//             #read_only_ids_snake_case: #self_postgresql_type_or_postgresql_json_type_as_postgresql_json_type_token_stream::#read_only_ids_upper_camel_case,
//             #create_snake_case: #self_postgresql_type_or_postgresql_json_type_as_postgresql_json_type_token_stream::#create_upper_camel_case
//         ) -> Vec<#self_postgresql_type_or_postgresql_json_type_as_postgresql_json_type_token_stream::#where_upper_camel_case> {
//             #content_token_stream
//         }
//     }
// }

fn generate_option_vec_create_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let option_vec_create_snake_case = naming::OptionVecCreateSnakeCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    quote::quote! {
        fn #option_vec_create_snake_case() -> Option<Vec<#path_token_stream::#create_upper_camel_case>> {
            #content_token_stream
        }
    }
}
fn generate_read_only_ids_to_two_dimensional_vec_read_inner_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_to_two_dimensional_vec_read_inner_snake_case =
        naming::ReadOnlyIdsToTwoDimensionalVecReadInnerSnakeCase;
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    let read_inner_upper_camel_case = naming::ReadInnerUpperCamelCase;
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    quote::quote! {
        fn #read_only_ids_to_two_dimensional_vec_read_inner_snake_case(
            #read_only_ids_snake_case: &#path_token_stream::#read_only_ids_upper_camel_case
        ) -> Vec<Vec<#path_token_stream::#read_inner_upper_camel_case>> {
            #content_token_stream
        }
    }
}
fn generate_read_inner_into_read_with_new_or_try_new_unwraped_token_stream(
    type_token_stream: &dyn quote::ToTokens,
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_inner_into_read_with_new_or_try_new_unwraped_snake_case =
        naming::ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    quote::quote! {
        fn #read_inner_into_read_with_new_or_try_new_unwraped_snake_case(
            #value_snake_case: #type_token_stream
        ) -> #path_token_stream::#read_upper_camel_case {
            #content_token_stream
        }
    }
}
fn generate_read_inner_into_update_with_new_or_try_new_unwraped_token_stream(
    type_token_stream: &dyn quote::ToTokens,
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_inner_into_update_with_new_or_try_new_unwraped_snake_case =
        naming::ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase;
    let update_upper_camel_case = naming::UpdateUpperCamelCase;
    let value_snake_case = naming::ValueSnakeCase;
    quote::quote! {
        fn #read_inner_into_update_with_new_or_try_new_unwraped_snake_case(#value_snake_case: #type_token_stream) -> #path_token_stream::#update_upper_camel_case {
            #content_token_stream
        }
    }
}
fn generate_update_to_read_only_ids_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let update_to_read_only_ids_snake_case = naming::UpdateToReadOnlyIdsSnakeCase;
    let update_upper_camel_case = naming::UpdateUpperCamelCase;
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    let value_snake_case = naming::ValueSnakeCase;
    quote::quote! {
        fn #update_to_read_only_ids_snake_case(
            #value_snake_case: &#path_token_stream::#update_upper_camel_case
        ) -> #path_token_stream::#read_only_ids_upper_camel_case {
            #content_token_stream
        }
    }
}
fn generate_read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream(
    import_path: ImportPath,
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case = naming::ReadOnlyIdsToOptionValueReadDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementSnakeCase;
    let value_upper_camel_case = naming::ValueUpperCamelCase;
    let value_snake_case = naming::ValueSnakeCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    quote::quote! {
        fn #read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case(
            #value_snake_case: &#path_token_stream::#read_only_ids_upper_camel_case
        ) -> Option<#import_path::#value_upper_camel_case<#path_token_stream::#read_upper_camel_case>> {
            #content_token_stream
        }
    }
}
fn generate_previous_read_merged_with_option_update_into_read_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let previous_read_merged_with_option_update_into_read_snake_case =
        naming::PreviousReadMergedWithOptionUpdateIntoReadSnakeCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    let read_snake_case = naming::ReadSnakeCase;
    let update_upper_camel_case = naming::UpdateUpperCamelCase;
    let option_update_snake_case = naming::OptionUpdateSnakeCase;
    quote::quote! {
        fn #previous_read_merged_with_option_update_into_read_snake_case(
            #read_snake_case: #path_token_stream::#read_upper_camel_case,
            #option_update_snake_case: Option<#path_token_stream::#update_upper_camel_case>,
        ) -> #path_token_stream::#read_upper_camel_case {
            #content_token_stream
        }
    }
}
fn generate_read_only_ids_merged_with_create_into_read_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_merged_with_create_into_read_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoReadSnakeCase;
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let create_snake_case = naming::CreateSnakeCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    quote::quote! {
        fn #read_only_ids_merged_with_create_into_read_snake_case(
            #read_only_ids_snake_case: #path_token_stream::#read_only_ids_upper_camel_case,
            #create_snake_case: #path_token_stream::#create_upper_camel_case
        ) -> #path_token_stream::#read_upper_camel_case {
            #content_token_stream
        }
    }
}
fn generate_read_only_ids_merged_with_create_into_option_value_read_token_stream(
    import_path: ImportPath,
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_merged_with_create_into_option_value_read_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoOptionValueReadSnakeCase;
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let create_snake_case = naming::CreateSnakeCase;
    let value_upper_camel_case = naming::ValueUpperCamelCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    quote::quote! {
        fn #read_only_ids_merged_with_create_into_option_value_read_snake_case(
            #read_only_ids_snake_case: #path_token_stream::#read_only_ids_upper_camel_case,
            #create_snake_case: #path_token_stream::#create_upper_camel_case
        ) -> Option<#import_path::#value_upper_camel_case<#path_token_stream::#read_upper_camel_case>> {
            #content_token_stream
        }
    }
}
fn generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_merged_with_create_into_table_type_declaration_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSnakeCase;
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let create_snake_case = naming::CreateSnakeCase;
    let table_type_declaration_upper_camel_case = naming::TableTypeDeclarationUpperCamelCase;
    quote::quote! {
        fn #read_only_ids_merged_with_create_into_table_type_declaration_snake_case(
            #read_only_ids_snake_case: #path_token_stream::#read_only_ids_upper_camel_case,
            #create_snake_case: #path_token_stream::#create_upper_camel_case
        ) -> #path_token_stream::#table_type_declaration_upper_camel_case {
            #content_token_stream
        }
    }
}

pub fn generate_read_only_ids_merged_with_create_into_where_equal_token_stream(
    read_only_ids_token_stream: &dyn quote::ToTokens,
    create_token_stream: &dyn quote::ToTokens,
    where_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_merged_with_create_into_where_equal_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoWhereEqualSnakeCase;
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    let create_snake_case = naming::CreateSnakeCase;
    quote::quote! {
        fn #read_only_ids_merged_with_create_into_where_equal_snake_case(
            #read_only_ids_snake_case: #read_only_ids_token_stream,
            #create_snake_case: #create_token_stream
        ) -> #where_token_stream {
            #content_token_stream
        }
    }
}
pub fn generate_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream(
    read_only_ids_token_stream: &dyn quote::ToTokens,
    create_token_stream: &dyn quote::ToTokens,
    where_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSnakeCase;
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    let create_snake_case = naming::CreateSnakeCase;
    quote::quote! {
        fn #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case(
            #read_only_ids_snake_case: #read_only_ids_token_stream,
            #create_snake_case: #create_token_stream
        ) -> Vec<#where_token_stream> {
            #content_token_stream
        }
    }
}

fn generate_read_only_ids_merged_with_create_into_vec_or_option_vec_where_equal_to_json_field_postgresql_type_or_postgresql_json_type_token_stream(
    read_only_ids_token_stream: &dyn quote::ToTokens,
    create_token_stream: &dyn quote::ToTokens,
    where_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
    postgresql_type_or_postgresql_json_type: PostgresqlTypeOrPostgresqlJsonType,
) -> proc_macro2::TokenStream {
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    let create_snake_case = naming::CreateSnakeCase;
    let return_type_token_stream = {
        let return_type_handle_token_stream = quote::quote! {Vec<#where_token_stream>};
        match &postgresql_type_or_postgresql_json_type {
            PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => {
                generate_std_option_option_tokens_declaration_token_stream(
                    &return_type_handle_token_stream,
                )
            }
            PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => {
                return_type_handle_token_stream
            }
        }
    };
    let name_token_stream: &dyn quote::ToTokens = match &postgresql_type_or_postgresql_json_type {
        PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => {
            &naming::ReadOnlyIdsMergedWithCreateIntoOptionVecWhereEqualToJsonFieldSnakeCase
        }
        PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => {
            &naming::ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSnakeCase
        }
    };
    quote::quote! {
        fn #name_token_stream(
            #read_only_ids_snake_case: #read_only_ids_token_stream,
            #create_snake_case: #create_token_stream
        ) -> #return_type_token_stream {
            #content_token_stream
        }
    }
}
pub fn generate_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream(
    read_only_ids_token_stream: &dyn quote::ToTokens,
    create_token_stream: &dyn quote::ToTokens,
    where_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_vec_or_option_vec_where_equal_to_json_field_postgresql_type_or_postgresql_json_type_token_stream(&read_only_ids_token_stream, &create_token_stream, &where_token_stream, &content_token_stream, PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType)
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_token_stream(
    name_token_stream: &dyn quote::ToTokens,
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let create_snake_case = naming::CreateSnakeCase;
    let where_upper_camel_case = naming::WhereUpperCamelCase;
    quote::quote! {
        fn #name_token_stream(
            #read_only_ids_snake_case: #path_token_stream::#read_only_ids_upper_camel_case,
            #create_snake_case: #path_token_stream::#create_upper_camel_case
        ) -> Option<Vec<#path_token_stream::#where_upper_camel_case>> {
            #content_token_stream
        }
    }
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_token_stream(&naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionOneEqualSnakeCase, &path_token_stream, &content_token_stream)
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_token_stream(&naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionTwoEqualSnakeCase, &path_token_stream, &content_token_stream)
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_token_stream(&naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionThreeEqualSnakeCase, &path_token_stream, &content_token_stream)
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_token_stream(&naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionFourEqualSnakeCase, &path_token_stream, &content_token_stream)
}

fn generate_create_into_postgresql_json_type_option_vec_where_length_equal_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let create_into_postgresql_json_type_option_vec_where_length_equal_snake_case =
        naming::CreateIntoPostgresqlJsonTypeOptionVecWhereLengthEqualSnakeCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let create_snake_case = naming::CreateSnakeCase;
    let where_upper_camel_case = naming::WhereUpperCamelCase;
    quote::quote! {
        fn #create_into_postgresql_json_type_option_vec_where_length_equal_snake_case(
            #create_snake_case: #path_token_stream::#create_upper_camel_case
        ) -> Option<Vec<#path_token_stream::#where_upper_camel_case>> {
            #content_token_stream
        }
    }
}

fn generate_create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let create_into_postgresql_json_type_option_vec_where_length_greater_than_snake_case =
        naming::CreateIntoPostgresqlJsonTypeOptionVecWhereLengthGreaterThanSnakeCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let create_snake_case = naming::CreateSnakeCase;
    let where_upper_camel_case = naming::WhereUpperCamelCase;
    quote::quote! {
        fn #create_into_postgresql_json_type_option_vec_where_length_greater_than_snake_case(
            #create_snake_case: #path_token_stream::#create_upper_camel_case
        ) -> Option<Vec<#path_token_stream::#where_upper_camel_case>> {
            #content_token_stream
        }
    }
}

fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereGreaterThanSnakeCase;
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let create_snake_case = naming::CreateSnakeCase;
    let where_upper_camel_case = naming::WhereUpperCamelCase;
    quote::quote! {
        fn #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_snake_case(
            #read_only_ids_snake_case: #path_token_stream::#read_only_ids_upper_camel_case,
            #create_snake_case: #path_token_stream::#create_upper_camel_case
        ) -> Option<Vec<#path_token_stream::#where_upper_camel_case>> {
            #content_token_stream
        }
    }
}

fn generate_postgresql_json_type_option_vec_where_length_greater_than_test_token_stream(
    postgresql_type_or_postgresql_json_type: PostgresqlTypeOrPostgresqlJsonType,
    import_path: ImportPath,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_json_type_option_vec_where_length_greater_than_test_snake_case =
        naming::PostgresqlJsonTypeOptionVecWhereLengthGreaterThanTestSnakeCase;
    let (first_token_stream, second_token_stream) = match &postgresql_type_or_postgresql_json_type {
        PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => (
            quote::quote! {PostgresqlTypeLengthGreaterThanTest},
            quote::quote! {PostgresqlType},
        ),
        PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => (
            quote::quote! {PostgresqlJsonTypeLengthGreaterThanTest},
            quote::quote! {PostgresqlJsonType},
        ),
    };
    quote::quote! {
        fn #postgresql_json_type_option_vec_where_length_greater_than_test_snake_case() -> Option<Vec<#import_path::#first_token_stream<Self::#second_token_stream>>> {
            #content_token_stream
        }
    }
}

pub fn generate_impl_postgresql_type_test_cases_for_ident_token_stream(
    cfg_token_stream: &dyn quote::ToTokens,
    import_path: &ImportPath,
    type_token_stream: &dyn quote::ToTokens,
    ident: &dyn quote::ToTokens,
    option_vec_create_token_stream: &dyn quote::ToTokens,
    read_only_ids_to_two_dimensional_vec_read_inner_token_stream: &dyn quote::ToTokens,
    read_inner_into_read_with_new_or_try_new_unwraped_token_stream: &dyn quote::ToTokens,
    read_inner_into_update_with_new_or_try_new_unwraped_token_stream: &dyn quote::ToTokens,
    update_to_read_only_ids_token_stream: &dyn quote::ToTokens,
    read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream: &dyn quote::ToTokens,
    previous_read_merged_with_option_update_into_read_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_read_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_option_value_read_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_table_type_declaration_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_where_equal_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_type_option_vec_where_dimension_one_equal_token_stream: &dyn quote::ToTokens,
    postgresql_type_option_vec_where_greater_than_test_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_length_equal_token_stream: &dyn quote::ToTokens,
    postgresql_json_type_option_vec_where_length_greater_than_test_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_upper_camel_case = naming::PostgresqlTypeUpperCamelCase;
    let postgresql_type_test_cases_upper_camel_case = naming::PostgresqlTypeTestCasesUpperCamelCase;
    let table_type_declaration_upper_camel_case = naming::TableTypeDeclarationUpperCamelCase;
    let table_type_declaration_snake_case = naming::TableTypeDeclarationSnakeCase;
    let self_upper_camel_case = naming::SelfUpperCamelCase;
    let select_upper_camel_case = naming::SelectUpperCamelCase;
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    let where_upper_camel_case = naming::WhereUpperCamelCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let create_snake_case = naming::CreateSnakeCase;
    let self_postgresql_type_as_postgresql_type_token_stream = quote::quote! {<#self_upper_camel_case::#postgresql_type_upper_camel_case as #import_path::#postgresql_type_upper_camel_case>};
    let self_postgresql_type_as_postgresql_type_read_only_ids_token_stream = quote::quote! {#self_postgresql_type_as_postgresql_type_token_stream::#read_only_ids_upper_camel_case};
    let self_postgresql_type_as_postgresql_type_create_token_stream = quote::quote! {#self_postgresql_type_as_postgresql_type_token_stream::#create_upper_camel_case};
    let self_postgresql_type_as_postgresql_type_where_token_stream = quote::quote! {#self_postgresql_type_as_postgresql_type_token_stream::#where_upper_camel_case};
    let ident_select_upper_camel_case =
        naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
    let option_vec_create_content_token_stream = generate_option_vec_create_token_stream(
        &self_postgresql_type_as_postgresql_type_token_stream,
        &option_vec_create_token_stream,
    );
    let read_only_ids_to_two_dimensional_vec_read_inner_content_token_stream =
        generate_read_only_ids_to_two_dimensional_vec_read_inner_token_stream(
            &self_postgresql_type_as_postgresql_type_token_stream,
            &read_only_ids_to_two_dimensional_vec_read_inner_token_stream,
        );
    let read_inner_into_read_with_new_or_try_new_unwraped_content_token_stream =
        generate_read_inner_into_read_with_new_or_try_new_unwraped_token_stream(
            &type_token_stream,
            &self_postgresql_type_as_postgresql_type_token_stream,
            &read_inner_into_read_with_new_or_try_new_unwraped_token_stream,
        );
    let read_inner_into_update_with_new_or_try_new_unwraped_content_token_stream =
        generate_read_inner_into_update_with_new_or_try_new_unwraped_token_stream(
            &type_token_stream,
            &self_postgresql_type_as_postgresql_type_token_stream,
            &read_inner_into_update_with_new_or_try_new_unwraped_token_stream,
        );
    let update_to_read_only_ids_content_token_stream =
        generate_update_to_read_only_ids_token_stream(
            &self_postgresql_type_as_postgresql_type_token_stream,
            &update_to_read_only_ids_token_stream,
        );
    let read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_content_token_stream =
        generate_read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream(*import_path, &self_postgresql_type_as_postgresql_type_token_stream, &read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream);
    let previous_read_merged_with_option_update_into_read_content_token_stream =
        generate_previous_read_merged_with_option_update_into_read_token_stream(
            &self_postgresql_type_as_postgresql_type_token_stream,
            &previous_read_merged_with_option_update_into_read_token_stream,
        );
    let read_only_ids_merged_with_create_into_read_content_token_stream =
        generate_read_only_ids_merged_with_create_into_read_token_stream(
            &self_postgresql_type_as_postgresql_type_token_stream,
            &read_only_ids_merged_with_create_into_read_token_stream,
        );
    let read_only_ids_merged_with_create_into_option_value_read_content_token_stream =
        generate_read_only_ids_merged_with_create_into_option_value_read_token_stream(
            *import_path,
            &self_postgresql_type_as_postgresql_type_token_stream,
            &read_only_ids_merged_with_create_into_option_value_read_token_stream,
        );
    let read_only_ids_merged_with_create_into_table_type_declaration_content_token_stream =
        generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream(
            &self_postgresql_type_as_postgresql_type_token_stream,
            &read_only_ids_merged_with_create_into_table_type_declaration_token_stream,
        );
    let read_only_ids_merged_with_create_into_where_equal_content_token_stream =
        generate_read_only_ids_merged_with_create_into_where_equal_token_stream(
            &self_postgresql_type_as_postgresql_type_read_only_ids_token_stream,
            &self_postgresql_type_as_postgresql_type_create_token_stream,
            &self_postgresql_type_as_postgresql_type_where_token_stream,
            &read_only_ids_merged_with_create_into_where_equal_token_stream,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_content_token_stream =
        generate_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream(
            &self_postgresql_type_as_postgresql_type_read_only_ids_token_stream,
            &self_postgresql_type_as_postgresql_type_create_token_stream,
            &self_postgresql_type_as_postgresql_type_where_token_stream,
            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream,
        );
    let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_content_token_stream = generate_read_only_ids_merged_with_create_into_vec_or_option_vec_where_equal_to_json_field_postgresql_type_or_postgresql_json_type_token_stream(
        &self_postgresql_type_as_postgresql_type_read_only_ids_token_stream,
        &self_postgresql_type_as_postgresql_type_create_token_stream,
        &self_postgresql_type_as_postgresql_type_where_token_stream,
        &read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_token_stream,
        PostgresqlTypeOrPostgresqlJsonType::PostgresqlType,
    );
    let create_into_postgresql_type_option_vec_where_dimension_one_equal_snake_case =
        naming::CreateIntoPostgresqlTypeOptionVecWhereDimensionOneEqualSnakeCase;
    let postgresql_type_option_vec_where_greater_than_test_snake_case =
        naming::PostgresqlTypeOptionVecWhereGreaterThanTestSnakeCase;
    let read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_snake_case = naming::ReadOnlyIdsMergedWithTableTypeDeclarationIntoPostgresqlTypeOptionWhereGreaterThanSnakeCase;
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_content_token_stream = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream(&self_postgresql_type_as_postgresql_type_token_stream, &create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream);
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_content_token_stream = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream(&self_postgresql_type_as_postgresql_type_token_stream, &create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream);
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_content_token_stream = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream(&self_postgresql_type_as_postgresql_type_token_stream, &create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream);
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_content_token_stream = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream(&self_postgresql_type_as_postgresql_type_token_stream, &create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream);
    let create_into_postgresql_json_type_option_vec_where_length_equal_content_token_stream =
        generate_create_into_postgresql_json_type_option_vec_where_length_equal_token_stream(
            &self_postgresql_type_as_postgresql_type_token_stream,
            &create_into_postgresql_json_type_option_vec_where_length_equal_token_stream,
        );
    let postgresql_json_type_option_vec_where_length_greater_than_test_content_token_stream =
        generate_postgresql_json_type_option_vec_where_length_greater_than_test_token_stream(
            PostgresqlTypeOrPostgresqlJsonType::PostgresqlType,
            *import_path,
            &postgresql_json_type_option_vec_where_length_greater_than_test_token_stream,
        );
    let create_into_postgresql_json_type_option_vec_where_length_greater_than_content_token_stream =
        generate_create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream(
            &self_postgresql_type_as_postgresql_type_token_stream,
            &create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_content_token_stream =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream(
            &self_postgresql_type_as_postgresql_type_token_stream,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream,
        );
    quote::quote! {
        #cfg_token_stream
        impl #import_path::#postgresql_type_test_cases_upper_camel_case for #ident {
            type #postgresql_type_upper_camel_case = #self_upper_camel_case;
            type #select_upper_camel_case = #ident_select_upper_camel_case;
            #option_vec_create_content_token_stream
            #read_only_ids_to_two_dimensional_vec_read_inner_content_token_stream
            #read_inner_into_read_with_new_or_try_new_unwraped_content_token_stream
            #read_inner_into_update_with_new_or_try_new_unwraped_content_token_stream
            #update_to_read_only_ids_content_token_stream
            #read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_content_token_stream
            #previous_read_merged_with_option_update_into_read_content_token_stream
            #read_only_ids_merged_with_create_into_read_content_token_stream
            #read_only_ids_merged_with_create_into_option_value_read_content_token_stream
            #read_only_ids_merged_with_create_into_table_type_declaration_content_token_stream
            #read_only_ids_merged_with_create_into_where_equal_content_token_stream
            #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_content_token_stream
            #read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_content_token_stream
            fn #create_into_postgresql_type_option_vec_where_dimension_one_equal_snake_case(
                #create_snake_case: #self_postgresql_type_as_postgresql_type_token_stream::#create_upper_camel_case
            ) -> Option<Vec<#self_postgresql_type_as_postgresql_type_token_stream::#where_upper_camel_case>> {
                #create_into_postgresql_type_option_vec_where_dimension_one_equal_token_stream
            }
            fn #postgresql_type_option_vec_where_greater_than_test_snake_case() -> Option<Vec<#import_path::PostgresqlTypeGreaterThanTest<#self_upper_camel_case::#postgresql_type_upper_camel_case>>> {
                #postgresql_type_option_vec_where_greater_than_test_token_stream
            }
            fn #read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_snake_case(
                greater_than_variant: #import_path::PostgresqlTypeGreaterThanVariant,
                #read_only_ids_snake_case: #self_postgresql_type_as_postgresql_type_token_stream::#read_only_ids_upper_camel_case,
                #table_type_declaration_snake_case: #self_postgresql_type_as_postgresql_type_token_stream::#table_type_declaration_upper_camel_case,
            ) -> Option<#self_postgresql_type_as_postgresql_type_token_stream::#where_upper_camel_case> {
                #read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_token_stream
            }
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_content_token_stream
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_content_token_stream
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_content_token_stream
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_content_token_stream
            #create_into_postgresql_json_type_option_vec_where_length_equal_content_token_stream
            #postgresql_json_type_option_vec_where_length_greater_than_test_content_token_stream
            #create_into_postgresql_json_type_option_vec_where_length_greater_than_content_token_stream
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_content_token_stream
        }
    }
}

pub fn generate_impl_postgresql_json_type_test_cases_for_ident_token_stream(
    cfg_token_stream: &dyn quote::ToTokens,
    import_path: &ImportPath,
    type_token_stream: &dyn quote::ToTokens,
    ident: &dyn quote::ToTokens,
    option_vec_create_token_stream: &dyn quote::ToTokens,
    read_only_ids_to_two_dimensional_vec_read_inner_token_stream: &dyn quote::ToTokens,
    read_inner_into_read_with_new_or_try_new_unwraped_token_stream: &dyn quote::ToTokens,
    read_inner_into_update_with_new_or_try_new_unwraped_token_stream: &dyn quote::ToTokens,
    read_only_ids_into_option_value_read_inner_token_stream: &dyn quote::ToTokens,
    update_to_read_only_ids_token_stream: &dyn quote::ToTokens,
    read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream: &dyn quote::ToTokens,
    previous_read_merged_with_option_update_into_read_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_read_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_option_value_read_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_table_type_declaration_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_where_equal_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_length_equal_token_stream: &dyn quote::ToTokens,
    postgresql_json_type_option_vec_where_length_greater_than_test_token_stream: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let value_upper_camel_case = naming::ValueUpperCamelCase;
    let value_snake_case = naming::ValueSnakeCase;
    let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
    let postgresql_json_type_test_cases_upper_camel_case =
        naming::PostgresqlJsonTypeTestCasesUpperCamelCase;
    let read_inner_upper_camel_case = naming::ReadInnerUpperCamelCase;
    let self_upper_camel_case = naming::SelfUpperCamelCase;
    let read_only_ids_upper_camel_case = naming::ReadOnlyIdsUpperCamelCase;
    let read_only_ids_into_option_value_read_inner_snake_case =
        naming::ReadOnlyIdsIntoOptionValueReadInnerSnakeCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let select_upper_camel_case = naming::SelectUpperCamelCase;
    let where_upper_camel_case = naming::WhereUpperCamelCase;
    let self_postgresql_json_type_as_postgresql_json_type_token_stream = quote::quote! {<#self_upper_camel_case::#postgresql_json_type_upper_camel_case as #import_path::#postgresql_json_type_upper_camel_case>};
    let self_postgresql_json_type_as_postgresql_json_type_read_only_ids_token_stream = quote::quote! {#self_postgresql_json_type_as_postgresql_json_type_token_stream::#read_only_ids_upper_camel_case};
    let self_postgresql_json_type_as_postgresql_json_type_create_token_stream = quote::quote! {#self_postgresql_json_type_as_postgresql_json_type_token_stream::#create_upper_camel_case};
    let self_postgresql_json_type_as_postgresql_json_type_where_token_stream = quote::quote! {#self_postgresql_json_type_as_postgresql_json_type_token_stream::#where_upper_camel_case};
    let ident_select_upper_camel_case =
        naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
    let option_vec_create_content_token_stream = generate_option_vec_create_token_stream(
        &self_postgresql_json_type_as_postgresql_json_type_token_stream,
        &option_vec_create_token_stream,
    );
    let read_only_ids_to_two_dimensional_vec_read_inner_content_token_stream =
        generate_read_only_ids_to_two_dimensional_vec_read_inner_token_stream(
            &self_postgresql_json_type_as_postgresql_json_type_token_stream,
            &read_only_ids_to_two_dimensional_vec_read_inner_token_stream,
        );
    let read_inner_into_read_with_new_or_try_new_unwraped_content_token_stream =
        generate_read_inner_into_read_with_new_or_try_new_unwraped_token_stream(
            &type_token_stream,
            &self_postgresql_json_type_as_postgresql_json_type_token_stream,
            &read_inner_into_read_with_new_or_try_new_unwraped_token_stream,
        );
    let read_inner_into_update_with_new_or_try_new_unwraped_content_token_stream =
        generate_read_inner_into_update_with_new_or_try_new_unwraped_token_stream(
            &type_token_stream,
            &self_postgresql_json_type_as_postgresql_json_type_token_stream,
            &read_inner_into_update_with_new_or_try_new_unwraped_token_stream,
        );
    let update_to_read_only_ids_content_token_stream =
        generate_update_to_read_only_ids_token_stream(
            &self_postgresql_json_type_as_postgresql_json_type_token_stream,
            &update_to_read_only_ids_token_stream,
        );
    let read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_content_token_stream =
        generate_read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream(*import_path, &self_postgresql_json_type_as_postgresql_json_type_token_stream, &read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream);
    let previous_read_merged_with_option_update_into_read_content_token_stream =
        generate_previous_read_merged_with_option_update_into_read_token_stream(
            &self_postgresql_json_type_as_postgresql_json_type_token_stream,
            &previous_read_merged_with_option_update_into_read_token_stream,
        );
    let read_only_ids_merged_with_create_into_read_content_token_stream =
        generate_read_only_ids_merged_with_create_into_read_token_stream(
            &self_postgresql_json_type_as_postgresql_json_type_token_stream,
            &read_only_ids_merged_with_create_into_read_token_stream,
        );
    let read_only_ids_merged_with_create_into_option_value_read_content_token_stream =
        generate_read_only_ids_merged_with_create_into_option_value_read_token_stream(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_token_stream,
            &read_only_ids_merged_with_create_into_option_value_read_token_stream,
        );
    let read_only_ids_merged_with_create_into_table_type_declaration_content_token_stream =
        generate_read_only_ids_merged_with_create_into_table_type_declaration_token_stream(
            &self_postgresql_json_type_as_postgresql_json_type_token_stream,
            &read_only_ids_merged_with_create_into_table_type_declaration_token_stream,
        );
    let read_only_ids_merged_with_create_into_where_equal_content_token_stream =
        generate_read_only_ids_merged_with_create_into_where_equal_token_stream(
            &self_postgresql_json_type_as_postgresql_json_type_read_only_ids_token_stream,
            &self_postgresql_json_type_as_postgresql_json_type_create_token_stream,
            &self_postgresql_json_type_as_postgresql_json_type_where_token_stream,
            &read_only_ids_merged_with_create_into_where_equal_token_stream,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_content_token_stream =
        generate_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream(
            &self_postgresql_json_type_as_postgresql_json_type_read_only_ids_token_stream,
            &self_postgresql_json_type_as_postgresql_json_type_create_token_stream,
            &self_postgresql_json_type_as_postgresql_json_type_where_token_stream,
            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_content_token_stream =
        generate_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream(
            &self_postgresql_json_type_as_postgresql_json_type_read_only_ids_token_stream,
            &self_postgresql_json_type_as_postgresql_json_type_create_token_stream,
            &self_postgresql_json_type_as_postgresql_json_type_where_token_stream,
            &read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_content_token_stream = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream(&self_postgresql_json_type_as_postgresql_json_type_token_stream, &create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream);
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_content_token_stream = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream(&self_postgresql_json_type_as_postgresql_json_type_token_stream, &create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream);
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_content_token_stream =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream(&self_postgresql_json_type_as_postgresql_json_type_token_stream, &create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream);
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_content_token_stream = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream(&self_postgresql_json_type_as_postgresql_json_type_token_stream, &create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream);
    let create_into_postgresql_json_type_option_vec_where_length_equal_content_token_stream =
        generate_create_into_postgresql_json_type_option_vec_where_length_equal_token_stream(
            &self_postgresql_json_type_as_postgresql_json_type_token_stream,
            &create_into_postgresql_json_type_option_vec_where_length_equal_token_stream,
        );
    let postgresql_json_type_option_vec_where_length_greater_than_test_content_token_stream =
        generate_postgresql_json_type_option_vec_where_length_greater_than_test_token_stream(
            PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType,
            *import_path,
            &postgresql_json_type_option_vec_where_length_greater_than_test_token_stream,
        );
    let create_into_postgresql_json_type_option_vec_where_length_greater_than_content_token_stream =
        generate_create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream(
            &self_postgresql_json_type_as_postgresql_json_type_token_stream,
            &create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_content_token_stream =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream(
            &self_postgresql_json_type_as_postgresql_json_type_token_stream,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream,
        );
    quote::quote! {
        #cfg_token_stream
        impl #import_path::#postgresql_json_type_test_cases_upper_camel_case for #ident {
            type #postgresql_json_type_upper_camel_case = #self_upper_camel_case;
            type #select_upper_camel_case = #ident_select_upper_camel_case;
            #option_vec_create_content_token_stream
            #read_only_ids_to_two_dimensional_vec_read_inner_content_token_stream
            #read_inner_into_read_with_new_or_try_new_unwraped_content_token_stream
            #read_inner_into_update_with_new_or_try_new_unwraped_content_token_stream
            fn #read_only_ids_into_option_value_read_inner_snake_case(
                #value_snake_case: #self_postgresql_json_type_as_postgresql_json_type_token_stream::#read_only_ids_upper_camel_case
            ) -> Option<#import_path::#value_upper_camel_case<#self_postgresql_json_type_as_postgresql_json_type_token_stream::#read_inner_upper_camel_case>> {
                #read_only_ids_into_option_value_read_inner_token_stream
            }
            #update_to_read_only_ids_content_token_stream
            #read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_content_token_stream
            #previous_read_merged_with_option_update_into_read_content_token_stream
            #read_only_ids_merged_with_create_into_read_content_token_stream
            #read_only_ids_merged_with_create_into_option_value_read_content_token_stream
            #read_only_ids_merged_with_create_into_table_type_declaration_content_token_stream
            #read_only_ids_merged_with_create_into_where_equal_content_token_stream
            #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_content_token_stream
            #read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_content_token_stream
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_content_token_stream
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_content_token_stream
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_content_token_stream
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_content_token_stream
            #create_into_postgresql_json_type_option_vec_where_length_equal_content_token_stream
            #postgresql_json_type_option_vec_where_length_greater_than_test_content_token_stream
            #create_into_postgresql_json_type_option_vec_where_length_greater_than_content_token_stream
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_content_token_stream
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ReadOrUpdate {
    Read,
    Update,
}
impl ReadOrUpdate {
    pub fn upper_camel_case(&self) -> &dyn naming::StdFmtDisplayPlusQuoteToTokens {
        match &self {
            Self::Read => &naming::ReadUpperCamelCase,
            Self::Update => &naming::UpdateUpperCamelCase,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IsPrimaryKeyUnderscore {
    True,
    False,
}
impl quote::ToTokens for IsPrimaryKeyUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {_}.to_tokens(tokens),
            Self::False => naming::IsPrimaryKeySnakeCase.to_tokens(tokens),
        }
    }
}

pub fn postgresql_crud_common_query_part_error_named_checked_add_initialization_token_stream()
-> proc_macro2::TokenStream {
    quote::quote! {postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }}
}

pub fn generate_impl_crate_is_string_empty_for_ident_token_stream(
    ident: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl postgresql_crud_common::IsStringEmpty for #ident {
            fn is_string_empty(&self) -> bool {
                self.0.to_string().is_empty()
            }
        }
    }
}

pub fn generate_match_try_new_in_deserialize_token_stream(
    ident: &dyn quote::ToTokens,
    initialization_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        match #ident::try_new(#initialization_token_stream) {
            Ok(value) => Ok(value),
            Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
        }
    }
}
pub fn generate_impl_serde_deserialize_for_struct_token_stream(
    ident: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    vec_ident_type: &[(&syn::Ident, &syn::Type)],
    len: usize,
    generate_type_token_stream: &dyn Fn(&syn::Ident, &syn::Type) -> proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    fn generate_underscore_underscore_field_index_token_stream(
        index: usize,
    ) -> proc_macro2::TokenStream {
        let value = format!("__field{index}");
        value
            .parse::<proc_macro2::TokenStream>()
            .expect("09a0c518-28da-455b-bce8-fb6defae8a3b")
    }
    fn generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
        field_name_double_quotes_token_stream: &dyn quote::ToTokens,
        index: usize,
    ) -> proc_macro2::TokenStream {
        let field_index_token_stream =
            generate_underscore_underscore_field_index_token_stream(index);
        quote::quote! {#field_name_double_quotes_token_stream => Ok(__Field::#field_index_token_stream)}
    }
    let vec_ident = vec_ident_type
        .iter()
        .map(|element| element.0)
        .collect::<Vec<&syn::Ident>>();
    let field_enum_variants_token_stream = {
        let field_enum_variants_token_stream = {
            let mut vec = Vec::new();
            for element in 0..len {
                let value = format!("__{}{element}", naming::FieldSnakeCase);
                vec.push(
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .expect("c46314b0-baee-41c8-b9c6-54b888310ca8"),
                );
            }
            vec
        };
        quote::quote! {#(#field_enum_variants_token_stream),*}
    };
    let visit_u64_value_enum_variants_token_stream = {
        let visit_u64_value_enum_variants_token_stream = {
            let mut acc = Vec::new();
            for index in 0..len {
                let index_u64_token_stream = {
                    let value = format!("{index}u64");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .expect("828ff7b4-5b7c-4109-8739-c6aa240f0f66")
                };
                let field_index_token_stream =
                    generate_underscore_underscore_field_index_token_stream(index);
                acc.push(quote::quote! {
                    #index_u64_token_stream => Ok(__Field::#field_index_token_stream)
                });
            }
            acc
        };
        quote::quote! {#(#visit_u64_value_enum_variants_token_stream),*}
    };
    let visit_str_value_enum_variants_token_stream = {
        let visit_str_value_enum_variants_token_stream =
            vec_ident.iter().enumerate().map(|(index, element)| {
                let field_name_double_quotes_token_stream =
                    generate_quotes::double_quotes_token_stream(&element);
                generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                    &field_name_double_quotes_token_stream,
                    index,
                )
            });
        quote::quote! {#(#visit_str_value_enum_variants_token_stream),*,}
    };
    let visit_bytes_value_enum_variants_token_stream = {
        let visit_bytes_value_enum_variants_token_stream =
            vec_ident.iter().enumerate().map(|(index, element)| {
                let b_field_name_double_quotes_token_stream = {
                    let element_ident_double_quotes_stringified =
                        generate_quotes::double_quotes_stringified(&element.to_string());
                    let value = format!("b{element_ident_double_quotes_stringified}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .expect("9e33625e-5f3d-4110-9641-204910c7f08e")
                };
                generate_field_ident_double_quotes_serde_private_ok_field_token_stream(
                    &b_field_name_double_quotes_token_stream,
                    index,
                )
            });
        quote::quote! {#(#visit_bytes_value_enum_variants_token_stream),*,}
    };
    let struct_ident_double_quotes_token_stream =
        generate_struct_ident_double_quotes_token_stream(&ident);
    let visit_seq_fields_initialization_token_stream = {
        let visit_seq_fields_initialization_token_stream = vec_ident_type.iter().enumerate().map(|(index, (element_ident, element_type))| {
            let field_index_token_stream = generate_underscore_underscore_field_index_token_stream(index);
            let type_token_stream = generate_type_token_stream(element_ident, element_type);
            let struct_ident_options_with_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("struct {ident} with {len} elements"));
            quote::quote! {
                let Some(#field_index_token_stream) = serde::de::SeqAccess::next_element::<#type_token_stream>(&mut __seq)? else {
                    return Err(serde::de::Error::invalid_length(0usize, &#struct_ident_options_with_double_quotes_token_stream));
                };
            }
        });
        quote::quote! {#(#visit_seq_fields_initialization_token_stream)*}
    };
    let match_try_new_in_deserialize_token_stream =
        generate_match_try_new_in_deserialize_token_stream(&ident, &{
            let fields_token_stream = {
                let mut acc = Vec::new();
                for element in 0..len {
                    acc.push(generate_underscore_underscore_field_index_token_stream(
                        element,
                    ));
                }
                acc
            };
            quote::quote! {#(#fields_token_stream),*}
        });
    let visit_map_fields_initialization_token_stream = {
        let content_token_stream =
            vec_ident_type
                .iter()
                .enumerate()
                .map(|(index, (element_ident, element_type))| {
                    let type_token_stream = generate_type_token_stream(element_ident, element_type);
                    let field_index_token_stream =
                        generate_underscore_underscore_field_index_token_stream(index);
                    quote::quote! {
                        let mut #field_index_token_stream: Option<#type_token_stream> = None;
                    }
                });
        quote::quote! {#(#content_token_stream)*}
    };
    let visit_map_match_variants_token_stream = {
        let visit_map_match_variants_token_stream = vec_ident_type.iter().enumerate().map(|(index, (element_ident, element_type))| {
            let field_index_token_stream = generate_underscore_underscore_field_index_token_stream(index);
            let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element_ident);
            let type_token_stream = generate_type_token_stream(element_ident, element_type);
            quote::quote! {
                __Field::#field_index_token_stream => {
                    if Option::is_some(&#field_index_token_stream) {
                        return Err(
                            <__A::Error as serde::de::Error>::duplicate_field(#field_ident_double_quotes_token_stream),
                        );
                    }
                    #field_index_token_stream = Some(
                        serde::de::MapAccess::next_value::<#type_token_stream>(&mut __map)?,
                    );
                }
            }
        });
        quote::quote! {#(#visit_map_match_variants_token_stream)*}
    };
    let visit_map_missing_fields_check_token_stream = {
        let content_token_stream = vec_ident.iter().enumerate().map(|(index, element)| {
            let field_index_token_stream = generate_underscore_underscore_field_index_token_stream(index);
            let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element);
            quote::quote! {
                let #field_index_token_stream = match #field_index_token_stream {
                    Some(some_value) => some_value,
                    None => {
                        serde::__private228::de::missing_field(#field_ident_double_quotes_token_stream)?
                    }
                };
            }
        });
        quote::quote! {#(#content_token_stream)*}
    };
    let fields_array_elements_token_stream = {
        let fields_array_elements_token_stream = vec_ident
            .iter()
            .map(|element| generate_quotes::double_quotes_token_stream(&element));
        quote::quote! {#(#fields_array_elements_token_stream),*}
    };
    let ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident);
    quote::quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #ident {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        #field_enum_variants_token_stream,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl _serde::de::Visitor<'_> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter<'_>,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                #visit_u64_value_enum_variants_token_stream,
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                #visit_str_value_enum_variants_token_stream
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                #visit_bytes_value_enum_variants_token_stream
                                _ => Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> Result<Self, __D::Error>
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
                        marker: _serde::__private228::PhantomData<#ident>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #ident;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter<'_>,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                #struct_ident_double_quotes_token_stream,
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            #visit_seq_fields_initialization_token_stream
                            #match_try_new_in_deserialize_token_stream
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            #visit_map_fields_initialization_token_stream
                            while let Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    #visit_map_match_variants_token_stream
                                    __Field::__ignore => {
                                        let _: serde::de::IgnoredAny = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            #visit_map_missing_fields_check_token_stream
                            #match_try_new_in_deserialize_token_stream
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &[&str] = &[#fields_array_elements_token_stream];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        #ident_double_quotes_token_stream,
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<Self>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
    }
}

pub fn wrap_content_into_scopes_token_stream(
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {(#content_token_stream)}
}

pub fn maybe_wrap_into_braces_token_stream(
    content_token_stream: &dyn quote::ToTokens,
    std_primitive_bool: bool,
) -> proc_macro2::TokenStream {
    if std_primitive_bool {
        wrap_content_into_scopes_token_stream(&content_token_stream)
    } else {
        quote::quote! {#content_token_stream}
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PostgresqlTypeOrPostgresqlJsonType {
    PostgresqlType,
    PostgresqlJsonType,
}

#[derive(Debug, Clone, Copy)]
pub enum DefaultSomeOneOrDefaultSomeOneWithMaxPageSize {
    DefaultSomeOne,
    DefaultSomeOneWithMaxPageSize,
}

pub fn generate_value_initialization_token_stream(
    import_path: &ImportPath,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    quote::quote! {#import_path::Value { #value_snake_case: #content_token_stream }}
}

#[derive(Debug, Clone, Copy)]
pub enum EqualOrEqualUsingFields {
    Equal,
    EqualUsingFields,
}

#[derive(Debug, Clone, Copy)]
pub enum EqualOperatorHandle {
    Equal,
    IsNull,
}
impl EqualOperatorHandle {
    pub fn to_tokens_path(&self, import_path: &ImportPath) -> proc_macro2::TokenStream {
        let equal_operator_upper_camel_case = naming::EqualOperatorUpperCamelCase;
        let content_token_stream = match &self {
            Self::Equal => quote::quote! {Equal},
            Self::IsNull => quote::quote! {IsNull},
        };
        quote::quote! {#import_path::#equal_operator_upper_camel_case::#content_token_stream}
    }
}
pub fn impl_postgresql_type_equal_operator_for_ident_token_stream(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_equal_operator_upper_camel_case =
        naming::PostgresqlTypeEqualOperatorUpperCamelCase;
    let equal_operator_upper_camel_case = naming::EqualOperatorUpperCamelCase;
    quote::quote! {
        impl #import_path::#postgresql_type_equal_operator_upper_camel_case for #ident {
            fn operator(&self) -> #import_path::#equal_operator_upper_camel_case {
                #content_token_stream
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    One,
    Two,
    Three,
    Four,
}
impl Dimension {
    pub fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case(
        &self,
    ) -> Box<dyn naming::StdFmtDisplayPlusQuoteToTokens> {
        match self {
            Self::One => Box::new(naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionOneEqualSnakeCase),
            Self::Two => Box::new(naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionTwoEqualSnakeCase),
            Self::Three => Box::new(naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionThreeEqualSnakeCase),
            Self::Four => Box::new(naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionFourEqualSnakeCase),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum DimensionIndexNumber {
    Zero,
    One,
    Two,
    Three,
}
impl From<&Dimension> for DimensionIndexNumber {
    fn from(value: &Dimension) -> Self {
        match &value {
            Dimension::One => Self::Zero,
            Dimension::Two => Self::One,
            Dimension::Three => Self::Two,
            Dimension::Four => Self::Three,
        }
    }
}

pub fn generate_query_part_error_named_write_into_buffer_token_stream(
    import_path: ImportPath,
) -> proc_macro2::TokenStream {
    quote::quote! {
        #import_path::QueryPartErrorNamed::WriteIntoBuffer {
            code_occurence: error_occurence_lib::code_occurence!()
        }
    }
}
pub fn generate_return_err_query_part_error_named_write_into_buffer_token_stream(
    import_path: ImportPath,
) -> proc_macro2::TokenStream {
    let content_token_stream =
        generate_query_part_error_named_write_into_buffer_token_stream(import_path);
    quote::quote! {return Err(#content_token_stream);}
}
