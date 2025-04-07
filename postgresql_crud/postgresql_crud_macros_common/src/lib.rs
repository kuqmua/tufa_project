mod filters;

pub use filters::*;

pub enum PostgresqlTypeNotNullOrNullable {
    NotNull,
    Nullable,
}

pub fn generate_postgresql_type_where_element_token_stream<'a, GenerateGenericTypeTokenStream>(
    variants: &std::vec::Vec<&dyn crate::PostgresqlFilter>,
    generate_where_element_variants_types_generic_token_stream: GenerateGenericTypeTokenStream,
    prefix: &dyn std::fmt::Display,
    should_implement_schemars_json_schema: &crate::ShouldDeriveSchemarsJsonSchema,
) -> proc_macro2::TokenStream 
where
    GenerateGenericTypeTokenStream: Fn(std::primitive::bool) -> &'a dyn naming::StdFmtDisplayPlusQuoteToTokens,
{
    let ident = naming::parameter::SelfWhereElementUpperCamelCase::from_display(&prefix);
    let value_snake_case = naming::ValueSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let postgresql_type_tokens_where_element_token_stream = {
        let variants_token_stream = variants.iter().map(|element| {
            let element_upper_camel_case = element.upper_camel_case();
            let type_token_stream = {
                let prefix_where_element_self_upper_camel_case = element.prefix_where_element_self_upper_camel_case();
                let maybe_generic_token_stream = if element.has_generic() {
                    let type_token_stream = generate_where_element_variants_types_generic_token_stream(element.is_relevant_only_for_not_null());
                    quote::quote! {<#type_token_stream>}
                }
                else {
                    proc_macro2::TokenStream::new()
                };
                quote::quote! {crate::where_element_filters::#prefix_where_element_self_upper_camel_case #maybe_generic_token_stream}
            };
            quote::quote! {#element_upper_camel_case(#type_token_stream)}
        });
        quote::quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_implement_schemars_json_schema)]
            pub enum #ident {
                #(#variants_token_stream),*
            }
        }
    };
    let impl_crate_postgresql_type_postgresql_type_where_filter_for_postgresql_type_tokens_where_element_token_stream = crate::impl_postgresql_type_where_filter_for_ident_token_stream(
        &quote::quote!{<'a>},
        &ident,
        &proc_macro2::TokenStream::new(),
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
    let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_element_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(
        &proc_macro2::TokenStream::new(),
        &ident,
        &proc_macro2::TokenStream::new(),
        &quote::quote! {format!("{self:#?}")}
    );
    let impl_crate_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_tokens_where_element_token_stream =
        crate::generate_impl_crate_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident, &{
            let variants_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.upper_camel_case();
                let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream =
                    token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
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

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum PostgresqlJsonTypePatternType {
    FullTypePath,
    StdVecVecFullTypePath,
    StdVecVecStdVecVecFullTypePath,
}
impl PostgresqlJsonTypePatternType {
    pub fn prefix_stringified(&self) -> std::string::String {
        match &self {
            PostgresqlJsonTypePatternType::FullTypePath => std::string::String::default(),
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath => naming::StdVecVecUpperCamelCase.to_string(),
            PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => naming::StdVecVecStdVecVecUpperCamelCase.to_string(),
        }
    }
}

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum PostgresqlJsonTypePatternIsOptional {
    True,
    False,
}
impl PostgresqlJsonTypePatternIsOptional {
    fn prefix_stringified(&self) -> std::string::String {
        match &self {
            Self::True => std::string::String::from("StdOptionOption"),
            Self::False => std::string::String::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostgresqlJsonTypePattern {
    pub postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional,
    pub postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType,
}
impl PostgresqlJsonTypePattern {
    fn prefix_stringified(&self) -> std::string::String {
        format!("{}{}", &self.postgresql_json_type_pattern_is_optional.prefix_stringified(), &self.postgresql_json_type_pattern_type.prefix_stringified(),)
    }
    // fn all_variants() -> std::vec::Vec<Self> {
    //     let mut acc = vec![];
    //     for postgresql_json_type_pattern_type in PostgresqlJsonTypePatternType::into_array() {
    //         for postgresql_json_type_pattern_is_optional in PostgresqlJsonTypePatternIsOptional::into_array() {
    //             acc.push(Self {
    //                 postgresql_json_type_pattern_is_optional: postgresql_json_type_pattern_is_optional,
    //                 postgresql_json_type_pattern_type: postgresql_json_type_pattern_type.clone(),
    //             });
    //         }
    //     }
    //     acc
    // }
}

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
pub enum PostgresqlJsonTypeHandle {
    StdPrimitiveI8,
    StdPrimitiveI16,
    StdPrimitiveI32,
    StdPrimitiveI64,
    StdPrimitiveU8,
    StdPrimitiveU16,
    StdPrimitiveU32,
    StdPrimitiveU64,
    StdPrimitiveF32,
    StdPrimitiveF64,
    StdPrimitiveBool,
    StdStringString,
    UuidUuid,
}
impl PostgresqlJsonTypeHandle {
    fn field_type_stringified(&self) -> &'static std::primitive::str {
        match &self {
            Self::StdPrimitiveI8 => "std::primitive::i8",
            Self::StdPrimitiveI16 => "std::primitive::i16",
            Self::StdPrimitiveI32 => "std::primitive::i32",
            Self::StdPrimitiveI64 => "std::primitive::i64",
            Self::StdPrimitiveU8 => "std::primitive::u8",
            Self::StdPrimitiveU16 => "std::primitive::u16",
            Self::StdPrimitiveU32 => "std::primitive::u32",
            Self::StdPrimitiveU64 => "std::primitive::u64",
            Self::StdPrimitiveF32 => "std::primitive::f32",
            Self::StdPrimitiveF64 => "std::primitive::f64",
            Self::StdPrimitiveBool => "std::primitive::bool",
            Self::StdStringString => "std::string::String",
            Self::UuidUuid => "uuid::Uuid",
        }
    }
    fn field_type_token_stream(&self) -> proc_macro2::TokenStream {
        self.field_type_stringified().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonTypeHandle to proc_macro2::TokenStream"))
    }
    fn full_type_path_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            Self::StdPrimitiveI8
            | Self::StdPrimitiveI16
            | Self::StdPrimitiveI32
            | Self::StdPrimitiveI64
            | Self::StdPrimitiveU8
            | Self::StdPrimitiveU16
            | Self::StdPrimitiveU32
            | Self::StdPrimitiveU64
            | Self::StdPrimitiveF32
            | Self::StdPrimitiveF64
            | Self::StdPrimitiveBool
            | Self::StdStringString => {
                let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
                quote::quote! {#core_default_default_default_token_stream}
            }
            Self::UuidUuid => quote::quote! {
                uuid::Uuid::new_v4()
            },
        }
    }
}
impl quote::ToTokens for PostgresqlJsonTypeHandle {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.to_string().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonTypeHandle to proc_macro2::TokenStream")).to_tokens(tokens)
    }
}

pub struct PostgresqlJsonTypeVariant {
    pub postgresql_json_type_handle: PostgresqlJsonTypeHandle,
    pub postgresql_json_type_pattern: PostgresqlJsonTypePattern,
}
impl PostgresqlJsonTypeVariant {
    // fn all_variants() -> std::vec::Vec<Self> {
    //     let mut acc = vec![];
    //     for postgresql_json_type_handle in PostgresqlJsonTypeHandle::into_array() {
    //         for postgresql_json_type_pattern in PostgresqlJsonTypePattern::all_variants() {
    //             acc.push(Self {
    //                 postgresql_json_type_handle: postgresql_json_type_handle.clone(),
    //                 postgresql_json_type_pattern: postgresql_json_type_pattern,
    //             });
    //         }
    //     }
    //     acc
    // }
    pub fn is_vec_element_type(&self) -> std::primitive::bool {
        match &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
            PostgresqlJsonTypePatternType::FullTypePath => false,
            //todo maybe wrong
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath |
            PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => true,
        }
    }
    pub fn postgresql_json_type_ident_wrapper(&self) -> proc_macro2::TokenStream {
        format!("{}{}", self.postgresql_json_type_pattern.prefix_stringified(), self.postgresql_json_type_handle).parse::<proc_macro2::TokenStream>().unwrap()
    }

    pub fn handle_field_type(&self, is_wrapper: std::primitive::bool) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &self.postgresql_json_type_handle;
        match (&self.postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::FullTypePath) => {
                if is_wrapper {
                    quote::quote! {#postgresql_json_type_handle}
                } else {
                    postgresql_json_type_handle.field_type_token_stream()
                }
            }
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::FullTypePath) => quote::quote! {std::option::Option<#postgresql_json_type_handle>},
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => quote::quote! {std::vec::Vec<#postgresql_json_type_handle>},

            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => {
                // let value = {
                //     format!("{}{postgresql_json_type_handle}", &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type.prefix_stringified())
                //     .parse::<proc_macro2::TokenStream>()
                //     .unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonTypeHandle to proc_macro2::TokenStream"))
                // };
                // quote::quote!{std::option::Option<#value>}
                quote::quote! {std::option::Option<std::vec::Vec<#postgresql_json_type_handle>>}
            }
            (&PostgresqlJsonTypePatternIsOptional::False, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote! {std::vec::Vec<std::vec::Vec<#postgresql_json_type_handle>>}
            }
            (&PostgresqlJsonTypePatternIsOptional::True, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote! {std::option::Option<std::vec::Vec<std::vec::Vec<#postgresql_json_type_handle>>>}
            }
        }
    }
    pub fn handle_initialization_token_stream(&self, is_wrapper: std::primitive::bool) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &self.postgresql_json_type_handle;
        let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream =
            token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
        match (&self.postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::FullTypePath) => {
                if is_wrapper {
                    quote::quote! {#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream}
                } else {
                    postgresql_json_type_handle.full_type_path_initialization_token_stream()
                }
            }
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::FullTypePath) => quote::quote! {Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => quote::quote! {vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]},
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => {
                quote::quote! {Some(vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])}
                // quote::quote!{Some(#default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)}
            }
            (&PostgresqlJsonTypePatternIsOptional::False, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote! {vec![vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]]}
            }
            (&PostgresqlJsonTypePatternIsOptional::True, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote! {Some(vec![vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]])}
            }
        }
    }
    pub fn field_type(&self) -> proc_macro2::TokenStream {
        self.handle_field_type(false)
    }
    pub fn initialization_token_stream(&self) -> proc_macro2::TokenStream {
        self.handle_initialization_token_stream(false)
    }

    pub fn wrapper_field_type(&self) -> proc_macro2::TokenStream {
        self.handle_field_type(true)
    }
    pub fn wrapper_non_optional_field_type(&self) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &self.postgresql_json_type_handle;
        match &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
            PostgresqlJsonTypePatternType::FullTypePath => quote::quote! {#postgresql_json_type_handle},
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath => quote::quote! {std::vec::Vec<#postgresql_json_type_handle>},
            PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => quote::quote! {std::vec::Vec<std::vec::Vec<#postgresql_json_type_handle>>},
        }
    }
    pub fn wrapper_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        self.handle_initialization_token_stream(true)
    }
    pub fn wrapper_non_optional_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream =
            token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
        match &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
            PostgresqlJsonTypePatternType::FullTypePath => quote::quote! {#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream},
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath => quote::quote! {vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]},
            PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => quote::quote! {vec![vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]]},
        }
    }
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
impl std::convert::From<&PostgresqlTypeOrJsonType> for ShouldDeriveSchemarsJsonSchema {
    fn from(value: &PostgresqlTypeOrJsonType) -> Self {
        match &value {
            PostgresqlTypeOrJsonType::PostgresqlType => Self::False,
            PostgresqlTypeOrJsonType::PostgresqlJsonType => Self::True,
        }
    }
}
impl quote::ToTokens for ShouldDeriveSchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {, schemars::JsonSchema}.to_tokens(tokens),
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
        }
    }
}

