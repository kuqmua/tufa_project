mod filters;

struct PostgresqlJsonTypeVariant {
    postgresql_json_type_handle: PostgresqlJsonTypeHandle,
    postgresql_json_type_pattern: PostgresqlJsonTypePattern,
}
impl PostgresqlJsonTypeVariant {
    fn all_variants() -> std::vec::Vec<Self> {
        let mut acc = vec![];
        let postgresql_json_type_pattern_array = PostgresqlJsonTypePattern::all_variants();
        for postgresql_json_type_handle in PostgresqlJsonTypeHandle::into_array() {
            for postgresql_json_type_pattern in &postgresql_json_type_pattern_array {
                acc.push(Self {
                    postgresql_json_type_handle: postgresql_json_type_handle.clone(),
                    postgresql_json_type_pattern: postgresql_json_type_pattern.clone(),
                });
            }
        }
        acc
    }
    fn try_into_vec_element_type(&self) -> Result<Self, ()> {
        match &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
            PostgresqlJsonTypePatternType::FullTypePath => Err(()),
            //todo maybe wrong
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath |
            PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => Ok(Self {
                postgresql_json_type_handle: self.postgresql_json_type_handle.clone(),
                postgresql_json_type_pattern: self.postgresql_json_type_pattern.clone(),
            }),
        }
    }
    fn postgresql_json_type_ident_wrapper(&self) -> proc_macro2::TokenStream {
        format!(
            "{}{}",
            self.postgresql_json_type_pattern.prefix_stringified(),
            self.postgresql_json_type_handle,
        )
        .parse::<proc_macro2::TokenStream>()
        .unwrap()
    }

    fn handle_field_type(&self, is_wrapper: std::primitive::bool) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &self.postgresql_json_type_handle;
        match (&self.postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::FullTypePath) => if is_wrapper {
                quote::quote!{#postgresql_json_type_handle}
            }
            else {
                postgresql_json_type_handle.field_type_token_stream()
            },
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::FullTypePath) => quote::quote!{std::option::Option<#postgresql_json_type_handle>},
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => quote::quote!{std::vec::Vec<#postgresql_json_type_handle>},

            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => {
                // let value = {
                //     format!("{}{postgresql_json_type_handle}", &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type.prefix_stringified())
                //     .parse::<proc_macro2::TokenStream>()
                //     .unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonTypeHandle to proc_macro2::TokenStream"))
                // };
                // quote::quote!{std::option::Option<#value>}
                quote::quote!{std::option::Option<std::vec::Vec<#postgresql_json_type_handle>>}
            },
            (&PostgresqlJsonTypePatternIsOptional::False, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote!{std::vec::Vec<std::vec::Vec<#postgresql_json_type_handle>>}
            },
            (&PostgresqlJsonTypePatternIsOptional::True, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote!{std::option::Option<std::vec::Vec<std::vec::Vec<#postgresql_json_type_handle>>>}
            },
        }
    }
    fn handle_initialization_token_stream(&self, is_wrapper: std::primitive::bool) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &self.postgresql_json_type_handle;
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
            let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
            quote::quote! {
                crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
            }
        };
        //todo better code reuse
        match (&self.postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::FullTypePath) => if is_wrapper {
                quote::quote!{#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream}
            }
            else {
                postgresql_json_type_handle.full_type_path_initialization_token_stream()
            },
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::FullTypePath) => quote::quote!{Some(#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream)},
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => quote::quote!{vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream]},
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) => {
                quote::quote!{Some(vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream])}
                // quote::quote!{Some(#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream)}
            },
            (&PostgresqlJsonTypePatternIsOptional::False, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote!{vec![vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream]]}
            },
            (&PostgresqlJsonTypePatternIsOptional::True, &PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => {
                quote::quote!{Some(vec![vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream]])}
            },
        }
    }
    fn field_type(&self) -> proc_macro2::TokenStream {
        self.handle_field_type(false)
    }
    fn initialization_token_stream(&self) -> proc_macro2::TokenStream {
        self.handle_initialization_token_stream(false)
    }

    fn wrapper_field_type(&self) -> proc_macro2::TokenStream {
        self.handle_field_type(true)
    }
    fn wrapper_non_optional_field_type(&self) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &self.postgresql_json_type_handle;
        match &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
            PostgresqlJsonTypePatternType::FullTypePath => quote::quote!{#postgresql_json_type_handle},
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath => quote::quote!{std::vec::Vec<#postgresql_json_type_handle>},
            PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => quote::quote!{std::vec::Vec<std::vec::Vec<#postgresql_json_type_handle>>},
        }
    }
    fn wrapper_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        self.handle_initialization_token_stream(true)
    }
    fn wrapper_non_optional_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        let crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
            let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
            let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
            quote::quote! {
                crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()
            }
        };
        match &self.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
            PostgresqlJsonTypePatternType::FullTypePath => quote::quote!{#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream},
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath => quote::quote!{vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream]},
            PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => quote::quote!{vec![vec![#crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream]]},
        }
    }
}

