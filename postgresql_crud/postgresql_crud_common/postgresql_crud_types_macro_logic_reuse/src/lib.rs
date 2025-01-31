mod filters;

#[derive(Debug, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
enum PostgresqlJsonTypeHandle {
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
}
impl PostgresqlJsonTypeHandle {
    fn field_type(&self) -> proc_macro2::TokenStream {
        match &self {
            Self::StdPrimitiveI8 => quote::quote!{std::primitive::i8},
            Self::StdPrimitiveI16 => quote::quote!{std::primitive::i16},
            Self::StdPrimitiveI32 => quote::quote!{std::primitive::i32},
            Self::StdPrimitiveI64 => quote::quote!{std::primitive::i64},
            Self::StdPrimitiveU8 => quote::quote!{std::primitive::u8},
            Self::StdPrimitiveU16 => quote::quote!{std::primitive::u16},
            Self::StdPrimitiveU32 => quote::quote!{std::primitive::u32},
            Self::StdPrimitiveU64 => quote::quote!{std::primitive::u64},
            Self::StdPrimitiveF32 => quote::quote!{std::primitive::f32},
            Self::StdPrimitiveF64 => quote::quote!{std::primitive::f64},
            Self::StdPrimitiveBool => quote::quote!{std::primitive::bool},
            Self::StdStringString => quote::quote!{std::string::String},
        }
    }
}
impl quote::ToTokens for PostgresqlJsonTypeHandle {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.to_string()
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonTypeHandle to proc_macro2::TokenStream"))
        .to_tokens(tokens)
    }
}