pub enum PostgresqlTypeOrJsonType {
    PostgresqlType,
    PostgresqlJsonType,
}
impl std::fmt::Display for PostgresqlTypeOrJsonType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::PostgresqlType => write!(formatter, "{}", naming::PostgresqlTypeUpperCamelCase),
            Self::PostgresqlJsonType => write!(formatter, "{}", naming::PostgresqlJsonTypeUpperCamelCase),
        }
    }
}

pub fn generate_postgresql_json_type_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    ident: &dyn quote::ToTokens,
    create_type_token_stream: &dyn quote::ToTokens,
    create_query_part_token_stream: &dyn quote::ToTokens,
    create_query_bind_token_stream: &dyn quote::ToTokens,
    select_type_token_stream: &dyn quote::ToTokens,
    read_type_token_stream: &dyn quote::ToTokens,
    select_query_part_token_stream: &dyn quote::ToTokens,
    where_element_type_token_stream: &dyn quote::ToTokens,
    update_type_token_stream: &dyn quote::ToTokens,
    update_query_part_token_stream: &dyn quote::ToTokens,
    update_query_bind_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let create_upper_camel_case = naming::CreateUpperCamelCase;
    let value_snake_case = naming::ValueSnakeCase;
    let select_upper_camel_case = naming::SelectUpperCamelCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    let where_element_upper_camel_case = naming::WhereElementUpperCamelCase;
    let update_upper_camel_case = naming::UpdateUpperCamelCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
    let query_snake_case = naming::QuerySnakeCase;
    let field_ident_snake_case = naming::FieldIdentSnakeCase;
    let column_name_and_maybe_field_getter_snake_case = naming::ColumnNameAndMaybeFieldGetterSnakeCase;
    let column_name_and_maybe_field_getter_for_error_message_snake_case = naming::ColumnNameAndMaybeFieldGetterForErrorMessageSnakeCase;
    let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
    let jsonb_set_target_snake_case = naming::JsonbSetTargetSnakeCase;
    let jsonb_set_path_snake_case = naming::JsonbSetPathSnakeCase;
    let create_query_part_snake_case = naming::CreateQueryPartSnakeCase;
    let create_query_bind_snake_case = naming::CreateQueryBindSnakeCase;
    let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
    let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
    let reference_std_primitive_str_token_stream = quote::quote! {&std::primitive::str};
    let reference_mut_std_primitive_u64_token_stream = quote::quote! {&mut std::primitive::u64};
    let query_postgres_arguments_token_stream = quote::quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let mut_query_sqlx_query_postgres_arguments_token_stream = quote::quote! {mut #query_snake_case: #query_postgres_arguments_token_stream};
    let std_string_string_token_stream = token_patterns::StdStringString;
    //todo maybe reexport sqlx?
    quote::quote! {
        impl #path_token_stream #postgresql_json_type_upper_camel_case for #ident {
            type #create_upper_camel_case = #create_type_token_stream;
            fn #create_query_part_snake_case(
                #value_snake_case: &Self::#create_upper_camel_case,
                #increment_snake_case: #reference_mut_std_primitive_u64_token_stream
            ) -> Result<#std_string_string_token_stream, #path_token_stream QueryPartErrorNamed> {
                #create_query_part_token_stream
            }
            fn #create_query_bind_snake_case(
                #value_snake_case: Self::#create_upper_camel_case,
                #mut_query_sqlx_query_postgres_arguments_token_stream
            ) -> #query_postgres_arguments_token_stream {
                #create_query_bind_token_stream
            }
            type #select_upper_camel_case = #select_type_token_stream;
            fn #select_query_part_snake_case(
                #value_snake_case: &Self::#select_upper_camel_case,
                #field_ident_snake_case: #reference_std_primitive_str_token_stream,
                #column_name_and_maybe_field_getter_snake_case: #reference_std_primitive_str_token_stream,
                #column_name_and_maybe_field_getter_for_error_message_snake_case: #reference_std_primitive_str_token_stream,
                is_postgresql_type: std::primitive::bool,
            ) -> #std_string_string_token_stream {
                #select_query_part_token_stream
            }
            type #where_element_upper_camel_case = #where_element_type_token_stream;
            type #read_upper_camel_case = #read_type_token_stream;
            type #update_upper_camel_case = #update_type_token_stream;
            fn #update_query_part_snake_case(
                #value_snake_case: &Self::#update_upper_camel_case,
                #jsonb_set_accumulator_snake_case: #reference_std_primitive_str_token_stream,
                #jsonb_set_target_snake_case: #reference_std_primitive_str_token_stream,
                #jsonb_set_path_snake_case: #reference_std_primitive_str_token_stream,
                #increment_snake_case: #reference_mut_std_primitive_u64_token_stream,
            ) -> Result<#std_string_string_token_stream, #path_token_stream QueryPartErrorNamed> {
                #update_query_part_token_stream
            }
            fn #update_query_bind_snake_case(
                #value_snake_case: Self::#update_upper_camel_case,
                #mut_query_sqlx_query_postgres_arguments_token_stream
            ) -> #query_postgres_arguments_token_stream {
                #update_query_bind_token_stream
            }
        }
    }
}