#[derive(Debug, Clone)]
enum PostgresqlJsonTypeSpecific {
    Number,
    Bool,
    String,
}
impl std::convert::From<&PostgresqlJsonTypeHandle> for PostgresqlJsonTypeSpecific {
    fn from(value: &PostgresqlJsonTypeHandle) -> Self {
        match value {
            PostgresqlJsonTypeHandle::StdPrimitiveI8 |
            PostgresqlJsonTypeHandle::StdPrimitiveI16 |
            PostgresqlJsonTypeHandle::StdPrimitiveI32 |
            PostgresqlJsonTypeHandle::StdPrimitiveI64 |
            PostgresqlJsonTypeHandle::StdPrimitiveU8 |
            PostgresqlJsonTypeHandle::StdPrimitiveU16 |
            PostgresqlJsonTypeHandle::StdPrimitiveU32 |
            PostgresqlJsonTypeHandle::StdPrimitiveU64 |
            PostgresqlJsonTypeHandle::StdPrimitiveF32 |
            PostgresqlJsonTypeHandle::StdPrimitiveF64 => Self::Number,
            PostgresqlJsonTypeHandle::StdPrimitiveBool => Self::Bool,
            PostgresqlJsonTypeHandle::StdStringString |
            PostgresqlJsonTypeHandle::UuidUuid => Self::String,
        }
    }
}

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
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
    UuidUuid
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
        self.field_type_stringified()
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonTypeHandle to proc_macro2::TokenStream"))
    }
    fn full_type_path_initialization_token_stream(&self) -> proc_macro2::TokenStream {
        match &self {
            Self::StdPrimitiveI8 |
            Self::StdPrimitiveI16 |
            Self::StdPrimitiveI32 |
            Self::StdPrimitiveI64 |
            Self::StdPrimitiveU8 |
            Self::StdPrimitiveU16 |
            Self::StdPrimitiveU32 |
            Self::StdPrimitiveU64 |
            Self::StdPrimitiveF32 |
            Self::StdPrimitiveF64 |
            Self::StdPrimitiveBool |
            Self::StdStringString => {
                let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
                quote::quote!{#core_default_default_default_token_stream}
            },
            Self::UuidUuid => quote::quote!{
                uuid::Uuid::new_v4()
            },
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

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
enum PostgresqlJsonTypePatternIsOptional {
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

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
enum PostgresqlJsonTypePatternType {
    FullTypePath,
    StdVecVecFullTypePath,
    StdVecVecStdVecVecFullTypePath,
}
impl PostgresqlJsonTypePatternType {
    fn prefix_stringified(&self) -> std::string::String {
        match &self {
            //todo maybe reuse from naming:: ?
            PostgresqlJsonTypePatternType::FullTypePath => std::string::String::default(),
            PostgresqlJsonTypePatternType::StdVecVecFullTypePath => std::string::String::from("StdVecVec"),
            PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => std::string::String::from("StdVecVecStdVecVec"),
        }
    }
}

#[derive(Debug, Clone)]
struct PostgresqlJsonTypePattern {
    postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional,
    postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType,
}
impl PostgresqlJsonTypePattern {
    fn prefix_stringified(&self) -> std::string::String {
        format!(
            "{}{}",
            &self.postgresql_json_type_pattern_is_optional.prefix_stringified(),
            &self.postgresql_json_type_pattern_type.prefix_stringified(),
        )
    }
    fn all_variants() -> std::vec::Vec<Self> {
        let mut acc = vec![];
        let postgresql_json_type_pattern_is_optional_array = PostgresqlJsonTypePatternIsOptional::into_array();
        for postgresql_json_type_pattern_type in PostgresqlJsonTypePatternType::into_array() {
            for postgresql_json_type_pattern_is_optional in &postgresql_json_type_pattern_is_optional_array {
                acc.push(Self {
                    postgresql_json_type_pattern_is_optional: postgresql_json_type_pattern_is_optional.clone(),
                    postgresql_json_type_pattern_type: postgresql_json_type_pattern_type.clone(),
                });
            }
        }
        acc
    }
}

#[proc_macro]
pub fn generate_postgresql_json_types(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    fn generate_postgresql_json_type_handle_token_stream(postgresql_json_type_variant: &PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
        let postgresql_json_type_handle = &postgresql_json_type_variant.postgresql_json_type_handle;
        let postgresql_json_type_pattern = &postgresql_json_type_variant.postgresql_json_type_pattern;
        let postgresql_json_type_ident_wrapper = postgresql_json_type_variant.postgresql_json_type_ident_wrapper();

        let ident: &dyn quote::ToTokens = &postgresql_json_type_ident_wrapper;
        let field_type = &postgresql_json_type_variant.field_type();

        let ident_token_stream = {
            let maybe_derive_schemars_json_schema_token_stream = {
                let schemars_json_schema_token_stream = quote::quote!{schemars::JsonSchema,};
                match (&postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
                    (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::FullTypePath) => match &postgresql_json_type_handle {
                        PostgresqlJsonTypeHandle::StdPrimitiveI8 |
                        PostgresqlJsonTypeHandle::StdPrimitiveI16 |
                        PostgresqlJsonTypeHandle::StdPrimitiveI32 |
                        PostgresqlJsonTypeHandle::StdPrimitiveI64 |
                        PostgresqlJsonTypeHandle::StdPrimitiveU8 |
                        PostgresqlJsonTypeHandle::StdPrimitiveU16 |
                        PostgresqlJsonTypeHandle::StdPrimitiveU32 |
                        PostgresqlJsonTypeHandle::StdPrimitiveU64 |
                        PostgresqlJsonTypeHandle::StdPrimitiveF32 |
                        PostgresqlJsonTypeHandle::StdPrimitiveF64 |
                        PostgresqlJsonTypeHandle::StdPrimitiveBool |
                        PostgresqlJsonTypeHandle::StdStringString => schemars_json_schema_token_stream,
                        PostgresqlJsonTypeHandle::UuidUuid => proc_macro2::TokenStream::new(),
                    },
                    (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::FullTypePath) |
                    (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) |
                    (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) |
                    (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) |
                    (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => schemars_json_schema_token_stream,
                }
            };
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
                    #maybe_derive_schemars_json_schema_token_stream
                )]
                pub struct #ident(pub #field_type);//todo #[validate(range(min = -128i8, max = 127i8))]
            }
        };
        let maybe_impl_schemars_json_schema_for_ident_token_stream = match (&postgresql_json_type_pattern.postgresql_json_type_pattern_is_optional, &postgresql_json_type_pattern.postgresql_json_type_pattern_type) {
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::FullTypePath) => match &postgresql_json_type_handle {
                PostgresqlJsonTypeHandle::StdPrimitiveI8 |
                PostgresqlJsonTypeHandle::StdPrimitiveI16 |
                PostgresqlJsonTypeHandle::StdPrimitiveI32 |
                PostgresqlJsonTypeHandle::StdPrimitiveI64 |
                PostgresqlJsonTypeHandle::StdPrimitiveU8 |
                PostgresqlJsonTypeHandle::StdPrimitiveU16 |
                PostgresqlJsonTypeHandle::StdPrimitiveU32 |
                PostgresqlJsonTypeHandle::StdPrimitiveU64 |
                PostgresqlJsonTypeHandle::StdPrimitiveF32 |
                PostgresqlJsonTypeHandle::StdPrimitiveF64 |
                PostgresqlJsonTypeHandle::StdPrimitiveBool |
                PostgresqlJsonTypeHandle::StdStringString => proc_macro2::TokenStream::new(),
                PostgresqlJsonTypeHandle::UuidUuid => {
                    quote::quote!{
                        impl schemars::JsonSchema for UuidUuid {
                            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                                schemars::_private::alloc::borrow::Cow::Borrowed("UuidUuid")
                            }
                            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                                schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::UuidUuid")
                            }
                            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                                {
                                    let mut schema = generator.subschema_for::<std::string::String>();
                                    schemars::_private::insert_validation_property(&mut schema, "string", "minLength", 36);
                                    schemars::_private::insert_validation_property(&mut schema, "string", "maxLength", 36);
                                    schemars::_private::insert_validation_property(&mut schema, "array", "minItems", 36);
                                    schemars::_private::insert_validation_property(&mut schema, "array", "maxItems", 36);
                                    schema
                                }
                            }
                        }
                    }
                },
            },
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::FullTypePath) |
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) |
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecFullTypePath) |
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) |
            (PostgresqlJsonTypePatternIsOptional::False, PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) |
            (PostgresqlJsonTypePatternIsOptional::True, PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath) => proc_macro2::TokenStream::new(),
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
                let content_token_stream = postgresql_json_type_variant.initialization_token_stream();
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
            let content_token_stream = match &postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                PostgresqlJsonTypePatternType::FullTypePath => quote::quote!{{}},
                PostgresqlJsonTypePatternType::StdVecVecFullTypePath => quote::quote!{{ pagination: crate::pagination::Pagination }},
                PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => quote::quote!{{ pagination: crate::pagination::Pagination }},//todo another pagination?
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
                match &postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                    PostgresqlJsonTypePatternType::FullTypePath => {
                        let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                        quote::quote! {
                            #core_default_default_default
                        }
                    },
                    PostgresqlJsonTypePatternType::StdVecVecFullTypePath => {
                        let generate_postgresql_json_type_snake_case = naming::GeneratePostgresqlJsonTypeSnakeCase;
                        let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
                        let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
                        quote::quote! {
                            Self {
                                pagination: crate::#generate_postgresql_json_type_snake_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case(),
                            }
                        }
                    },
                    PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => {
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
        let postgresql_json_type_where_token_stream = generate_postgresql_type_or_json_type_where_token_stream(
            &PostgresqlTypeOrJsonType::PostgresqlJsonType,
            &ident,
            &postgresql_json_type_ident_where_element_upper_camel_case,
            &postgresql_json_type_ident_where_upper_camel_case
        );
        enum MaybePostgresqlJsonTypeIdentWhereElementFilter<'a> {
            Some {
                where_operator_name: &'a dyn crate::filters::WhereOperatorName,
                token_stream: proc_macro2::TokenStream,
            },
            None,
        }
        impl quote::ToTokens for MaybePostgresqlJsonTypeIdentWhereElementFilter<'_> {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                match &self {
                    Self::Some { where_operator_name: _, token_stream } => {
                        token_stream.to_tokens(tokens)
                    },
                    Self::None => proc_macro2::TokenStream::new().to_tokens(tokens)
                }
            }
        }
        let postgresql_json_type_ident_where_element_token_stream = {
            let postgresql_json_type_ident_where_element_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfWhereElementUpperCamelCase::from_tokens(&ident);
            
            let equal = crate::filters::Equal;
            let postgresql_json_type_ident_where_element_equal_token_stream = equal.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                &postgresql_json_type_variant,
            );

            let common_postgresql_json_type_filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = vec![
                &equal,
            ];
            let mut common_postgresql_json_type_filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = vec![
                postgresql_json_type_ident_where_element_equal_token_stream
            ];
            
            let length_equal = crate::filters::LengthEqual;
            let postgresql_json_type_ident_where_element_length_equal_token_stream = length_equal.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                &postgresql_json_type_variant,
            );
            let length_more_than = crate::filters::LengthMoreThan;
            let postgresql_json_type_ident_where_element_length_more_than_token_stream = length_more_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                &postgresql_json_type_variant,
            );

            let position_equal = crate::filters::PositionEqual;
            let position_greater_than = crate::filters::PositionGreaterThan;
            let position_case_sensitive_regular_expression = crate::filters::PositionCaseSensitiveRegularExpression;
            let position_case_insensitive_regular_expression = crate::filters::PositionCaseInsensitiveRegularExpression;
            let contains_all_elements_of_array = crate::filters::ContainsAllElementsOfArray;
            let contained_in_array = crate::filters::ContainedInArray;
            let overlaps_with_array = crate::filters::OverlapsWithArray;
            let all_elements_equal = crate::filters::AllElementsEqual;
            let contains_element_greater_than = crate::filters::ContainsElementGreaterThan;
            let all_elements_greater_than = crate::filters::AllElementsGreaterThan;
            let contains_element_case_sensitive_regular_expression = crate::filters::ContainsElementCaseSensitiveRegularExpression;
            let contains_element_case_insensitive_regular_expression = crate::filters::ContainsElementCaseInsensitiveRegularExpression;
            let all_elements_case_sensitive_regular_expression = crate::filters::AllElementsCaseSensitiveRegularExpression;
            let all_elements_case_insensitive_regular_expression = crate::filters::AllElementsCaseInsensitiveRegularExpression;

            let (
                maybe_postgresql_json_type_ident_where_element_position_equal,
                maybe_postgresql_json_type_ident_where_element_position_greater_than,
                maybe_postgresql_json_type_ident_where_element_position_case_sensitive_regular_expression,
                maybe_postgresql_json_type_ident_where_element_position_case_insensitive_regular_expression,
                maybe_postgresql_json_type_ident_where_element_contains_all_elements_of_array,
                maybe_postgresql_json_type_ident_where_element_contained_in_array,
                maybe_postgresql_json_type_ident_where_element_overlaps_with_array,
                maybe_postgresql_json_type_ident_where_element_all_elements_equal,
                maybe_postgresql_json_type_ident_where_element_contains_element_greater_than,
                maybe_postgresql_json_type_ident_where_element_all_elements_greater_than,

                maybe_postgresql_json_type_ident_where_element_contains_element_case_sensitive_regular_expression,
                maybe_postgresql_json_type_ident_where_element_contains_element_case_insensitive_regular_expression,
                maybe_postgresql_json_type_ident_where_element_all_elements_case_sensitive_regular_expression,
                maybe_postgresql_json_type_ident_where_element_all_elements_case_insensitive_regular_expression,
            ) = match postgresql_json_type_variant.try_into_vec_element_type() {
                Ok(value) => (
                    //todo maybe should use value type in regular expression
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &position_equal,
                        token_stream: position_equal.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                            &value,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &position_greater_than,
                        token_stream: position_greater_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                            &value,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &position_case_sensitive_regular_expression,
                        token_stream: position_case_sensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &position_case_insensitive_regular_expression,
                        token_stream: position_case_insensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &contains_all_elements_of_array,
                        token_stream: contains_all_elements_of_array.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                            &value,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &contained_in_array,
                        token_stream: contained_in_array.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                            &value,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &overlaps_with_array,
                        token_stream: overlaps_with_array.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                            &value,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &all_elements_equal,
                        token_stream: all_elements_equal.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_ident_wrapper,
                            &value,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &contains_element_greater_than,
                        token_stream: contains_element_greater_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                            &value,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &all_elements_greater_than,
                        token_stream: all_elements_greater_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                            &value,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &contains_element_case_sensitive_regular_expression,
                        token_stream: contains_element_case_sensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &contains_element_case_insensitive_regular_expression,
                        token_stream: contains_element_case_insensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &all_elements_case_sensitive_regular_expression,
                        token_stream: all_elements_case_sensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                        )
                    },
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::Some {
                        where_operator_name: &all_elements_case_insensitive_regular_expression,
                        token_stream: all_elements_case_insensitive_regular_expression.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_json_type_variant,
                        )
                    },
                ),
                Err(_) => (
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                    MaybePostgresqlJsonTypeIdentWhereElementFilter::None,
                )
            };

            let mut common_postgresql_json_type_vec_filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = {
                let mut vec: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_filters_variants.clone();
                vec.push(&length_equal);
                vec.push(&length_more_than);
                vec
            };
            let mut common_postgresql_json_type_vec_filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = {
                let mut vec: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_filters_token_stream.clone();
                vec.push(postgresql_json_type_ident_where_element_length_equal_token_stream);
                vec.push(postgresql_json_type_ident_where_element_length_more_than_token_stream);
                vec
            };

            if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_position_equal {
                common_postgresql_json_type_vec_filters_variants.push(where_operator_name);
                common_postgresql_json_type_vec_filters_token_stream.push(token_stream);
            }
            if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_contains_all_elements_of_array {
                common_postgresql_json_type_vec_filters_variants.push(where_operator_name);
                common_postgresql_json_type_vec_filters_token_stream.push(token_stream);
            }
            //will not use it coz do not understand how it works
            // if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_contained_in_array {
            //     common_postgresql_json_type_vec_filters_variants.push(where_operator_name);
            //     common_postgresql_json_type_vec_filters_token_stream.push(token_stream);
            // }
            if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_overlaps_with_array {
                common_postgresql_json_type_vec_filters_variants.push(where_operator_name);
                common_postgresql_json_type_vec_filters_token_stream.push(token_stream);
            }
            if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_all_elements_equal {
                common_postgresql_json_type_vec_filters_variants.push(where_operator_name);
                common_postgresql_json_type_vec_filters_token_stream.push(token_stream);
            }

            let generate_postgresql_json_type_where_element_number_token_stream = || {
                //todo maybe remove ident, field_type from arguments. variant is enough
                let greater_than = crate::filters::GreaterThan;
                let postgresql_json_type_ident_where_element_greater_than_token_stream = greater_than.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_json_type_variant
                );
                let between = crate::filters::Between;
                let postgresql_json_type_ident_where_element_between_token_stream = between.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &postgresql_json_type_variant
                );
                let in_handle = crate::filters::In;
                let postgresql_json_type_ident_where_element_in_token_stream = in_handle.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_json_type_variant
                );
                //todo write wrapper around it with reuse parameters
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &{
                        let mut vec = common_postgresql_json_type_filters_variants.clone();
                        vec.push(&greater_than);
                        vec.push(&between);
                        vec.push(&in_handle);
                        vec
                    },
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #(#common_postgresql_json_type_filters_token_stream)*

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
                    &{
                        let vec = common_postgresql_json_type_filters_variants.clone();
                        vec
                    },
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #(#common_postgresql_json_type_filters_token_stream)*

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
                    &{
                        let mut vec = common_postgresql_json_type_filters_variants.clone();
                        vec.push(&case_sensitive_regular_expression);
                        vec.push(&case_insensitive_regular_expression);
                        vec
                    },
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #(#common_postgresql_json_type_filters_token_stream)*

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
                let mut filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
                let mut filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_vec_filters_token_stream.clone();
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_position_greater_than {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_contains_element_greater_than {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_all_elements_greater_than {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &filters_variants,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #(#filters_token_stream)*
                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let generate_postgresql_json_type_where_element_vec_bool_token_stream = || {
                let mut filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
                let mut filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_vec_filters_token_stream.clone();
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &filters_variants,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #(#filters_token_stream)*
                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let generate_postgresql_json_type_where_element_vec_string_token_stream = || {
                let mut filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
                let mut filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_vec_filters_token_stream.clone();
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_position_case_sensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_position_case_insensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_contains_element_case_sensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_contains_element_case_insensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_all_elements_case_sensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                if let MaybePostgresqlJsonTypeIdentWhereElementFilter::Some { where_operator_name, token_stream } = maybe_postgresql_json_type_ident_where_element_all_elements_case_insensitive_regular_expression {
                    filters_variants.push(where_operator_name);
                    filters_token_stream.push(token_stream);
                }
                let postgresql_json_type_ident_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &filters_variants,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #(#filters_token_stream)*
                    #postgresql_json_type_ident_where_element_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let postgresql_json_type_specific = PostgresqlJsonTypeSpecific::from(&postgresql_json_type_variant.postgresql_json_type_handle);
            match &postgresql_json_type_variant.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                PostgresqlJsonTypePatternType::FullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => generate_postgresql_json_type_where_element_number_token_stream(),
                    PostgresqlJsonTypeSpecific::Bool => generate_postgresql_json_type_where_element_bool_token_stream(),
                    PostgresqlJsonTypeSpecific::String => generate_postgresql_json_type_where_element_string_token_stream(),
                },
                PostgresqlJsonTypePatternType::StdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => generate_postgresql_json_type_where_element_vec_number_token_stream(),
                    PostgresqlJsonTypeSpecific::Bool => generate_postgresql_json_type_where_element_vec_bool_token_stream(),
                    PostgresqlJsonTypeSpecific::String => generate_postgresql_json_type_where_element_vec_string_token_stream(),
                },
                PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => generate_postgresql_json_type_where_element_vec_number_token_stream(),
                    PostgresqlJsonTypeSpecific::Bool => generate_postgresql_json_type_where_element_vec_bool_token_stream(),
                    PostgresqlJsonTypeSpecific::String => generate_postgresql_json_type_where_element_vec_string_token_stream(),
                },
            }
        };
        //
        let postgresql_json_type_ident_where_element_second_dimension_token_stream = {
            let postgresql_json_type_ident_where_element_second_dimension_upper_camel_case = naming::parameter::PostgresqlJsonTypeSelfWhereElementSecondDimensionUpperCamelCase::from_tokens(&ident);
            
            let equal_second_dimension = crate::filters::EqualSecondDimension;
            let postgresql_json_type_ident_where_element_equal_second_dimension_token_stream = equal_second_dimension.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
                &postgresql_json_type_variant,
            );

            let common_postgresql_json_type_filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = vec![
                &equal_second_dimension,
            ];
            let mut common_postgresql_json_type_filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = vec![
                postgresql_json_type_ident_where_element_equal_second_dimension_token_stream
            ];

            // let length_equal_second_dimension = crate::filters::LengthEqualSecondDimension;
            // let postgresql_json_type_ident_where_element_length_equal_token_stream = length_equal_second_dimension.generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
            //     &postgresql_json_type_variant,
            // );

            let mut common_postgresql_json_type_vec_filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = {
                let mut vec: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_filters_variants.clone();
                vec
            };
            let mut common_postgresql_json_type_vec_filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = {
                let mut vec: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_filters_token_stream.clone();
                vec
            };

            let generate_postgresql_json_type_where_element_vec_string_second_dimension_token_stream = || {
                let filters_variants: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = common_postgresql_json_type_vec_filters_variants.clone();
                let filters_token_stream: std::vec::Vec<proc_macro2::TokenStream> = common_postgresql_json_type_vec_filters_token_stream.clone();
                let postgresql_json_type_ident_where_element_second_dimension_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
                    &ident,
                    &filters_variants,
                    &postgresql_json_type_ident_where_element_second_dimension_upper_camel_case,
                    &postgresql_json_type_ident_where_element_upper_camel_case,
                    &ShouldDeriveSchemarsJsonSchema::True,
                );
                let generated = quote::quote!{
                    #(#filters_token_stream)*
                    #postgresql_json_type_ident_where_element_second_dimension_token_stream
                };
                // if ident == "" {
                //     println!("{generated}");
                //     println!("-------");
                // }
                generated
            };
            let postgresql_json_type_specific = PostgresqlJsonTypeSpecific::from(&postgresql_json_type_variant.postgresql_json_type_handle);
            match &postgresql_json_type_variant.postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                PostgresqlJsonTypePatternType::FullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => proc_macro2::TokenStream::new(),
                    PostgresqlJsonTypeSpecific::Bool => proc_macro2::TokenStream::new(),
                    PostgresqlJsonTypeSpecific::String => proc_macro2::TokenStream::new(),
                },
                PostgresqlJsonTypePatternType::StdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => proc_macro2::TokenStream::new(),
                    PostgresqlJsonTypeSpecific::Bool => proc_macro2::TokenStream::new(),
                    PostgresqlJsonTypeSpecific::String => generate_postgresql_json_type_where_element_vec_string_second_dimension_token_stream(),
                },
                PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => proc_macro2::TokenStream::new(),
                    PostgresqlJsonTypeSpecific::Bool => proc_macro2::TokenStream::new(),
                    PostgresqlJsonTypeSpecific::String => proc_macro2::TokenStream::new(),
                },
            }
        };
        // println!("{}", quote::quote!{#postgresql_json_type_ident_where_element_second_dimension_token_stream});
        //
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
                
                match &postgresql_json_type_pattern.postgresql_json_type_pattern_type {
                    PostgresqlJsonTypePatternType::FullTypePath => {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                            &format!("jsonb_build_object('{{field_ident}}', jsonb_build_object('value', {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}'))")
                        );
                        quote::quote! {
                            format!(#format_handle_token_stream)
                        }
                    },
                    PostgresqlJsonTypePatternType::StdVecVecFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                        &generate_quotes::double_quotes_token_stream(
                            &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))")
                        )
                    ),
                    // PostgresqlJsonTypePatternType::StdVecVecStdOptionOptionFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                    //     &generate_quotes::double_quotes_token_stream(
                    //         &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value', case when jsonb_typeof({{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}') = 'array' then (select jsonb_agg(value) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}}) else null end))")
                    //     )
                    // ),
                    PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                        &generate_quotes::double_quotes_token_stream(
                            &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))")
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
        //todo maybe impl Encode instead of just wrap into sqlx::types::Json
        let mut generated = quote::quote!{
            #ident_token_stream
            #maybe_impl_schemars_json_schema_for_ident_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_ident_token_stream

            #postgresql_json_type_ident_to_create_alias_token_stream
            #postgresql_json_type_ident_field_reader_token_stream
            #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_json_type_ident_field_reader_token_stream
            #postgresql_json_type_ident_options_to_read_alias_token_stream
            #postgresql_json_type_where_token_stream
            #postgresql_json_type_ident_where_element_token_stream
            #postgresql_json_type_ident_where_element_second_dimension_token_stream
            #postgresql_json_type_ident_option_to_update_alias_token_stream
            #postgresql_json_type_ident_option_to_update_try_generate_bind_increments_error_named_token_stream
            #impl_crate_generate_postgresql_json_type_postgresql_json_type_for_ident_token_stream
        };
        // println!("{}", quote::quote!{#ident});
        // if quote::quote!{#ident}.to_string() == "StdVecVecStdVecVecUuidUuid" {
        //    //  println!("{generated}");
        //    //  println!("-------");
        //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //         "PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeTime",
        //         &generated,
        //     );
        //     generated = quote::quote!{}
        // }
        generated
    }
    let variants_token_stream = 
    // PostgresqlJsonTypeVariant::all_variants()
    [
        // PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: PostgresqlJsonTypeHandle::StdPrimitiveI8,
        //     postgresql_json_type_pattern: PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::False,
        //         postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::FullTypePath,
        //     },
        // },

        PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
            postgresql_json_type_pattern: PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::False,
                postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::FullTypePath,
            },
        },
        PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
            postgresql_json_type_pattern: PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::True,
                postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::FullTypePath,
            },
        },
        PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
            postgresql_json_type_pattern: PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::False,
                postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::StdVecVecFullTypePath,
            },
        },
        PostgresqlJsonTypeVariant {
            postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
            postgresql_json_type_pattern: PostgresqlJsonTypePattern {
                postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::True,
                postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::StdVecVecFullTypePath,
            },
        },
        // PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
        //     postgresql_json_type_pattern: PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::False,
        //         postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath,
        //     },
        // },
        // PostgresqlJsonTypeVariant {
        //     postgresql_json_type_handle: PostgresqlJsonTypeHandle::UuidUuid,
        //     postgresql_json_type_pattern: PostgresqlJsonTypePattern {
        //         postgresql_json_type_pattern_is_optional: PostgresqlJsonTypePatternIsOptional::True,
        //         postgresql_json_type_pattern_type: PostgresqlJsonTypePatternType::StdVecVecStdVecVecFullTypePath,
        //     },
        // },
    ]
    .into_iter().map(|element|generate_postgresql_json_type_handle_token_stream(&element));
    let generated = quote::quote! {
        #(#variants_token_stream)*
    };
    //  if ident == "" {
    //      println!("{generated}");
    //      println!("-------");
    //  }
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeTime",
    //     &generated,
    // );
    generated.into()
}