enum PostgresqlJsonTypePattern {
    FullTypePath,
    StdOptionOptionFullTypePath,
    StdVecVecFullTypePath,
    StdOptionOptionStdVecVecFullTypePath,
    StdVecVecStdOptionOptionFullTypePath,
    StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
}
impl PostgresqlJsonTypePattern {
    // fn field_type(&self, postgresql_json_type_handle: &PostgresqlJsonTypeHandle) -> proc_macro2::TokenStream {
    //     let field_type = postgresql_json_type_handle.field_type();
    //     match &self {
    //         Self::FullTypePath => field_type,
    //         Self::StdOptionOptionFullTypePath => quote::quote!{std::option::Option<#field_type>},
    //         Self::StdVecVecFullTypePath => quote::quote!{std::vec::Vec<#field_type>},
    //         Self::StdOptionOptionStdVecVecFullTypePath => quote::quote!{std::option::Option<std::vec::Vec<#field_type>>},
    //         Self::StdVecVecStdOptionOptionFullTypePath => quote::quote!{std::vec::Vec<std::option::Option<#field_type>>},
    //         Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<#field_type>>>},
    //     }
    // }
    fn field_type(&self, postgresql_json_type_handle: &PostgresqlJsonTypeHandle) -> proc_macro2::TokenStream {
        let field_type = postgresql_json_type_handle.field_type();
        match &self {
            Self::FullTypePath => field_type,
            Self::StdOptionOptionFullTypePath => quote::quote!{std::option::Option<#postgresql_json_type_handle>},
            Self::StdVecVecFullTypePath => quote::quote!{std::vec::Vec<#postgresql_json_type_handle>},
            Self::StdOptionOptionStdVecVecFullTypePath => quote::quote!{std::option::Option<std::vec::Vec<#postgresql_json_type_handle>>},
            Self::StdVecVecStdOptionOptionFullTypePath => quote::quote!{std::vec::Vec<std::option::Option<#postgresql_json_type_handle>>},
            Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<#postgresql_json_type_handle>>>},
        }
    }
    fn non_optional_field_type(&self, postgresql_json_type_handle: &PostgresqlJsonTypeHandle) -> proc_macro2::TokenStream {
        let field_type = postgresql_json_type_handle.field_type();
        match &self {
            Self::FullTypePath => field_type,
            Self::StdOptionOptionFullTypePath => quote::quote!{#postgresql_json_type_handle},
            Self::StdVecVecFullTypePath => quote::quote!{std::vec::Vec<#postgresql_json_type_handle>},
            Self::StdOptionOptionStdVecVecFullTypePath => quote::quote!{std::vec::Vec<#postgresql_json_type_handle>},
            Self::StdVecVecStdOptionOptionFullTypePath => quote::quote!{std::vec::Vec<#postgresql_json_type_handle>},
            Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote!{std::vec::Vec<#postgresql_json_type_handle>},
        }
    }
    fn initialization_token_stream(&self) -> proc_macro2::TokenStream {
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
            let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
            quote::quote! {
                crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
            }
        };
        match &self {
            Self::FullTypePath => {
                let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
                quote::quote!{#core_default_default_default_token_stream}
            },
            Self::StdOptionOptionFullTypePath => quote::quote!{Some(#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream)},
            Self::StdVecVecFullTypePath => quote::quote!{vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream]},
            Self::StdOptionOptionStdVecVecFullTypePath => quote::quote!{Some(vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream])},
            Self::StdVecVecStdOptionOptionFullTypePath => quote::quote!{vec![Some(#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream)]},
            Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote!{Some(vec![Some(#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream)])},
        }
    }
    fn non_optional_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
            let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
            quote::quote! {
                crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
            }
        };
        match &self {
            Self::FullTypePath => {
                let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
                quote::quote!{#core_default_default_default_token_stream}
            },
            Self::StdOptionOptionFullTypePath => quote::quote!{#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream},
            Self::StdVecVecFullTypePath => quote::quote!{vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream]},
            Self::StdOptionOptionStdVecVecFullTypePath => quote::quote!{vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream]},
            Self::StdVecVecStdOptionOptionFullTypePath => quote::quote!{vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream]},
            Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote!{vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream]},
        }
    }
}
impl std::convert::From<&PostgresqlJsonTypePatternSpecific> for PostgresqlJsonTypePattern {
    fn from(value: &PostgresqlJsonTypePatternSpecific) -> Self {
        match value {
            PostgresqlJsonTypePatternSpecific::FullTypePathNumber => Self::FullTypePath,
            PostgresqlJsonTypePatternSpecific::FullTypePathBool => Self::FullTypePath,
            PostgresqlJsonTypePatternSpecific::FullTypePathString => Self::FullTypePath,
            PostgresqlJsonTypePatternSpecific::StdOptionOptionFullTypePathNumber => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdOptionOptionFullTypePathBool => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdOptionOptionFullTypePathString => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdVecVecFullTypePathNumber => Self::StdVecVecFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdVecVecFullTypePathBool => Self::StdVecVecFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdVecVecFullTypePathString => Self::StdVecVecFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecFullTypePathNumber => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecFullTypePathBool => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecFullTypePathString => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdVecVecStdOptionOptionFullTypePathNumber => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdVecVecStdOptionOptionFullTypePathBool => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdVecVecStdOptionOptionFullTypePathString => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecStdOptionOptionFullTypePathBool => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecStdOptionOptionFullTypePathString => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
        }
    }
}
impl std::convert::From<&PostgresqlJsonType> for PostgresqlJsonTypePattern {
    fn from(value: &PostgresqlJsonType) -> Self {
        match value {
            PostgresqlJsonType::StdPrimitiveI8 => Self::FullTypePath,
            PostgresqlJsonType::StdPrimitiveI16 => Self::FullTypePath,
            PostgresqlJsonType::StdPrimitiveI32 => Self::FullTypePath,
            PostgresqlJsonType::StdPrimitiveI64 => Self::FullTypePath,
            PostgresqlJsonType::StdPrimitiveU8 => Self::FullTypePath,
            PostgresqlJsonType::StdPrimitiveU16 => Self::FullTypePath,
            PostgresqlJsonType::StdPrimitiveU32 => Self::FullTypePath,
            PostgresqlJsonType::StdPrimitiveU64 => Self::FullTypePath,
            PostgresqlJsonType::StdPrimitiveF32 => Self::FullTypePath,
            PostgresqlJsonType::StdPrimitiveF64 => Self::FullTypePath,
            PostgresqlJsonType::StdPrimitiveBool => Self::FullTypePath,
            PostgresqlJsonType::StdStringString => Self::FullTypePath,

            PostgresqlJsonType::StdOptionOptionStdPrimitiveI8 => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveI16 => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveI32 => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveI64 => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU8 => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU16 => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU32 => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU64 => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveF32 => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveF64 => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveBool => Self::StdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdStringString => Self::StdOptionOptionFullTypePath,

            PostgresqlJsonType::StdVecVecStdPrimitiveI8 => Self::StdVecVecFullTypePath,
            PostgresqlJsonType::StdVecVecStdPrimitiveI16 => Self::StdVecVecFullTypePath,
            PostgresqlJsonType::StdVecVecStdPrimitiveI32 => Self::StdVecVecFullTypePath,
            PostgresqlJsonType::StdVecVecStdPrimitiveI64 => Self::StdVecVecFullTypePath,
            PostgresqlJsonType::StdVecVecStdPrimitiveU8 => Self::StdVecVecFullTypePath,
            PostgresqlJsonType::StdVecVecStdPrimitiveU16 => Self::StdVecVecFullTypePath,
            PostgresqlJsonType::StdVecVecStdPrimitiveU32 => Self::StdVecVecFullTypePath,
            PostgresqlJsonType::StdVecVecStdPrimitiveU64 => Self::StdVecVecFullTypePath,
            PostgresqlJsonType::StdVecVecStdPrimitiveF32 => Self::StdVecVecFullTypePath,
            PostgresqlJsonType::StdVecVecStdPrimitiveF64 => Self::StdVecVecFullTypePath,
            PostgresqlJsonType::StdVecVecStdPrimitiveBool => Self::StdVecVecFullTypePath,
            PostgresqlJsonType::StdVecVecStdStringString => Self::StdVecVecFullTypePath,

            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI8 => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI16 => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI32 => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI64 => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU8 => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU16 => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU32 => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU64 => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveF32 => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveF64 => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveBool => Self::StdOptionOptionStdVecVecFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdStringString => Self::StdOptionOptionStdVecVecFullTypePath,

            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI8 => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI16 => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI32 => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI64 => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU8 => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU16 => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU32 => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU64 => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveF32 => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveF64 => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveBool => Self::StdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdStringString => Self::StdVecVecStdOptionOptionFullTypePath,

            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdStringString => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
        }
    }
}
enum PostgresqlJsonTypePatternSpecific {
    FullTypePathNumber,
    FullTypePathBool,
    FullTypePathString,
    StdOptionOptionFullTypePathNumber,
    StdOptionOptionFullTypePathBool,
    StdOptionOptionFullTypePathString,
    StdVecVecFullTypePathNumber,
    StdVecVecFullTypePathBool,
    StdVecVecFullTypePathString,
    StdOptionOptionStdVecVecFullTypePathNumber,
    StdOptionOptionStdVecVecFullTypePathBool,
    StdOptionOptionStdVecVecFullTypePathString,
    StdVecVecStdOptionOptionFullTypePathNumber,
    StdVecVecStdOptionOptionFullTypePathBool,
    StdVecVecStdOptionOptionFullTypePathString,
    StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber,
    StdOptionOptionStdVecVecStdOptionOptionFullTypePathBool,
    StdOptionOptionStdVecVecStdOptionOptionFullTypePathString,
}
impl std::convert::From<&PostgresqlJsonType> for PostgresqlJsonTypePatternSpecific {
    fn from(value: &PostgresqlJsonType) -> Self {
        match value {
            PostgresqlJsonType::StdPrimitiveI8 => Self::FullTypePathNumber,
            PostgresqlJsonType::StdPrimitiveI16 => Self::FullTypePathNumber,
            PostgresqlJsonType::StdPrimitiveI32 => Self::FullTypePathNumber,
            PostgresqlJsonType::StdPrimitiveI64 => Self::FullTypePathNumber,
            PostgresqlJsonType::StdPrimitiveU8 => Self::FullTypePathNumber,
            PostgresqlJsonType::StdPrimitiveU16 => Self::FullTypePathNumber,
            PostgresqlJsonType::StdPrimitiveU32 => Self::FullTypePathNumber,
            PostgresqlJsonType::StdPrimitiveU64 => Self::FullTypePathNumber,
            PostgresqlJsonType::StdPrimitiveF32 => Self::FullTypePathNumber,
            PostgresqlJsonType::StdPrimitiveF64 => Self::FullTypePathNumber,
            PostgresqlJsonType::StdPrimitiveBool => Self::FullTypePathBool,
            PostgresqlJsonType::StdStringString => Self::FullTypePathString,

            PostgresqlJsonType::StdOptionOptionStdPrimitiveI8 => Self::StdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveI16 => Self::StdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveI32 => Self::StdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveI64 => Self::StdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU8 => Self::StdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU16 => Self::StdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU32 => Self::StdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU64 => Self::StdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveF32 => Self::StdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveF64 => Self::StdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdPrimitiveBool => Self::StdOptionOptionFullTypePathBool,
            PostgresqlJsonType::StdOptionOptionStdStringString => Self::StdOptionOptionFullTypePathString,

            PostgresqlJsonType::StdVecVecStdPrimitiveI8 => Self::StdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdPrimitiveI16 => Self::StdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdPrimitiveI32 => Self::StdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdPrimitiveI64 => Self::StdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdPrimitiveU8 => Self::StdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdPrimitiveU16 => Self::StdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdPrimitiveU32 => Self::StdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdPrimitiveU64 => Self::StdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdPrimitiveF32 => Self::StdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdPrimitiveF64 => Self::StdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdPrimitiveBool => Self::StdVecVecFullTypePathBool,
            PostgresqlJsonType::StdVecVecStdStringString => Self::StdVecVecFullTypePathString,

            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI8 => Self::StdOptionOptionStdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI16 => Self::StdOptionOptionStdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI32 => Self::StdOptionOptionStdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI64 => Self::StdOptionOptionStdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU8 => Self::StdOptionOptionStdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU16 => Self::StdOptionOptionStdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU32 => Self::StdOptionOptionStdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU64 => Self::StdOptionOptionStdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveF32 => Self::StdOptionOptionStdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveF64 => Self::StdOptionOptionStdVecVecFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveBool => Self::StdOptionOptionStdVecVecFullTypePathBool,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdStringString => Self::StdOptionOptionStdVecVecFullTypePathString,

            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI8 => Self::StdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI16 => Self::StdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI32 => Self::StdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI64 => Self::StdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU8 => Self::StdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU16 => Self::StdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU32 => Self::StdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU64 => Self::StdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveF32 => Self::StdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveF64 => Self::StdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveBool => Self::StdVecVecStdOptionOptionFullTypePathBool,
            PostgresqlJsonType::StdVecVecStdOptionOptionStdStringString => Self::StdVecVecStdOptionOptionFullTypePathString,

            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathBool,
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdStringString => Self::StdOptionOptionStdVecVecStdOptionOptionFullTypePathString,
        }
    }
}

#[derive(Debug, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
enum PostgresqlJsonType {
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

    StdOptionOptionStdPrimitiveI8,
    StdOptionOptionStdPrimitiveI16,
    StdOptionOptionStdPrimitiveI32,
    StdOptionOptionStdPrimitiveI64,
    StdOptionOptionStdPrimitiveU8,
    StdOptionOptionStdPrimitiveU16,
    StdOptionOptionStdPrimitiveU32,
    StdOptionOptionStdPrimitiveU64,
    StdOptionOptionStdPrimitiveF32,
    StdOptionOptionStdPrimitiveF64,
    StdOptionOptionStdPrimitiveBool,
    StdOptionOptionStdStringString,

    StdVecVecStdPrimitiveI8,
    StdVecVecStdPrimitiveI16,
    StdVecVecStdPrimitiveI32,
    StdVecVecStdPrimitiveI64,
    StdVecVecStdPrimitiveU8,
    StdVecVecStdPrimitiveU16,
    StdVecVecStdPrimitiveU32,
    StdVecVecStdPrimitiveU64,
    StdVecVecStdPrimitiveF32,
    StdVecVecStdPrimitiveF64,
    StdVecVecStdPrimitiveBool,
    StdVecVecStdStringString,

    StdOptionOptionStdVecVecStdPrimitiveI8,
    StdOptionOptionStdVecVecStdPrimitiveI16,
    StdOptionOptionStdVecVecStdPrimitiveI32,
    StdOptionOptionStdVecVecStdPrimitiveI64,
    StdOptionOptionStdVecVecStdPrimitiveU8,
    StdOptionOptionStdVecVecStdPrimitiveU16,
    StdOptionOptionStdVecVecStdPrimitiveU32,
    StdOptionOptionStdVecVecStdPrimitiveU64,
    StdOptionOptionStdVecVecStdPrimitiveF32,
    StdOptionOptionStdVecVecStdPrimitiveF64,
    StdOptionOptionStdVecVecStdPrimitiveBool,
    StdOptionOptionStdVecVecStdStringString,

    StdVecVecStdOptionOptionStdPrimitiveI8,
    StdVecVecStdOptionOptionStdPrimitiveI16,
    StdVecVecStdOptionOptionStdPrimitiveI32,
    StdVecVecStdOptionOptionStdPrimitiveI64,
    StdVecVecStdOptionOptionStdPrimitiveU8,
    StdVecVecStdOptionOptionStdPrimitiveU16,
    StdVecVecStdOptionOptionStdPrimitiveU32,
    StdVecVecStdOptionOptionStdPrimitiveU64,
    StdVecVecStdOptionOptionStdPrimitiveF32,
    StdVecVecStdOptionOptionStdPrimitiveF64,
    StdVecVecStdOptionOptionStdPrimitiveBool,
    StdVecVecStdOptionOptionStdStringString,

    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
    StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
    StdOptionOptionStdVecVecStdOptionOptionStdStringString,
}
impl PostgresqlJsonType {
    fn to_postgresql_json_type_handle_and_postgresql_json_type_pattern(&self) -> (PostgresqlJsonTypeHandle, PostgresqlJsonTypePattern) {
        match &self {
            Self::StdPrimitiveI8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI8,
                PostgresqlJsonTypePattern::FullTypePath,
            ),
            Self::StdPrimitiveI16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI16,
                PostgresqlJsonTypePattern::FullTypePath,
            ),
            Self::StdPrimitiveI32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI32,
                PostgresqlJsonTypePattern::FullTypePath,
            ),
            Self::StdPrimitiveI64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI64,
                PostgresqlJsonTypePattern::FullTypePath,
            ),
            Self::StdPrimitiveU8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU8,
                PostgresqlJsonTypePattern::FullTypePath,
            ),
            Self::StdPrimitiveU16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU16,
                PostgresqlJsonTypePattern::FullTypePath,
            ),
            Self::StdPrimitiveU32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU32,
                PostgresqlJsonTypePattern::FullTypePath,
            ),
            Self::StdPrimitiveU64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU64,
                PostgresqlJsonTypePattern::FullTypePath,
            ),
            Self::StdPrimitiveF32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF32,
                PostgresqlJsonTypePattern::FullTypePath,
            ),
            Self::StdPrimitiveF64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF64,
                PostgresqlJsonTypePattern::FullTypePath,
            ),
            Self::StdPrimitiveBool => (
                PostgresqlJsonTypeHandle::StdPrimitiveBool,
                PostgresqlJsonTypePattern::FullTypePath,
            ),
            Self::StdStringString => (
                PostgresqlJsonTypeHandle::StdStringString,
                PostgresqlJsonTypePattern::FullTypePath,
            ),

            Self::StdOptionOptionStdPrimitiveI8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI8,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdPrimitiveI16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI16,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdPrimitiveI32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI32,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdPrimitiveI64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI64,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdPrimitiveU8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU8,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdPrimitiveU16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU16,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdPrimitiveU32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU32,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdPrimitiveU64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU64,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdPrimitiveF32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF32,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdPrimitiveF64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF64,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdPrimitiveBool => (
                PostgresqlJsonTypeHandle::StdPrimitiveBool,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdStringString => (
                PostgresqlJsonTypeHandle::StdStringString,
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath,
            ),

            Self::StdVecVecStdPrimitiveI8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI8,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),
            Self::StdVecVecStdPrimitiveI16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI16,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),
            Self::StdVecVecStdPrimitiveI32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI32,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),
            Self::StdVecVecStdPrimitiveI64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI64,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),
            Self::StdVecVecStdPrimitiveU8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU8,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),
            Self::StdVecVecStdPrimitiveU16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU16,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),
            Self::StdVecVecStdPrimitiveU32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU32,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),
            Self::StdVecVecStdPrimitiveU64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU64,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),
            Self::StdVecVecStdPrimitiveF32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF32,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),
            Self::StdVecVecStdPrimitiveF64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF64,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),
            Self::StdVecVecStdPrimitiveBool => (
                PostgresqlJsonTypeHandle::StdPrimitiveBool,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),
            Self::StdVecVecStdStringString => (
                PostgresqlJsonTypeHandle::StdStringString,
                PostgresqlJsonTypePattern::StdVecVecFullTypePath,
            ),

            Self::StdOptionOptionStdVecVecStdPrimitiveI8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI8,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdPrimitiveI16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI16,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdPrimitiveI32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI32,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdPrimitiveI64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI64,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdPrimitiveU8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU8,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdPrimitiveU16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU16,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdPrimitiveU32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU32,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdPrimitiveU64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU64,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdPrimitiveF32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF32,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdPrimitiveF64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF64,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdPrimitiveBool => (
                PostgresqlJsonTypeHandle::StdPrimitiveBool,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdStringString => (
                PostgresqlJsonTypeHandle::StdStringString,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath,
            ),

            Self::StdVecVecStdOptionOptionStdPrimitiveI8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI8,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdVecVecStdOptionOptionStdPrimitiveI16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI16,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdVecVecStdOptionOptionStdPrimitiveI32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI32,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdVecVecStdOptionOptionStdPrimitiveI64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI64,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdVecVecStdOptionOptionStdPrimitiveU8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU8,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdVecVecStdOptionOptionStdPrimitiveU16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU16,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdVecVecStdOptionOptionStdPrimitiveU32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU32,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdVecVecStdOptionOptionStdPrimitiveU64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU64,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdVecVecStdOptionOptionStdPrimitiveF32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF32,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdVecVecStdOptionOptionStdPrimitiveF64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF64,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdVecVecStdOptionOptionStdPrimitiveBool => (
                PostgresqlJsonTypeHandle::StdPrimitiveBool,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdVecVecStdOptionOptionStdStringString => (
                PostgresqlJsonTypeHandle::StdStringString,
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath,
            ),

            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI8,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI16,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI32,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveI64,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU8,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU16,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU32,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveU64,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF32,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => (
                PostgresqlJsonTypeHandle::StdPrimitiveF64,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => (
                PostgresqlJsonTypeHandle::StdPrimitiveBool,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
            Self::StdOptionOptionStdVecVecStdOptionOptionStdStringString => (
                PostgresqlJsonTypeHandle::StdStringString,
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath,
            ),
        }
    }
}
impl quote::ToTokens for PostgresqlJsonType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.to_string()
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonType to proc_macro2::TokenStream"))
        .to_tokens(tokens)
    }
}

#[derive(Debug, strum_macros::Display)]
enum PostgresqlJsonArrayElementType {
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