pub enum PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement {
    Crate,
    PostgresqlCrud,
}
impl PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::Crate => &token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
            PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::PostgresqlCrud => &token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
        }
    }
    fn all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::Crate => &token_patterns::CrateAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
            PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::PostgresqlCrud => &token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
        }
    }
}
pub fn generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    path_default_but_option_is_always_some_and_vec_always_contains_one_element: &PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement,
    ident: &dyn quote::ToTokens,
    ident_generic_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let path_trait_token_stream = path_default_but_option_is_always_some_and_vec_always_contains_one_element.default_but_option_is_always_some_and_vec_always_contains_one_element();
    let default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementSnakeCase;
    quote::quote! {
        impl #impl_generic_token_stream #path_trait_token_stream for #ident #ident_generic_token_stream {
            fn #default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case() -> Self {
                #content_token_stream
            }
        }
    }
}
pub fn generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
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

pub fn generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
        &proc_macro2::TokenStream::new(),
        &PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::Crate,
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
        &proc_macro2::TokenStream::new(),
        &PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::PostgresqlCrud,
        ident,
        lifetime_token_stream,
        content_token_stream
    )
}
pub fn generate_impl_crate_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::Crate, ident, content_token_stream)
}
pub fn generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    generate_impl_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&PathDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::PostgresqlCrud, ident, content_token_stream)
}

