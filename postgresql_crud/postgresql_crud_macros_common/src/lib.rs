mod filters;

pub use filters::*;

pub enum DeriveOrImpl {
    Derive,
    Impl(proc_macro2::TokenStream),
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum NotNullOrNullable {
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
    pub fn maybe_option_wrap(&self, content_token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match &self {
            Self::NotNull => content_token_stream,
            Self::Nullable => quote::quote! {std::option::Option<#content_token_stream>},
        }
    }
    pub fn maybe_some_wrap(&self, content_token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match &self {
            Self::NotNull => content_token_stream,
            Self::Nullable => quote::quote! {Some(#content_token_stream)},
        }
    }
    //json
    pub fn prefix_stringified(&self) -> std::string::String {
        match &self {
            Self::NotNull => std::string::String::default(),
            Self::Nullable => std::string::String::from("StdOptionOption"),
        }
    }
}
impl std::default::Default for NotNullOrNullable {
    fn default() -> Self {
        Self::NotNull
    }
}

pub fn generate_postgresql_type_where_element_token_stream(
    variants: &std::vec::Vec<&dyn crate::PostgresqlFilter>,
    prefix: &dyn quote::ToTokens,
    should_implement_schemars_json_schema: &crate::ShouldDeriveSchemarsJsonSchema,
    is_query_bind_mutable: &IsQueryBindMutable,
) -> proc_macro2::TokenStream {
    let ident = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&prefix);
    let value_snake_case = naming::ValueSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let postgresql_type_tokens_where_element_token_stream = {
        let variants_token_stream = variants.iter().map(|element| {
            let element_upper_camel_case = element.upper_camel_case();
            let prefix_where_element_self_upper_camel_case = element.prefix_where_element_self_upper_camel_case();
            let option_type_token_stream: std::option::Option<proc_macro2::TokenStream> = element.maybe_generic();
            let type_token_stream = match option_type_token_stream {
                Some(value) => quote::quote!{<#value>},
                None => proc_macro2::TokenStream::new(),
            };
            quote::quote! {#element_upper_camel_case(crate::where_element_filters::#prefix_where_element_self_upper_camel_case #type_token_stream)}
        });
        quote::quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_implement_schemars_json_schema)]
            pub enum #ident {
                #(#variants_token_stream),*
            }
        }
    };
    let impl_crate_postgresql_type_postgresql_type_where_filter_for_postgresql_type_tokens_where_element_token_stream = impl_postgresql_type_where_filter_for_ident_token_stream(
        &quote::quote! {<'a>},
        &ident,
        &proc_macro2::TokenStream::new(),
        &IncrementParameterUnderscore::False,
        &IsNeedToAddLogicalOperatorUnderscore::False,
        &{
            let variants_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.upper_camel_case();
                quote::quote! {
                    Self::#element_upper_camel_case(#value_snake_case) => crate::PostgresqlTypeWhereFilter::query_part(
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
                    Self::#element_upper_camel_case(#value_snake_case) => crate::PostgresqlTypeWhereFilter::query_bind(
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
        &crate::ImportPath::Crate,
    );
    let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_element_token_stream =
        macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident, &proc_macro2::TokenStream::new(), &quote::quote! {format!("{self:#?}")});
    let impl_crate_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_tokens_where_element_token_stream =
        crate::generate_impl_crate_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident, &{
            let variants_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.upper_camel_case();
                let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
                quote::quote! {
                    Self::#element_upper_camel_case(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                }
            });
            quote::quote! {vec![#(#variants_token_stream),*]}
        });
    quote::quote! {
        #postgresql_type_tokens_where_element_token_stream
        #impl_crate_postgresql_type_postgresql_type_where_filter_for_postgresql_type_tokens_where_element_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_element_token_stream
        #impl_crate_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_tokens_where_element_token_stream
    }
}

pub fn crate_query_part_error_named_token_stream() -> proc_macro2::TokenStream {
    let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
    quote::quote! {crate::#query_part_error_named_upper_camel_case}
}

pub fn generate_struct_ident_double_quotes_token_stream(postgresql_type_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_token_stream(&format!("struct {postgresql_type_where_element_tokens_upper_camel_case}"))
}
pub fn generate_struct_ident_with_number_elements_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens, length: std::primitive::usize) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_token_stream(&format!("struct {postgresql_type_ident_where_element_tokens_upper_camel_case} with {length} elements"))
}

pub fn generate_sqlx_types_json_type_declaration_token_stream(type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {sqlx::types::Json<#type_token_stream>}
}
pub fn generate_std_option_option_tokens_declaration_token_stream(type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {std::option::Option<#type_token_stream>}
}
pub fn generate_std_vec_vec_tokens_declaration_token_stream(type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {std::vec::Vec<#type_token_stream>}
}

pub fn generate_serde_deserialize_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens, length: std::primitive::usize) -> (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream = generate_struct_ident_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case);
    let struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream = generate_struct_ident_with_number_elements_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case, length);
    let postgresql_type_ident_where_element_tokens_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&postgresql_type_ident_where_element_tokens_upper_camel_case);
    (
        struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream,
        struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream,
        postgresql_type_ident_where_element_tokens_double_quotes_token_stream,
    )
}

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

pub enum IsCreateQueryPartSelfCreateUsed {
    True,
    False,
}
impl quote::ToTokens for IsCreateQueryPartSelfCreateUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::ValueSnakeCase.to_tokens(tokens),
            Self::False => quote::quote!{_}.to_tokens(tokens),
        }
    }
}
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
pub enum IsSelectQueryPartSelfSelectUsed {
    True,
    False,
}
impl quote::ToTokens for IsSelectQueryPartSelfSelectUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::ValueSnakeCase.to_tokens(tokens),
            Self::False => quote::quote!{_}.to_tokens(tokens),
        }
    }
}
pub enum IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed {
    True,
    False,
}
impl quote::ToTokens for IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::ColumnNameAndMaybeFieldGetterForErrorMessageSnakeCase.to_tokens(tokens),
            Self::False => quote::quote!{_}.to_tokens(tokens),
        }
    }
}
pub enum IsSelectQueryPartIsPostgresqlTypeUsed {
    True,
    False,
}
impl quote::ToTokens for IsSelectQueryPartIsPostgresqlTypeUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote!{is_postgresql_type}.to_tokens(tokens),
            Self::False => quote::quote!{_}.to_tokens(tokens),
        }
    }
}
pub enum IsUpdateQueryPartSelfUpdateUsed {
    True,
    False,
}
impl quote::ToTokens for IsUpdateQueryPartSelfUpdateUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::ValueSnakeCase.to_tokens(tokens),
            Self::False => quote::quote!{_}.to_tokens(tokens),
        }
    }
}
pub enum IsUpdateQueryPartJsonbSetTargetUsed {
    True,
    False,
}
impl quote::ToTokens for IsUpdateQueryPartJsonbSetTargetUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => naming::JsonbSetTargetSnakeCase.to_tokens(tokens),
            Self::False => quote::quote!{_}.to_tokens(tokens),
        }
    }
}
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
pub fn generate_postgresql_json_type_token_stream(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    table_type_declaration_type_token_stream: &dyn quote::ToTokens,
    create_type_token_stream: &dyn quote::ToTokens,
    is_create_query_part_self_create_used: &IsCreateQueryPartSelfCreateUsed,
    create_query_part_token_stream: &dyn quote::ToTokens,
    is_create_query_bind_mutable: &IsCreateQueryBindMutable,
    create_query_bind_token_stream: &dyn quote::ToTokens,
    select_type_token_stream: &dyn quote::ToTokens,
    is_select_query_part_self_select_used: &IsSelectQueryPartSelfSelectUsed,
    is_select_query_part_column_name_and_maybe_field_getter_for_error_message_used: &IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed,
    is_select_query_part_is_postgresql_type_used: &IsSelectQueryPartIsPostgresqlTypeUsed,
    select_query_part_token_stream: &dyn quote::ToTokens,
    where_element_type_token_stream: &dyn quote::ToTokens,
    read_type_token_stream: &dyn quote::ToTokens,
    read_inner_type_token_stream: &dyn quote::ToTokens,
    into_inner_token_stream: &dyn quote::ToTokens,
    update_type_token_stream: &dyn quote::ToTokens,
    update_query_part_token_stream: &dyn quote::ToTokens,
    is_update_query_part_self_update_used: &IsUpdateQueryPartSelfUpdateUsed,
    is_update_query_part_jsonb_set_target_used: &IsUpdateQueryPartJsonbSetTargetUsed,
    is_update_query_bind_mutable: &IsUpdateQueryBindMutable,
    update_query_bind_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let path_token_stream = quote::quote! {#import_path ::};
    let table_type_declaration_upper_camel_case = naming::TableTypeDeclarationUpperCamelCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let value_snake_case = naming::ValueSnakeCase;
    let select_upper_camel_case = naming::SelectUpperCamelCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    let read_inner_upper_camel_case = naming::ReadInnerUpperCamelCase;
    let where_element_upper_camel_case = naming::WhereElementUpperCamelCase;
    let update_upper_camel_case = naming::UpdateUpperCamelCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
    let query_snake_case = naming::QuerySnakeCase;
    let field_ident_snake_case = naming::FieldIdentSnakeCase;
    let column_name_and_maybe_field_getter_snake_case = naming::ColumnNameAndMaybeFieldGetterSnakeCase;
    let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
    let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;
    let create_query_part_snake_case = naming::CreateQueryPartSnakeCase;
    let create_query_bind_snake_case = naming::CreateQueryBindSnakeCase;
    let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
    let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
    let reference_std_primitive_str_token_stream = token_patterns::RefStdPrimitiveStr;
    let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;
    let reference_mut_std_primitive_u64_token_stream = {
        let std_primitive_u64_token_stream = token_patterns::StdPrimitiveU64;
        quote::quote! {&mut #std_primitive_u64_token_stream}
    };
    let query_postgres_arguments_token_stream = quote::quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let std_string_string_token_stream = token_patterns::StdStringString;
    //todo maybe reexport sqlx?
    quote::quote! {
        impl #path_token_stream #postgresql_json_type_upper_camel_case for #ident {
            type #table_type_declaration_upper_camel_case = #table_type_declaration_type_token_stream;
            type #create_upper_camel_case = #create_type_token_stream;
            fn #create_query_part_snake_case(
                #is_create_query_part_self_create_used: &Self::#create_upper_camel_case,
                #increment_snake_case: #reference_mut_std_primitive_u64_token_stream
            ) -> Result<#std_string_string_token_stream, #path_token_stream QueryPartErrorNamed> {
                #create_query_part_token_stream
            }
            fn #create_query_bind_snake_case(
                #value_snake_case: Self::#create_upper_camel_case,
                #is_create_query_bind_mutable #query_snake_case: #query_postgres_arguments_token_stream
            ) -> #query_postgres_arguments_token_stream {
                #create_query_bind_token_stream
            }
            type #select_upper_camel_case = #select_type_token_stream;
            fn #select_query_part_snake_case(
                #is_select_query_part_self_select_used: &Self::#select_upper_camel_case,
                #field_ident_snake_case: #reference_std_primitive_str_token_stream,
                #column_name_and_maybe_field_getter_snake_case: #reference_std_primitive_str_token_stream,
                #is_select_query_part_column_name_and_maybe_field_getter_for_error_message_used: #reference_std_primitive_str_token_stream,
                #is_select_query_part_is_postgresql_type_used: #std_primitive_bool_token_stream,
            ) -> #std_string_string_token_stream {
                #select_query_part_token_stream
            }
            type #where_element_upper_camel_case = #where_element_type_token_stream;
            type #read_upper_camel_case = #read_type_token_stream;
            type #read_inner_upper_camel_case = #read_inner_type_token_stream;
            fn into_inner(#value_snake_case: Self::#read_upper_camel_case) -> Self::#read_inner_upper_camel_case {
                #into_inner_token_stream
            }
            type #update_upper_camel_case = #update_type_token_stream;
            fn #update_query_part_snake_case(
                #is_update_query_part_self_update_used: &Self::#update_upper_camel_case,
                #jsonb_set_accumulator_snake_case: #reference_std_primitive_str_token_stream,
                #is_update_query_part_jsonb_set_target_used: #reference_std_primitive_str_token_stream,
                #jsonb_set_path_snake_case: #reference_std_primitive_str_token_stream,
                #increment_snake_case: #reference_mut_std_primitive_u64_token_stream,
            ) -> Result<#std_string_string_token_stream, #path_token_stream QueryPartErrorNamed> {
                #update_query_part_token_stream
            }
            fn #update_query_bind_snake_case(
                #value_snake_case: Self::#update_upper_camel_case,
                #is_update_query_bind_mutable #query_snake_case: #query_postgres_arguments_token_stream
            ) -> #query_postgres_arguments_token_stream {
                #update_query_bind_token_stream
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
    let path_trait_token_stream = import_path.default_but_option_is_always_some_and_vec_always_contains_one_element();
    let default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementSnakeCase;
    quote::quote! {
        impl #impl_generic_token_stream #path_trait_token_stream for #ident #ident_generic_token_stream {
            fn #default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case() -> Self {
                #content_token_stream
            }
        }
    }
}
pub fn generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(import_path: &ImportPath, ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let path_trait_token_stream = import_path.all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element();
    let all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::AllEnumVariantsArrayDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
    quote::quote! {
        impl #path_trait_token_stream for #ident {
            fn #all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case() -> std::vec::Vec<Self> {
                #content_token_stream
            }
        }
    }
}