    StdOptionOptionStdPrimitiveI8,
    StdOptionOptionStdPrimitiveI16,
    StdOptionOptionStdPrimitiveI32,
    StdOptionOptionStdPrimitiveI64,
    StdOptionOptionStdPrimitiveU8,
    StdOptionOptionStdPrimitiveU16,
    StdOptionOptionStdPrimitiveU32,
    StdOptionOptionStdPrimitiveU64,
    StdOptionOptionStdPrimitiveF32,
    StdOptionOptionStdPrimitiveF64,
    StdOptionOptionStdPrimitiveBool,
    StdOptionOptionStdStringString,
}
impl std::convert::TryFrom<&PostgresqlJsonType> for PostgresqlJsonArrayElementType {
    type Error = ();
    fn try_from(value: &PostgresqlJsonType) -> Result<Self, Self::Error> {
        match &value {
            PostgresqlJsonType::StdPrimitiveI8 => Err(()),
            PostgresqlJsonType::StdPrimitiveI16 => Err(()),
            PostgresqlJsonType::StdPrimitiveI32 => Err(()),
            PostgresqlJsonType::StdPrimitiveI64 => Err(()),
            PostgresqlJsonType::StdPrimitiveU8 => Err(()),
            PostgresqlJsonType::StdPrimitiveU16 => Err(()),
            PostgresqlJsonType::StdPrimitiveU32 => Err(()),
            PostgresqlJsonType::StdPrimitiveU64 => Err(()),
            PostgresqlJsonType::StdPrimitiveF32 => Err(()),
            PostgresqlJsonType::StdPrimitiveF64 => Err(()),
            PostgresqlJsonType::StdPrimitiveBool => Err(()),
            PostgresqlJsonType::StdStringString => Err(()),

            PostgresqlJsonType::StdOptionOptionStdPrimitiveI8 => Err(()),
            PostgresqlJsonType::StdOptionOptionStdPrimitiveI16 => Err(()),
            PostgresqlJsonType::StdOptionOptionStdPrimitiveI32 => Err(()),
            PostgresqlJsonType::StdOptionOptionStdPrimitiveI64 => Err(()),
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU8 => Err(()),
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU16 => Err(()),
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU32 => Err(()),
            PostgresqlJsonType::StdOptionOptionStdPrimitiveU64 => Err(()),
            PostgresqlJsonType::StdOptionOptionStdPrimitiveF32 => Err(()),
            PostgresqlJsonType::StdOptionOptionStdPrimitiveF64 => Err(()),
            PostgresqlJsonType::StdOptionOptionStdPrimitiveBool => Err(()),
            PostgresqlJsonType::StdOptionOptionStdStringString => Err(()),

            PostgresqlJsonType::StdVecVecStdPrimitiveI8 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveI8),
            PostgresqlJsonType::StdVecVecStdPrimitiveI16 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveI16),
            PostgresqlJsonType::StdVecVecStdPrimitiveI32 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveI32),
            PostgresqlJsonType::StdVecVecStdPrimitiveI64 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveI64),
            PostgresqlJsonType::StdVecVecStdPrimitiveU8 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveU8),
            PostgresqlJsonType::StdVecVecStdPrimitiveU16 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveU16),
            PostgresqlJsonType::StdVecVecStdPrimitiveU32 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveU32),
            PostgresqlJsonType::StdVecVecStdPrimitiveU64 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveU64),
            PostgresqlJsonType::StdVecVecStdPrimitiveF32 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveF32),
            PostgresqlJsonType::StdVecVecStdPrimitiveF64 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveF64),
            PostgresqlJsonType::StdVecVecStdPrimitiveBool => Ok(PostgresqlJsonArrayElementType::StdPrimitiveBool),
            PostgresqlJsonType::StdVecVecStdStringString => Ok(PostgresqlJsonArrayElementType::StdStringString),

            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI8 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveI8),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI16 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveI16),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI32 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveI32),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveI64 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveI64),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU8 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveU8),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU16 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveU16),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU32 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveU32),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveU64 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveU64),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveF32 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveF32),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveF64 => Ok(PostgresqlJsonArrayElementType::StdPrimitiveF64),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdPrimitiveBool => Ok(PostgresqlJsonArrayElementType::StdPrimitiveBool),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdStringString => Ok(PostgresqlJsonArrayElementType::StdStringString),

            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI8 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI8),
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI16 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI16),
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI32 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI32),
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveI64 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI64),
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU8 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU8),
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU16 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU16),
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU32 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU32),
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveU64 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU64),
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveF32 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveF32),
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveF64 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveF64),
            PostgresqlJsonType::StdVecVecStdOptionOptionStdPrimitiveBool => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveBool),
            PostgresqlJsonType::StdVecVecStdOptionOptionStdStringString => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdStringString),

            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI8),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI16),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI32),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI64),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU8),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU16),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU32),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU64),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveF32),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveF64),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveBool),
            PostgresqlJsonType::StdOptionOptionStdVecVecStdOptionOptionStdStringString => Ok(PostgresqlJsonArrayElementType::StdOptionOptionStdStringString),
        }
    }
}
impl quote::ToTokens for PostgresqlJsonArrayElementType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.to_string()
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonArrayElementType to proc_macro2::TokenStream"))
        .to_tokens(tokens)
    }
}
impl std::convert::From<&PostgresqlJsonArrayElementType> for PostgresqlJsonTypeHandle {
    fn from(value: &PostgresqlJsonArrayElementType) -> Self {
        match &value {
            PostgresqlJsonArrayElementType::StdPrimitiveI8 => Self::StdPrimitiveI8,
            PostgresqlJsonArrayElementType::StdPrimitiveI16 => Self::StdPrimitiveI16,
            PostgresqlJsonArrayElementType::StdPrimitiveI32 => Self::StdPrimitiveI32,
            PostgresqlJsonArrayElementType::StdPrimitiveI64 => Self::StdPrimitiveI64,
            PostgresqlJsonArrayElementType::StdPrimitiveU8 => Self::StdPrimitiveU8,
            PostgresqlJsonArrayElementType::StdPrimitiveU16 => Self::StdPrimitiveU16,
            PostgresqlJsonArrayElementType::StdPrimitiveU32 => Self::StdPrimitiveU32,
            PostgresqlJsonArrayElementType::StdPrimitiveU64 => Self::StdPrimitiveU64,
            PostgresqlJsonArrayElementType::StdPrimitiveF32 => Self::StdPrimitiveF32,
            PostgresqlJsonArrayElementType::StdPrimitiveF64 => Self::StdPrimitiveF64,
            PostgresqlJsonArrayElementType::StdPrimitiveBool => Self::StdPrimitiveBool,
            PostgresqlJsonArrayElementType::StdStringString => Self::StdStringString,

            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI8 => Self::StdPrimitiveI8,
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI16 => Self::StdPrimitiveI16,
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI32 => Self::StdPrimitiveI32,
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI64 => Self::StdPrimitiveI64,
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU8 => Self::StdPrimitiveU8,
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU16 => Self::StdPrimitiveU16,
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU32 => Self::StdPrimitiveU32,
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU64 => Self::StdPrimitiveU64,
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveF32 => Self::StdPrimitiveF32,
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveF64 => Self::StdPrimitiveF64,
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveBool => Self::StdPrimitiveBool,
            PostgresqlJsonArrayElementType::StdOptionOptionStdStringString => Self::StdStringString,
        }
    }
}
impl std::convert::From<&PostgresqlJsonArrayElementType> for PostgresqlJsonTypePattern {
    fn from(value: &PostgresqlJsonArrayElementType) -> Self {
        match value {
            PostgresqlJsonArrayElementType::StdPrimitiveI8 |
            PostgresqlJsonArrayElementType::StdPrimitiveI16 |
            PostgresqlJsonArrayElementType::StdPrimitiveI32 |
            PostgresqlJsonArrayElementType::StdPrimitiveI64 |
            PostgresqlJsonArrayElementType::StdPrimitiveU8 |
            PostgresqlJsonArrayElementType::StdPrimitiveU16 |
            PostgresqlJsonArrayElementType::StdPrimitiveU32 |
            PostgresqlJsonArrayElementType::StdPrimitiveU64 |
            PostgresqlJsonArrayElementType::StdPrimitiveF32 |
            PostgresqlJsonArrayElementType::StdPrimitiveF64 |
            PostgresqlJsonArrayElementType::StdPrimitiveBool |
            PostgresqlJsonArrayElementType::StdStringString => Self::FullTypePath,

            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI8 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI16 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI32 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI64 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU8 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU16 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU32 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU64 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveF32 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveF64 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveBool |
            PostgresqlJsonArrayElementType::StdOptionOptionStdStringString => Self::StdOptionOptionFullTypePath,
        }
    }
}
impl std::convert::From<&PostgresqlJsonArrayElementType> for PostgresqlJsonTypePatternSpecific {
    fn from(value: &PostgresqlJsonArrayElementType) -> Self {
        match value {
            PostgresqlJsonArrayElementType::StdPrimitiveI8 |
            PostgresqlJsonArrayElementType::StdPrimitiveI16 |
            PostgresqlJsonArrayElementType::StdPrimitiveI32 |
            PostgresqlJsonArrayElementType::StdPrimitiveI64 |
            PostgresqlJsonArrayElementType::StdPrimitiveU8 |
            PostgresqlJsonArrayElementType::StdPrimitiveU16 |
            PostgresqlJsonArrayElementType::StdPrimitiveU32 |
            PostgresqlJsonArrayElementType::StdPrimitiveU64 |
            PostgresqlJsonArrayElementType::StdPrimitiveF32 |
            PostgresqlJsonArrayElementType::StdPrimitiveF64 => Self::FullTypePathNumber,
            PostgresqlJsonArrayElementType::StdPrimitiveBool => Self::FullTypePathBool,
            PostgresqlJsonArrayElementType::StdStringString => Self::FullTypePathString,

            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI8 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI16 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI32 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveI64 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU8 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU16 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU32 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveU64 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveF32 |
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveF64 => Self::StdOptionOptionFullTypePathNumber,
            PostgresqlJsonArrayElementType::StdOptionOptionStdPrimitiveBool => Self::StdOptionOptionFullTypePathBool,
            PostgresqlJsonArrayElementType::StdOptionOptionStdStringString => Self::StdOptionOptionFullTypePathString,
        }
    }
}