fn generate_postgresql_type_or_json_type_where_token_stream(
    postgresql_type_or_json_type: &PostgresqlTypeOrJsonType,
    ident: &dyn quote::ToTokens,
    postgresql_type_or_json_type_ident_where_element_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    postgresql_type_or_json_type_ident_where_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_or_json_type_ident_where_token_stream = {
        let should_derive_schemars_json_schema = ShouldDeriveSchemarsJsonSchema::from(postgresql_type_or_json_type);
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize #should_derive_schemars_json_schema)]
            pub struct #postgresql_type_or_json_type_ident_where_upper_camel_case {
                logical_operator: crate::LogicalOperator,
                value: std::vec::Vec<#postgresql_type_or_json_type_ident_where_element_upper_camel_case>,
            }
        }
    };
    let postgresql_type_or_json_type_ident_where_try_new_error_named_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &postgresql_type_or_json_type {
        PostgresqlTypeOrJsonType::PostgresqlType => &naming::parameter::PostgresqlTypeSelfWhereTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
        PostgresqlTypeOrJsonType::PostgresqlJsonType => &naming::parameter::PostgresqlJsonTypeSelfWhereTryNewErrorNamedUpperCamelCase::from_tokens(&ident),
    };
    let is_empty_upper_camel_case = naming::IsEmptyUpperCamelCase;
    let not_unique_upper_camel_case = naming::NotUniqueUpperCamelCase;
    let postgresql_type_or_json_type_ident_where_try_new_error_named_token_stream = {
        quote::quote!{
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            pub enum #postgresql_type_or_json_type_ident_where_try_new_error_named_upper_camel_case {
                #is_empty_upper_camel_case {
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                #not_unique_upper_camel_case {
                    #[eo_to_std_string_string_serialize_deserialize]
                    value: #postgresql_type_or_json_type_ident_where_element_upper_camel_case,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
            }
        }
    };
    let impl_postgresql_type_or_json_type_ident_where_try_new_token_stream = {
        let logical_operator_upper_camel_case = naming::LogicalOperatorUpperCamelCase;
        let logical_operator_snake_case = naming::LogicalOperatorSnakeCase;
        quote::quote!{
            impl #postgresql_type_or_json_type_ident_where_upper_camel_case {
                fn try_new(
                    #logical_operator_snake_case: crate::#logical_operator_upper_camel_case,
                    value: std::vec::Vec<#postgresql_type_or_json_type_ident_where_element_upper_camel_case>,
                ) -> Result<Self, #postgresql_type_or_json_type_ident_where_try_new_error_named_upper_camel_case> {
                    if value.is_empty() {
                        return Err(#postgresql_type_or_json_type_ident_where_try_new_error_named_upper_camel_case::#is_empty_upper_camel_case {
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
                                return Err(#postgresql_type_or_json_type_ident_where_try_new_error_named_upper_camel_case::#not_unique_upper_camel_case {
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
    let impl_serde_deserialize_for_postgresql_type_or_json_type_ident_where_token_stream = {
        let (
            struct_postgresql_json_type_ident_where_double_quotes_token_stream,
            struct_postgresql_json_type_ident_where_with_2_elements_double_quotes_token_stream,
            postgresql_json_type_ident_where_double_quotes_token_stream
        ) = crate::generate_serde_deserialize_double_quotes_token_stream(&postgresql_type_or_json_type_ident_where_upper_camel_case, 2);
        quote::quote!{
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for #postgresql_type_or_json_type_ident_where_upper_camel_case {
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
                            marker: _serde::__private::PhantomData<#postgresql_type_or_json_type_ident_where_upper_camel_case>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = #postgresql_type_or_json_type_ident_where_upper_camel_case;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    #struct_postgresql_json_type_ident_where_double_quotes_token_stream,
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
                                                &#struct_postgresql_json_type_ident_where_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    std::vec::Vec<#postgresql_type_or_json_type_ident_where_element_upper_camel_case>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &#struct_postgresql_json_type_ident_where_with_2_elements_double_quotes_token_stream,
                                            ),
                                        );
                                    }
                                };
                                match #postgresql_type_or_json_type_ident_where_upper_camel_case::try_new(__field0, __field1) {
                                    Ok(value) => serde::__private::Ok(value),
                                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
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
                                let mut __field1: _serde::__private::Option<std::vec::Vec<#postgresql_type_or_json_type_ident_where_element_upper_camel_case>> = _serde::__private::None;
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
                                                    std::vec::Vec<#postgresql_type_or_json_type_ident_where_element_upper_camel_case>,
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
                                match #postgresql_type_or_json_type_ident_where_upper_camel_case::try_new(__field0, __field1) {
                                    Ok(value) => serde::__private::Ok(value),
                                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
                                }
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            #postgresql_json_type_ident_where_double_quotes_token_stream,
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<#postgresql_type_or_json_type_ident_where_upper_camel_case>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
        }
    };
    let impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_or_json_type_ident_where_token_stream = {
        quote::quote!{
            impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for #postgresql_type_or_json_type_ident_where_upper_camel_case {
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
    let impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_or_json_type_ident_where_token_stream = generate_impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
        &postgresql_type_or_json_type_ident_where_upper_camel_case,
        &quote::quote!{
            Self {
                logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }
        },
    );
    quote::quote!{
        #postgresql_type_or_json_type_ident_where_token_stream
        #postgresql_type_or_json_type_ident_where_try_new_error_named_token_stream
        #impl_postgresql_type_or_json_type_ident_where_try_new_token_stream
        #impl_serde_deserialize_for_postgresql_type_or_json_type_ident_where_token_stream
        #impl_crate_postgresql_type_postgresql_type_trait_postgresql_type_self_where_filter_for_postgresql_type_or_json_type_ident_where_token_stream
        #impl_crate_generate_postgresql_json_type_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_postgresql_type_or_json_type_ident_where_token_stream
    }
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
                    " not null"
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
    fn ident_handle(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
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
fn generate_postgresql_type_initialized_by_tokens(input: proc_macro::TokenStream, postgresql_type_initialized_by_tokens: PostgresqlTypeInitializedByTokens) -> proc_macro2::TokenStream {
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
        let postgresql_type_ident_where_token_stream = generate_postgresql_type_or_json_type_where_token_stream(
            &PostgresqlTypeOrJsonType::PostgresqlType,
            &ident_handle,
            &postgresql_type_ident_where_element_upper_camel_case,
            &postgresql_type_ident_where_upper_camel_case
        );

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
            let postgresql_type_self_where_element_upper_camel_case = naming::PostgresqlTypeSelfWhereElementUpperCamelCase;
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
    generated
    // .into()
}
#[proc_macro_derive(PostgresqlTypeInitializedUsingUuidGenerateV4FunctionByPostgresqlTokens)]
pub fn postgresql_type_initialized_using_uuid_generate_v4_function_by_postgresql_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_type_initialized_by_tokens(input, PostgresqlTypeInitializedByTokens::InitializedUsingUuidGenerateV4FunctionByPostgresql).into()
}
#[proc_macro_derive(PostgresqlTypeInitializedUsingDefaultKeywordByPostgresqlTokens)]
pub fn postgresql_type_initialized_using_default_keyword_by_postgresql_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_type_initialized_by_tokens(input, PostgresqlTypeInitializedByTokens::InitializedUsingDefaultKeywordByPostgresql).into()
}
#[proc_macro_derive(PostgresqlTypeInitializedByClientTokens)]
pub fn postgresql_type_initialized_by_client_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_postgresql_type_initialized_by_tokens(input, PostgresqlTypeInitializedByTokens::InitializedByClient).into()
}