pub fn generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&proc_macro2::TokenStream::new(), &ImportPath::Crate, ident, &proc_macro2::TokenStream::new(), content_token_stream)
}
pub fn generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(ident: &dyn quote::ToTokens, lifetime_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&proc_macro2::TokenStream::new(), &ImportPath::PostgresqlCrud, ident, lifetime_token_stream, content_token_stream)
}
pub fn generate_impl_crate_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ImportPath::Crate, ident, content_token_stream)
}
pub fn generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ImportPath::PostgresqlCrud, ident, content_token_stream)
}

pub enum ImportPath {
    Crate,
    PostgresqlCrud,
}
impl quote::ToTokens for ImportPath {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::Crate => quote::quote! {crate},
            Self::PostgresqlCrud => quote::quote! {postgresql_crud},
        }
        .to_tokens(tokens)
    }
}
impl ImportPath {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            ImportPath::Crate => &token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
            ImportPath::PostgresqlCrud => &token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
        }
    }
    fn all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            ImportPath::Crate => &token_patterns::CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
            ImportPath::PostgresqlCrud => &token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
        }
    }
    pub fn to_path(&self) -> &'static std::primitive::str {
        match &self {
            ImportPath::Crate => "crate",
            ImportPath::PostgresqlCrud => "postgresql_crud",
        }
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum IncrementParameterUnderscore {
    True,
    False,
}
impl quote::ToTokens for IncrementParameterUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote!{_}.to_tokens(tokens),
            Self::False => naming::IncrementSnakeCase.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IsNeedToAddLogicalOperatorUnderscore {
    True,
    False,
}
impl quote::ToTokens for IsNeedToAddLogicalOperatorUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote!{_}.to_tokens(tokens),
            Self::False => naming::IsNeedToAddLogicalOperatorSnakeCase.to_tokens(tokens),
        }
    }
}
pub fn impl_postgresql_type_where_filter_for_ident_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    ident_token_stream: &dyn quote::ToTokens,
    ident_generic_token_stream: &dyn quote::ToTokens,
    increment_parameter_underscore: &IncrementParameterUnderscore,
    is_need_to_add_logical_operator_underscore: &IsNeedToAddLogicalOperatorUnderscore,
    query_part_content_token_stream: &dyn quote::ToTokens,
    is_query_bind_mutable: &IsQueryBindMutable,
    query_bind_content_token_stream: &dyn quote::ToTokens,
    import_path: &ImportPath,
) -> proc_macro2::TokenStream {
    let column_snake_case = naming::ColumnSnakeCase;
    let std_primitive_u64_token_stream = token_patterns::StdPrimitiveU64;
    let std_fmt_display_token_stream = token_patterns::StdFmtDisplay;
    let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;
    let std_string_string_token_stream = token_patterns::StdStringString;
    let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
    let query_part_snake_case = naming::QueryPartSnakeCase;
    let query_bind_snake_case = naming::QueryBindSnakeCase;
    let postgresql_type_where_filter_upper_camel_case = naming::PostgresqlTypeWhereFilterUpperCamelCase;
    quote::quote! {
        impl #impl_generic_token_stream #import_path ::#postgresql_type_where_filter_upper_camel_case<'a> for #ident_token_stream #ident_generic_token_stream {
            fn #query_part_snake_case(
                &self,
                #increment_parameter_underscore: &mut #std_primitive_u64_token_stream,
                #column_snake_case: &dyn #std_fmt_display_token_stream,
                #is_need_to_add_logical_operator_underscore: #std_primitive_bool_token_stream
            ) -> Result<#std_string_string_token_stream, #import_path::#query_part_error_named_upper_camel_case> {
                #query_part_content_token_stream
            }
            fn #query_bind_snake_case(self, #is_query_bind_mutable query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #query_bind_content_token_stream
            }
        }
    }
}