#[proc_macro]
pub fn generate_postgresql_json_types(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    fn generate_postgresql_json_type_handle_token_stream(variant: &PostgresqlJsonType) -> proc_macro2::TokenStream {
        let (
            postgresql_json_type_handle,
            postgresql_json_type_pattern
        ) = variant.to_postgresql_json_type_handle_and_postgresql_json_type_pattern();
        let ident: &dyn naming::StdFmtDisplayPlusQuoteToTokens = &variant;//PostgresqlJsonType::from((&postgresql_json_type_pattern, &postgresql_json_type_handle))
        let field_type = &postgresql_json_type_pattern.field_type(&postgresql_json_type_handle);

        let ident_token_stream = {
            quote::quote!{
                #[derive(
                    Debug,
                    Clone,
                    PartialEq,
                    PartialOrd,
                    Default,
                    serde::Serialize,
                    serde::Deserialize,
                    utoipa::ToSchema,
                    schemars::JsonSchema,
                )]
                pub struct #ident(pub #field_type);//todo #[validate(range(min = -128i8, max = 127i8))]
            }
        };

        //todo migrate to newest version of schemars crate then write validation logic.

        // pub struct StdPrimitiveI8(#[validate(range(min = -128i8, max = 127i8))] pub std::primitive::i8);
        // pub struct StdPrimitiveI16(#[validate(range(min = -32_768i16, max = 32_767i16))] pub std::primitive::i16);
        // pub struct StdPrimitiveI32(#[validate(range(min = -2_147_483_648i32, max = 2_147_483_647i32))] pub std::primitive::i32);
        // pub struct StdPrimitiveI64(#[validate(range(min = -9_223_372_036_854_775_808i64, max = 9_223_372_036_854_775_807i64))] pub std::primitive::i64);
        // pub struct StdPrimitiveU8(#[validate(range(min = 0u8, max = 255u8))] pub std::primitive::u8);
        // pub struct StdPrimitiveU16(#[validate(range(min = 0u16, max = 65_535u16))] pub std::primitive::u16);
        // pub struct StdPrimitiveU32(#[validate(range(min = 0u32, max = 4_294_967_295u32))] pub std::primitive::u32);
        // pub struct StdPrimitiveU64(#[validate(range(min = 0u64, max = 18_446_744_073_709_551_615u64))] pub std::primitive::u64);
        // //todo maybe its not correct. https://doc.rust-lang.org/std/primitive.f32.html
        // pub struct StdPrimitiveF32(#[validate(range(min = -3.40282347E+38f32, max = 3.40282347E+38f32))] pub std::primitive::f32);
        // //todo maybe its not correct. https://doc.rust-lang.org/core/primitive.f64.html
        // pub struct StdPrimitiveF64(#[validate(range(min = -1.7976931348623157E+308f64, max = 1.7976931348623157E+308f64))] pub std::primitive::f64);

        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &ident,
            &{
                let content_token_stream = postgresql_json_type_pattern.initialization_token_stream();
                quote::quote!{
                    Self(#content_token_stream)
                }
            },
        );
        let impl_error_occurence_lib_to_std_string_string_for_ident_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(
            &ident,
            &quote::quote!{format!("{self:#?}")}
        );

        let postgresql_json_type_ident_to_create_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfToCreateUpperCamelCase::from_tokens(&ident);
        let postgresql_json_type_ident_to_create_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_json_type_ident_to_create_upper_camel_case, &ident);
        let postgresql_json_type_ident_field_reader_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfFieldReaderUpperCamelCase::from_tokens(&ident);
        let postgresql_json_type_ident_field_reader_token_stream = {
            let content_token_stream = match &postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::FullTypePath |
                PostgresqlJsonTypePattern::StdOptionOptionFullTypePath => quote::quote!{{}},
                PostgresqlJsonTypePattern::StdVecVecFullTypePath |
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath |
                PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath |
                PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => quote::quote!{{ pagination: crate::pagination::Pagination }},
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
                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::FullTypePath |
                    PostgresqlJsonTypePattern::StdOptionOptionFullTypePath => {
                        let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                        quote::quote! {
                            #core_default_default_default
                        }
                    },
                    PostgresqlJsonTypePattern::StdVecVecFullTypePath |
                    PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath |
                    PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath |
                    PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => {
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
        let postgresql_json_type_ident_options_to_read_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfOptionsToReadUpperCamelCase::from_tokens(&ident);
        let postgresql_json_type_ident_options_to_read_alias_token_stream = macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(&postgresql_json_type_ident_options_to_read_upper_camel_case, &ident);
        
        let postgresql_json_type_ident_where_element_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&ident);
        let postgresql_json_type_ident_where_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfWhereUpperCamelCase::from_tokens(&ident);
        //todo impl try_new + custom serde::Deserialize
        let postgresql_json_type_ident_where_token_stream = {
            quote::quote!{
                #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
                pub struct #postgresql_json_type_ident_where_upper_camel_case {
                    logical_operator: crate::LogicalOperator,
                    value: std::vec::Vec<#postgresql_json_type_ident_where_element_upper_camel_case>,
                }
            }
        };
        let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_json_type_ident_where_token_stream = {
            quote::quote!{
                impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_json_type_ident_where_upper_camel_case {
                    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
                        let mut acc = std::string::String::default();
                        let mut is_need_to_add_logical_operator_inner_handle = false;
                        for element in &self.value {
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
                        Ok(format!("{}({acc})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                    }
                    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        for element in self.value {
                            query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
                        }
                        query
                    }
                }
            }
        };
        let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_where_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &postgresql_json_type_ident_where_upper_camel_case,
            &quote::quote!{
                Self {
                    logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                    value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                }
            },
        );
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
                
                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::FullTypePath |
                    PostgresqlJsonTypePattern::StdOptionOptionFullTypePath => {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                            &format!("jsonb_build_object('{{field_ident}}', jsonb_build_object('value', {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}'))")
                        );
                        quote::quote! {
                            format!(#format_handle_token_stream)
                        }
                    },
                    PostgresqlJsonTypePattern::StdVecVecFullTypePath |
                    PostgresqlJsonTypePattern::StdVecVecStdOptionOptionFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                        &generate_quotes::double_quotes_token_stream(
                            &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))")
                        )
                    ),
                    PostgresqlJsonTypePattern::StdOptionOptionStdVecVecFullTypePath |
                    PostgresqlJsonTypePattern::StdOptionOptionStdVecVecStdOptionOptionFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                        &generate_quotes::double_quotes_token_stream(
                            &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value', case when jsonb_typeof({{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}') = 'array' then (select jsonb_agg(value) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}}) else null end))")
                        )
                    ),
                }
            },
            &postgresql_json_type_ident_where_element_upper_camel_case,
            &postgresql_json_type_ident_where_upper_camel_case,
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
                impl crate::BindQuery<'_> for #ident {
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
        let postgresql_json_type_ident_where_element_token_stream = {
            let postgresql_json_type_ident_where_element_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&ident);
            
            let equal = crate::filters::Equal;
            let postgresql_json_type_ident_where_element_equal_token_stream = equal.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                &variant,
            );
            let length_more_than = crate::filters::LengthMoreThan;
            let postgresql_json_type_ident_where_element_length_more_than_token_stream = length_more_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                &variant,
            );


            let position_equals = PositionEquals;
            #[derive(Debug)]
            enum MaybePostgresqlJsonTypeIdentWhereElementPositionEquals {
                Some {
                    postgresql_json_type_ident_where_element_position_equals_token_stream: proc_macro2::TokenStream,
                },
                None,
            }
            impl quote::ToTokens for MaybePostgresqlJsonTypeIdentWhereElementPositionEquals {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    match &self {
                        Self::Some {
                            postgresql_json_type_ident_where_element_position_equals_token_stream
                        } => {
                            postgresql_json_type_ident_where_element_position_equals_token_stream.to_tokens(tokens)
                        },
                        Self::None => proc_macro2::TokenStream::new().to_tokens(tokens)
                    }
                }
            }
            let maybe_postgresql_json_type_ident_where_element_position_equals = match PostgresqlJsonArrayElementType::try_from(variant) {
                Ok(value) => MaybePostgresqlJsonTypeIdentWhereElementPositionEquals::Some {
                    postgresql_json_type_ident_where_element_position_equals_token_stream: position_equals.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&variant, &value)
                },
                Err(_) => MaybePostgresqlJsonTypeIdentWhereElementPositionEquals::None
            };

            // let = Equal;
            // let = GreaterThan;
            // let = Between;
            // let = In;
            // let = CaseSensitiveRegularExpression;
            // let = CaseInsensitiveRegularExpression;

            // let = Before;
            // let = CurrentDate;
            // let = GreaterThanCurrentDate;
            // let = CurrentTimestamp;
            // let = GreaterThanCurrentTimestamp;
            // let = CurrentTime;
            // let = GreaterThanCurrentTime;
            // let = LengthMoreThan;+
            // let = EqualToEncodedStringRepresentation;
            // let = ValueIsContainedWithinRange;
            // let = ContainsAnotherRange;
            // let = StrictlyToLeftOfRange;
            // let = StrictlyToRightOfRange;
            // let = IncludedLowerBound;
            // let = ExcludedUpperBound;
            // let = GreaterThanLowerBound;
            // let = OverlapWithRange;
            // let = AdjacentWithRange;
            // let = RangeLength;
            // let = PositionEquals;+
            
            //todo remove generate_postgresql_json_type_where_element_token_stream later
            let generate_postgresql_json_type_where_element_token_stream = || -> proc_macro2::TokenStream {
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &vec![
                        &equal,
                    ],
                    &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&ident),
                    &ShouldImplementSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #postgresql_json_type_ident_where_element_equal_token_stream

                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let generate_postgresql_json_type_where_element_number_token_stream = || {
                //todo maybe remove ident, field_type from arguments. variant is enough
                let greater_than = crate::filters::GreaterThan;
                let postgresql_json_type_ident_where_element_greater_than_token_stream = greater_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                    &variant,
                );
                let between = crate::filters::Between;
                let postgresql_json_type_ident_where_element_between_token_stream = between.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &variant,
                );
                let in_handle = crate::filters::In;
                let postgresql_json_type_ident_where_element_in_token_stream = in_handle.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                    &variant,
                );
                //todo write wrapper around it with reuse parameters
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &vec![
                        &equal,
                        &greater_than,
                        &between,
                        &in_handle,
                    ],
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldImplementSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #postgresql_json_type_ident_where_element_equal_token_stream
                    #postgresql_json_type_ident_where_element_greater_than_token_stream
                    #postgresql_json_type_ident_where_element_between_token_stream
                    #postgresql_json_type_ident_where_element_in_token_stream

                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let generate_postgresql_json_type_where_element_bool_token_stream = || {
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &vec![
                        &equal,
                    ],
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldImplementSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #postgresql_json_type_ident_where_element_equal_token_stream

                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let generate_postgresql_json_type_where_element_string_token_stream = || {
                let case_sensitive_regular_expression = crate::filters::CaseSensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                    &ident,
                );
                let case_insensitive_regular_expression = crate::filters::CaseInsensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                    &ident,
                );

                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &vec![
                        &equal,
                        &case_sensitive_regular_expression,
                        &case_insensitive_regular_expression,
                    ],
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldImplementSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #postgresql_json_type_ident_where_element_equal_token_stream
                    #postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream
                    #postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream

                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };


            let generate_postgresql_json_type_where_element_vec_number_token_stream = || {
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &{
                        let mut vec: std::vec::Vec<&dyn WhereOperatorName> = vec![
                            &equal,
                            &length_more_than,
                        ];
                        if let MaybePostgresqlJsonTypeIdentWhereElementPositionEquals::Some { postgresql_json_type_ident_where_element_position_equals_token_stream: _ } = maybe_postgresql_json_type_ident_where_element_position_equals {
                            vec.push(&position_equals);
                        }
                        vec
                    },
                    &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&ident),
                    &ShouldImplementSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #postgresql_json_type_ident_where_element_equal_token_stream
                    #postgresql_json_type_ident_where_element_length_more_than_token_stream
                    #maybe_postgresql_json_type_ident_where_element_position_equals

                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let generate_postgresql_json_type_where_element_vec_bool_token_stream = || {
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &{
                        let mut vec: std::vec::Vec<&dyn WhereOperatorName> = vec![
                            &equal,
                            &length_more_than,
                        ];
                        if let MaybePostgresqlJsonTypeIdentWhereElementPositionEquals::Some { postgresql_json_type_ident_where_element_position_equals_token_stream: _ } = maybe_postgresql_json_type_ident_where_element_position_equals {
                            vec.push(&position_equals);
                        }
                        vec
                    },
                    &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&ident),
                    &ShouldImplementSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #postgresql_json_type_ident_where_element_equal_token_stream
                    #postgresql_json_type_ident_where_element_length_more_than_token_stream
                    #maybe_postgresql_json_type_ident_where_element_position_equals

                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let generate_postgresql_json_type_where_element_vec_string_token_stream = || {
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &{
                        let mut vec: std::vec::Vec<&dyn WhereOperatorName> = vec![
                            &equal,
                            &length_more_than,
                        ];
                        if let MaybePostgresqlJsonTypeIdentWhereElementPositionEquals::Some { postgresql_json_type_ident_where_element_position_equals_token_stream: _ } = maybe_postgresql_json_type_ident_where_element_position_equals {
                            vec.push(&position_equals);
                        }
                        vec
                    },
                    &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&ident),
                    &ShouldImplementSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #postgresql_json_type_ident_where_element_equal_token_stream
                    #postgresql_json_type_ident_where_element_length_more_than_token_stream
                    #maybe_postgresql_json_type_ident_where_element_position_equals

                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            match &PostgresqlJsonTypePatternSpecific::from(variant) {
                PostgresqlJsonTypePatternSpecific::FullTypePathNumber => generate_postgresql_json_type_where_element_number_token_stream(),
                PostgresqlJsonTypePatternSpecific::FullTypePathBool => generate_postgresql_json_type_where_element_bool_token_stream(),
                PostgresqlJsonTypePatternSpecific::FullTypePathString => generate_postgresql_json_type_where_element_string_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdOptionOptionFullTypePathNumber => generate_postgresql_json_type_where_element_number_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdOptionOptionFullTypePathBool => generate_postgresql_json_type_where_element_bool_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdOptionOptionFullTypePathString => generate_postgresql_json_type_where_element_string_token_stream(),
                
                PostgresqlJsonTypePatternSpecific::StdVecVecFullTypePathNumber => generate_postgresql_json_type_where_element_vec_number_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdVecVecFullTypePathBool => generate_postgresql_json_type_where_element_vec_bool_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdVecVecFullTypePathString => generate_postgresql_json_type_where_element_vec_string_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecFullTypePathNumber => generate_postgresql_json_type_where_element_vec_number_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecFullTypePathBool => generate_postgresql_json_type_where_element_vec_bool_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecFullTypePathString => generate_postgresql_json_type_where_element_vec_string_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdVecVecStdOptionOptionFullTypePathNumber => generate_postgresql_json_type_where_element_vec_number_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdVecVecStdOptionOptionFullTypePathBool => generate_postgresql_json_type_where_element_vec_bool_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdVecVecStdOptionOptionFullTypePathString => generate_postgresql_json_type_where_element_vec_string_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecStdOptionOptionFullTypePathNumber => generate_postgresql_json_type_where_element_vec_number_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecStdOptionOptionFullTypePathBool => generate_postgresql_json_type_where_element_vec_bool_token_stream(),
                PostgresqlJsonTypePatternSpecific::StdOptionOptionStdVecVecStdOptionOptionFullTypePathString => generate_postgresql_json_type_where_element_vec_string_token_stream(),
            }
        };
        //todo maybe impl Encode instead of just wrap into sqlx::types::Json
        let mut generated = quote::quote!{
            #ident_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_ident_token_stream

            #postgresql_json_type_ident_to_create_alias_token_stream
            #postgresql_json_type_ident_field_reader_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_field_reader_token_stream
            #postgresql_json_type_ident_options_to_read_alias_token_stream
            #postgresql_json_type_ident_where_token_stream
            #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_json_type_ident_where_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_where_token_stream
            #postgresql_json_type_ident_option_to_update_alias_token_stream
            #postgresql_json_type_ident_option_to_update_try_generate_bind_increments_error_named_token_stream
            #impl_crate_generate_postgresql_json_type_postgresql_json_type_for_ident_token_stream

            #impl_crate_bind_query_for_token_stream

            #postgresql_json_type_ident_where_element_token_stream
        };
        //  if quote::quote!{#ident}.to_string() == "" {
        //     //  println!("{generated}");
        //     //  println!("-------");
        //      macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //          "PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeTime",
        //          &generated,
        //      );
        //      generated = quote::quote!{}
        //  }
        generated
    }
    let variants_token_stream = 
    PostgresqlJsonType::into_array()
    // [
    //     PostgresqlJsonType::StdPrimitiveI8,
    //     // PostgresqlJsonType::StdPrimitiveBool,
    //     // PostgresqlJsonType::StdStringString,
    //     // PostgresqlJsonType::StdOptionOptionStdPrimitiveI8,
    //     // PostgresqlJsonType::StdOptionOptionStdPrimitiveBool,
    //     // PostgresqlJsonType::StdOptionOptionStdStringString,
    //     PostgresqlJsonType::StdVecVecStdPrimitiveI8,
    // ]
    .into_iter().map(|element|generate_postgresql_json_type_handle_token_stream(&element));
    let generated = quote::quote! {
        #(#variants_token_stream)*
    };
    //  if ident == "" {
    //      println!("{generated}");
    //      println!("-------");
    //  }
    generated.into()
}

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
    let crate_bind_query_token_stream = quote::quote!{crate::BindQuery::};
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
            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
                <#field_type_token_stream as sqlx::Type<sqlx::Postgres>>::compatible(ty)
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