pub enum ImportPath {
    Crate,
    PostgresqlCrud,
}
impl quote::ToTokens for ImportPath {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::Crate => quote::quote!{crate},
            Self::PostgresqlCrud => quote::quote!{postgresql_crud},
        }.to_tokens(tokens)
    }
}
pub fn impl_postgresql_type_where_filter_for_ident_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    ident_token_stream: &dyn quote::ToTokens,
    ident_generic_token_stream: &dyn quote::ToTokens,
    query_part_content_token_stream: &dyn quote::ToTokens,
    query_bind_content_token_stream: &dyn quote::ToTokens,
    import_path: &ImportPath,
) -> proc_macro2::TokenStream {
    let increment_snake_case = naming::IncrementSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let std_primitive_u64_token_stream = token_patterns::StdPrimitiveU64;
    let std_fmt_display_token_stream = token_patterns::StdFmtDisplay;
    let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;
    let std_string_string_token_stream = token_patterns::StdStringString;
    let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
    let query_part_snake_case = naming::QueryPartSnakeCase;
    let query_bind_snake_case = naming::QueryBindSnakeCase;
    let postgresql_type_where_filter_upper_camel_case = naming::PostgresqlTypeWhereFilterUpperCamelCase;
    quote::quote!{
        impl #impl_generic_token_stream #import_path ::#postgresql_type_where_filter_upper_camel_case<'a> for #ident_token_stream #ident_generic_token_stream {
            fn #query_part_snake_case(
                &self,
                #increment_snake_case: &mut #std_primitive_u64_token_stream,
                #column_snake_case: &dyn #std_fmt_display_token_stream,
                #is_need_to_add_logical_operator_snake_case: #std_primitive_bool_token_stream
            ) -> Result<#std_string_string_token_stream, #import_path::#query_part_error_named_upper_camel_case> {
                #query_part_content_token_stream
            }
            fn #query_bind_snake_case(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #query_bind_content_token_stream
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
pub fn generate_impl_sqlx_decode_sqlx_postgres_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    type_token_stream: &dyn quote::ToTokens,
    ok_value_match_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
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