pub fn generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(ident_token_stream: &dyn quote::ToTokens, type_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
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
pub fn generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(ident_token_stream: &dyn quote::ToTokens, type_token_stream: &dyn quote::ToTokens, ok_value_match_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    quote::quote! {
        impl sqlx::Decode<'_, sqlx::Postgres> for #ident_token_stream {
            fn decode(#value_snake_case: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                match <#type_token_stream as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                    Ok(value) => #ok_value_match_token_stream,
                    Err(error) => Err(error),
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum CreateQueryPartValueUnderscore {
    True,
    False
}
impl quote::ToTokens for CreateQueryPartValueUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote!{_}.to_tokens(tokens),
            Self::False => naming::ValueSnakeCase.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CreateQueryPartIncrementUnderscore {
    True,
    False
}
impl quote::ToTokens for CreateQueryPartIncrementUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote!{_}.to_tokens(tokens),
            Self::False => naming::IncrementSnakeCase.to_tokens(tokens),
        }
    }
}
pub fn generate_impl_postgresql_type_for_ident_token_stream(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    ident_table_type_declaration_upper_camel_case: &dyn quote::ToTokens,
    ident_create_upper_camel_case: &dyn quote::ToTokens,
    create_query_part_value_underscore: &CreateQueryPartValueUnderscore,
    create_query_part_increment_underscore: &CreateQueryPartIncrementUnderscore,
    create_query_part_content_token_stream: &dyn quote::ToTokens,
    is_create_query_bind_mutable: &IsCreateQueryBindMutable,
    create_query_bind_content_token_stream: &dyn quote::ToTokens,
    ident_select_upper_camel_case: &dyn quote::ToTokens,
    select_query_part_content_token_stream: &dyn quote::ToTokens,
    ident_where_element_upper_camel_case: &dyn quote::ToTokens,
    ident_read_upper_camel_case: &dyn quote::ToTokens,
    ident_read_inner_upper_camel_case: &dyn quote::ToTokens,
    into_inner_token_stream: &dyn quote::ToTokens,
    ident_update_upper_camel_case: &dyn quote::ToTokens,
    update_query_part_content_token_stream: &dyn quote::ToTokens,
    is_update_query_bind_mutable: &IsUpdateQueryBindMutable,
    update_query_bind_content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_upper_camel_case = naming::PostgresqlTypeUpperCamelCase;
    let table_type_declaration_upper_camel_case = naming::TableTypeDeclarationUpperCamelCase;
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let create_query_part_snake_case = naming::CreateQueryPartSnakeCase;
    let create_query_bind_snake_case = naming::CreateQueryBindSnakeCase;
    let select_upper_camel_case = naming::SelectUpperCamelCase;
    let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
    let where_element_upper_camel_case = naming::WhereElementUpperCamelCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    let read_inner_upper_camel_case = naming::ReadInnerUpperCamelCase;
    let update_upper_camel_case = naming::UpdateUpperCamelCase;
    let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
    let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
    let jsonb_set_target_snake_case = naming::JsonbSetTargetSnakeCase;
    let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;

    let value_snake_case = naming::ValueSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let std_string_string_token_stream = token_patterns::StdStringString;
    quote::quote! {
        impl #import_path :: #postgresql_type_upper_camel_case for #ident {
            type #table_type_declaration_upper_camel_case = #ident_table_type_declaration_upper_camel_case;
            type #create_upper_camel_case = #ident_create_upper_camel_case;
            fn #create_query_part_snake_case(
                #create_query_part_value_underscore: &Self::#create_upper_camel_case,
                #create_query_part_increment_underscore: &mut std::primitive::u64
            ) -> Result<#std_string_string_token_stream, #import_path ::QueryPartErrorNamed> {
                #create_query_part_content_token_stream
            }
            fn #create_query_bind_snake_case(
                #value_snake_case: Self::#create_upper_camel_case,
                #is_create_query_bind_mutable #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #create_query_bind_content_token_stream
            }
            type #select_upper_camel_case = #ident_select_upper_camel_case;
            fn #select_query_part_snake_case(
                #value_snake_case: &Self::#select_upper_camel_case,
                #column_snake_case: &std::primitive::str,
            ) -> #std_string_string_token_stream {
                #select_query_part_content_token_stream
            }
            type #where_element_upper_camel_case = #ident_where_element_upper_camel_case;
            type #read_upper_camel_case = #ident_read_upper_camel_case;
            type #read_inner_upper_camel_case = #ident_read_inner_upper_camel_case;
            fn into_inner(#value_snake_case: Self::#read_upper_camel_case) -> Self::#read_inner_upper_camel_case {
                #into_inner_token_stream
            }
            type #update_upper_camel_case = #ident_update_upper_camel_case;
            fn #update_query_part_snake_case(
                #value_snake_case: &Self::#update_upper_camel_case,
                #jsonb_set_accumulator_snake_case: &std::primitive::str,
                #jsonb_set_target_snake_case: &std::primitive::str,
                #jsonb_set_path_snake_case: &std::primitive::str,
                #increment_snake_case: &mut std::primitive::u64
            ) -> Result<#std_string_string_token_stream, #import_path ::QueryPartErrorNamed> {
                #update_query_part_content_token_stream
            }
            fn #update_query_bind_snake_case<'a>(
                #value_snake_case: Self::#update_upper_camel_case,
                #is_update_query_bind_mutable #query_snake_case: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #update_query_bind_content_token_stream
            }
        }
    }
}

