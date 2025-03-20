mod filters;

pub use filters::*;

pub fn generate_postgresql_type_where_element_token_stream(
    variants: &std::vec::Vec<&dyn crate::WhereOperatorName>,
    ident: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    variant_type_prefix_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    should_implement_schemars_json_schema: &crate::ShouldDeriveSchemarsJsonSchema,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let postgresql_type_tokens_where_element_token_stream = {
        let variants_token_stream = variants.iter().map(|element| {
            let element_upper_camel_case = element.upper_camel_case();
            let postgresql_type_tokens_where_element_filter_upper_camel_case = {
                let value = format!("{variant_type_prefix_upper_camel_case}{}", quote::quote! {#element_upper_camel_case});
                value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote! {#element_upper_camel_case(#postgresql_type_tokens_where_element_filter_upper_camel_case)}
        });
        quote::quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_implement_schemars_json_schema)]
            pub enum #ident {
                #(#variants_token_stream),*
            }
        }
    };
    let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_tokens_where_element_token_stream = crate::impl_postgresql_type_self_where_filter_for_ident_token_stream(
        &ident,
        &{
            let variants_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.upper_camel_case();
                quote::quote! {
                    Self::#element_upper_camel_case(#value_snake_case) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(
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
                    Self::#element_upper_camel_case(#value_snake_case) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(
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
        &crate::PostgresqlTypeSelfWhereFilterPath::Crate,
    );
    let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_element_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&ident, &quote::quote! {format!("{self:#?}")});
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
        #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_tokens_where_element_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_element_token_stream
        #impl_crate_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_tokens_where_element_token_stream
    }
}
//todo its just a copy of generate_postgresql_type_where_element_token_stream for the time for refactoring
pub fn generate_postgresql_type_where_element_refactoring_token_stream(
    variants: &std::vec::Vec<&dyn crate::WhereOperatorName>,
    postgresql_type_not_null_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    ident: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    variant_type_prefix_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    should_implement_schemars_json_schema: &crate::ShouldDeriveSchemarsJsonSchema,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let postgresql_type_tokens_where_element_token_stream = {
        let variants_token_stream = variants.iter().map(|element| {
            let element_upper_camel_case = element.upper_camel_case();
            //todo temp if - need to remove it later
            let type_token_stream = if 
            "Equal" == &element_upper_camel_case.to_string() ||
            "GreaterThan" == &element_upper_camel_case.to_string() ||
            "Between" == &element_upper_camel_case.to_string()
            
            {
                quote::quote! {crate::where_element_filters::PostgresqlTypeWhereElementBetween<#postgresql_type_not_null_upper_camel_case>}
            }
            else {
                let value = format!("{variant_type_prefix_upper_camel_case}{}", quote::quote! {#element_upper_camel_case});
                value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
    let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_tokens_where_element_token_stream = crate::impl_postgresql_type_self_where_filter_for_ident_token_stream(
        &ident,
        &{
            let variants_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.upper_camel_case();
                quote::quote! {
                    Self::#element_upper_camel_case(#value_snake_case) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(
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
                    Self::#element_upper_camel_case(#value_snake_case) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(
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
        &crate::PostgresqlTypeSelfWhereFilterPath::Crate,
    );
    let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_element_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&ident, &quote::quote! {format!("{self:#?}")});
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
        #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_tokens_where_element_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_element_token_stream
        #impl_crate_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_type_tokens_where_element_token_stream
    }
}

pub fn crate_try_generate_bind_increments_error_named_token_stream() -> proc_macro2::TokenStream {
    let try_generate_bind_increments_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
    quote::quote! {crate::#try_generate_bind_increments_error_named_upper_camel_case}
}
pub fn crate_bind_query_token_stream() -> proc_macro2::TokenStream {
    quote::quote! {crate::BindQuery::}
}
pub fn generate_impl_crate_bind_query_for_tokens_token_stream(ident_token_stream: &dyn quote::ToTokens, try_generate_bind_increments_token_stream: &dyn quote::ToTokens, bind_value_to_query_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let std_string_string_token_stream = token_patterns::StdStringString;
    let self_snake_case = naming::SelfSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let try_generate_bind_increments_snake_case = naming::TryGenerateBindIncrementsSnakeCase;
    let bind_value_to_query_snake_case = naming::BindValueToQuerySnakeCase;
    let crate_try_generate_bind_increments_error_named_token_stream = crate_try_generate_bind_increments_error_named_token_stream();
    quote::quote! {
        impl crate::BindQuery for #ident_token_stream {
            fn #try_generate_bind_increments_snake_case(&#self_snake_case, #increment_snake_case: &mut std::primitive::u64) -> Result<#std_string_string_token_stream, #crate_try_generate_bind_increments_error_named_token_stream> {
                #try_generate_bind_increments_token_stream
            }
            fn #bind_value_to_query_snake_case(#self_snake_case, mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #bind_value_to_query_token_stream
            }
        }
    }
}

pub fn generate_struct_postgresql_type_where_element_tokens_double_quotes_token_stream(postgresql_type_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_token_stream(&format!("struct {postgresql_type_where_element_tokens_upper_camel_case}"))
}
pub fn generate_struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens, length: std::primitive::usize) -> proc_macro2::TokenStream {
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
    pub fn try_to_vec_element_type(&self) -> Result<Self, ()> {
        match &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
            PostgresqlJsonTypePatternType::FullTypePath => Err(()),
            //todo maybe wrong
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath | PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => Ok(Self {
                postgresql_json_type_handle: self.postgresql_json_type_handle.clone(),
                postgresql_json_type_pattern: self.postgresql_json_type_pattern.clone(),
            }),
        }
    }
    pub fn postgresql_json_type_ident_wrapper(&self) -> proc_macro2::TokenStream {
        format!("{}{}", self.postgresql_json_type_pattern.prefix_stringified(), self.postgresql_json_type_handle,).parse::<proc_macro2::TokenStream>().unwrap()
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
    let struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream = generate_struct_postgresql_type_where_element_tokens_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case);
    let struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream = generate_struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case, length);
    let postgresql_type_ident_where_element_tokens_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&postgresql_type_ident_where_element_tokens_upper_camel_case);
    (
        struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream,
        struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream,
        postgresql_type_ident_where_element_tokens_double_quotes_token_stream,
    )
}

pub enum WhereOperatorType<'a> {
    Ident(&'a dyn quote::ToTokens),
    FieldType { field_type: &'a dyn quote::ToTokens, default_initialization_token_stream: &'a dyn quote::ToTokens },
}
impl WhereOperatorType<'_> {
    fn type_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(value) => quote::quote! {#value},
            WhereOperatorType::FieldType { field_type, .. } => quote::quote! {#field_type},
        }
    }
    // fn std_option_option_type_token_stream(&self) -> proc_macro2::TokenStream {
    //     match &self {
    //         WhereOperatorType::Ident(value) => quote::quote! {std::option::Option<#value>},
    //         WhereOperatorType::FieldType { field_type, .. } => quote::quote! {std::option::Option<#field_type>},
    //     }
    // }
    fn additional_bind_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(_) => quote::quote! {.0},
            WhereOperatorType::FieldType { .. } => proc_macro2::TokenStream::new(),
        }
    }
    fn default_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(_) => {
                let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream =
                    token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
                quote::quote! {#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream}
            }
            WhereOperatorType::FieldType { field_type: _, default_initialization_token_stream } => quote::quote! {#default_initialization_token_stream},
        }
    }
    // fn std_option_option_default_initialization_token_stream(&self) -> proc_macro2::TokenStream {
    //     match &self {
    //         WhereOperatorType::Ident(_) => {
    //             let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream =
    //                 token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
    //             quote::quote! {Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)}
    //         }
    //         WhereOperatorType::FieldType { field_type: _, default_initialization_token_stream } => quote::quote! {Some(#default_initialization_token_stream)},
    //     }
    // }
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
        impl #postgresql_type_self_where_filter_path ::postgresql_type_trait::#postgresql_type_self_where_filter_upper_camel_case<'_> for #ident_token_stream {
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