pub fn generate_impl_postgresql_type_for_ident_token_stream(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    ident_table_type_declaration_upper_camel_case: &dyn quote::ToTokens,
    ident_create_upper_camel_case: &dyn quote::ToTokens,
    create_query_part_content_token_stream: &dyn quote::ToTokens,
    create_query_bind_content_token_stream: &dyn quote::ToTokens,
    ident_select_upper_camel_case: &dyn quote::ToTokens,
    select_query_part_content_token_stream: &dyn quote::ToTokens,
    ident_where_element_upper_camel_case: &dyn quote::ToTokens,
    ident_read_upper_camel_case: &dyn quote::ToTokens,
    ident_update_upper_camel_case: &dyn quote::ToTokens,
    update_query_part_content_token_stream: &dyn quote::ToTokens,
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
                #value_snake_case: &Self::#create_upper_camel_case,
                #increment_snake_case: &mut std::primitive::u64
            ) -> Result<#std_string_string_token_stream, #import_path ::QueryPartErrorNamed> {
                #create_query_part_content_token_stream
            }
            fn #create_query_bind_snake_case(
                #value_snake_case: Self::#create_upper_camel_case,
                mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
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
                mut #query_snake_case: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #update_query_bind_content_token_stream
            }
        }
    }
}

pub fn generate_create_table_column_query_part_token_stream(
    ident: &dyn quote::ToTokens,
    maybe_fixed_length_parameter_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let create_table_column_query_part_snake_case = naming::CreateTableColumnQueryPartSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let is_primary_key_snake_case = naming::IsPrimaryKeySnakeCase;
    let std_fmt_display_token_stream = token_patterns::StdFmtDisplay;
    let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;
    quote::quote! {
        impl #ident {
            pub fn #create_table_column_query_part_snake_case(
                #column_snake_case: &dyn #std_fmt_display_token_stream,
                #is_primary_key_snake_case: #std_primitive_bool_token_stream #maybe_fixed_length_parameter_token_stream
            ) -> impl #std_fmt_display_token_stream {
                #content_token_stream
            }
        }
    }
}

pub fn crate_query_part_error_named_checked_add_initialization_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }}
}

pub fn generate_impl_crate_is_empty_for_ident_token_stream(ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {
        impl crate::IsEmpty for #ident {
            fn is_empty(&self) -> std::primitive::bool {
                self.0.to_string().is_empty()
            }
        }
    }
}