pub enum IsPrimaryKeyUsed {
    True,
    False
}
pub fn generate_create_table_column_query_part_token_stream(
    ident: &dyn quote::ToTokens,
    is_primary_key_used: IsPrimaryKeyUsed,
    maybe_fixed_length_parameter_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens
) -> proc_macro2::TokenStream {
    let create_table_column_query_part_snake_case = naming::CreateTableColumnQueryPartSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let std_fmt_display_token_stream = token_patterns::StdFmtDisplay;
    let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;
    let is_primary_key_or_underscore_token_stream: &dyn quote::ToTokens = match &is_primary_key_used {
        IsPrimaryKeyUsed::True => &naming::IsPrimaryKeySnakeCase,
        IsPrimaryKeyUsed::False => &quote::quote!{_}
    };
    quote::quote! {
        impl #ident {
            pub fn #create_table_column_query_part_snake_case(
                #column_snake_case: &dyn #std_fmt_display_token_stream,
                #is_primary_key_or_underscore_token_stream: #std_primitive_bool_token_stream #maybe_fixed_length_parameter_token_stream
            ) -> impl #std_fmt_display_token_stream {
                #content_token_stream
            }
        }
    }
}

pub fn crate_query_part_error_named_checked_add_initialization_token_stream() -> proc_macro2::TokenStream {
    quote::quote! {crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }}
}

pub fn generate_impl_crate_is_string_empty_for_ident_token_stream(ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {
        impl crate::IsStringEmpty for #ident {
            fn is_string_empty(&self) -> std::primitive::bool {
                self.0.to_string().is_empty()
            }
        }
    }
}

pub fn generate_match_try_new_in_deserialize_token_stream(ident: &dyn quote::ToTokens, initialization_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {
        match #ident::try_new(#initialization_token_stream) {
            Ok(value) => serde::__private::Ok(value),
            Err(error) => {
                return Err(serde::de::Error::custom(format!("{error:?}")));
            }
        }
    }
}