enum PostgresqlTypeNullableOrNotNull {
    Nullable,
    NotNull
}
impl PostgresqlTypeNullableOrNotNull {
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

enum PostgresqlTypeInitializedByTokens {
    InitializedUsingDefaultKeywordByPostgresql,
    InitializedUsingUuidGenerateV4FunctionByPostgresql,
    InitializedByClient,
}
fn generate_postgresql_type_initialized_by_tokens(input: proc_macro::TokenStream, postgresql_type_initialized_by_tokens: PostgresqlTypeInitializedByTokens) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generate_postgresql_type_nullable_or_not_null = |postgresql_type_nullable_or_not_null: &PostgresqlTypeNullableOrNotNull| -> proc_macro2::TokenStream {
        let postgresql_type_field_type_where_element_upper_camel_case: &dyn quote::ToTokens = &postgresql_type_nullable_or_not_null.postgresql_type_field_type_where_element_upper_camel_case(&field_type);
        let ident_handle: &dyn quote::ToTokens = &postgresql_type_nullable_or_not_null.ident_handle(&ident);
        let field_type_handle: &dyn quote::ToTokens = &postgresql_type_nullable_or_not_null.field_type_handle(&field_type);
        let postgresql_type_ident_column_upper_camel_case = naming::parameter::PostgresqlTypeSelfColumnUpperCamelCase::from_tokens(&ident_handle);

        let try_generate_bind_increments_snake_case = naming::TryGenerateBindIncrementsSnakeCase;
        let bind_value_to_query_snake_case = naming::BindValueToQuerySnakeCase;
        let crate_bind_query_token_stream = quote::quote!{crate::BindQuery::};

        let crate_bind_query_try_generate_bind_increments_token_stream = quote::quote!{#crate_bind_query_token_stream #try_generate_bind_increments_snake_case};
        let crate_bind_query_bind_value_to_query_token_stream = quote::quote!{#crate_bind_query_token_stream #bind_value_to_query_snake_case};

        let increment_snake_case = naming::IncrementSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let self_snake_case = naming::SelfSnakeCase;
        let self_dot_zero_token_stream = quote::quote!{#self_snake_case.0};
        let ok_std_string_string_from_default_token_stream = quote::quote!{Ok(std::string::String::from("DEFAULT"))};
        let ok_std_string_string_from_uuid_generate_v4_token_stream = quote::quote!{Ok(std::string::String::from("uuid_generate_v4()"))};
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
        let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
        let self_core_default_default_default_token_stream = quote::quote!{Self(#core_default_default_default_token_stream)};
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
        let empty_struct_content_token_stream = quote::quote!{(());};
        let postgresql_type_ident_to_create_upper_camel_case = naming::parameter::PostgresqlTypeSelfToCreateUpperCamelCase::from_tokens(&ident_handle);
        let postgresql_type_ident_to_create_token_stream = {
            let postgresql_type_ident_to_create_token_stream = generate_pub_struct_tokens_token_stream(
                Visibility::Pub,
                &postgresql_type_ident_to_create_upper_camel_case,
                match &postgresql_type_initialized_by_tokens {
                    PostgresqlTypeInitializedByTokens::InitializedUsingDefaultKeywordByPostgresql => &empty_struct_content_token_stream,
                    PostgresqlTypeInitializedByTokens::InitializedUsingUuidGenerateV4FunctionByPostgresql => &empty_struct_content_token_stream,
                    PostgresqlTypeInitializedByTokens::InitializedByClient => &field_type_struct_content_token_stream,
                },
                false,
                true,
            );
            let impl_crate_bind_query_for_postgresql_type_ident_to_create_token_stream = generate_impl_crate_bind_query_for_tokens_token_stream(
                &postgresql_type_ident_to_create_upper_camel_case,
                match &postgresql_type_initialized_by_tokens {
                    PostgresqlTypeInitializedByTokens::InitializedUsingDefaultKeywordByPostgresql => &ok_std_string_string_from_default_token_stream,
                    PostgresqlTypeInitializedByTokens::InitializedUsingUuidGenerateV4FunctionByPostgresql => &ok_std_string_string_from_uuid_generate_v4_token_stream,
                    PostgresqlTypeInitializedByTokens::InitializedByClient => &crate_bind_query_try_generate_bind_increments_self_zero_increment_token_stream,
                },
                match &postgresql_type_initialized_by_tokens {
                    PostgresqlTypeInitializedByTokens::InitializedUsingDefaultKeywordByPostgresql => &query_snake_case,
                    PostgresqlTypeInitializedByTokens::InitializedUsingUuidGenerateV4FunctionByPostgresql => &query_snake_case,
                    PostgresqlTypeInitializedByTokens::InitializedByClient => &crate_bind_query_bind_value_to_query_self_zero_query_token_stream,
                },
            );
            let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_to_create_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &postgresql_type_ident_to_create_upper_camel_case,
                match &postgresql_type_initialized_by_tokens {
                    PostgresqlTypeInitializedByTokens::InitializedUsingDefaultKeywordByPostgresql => &self_core_default_default_default_token_stream,
                    PostgresqlTypeInitializedByTokens::InitializedUsingUuidGenerateV4FunctionByPostgresql => &self_core_default_default_default_token_stream,
                    PostgresqlTypeInitializedByTokens::InitializedByClient => &self_braces_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                },
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
    //i think its logical - auto generated types in postgresql cannot be null? right? 
    let maybe_ident_nullable_token_stream = match &postgresql_type_initialized_by_tokens {
        PostgresqlTypeInitializedByTokens::InitializedUsingDefaultKeywordByPostgresql => proc_macro2::TokenStream::new(),
        PostgresqlTypeInitializedByTokens::InitializedUsingUuidGenerateV4FunctionByPostgresql => proc_macro2::TokenStream::new(),
        PostgresqlTypeInitializedByTokens::InitializedByClient => generate_postgresql_type_nullable_or_not_null(&PostgresqlTypeNullableOrNotNull::Nullable),
    };
    let ident_not_null_token_stream = generate_postgresql_type_nullable_or_not_null(&PostgresqlTypeNullableOrNotNull::NotNull);
    let generated = quote::quote!{
        #maybe_ident_nullable_token_stream
        #ident_not_null_token_stream
    };
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "GeneratePostgresqlTypeInitializedByTokens",
    //         &generated,
    //     );
    // }
    generated.into()
}
#[proc_macro_derive(PostgresqlTypeInitializedUsingUuidGenerateV4FunctionByPostgresqlTokens)]
pub fn postgresql_type_initialized_using_uuid_generate_v4_function_by_postgresql_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_type_initialized_by_tokens(input, PostgresqlTypeInitializedByTokens::InitializedUsingUuidGenerateV4FunctionByPostgresql)
}
#[proc_macro_derive(PostgresqlTypeInitializedUsingDefaultKeywordByPostgresqlTokens)]
pub fn postgresql_type_initialized_using_default_keyword_by_postgresql_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_type_initialized_by_tokens(input, PostgresqlTypeInitializedByTokens::InitializedUsingDefaultKeywordByPostgresql)
}
#[proc_macro_derive(PostgresqlTypeInitializedByClientTokens)]
pub fn postgresql_type_initialized_by_client_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_type_initialized_by_tokens(input, PostgresqlTypeInitializedByTokens::InitializedByClient)
}

enum IsPrimaryKey {
    True,
    False,
}
fn generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
    postgresql_type_nullable_or_not_null: &PostgresqlTypeNullableOrNotNull,
    ident: &syn::Ident,
    field_type: &syn::Type,
    is_primary_key: &IsPrimaryKey,
) -> proc_macro2::TokenStream {
    let ident_handle: &dyn quote::ToTokens = &postgresql_type_nullable_or_not_null.ident_handle(&ident);
    let ident_as_crate_create_table_column_query_part_create_table_column_query_part_column_is_primary_key_token_stream = quote::quote!{
        <#ident as crate::CreateTableColumnQueryPart>::create_table_column_query_part(column, is_primary_key)
    };
    let content_token_stream: &dyn quote::ToTokens = match &postgresql_type_nullable_or_not_null {
        PostgresqlTypeNullableOrNotNull::Nullable => &ident_as_crate_create_table_column_query_part_create_table_column_query_part_column_is_primary_key_token_stream,
        PostgresqlTypeNullableOrNotNull::NotNull => &match is_primary_key {
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
       &PostgresqlTypeNullableOrNotNull::Nullable,
        &ident,
        &field_type,
        &is_primary_key,
    );
    let impl_crate_create_table_column_query_part_for_ident_not_null_token_stream = generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
       &PostgresqlTypeNullableOrNotNull::NotNull,
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
    let impl_crate_create_table_column_query_part_for_ident_not_null_token_stream = generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
       &PostgresqlTypeNullableOrNotNull::NotNull,
        &ident,
        &field_type,
        &is_primary_key,
    );
    let generated = quote::quote!{
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
    let crate_bind_query_token_stream = quote::quote!{crate::BindQuery::};

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
        let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_create_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
            &postgresql_type_ident_not_null_to_create_upper_camel_case,
            &field_type
        );
        quote::quote! {
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
enum ShouldImplementSchemarsJsonSchema {
    True,
    False,
}
impl quote::ToTokens for ShouldImplementSchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::True => quote::quote! {, schemars::JsonSchema}.to_tokens(tokens),
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
        }
    }
}
fn generate_pub_enum_postgresql_type_tokens_where_element_token_stream(
    should_implement_schemars_json_schema: &ShouldImplementSchemarsJsonSchema,
    ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_implement_schemars_json_schema)]
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
    fn maybe_generate_postgresql_json_type_std_option_option_tokens_where_element_is_null_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
        match &self  {
            Self::True => {
                let column_snake_case = naming::ColumnSnakeCase;
                let query_snake_case = naming::QuerySnakeCase;
                let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
                    crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
                };
                let postgresql_json_type_ident_where_element_is_null_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfWhereElementIsNullUpperCamelCase::from_tokens(&ident);
                let postgresql_json_type_ident_where_element_is_null_token_stream = generate_postgresql_type_tokens_where_element_tokens_token_stream(
                    &postgresql_json_type_ident_where_element_is_null_upper_camel_case,
                    &ShouldWhereElementFieldsBePublic::True,
                    &ShouldImplementSchemarsJsonSchema::True,
                    &proc_macro2::TokenStream::new()
                );
                let impl_crate_generate_postgresql_json_type_std_default_default_but_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_where_element_is_null_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                    &postgresql_json_type_ident_where_element_is_null_upper_camel_case,
                    &quote::quote! {Self {
                        logical_operator: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    }},
                );
                let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_json_type_ident_where_element_is_null_token_stream = generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_token_stream(
                    &postgresql_json_type_ident_where_element_is_null_upper_camel_case,
                    &quote::quote! {
                        Ok(format!(
                            "{}({} = 'null'::jsonb)",
                            &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                            #column_snake_case,
                        ))
                    },
                    &query_snake_case
                );
                quote::quote! {
                    #postgresql_json_type_ident_where_element_is_null_token_stream
                    #impl_crate_generate_postgresql_json_type_std_default_default_but_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_where_element_is_null_token_stream
                    #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_json_type_ident_where_element_is_null_token_stream
                }
            },
            Self::False => proc_macro2::TokenStream::new(),
        }
    }
}
enum PostgresqlTypeOrJsonType {
    PostgresqlType,
    PostgresqlJsonType,
}
impl std::fmt::Display for PostgresqlTypeOrJsonType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::PostgresqlType => write!(formatter, "{}", naming::PostgresqlTypeUpperCamelCase),
            Self::PostgresqlJsonType => write!(formatter, "{}", naming::PostgresqlJsonTypeUpperCamelCase),
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
    fn maybe_generate_try_new_error_named_and_try_new_and_deserialize_token_stream(&self, postgresql_type_or_json_type: &PostgresqlTypeOrJsonType) -> proc_macro2::TokenStream {
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
                        "{postgresql_type_or_json_type}{}{postfix}{}",
                        naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident),
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
                        "{postgresql_type_or_json_type}{}{postfix}",
                        naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident),
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
fn generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
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
    let postgresql_type_std_option_option_ident_where_element_tokens_upper_camel_case = generate_postgresql_type_ident_where_element_tokens_upper_camel_case(
        &naming::parameter::PostgresqlTypeStdOptionOptionSelfWhereElementUpperCamelCase::from_tokens(&ident)
    );
    match &is_nullable {
        IsNullable::True => macros_helpers::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(
            &postgresql_type_std_option_option_ident_where_element_tokens_upper_camel_case,
            &postgresql_type_ident_where_element_tokens_upper_camel_case
        ),
        IsNullable::False => generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &PostgresqlTypeOrJsonType::PostgresqlType,
            &postgresql_type_ident_where_element_tokens_upper_camel_case,
            should_where_element_fields_be_public,
            &ShouldImplementSchemarsJsonSchema::False,
            &additional_type_declaration_token_stream,
            &additional_default_initialization_token_stream,
            &postgresql_type_self_where_try_generate_bind_increments_token_stream,
            &postgresql_type_self_where_bind_value_to_query_token_stream,
        )
    }
}
///////////////////////
fn generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
    postgresql_type_or_json_type: &PostgresqlTypeOrJsonType,
    postgresql_type_or_postgresql_json_type_ident_where_element_tokens_upper_camel_case: &dyn quote::ToTokens,
    should_where_element_fields_be_public: ShouldWhereElementFieldsBePublic,
    should_implement_schemars_json_schema: &ShouldImplementSchemarsJsonSchema,
    additional_type_declaration_token_stream: &dyn quote::ToTokens,
    additional_default_initialization_token_stream: &dyn quote::ToTokens,
    postgresql_type_self_where_try_generate_bind_increments_token_stream: &dyn quote::ToTokens,
    postgresql_type_self_where_bind_value_to_query_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_ident_where_element_tokens_token_stream = generate_postgresql_type_tokens_where_element_tokens_token_stream(
        &postgresql_type_or_postgresql_json_type_ident_where_element_tokens_upper_camel_case,
        &should_where_element_fields_be_public,
        &should_implement_schemars_json_schema,
        &additional_type_declaration_token_stream,
    );
    let maybe_try_new_error_named_and_try_new_and_deserialize_token_stream = should_where_element_fields_be_public.maybe_generate_try_new_error_named_and_try_new_and_deserialize_token_stream(postgresql_type_or_json_type);
    let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_ident_where_element_tokens_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
        &postgresql_type_or_postgresql_json_type_ident_where_element_tokens_upper_camel_case,
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
        &postgresql_type_or_postgresql_json_type_ident_where_element_tokens_upper_camel_case,
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
///////////////////////
fn generate_postgresql_type_tokens_where_element_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    should_where_element_fields_be_public: &ShouldWhereElementFieldsBePublic,
    should_implement_schemars_json_schema: &ShouldImplementSchemarsJsonSchema,
    additional_type_declaration_token_stream: &dyn quote::ToTokens,
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
        ShouldWhereElementFieldsBePublic::True => quote::quote! {, serde::Deserialize},
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
        #[derive(Debug, Clone, PartialEq, serde::Serialize #maybe_impl_serde_deserialize_token_stream #should_implement_schemars_json_schema)]
        pub struct #ident {
            #maybe_pub_token_stream #logical_operator_snake_case: crate::#logical_operator_upper_camel_case,
            #additional_type_declaration_token_stream
        }
    }
}
fn generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
    ident: &dyn quote::ToTokens,
    variants: &std::vec::Vec<&dyn WhereOperatorName>,
    postgresql_type_tokens_where_element_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    should_implement_schemars_json_schema: &ShouldImplementSchemarsJsonSchema,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let is_need_to_add_logical_operator_snake_case = naming::IsNeedToAddLogicalOperatorSnakeCase;
    let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    };
    let postgresql_type_tokens_where_element_token_stream = generate_pub_enum_postgresql_type_tokens_where_element_token_stream(
        should_implement_schemars_json_schema,
        &postgresql_type_tokens_where_element_upper_camel_case,
        &{
            let variants_token_stream = variants.iter().map(|element|{
                let element_upper_camel_case = element.upper_camel_case();
                let postgresql_type_tokens_where_element_equal_upper_camel_case = {
                    let value = format!("{postgresql_type_tokens_where_element_upper_camel_case}{}", quote::quote!{#element_upper_camel_case});
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote!{#element_upper_camel_case(#postgresql_type_tokens_where_element_equal_upper_camel_case)}
            });
            quote::quote!{#(#variants_token_stream),*}
        }
    );
    let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_tokens_where_element_token_stream = generate_impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_tokens_token_stream(
        &postgresql_type_tokens_where_element_upper_camel_case,
        &{
            let variants_token_stream = variants.iter().map(|element|{
                let element_upper_camel_case = element.upper_camel_case();
                quote::quote!{
                    Self::#element_upper_camel_case(#value_snake_case) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                        #value_snake_case,
                        #increment_snake_case,
                        #column_snake_case,
                        #is_need_to_add_logical_operator_snake_case,
                    )
                }
            });
            quote::quote!{
                match &self {
                    #(#variants_token_stream),*
                }
            }
        },
        &{
            let variants_token_stream = variants.iter().map(|element|{
                let element_upper_camel_case = element.upper_camel_case();
                quote::quote!{
                    Self::#element_upper_camel_case(#value_snake_case) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                        #value_snake_case,
                        #query_snake_case
                    )
                }
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
            let variants_token_stream = variants.iter().map(|element|{
                let element_upper_camel_case = element.upper_camel_case();
                quote::quote!{
                    Self::#element_upper_camel_case(
                        #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                    )
                }
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
fn generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
    is_nullable: IsNullable,
    ident: &dyn quote::ToTokens,
    variants: &std::vec::Vec<&dyn WhereOperatorName>
) -> proc_macro2::TokenStream {
    let postgresql_type_tokens_where_element_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &is_nullable {
        IsNullable::True => &naming::parameter::PostgresqlTypeStdOptionOptionSelfWhereElementUpperCamelCase::from_tokens(&ident),
        IsNullable::False => &naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident)
    };
    generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
        &ident,
        &variants,
        &postgresql_type_tokens_where_element_upper_camel_case,
        &ShouldImplementSchemarsJsonSchema::False,
    )
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
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens;
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
    fn std_option_option_type_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(value) => quote::quote!{std::option::Option<#value>},
            WhereOperatorType::FieldType {
                field_type,
                ..
            } => quote::quote!{std::option::Option<#field_type>},
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
    fn std_option_option_default_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            WhereOperatorType::Ident(_) => quote::quote!{
                Some(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
            },
            WhereOperatorType::FieldType {
                field_type: _,
                default_initialization_token_stream
            } => quote::quote!{Some(#default_initialization_token_stream)},
        }
    }
}

struct IsNull;
impl crate::WhereOperatorName for IsNull {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::IsNullUpperCamelCase
    }
}
enum IsNullablePostgresqlType<'a> {
    NullablePostgresqlType {
        where_operator_type: &'a WhereOperatorType<'a>,
    },
    NotNullPostgresqlType {
        where_operator_type: &'a WhereOperatorType<'a>,
    },
    PostgresqlJsonType,
}

struct StrictlyToLeftOfRange;
impl crate::WhereOperatorName for StrictlyToLeftOfRange {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::StrictlyToLeftOfRangeUpperCamelCase
    }
}
impl StrictlyToLeftOfRange {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
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
        )
    }
}
struct StrictlyToRightOfRange;
impl crate::WhereOperatorName for StrictlyToRightOfRange {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::StrictlyToRightOfRangeUpperCamelCase
    }
}
impl StrictlyToRightOfRange {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
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
        )
    }
}
struct IncludedLowerBound;
impl crate::WhereOperatorName for IncludedLowerBound {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::IncludedLowerBoundUpperCamelCase
    }
}
impl IncludedLowerBound {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
        range_type_token_stream: &dyn quote::ToTokens,
        range_type_default_initialization_token_stream: &dyn quote::ToTokens,
        range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
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
        )
    }
}
struct ExcludedUpperBound;
impl crate::WhereOperatorName for ExcludedUpperBound {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::ExcludedUpperBoundUpperCamelCase
    }
}
impl ExcludedUpperBound {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
        range_type_token_stream: &dyn quote::ToTokens,
        range_type_default_initialization_token_stream: &dyn quote::ToTokens,
        range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
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
        )
    }
}
struct GreaterThanLowerBound;
impl crate::WhereOperatorName for GreaterThanLowerBound {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::GreaterThanLowerBoundUpperCamelCase
    }
}
impl GreaterThanLowerBound {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
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
        )
    }
}
struct OverlapWithRange;
impl crate::WhereOperatorName for OverlapWithRange {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::OverlapWithRangeUpperCamelCase
    }
}
impl OverlapWithRange {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
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
        )
    }
}
struct AdjacentWithRange;
impl crate::WhereOperatorName for AdjacentWithRange {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::AdjacentWithRangeUpperCamelCase
    }
}
impl AdjacentWithRange {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            crate::WhereOperatorName::upper_camel_case(self),
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
        )
    }
}
struct RangeLength;
impl crate::WhereOperatorName for RangeLength {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::RangeLengthUpperCamelCase
    }
}
impl RangeLength {
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        };
        let self_upper_camel_case = crate::WhereOperatorName::upper_camel_case(self);
        let length_is_negative_or_zero_upper_camel_case = naming::LengthIsNegativeOrZeroUpperCamelCase;
        let std_primitive_i64_token_stream = quote::quote!{std::primitive::i64};
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &self_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::False {
                ident: &ident,
                postfix: &self_upper_camel_case,
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
                    ) = generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_ident_where_element_range_length_upper_camel_case, 2, &self_upper_camel_case);
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
    }
}
struct BitVecPositionEquals;
impl crate::WhereOperatorName for BitVecPositionEquals {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::BitVecPositionEqualsUpperCamelCase
    }
}
impl BitVecPositionEquals {
    fn std_primitive_bool_token_stream() -> proc_macro2::TokenStream {
        quote::quote!{std::primitive::bool}
    }
    //todo maybe should support js int instead of i32 and i64. (i64 wrapper with custom try_new impl for js max and min value check)
    fn std_primitive_i32_token_stream() -> proc_macro2::TokenStream {
        quote::quote!{std::primitive::i32}
    }
    fn position_snake_case() -> naming::PositionSnakeCase {
        naming::PositionSnakeCase
    }
    fn position_is_less_or_equal_zero_upper_camel_case() -> naming::PositionIsLessOrEqualZeroUpperCamelCase {
        naming::PositionIsLessOrEqualZeroUpperCamelCase
    }
    fn generate_try_new_error_named_variants_token_stream() -> proc_macro2::TokenStream {
        let position_snake_case = Self::position_snake_case();
        let position_is_less_or_equal_zero_upper_camel_case = Self::position_is_less_or_equal_zero_upper_camel_case();
        let std_primitive_i32_token_stream = Self::std_primitive_i32_token_stream();
        quote::quote!{
            #position_is_less_or_equal_zero_upper_camel_case {
                #[eo_to_std_string_string_serialize_deserialize]
                #position_snake_case: #std_primitive_i32_token_stream,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
            },
        }
    }
    fn generate_try_new_content_token_stream(
        ident: &dyn quote::ToTokens,
        postgresql_type_or_json_type: &PostgresqlTypeOrJsonType,
    ) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let position_snake_case = Self::position_snake_case();
        let position_is_less_or_equal_zero_upper_camel_case = Self::position_is_less_or_equal_zero_upper_camel_case();
        let postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = match &postgresql_type_or_json_type {
            PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementBitVecPositionEqualsTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
            PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementBitVecPositionEqualsTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        };
        quote::quote!{
            if #position_snake_case > 0 {
                Ok(Self {
                    logical_operator,
                    #value_snake_case,
                    #position_snake_case,
                })
            }
            else {
                Err(#postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_try_new_error_named_upper_camel_case::#position_is_less_or_equal_zero_upper_camel_case {
                    #position_snake_case,
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            }
        }
    }
    fn generate_impl_deserialize_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        postgresql_type_or_json_type: &PostgresqlTypeOrJsonType,
    ) -> proc_macro2::TokenStream {
        let postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_type_or_json_type {
            PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementBitVecPositionEqualsUpperCamelCase::from_tokens(&ident),
            PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementBitVecPositionEqualsUpperCamelCase::from_tokens(&ident),
        };
        let (
            struct_postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_double_quotes_token_stream,
            struct_postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_with_2_elements_double_quotes_token_stream,
            postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_double_quotes_token_stream
        ) = generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_upper_camel_case, 2, &crate::WhereOperatorName::upper_camel_case(self));
        let std_primitive_bool_token_stream = Self::std_primitive_bool_token_stream();
        let std_primitive_i32_token_stream = Self::std_primitive_i32_token_stream();
        quote::quote! {
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de>
                for #postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_upper_camel_case {
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
                                #postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_upper_camel_case,
                            >,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_upper_camel_case;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    #struct_postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_double_quotes_token_stream,
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
                                                &#struct_postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_with_2_elements_double_quotes_token_stream,
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
                                                &#struct_postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_with_2_elements_double_quotes_token_stream,
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
                                                &#struct_postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                match #postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_upper_camel_case::try_new(__field0, __field1, __field2) {
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
                                match #postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_upper_camel_case::try_new(__field0, __field1, __field2) {
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
                            #postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_double_quotes_token_stream,
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    #postgresql_type_or_json_type_ident_where_element_bit_vec_position_equals_upper_camel_case,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
        }
    }
    fn generate_additional_type_declaration_token_stream() -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let position_snake_case = Self::position_snake_case();
        let std_primitive_bool_token_stream = Self::std_primitive_bool_token_stream();
        let std_primitive_i32_token_stream = Self::std_primitive_i32_token_stream();
        quote::quote!{
            #value_snake_case: #std_primitive_bool_token_stream,
            #position_snake_case: #std_primitive_i32_token_stream,
        }
    }
    fn generate_additional_default_initialization_token_stream() -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let position_snake_case = Self::position_snake_case();
        let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
        quote::quote!{
            #value_snake_case: #core_default_default_default,
            #position_snake_case: #core_default_default_default,
        }
    }
    fn generate_try_generate_bind_increments_token_stream(postgresql_type_or_json_type: &PostgresqlTypeOrJsonType) -> proc_macro2::TokenStream {
        let increment_snake_case = naming::IncrementSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        match &postgresql_type_or_json_type {
            PostgresqlTypeOrJsonType::PostgresqlType => {
                quote::quote!{
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
                }
            },
            PostgresqlTypeOrJsonType::PostgresqlJsonType => {
                quote::quote!{
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
                }
            }
        }
    }
    fn generate_bind_value_to_query_token_stream(postgresql_type_or_json_type: &PostgresqlTypeOrJsonType) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let position_snake_case = Self::position_snake_case();
        match &postgresql_type_or_json_type {
            PostgresqlTypeOrJsonType::PostgresqlType => {
                quote::quote!{
                    #query_snake_case = #query_snake_case.bind(self.#position_snake_case);
                    #query_snake_case = #query_snake_case.bind(if self.#value_snake_case {
                        "1"
                    }
                    else {
                        "0"
                    });
                    #query_snake_case
                }
            },
            PostgresqlTypeOrJsonType::PostgresqlJsonType => {
                quote::quote!{
                    #query_snake_case = #query_snake_case.bind(self.#position_snake_case);
                    #query_snake_case = #query_snake_case.bind(if self.#value_snake_case {
                        "1"
                    }
                    else {
                        "0"
                    });
                    #query_snake_case
                }
            }
        }
    }
    fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        is_nullable: &IsNullable,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = crate::WhereOperatorName::upper_camel_case(self);
        let postgresql_type_or_json_type = PostgresqlTypeOrJsonType::PostgresqlType;
        let additional_type_declaration_token_stream = Self::generate_additional_type_declaration_token_stream();
        generate_maybe_nullable_postgresql_type_tokens_where_element_variant_token_stream(
            &ident,
            &self_upper_camel_case,
            &is_nullable,
            ShouldWhereElementFieldsBePublic::False {
                ident: &ident,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &Self::generate_try_new_error_named_variants_token_stream(),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &Self::generate_try_new_content_token_stream(&ident, &postgresql_type_or_json_type),
                impl_deserialize_token_stream: &self.generate_impl_deserialize_token_stream(
                    &ident,
                    &postgresql_type_or_json_type,
                ),
            },
            &additional_type_declaration_token_stream,
            &Self::generate_additional_default_initialization_token_stream(),
            &Self::generate_try_generate_bind_increments_token_stream(&postgresql_type_or_json_type),
            &Self::generate_bind_value_to_query_token_stream(&postgresql_type_or_json_type),
        )
    }
}