enum IsPrimaryKey {
    True,
    False,
}
fn generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
    postgresql_type_nullable_or_not_null: &PostgresqlTypeNullableOrNotNull,
    ident: &dyn quote::ToTokens,
    field_type: &dyn quote::ToTokens,
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
                format!("{} not null {}", #ident_as_crate_create_table_column_query_part_create_table_column_query_part_column_is_primary_key_token_stream, crate::maybe_primary_key(is_primary_key))
            },
            IsPrimaryKey::False => quote::quote!{
                format!("{} not null", #ident_as_crate_create_table_column_query_part_create_table_column_query_part_column_is_primary_key_token_stream)
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
enum ShouldDeriveSchemarsJsonSchema {
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
fn generate_pub_enum_postgresql_type_tokens_where_element_token_stream(
    should_implement_schemars_json_schema: &ShouldDeriveSchemarsJsonSchema,
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
                    &ShouldDeriveSchemarsJsonSchema::True,
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

fn generate_struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream(
    postgresql_type_ident_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_token_stream(
        &format!(
            "struct {postgresql_type_ident_where_element_tokens_upper_camel_case}"
        )
    )
}
fn generate_struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream(
    postgresql_type_ident_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    length: std::primitive::usize,
) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_token_stream(
        &format!(
            "struct {postgresql_type_ident_where_element_tokens_upper_camel_case} with {length} elements"
        )
    )
}
fn generate_serde_deserialize_double_quotes_token_stream(postgresql_type_ident_where_element_tokens_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens, length: std::primitive::usize) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream
) {
    let struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream = generate_struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream(
        postgresql_type_ident_where_element_tokens_upper_camel_case
    );
    let struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream = generate_struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream(
        postgresql_type_ident_where_element_tokens_upper_camel_case,
        length,
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

///////////////////////
fn generate_postgresql_type_tokens_where_element_tokens_token_stream(
    ident: &dyn quote::ToTokens,
    should_where_element_fields_be_public: &ShouldWhereElementFieldsBePublic,
    should_implement_schemars_json_schema: &ShouldDeriveSchemarsJsonSchema,
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
    variants: &std::vec::Vec<&dyn crate::filters::WhereOperatorName>,
    postgresql_type_tokens_where_element_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    variant_type_prefix_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens,
    should_implement_schemars_json_schema: &ShouldDeriveSchemarsJsonSchema,
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
                let postgresql_type_tokens_where_element_filter_upper_camel_case = {
                    let value = format!("{variant_type_prefix_upper_camel_case}{}", quote::quote!{#element_upper_camel_case});
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote!{#element_upper_camel_case(#postgresql_type_tokens_where_element_filter_upper_camel_case)}
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
    variants: &std::vec::Vec<&dyn crate::filters::WhereOperatorName>
) -> proc_macro2::TokenStream {
    let postgresql_type_tokens_where_element_upper_camel_case: &dyn naming::StdFmtDisplayPlusQuoteToTokens = match &is_nullable {
        IsNullable::True => &naming::parameter::PostgresqlTypeStdOptionOptionSelfWhereElementUpperCamelCase::from_tokens(&ident),
        IsNullable::False => &naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&ident)
    };
    generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_handle_token_stream(
        &ident,
        &variants,
        &postgresql_type_tokens_where_element_upper_camel_case,
        &postgresql_type_tokens_where_element_upper_camel_case,
        &ShouldDeriveSchemarsJsonSchema::False,
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

enum WhereOperatorType<'a> {
    Ident(&'a dyn quote::ToTokens),
    FieldType {
        field_type: &'a dyn quote::ToTokens,
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
        //commented just to migrate to new macro type (path wasnt the same to EncodeFormat)
        // let equal_to_encoded_string_representation = crate::filters::EqualToEncodedStringRepresentation;
        // let postgresql_type_tokens_where_element_equal_to_encoded_string_representation_token_stream = equal_to_encoded_string_representation.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        //     &ident,
        //     &is_nullable,
        // );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &length_more_than,
                // &equal_to_encoded_string_representation,
            ]
        );
        quote::quote! {
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_length_more_than_token_stream
            // #postgresql_type_tokens_where_element_equal_to_encoded_string_representation_token_stream
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
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => quote::quote!{SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz},
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => quote::quote!{SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp},
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => quote::quote!{SqlxTypesTimeDateAsPostgresqlDate},
            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => quote::quote!{SqlxTypesBigDecimalAsPostgresqlNumeric},
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
        //commented just for path compatibility with new macro
        // let value_is_contained_within_range = crate::filters::ValueIsContainedWithinRange;
        // let postgresql_type_tokens_where_element_value_is_contained_within_range_token_stream = value_is_contained_within_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
        //     &ident,
        //     &is_nullable,
        //     &range_type_token_stream,
        //     &range_type_should_impl_range_length,
        //     &range_type_default_initialization_token_stream,
        //     &range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream,
        // );
        let contains_another_range = crate::filters::ContainsAnotherRange;
        let postgresql_type_tokens_where_element_contains_another_range_token_stream = contains_another_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let strictly_to_left_of_range = crate::filters::StrictlyToLeftOfRange;
        let postgresql_type_tokens_where_element_strictly_to_left_of_range_token_stream = strictly_to_left_of_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let strictly_to_right_of_range = crate::filters::StrictlyToRightOfRange;
        let postgresql_type_tokens_where_element_strictly_to_right_of_range_token_stream = strictly_to_right_of_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let included_lower_bound = crate::filters::IncludedLowerBound;
        let postgresql_type_tokens_where_element_included_lower_bound_token_stream = included_lower_bound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &range_type_token_stream,
            &range_type_default_initialization_token_stream,
            &range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream,
        );
        let excluded_upper_bound = crate::filters::ExcludedUpperBound;
        let postgresql_type_tokens_where_element_excluded_upper_bound_token_stream = excluded_upper_bound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
            &range_type_token_stream,
            &range_type_default_initialization_token_stream,
            &range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream,
        );
        let greater_than_lower_bound = crate::filters::GreaterThanLowerBound;
        let postgresql_type_tokens_where_element_greater_than_lower_bound_token_stream = greater_than_lower_bound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let overlap_with_range = crate::filters::OverlapWithRange;
        let postgresql_type_tokens_where_element_overlap_with_range_token_stream = overlap_with_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let adjacent_with_range = crate::filters::AdjacentWithRange;
        let postgresql_type_tokens_where_element_adjacent_with_range_token_stream = adjacent_with_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        //todo find out maximum length of range(INT8RANGE, INT4RANGE) in postgresql
        let range_length = crate::filters::RangeLength;
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
                let mut value: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = vec![
                    &equal,
                    // &value_is_contained_within_range,
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
            // #postgresql_type_tokens_where_element_value_is_contained_within_range_token_stream
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
// #[proc_macro_derive(PostgresqlBaseTypeTokensStdNetIpAddr)]
// pub fn postgresql_base_type_tokens_std_net_ip_addr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     generate_postgresql_base_type_tokens(
//         input,
//         &std_net_ip_addr_v4_std_net_ipv4_addr_unspecified_token_stream()
//     )
// }
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
        let bit_vec_position_equal = crate::filters::BitVecPositionEqual;
        let postgresql_type_tokens_where_element_bit_vec_position_equal_token_stream = bit_vec_position_equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
            &ident,
            &is_nullable,
        );
        let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
            is_nullable,
            &ident,
            &vec![
                &equal,
                &bit_vec_position_equal,
            ]
        );
        quote::quote! {
            #postgresql_type_tokens_where_element_equal_token_stream
            #postgresql_type_tokens_where_element_bit_vec_position_equal_token_stream
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

#[proc_macro]
pub fn generate_postgresql_types(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();

    let self_snake_case = naming::SelfSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let acc_snake_case = naming::AccSnakeCase;
    let start_snake_case = naming::StartSnakeCase;
    let end_snake_case = naming::EndSnakeCase;

    let digits_snake_case = naming::DigitsSnakeCase;
    let scale_snake_case = naming::ScaleSnakeCase;
    let year_snake_case = naming::YearSnakeCase;
    let month_snake_case = naming::MonthSnakeCase;
    let day_snake_case = naming::DaySnakeCase;
    let date_snake_case = naming::DateSnakeCase;
    let time_snake_case = naming::TimeSnakeCase;
    let offset_snake_case = naming::OffsetSnakeCase;
    let months_snake_case = naming::MonthsSnakeCase;
    let days_snake_case = naming::DaysSnakeCase;
    let microseconds_snake_case = naming::MicrosecondsSnakeCase;

    let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
    let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
    let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
    let default_try_generate_bind_increments_token_stream = {
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
    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = quote::quote!{
        crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    };
    let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();

    #[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    enum PostgresqlType {
        StdPrimitiveI16AsPostgresqlInt2,
        StdPrimitiveI32AsPostgresqlInt4,
        StdPrimitiveI64AsPostgresqlInt8,
        StdPrimitiveF32AsPostgresqlFloat4,
        StdPrimitiveF64AsPostgresqlFloat8,
        StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql,
        StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql,
        StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql,
        SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
        SqlxTypesDecimalAsPostgresqlNumeric,
        SqlxTypesBigDecimalAsPostgresqlNumeric,
        StdPrimitiveBoolAsPostgresqlBool,
        StdStringStringAsPostgresqlCharN,
        StdStringStringAsPostgresqlVarchar,
        StdStringStringAsPostgresqlText,
        StdVecVecStdPrimitiveU8AsPostgresqlBytea,
        SqlxTypesTimeDateAsPostgresqlDate,
        SqlxTypesChronoNaiveDateAsPostgresqlDate,
        SqlxTypesChronoNaiveTimeAsPostgresqlTime,
        SqlxTypesTimeTimeAsPostgresqlTime,
        SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
        SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
        SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,
        SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,
        SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
        SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
        SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
        SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
        SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
        SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,
        SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
        SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
        SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql,
        SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient,
        SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
        SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
        SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
        SqlxTypesBitVecAsPostgresqlBit,
        SqlxTypesBitVecAsPostgresqlVarbit,
    }
    impl quote::ToTokens for PostgresqlType {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string()
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("failed to parse PostgresqlType to proc_macro2::TokenStream"))
            .to_tokens(tokens)
        }
    }

    let generate_postgresql_type_token_stream = |postgresql_type: PostgresqlType|{
        let field_type = {
            let value = match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => "std::primitive::i16",
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => "std::primitive::i32",
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => "std::primitive::i64",
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => "std::primitive::f32",
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => "std::primitive::f64",
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => "std::primitive::i16",
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => "std::primitive::i32",
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => "std::primitive::i64",
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => "sqlx::postgres::types::PgMoney",
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => "sqlx::types::Decimal",
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => "sqlx::types::BigDecimal",
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => "std::primitive::bool",
                PostgresqlType::StdStringStringAsPostgresqlCharN => "std::string::String",
                PostgresqlType::StdStringStringAsPostgresqlVarchar => "std::string::String",
                PostgresqlType::StdStringStringAsPostgresqlText => "std::string::String",
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => "std::vec::Vec<std::primitive::u8>",
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => "sqlx::types::time::Date",
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => "sqlx::types::chrono::NaiveDate",
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => "sqlx::types::chrono::NaiveTime",
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => "sqlx::types::time::Time",
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => "sqlx::postgres::types::PgInterval",
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => "sqlx::postgres::types::PgRange<std::primitive::i32>",
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => "sqlx::postgres::types::PgRange<std::primitive::i64>",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => "sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => "sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => "sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => "sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => "sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => "sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => "sqlx::postgres::types::PgRange<sqlx::types::time::Date>",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => "sqlx::postgres::types::PgRange<sqlx::types::Decimal>",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => "sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>",
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => "sqlx::types::chrono::NaiveDateTime",
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => "sqlx::types::time::PrimitiveDateTime",
                PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => "sqlx::types::time::OffsetDateTime",
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => "sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>",
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => "sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>",
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => "sqlx::types::uuid::Uuid",
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => "sqlx::types::uuid::Uuid",
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => "sqlx::types::ipnetwork::IpNetwork",
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => "sqlx::types::ipnetwork::IpNetwork",
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => "sqlx::types::mac_address::MacAddress",
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => "sqlx::types::BitVec",
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => "sqlx::types::BitVec",
            };
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonTypeHandle to proc_macro2::TokenStream"))
        };
        let std_option_option_ident_upper_camel_case = naming::parameter::StdOptionOptionSelfUpperCamelCase::from_tokens(&postgresql_type);

        let ident_token_stream = {
            let maybe_derive_serde_serialize_token_stream = {
                let serde_serialize_comma_token_stream = quote::quote!{serde::Serialize,};
                match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => serde_serialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => serde_serialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => serde_serialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => serde_serialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlText => serde_serialize_comma_token_stream,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => serde_serialize_comma_token_stream,//todo maybe impl serialize mannually
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => serde_serialize_comma_token_stream,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => proc_macro2_token_stream_new.clone(),
                }
            };
            let maybe_derive_serde_deserialize_token_stream = {
                let serde_deserialize_comma_token_stream = quote::quote!{serde::Deserialize,};
                match &postgresql_type {
                    PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => serde_deserialize_comma_token_stream,
                    PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => serde_deserialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlCharN => serde_deserialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlVarchar => serde_deserialize_comma_token_stream,
                    PostgresqlType::StdStringStringAsPostgresqlText => serde_deserialize_comma_token_stream,
                    PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => serde_deserialize_comma_token_stream,//impl_serde_deserialize_for_sqlx_types_time_offset_date_time_token_stream
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => serde_deserialize_comma_token_stream,
                    PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => proc_macro2_token_stream_new.clone(),
                    PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => proc_macro2_token_stream_new.clone(),
                }
            };
            quote::quote!{
                #[derive(
                    Debug,
                    Clone,
                    PartialEq,
                    #maybe_derive_serde_serialize_token_stream
                    #maybe_derive_serde_deserialize_token_stream
                )]
                struct #postgresql_type(#field_type);
            }
        };
        let maybe_impl_try_new_token_stream = {
            let sqlx_types_time_date_token_stream = {
                let ident_try_new_error_named_upper_camel_case = naming::parameter::SelfTryNewErrorNamedUpperCamelCase::from_tokens(&postgresql_type);
                let from_calendar_date_upper_camel_case = naming::FromCalendarDateUpperCamelCase;
                let less_than_minimum_postgresql_value_upper_camel_case = naming::LessThanMinimumPostgresqlValueUpperCamelCase;
                quote::quote!{
                    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                    pub enum #ident_try_new_error_named_upper_camel_case {
                        #from_calendar_date_upper_camel_case {
                            #[eo_to_std_string_string_serialize_deserialize]
                            value: std::string::String,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        },
                        #less_than_minimum_postgresql_value_upper_camel_case {
                            #[eo_to_std_string_string_serialize_deserialize]
                            value: std::string::String,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        },
                    }
                    impl #postgresql_type {
                        fn try_new(
                            year: std::primitive::i32,
                            month: time::Month,
                            day: std::primitive::u8,
                        ) -> Result<Self, #ident_try_new_error_named_upper_camel_case> {
                            match sqlx::types::time::Date::from_calendar_date(
                                year,
                                month,
                                day,
                            ) {
                                Ok(value) => {
                                    //postgresql having minimum value "year": -4712, "month": 1, "day": 1. maximum "year": 5874897, "month": 12, "day": 31. but library type does not impl that correctly(in type max is 9999)
                                    let minimum = sqlx::types::time::Date::from_calendar_date(
                                        -4713,
                                        time::Month::December,
                                        31,
                                    ).unwrap();
                                    if minimum > value {
                                        Err(#ident_try_new_error_named_upper_camel_case::#less_than_minimum_postgresql_value_upper_camel_case {
                                            value: format!("{value:?}"),
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        })
                                    }
                                    else {
                                        Ok(Self(value))
                                    }
                                },
                                Err(error) => Err(#ident_try_new_error_named_upper_camel_case::#from_calendar_date_upper_camel_case {
                                    value: format!("{error:?}"),
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                })
                            }
                        }
                    }
                }
            };
            match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlCharN => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlVarchar => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlText => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => sqlx_types_time_date_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),//impl_serde_deserialize_for_sqlx_types_time_offset_date_time_token_stream
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => proc_macro2_token_stream_new.clone(),
            }
        };
        enum ParameterNumber {
            One,
            Two,
            Three,
        }
        impl ParameterNumber {
            fn get_index(&self) -> std::primitive::u8 {
                match &self {
                    Self::One => 0,
                    Self::Two => 1,
                    Self::Three => 2,
                }
            }
            fn get_std_primitive_u8(&self) -> std::primitive::u8 {
                match &self {
                    Self::One => 1,
                    Self::Two => 2,
                    Self::Three => 3,
                }
            }
        }
        let maybe_impl_serde_serialize_token_stream = {
            let ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&postgresql_type);
            let generate_impl_serde_serialize_for_tokens = |content_token_stream: &dyn quote::ToTokens|{
                quote::quote!{
                    const _: () = {
                        #[allow(unused_extern_crates, clippy::useless_attribute)]
                        extern crate serde as _serde;
                        #[automatically_derived]
                        impl _serde::Serialize for #postgresql_type{
                            fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                #content_token_stream
                            }
                        }
                    };
                }
            };

            let generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4 = |value_token_stream: &dyn quote::ToTokens|{
                quote::quote!{_serde::Serializer::serialize_newtype_struct(__serializer, #ident_double_quotes_token_stream, &self.0 #value_token_stream)}
            };
            let generate_serde_state_initialization_token_stream = |parameter_number: ParameterNumber|{
                let parameter_number_token_stream = match parameter_number {
                    ParameterNumber::One => quote::quote!{+ 1 + 1},
                    ParameterNumber::Two => quote::quote!{+ 1 + 1},
                    ParameterNumber::Three => quote::quote!{+ 1 + 1 + 1},
                };
                quote::quote!{
                    let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #ident_double_quotes_token_stream, false as usize #parameter_number_token_stream)?;
                }
            };
            let serde_state_initialization_two_fields_token_stream = generate_serde_state_initialization_token_stream(ParameterNumber::Two);
            let serde_state_initialization_three_fields_token_stream = generate_serde_state_initialization_token_stream(ParameterNumber::Three);

            let generate_serialize_field_token_stream = |field_name: &dyn std::fmt::Display, third_parameter_token_stream: &dyn quote::ToTokens|{
                let field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_name);
                quote::quote!{_serde::ser::SerializeStruct::serialize_field(&mut __serde_state, #field_name_double_quotes_token_stream, #third_parameter_token_stream)?;}
            };

            let serde_ser_serialize_struct_end_token_stream = quote::quote!{_serde::ser::SerializeStruct::end(__serde_state)};
            let serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream = {
                let generate_self_zero_tokens_token_stream = |value_token_stream: &dyn quote::ToTokens|{
                    quote::quote!{&self.0.#value_token_stream}
                };
                let start_serialize_field_token_stream = generate_serialize_field_token_stream(&start_snake_case, &generate_self_zero_tokens_token_stream(&start_snake_case));
                let end_serialize_field_token_stream = generate_serialize_field_token_stream(&end_snake_case, &generate_self_zero_tokens_token_stream(&end_snake_case));
                quote::quote!{
                    #serde_state_initialization_two_fields_token_stream
                    #start_serialize_field_token_stream
                    #end_serialize_field_token_stream
                    #serde_ser_serialize_struct_end_token_stream
                }
            };
            let generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25 = |
                type_token_stream: &dyn quote::ToTokens,
                is_need_to_be_cloned: std::primitive::bool,
            |{
                let maybe_clone_token_stream = if is_need_to_be_cloned {
                    quote::quote!{.clone()}
                }
                else {
                    proc_macro2::TokenStream::new()
                };
                let generate_self_zero_match_tokens_token_stream = |value_token_stream: &dyn quote::ToTokens|{
                    quote::quote!{&match self.0.#value_token_stream #maybe_clone_token_stream {
                        std::collections::Bound::Included(value) => std::collections::Bound::Included(#type_token_stream(value)),
                        std::collections::Bound::Excluded(value) => std::collections::Bound::Excluded(#type_token_stream(value)),
                        std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                    }}
                };
                let start_serialize_field_token_stream = generate_serialize_field_token_stream(
                    &start_snake_case,
                    &generate_self_zero_match_tokens_token_stream(&start_snake_case)
                );
                let end_serialize_field_token_stream = generate_serialize_field_token_stream(
                    &end_snake_case,
                    &generate_self_zero_match_tokens_token_stream(&end_snake_case)
                );
                quote::quote!{
                    #serde_state_initialization_two_fields_token_stream
                    #start_serialize_field_token_stream
                    #end_serialize_field_token_stream
                    #serde_ser_serialize_struct_end_token_stream
                }
            };

            let impl_serde_serialize_for_sqlx_postgres_types_pg_money_token_stream = generate_impl_serde_serialize_for_tokens(
                &generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote!{.0})
            );
            let impl_serde_serialize_for_sqlx_types_big_decimal_token_stream = generate_impl_serde_serialize_for_tokens(&{
                let digits_serialize_field_token_stream = generate_serialize_field_token_stream(&naming::DigitsSnakeCase, &quote::quote!{&crate::postgresql_type::postgresql_base_type::NumBigintBigInt(bigint)});
                let scale_serialize_field_token_stream = generate_serialize_field_token_stream(&naming::ScaleSnakeCase, &quote::quote!{&exponent});
                quote::quote!{
                    let (bigint, exponent) = self.0.clone().into_bigint_and_exponent();
                    #serde_state_initialization_two_fields_token_stream
                    #digits_serialize_field_token_stream
                    #scale_serialize_field_token_stream
                    #serde_ser_serialize_struct_end_token_stream
                }
            });
            let impl_serde_serialize_for_sqlx_types_time_date_token_stream = generate_impl_serde_serialize_for_tokens(&{
                let year_snake_case = naming::YearSnakeCase;
                let month_snake_case = naming::MonthSnakeCase;
                let day_snake_case = naming::DaySnakeCase;
                let generate_self_zero_tokens_token_stream = |value_token_stream: &dyn quote::ToTokens|{
                    quote::quote!{&self.0.#value_token_stream()}
                };
                let year_serialize_field_token_stream = generate_serialize_field_token_stream(&year_snake_case, &generate_self_zero_tokens_token_stream(&year_snake_case));
                let month_serialize_field_token_stream = generate_serialize_field_token_stream(&month_snake_case, &generate_self_zero_tokens_token_stream(&month_snake_case));
                let day_serialize_field_token_stream = generate_serialize_field_token_stream(&day_snake_case, &generate_self_zero_tokens_token_stream(&day_snake_case));
                quote::quote!{
                    #serde_state_initialization_three_fields_token_stream
                    #year_serialize_field_token_stream
                    #month_serialize_field_token_stream
                    #day_serialize_field_token_stream
                    #serde_ser_serialize_struct_end_token_stream
                }
            });
            let impl_serde_serialize_for_sqlx_postgres_types_pg_interval_token_stream = generate_impl_serde_serialize_for_tokens(&{
                let months_serialize_field_token_stream = generate_serialize_field_token_stream(&naming::MonthsSnakeCase, &quote::quote!{&self.0.months});
                let days_serialize_field_token_stream = generate_serialize_field_token_stream(&naming::DaysSnakeCase, &quote::quote!{&self.0.days});
                let microseconds_serialize_field_token_stream = generate_serialize_field_token_stream(&naming::MicrosecondsSnakeCase, &quote::quote!{&self.0.microseconds});
                quote::quote!{
                    #serde_state_initialization_three_fields_token_stream
                    #months_serialize_field_token_stream
                    #days_serialize_field_token_stream
                    #microseconds_serialize_field_token_stream
                    #serde_ser_serialize_struct_end_token_stream
                }
            });
            let impl_serde_serialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream = generate_impl_serde_serialize_for_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream);
            let impl_serde_serialize_for_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream = generate_impl_serde_serialize_for_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream);
            let impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream = generate_impl_serde_serialize_for_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream);
            let impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream = generate_impl_serde_serialize_for_tokens(
                &generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(
                    &quote::quote!{SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp},//todo how to reuse naming?
                    false,
                )
            );
            let impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_impl_serde_serialize_for_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream);
            let impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_impl_serde_serialize_for_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream);
            let impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream = generate_impl_serde_serialize_for_tokens(
                &generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(
                    &quote::quote!{SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz},//todo how to reuse naming?
                    false,
                )
            );
            let impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream = generate_impl_serde_serialize_for_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream);
            let impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream = generate_impl_serde_serialize_for_tokens(
                &generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(
                    &quote::quote!{SqlxTypesTimeDateAsPostgresqlDate},//todo how to reuse naming?
                    false,
                )
            );
            let impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream = generate_impl_serde_serialize_for_tokens(&serde_serialize_content_e5bb5640_d9fe_4ed3_9862_6943f8efee90_token_stream);
            let impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream = generate_impl_serde_serialize_for_tokens(
                &generate_serde_serialize_content_b1e2ccdf_3707_4f59_b809_20c0f087ab25(
                    &quote::quote!{SqlxTypesBigDecimalAsPostgresqlNumeric},//todo how to reuse naming?
                    true,
                )
            );
            let impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream = generate_impl_serde_serialize_for_tokens(
                &generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote!{.to_string()})
            );
            let impl_serde_serialize_for_sqlx_types_mac_address_mac_address_token_stream = generate_impl_serde_serialize_for_tokens(
                &generate_serde_serialize_content_b5af560e_5f3f_4f23_9286_c72dd986a1b4(&quote::quote!{.bytes()})
            );
            let impl_serde_serialize_for_sqlx_types_bit_vec_token_stream = generate_impl_serde_serialize_for_tokens(&quote::quote!{
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    #ident_double_quotes_token_stream,
                    &self.0.iter().collect::<std::vec::Vec<std::primitive::bool>>(),
                )
            });
            match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => impl_serde_serialize_for_sqlx_postgres_types_pg_money_token_stream,
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => impl_serde_serialize_for_sqlx_types_big_decimal_token_stream,
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlCharN => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlVarchar => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlText => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => impl_serde_serialize_for_sqlx_types_time_date_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => impl_serde_serialize_for_sqlx_postgres_types_pg_interval_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => impl_serde_serialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => impl_serde_serialize_for_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => impl_serde_serialize_for_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => impl_serde_serialize_for_sqlx_types_uuid_uuid_token_stream,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => impl_serde_serialize_for_sqlx_types_mac_address_mac_address_token_stream,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => impl_serde_serialize_for_sqlx_types_bit_vec_token_stream,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => impl_serde_serialize_for_sqlx_types_bit_vec_token_stream,
            }
        };
        let maybe_impl_serde_deserialize_token_stream = {
            let ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&postgresql_type);
            let struct_ident_double_quotes_token_stream = generate_struct_postgresql_type_ident_where_element_tokens_double_quotes_token_stream(&postgresql_type);
            let struct_ident_with_one_element_double_quotes_token_stream = generate_struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream(
                &postgresql_type,
                1,
            );
            let struct_ident_with_two_elements_double_quotes_token_stream = generate_struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream(
                &postgresql_type,
                2,
            );
            let struct_ident_with_three_elements_double_quotes_token_stream = generate_struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream(
                &postgresql_type,
                3,
            );
            let ident_visitor_upper_camel_case = naming::parameter::SelfVisitorUpperCamelCase::from_tokens(&postgresql_type);

            let struct_visitor_token_stream = quote::quote! {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: serde::__private::PhantomData<#postgresql_type>,
                    lifetime: serde::__private::PhantomData<&'de ()>,
                }
            };

            let digits_scale_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 2] = [&digits_snake_case, &scale_snake_case];
            let year_month_day_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 3] = [&year_snake_case, &month_snake_case, &day_snake_case];
            let start_end_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 2] = [&start_snake_case, &end_snake_case];
            let date_time_offset_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 3] = [&date_snake_case, &time_snake_case, &offset_snake_case];
            let months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array: [&dyn naming::StdFmtDisplayPlusQuoteToTokens; 3] = [&months_snake_case, &days_snake_case, &microseconds_snake_case];

            let (
                serde_deserializer_deserialize_struct_visitor_token_stream,
                serde_deserializer_deserialize_struct_ident_visitor_token_stream
            ) = {
                let generate_serde_deserializer_deserialize_struct_visitor_token_stream = |content_token_stream: &dyn quote::ToTokens|{
                    quote::quote!{
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            #ident_double_quotes_token_stream,
                            FIELDS,
                            #content_token_stream
                        )
                    }
                };
                (
                    generate_serde_deserializer_deserialize_struct_visitor_token_stream(&quote::quote!{
                        __Visitor {
                            marker: _serde::__private::PhantomData::<#postgresql_type>,
                            lifetime: _serde::__private::PhantomData,
                        }
                    }),
                    generate_serde_deserializer_deserialize_struct_visitor_token_stream(&ident_visitor_upper_camel_case)
                )
            };

            let serde_deserializer_deserialize_newtype_struct_token_stream = quote::quote! {
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    #ident_double_quotes_token_stream,
                    __Visitor {
                        marker: serde::__private::PhantomData::<#postgresql_type>,
                        lifetime: serde::__private::PhantomData,
                    },
                )
            };

            let generate_impl_serde_deserialize_for_tokens_token_stream = |content_token_stream: &dyn quote::ToTokens|{
                quote::quote!{
                    const _: () = {
                        #[allow(unused_extern_crates, clippy::useless_attribute)]
                        extern crate serde as _serde;
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for #postgresql_type {
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                #content_token_stream
                            }
                        }
                    };
                }
            };

            let parameter_number_one = ParameterNumber::One;
            let parameter_number_two = ParameterNumber::Two;
            let parameter_number_three = ParameterNumber::Three;

            let (
                enum_field_two_token_stream,
                enum_field_three_token_stream,
            ) = {
                let generate_enum_field_token_stream = |parameter_number: &ParameterNumber|{
                    let fields_token_stream = {
                        let number = parameter_number.get_std_primitive_u8();
                        let value = (0..number).collect::<std::vec::Vec<std::primitive::u8>>();
                        let fields_token_stream = value.iter().map(|element|{
                            format!("__{}{element}", naming::FieldSnakeCase)
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap()
                        });
                        quote::quote!{#(#fields_token_stream),*}
                    };
                    quote::quote!{
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            #fields_token_stream,
                            __ignore,
                        }
                    }
                };
                (
                    generate_enum_field_token_stream(&parameter_number_two),
                    generate_enum_field_token_stream(&parameter_number_three),
                )
            };

            let (
                fn_expecting_struct_ident_double_quotes_token_stream,
                fn_expecting_field_identifier_token_stream,
                fn_expecting_months_or_days_or_microseconds_token_stream,
                fn_expecting_start_or_end_token_stream,
            ) = {
                let generate_fn_expecting_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote!{
                    fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                        serde::__private::Formatter::write_str(__formatter, #content_token_stream)
                    }
                };
                (
                    generate_fn_expecting_token_stream(&struct_ident_double_quotes_token_stream),
                    generate_fn_expecting_token_stream(&quote::quote!{"field identifier"}),
                    generate_fn_expecting_token_stream(&quote::quote!{"`months` or `days` or `microseconds`"}),
                    generate_fn_expecting_token_stream(&quote::quote!{"`start` or `end`"}),
                )
            };

            let generate_serde_private_ok_token_stream = |content_token_stream: &dyn quote::ToTokens|{quote::quote!{serde::__private::Ok(#content_token_stream)}};
            let generate_serde_private_ok_postgresql_type_token_stream = |content_token_stream: &dyn quote::ToTokens|{generate_serde_private_ok_token_stream(&quote::quote!{#postgresql_type(#content_token_stream)})};

            let (
                fn_visit_newtype_struct_pg_money_token_stream,
                fn_visit_newtype_struct_uuid_token_stream,
                fn_visit_newtype_struct_mac_address_token_stream,
                fn_visit_newtype_struct_bit_vec_token_stream,
            ) = {
                let generate_fn_visit_newtype_struct_token_stream = |type_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens|{
                    let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(content_token_stream);
                    quote::quote!{
                        #[inline]
                        fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
                        where
                            __E: serde::Deserializer<'de>,
                        {
                            let __field0 = <#type_token_stream as serde::Deserialize>::deserialize(__e)?;
                            #serde_private_ok_postgresql_type_token_stream
                        }
                    }
                };
                (
                    generate_fn_visit_newtype_struct_token_stream(
                        &token_patterns::StdPrimitiveI64,
                        &quote::quote!{sqlx::postgres::types::PgMoney(__field0)}
                    ),
                    generate_fn_visit_newtype_struct_token_stream(
                        &token_patterns::StdStringString,
                        &quote::quote!{match sqlx::types::uuid::Uuid::try_parse(&__field0) {
                            Ok(value) => value,
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        }}
                    ),
                    generate_fn_visit_newtype_struct_token_stream(
                        &quote::quote!{[std::primitive::u8; 6]},
                        &quote::quote!{sqlx::types::mac_address::MacAddress::new(__field0)}
                    ),
                    generate_fn_visit_newtype_struct_token_stream(
                        &quote::quote!{std::vec::Vec<std::primitive::bool>},
                        &quote::quote!{{
                            let mut bit_vec = sqlx::types::BitVec::from_elem(__field0.len(), false);
                            __field0.into_iter().enumerate().for_each(|(index, element)|{
                                bit_vec.set(index, element);
                            });
                            bit_vec
                        }}
                    )
                )
            };

            let generate_fn_visit_seq_token_stream = |content_token_stream: &dyn quote::ToTokens|{
                quote::quote!{
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: serde::de::SeqAccess<'de>,
                    {
                        #content_token_stream
                    }
                }
            };
            let generate_fields_serde_de_seq_access_next_element_initialization_token_stream = |vec_token_stream: &[&dyn quote::ToTokens]|{
                let error_message_token_stream = generate_struct_postgresql_type_ident_where_element_tokens_with_number_elements_double_quotes_token_stream(
                    &postgresql_type,
                    vec_token_stream.len(),
                );
                let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)|{
                    //todo reuse
                    let field_index_name_token_stream = format!("__{}{index}", naming::FieldSnakeCase)
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap();
                    let index_usize_token_stream = format!("{index}usize")
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap();
                    quote::quote!{
                        let #field_index_name_token_stream = match serde::de::SeqAccess::next_element::<#element>(&mut __seq)? {
                            serde::__private::Some(__value) => __value,
                            serde::__private::None => {
                                return serde::__private::Err(serde::de::Error::invalid_length(#index_usize_token_stream, &#error_message_token_stream));
                            }
                        };
                    }
                });
                quote::quote!{#(#fields_initialization_token_stream)*}
            };

            let fn_visit_seq_pg_money_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                    &token_patterns::StdPrimitiveI64,
                ]);
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote!{sqlx::postgres::types::PgMoney(__field0)});
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_types_big_decimal_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                    &quote::quote!{crate::postgresql_type::postgresql_base_type::NumBigintBigInt},
                    &token_patterns::StdPrimitiveI64,
                ]);
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote!{sqlx::types::BigDecimal::new(__field0.0, __field1)});
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_types_time_date_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                    &token_patterns::StdPrimitiveI32,
                    &quote::quote!{time::Month},
                    &token_patterns::StdPrimitiveU8,
                ]);
                quote::quote!{
                    #fields_initialization_token_stream
                    match #postgresql_type::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                    }
                }
            });

            let (
                seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream,
                seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream,
                seq_next_element_ok_or_else_serde_de_error_invalid_length_two_token_stream,
            ) = {
                let generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream = |parameter_number: &ParameterNumber|{
                    let index_token_stream = match &parameter_number {
                        ParameterNumber::One => quote::quote!{0},
                        ParameterNumber::Two => quote::quote!{1},
                        ParameterNumber::Three => quote::quote!{2},
                    };
                    quote::quote!{__seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(#index_token_stream, &self))?;}
                };
                (
                    generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream(&ParameterNumber::One),
                    generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream(&ParameterNumber::Two),
                    generate_seq_next_element_ok_or_else_serde_de_error_invalid_length_index_token_stream(&ParameterNumber::Three),
                )
            };
            let fn_visit_seq_sqlx_postgres_types_pg_interval_token_stream = generate_fn_visit_seq_token_stream(&{
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote!{sqlx::postgres::types::PgInterval { months, days, microseconds }});
                //todo
                quote::quote!{
                    let months = #seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream
                    let days = #seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream
                    let microseconds = #seq_next_element_ok_or_else_serde_de_error_invalid_length_two_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let sqlx_postgres_types_pg_range_start_end_token_stream = quote::quote!{sqlx::postgres::types::PgRange { #start_snake_case: __field0, #end_snake_case: __field1 }};
            let sqlx_postgres_types_pg_range_bound_start_end_token_stream = quote::quote!{sqlx::postgres::types::PgRange {
                #start_snake_case: match __field0 {
                    std::collections::Bound::Included(value) => std::collections::Bound::Included(value.0),
                    std::collections::Bound::Excluded(value) => std::collections::Bound::Excluded(value.0),
                    std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                },
                #end_snake_case: match __field1 {
                    std::collections::Bound::Included(value) => std::collections::Bound::Included(value.0),
                    std::collections::Bound::Excluded(value) => std::collections::Bound::Excluded(value.0),
                    std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                },
            }};
            let fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream = generate_fn_visit_seq_token_stream(&{
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote!{
                    let __field0 = #seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream
                    let __field1 = #seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream = generate_fn_visit_seq_token_stream(&{
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote!{
                    let __field0 = #seq_next_element_ok_or_else_serde_de_error_invalid_length_zero_token_stream
                    let __field1 = #seq_next_element_ok_or_else_serde_de_error_invalid_length_one_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream = quote::quote!{std::collections::Bound<sqlx::types::chrono::NaiveDateTime>};
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                        &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream,
                        &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream,
                    ])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream = quote::quote!{std::collections::Bound<SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp>};
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                        &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                        &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                    ])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = quote::quote!{std::collections::Bound<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>};
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                        &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                    ])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = quote::quote!{std::collections::Bound<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>};
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                        &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                        &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                    ])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_token_stream = quote::quote!{std::collections::Bound<SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz>};
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                        &std_collections_bound_sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_token_stream,
                        &std_collections_bound_sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_token_stream,
                    ])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_chrono_naive_date_token_stream = quote::quote!{std::collections::Bound<sqlx::types::chrono::NaiveDate>};
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                        &std_collections_bound_sqlx_types_chrono_naive_date_token_stream,
                        &std_collections_bound_sqlx_types_chrono_naive_date_token_stream,
                    ])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream = quote::quote!{std::collections::Bound<SqlxTypesTimeDateAsPostgresqlDate>};
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                        &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream,
                        &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream,
                    ])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let token_stream = quote::quote!{std::collections::Bound<sqlx::types::Decimal>};
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                        &token_stream,
                        &token_stream,
                    ])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = {
                    let token_stream = quote::quote!{std::collections::Bound<SqlxTypesBigDecimalAsPostgresqlNumeric>};
                    generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                        &token_stream,
                        &token_stream,
                    ])
                };
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_types_time_offset_date_time_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                    &quote::quote!{sqlx::types::time::Date},
                    &quote::quote!{sqlx::types::time::Time},
                    &quote::quote!{sqlx::types::time::UtcOffset},
                ]);
                quote::quote!{
                    #fields_initialization_token_stream
                    _serde::__private::Ok(#postgresql_type {
                        date: __field0,
                        time: __field1,
                        offset: __field2,
                    })
                }
            });
            let fn_visit_seq_sqlx_types_uuid_uuid_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                    &token_patterns::StdStringString,
                ]);
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote!{
                    match sqlx::types::uuid::Uuid::try_parse(&__field0) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(serde::de::Error::custom(error));
                        }
                    }
                });
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_types_mac_address_mac_address_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                    &quote::quote!{[std::primitive::u8; 6]},
                ]);
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote!{
                    sqlx::types::mac_address::MacAddress::new(__field0)
                });
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });
            let fn_visit_seq_sqlx_types_bit_vec_token_stream = generate_fn_visit_seq_token_stream(&{
                let fields_initialization_token_stream = generate_fields_serde_de_seq_access_next_element_initialization_token_stream(&[
                    &quote::quote!{std::vec::Vec<std::primitive::bool>},
                ]);
                let serde_private_ok_postgresql_type_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote!{
                    {
                        let mut bit_vec = sqlx::types::BitVec::from_elem(__field0.len(), false);
                        __field0.into_iter().enumerate().for_each(|(index, element)|{
                            bit_vec.set(index, element);
                        });
                        bit_vec
                    }
                });
                quote::quote!{
                    #fields_initialization_token_stream
                    #serde_private_ok_postgresql_type_token_stream
                }
            });

            let (
                fn_visit_u64_one_token_stream,
                fn_visit_u64_two_token_stream,
                fn_visit_u64_three_token_stream,
            ) = {
                let generate_fn_visit_u64_token_stream = |parameter_number: &ParameterNumber|{
                    let fields_token_stream = {
                        (0..parameter_number.get_std_primitive_u8())
                        .collect::<std::vec::Vec<std::primitive::u8>>()
                        .into_iter().map(|element|{
                            let index_variant_token_stream = format!("{element}u64")
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap();
                            let field_index_token_stream = format!("__{}{element}", naming::FieldSnakeCase)
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap();
                            quote::quote!{#index_variant_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)}
                        })
                    };
                    quote::quote!{
                        fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
                        where
                            __E: serde::de::Error,
                        {
                            match __value {
                                #(#fields_token_stream),*,
                                _ => serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                };
                (
                    generate_fn_visit_u64_token_stream(&ParameterNumber::One),
                    generate_fn_visit_u64_token_stream(&ParameterNumber::Two),
                    generate_fn_visit_u64_token_stream(&ParameterNumber::Three),
                )
            };
            let (
                fn_visit_str_digits_scale_token_stream,
                fn_visit_str_year_month_day_token_stream,
                fn_visit_str_start_end_token_stream,
                fn_visit_str_date_time_offset_token_stream
            ) = {
                let generate_fn_visit_str_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]|{
                    let fields_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)|{
                        //todo reuse
                        let element_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element);
                        let field_index_name_token_stream = format!("__{}{index}", naming::FieldSnakeCase)
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap();
                        quote::quote!{
                            #element_double_quotes_token_stream => _serde::__private::Ok(__Field::#field_index_name_token_stream)
                        }
                    });
                    quote::quote!{
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                #(#fields_token_stream),*,
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                };
                (
                    generate_fn_visit_str_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                    generate_fn_visit_str_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                    generate_fn_visit_str_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                    generate_fn_visit_str_token_stream(&date_time_offset_std_fmt_display_plus_quote_to_tokens_array)
                )
            };

            let (
                fn_visit_bytes_digits_scale_token_stream,
                fn_visit_bytes_year_month_day_token_stream,
                fn_visit_bytes_start_end_token_stream,
                fn_visit_bytes_date_time_offset_token_stream
            ) = {
                let generate_fn_visit_bytes_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]|{
                    let fields_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)|{
                        //todo reuse
                        let b_element_double_quotes_token_stream = format!("b{}", generate_quotes::double_quotes_stringified(&element))
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap();
                        let field_index_name_token_stream = format!("__{}{index}", naming::FieldSnakeCase)
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap();
                        quote::quote!{
                            #b_element_double_quotes_token_stream => serde::__private::Ok(__Field::#field_index_name_token_stream)
                        }
                    });
                    quote::quote!{
                        fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
                        where
                            __E: serde::de::Error,
                        {
                            match __value {
                                #(#fields_token_stream),*,
                                _ => serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                };
                (
                    generate_fn_visit_bytes_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                    generate_fn_visit_bytes_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                    generate_fn_visit_bytes_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                    generate_fn_visit_bytes_token_stream(&date_time_offset_std_fmt_display_plus_quote_to_tokens_array)
                )
            };

            let impl_serde_deserialize_for_field_token_stream = quote::quote!{
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
            };

            let (
                std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream,
                std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                std_collections_bound_sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_token_stream,
                std_collections_bound_sqlx_types_chrono_naive_date_token_stream,
                std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream,
                std_collections_bound_sqlx_types_decimal_token_stream,
                std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream
            ) = {
                let generate_std_collections_bound_token_stream = |type_token_stream: &dyn quote::ToTokens|{
                    quote::quote!{std::collections::Bound<#type_token_stream>}
                };
                (
                    generate_std_collections_bound_token_stream(&quote::quote!{sqlx::types::chrono::NaiveDateTime}),
                    generate_std_collections_bound_token_stream(&quote::quote!{SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp}),
                    generate_std_collections_bound_token_stream(&quote::quote!{sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>}),
                    generate_std_collections_bound_token_stream(&quote::quote!{sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>}),
                    generate_std_collections_bound_token_stream(&quote::quote!{SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz}),
                    generate_std_collections_bound_token_stream(&quote::quote!{sqlx::types::chrono::NaiveDate}),
                    generate_std_collections_bound_token_stream(&quote::quote!{SqlxTypesTimeDateAsPostgresqlDate}),
                    generate_std_collections_bound_token_stream(&quote::quote!{sqlx::types::Decimal}),
                    generate_std_collections_bound_token_stream(&quote::quote!{SqlxTypesBigDecimalAsPostgresqlNumeric})
                )
            };

            let (
                fn_visit_map_sqlx_types_big_decimal_token_stream,
                fn_visit_map_sqlx_types_time_date_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                fn_visit_map_sqlx_types_time_offset_date_time_token_stream,
            ) = {
                let generate_fn_visit_map_token_stream = |
                    field_option_none_initialization_token_stream: &dyn quote::ToTokens,
                    while_some_next_key_field_token_stream: &dyn quote::ToTokens,
                    match_field_initialization_token_stream: &dyn quote::ToTokens,
                    serde_private_ok_token_stream: &dyn quote::ToTokens,
                |{
                    quote::quote!{
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: serde::de::MapAccess<'de>,
                        {
                            #field_option_none_initialization_token_stream
                            #while_some_next_key_field_token_stream
                            #match_field_initialization_token_stream
                            #serde_private_ok_token_stream
                        }
                    }
                };

                let (
                    field_option_none_initialization_sqlx_types_big_decimal_token_stream,
                    field_option_none_initialization_sqlx_types_time_date_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                    field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                    field_option_none_initialization_sqlx_types_time_offset_date_time_token_stream
                ) = {
                    let generate_field_option_none_initialization_token_stream = |vec_token_stream: &[&dyn quote::ToTokens]|{
                        let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)|{
                            //todo reuse
                            let field_index_name_token_stream = format!("__{}{index}", naming::FieldSnakeCase)
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap();
                            quote::quote!{
                                let mut #field_index_name_token_stream: serde::__private::Option<#element> = serde::__private::None;
                            }
                        });
                        quote::quote!{#(#fields_initialization_token_stream)*}
                    };
                    (
                        generate_field_option_none_initialization_token_stream(&[
                            &quote::quote!{crate::postgresql_type::postgresql_base_type::NumBigintBigInt},
                            &token_patterns::StdPrimitiveI64,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &token_patterns::StdPrimitiveI32,
                            &quote::quote!{time::Month},
                            &token_patterns::StdPrimitiveU8,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream,
                            &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                            &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                            &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                            &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_token_stream,
                            &std_collections_bound_sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_token_stream,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_chrono_naive_date_token_stream,
                            &std_collections_bound_sqlx_types_chrono_naive_date_token_stream,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream,
                            &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_decimal_token_stream,
                            &std_collections_bound_sqlx_types_decimal_token_stream,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream,
                            &std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &quote::quote!{sqlx::types::time::Date},
                            &quote::quote!{sqlx::types::time::Time},
                            &quote::quote!{sqlx::types::time::UtcOffset},
                        ])
                    )
                };

                let (
                    while_some_next_key_field_sqlx_types_big_decimal_token_stream,
                    while_some_next_key_field_sqlx_types_time_date_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                    while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                    while_some_next_key_field_sqlx_types_time_offset_date_time_token_stream
                ) = {
                    let generate_while_some_next_key_field_token_stream = |vec_token_stream: &[(&dyn std::fmt::Display, &dyn quote::ToTokens)]|{
                        let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)|{
                            let field_name_double_quotes_token_stream = generate_quotes::double_quotes_stringified(&element.0);
                            let field_type_token_stream = &element.1;
                            //todo reuse
                            let field_index_name_token_stream = format!("__{}{index}", naming::FieldSnakeCase)
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap();
                            quote::quote!{
                                __Field::#field_index_name_token_stream => {
                                    if serde::__private::Option::is_some(&#field_index_name_token_stream) {
                                        return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field(#field_name_double_quotes_token_stream));
                                    }
                                    #field_index_name_token_stream = serde::__private::Some(serde::de::MapAccess::next_value::<#field_type_token_stream>(&mut __map)?);
                                }
                            }
                        });
                        quote::quote!{
                            while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                match __key {
                                    #(#fields_initialization_token_stream)*
                                    _ => {
                                        let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                                    }
                                }
                            }
                        }
                    };
                    (
                        generate_while_some_next_key_field_token_stream(&[
                            (&digits_snake_case, &quote::quote!{crate::postgresql_type::postgresql_base_type::NumBigintBigInt}),
                            (&scale_snake_case, &token_patterns::StdPrimitiveI64)
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&year_snake_case, &token_patterns::StdPrimitiveI32),
                            (&month_snake_case, &quote::quote!{time::Month}),
                            (&day_snake_case, &token_patterns::StdPrimitiveU8)
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_time_token_stream)
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_time_primitive_date_time_as_postgresql_timestamp_token_stream)
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream)
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream)
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_token_stream)
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_chrono_naive_date_token_stream)
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_time_date_as_postgresql_date_token_stream)
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_decimal_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_decimal_token_stream)
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&start_snake_case, &std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream),
                            (&end_snake_case, &std_collections_bound_sqlx_types_big_decimal_as_postgresql_numeric_token_stream)
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            (&date_snake_case, &quote::quote!{sqlx::types::time::Date}),
                            (&time_snake_case, &quote::quote!{sqlx::types::time::Time}),
                            (&offset_snake_case, &quote::quote!{sqlx::types::time::UtcOffset}),
                        ])
                    )
                };

                let (
                    match_field_initialization_sqlx_types_big_decimal_token_stream,
                    match_field_initialization_sqlx_types_time_date_token_stream,
                    match_field_initialization_start_end_token_stream,
                    match_field_initialization_sqlx_types_time_offset_date_time_token_stream
                ) = {
                    let generate_match_field_initialization_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]|{
                        let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)|{
                            let field_name_double_quotes_token_stream = generate_quotes::double_quotes_stringified(&element);
                            //todo reuse
                            let field_index_name_token_stream = format!("__{}{index}", naming::FieldSnakeCase)
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap();
                            quote::quote!{
                                let #field_index_name_token_stream = match #field_index_name_token_stream {
                                    serde::__private::Some(#field_index_name_token_stream) => #field_index_name_token_stream,
                                    serde::__private::None => serde::__private::de::missing_field(#field_name_double_quotes_token_stream)?,
                                };
                            }
                        });
                        quote::quote!{#(#fields_initialization_token_stream)*}
                    };
                    (
                        generate_match_field_initialization_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                        generate_match_field_initialization_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                        generate_match_field_initialization_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                        generate_match_field_initialization_token_stream(&date_time_offset_std_fmt_display_plus_quote_to_tokens_array)
                    )
                };

                let serde_private_ok_postgresql_type_sqlx_types_big_decimal_new_field0_field1_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&quote::quote!{sqlx::types::BigDecimal::new(__field0.0, __field1)});
                let match_postgresql_type_try_new_field0_field1_field2_token_stream = quote::quote!{
                    match #postgresql_type::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                    }
                };
                let serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_start_end_token_stream);
                let serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&sqlx_postgres_types_pg_range_bound_start_end_token_stream);
                let serde_private_ok_postgresql_type_date_time_offset_token_stream = generate_serde_private_ok_token_stream(&quote::quote!{
                    #postgresql_type {
                        date: __field0,
                        time: __field1,
                        offset: __field2,
                    }
                });

                (
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_types_big_decimal_token_stream,
                        &while_some_next_key_field_sqlx_types_big_decimal_token_stream,
                        &match_field_initialization_sqlx_types_big_decimal_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_types_big_decimal_new_field0_field1_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_types_time_date_token_stream,
                        &while_some_next_key_field_sqlx_types_time_date_token_stream,
                        &match_field_initialization_sqlx_types_time_date_token_stream,
                        &match_postgresql_type_try_new_field0_field1_field2_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                        &while_some_next_key_field_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &serde_private_ok_postgresql_type_sqlx_postgres_types_pg_range_bound_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_sqlx_types_time_offset_date_time_token_stream,
                        &while_some_next_key_field_sqlx_types_time_offset_date_time_token_stream,
                        &match_field_initialization_sqlx_types_time_offset_date_time_token_stream,
                        &serde_private_ok_postgresql_type_date_time_offset_token_stream,
                    )
                )
            };

            let (
                fn_visit_map_sqlx_postgres_types_pg_interval_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
            ) = {
                let generate_fn_visit_map_token_stream = |
                    field_option_none_initialization_token_stream: &dyn quote::ToTokens,
                    while_some_next_key_field_token_stream: &dyn quote::ToTokens,
                    match_field_initialization_token_stream: &dyn quote::ToTokens,
                    serde_private_ok_token_stream: &dyn quote::ToTokens,
                |{
                    let serde_private_ok_token_stream = generate_serde_private_ok_postgresql_type_token_stream(&serde_private_ok_token_stream);
                    quote::quote!{
                        #[inline]
                        fn visit_map<V>(self, mut map: V) -> Result<#postgresql_type, V::Error>
                        where
                            V: serde::de::MapAccess<'de>,
                        {
                            #field_option_none_initialization_token_stream
                            #while_some_next_key_field_token_stream
                            #match_field_initialization_token_stream
                            #serde_private_ok_token_stream
                        }
                    }
                };
                
                let months_snake_case = naming::MonthsSnakeCase;
                let days_snake_case = naming::DaysSnakeCase;
                let microseconds_snake_case = naming::MicrosecondsSnakeCase;

                let (
                    field_option_none_initialization_months_days_microseconds_token_stream,
                    field_option_none_initialization_start_end_token_stream
                ) = {
                    let generate_field_option_none_initialization_token_stream = |vec_token_stream: &[&dyn quote::ToTokens]|{
                        let fields_initialization_token_stream = vec_token_stream.iter().map(|element|{
                            quote::quote!{
                                let mut #element = None;
                            }
                        });
                        quote::quote!{#(#fields_initialization_token_stream)*}
                    };
                    (
                        generate_field_option_none_initialization_token_stream(&[
                            &months_snake_case,
                            &days_snake_case,
                            &microseconds_snake_case,
                        ]),
                        generate_field_option_none_initialization_token_stream(&[
                            &start_snake_case,
                            &end_snake_case,
                        ])
                    )
                };

                let (
                    while_some_next_key_field_months_days_microseconds_token_stream,
                    while_some_next_key_field_start_end_token_stream,
                ) = {
                    let generate_while_some_next_key_field_token_stream = |vec_token_stream: &[(&dyn naming::StdFmtDisplayPlusQuoteToTokens)]|{
                        let fields_initialization_token_stream = vec_token_stream.iter().map(|element|{
                            let field_name_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element);
                            let element_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::new_or_panic(&element);
                            quote::quote!{
                                Field::#element_upper_camel_case_token_stream => {
                                    if #element.is_some() {
                                        return Err(serde::de::Error::duplicate_field(#field_name_double_quotes_token_stream));
                                    }
                                    #element = Some(map.next_value()?);
                                }
                            }
                        });
                        quote::quote!{
                            while let Some(key) = map.next_key()? {
                                match key {
                                    #(#fields_initialization_token_stream)*
                                }
                            }
                        }
                    };
                    (
                        generate_while_some_next_key_field_token_stream(&[
                            &months_snake_case,
                            &days_snake_case,
                            &microseconds_snake_case,
                        ]),
                        generate_while_some_next_key_field_token_stream(&[
                            &start_snake_case,
                            &end_snake_case,
                        ]),
                    )
                };

                let (
                    match_field_initialization_months_days_microseconds_token_stream,
                    match_field_initialization_start_end_token_stream,
                ) = {
                    let generate_match_field_initialization_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]|{
                        let fields_initialization_token_stream = vec_token_stream.iter().enumerate().map(|(index, element)|{
                            //todo reuse
                            let field_index_name_token_stream = format!("__{}{index}", naming::FieldSnakeCase)
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap();
                            let field_name_double_quotes_token_stream = generate_quotes::double_quotes_stringified(&element);
                            quote::quote!{
                                let #field_index_name_token_stream = #element.ok_or_else(|| serde::de::Error::missing_field(#field_name_double_quotes_token_stream))?;
                            }
                        });
                        quote::quote!{#(#fields_initialization_token_stream)*}
                    };
                    (
                        generate_match_field_initialization_token_stream(&[
                            &months_snake_case,
                            &days_snake_case,
                            &microseconds_snake_case,
                        ]),
                        generate_match_field_initialization_token_stream(&[
                            &start_snake_case,
                            &end_snake_case,
                        ]),
                    )
                };
                (
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_months_days_microseconds_token_stream,
                        &while_some_next_key_field_months_days_microseconds_token_stream,
                        &match_field_initialization_months_days_microseconds_token_stream,
                        &quote::quote!{sqlx::postgres::types::PgInterval {
                            #months_snake_case: __field0,
                            #days_snake_case: __field1,
                            #microseconds_snake_case: __field2,
                        }},
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_start_end_token_stream,
                        &while_some_next_key_field_start_end_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                    generate_fn_visit_map_token_stream(
                        &field_option_none_initialization_start_end_token_stream,
                        &while_some_next_key_field_start_end_token_stream,
                        &match_field_initialization_start_end_token_stream,
                        &sqlx_postgres_types_pg_range_start_end_token_stream,
                    ),
                )
            };

            let (
                const_fields_sqlx_types_big_decimal_token_stream,
                const_fields_sqlx_types_time_date_token_stream,
                const_fields_sqlx_postgres_types_pg_interval_token_stream,
                const_fields_start_end_token_stream,
                const_fields_sqlx_types_time_offset_date_time_token_stream,
            ) = {
                let generate_const_fields_token_stream = |vec_token_stream: &[&dyn naming::StdFmtDisplayPlusQuoteToTokens]|{
                    let field_names_token_stream = vec_token_stream.iter().map(|element| generate_quotes::double_quotes_token_stream(&element));
                    quote::quote!{
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[#(#field_names_token_stream),*];
                    }
                };
                (
                    generate_const_fields_token_stream(&digits_scale_std_fmt_display_plus_quote_to_tokens_array),
                    generate_const_fields_token_stream(&year_month_day_std_fmt_display_plus_quote_to_tokens_array),
                    generate_const_fields_token_stream(&months_days_microseconds_std_fmt_display_plus_quote_to_tokens_array),
                    generate_const_fields_token_stream(&start_end_std_fmt_display_plus_quote_to_tokens_array),
                    generate_const_fields_token_stream(&date_time_offset_std_fmt_display_plus_quote_to_tokens_array)
                )
            };

            let impl_serde_deserialize_for_sqlx_postgres_types_pg_money_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #struct_visitor_token_stream
                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_newtype_struct_pg_money_token_stream
                        #fn_visit_seq_pg_money_token_stream
                    }
                    #serde_deserializer_deserialize_newtype_struct_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_types_big_decimal_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #enum_field_two_token_stream
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl serde::de::Visitor<'_> for __FieldVisitor {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_digits_scale_token_stream
                        #fn_visit_bytes_digits_scale_token_stream
                    }
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_types_big_decimal_token_stream
                        #fn_visit_map_sqlx_types_big_decimal_token_stream
                    }
                    #const_fields_sqlx_types_big_decimal_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                }
            });
            //todo default deserialize impl can cause an postgresql error "date of out range". pub const fn from_ordinal_date( do it too. if u want to check it just use sqlx::types::time::Date::MIN
            let impl_serde_deserialize_for_sqlx_types_time_date_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #enum_field_three_token_stream
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_three_token_stream
                        #fn_visit_str_year_month_day_token_stream
                        #fn_visit_bytes_year_month_day_token_stream
                    }
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_types_time_date_token_stream
                        #fn_visit_map_sqlx_types_time_date_token_stream
                    }
                    #const_fields_sqlx_types_time_date_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_interval_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    enum Field {
                        Months,
                        Days,
                        Microseconds,
                    }
                    impl<'de> serde::Deserialize<'de> for Field {
                        fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct FieldVisitor;
                            impl serde::de::Visitor<'_> for FieldVisitor {
                                type Value = Field;
                                #fn_expecting_months_or_days_or_microseconds_token_stream
                                fn visit_str<E>(self, value: &str) -> Result<Field, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "months" => Ok(Field::Months),
                                        "days" => Ok(Field::Days),
                                        "microseconds" => Ok(Field::Microseconds),
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(FieldVisitor)
                        }
                    }
                    struct #ident_visitor_upper_camel_case;
                    impl<'de> serde::de::Visitor<'de> for #ident_visitor_upper_camel_case {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_interval_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_interval_token_stream
                    }
                    #const_fields_sqlx_postgres_types_pg_interval_token_stream
                    #serde_deserializer_deserialize_struct_ident_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    enum Field {
                        Start,
                        End,
                    }
                    impl<'de> serde::Deserialize<'de> for Field {
                        fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct FieldVisitor;
                            impl serde::de::Visitor<'_> for FieldVisitor {
                                type Value = Field;
                                #fn_expecting_start_or_end_token_stream
                                fn visit_str<E>(self, value: &str) -> Result<Field, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "start" => Ok(Field::Start),
                                        "end" => Ok(Field::End),
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(FieldVisitor)
                        }
                    }
                    struct #ident_visitor_upper_camel_case;
                    impl<'de> serde::de::Visitor<'de> for #ident_visitor_upper_camel_case {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream
                    }
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_ident_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    enum Field {
                        Start,
                        End,
                    }
                    impl<'de> serde::Deserialize<'de> for Field {
                        fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct FieldVisitor;
                            impl serde::de::Visitor<'_> for FieldVisitor {
                                type Value = Field;
                                #fn_expecting_start_or_end_token_stream
                                fn visit_str<E>(self, value: &str) -> Result<Field, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "start" => Ok(Field::Start),
                                        "end" => Ok(Field::End),
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(FieldVisitor)
                        }
                    }
                    struct #ident_visitor_upper_camel_case;
                    impl<'de> serde::de::Visitor<'de> for #ident_visitor_upper_camel_case {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream
                    }
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_ident_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #enum_field_two_token_stream
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_start_end_token_stream
                        #fn_visit_bytes_start_end_token_stream
                    }
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream
                    }
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #enum_field_two_token_stream
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_start_end_token_stream
                        #fn_visit_bytes_start_end_token_stream
                    }
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream
                    }
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #enum_field_two_token_stream
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_start_end_token_stream
                        #fn_visit_bytes_start_end_token_stream
                    }
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream
                    }
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #enum_field_two_token_stream
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_start_end_token_stream
                        #fn_visit_bytes_start_end_token_stream
                    }
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream
                    }
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #enum_field_two_token_stream
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_start_end_token_stream
                        #fn_visit_bytes_start_end_token_stream
                    }
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream
                    }
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #enum_field_two_token_stream
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_start_end_token_stream
                        #fn_visit_bytes_start_end_token_stream
                    }
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream
                    }
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #enum_field_two_token_stream
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_start_end_token_stream
                        #fn_visit_bytes_start_end_token_stream
                    }
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream
                    }
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #enum_field_two_token_stream
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_start_end_token_stream
                        #fn_visit_bytes_start_end_token_stream
                    }
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream
                    }
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #enum_field_two_token_stream
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        #fn_expecting_field_identifier_token_stream
                        #fn_visit_u64_two_token_stream
                        #fn_visit_str_start_end_token_stream
                        #fn_visit_bytes_start_end_token_stream
                    }
                    #impl_serde_deserialize_for_field_token_stream
                    #struct_visitor_token_stream
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_seq_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream
                        #fn_visit_map_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream
                    }
                    #const_fields_start_end_token_stream
                    #serde_deserializer_deserialize_struct_visitor_token_stream
                }
            });
            // let impl_serde_deserialize_for_sqlx_types_time_offset_date_time_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
            //     quote::quote!{
            //         #enum_field_three_token_stream
            //         #[doc(hidden)]
            //         struct __FieldVisitor;
            //         impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
            //             type Value = __Field;
            //             #fn_expecting_field_identifier_token_stream
            //             #fn_visit_u64_three_token_stream
            //             #fn_visit_str_date_time_offset_token_stream
            //             #fn_visit_bytes_date_time_offset_token_stream
            //         }
            //         #impl_serde_deserialize_for_field_token_stream
            //         #struct_visitor_token_stream
            //         impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
            //             type Value = #postgresql_type;
            //             #fn_expecting_struct_ident_double_quotes_token_stream
            //             #fn_visit_seq_sqlx_types_time_offset_date_time_token_stream
            //             #fn_visit_map_sqlx_types_time_offset_date_time_token_stream
            //         }
            //         #const_fields_sqlx_types_time_offset_date_time_token_stream
            //         #serde_deserializer_deserialize_struct_visitor_token_stream
            //     }
            // });
            let impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #struct_visitor_token_stream
                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_newtype_struct_uuid_token_stream
                        #fn_visit_seq_sqlx_types_uuid_uuid_token_stream
                    }
                    #serde_deserializer_deserialize_newtype_struct_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_types_mac_address_mac_address_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #struct_visitor_token_stream
                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_newtype_struct_mac_address_token_stream
                        #fn_visit_seq_sqlx_types_mac_address_mac_address_token_stream
                    }
                    #serde_deserializer_deserialize_newtype_struct_token_stream
                }
            });
            let impl_serde_deserialize_for_sqlx_types_bit_vec_token_stream = generate_impl_serde_deserialize_for_tokens_token_stream(&{
                quote::quote!{
                    #struct_visitor_token_stream
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #postgresql_type;
                        #fn_expecting_struct_ident_double_quotes_token_stream
                        #fn_visit_newtype_struct_bit_vec_token_stream
                        #fn_visit_seq_sqlx_types_bit_vec_token_stream
                    }
                    #serde_deserializer_deserialize_newtype_struct_token_stream
                }
            });
            match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => impl_serde_deserialize_for_sqlx_postgres_types_pg_money_token_stream,
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => impl_serde_deserialize_for_sqlx_types_big_decimal_token_stream,
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlCharN => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlVarchar => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlText => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => impl_serde_deserialize_for_sqlx_types_time_date_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => impl_serde_deserialize_for_sqlx_postgres_types_pg_interval_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => impl_serde_deserialize_for_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),//impl_serde_deserialize_for_sqlx_types_time_offset_date_time_token_stream
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => impl_serde_deserialize_for_sqlx_types_uuid_uuid_token_stream,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => impl_serde_deserialize_for_sqlx_types_mac_address_mac_address_token_stream,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => impl_serde_deserialize_for_sqlx_types_bit_vec_token_stream,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => impl_serde_deserialize_for_sqlx_types_bit_vec_token_stream,
            }
        };
    
        let impl_crate_create_table_column_query_part_for_ident_token_stream = {
            let postgresql_query_type = match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => "int2",
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => "int4",
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => "int8",
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => "float4",
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => "float8",
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => "smallserial",
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => "serial",
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => "bigserial",
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => "money",
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => "numeric",
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => "numeric",
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => "bool",
                PostgresqlType::StdStringStringAsPostgresqlCharN => "char(10)",
                PostgresqlType::StdStringStringAsPostgresqlVarchar => "varchar(8)",
                PostgresqlType::StdStringStringAsPostgresqlText => "text",
                // PostgresqlType::StdStringStringAsPostgresqlCiText => "citext",
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => "bytea",
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => "date",
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => "date",
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => "time",
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => "time",
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => "interval",
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => "int4range",
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => "int8range",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => "tsrange",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => "tsrange",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => "tstzrange",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => "tstzrange",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => "tstzrange",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => "daterange",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => "daterange",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => "numrange",
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => "numrange",
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => "timestamp",
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => "timestamp",
                PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => "timestamptz",
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => "timestamptz",
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => "timestamptz",
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => "uuid",
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => "uuid",
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => "inet",
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => "cidr",
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => "macaddr",
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => "bit(9)",//todo
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => "bit varying(9)",
            };
            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                &format!("{{column}} {postgresql_query_type}")
            );
            quote::quote!{
                impl crate::CreateTableColumnQueryPart for #postgresql_type{
                    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
                        format!(#format_handle_token_stream)
                    }
                }
            }
        };

        let impl_std_fmt_display_for_ident_token_stream = generate_impl_std_fmt_display_for_tokens_token_stream(
            &postgresql_type,
            &quote::quote!{"{self:?}"}
        );

        let postgresql_base_type_tokens = {
            panic_location::panic_location();
            let std_option_option_ident_token_stream = quote::quote!{std::option::Option<#postgresql_type>};
            let impl_error_occurence_lib_to_std_string_string_for_ident_token_stream = generate_impl_error_occurence_lib_to_std_string_string_for_tokens_token_stream(
                &postgresql_type,
                &quote::quote!{self.to_string()},
            );
            let impl_sqlx_type_sqlx_postgres_for_ident_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
                &postgresql_type,
                &field_type
            );
            let impl_sqlx_type_sqlx_postgres_for_std_option_option_ident_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
                &std_option_option_ident_upper_camel_case,
                &std_option_option_ident_token_stream
            );
            let impl_sqlx_decode_sqlx_postgres_for_ident_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
                &postgresql_type,
                &field_type
            );
            let impl_sqlx_decode_sqlx_postgres_for_std_option_option_ident_token_stream = generate_impl_sqlx_decode_sqlx_postgres_for_tokens_token_stream(
                &std_option_option_ident_upper_camel_case,
                &quote::quote! {std::option::Option<#postgresql_type>}
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
                &postgresql_type,
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
                &quote::quote!{(pub std::option::Option<#postgresql_type>);},
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
                        &postgresql_type,
                        &{
                            let sqlx_postgres_types_pg_range_core_default_default_default_token_stream = generate_sqlx_postgres_types_pg_range_token_steram(//todo fix naming
                                &core_default_default_default_token_stream,
                                &core_default_default_default_token_stream,
                            );
                            let initialization_token_stream: &dyn quote::ToTokens = match &postgresql_type {
                                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 |
                                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 |
                                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 |
                                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 |
                                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 |
                                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql |
                                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql |
                                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => &core_default_default_default_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => &quote::quote!{sqlx::postgres::types::PgMoney(#core_default_default_default_token_stream)},
                                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric |
                                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric |
                                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool |
                                PostgresqlType::StdStringStringAsPostgresqlCharN |
                                PostgresqlType::StdStringStringAsPostgresqlVarchar |
                                PostgresqlType::StdStringStringAsPostgresqlText => &core_default_default_default_token_stream,
                                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => &quote::quote!{vec![#core_default_default_default_token_stream]},
                                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream(),
                                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate |
                                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => &core_default_default_default_token_stream,
                                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => &{
                                    let value = token_patterns::SqlxTypesTimeTimeMidnight;
                                    quote::quote!{#value}
                                },
                                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => &{
                                    quote::quote!{sqlx::postgres::types::PgInterval {
                                        months: #core_default_default_default_token_stream,
                                        days: #core_default_default_default_token_stream,
                                        microseconds: #core_default_default_default_token_stream,
                                    }}
                                },
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range |
                                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range |
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => &{
                                    let sqlx_types_time_primitive_date_time_new_token_stream = sqlx_types_time_primitive_date_time_new_token_stream();
                                    generate_sqlx_postgres_types_pg_range_token_steram(
                                        &sqlx_types_time_primitive_date_time_new_token_stream,
                                        &sqlx_types_time_primitive_date_time_new_token_stream,
                                    )
                                },
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange |
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => &{
                                    let sqlx_types_time_offset_date_time_unix_epoch = token_patterns::SqlxTypesTimeOffsetDateTimeUnixEpoch;
                                    generate_sqlx_postgres_types_pg_range_token_steram(
                                        &sqlx_types_time_offset_date_time_unix_epoch,
                                        &sqlx_types_time_offset_date_time_unix_epoch,
                                    )
                                },
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => &{
                                    let sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream = sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream();
                                    generate_sqlx_postgres_types_pg_range_token_steram(
                                        &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                                        &sqlx_types_time_date_from_ordinal_date_core_default_default_default_one_unwrap_token_stream,
                                    )
                                },
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange |
                                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => &sqlx_postgres_types_pg_range_core_default_default_default_token_stream,

                                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => &core_default_default_default_token_stream,
                                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => &sqlx_types_time_primitive_date_time_new_token_stream(),
                                PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => &{
                                    let value = token_patterns::SqlxTypesTimeOffsetDateTimeUnixEpoch;
                                    quote::quote!{#value}
                                },
                                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz |
                                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz |
                                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql |
                                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => &core_default_default_default_token_stream,
                                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet |
                                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => &sqlx_types_ipnetwork_ip_network_v4_token_stream(),
                                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => &core_default_default_default_token_stream,
                                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit |
                                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => &quote::quote!{{
                                    let mut value = sqlx::types::BitVec::new();
                                    value.push(false);
                                    value
                                }}
                            };
                            quote::quote!{Self(#initialization_token_stream)}
                        },
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
                            " not null"
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
                        &postgresql_type,
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
                    impl crate::postgresql_type::postgresql_base_type_trait:: #postgresql_base_type_self_traits_upper_camel_case<'_> for #postgresql_type{}
                }
            };
            let impl_postgresql_base_type_for_ident_token_stream = {
                let postgresql_base_type_upper_camel_case = naming::PostgresqlBaseTypeUpperCamelCase;
                let postgresql_base_type_self_upper_camel_case = naming::PostgresqlBaseTypeSelfUpperCamelCase;
                let postgresql_base_type_std_option_option_self_upper_camel_case = naming::PostgresqlBaseTypeStdOptionOptionSelfUpperCamelCase;
                quote::quote! {
                    impl crate::postgresql_type::postgresql_base_type_trait:: #postgresql_base_type_upper_camel_case<'_> for #postgresql_type{
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
            generated
        };
        let maybe_primary_key_tokens_token_stream = {
            let primary_key_token_stream = {
                let impl_sqlx_encode_sqlx_postgres_for_ident_token_stream = generate_impl_sqlx_encode_sqlx_postgres_for_tokens_token_stream(&postgresql_type);
                let impl_sqlx_postgres_pg_has_array_type_for_token_stream = {
                    quote::quote!{
                        impl sqlx::postgres::PgHasArrayType for #postgresql_type{
                            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                                <#field_type as sqlx::postgres::PgHasArrayType>::array_type_info()
                            }
                        }
                    }
                };
                let impl_crate_postgresql_type_postgresql_base_type_trait_postgresql_base_type_primary_key_for_ident_token_stream = {
                    let postgresql_base_type_primary_key_upper_camel_case = naming::PostgresqlBaseTypePrimaryKeyUpperCamelCase;
                    let postgresql_base_type_self_upper_camel_case = naming::PostgresqlBaseTypeSelfUpperCamelCase;
                    quote::quote!{
                        impl crate::postgresql_type::postgresql_base_type_trait:: #postgresql_base_type_primary_key_upper_camel_case<'_> for #postgresql_type{
                            type #postgresql_base_type_self_upper_camel_case = Self;
                        }
                    }
                };
                quote::quote!{
                    #impl_sqlx_encode_sqlx_postgres_for_ident_token_stream
                    #impl_sqlx_postgres_pg_has_array_type_for_token_stream
                    #impl_crate_postgresql_type_postgresql_base_type_trait_postgresql_base_type_primary_key_for_ident_token_stream
                }
            };
            match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 |
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 |
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 |
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 |
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => proc_macro2::TokenStream::new(),
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql |
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql |
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => primary_key_token_stream,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney |
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric |
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric |
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool |
                PostgresqlType::StdStringStringAsPostgresqlCharN |
                PostgresqlType::StdStringStringAsPostgresqlVarchar |
                PostgresqlType::StdStringStringAsPostgresqlText |
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea |
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate |
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate |
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime |
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime |
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval |
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range |
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange |
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange |
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp |
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp |
                PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz |
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz |
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => proc_macro2::TokenStream::new(),
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql |
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => primary_key_token_stream,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet |
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr |
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr |
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit |
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => proc_macro2::TokenStream::new(),
            }
        };

        let where_element_token_stream = {
            let where_operator_type_field_type_default = WhereOperatorType::FieldType {
                field_type: &field_type,
                default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
            };
            let where_element_number_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let in_handle = crate::filters::In;
                let postgresql_type_tokens_where_element_in_token_stream = in_handle.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_postgres_types_pg_money_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let where_operator_type_ident = WhereOperatorType::Ident(&postgresql_type);
        
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::True,
                );
                let in_handle = crate::filters::In;
                let postgresql_type_tokens_where_element_in_token_stream = in_handle.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_decimal_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_big_decimal_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let where_operator_type_ident = WhereOperatorType::Ident(&postgresql_type);
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_bool_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &WhereOperatorType::FieldType {
                        field_type: &field_type,
                        default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
                    },
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
                    &vec![
                        &equal,
                    ]
                );
                quote::quote! {
                    #postgresql_type_tokens_where_element_equal_token_stream
                    #postgresql_type_tokens_where_element_token_stream
                }
            });
            let where_element_std_string_string_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let case_sensitive_regular_expression = crate::filters::CaseSensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let case_insensitive_regular_expression = crate::filters::CaseInsensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_std_vec_vec_std_primitive_u8_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &WhereOperatorType::FieldType {
                        field_type: &field_type,
                        default_initialization_token_stream: &token_patterns::CoreDefaultDefaultDefault,
                    },
                );
                let length_more_than = crate::filters::LengthMoreThan;
                let postgresql_type_tokens_where_element_length_more_than_token_stream = length_more_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let equal_to_encoded_string_representation = crate::filters::EqualToEncodedStringRepresentation;
                let postgresql_type_tokens_where_element_equal_to_encoded_string_representation_token_stream = equal_to_encoded_string_representation.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_time_date_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let where_operator_type_ident = WhereOperatorType::Ident(&postgresql_type);
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let current_date = crate::filters::CurrentDate;
                let postgresql_type_tokens_where_element_current_date_token_stream = current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let greater_than_current_date = crate::filters::GreaterThanCurrentDate;
                let postgresql_type_tokens_where_element_greater_than_current_date_token_stream = greater_than_current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_chrono_naive_date_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let current_date = crate::filters::CurrentDate;
                let postgresql_type_tokens_where_element_current_date_token_stream = current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let greater_than_current_date = crate::filters::GreaterThanCurrentDate;
                let postgresql_type_tokens_where_element_greater_than_current_date_token_stream = greater_than_current_date.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_chrono_naive_time_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let current_time = crate::filters::CurrentTime;
                let postgresql_type_tokens_where_element_current_time_token_stream = current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let greater_than_current_time = crate::filters::GreaterThanCurrentTime;
                let postgresql_type_tokens_where_element_greater_than_current_time_token_stream = greater_than_current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_time_time_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let where_operator_type_field_type = WhereOperatorType::FieldType {
                    field_type: &field_type,
                    default_initialization_token_stream: &token_patterns::SqlxTypesTimeTimeMidnight,
                };
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type,
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let current_time = crate::filters::CurrentTime;
                let postgresql_type_tokens_where_element_current_time_token_stream = current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let greater_than_current_time = crate::filters::GreaterThanCurrentTime;
                let postgresql_type_tokens_where_element_greater_than_current_time_token_stream = greater_than_current_time.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_postgres_types_pg_interval_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let where_operator_type_ident = WhereOperatorType::Ident(&postgresql_type);
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                    &crate::filters::BetweenTryNewErrorType::StartIsEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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

            let generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream = |range_type: RangeType|{
                generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
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
                        &postgresql_type,
                        &is_nullable,
                        &WhereOperatorType::Ident(&postgresql_type),
                    );
                    let value_is_contained_within_range = crate::filters::ValueIsContainedWithinRange;
                    let postgresql_type_tokens_where_element_value_is_contained_within_range_token_stream = value_is_contained_within_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                        &postgresql_type,
                        &is_nullable,
                        &range_type_token_stream,
                        &range_type_should_impl_range_length,
                        &range_type_default_initialization_token_stream,
                        &range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream,
                    );
                    let contains_another_range = crate::filters::ContainsAnotherRange;
                    let postgresql_type_tokens_where_element_contains_another_range_token_stream = contains_another_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                        &postgresql_type,
                        &is_nullable,
                    );
                    let strictly_to_left_of_range = crate::filters::StrictlyToLeftOfRange;
                    let postgresql_type_tokens_where_element_strictly_to_left_of_range_token_stream = strictly_to_left_of_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                        &postgresql_type,
                        &is_nullable,
                    );
                    let strictly_to_right_of_range = crate::filters::StrictlyToRightOfRange;
                    let postgresql_type_tokens_where_element_strictly_to_right_of_range_token_stream = strictly_to_right_of_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                        &postgresql_type,
                        &is_nullable,
                    );
                    let included_lower_bound = crate::filters::IncludedLowerBound;
                    let postgresql_type_tokens_where_element_included_lower_bound_token_stream = included_lower_bound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                        &postgresql_type,
                        &is_nullable,
                        &range_type_token_stream,
                        &range_type_default_initialization_token_stream,
                        &range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream,
                    );
                    let excluded_upper_bound = crate::filters::ExcludedUpperBound;
                    let postgresql_type_tokens_where_element_excluded_upper_bound_token_stream = excluded_upper_bound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                        &postgresql_type,
                        &is_nullable,
                        &range_type_token_stream,
                        &range_type_default_initialization_token_stream,
                        &range_type_postgresql_type_self_where_bind_value_to_query_parameter_token_stream,
                    );
                    let greater_than_lower_bound = crate::filters::GreaterThanLowerBound;
                    let postgresql_type_tokens_where_element_greater_than_lower_bound_token_stream = greater_than_lower_bound.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                        &postgresql_type,
                        &is_nullable,
                    );
                    let overlap_with_range = crate::filters::OverlapWithRange;
                    let postgresql_type_tokens_where_element_overlap_with_range_token_stream = overlap_with_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                        &postgresql_type,
                        &is_nullable,
                    );
                    let adjacent_with_range = crate::filters::AdjacentWithRange;
                    let postgresql_type_tokens_where_element_adjacent_with_range_token_stream = adjacent_with_range.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                        &postgresql_type,
                        &is_nullable,
                    );
                    //todo find out maximum length of range(INT8RANGE, INT4RANGE) in postgresql
                    let range_length = crate::filters::RangeLength;
                    let maybe_postgresql_type_tokens_where_element_range_length_token_stream = match &range_type_should_impl_range_length {
                        ShouldImplRangeLength::True => range_length.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                            &postgresql_type,
                            &is_nullable,
                        ),
                        ShouldImplRangeLength::False => proc_macro2::TokenStream::new(), 
                    };
                    let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                        is_nullable,
                        &postgresql_type,
                        &{
                            let mut value: std::vec::Vec<&dyn crate::filters::WhereOperatorName> = vec![
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
                })
            };

            let where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::I32);
            let where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::I64);
            let where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime);
            let where_element_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime);
            let where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc);
            let where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal);
            let where_element_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime);
            let where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate);
            let where_element_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxPostgresTypesPgRangeSqlxTypesTimeDate);
            let where_element_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxPostgresTypesPgRangeSqlxTypesDecimal);
            let where_element_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream = generate_where_element_sqlx_postgres_types_pg_range_filter_token_stream(RangeType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal);
            let where_element_sqlx_types_chrono_naive_date_time_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type_default,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let current_timestamp = crate::filters::CurrentTimestamp;
                let postgresql_type_tokens_where_element_current_timestamp_token_stream = current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let greater_than_current_timestamp = crate::filters::GreaterThanCurrentTimestamp;
                let postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream = greater_than_current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_time_primitive_date_time_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let sqlx_types_time_time_midnight = token_patterns::SqlxTypesTimeTimeMidnight;
                let where_operator_type_field_type = WhereOperatorType::FieldType {
                    field_type: &field_type,
                    default_initialization_token_stream: &sqlx_types_time_primitive_date_time_new_token_stream(),
                };
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type,
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_field_type,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let current_timestamp = crate::filters::CurrentTimestamp;
                let postgresql_type_tokens_where_element_current_timestamp_token_stream = current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let greater_than_current_timestamp = crate::filters::GreaterThanCurrentTimestamp;
                let postgresql_type_tokens_where_element_greater_than_current_timestamp_token_stream = greater_than_current_timestamp.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_time_offset_date_time_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let where_operator_type_ident = WhereOperatorType::Ident(&postgresql_type);
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                );
                let before = crate::filters::Before;
                let postgresql_type_tokens_where_element_before_token_stream = before.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
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
                    &postgresql_type,
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
            let where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let where_operator_type_ident = WhereOperatorType::Ident(&postgresql_type);
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                );
                let before = crate::filters::Before;
                let postgresql_type_tokens_where_element_before_token_stream = before.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let where_operator_type_ident = WhereOperatorType::Ident(&postgresql_type);
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                );
                let before = crate::filters::Before;
                let postgresql_type_tokens_where_element_before_token_stream = before.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let between = crate::filters::Between;
                let postgresql_type_tokens_where_element_between_token_stream = between.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                    &crate::filters::BetweenTryNewErrorType::StartMoreOrEqualToEnd,
                    &crate::filters::ShouldAddDotZero::False,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_uuid_uuid_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &WhereOperatorType::Ident(&postgresql_type),
                );
                let case_sensitive_regular_expression = crate::filters::CaseSensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let case_insensitive_regular_expression = crate::filters::CaseInsensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_ipnetwork_ip_network_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &WhereOperatorType::FieldType {
                        field_type: &field_type,
                        default_initialization_token_stream: &sqlx_types_ipnetwork_ip_network_v4_token_stream()
                    },
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
                    &vec![
                        &equal,
                    ]
                );
                quote::quote! {
                    #postgresql_type_tokens_where_element_equal_token_stream
                    #postgresql_type_tokens_where_element_token_stream
                }
            });
            let where_element_sqlx_types_mac_address_mac_address_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let where_operator_type_ident = WhereOperatorType::Ident(&postgresql_type);
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident,
                );
                let greater_than = crate::filters::GreaterThan;
                let postgresql_type_tokens_where_element_greater_than_token_stream = greater_than.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &where_operator_type_ident
                );
                let case_sensitive_regular_expression = crate::filters::CaseSensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_sensitive_regular_expression_token_stream = case_sensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let case_insensitive_regular_expression = crate::filters::CaseInsensitiveRegularExpression;
                let postgresql_type_tokens_where_element_case_insensitive_regular_expression_token_stream = case_insensitive_regular_expression.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
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
            let where_element_sqlx_types_bit_vec_token_stream = generate_nullable_and_not_nullable_token_stream(|is_nullable: IsNullable| -> proc_macro2::TokenStream {
                let increment_snake_case = naming::IncrementSnakeCase;
                let value_snake_case = naming::ValueSnakeCase;
                let column_snake_case = naming::ColumnSnakeCase;
                let query_snake_case = naming::QuerySnakeCase;
                let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
                let try_generate_bind_increments_error_named_upper_camel_case = naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase;
                let equal = crate::filters::Equal;
                let postgresql_type_tokens_where_element_equal_token_stream = equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                    &WhereOperatorType::Ident(&postgresql_type),
                );
                let bit_vec_position_equal = crate::filters::BitVecPositionEqual;
                let postgresql_type_tokens_where_element_bit_vec_position_equal_token_stream = bit_vec_position_equal.generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
                    &postgresql_type,
                    &is_nullable,
                );
                let postgresql_type_tokens_where_element_token_stream = generate_postgresql_type_tokens_where_element_and_postgresql_type_std_option_option_tokens_where_element_token_stream(
                    is_nullable,
                    &postgresql_type,
                    &vec![
                        &equal,
                        &bit_vec_position_equal,
                    ]
                );
                quote::quote! {
                    #postgresql_type_tokens_where_element_equal_token_stream
                    #postgresql_type_tokens_where_element_bit_vec_position_equal_token_stream
                    #postgresql_type_tokens_where_element_token_stream
                }
            });
            match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => where_element_number_token_stream,
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => where_element_number_token_stream,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => where_element_sqlx_postgres_types_pg_money_token_stream,
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => where_element_sqlx_types_decimal_token_stream,
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => where_element_sqlx_types_big_decimal_token_stream,
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => where_element_bool_token_stream,
                PostgresqlType::StdStringStringAsPostgresqlCharN => where_element_std_string_string_token_stream,
                PostgresqlType::StdStringStringAsPostgresqlVarchar => where_element_std_string_string_token_stream,
                PostgresqlType::StdStringStringAsPostgresqlText => where_element_std_string_string_token_stream,
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => where_element_std_vec_vec_std_primitive_u8_token_stream,
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => where_element_sqlx_types_time_date_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => where_element_sqlx_types_chrono_naive_date_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => where_element_sqlx_types_chrono_naive_time_token_stream,
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => where_element_sqlx_types_time_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => where_element_sqlx_postgres_types_pg_interval_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i32_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => where_element_sqlx_postgres_types_pg_range_std_primitive_i64_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_time_date_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_decimal_token_stream,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => where_element_sqlx_postgres_types_pg_range_sqlx_types_big_decimal_token_stream,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => where_element_sqlx_types_chrono_naive_date_time_token_stream,
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => where_element_sqlx_types_time_primitive_date_time_token_stream,
                PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => where_element_sqlx_types_time_offset_date_time_token_stream,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_token_stream,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => where_element_sqlx_types_chrono_date_time_sqlx_types_chrono_local_token_stream,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => where_element_sqlx_types_uuid_uuid_token_stream,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => where_element_sqlx_types_uuid_uuid_token_stream,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => where_element_sqlx_types_ipnetwork_ip_network_token_stream,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => where_element_sqlx_types_ipnetwork_ip_network_token_stream,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => where_element_sqlx_types_mac_address_mac_address_token_stream,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => where_element_sqlx_types_bit_vec_token_stream,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => where_element_sqlx_types_bit_vec_token_stream,
            }
        };

        let postgresql_type_initialized_by_tokens_token_stream = {
            let postgresql_type_initialized_by_tokens = match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => PostgresqlTypeInitializedByTokens::InitializedUsingDefaultKeywordByPostgresql,
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => PostgresqlTypeInitializedByTokens::InitializedUsingDefaultKeywordByPostgresql,
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => PostgresqlTypeInitializedByTokens::InitializedUsingDefaultKeywordByPostgresql,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::StdStringStringAsPostgresqlCharN => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::StdStringStringAsPostgresqlVarchar => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::StdStringStringAsPostgresqlText => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => PostgresqlTypeInitializedByTokens::InitializedUsingUuidGenerateV4FunctionByPostgresql,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => PostgresqlTypeInitializedByTokens::InitializedByClient,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => PostgresqlTypeInitializedByTokens::InitializedByClient,
            };
            let generate_postgresql_type_nullable_or_not_null = |postgresql_type_nullable_or_not_null: &PostgresqlTypeNullableOrNotNull| -> proc_macro2::TokenStream {
                let postgresql_type_field_type_where_element_upper_camel_case: &dyn quote::ToTokens = match &postgresql_type_nullable_or_not_null {
                    PostgresqlTypeNullableOrNotNull::Nullable => &naming::parameter::PostgresqlTypeStdOptionOptionSelfWhereElementUpperCamelCase::from_tokens(&postgresql_type),
                    PostgresqlTypeNullableOrNotNull::NotNull => &naming::parameter::PostgresqlTypeSelfWhereElementUpperCamelCase::from_tokens(&postgresql_type),
                };
                let ident_handle: &dyn quote::ToTokens = &postgresql_type_nullable_or_not_null.ident_handle(&postgresql_type);
                let field_type_handle: &dyn quote::ToTokens = match &postgresql_type_nullable_or_not_null {
                    PostgresqlTypeNullableOrNotNull::Nullable => &std_option_option_ident_upper_camel_case,
                    PostgresqlTypeNullableOrNotNull::NotNull => &postgresql_type,
                };

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
                let postgresql_type_ident_where_token_stream = generate_postgresql_type_or_json_type_where_token_stream(
                    &PostgresqlTypeOrJsonType::PostgresqlType,
                    &ident_handle,
                    &postgresql_type_ident_where_element_upper_camel_case,
                    &postgresql_type_ident_where_upper_camel_case
                );

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
                    let postgresql_type_self_where_element_upper_camel_case = naming::PostgresqlTypeSelfWhereElementUpperCamelCase;
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
            generated
        };

        let is_primary_key = IsPrimaryKey::False;
        let impl_crate_create_table_column_query_part_for_ident_nullable_token_stream = generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
           &PostgresqlTypeNullableOrNotNull::Nullable,
            &postgresql_type,
            &field_type,
            &is_primary_key,
        );
        let impl_crate_create_table_column_query_part_for_ident_not_null_token_stream = generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
           &PostgresqlTypeNullableOrNotNull::NotNull,
            &postgresql_type,
            &field_type,
            &is_primary_key,
        );

        let postgresql_type_create_table_column_query_part_token_stream = {
            let postgresql_type_create_table_column_query_part_tokens = {
                let is_primary_key = IsPrimaryKey::False;
                let impl_crate_create_table_column_query_part_for_ident_nullable_token_stream = generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
                   &PostgresqlTypeNullableOrNotNull::Nullable,
                    &postgresql_type,
                    &field_type,
                    &is_primary_key,
                );
                let impl_crate_create_table_column_query_part_for_ident_not_null_token_stream = generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
                   &PostgresqlTypeNullableOrNotNull::NotNull,
                    &postgresql_type,
                    &field_type,
                    &is_primary_key,
                );
                let generated = quote::quote!{
                    #impl_crate_create_table_column_query_part_for_ident_nullable_token_stream
                    #impl_crate_create_table_column_query_part_for_ident_not_null_token_stream
                };
                generated
            };
            let postgresql_type_create_table_column_query_part_primary_key_tokens = {
                let is_primary_key = IsPrimaryKey::True;
                let impl_crate_create_table_column_query_part_for_ident_not_null_token_stream = generate_impl_crate_create_table_column_query_part_for_ident_token_stream(
                   &PostgresqlTypeNullableOrNotNull::NotNull,
                    &postgresql_type,
                    &field_type,
                    &is_primary_key,
                );
                let generated = quote::quote!{
                    #impl_crate_create_table_column_query_part_for_ident_not_null_token_stream
                };
                generated.into()
            };
            match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => postgresql_type_create_table_column_query_part_primary_key_tokens,
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => postgresql_type_create_table_column_query_part_primary_key_tokens,
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => postgresql_type_create_table_column_query_part_primary_key_tokens,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::StdStringStringAsPostgresqlCharN => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::StdStringStringAsPostgresqlVarchar => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::StdStringStringAsPostgresqlText => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => postgresql_type_create_table_column_query_part_primary_key_tokens,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => postgresql_type_create_table_column_query_part_tokens,
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => postgresql_type_create_table_column_query_part_tokens,
            }
        };

        let maybe_postgresql_type_primary_key_token_stream = {
            let postgresql_type_primary_key_token_stream = {
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
                let postgresql_type_struct_content_token_stream = quote::quote!{(#postgresql_type);};
                let postgresql_type_ident_not_null_to_create_upper_camel_case = naming::parameter::PostgresqlTypeSelfNotNullToCreateUpperCamelCase::from_tokens(&postgresql_type);
                let postgresql_type_ident_not_null_to_create_token_stream = {
                    let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_create_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
                        &postgresql_type_ident_not_null_to_create_upper_camel_case,
                        &postgresql_type,
                    );
                    quote::quote! {
                        #impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_create_token_stream
                    }
                };
                let postgresql_type_ident_not_null_to_read_upper_camel_case = naming::parameter::PostgresqlTypeSelfNotNullToReadUpperCamelCase::from_tokens(&postgresql_type);
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
                let postgresql_type_ident_not_null_to_update_upper_camel_case = naming::parameter::PostgresqlTypeSelfNotNullToUpdateUpperCamelCase::from_tokens(&postgresql_type);
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
                        &postgresql_type,
                    );
                    let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_update_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
                        &postgresql_type_ident_not_null_to_update_upper_camel_case,
                        &postgresql_type,
                    );
                    quote::quote! {
                        #impl_std_fmt_display_for_postgresql_type_ident_not_null_to_update_token_stream
                        #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_ident_not_null_to_update_token_stream
                        #impl_sqlx_encode_sqlx_postgres_for_postgresql_type_ident_not_null_to_update_token_stream
                        #impl_sqlx_decode_sqlx_postgres_for_postgresql_type_ident_not_null_to_update_token_stream
                        #impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_update_token_stream
                    }
                };
                let postgresql_type_ident_not_null_to_delete_upper_camel_case = naming::parameter::SelfNotNullToDeleteUpperCamelCase::from_tokens(&postgresql_type);
                let postgresql_type_ident_not_null_to_delete_token_stream = {
                    let postgresql_type_ident_not_null_to_delete_token_stream = generate_pub_struct_tokens_token_stream(
                        Visibility::Pub,
                        &postgresql_type_ident_not_null_to_delete_upper_camel_case,
                        &postgresql_type_struct_content_token_stream,
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
                        &postgresql_type,
                    );
                    let impl_sqlx_type_sqlx_postgres_for_postgresql_type_ident_not_null_to_delete_token_stream = generate_impl_sqlx_type_sqlx_postgres_for_tokens_token_stream(
                        &postgresql_type_ident_not_null_to_delete_upper_camel_case,
                        &postgresql_type,
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
                    let ident_not_null = naming::parameter::SelfNotNullUpperCamelCase::from_tokens(&postgresql_type);
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
                generated
            };
            match &postgresql_type {
                PostgresqlType::StdPrimitiveI16AsPostgresqlInt2 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI32AsPostgresqlInt4 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI64AsPostgresqlInt8 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8 => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql => postgresql_type_primary_key_token_stream,
                PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql => postgresql_type_primary_key_token_stream,
                PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql => postgresql_type_primary_key_token_stream,
                PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdPrimitiveBoolAsPostgresqlBool => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlCharN => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlVarchar => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdStringStringAsPostgresqlText => proc_macro2_token_stream_new.clone(),
                PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql => postgresql_type_primary_key_token_stream,
                PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesBitVecAsPostgresqlBit => proc_macro2_token_stream_new.clone(),
                PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit => proc_macro2_token_stream_new.clone(),
            }
        };

        let generated = quote::quote!{
            #ident_token_stream
            #maybe_impl_try_new_token_stream
            #maybe_impl_serde_serialize_token_stream
            #maybe_impl_serde_deserialize_token_stream

            #impl_crate_create_table_column_query_part_for_ident_token_stream
            #impl_std_fmt_display_for_ident_token_stream

            #postgresql_base_type_tokens

            #maybe_primary_key_tokens_token_stream

            #where_element_token_stream

            #postgresql_type_initialized_by_tokens_token_stream

            #postgresql_type_create_table_column_query_part_token_stream

            #maybe_postgresql_type_primary_key_token_stream
        };
        // if ident == "" {
        //       macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //           "PostgresqlTypeTokens",
        //           &generated,
        //       );
        /// }
        generated
    };
    let postgresql_type_array = PostgresqlType::into_array().map(|element|generate_postgresql_type_token_stream(element));
    
    // let h1 = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI16AsPostgresqlInt2);
    // let h2 = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI32AsPostgresqlInt4);
    // let h3 = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI64AsPostgresqlInt8);
    // let h4 = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveF32AsPostgresqlFloat4);
    // let h5 = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveF64AsPostgresqlFloat8);
    // let h6 = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql);
    // let h7 = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql);
    // let h8 = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql);
    // let h9 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgMoneyAsPostgresqlMoney);
    // let h10 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesDecimalAsPostgresqlNumeric);
    // let h11 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesBigDecimalAsPostgresqlNumeric);
    // let h12 = generate_postgresql_type_token_stream(PostgresqlType::StdPrimitiveBoolAsPostgresqlBool);
    // let h13 = generate_postgresql_type_token_stream(PostgresqlType::StdStringStringAsPostgresqlCharN);
    // let h14 = generate_postgresql_type_token_stream(PostgresqlType::StdStringStringAsPostgresqlVarchar);
    // let h15 = generate_postgresql_type_token_stream(PostgresqlType::StdStringStringAsPostgresqlText);
    // let h16 = generate_postgresql_type_token_stream(PostgresqlType::StdVecVecStdPrimitiveU8AsPostgresqlBytea);
    // let h17 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesTimeDateAsPostgresqlDate);
    // let h18 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesChronoNaiveDateAsPostgresqlDate);
    // let h19 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesChronoNaiveTimeAsPostgresqlTime);
    // let h20 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesTimeTimeAsPostgresqlTime);
    // let h21 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgIntervalAsPostgresqlInterval);
    // let h22 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range);
    // let h23 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range);
    // let h24 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange);
    // let h25 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange);
    // let h26 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange);
    // let h27 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange);
    // let h28 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange);
    // let h29 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange);
    // let h30 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange);
    // let h31 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange);
    // let h32 = generate_postgresql_type_token_stream(PostgresqlType::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange);
    // let h33 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp);
    // let h34 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp);
    // let h35 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz);
    // let h36 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz);
    // let h37 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz);
    // let h38 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql);
    // let h39 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient);
    // let h40 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet);
    // let h41 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr);
    // let h42 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr);
    // let h43 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesBitVecAsPostgresqlBit);
    // let h44 = generate_postgresql_type_token_stream(PostgresqlType::SqlxTypesBitVecAsPostgresqlVarbit);
    let generated = quote::quote!{
        #(#postgresql_type_array)*

        // #h1
        // #h2
        // #h3
        // #h4
        // #h5
        // #h6
        // #h7
        // #h8
        // #h9
        // #h10
        // #h11
        // #h12
        // #h13
        // #h14
        // #h15
        // #h16
        // #h17
        // #h18
        // #h19
        // #h20
        // #h21
        // #h22
        // #h23
        // #h24
        // #h25
        // #h26
        // #h27
        // #h28
        // #h29
        // #h30
        // #h31
        // #h32
        // #h33
        // #h34
        // #h35
        // #h36
        // #h37
        // #h38
        // #h39
        // #h40
        // #h41
        // #h42
        // #h43
        // #h44
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "PostgresqlTypeTokens",
    //     &generated,
    // );
    generated.into()
}