struct PositionEquals;
impl crate::WhereOperatorName for PositionEquals {
    fn upper_camel_case(&self) -> &'static dyn naming::StdFmtDisplayPlusQuoteToTokens {
        &naming::PositionEqualsUpperCamelCase
    }
}
impl PositionEquals {
    fn std_primitive_bool_token_stream() -> proc_macro2::TokenStream {
        quote::quote!{std::primitive::bool}
    }
    //todo maybe should support js int instead of i32 and i64. (i64 wrapper with custom try_new impl for js max and min value check)
    fn std_primitive_i32_token_stream() -> proc_macro2::TokenStream {
        quote::quote!{std::primitive::i32}
    }
    fn position_snake_case() -> naming::PositionSnakeCase {
        naming::PositionSnakeCase
    }
    fn position_is_less_than_zero_upper_camel_case() -> naming::PositionIsLessThanZeroUpperCamelCase {
        naming::PositionIsLessThanZeroUpperCamelCase
    }
    fn generate_try_new_error_named_variants_token_stream() -> proc_macro2::TokenStream {
        let position_snake_case = Self::position_snake_case();
        let position_is_less_than_zero_upper_camel_case = Self::position_is_less_than_zero_upper_camel_case();
        let std_primitive_i32_token_stream = Self::std_primitive_i32_token_stream();
        quote::quote!{
            #position_is_less_than_zero_upper_camel_case {
                #[eo_to_std_string_string_serialize_deserialize]
                #position_snake_case: #std_primitive_i32_token_stream,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
            },
        }
    }
    fn generate_try_new_content_token_stream(
        ident: &dyn quote::ToTokens,
        postgresql_type_or_json_type: &PostgresqlTypeOrJsonType,
    ) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let position_snake_case = Self::position_snake_case();
        let position_is_less_than_zero_upper_camel_case = Self::position_is_less_than_zero_upper_camel_case();
        let postgresql_type_or_json_type_ident_where_element_position_equals_try_new_error_named_upper_camel_case: &dyn quote::ToTokens = match &postgresql_type_or_json_type {
            PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementPositionEqualsTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
            PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementPositionEqualsTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        };
        quote::quote!{
            if #position_snake_case >= 0 {
                Ok(Self {
                    logical_operator,
                    #value_snake_case,
                    #position_snake_case,
                })
            }
            else {
                Err(#postgresql_type_or_json_type_ident_where_element_position_equals_try_new_error_named_upper_camel_case::#position_is_less_than_zero_upper_camel_case {
                    #position_snake_case,
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            }
        }
    }
    fn generate_impl_deserialize_token_stream(
        &self,
        ident: &dyn quote::ToTokens,
        postgresql_type_or_json_type: &PostgresqlTypeOrJsonType,
        postgresql_json_array_element_type: &PostgresqlJsonArrayElementType
    ) -> proc_macro2::TokenStream {
        let postgresql_type_or_json_type_ident_where_element_position_equals_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_type_or_json_type {
            PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereElementPositionEqualsUpperCamelCase::from_tokens(&ident),
            PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereElementPositionEqualsUpperCamelCase::from_tokens(&ident),
        };
        let (
            struct_postgresql_type_or_json_type_ident_where_element_position_equals_double_quotes_token_stream,
            struct_postgresql_type_or_json_type_ident_where_element_position_equals_with_2_elements_double_quotes_token_stream,
            postgresql_type_or_json_type_ident_where_element_position_equals_double_quotes_token_stream
        ) = generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_element_position_equals_upper_camel_case, 2, &crate::WhereOperatorName::upper_camel_case(self));
        let std_primitive_i32_token_stream = Self::std_primitive_i32_token_stream();
        quote::quote! {
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de>
                for #postgresql_type_or_json_type_ident_where_element_position_equals_upper_camel_case {
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
                                #postgresql_type_or_json_type_ident_where_element_position_equals_upper_camel_case,
                            >,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #postgresql_type_or_json_type_ident_where_element_position_equals_upper_camel_case;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    #struct_postgresql_type_or_json_type_ident_where_element_position_equals_double_quotes_token_stream,
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
                                                &#struct_postgresql_type_or_json_type_ident_where_element_position_equals_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    #postgresql_json_array_element_type,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &#struct_postgresql_type_or_json_type_ident_where_element_position_equals_with_2_elements_double_quotes_token_stream,
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
                                                &#struct_postgresql_type_or_json_type_ident_where_element_position_equals_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                match #postgresql_type_or_json_type_ident_where_element_position_equals_upper_camel_case::try_new(__field0, __field1, __field2) {
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
                                let mut __field1: _serde::__private::Option<#postgresql_json_array_element_type> = _serde::__private::None;
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
                                                    #postgresql_json_array_element_type,
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
                                match #postgresql_type_or_json_type_ident_where_element_position_equals_upper_camel_case::try_new(__field0, __field1, __field2) {
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
                            #postgresql_type_or_json_type_ident_where_element_position_equals_double_quotes_token_stream,
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    #postgresql_type_or_json_type_ident_where_element_position_equals_upper_camel_case,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
        }
    }
    fn generate_additional_type_declaration_token_stream(postgresql_json_array_element_type: &PostgresqlJsonArrayElementType) -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let position_snake_case = Self::position_snake_case();
        let std_primitive_i32_token_stream = Self::std_primitive_i32_token_stream();
        quote::quote!{
            #value_snake_case: #postgresql_json_array_element_type,
            #position_snake_case: #std_primitive_i32_token_stream,
        }
    }
    fn generate_additional_default_initialization_token_stream() -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let position_snake_case = Self::position_snake_case();
        let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
            let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
            quote::quote! {
                crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
            }
        };
        quote::quote!{
            #value_snake_case: #crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream,
            #position_snake_case: #core_default_default_default,
        }
    }
    fn generate_try_generate_bind_increments_token_stream() -> proc_macro2::TokenStream {
        let increment_snake_case = naming::IncrementSnakeCase;
        let column_snake_case = naming::ColumnSnakeCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
        quote::quote!{
            match #increment_snake_case.checked_add(1) {
                Some(first_increment) => {
                    *#increment_snake_case = first_increment;
                    match #increment_snake_case.checked_add(1) {
                        Some(second_increment) => {
                            *#increment_snake_case = second_increment;
                            Ok(format!(
                                "{}({}->${} = ${})",
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
        }
    }
    fn generate_bind_value_to_query_token_stream() -> proc_macro2::TokenStream {
        let value_snake_case = naming::ValueSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let position_snake_case = Self::position_snake_case();
        quote::quote!{
            #query_snake_case = #query_snake_case.bind(self.#position_snake_case);
            #query_snake_case = #query_snake_case.bind(sqlx::types::Json(self.#value_snake_case));
            #query_snake_case
        }
    }
    fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
        &self,
        variant: &PostgresqlJsonType,
        postgresql_json_array_element_type: &PostgresqlJsonArrayElementType,
    ) -> proc_macro2::TokenStream {
        let self_upper_camel_case = crate::WhereOperatorName::upper_camel_case(self);
        let postgresql_json_type_ident_where_element_tokens_upper_camel_case = {
            let value = format!("{}{self_upper_camel_case}", &naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&variant));
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let postgresql_type_or_json_type = PostgresqlTypeOrJsonType::PostgresqlJsonType;
        let additional_type_declaration_token_stream = Self::generate_additional_type_declaration_token_stream(&postgresql_json_array_element_type);
        generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
            &postgresql_type_or_json_type,
            &postgresql_json_type_ident_where_element_tokens_upper_camel_case,
            ShouldWhereElementFieldsBePublic::False {
                ident: &variant,
                postfix: &self_upper_camel_case,
                try_new_error_named_variants_token_stream: &Self::generate_try_new_error_named_variants_token_stream(),
                try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
                try_new_content_token_stream: &Self::generate_try_new_content_token_stream(
                    &variant,
                    &postgresql_type_or_json_type,
                ),
                impl_deserialize_token_stream: &self.generate_impl_deserialize_token_stream(
                    &variant,
                    &postgresql_type_or_json_type,
                    &postgresql_json_array_element_type,
                )
            },
            &ShouldImplementSchemarsJsonSchema::True,
            &additional_type_declaration_token_stream,
            &Self::generate_additional_default_initialization_token_stream(),
            &Self::generate_try_generate_bind_increments_token_stream(),
            &Self::generate_bind_value_to_query_token_stream()
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
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
        };
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let in_handle = crate::filters::In;
        let postgresql_type_tokens_where_element_in_token_stream = in_handle.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &between,
                &in_handle,
            ]
        );
        quote::quote! {
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
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::True,
        );
        let in_handle = crate::filters::In;
        let postgresql_type_tokens_where_element_in_token_stream = in_handle.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &between,
                &in_handle,
            ]
        );
        quote::quote! {
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
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
        };
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &between,
            ]
        );
        quote::quote! {
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
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &between,
            ]
        );
        quote::quote! {
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
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
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
                &equal,
            ]
        );
        quote::quote! {
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
        let case_sensitive_regular_expression = crate::filters::CaseSensitiveRegularExpression;
        let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let case_insensitive_regular_expression = crate::filters::CaseInsensitiveRegularExpression;
        let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &case_sensitive_regular_expression,
                &case_insensitive_regular_expression,
            ]
        );
        quote::quote! {
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
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &WhereOperatorType::FieldType {
                field_type: &field_type,
                default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
            },
        );
        let length_more_than = crate::filters::LengthMoreThan;
        let postgresql_type_tokens_where_element_length_more_than_token_stream = length_more_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let equal_to_encoded_string_representation = crate::filters::EqualToEncodedStringRepresentation;
        let postgresql_type_tokens_where_element_equal_to_encoded_string_representation_token_stream = equal_to_encoded_string_representation.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &length_more_than,
                &equal_to_encoded_string_representation,
            ]
        );
        quote::quote! {
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
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let current_date = crate::filters::CurrentDate;
        let postgresql_type_tokens_where_element_current_date_token_stream = current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let greater_than_current_date = crate::filters::GreaterThanCurrentDate;
        let postgresql_type_tokens_where_element_greater_than_current_date_token_stream = greater_than_current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &between,
                &current_date,
                &greater_than_current_date,
            ]
        );
        quote::quote! {
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
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
        };
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let current_date = crate::filters::CurrentDate;
        let postgresql_type_tokens_where_element_current_date_token_stream = current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let greater_than_current_date = crate::filters::GreaterThanCurrentDate;
        let postgresql_type_tokens_where_element_greater_than_current_date_token_stream = greater_than_current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &between,
                &current_date,
                &greater_than_current_date,
            ]
        );
        quote::quote! {
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
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
        };
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let current_time = crate::filters::CurrentTime;
        let postgresql_type_tokens_where_element_current_time_token_stream = current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let greater_than_current_time = crate::filters::GreaterThanCurrentTime;
        let postgresql_type_tokens_where_element_greater_than_current_time_token_stream = greater_than_current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &between,
                &current_time,
                &greater_than_current_time,
            ]
        );
        quote::quote! {
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
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::SqlxTypesTimeTimeMidnight,
        };
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let current_time = crate::filters::CurrentTime;
        let postgresql_type_tokens_where_element_current_time_token_stream = current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let greater_than_current_time = crate::filters::GreaterThanCurrentTime;
        let postgresql_type_tokens_where_element_greater_than_current_time_token_stream = greater_than_current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &between,
                &current_time,
                &greater_than_current_time,
            ]
        );
        quote::quote! {
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
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &crate::filters::BetweenTryNewErrorType::StartIsEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &between,
            ]
        );
        quote::quote! {
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
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &WhereOperatorType::Ident(&ident),
        );
        let value_is_contained_within_range = crate::filters::ValueIsContainedWithinRange;
        let postgresql_type_tokens_where_element_value_is_contained_within_range_token_stream = value_is_contained_within_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &range_type_token_stream,
            &range_type_should_impl_range_length,
            &range_type_default_initialization_token_stream,
            &range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream,
        );
        let contains_another_range = crate::filters::ContainsAnotherRange;
        let postgresql_type_tokens_where_element_contains_another_range_token_stream = contains_another_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let strictly_to_left_of_range = StrictlyToLeftOfRange;
        let postgresql_type_tokens_where_element_strictly_to_left_of_range_token_stream = strictly_to_left_of_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let strictly_to_right_of_range = StrictlyToRightOfRange;
        let postgresql_type_tokens_where_element_strictly_to_right_of_range_token_stream = strictly_to_right_of_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let included_lower_bound = IncludedLowerBound;
        let postgresql_type_tokens_where_element_included_lower_bound_token_stream = included_lower_bound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &range_type_token_stream,
            &range_type_default_initialization_token_stream,
            &range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream,
        );
        let excluded_upper_bound = ExcludedUpperBound;
        let postgresql_type_tokens_where_element_excluded_upper_bound_token_stream = ExcludedUpperBound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &range_type_token_stream,
            &range_type_default_initialization_token_stream,
            &range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream,
        );
        let greater_than_lower_bound = GreaterThanLowerBound;
        let postgresql_type_tokens_where_element_greater_than_lower_bound_token_stream = greater_than_lower_bound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let overlap_with_range = OverlapWithRange;
        let postgresql_type_tokens_where_element_overlap_with_range_token_stream = overlap_with_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let adjacent_with_range = AdjacentWithRange;
        let postgresql_type_tokens_where_element_adjacent_with_range_token_stream = adjacent_with_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        //todo find out maximum length of range(INT8RANGE, INT4RANGE) in postgresql
        let range_length = RangeLength;
        let maybe_postgresql_type_tokens_where_element_range_length_token_stream = match &range_type_should_impl_range_length {
            ShouldImplRangeLength::True => range_length.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                &ident,
                &is_nullable,
            ),
            ShouldImplRangeLength::False => proc_macro2::TokenStream::new(), 
        };
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &{
                let mut value: std::vec::Vec<&dyn WhereOperatorName> = vec![
                    &equal,
                    &value_is_contained_within_range,
                    &contains_another_range,
                    &strictly_to_left_of_range,
                    &strictly_to_right_of_range,
                    &included_lower_bound,
                    &excluded_upper_bound,
                    &greater_than_lower_bound,
                    &overlap_with_range,
                    &adjacent_with_range,
                ];
                if let ShouldImplRangeLength::True = &range_type_should_impl_range_length {
                    value.push(&range_length);
                }
                value
            }
        );
        quote::quote! {
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
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
        };
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let current_timestamp = crate::filters::CurrentTimestamp;
        let postgresql_type_tokens_where_element_current_timestamp_token_stream = current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let greater_than_current_timestamp = crate::filters::GreaterThanCurrentTimestamp;
        let postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream = greater_than_current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &between,
                &current_timestamp,
                &greater_than_current_timestamp,
            ]
        );
        quote::quote! {
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
        let sqlx_types_time_time_midnight = token_patterns::SqlxTypesTimeTimeMidnight;
        let where_operator_type_field_type = WhereOperatorType::FieldType {
            field_type: &field_type,
            default_initialization_token_stream: &sqlx_types_time_primitive_date_time_new_token_stream(),
        };
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_field_type,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let current_timestamp = crate::filters::CurrentTimestamp;
        let postgresql_type_tokens_where_element_current_timestamp_token_stream = current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let greater_than_current_timestamp = crate::filters::GreaterThanCurrentTimestamp;
        let postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream = greater_than_current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &between,
                &current_timestamp,
                &greater_than_current_timestamp,
            ]
        );
        quote::quote! {
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
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let before = crate::filters::Before;
        let postgresql_type_tokens_where_element_before_token_stream = before.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
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
                &equal,
                &before,
                &between,
            ]
        );
        quote::quote! {
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
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let before = crate::filters::Before;
        let postgresql_type_tokens_where_element_before_token_stream = before.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &before,
                &between,
            ]
        );
        quote::quote! {
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
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let before = crate::filters::Before;
        let postgresql_type_tokens_where_element_before_token_stream = before.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let between = crate::filters::Between;
        let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
            &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
            &crate::filters::ShouldAddDotZero::False,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &before,
                &between,
            ]
        );
        quote::quote! {
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
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &WhereOperatorType::Ident(&ident),
        );
        let case_sensitive_regular_expression = crate::filters::CaseSensitiveRegularExpression;
        let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let case_insensitive_regular_expression = crate::filters::CaseInsensitiveRegularExpression;
        let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &case_sensitive_regular_expression,
                &case_insensitive_regular_expression,
            ]
        );
        quote::quote! {
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
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
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
                &equal,
            ]
        );
        quote::quote! {
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
fn sqlx_types_ipnetwork_ip_network_v4_token_stream() -> proc_macro2::TokenStream {
    let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
    quote::quote!{sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::new(core::net::Ipv4Addr::UNSPECIFIED, #core_default_default_default).unwrap())}
}
#[proc_macro_derive(PostgresqlBaseTypeTokensSqlxTypesIpnetworkIpNetwork)]
pub fn postgresql_base_type_tokens_sqlx_types_ipnetwork_ip_network(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_base_type_tokens(
        input,
        &sqlx_types_ipnetwork_ip_network_v4_token_stream()
    )
}
#[proc_macro_derive(PostgresqlBaseTypeTokensWhereElementSqlxTypesIpnetworkIpNetwork)]
pub fn postgresql_base_type_tokens_where_element_sqlx_types_ipnetwork_ip_network(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let field_type = extract_first_syn_type_from_unnamed_struct(&syn_derive_input);
    let generated = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &WhereOperatorType::FieldType {
                field_type: &field_type,
                default_initialization_token_stream: &sqlx_types_ipnetwork_ip_network_v4_token_stream()
            },
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
            ]
        );
        quote::quote! {
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_token_stream
        }
    });
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesIpnetworkIpNetwork",
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
        let where_operator_type_ident = WhereOperatorType::Ident(&ident);
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident,
        );
        let greater_than = crate::filters::GreaterThan;
        let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &where_operator_type_ident
        );
        let case_sensitive_regular_expression = crate::filters::CaseSensitiveRegularExpression;
        let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let case_insensitive_regular_expression = crate::filters::CaseInsensitiveRegularExpression;
        let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &greater_than,
                &case_sensitive_regular_expression,
                &case_insensitive_regular_expression,
            ]
        );
        quote::quote! {
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
        let equal = crate::filters::Equal;
        let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &WhereOperatorType::Ident(&ident),
        );
        let bit_vec_position_equals = BitVecPositionEquals;
        let postgresql_type_tokens_where_element_bit_vec_position_equals_token_stream = bit_vec_position_equals.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &bit_vec_position_equals,
            ]
        );
        quote::quote! {
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_bit_vec_position_equals_token_stream
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