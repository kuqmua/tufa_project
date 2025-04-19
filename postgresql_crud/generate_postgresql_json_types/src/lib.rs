#[proc_macro]
pub fn generate_postgresql_json_types(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();

    #[derive(Debug, strum_macros::Display)]
    enum RustTypeName {
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
    impl std::convert::From<&PostgresqlJsonType> for RustTypeName {
        fn from(value: &PostgresqlJsonType) -> Self {
            match &value {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => Self::StdPrimitiveI8,
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => Self::StdPrimitiveI16,
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => Self::StdPrimitiveI32,
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => Self::StdPrimitiveI64,
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => Self::StdPrimitiveU8,
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => Self::StdPrimitiveU16,
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => Self::StdPrimitiveU32,
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => Self::StdPrimitiveU64,
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => Self::StdPrimitiveF32,
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => Self::StdPrimitiveF64,
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => Self::StdPrimitiveBool,
                PostgresqlJsonType::StdStringStringAsJsonbString => Self::StdStringString,
                PostgresqlJsonType::UuidUuidAsJsonbString => Self::UuidUuid,
            }
        }
    }
    #[derive(Debug, strum_macros::Display)]
    enum PostgresqlJsonTypeName {
        JsonbNumber,
        JsonbBoolean,
        JsonbString,
    }
    impl std::convert::From<&PostgresqlJsonType> for PostgresqlJsonTypeName {
        fn from(value: &PostgresqlJsonType) -> Self {
            match &value {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => Self::JsonbNumber,
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => Self::JsonbNumber,
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => Self::JsonbNumber,
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => Self::JsonbNumber,
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => Self::JsonbNumber,
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => Self::JsonbNumber,
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => Self::JsonbNumber,
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => Self::JsonbNumber,
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => Self::JsonbNumber,
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => Self::JsonbNumber,
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => Self::JsonbBoolean,
                PostgresqlJsonType::StdStringStringAsJsonbString => Self::JsonbString,
                PostgresqlJsonType::UuidUuidAsJsonbString => Self::JsonbString,
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    pub enum PostgresqlJsonType {
        StdPrimitiveI8AsJsonbNumber,
        StdPrimitiveI16AsJsonbNumber,
        StdPrimitiveI32AsJsonbNumber,
        StdPrimitiveI64AsJsonbNumber,
        StdPrimitiveU8AsJsonbNumber,
        StdPrimitiveU16AsJsonbNumber,
        StdPrimitiveU32AsJsonbNumber,
        StdPrimitiveU64AsJsonbNumber,
        StdPrimitiveF32AsJsonbNumber,
        StdPrimitiveF64AsJsonbNumber,
        StdPrimitiveBoolAsJsonbBoolean,
        StdStringStringAsJsonbString,
        UuidUuidAsJsonbString,
    }
    impl PostgresqlJsonType {
        fn full_type_path_initialization_token_stream(&self) -> proc_macro2::TokenStream {
            match &self {
                Self::StdPrimitiveI8AsJsonbNumber
                | Self::StdPrimitiveI16AsJsonbNumber
                | Self::StdPrimitiveI32AsJsonbNumber
                | Self::StdPrimitiveI64AsJsonbNumber
                | Self::StdPrimitiveU8AsJsonbNumber
                | Self::StdPrimitiveU16AsJsonbNumber
                | Self::StdPrimitiveU32AsJsonbNumber
                | Self::StdPrimitiveU64AsJsonbNumber
                | Self::StdPrimitiveF32AsJsonbNumber
                | Self::StdPrimitiveF64AsJsonbNumber
                | Self::StdPrimitiveBoolAsJsonbBoolean
                | Self::StdStringStringAsJsonbString => {
                    let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
                    quote::quote! {#core_default_default_default_token_stream}
                }
                Self::UuidUuidAsJsonbString => quote::quote! {
                    uuid::Uuid::new_v4()
                },
            }
        }
        fn field_type_token_stream(&self) -> proc_macro2::TokenStream {
            match &self {
                Self::StdPrimitiveI8AsJsonbNumber => quote::quote!{std::primitive::i8},
                Self::StdPrimitiveI16AsJsonbNumber => quote::quote!{std::primitive::i16},
                Self::StdPrimitiveI32AsJsonbNumber => quote::quote!{std::primitive::i32},
                Self::StdPrimitiveI64AsJsonbNumber => quote::quote!{std::primitive::i64},
                Self::StdPrimitiveU8AsJsonbNumber => quote::quote!{std::primitive::u8},
                Self::StdPrimitiveU16AsJsonbNumber => quote::quote!{std::primitive::u16},
                Self::StdPrimitiveU32AsJsonbNumber => quote::quote!{std::primitive::u32},
                Self::StdPrimitiveU64AsJsonbNumber => quote::quote!{std::primitive::u64},
                Self::StdPrimitiveF32AsJsonbNumber => quote::quote!{std::primitive::f32},
                Self::StdPrimitiveF64AsJsonbNumber => quote::quote!{std::primitive::f64},
                Self::StdPrimitiveBoolAsJsonbBoolean => quote::quote!{std::primitive::bool},
                Self::StdStringStringAsJsonbString => quote::quote!{std::string::String},
                Self::UuidUuidAsJsonbString => quote::quote!{uuid::Uuid},
            }
        }
    }
    impl quote::ToTokens for PostgresqlJsonType {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string().parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("failed to parse PostgresqlJsonType to proc_macro2::TokenStream")).to_tokens(tokens)
        }
    }
    //todo maybe reuse if sqlx maintainers would support nexted arrays in the future versions
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    pub enum PostgresqlJsonTypePattern {
        Standart,
        ArrayDimension1 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        // ArrayDimension2 {
        //     dimension1_not_null_or_nullable: NotNullOrNullable,
        //     dimension2_not_null_or_nullable: NotNullOrNullable,
        // },
        // ArrayDimension3 {
        //     dimension1_not_null_or_nullable: NotNullOrNullable,
        //     dimension2_not_null_or_nullable: NotNullOrNullable,
        //     dimension3_not_null_or_nullable: NotNullOrNullable,
        // },
        // ArrayDimension4 {
        //     dimension1_not_null_or_nullable: NotNullOrNullable,
        //     dimension2_not_null_or_nullable: NotNullOrNullable,
        //     dimension3_not_null_or_nullable: NotNullOrNullable,
        //     dimension4_not_null_or_nullable: NotNullOrNullable,
        // },
    }
    impl PostgresqlJsonTypePattern {
        pub fn prefix_stringified(&self) -> std::string::String {
            match &self {
                Self::Standart => std::string::String::default(),
                Self::ArrayDimension1 {..} => naming::VecUpperCamelCase.to_string(),
            }
        }
        fn array_dimensions_number(&self) -> std::primitive::usize {
            match &self {
                PostgresqlJsonTypePattern::Standart => 0,
                PostgresqlJsonTypePattern::ArrayDimension1 { .. } => 1,
                // PostgresqlJsonTypePattern::ArrayDimension2 { .. } => 2,
                // PostgresqlJsonTypePattern::ArrayDimension3 { .. } => 3,
                // PostgresqlJsonTypePattern::ArrayDimension4 { .. } => 4,
            }
        }
        fn all_variants() -> std::vec::Vec<Self> {
            Self::into_array().into_iter().fold(vec![], |mut acc, postgresql_type_pattern| {
                match &postgresql_type_pattern {
                    Self::Standart => {
                        acc.push(postgresql_type_pattern);
                    },
                    Self::ArrayDimension1 {..} => {
                        postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                            acc.push(Self::ArrayDimension1 {
                                dimension1_not_null_or_nullable,
                             });
                        });
                    },
                    // Self::ArrayDimension2 {..} => {
                    //     postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                    //         postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension2_not_null_or_nullable| {
                    //             acc.push(Self::ArrayDimension2 {
                    //                 dimension1_not_null_or_nullable,
                    //                 dimension2_not_null_or_nullable,
                    //              });
                    //         });
                    //     });
                    // },
                    // Self::ArrayDimension3 {..} => {
                    //     postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                    //         postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension2_not_null_or_nullable| {
                    //             postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension3_not_null_or_nullable| {
                    //                 acc.push(Self::ArrayDimension3 {
                    //                     dimension1_not_null_or_nullable,
                    //                     dimension2_not_null_or_nullable,
                    //                     dimension3_not_null_or_nullable,
                    //                 });
                    //             });
                    //         });
                    //     });
                    // },
                    // Self::ArrayDimension4 {..} => {
                    //     postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension1_not_null_or_nullable| {
                    //         postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension2_not_null_or_nullable| {
                    //             postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension3_not_null_or_nullable| {
                    //                 postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|dimension4_not_null_or_nullable| {
                    //                     acc.push(Self::ArrayDimension4 {
                    //                         dimension1_not_null_or_nullable,
                    //                         dimension2_not_null_or_nullable,
                    //                         dimension3_not_null_or_nullable,
                    //                         dimension4_not_null_or_nullable,
                    //                     });
                    //                 });
                    //             });
                    //         });
                    //     });
                    // },
                }
                acc
            })
        }
    }
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PostgresqlJsonTypeRecord {
        pub postgresql_json_type: PostgresqlJsonType,
        pub not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        pub postgresql_json_type_pattern: PostgresqlJsonTypePattern,
    }
    impl PostgresqlJsonTypeRecord {
        pub fn all() -> std::vec::Vec<Self> {
            PostgresqlJsonType::into_array().into_iter().fold(vec![], |mut acc, postgresql_json_type| {
                postgresql_crud_macros_common::NotNullOrNullable::into_array().into_iter().for_each(|not_null_or_nullable| {
                    PostgresqlJsonTypePattern::all_variants().into_iter().for_each(|postgresql_json_type_pattern| {
                        acc.push(PostgresqlJsonTypeRecord {
                            postgresql_json_type: postgresql_json_type.clone(),
                            not_null_or_nullable: not_null_or_nullable.clone(),
                            postgresql_json_type_pattern,
                        });
                    });
                });
                acc
            })
        }
        pub fn is_vec_element_type(&self) -> std::primitive::bool {
            match &self.postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::Standart => false,
                //todo maybe wrong
                PostgresqlJsonTypePattern::ArrayDimension1 {..} => true,
            }
        }
        pub fn handle_field_type(&self, is_wrapper: std::primitive::bool) -> proc_macro2::TokenStream {
            let postgresql_json_type = &self.postgresql_json_type;
            match (&self.not_null_or_nullable, &self.postgresql_json_type_pattern) {
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) => {
                    if is_wrapper {
                        quote::quote! {#postgresql_json_type}
                    } else {
                        match &postgresql_json_type {
                            PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => quote::quote!{std::primitive::i8},
                            PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => quote::quote!{std::primitive::i16},
                            PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => quote::quote!{std::primitive::i32},
                            PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => quote::quote!{std::primitive::i64},
                            PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => quote::quote!{std::primitive::u8},
                            PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => quote::quote!{std::primitive::u16},
                            PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => quote::quote!{std::primitive::u32},
                            PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => quote::quote!{std::primitive::u64},
                            PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => quote::quote!{std::primitive::f32},
                            PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => quote::quote!{std::primitive::f64},
                            PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => quote::quote!{std::primitive::bool},
                            PostgresqlJsonType::StdStringStringAsJsonbString => quote::quote!{std::string::String},
                            PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote!{uuid::Uuid},
                        }
                    }
                }
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::Standart) => quote::quote! {std::option::Option<#postgresql_json_type>},
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension1 {..}) => quote::quote! {std::vec::Vec<#postgresql_json_type>},
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension1 {..}) => {
                    quote::quote! {std::option::Option<std::vec::Vec<#postgresql_json_type>>}
                }
            }
        }
        pub fn handle_initialization_token_stream(&self, is_wrapper: std::primitive::bool) -> proc_macro2::TokenStream {
            let postgresql_json_type = &self.postgresql_json_type;
            let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream =
                token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
            match (&self.not_null_or_nullable, &self.postgresql_json_type_pattern) {
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) => {
                    if is_wrapper {
                        quote::quote! {#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream}
                    } else {
                        postgresql_json_type.full_type_path_initialization_token_stream()
                    }
                }
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::Standart) => quote::quote! {Some(#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension1 {..}) => quote::quote! {vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]},
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension1 {..}) => {
                    quote::quote! {Some(vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])}
                }
            }
        }
        pub fn initialization_token_stream(&self) -> proc_macro2::TokenStream {
            self.handle_initialization_token_stream(false)
        }
    }

    let postgresql_json_type_record_vec = 
    // PostgresqlJsonTypeRecord::all()
    {
        let vec = serde_json::from_str::<std::vec::Vec<PostgresqlJsonTypeRecord>>(&input_token_stream.to_string())
        .expect("failed to get Config for generate_postgresql_json_type");
        let mut acc = vec![];
        for element in &vec {
            if acc.contains(&element) {
                panic!("not unique postgersql type provided: {element:#?}");
            }
            else {
                acc.push(&element);
            }
        }
        vec
    }
    .into_iter()
    .filter(|element|{
        let postgresql_type_filter = match &element.postgresql_json_type {
            PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => true,
            PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => true,
            PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => true,
            PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => true,
            PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => true,
            PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => true,
            PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => true,
            PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => true,
            PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => true,
            PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => true,
            PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => true,
            PostgresqlJsonType::StdStringStringAsJsonbString => true,
            PostgresqlJsonType::UuidUuidAsJsonbString => true,
        };
        let not_null_or_nullable_filter = match &element.not_null_or_nullable {
            postgresql_crud_macros_common::NotNullOrNullable::NotNull => true,
            postgresql_crud_macros_common::NotNullOrNullable::Nullable => true,
        };
        let postgresql_type_pattern_filter = match &element.postgresql_json_type_pattern {
            PostgresqlJsonTypePattern::Standart => true,
            PostgresqlJsonTypePattern::ArrayDimension1 {
                dimension1_not_null_or_nullable,
            } => match &dimension1_not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => true,
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => true,
            },
            // PostgresqlJsonTypePattern::ArrayDimension2 {
            //     dimension1_not_null_or_nullable,
            //     dimension2_not_null_or_nullable,
            // } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable) {
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            // },
            // PostgresqlJsonTypePattern::ArrayDimension3 {
            //     dimension1_not_null_or_nullable,
            //     dimension2_not_null_or_nullable,
            //     dimension3_not_null_or_nullable,
            // } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable) {
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            // },
            // PostgresqlJsonTypePattern::ArrayDimension4 {
            //     dimension1_not_null_or_nullable,
            //     dimension2_not_null_or_nullable,
            //     dimension3_not_null_or_nullable,
            //     dimension4_not_null_or_nullable,
            // } => match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => false,
            //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => false,
            // }
        };
        postgresql_type_filter && not_null_or_nullable_filter && postgresql_type_pattern_filter
    })
    .collect::<std::vec::Vec<PostgresqlJsonTypeRecord>>();
    // macros_helpers::write_string_into_file::write_string_into_file(
    //     "GeneratePostgresqlJsonTypesJsonVariants",
    //     &serde_json::to_string(&postgresql_json_type_record_vec).unwrap(),
    // );
    use rayon::iter::ParallelIterator;
    use rayon::iter::IntoParallelRefIterator;
    let (
        postgresql_crud_json_object_rust_struct_fields_token_stream,
        postgresql_json_type_array
    ) = postgresql_json_type_record_vec
    .par_iter()
    // .into_iter()//just for console prints ordering
    .map(|element|{
        let postgresql_json_type = &element.postgresql_json_type;
        let postgresql_json_type_pattern = &element.postgresql_json_type_pattern;
        let not_null_or_nullable = &element.not_null_or_nullable;

        let rust_type_name = RustTypeName::from(postgresql_json_type);
        let postgresql_json_type_name = PostgresqlJsonTypeName::from(postgresql_json_type);
        let array_dimensions_number = postgresql_json_type_pattern.array_dimensions_number();

        let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();

        let column_snake_case = naming::ColumnSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let value_snake_case = naming::ValueSnakeCase;
        let self_snake_case = naming::SelfSnakeCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let start_snake_case = naming::StartSnakeCase;
        let end_snake_case = naming::EndSnakeCase;
        let digits_snake_case = naming::DigitsSnakeCase;
        let scale_snake_case = naming::ScaleSnakeCase;
        let year_snake_case = naming::YearSnakeCase;
        let month_snake_case = naming::MonthSnakeCase;
        let day_snake_case = naming::DaySnakeCase;
        let months_snake_case = naming::MonthsSnakeCase;
        let days_snake_case = naming::DaysSnakeCase;
        let microseconds_snake_case = naming::MicrosecondsSnakeCase;
        let as_upper_camel_case = naming::AsUpperCamelCase;
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let none_upper_camel_case = naming::NoneUpperCamelCase;

        let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
        let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
        let std_primitive_u8_token_stream = token_patterns::StdPrimitiveU8;
        let std_string_string_token_stream = token_patterns::StdStringString;

        let core_default_default_default_token_stream = token_patterns::CoreDefaultDefaultDefault;
        let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;

        let postgresql_crud_macros_common_import_path_crate = postgresql_crud_macros_common::ImportPath::Crate;

        let generate_ident_not_null_token_stream = |postgresql_json_type: &PostgresqlJsonType|{
            let rust_type_name = RustTypeName::from(postgresql_json_type);
            let postgresql_json_type_name = PostgresqlJsonTypeName::from(postgresql_json_type);
            let not_null = postgresql_crud_macros_common::NotNullOrNullable::NotNull;
            let not_null_rust = not_null.rust();
            format!("{not_null_rust}{rust_type_name}{as_upper_camel_case}{not_null}{postgresql_json_type_name}")
            .parse::<proc_macro2::TokenStream>().unwrap()
        };
        let ident_standart_not_null_upper_camel_case = generate_ident_not_null_token_stream(&postgresql_json_type);
        let generate_ident_token_stream = |postgresql_json_type_pattern: &PostgresqlJsonTypePattern, not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable|{
            let vec_of_upper_camel_case = naming::VecOfUpperCamelCase;
            let array_of_upper_camel_case = naming::ArrayOfUpperCamelCase;
            let not_null_or_nullable_rust = not_null_or_nullable.rust();
            match (&postgresql_json_type_pattern, &not_null_or_nullable) {
                (
                    PostgresqlJsonTypePattern::Standart,
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull
                ) => quote::quote!{#ident_standart_not_null_upper_camel_case},
                (
                    PostgresqlJsonTypePattern::Standart,
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable
                ) => {
                    format!("{not_null_or_nullable_rust}{rust_type_name}{as_upper_camel_case}{not_null_or_nullable}{postgresql_json_type_name}")
                    .parse::<proc_macro2::TokenStream>().unwrap()
                },
                (
                    PostgresqlJsonTypePattern::ArrayDimension1 {
                        dimension1_not_null_or_nullable,
                    },
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull
                ) => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    format!("{not_null_or_nullable_rust}{vec_of_upper_camel_case}{d1_rust}{rust_type_name}{as_upper_camel_case}{not_null_or_nullable}{array_of_upper_camel_case}{d1}{postgresql_json_type_name}")
                    .parse::<proc_macro2::TokenStream>().unwrap()
                },
                (
                    PostgresqlJsonTypePattern::ArrayDimension1 {
                        dimension1_not_null_or_nullable,
                    },
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable
                ) => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    format!("{not_null_or_nullable_rust}{vec_of_upper_camel_case}{d1_rust}{rust_type_name}{as_upper_camel_case}{not_null_or_nullable}{array_of_upper_camel_case}{d1}{postgresql_json_type_name}")
                    .parse::<proc_macro2::TokenStream>().unwrap()
                },
                // (
                //     PostgresqlJsonTypePattern::ArrayDimension2 {
                //         dimension1_not_null_or_nullable,
                //         dimension2_not_null_or_nullable,
                //     },
                //     postgresql_crud_macros_common::NotNullOrNullable::NotNull
                // ) => {
                //     let d1 = dimension1_not_null_or_nullable;
                //     let d1_rust = dimension1_not_null_or_nullable.rust();
                //     let d2 = dimension2_not_null_or_nullable;
                //     let d2_rust = dimension2_not_null_or_nullable.rust();
                //     format!("{not_null_or_nullable_rust}{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{rust_type_name}{as_upper_camel_case}{not_null_or_nullable}{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{postgresql_type_name}")
                //     .parse::<proc_macro2::TokenStream>().unwrap()
                // },
                // (
                //     PostgresqlJsonTypePattern::ArrayDimension2 {
                //         dimension1_not_null_or_nullable,
                //         dimension2_not_null_or_nullable,
                //     },
                //     postgresql_crud_macros_common::NotNullOrNullable::Nullable
                // ) => {
                //     let d1 = dimension1_not_null_or_nullable;
                //     let d1_rust = dimension2_not_null_or_nullable.rust();
                //     let d2 = dimension2_not_null_or_nullable;
                //     let d2_rust = dimension2_not_null_or_nullable.rust();
                //     format!("{not_null_or_nullable_rust}{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{rust_type_name}{as_upper_camel_case}{not_null_or_nullable}{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{postgresql_type_name}")
                //     .parse::<proc_macro2::TokenStream>().unwrap()
                // },
                // (
                //     PostgresqlJsonTypePattern::ArrayDimension3 {
                //         dimension1_not_null_or_nullable,
                //         dimension2_not_null_or_nullable,
                //         dimension3_not_null_or_nullable,
                //     },
                //     postgresql_crud_macros_common::NotNullOrNullable::NotNull
                // ) => {
                //     let d1 = dimension1_not_null_or_nullable;
                //     let d1_rust = dimension1_not_null_or_nullable.rust();
                //     let d2 = dimension2_not_null_or_nullable;
                //     let d2_rust = dimension2_not_null_or_nullable.rust();
                //     let d3 = dimension3_not_null_or_nullable;
                //     let d3_rust = dimension3_not_null_or_nullable.rust();
                //     format!("{not_null_or_nullable_rust}{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{rust_type_name}{as_upper_camel_case}{not_null_or_nullable}{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{postgresql_type_name}")
                //     .parse::<proc_macro2::TokenStream>().unwrap()
                // },
                // (
                //     PostgresqlJsonTypePattern::ArrayDimension3 {
                //         dimension1_not_null_or_nullable,
                //         dimension2_not_null_or_nullable,
                //         dimension3_not_null_or_nullable,
                //     },
                //     postgresql_crud_macros_common::NotNullOrNullable::Nullable
                // ) => {
                //     let d1 = dimension1_not_null_or_nullable;
                //     let d1_rust = dimension1_not_null_or_nullable.rust();
                //     let d2 = dimension2_not_null_or_nullable;
                //     let d2_rust = dimension2_not_null_or_nullable.rust();
                //     let d3 = dimension3_not_null_or_nullable;
                //     let d3_rust = dimension3_not_null_or_nullable.rust();
                //     format!("{not_null_or_nullable_rust}{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{rust_type_name}{as_upper_camel_case}{not_null_or_nullable}{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{postgresql_type_name}")
                //     .parse::<proc_macro2::TokenStream>().unwrap()
                // },
                // (
                //     PostgresqlJsonTypePattern::ArrayDimension4 {
                //         dimension1_not_null_or_nullable,
                //         dimension2_not_null_or_nullable,
                //         dimension3_not_null_or_nullable,
                //         dimension4_not_null_or_nullable,
                //     },
                //     postgresql_crud_macros_common::NotNullOrNullable::NotNull
                // ) => {
                //     let d1 = dimension1_not_null_or_nullable;
                //     let d1_rust = dimension1_not_null_or_nullable.rust();
                //     let d2 = dimension2_not_null_or_nullable;
                //     let d2_rust = dimension2_not_null_or_nullable.rust();
                //     let d3 = dimension3_not_null_or_nullable;
                //     let d3_rust = dimension3_not_null_or_nullable.rust();
                //     let d4 = dimension4_not_null_or_nullable;
                //     let d4_rust = dimension4_not_null_or_nullable.rust();
                //     format!("{not_null_or_nullable_rust}{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{vec_of_upper_camel_case}{d4_rust}{rust_type_name}{as_upper_camel_case}{not_null_or_nullable}{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{array_of_upper_camel_case}{d4}{postgresql_type_name}")
                //     .parse::<proc_macro2::TokenStream>().unwrap()
                // },
                // (
                //     PostgresqlJsonTypePattern::ArrayDimension4 {
                //         dimension1_not_null_or_nullable,
                //         dimension2_not_null_or_nullable,
                //         dimension3_not_null_or_nullable,
                //         dimension4_not_null_or_nullable,
                //     },
                //     postgresql_crud_macros_common::NotNullOrNullable::Nullable
                // ) => {
                //     let d1 = dimension1_not_null_or_nullable;
                //     let d1_rust = dimension1_not_null_or_nullable.rust();
                //     let d2 = dimension2_not_null_or_nullable;
                //     let d2_rust = dimension2_not_null_or_nullable.rust();
                //     let d3 = dimension3_not_null_or_nullable;
                //     let d3_rust = dimension3_not_null_or_nullable.rust();
                //     let d4 = dimension4_not_null_or_nullable;
                //     let d4_rust = dimension4_not_null_or_nullable.rust();
                //     format!("{not_null_or_nullable_rust}{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{vec_of_upper_camel_case}{d4_rust}{rust_type_name}{as_upper_camel_case}{not_null_or_nullable}{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{array_of_upper_camel_case}{d4}{postgresql_type_name}")
                //     .parse::<proc_macro2::TokenStream>().unwrap()
                // },
            }
        };
        let ident = &generate_ident_token_stream(&postgresql_json_type_pattern, &not_null_or_nullable);
        let ident_token_stream = quote::quote! {
            #[derive(Debug)]
            pub struct #ident;
        };
        let ident_standart_not_null_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
        let ident_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident);

        let field_type = postgresql_json_type.field_type_token_stream();
        let generate_current_ident_origin_non_wrapping = |current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern, current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable|{
            naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(
                &current_postgresql_json_type_pattern,
                &current_not_null_or_nullable
            ))
        };
        let field_type_handle: &dyn quote::ToTokens = {
            let generate_current_ident_origin = |current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern, current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable|{
                let value = generate_current_ident_origin_non_wrapping(
                    &current_postgresql_json_type_pattern,
                    &current_not_null_or_nullable
                );
                match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&value),
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&value)
                }
            };
            match &postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => &field_type,
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_standart_not_null_origin_upper_camel_case)
                },
                PostgresqlJsonTypePattern::ArrayDimension1 {
                    dimension1_not_null_or_nullable,
                } => &{
                    let (
                        current_postgresql_json_type_pattern,
                        current_not_null_or_nullable,
                    ): (
                        &PostgresqlJsonTypePattern,
                        &postgresql_crud_macros_common::NotNullOrNullable,
                    ) = match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                            &PostgresqlJsonTypePattern::Standart,
                            &dimension1_not_null_or_nullable,
                        ),
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                            &postgresql_json_type_pattern,
                            &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                        )
                    };
                    generate_current_ident_origin(
                        &current_postgresql_json_type_pattern,
                        &current_not_null_or_nullable
                    )
                },
                // PostgresqlJsonTypePattern::ArrayDimension2 {
                //     dimension1_not_null_or_nullable,
                //     dimension2_not_null_or_nullable,
                // } => &{
                //     let (
                //         current_postgresql_json_type_pattern,
                //         current_not_null_or_nullable,
                //     ): (
                //         &PostgresqlJsonTypePattern,
                //         &postgresql_crud_macros_common::NotNullOrNullable,
                //     ) = match &not_null_or_nullable {
                //         postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                //             &PostgresqlJsonTypePattern::ArrayDimension1 {
                //                 dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                //             },
                //             &dimension1_not_null_or_nullable,
                //         ),
                //         postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                //             &postgresql_type_pattern,
                //             &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                //         )
                //     };
                //     generate_current_ident_origin(
                //         &current_postgresql_json_type_pattern,
                //         &current_not_null_or_nullable
                //     )
                // },
                // PostgresqlJsonTypePattern::ArrayDimension3 {
                //     dimension1_not_null_or_nullable,
                //     dimension2_not_null_or_nullable,
                //     dimension3_not_null_or_nullable,
                // } => &{
                //     let (
                //         current_postgresql_json_type_pattern,
                //         current_not_null_or_nullable,
                //     ): (
                //         &PostgresqlJsonTypePattern,
                //         &postgresql_crud_macros_common::NotNullOrNullable,
                //     ) = match &not_null_or_nullable {
                //         postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                //             &PostgresqlJsonTypePattern::ArrayDimension2 {
                //                 dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                //                 dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                //             },
                //             &dimension1_not_null_or_nullable,
                //         ),
                //         postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                //             &postgresql_type_pattern,
                //             &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                //         )
                //     };
                //     generate_current_ident_origin(
                //         &current_postgresql_json_type_pattern,
                //         &current_not_null_or_nullable
                //     )
                // },
                // PostgresqlJsonTypePattern::ArrayDimension4 {
                //     dimension1_not_null_or_nullable,
                //     dimension2_not_null_or_nullable,
                //     dimension3_not_null_or_nullable,
                //     dimension4_not_null_or_nullable,
                // } => &{
                //     let (
                //         current_postgresql_json_type_pattern,
                //         current_not_null_or_nullable,
                //     ): (
                //         &PostgresqlJsonTypePattern,
                //         &postgresql_crud_macros_common::NotNullOrNullable,
                //     ) = match &not_null_or_nullable {
                //         postgresql_crud_macros_common::NotNullOrNullable::NotNull => (
                //             &PostgresqlJsonTypePattern::ArrayDimension3 {
                //                 dimension1_not_null_or_nullable: dimension2_not_null_or_nullable.clone(),
                //                 dimension2_not_null_or_nullable: dimension3_not_null_or_nullable.clone(),
                //                 dimension3_not_null_or_nullable: dimension4_not_null_or_nullable.clone(),
                //             },
                //             &dimension1_not_null_or_nullable,
                //         ),
                //         postgresql_crud_macros_common::NotNullOrNullable::Nullable => (
                //             &postgresql_type_pattern,
                //             &postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                //         )
                //     };
                //     generate_current_ident_origin(
                //         &current_postgresql_json_type_pattern,
                //         &current_not_null_or_nullable
                //     )
                // },
            }
        };
        // println!("{}", quote::quote!{#field_type_handle});

        let ident_origin_token_stream = {
            let schema_name_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&ident_origin_upper_camel_case);
            let metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream = quote::quote!{
                Some(Box::new(schemars::schema::Metadata {
                    id: None,
                    title: Some(#schema_name_format_handle_token_stream.to_owned()),
                    description: None,
                    default: None,
                    deprecated: false,
                    read_only: false,
                    write_only: false,
                    examples: std::vec::Vec::default(),
                }))
            };
            let extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream = quote::quote!{schemars::Map::default()};
            struct SchemaObjectTokenStream<'a> {
                metadata: &'a dyn quote::ToTokens,
                instance_type: &'a dyn quote::ToTokens,
                format: &'a dyn quote::ToTokens,
                enum_values: &'a dyn quote::ToTokens,
                const_value: &'a dyn quote::ToTokens,
                subschemas: &'a dyn quote::ToTokens,
                number: &'a dyn quote::ToTokens,
                string: &'a dyn quote::ToTokens,
                array: &'a dyn quote::ToTokens,
                object: &'a dyn quote::ToTokens,
                reference: &'a dyn quote::ToTokens,
                extensions: &'a dyn quote::ToTokens,
            }
            enum SchemarsJsonSchema<'a> {
                Derive,
                Impl(SchemaObjectTokenStream<'a>)
            }
            let (
                instance_type_number_token_stream,
                instance_type_string_token_stream
            ) = {
                let generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type = |instance_type: &schemars::schema::InstanceType|{
                    let instance_type_token_stream: &dyn quote::ToTokens = match &instance_type {
                        schemars::schema::InstanceType::Null => &naming::NullUpperCamelCase,
                        schemars::schema::InstanceType::Boolean => &naming::BooleanUpperCamelCase,
                        schemars::schema::InstanceType::Object => &naming::ObjectUpperCamelCase,
                        schemars::schema::InstanceType::Array => &naming::ArrayUpperCamelCase,
                        schemars::schema::InstanceType::Number => &naming::NumberUpperCamelCase,
                        schemars::schema::InstanceType::String => &naming::StringUpperCamelCase,
                        schemars::schema::InstanceType::Integer => &naming::IntegerUpperCamelCase,
                    };
                    quote::quote!{Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::#instance_type_token_stream)))}
                };
                (
                    generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type(&schemars::schema::InstanceType::Number),
                    generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type(&schemars::schema::InstanceType::String)
                )
            };
            let number_token_stream = quote::quote!{Some(Box::new(schemars::schema::NumberValidation {
                multiple_of: None,
                maximum: Some(#field_type ::MAX as std::primitive::f64),
                exclusive_maximum: None,
                minimum: Some(#field_type ::MIN as std::primitive::f64),
                exclusive_minimum: None,
            }))};
            let string_token_stream = quote::quote!{Some(Box::new(schemars::schema::StringValidation {
                max_length: Some(36),
                min_length: Some(36),
                pattern: None,
            }))};
            let schemars_json_schema = {
                let schema_object_token_stream_integer = SchemaObjectTokenStream {
                    metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream,
                    instance_type: &instance_type_number_token_stream,
                    format: &none_upper_camel_case,
                    enum_values: &none_upper_camel_case,
                    const_value: &none_upper_camel_case,
                    subschemas: &none_upper_camel_case,
                    number: &number_token_stream,
                    string: &none_upper_camel_case,
                    array: &none_upper_camel_case,
                    object: &none_upper_camel_case,
                    reference: &none_upper_camel_case,
                    extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream,
                };
                match (&not_null_or_nullable, &postgresql_json_type_pattern) {
                    (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) => match &postgresql_json_type {
                        PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                        PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                        PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                        PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                        PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                        PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                        PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                        PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                        PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                        | PostgresqlJsonType::StdStringStringAsJsonbString => SchemarsJsonSchema::Derive,
                        PostgresqlJsonType::UuidUuidAsJsonbString => SchemarsJsonSchema::Impl(SchemaObjectTokenStream {
                            metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream,
                            instance_type: &instance_type_string_token_stream,
                            format: &none_upper_camel_case,
                            enum_values: &none_upper_camel_case,
                            const_value: &none_upper_camel_case,
                            subschemas: &none_upper_camel_case,
                            number: &none_upper_camel_case,
                            string: &string_token_stream,
                            array: &none_upper_camel_case,
                            object: &none_upper_camel_case,
                            reference: &none_upper_camel_case,
                            extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream,
                        }),
                    },
                    (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::Standart)
                    | (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension1 {..})
                    | (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension1 {..}) => SchemarsJsonSchema::Derive
                }
            };

            let ident_origin_token_stream = {
                let serde_serialize_comma_token_stream = quote::quote! {serde::Serialize,};
                let serde_deserialize_comma_token_stream = quote::quote! {serde::Deserialize,};
                let (
                    maybe_derive_partial_ord_token_stream,
                    maybe_derive_serde_serialize_token_stream,
                    maybe_derive_serde_deserialize_token_stream
                ) = if let (
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                    PostgresqlJsonTypePattern::Standart
                ) = (
                    &not_null_or_nullable,
                    &postgresql_json_type_pattern
                ) {
                    let maybe_derive_partial_ord_token_stream = {
                        let partial_ord_comma_token_stream = quote::quote! {PartialOrd,};
                        match &postgresql_json_type {
                            PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => partial_ord_comma_token_stream,
                            PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => partial_ord_comma_token_stream,
                            PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => partial_ord_comma_token_stream,
                            PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => partial_ord_comma_token_stream,
                            PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => partial_ord_comma_token_stream,
                            PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => partial_ord_comma_token_stream,
                            PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => partial_ord_comma_token_stream,
                            PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => partial_ord_comma_token_stream,
                            PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => partial_ord_comma_token_stream,
                            PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => partial_ord_comma_token_stream,
                            PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => partial_ord_comma_token_stream,
                            PostgresqlJsonType::StdStringStringAsJsonbString => partial_ord_comma_token_stream,
                            PostgresqlJsonType::UuidUuidAsJsonbString => partial_ord_comma_token_stream,
                        }
                    };
                    let maybe_derive_serde_serialize_token_stream = match &postgresql_json_type {
                        PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::StdStringStringAsJsonbString => serde_serialize_comma_token_stream,
                        PostgresqlJsonType::UuidUuidAsJsonbString => serde_serialize_comma_token_stream,
                    };
                    let maybe_derive_serde_deserialize_token_stream = match &postgresql_json_type {
                        PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::StdStringStringAsJsonbString => serde_deserialize_comma_token_stream,
                        PostgresqlJsonType::UuidUuidAsJsonbString => serde_deserialize_comma_token_stream,
                    };
                    (
                        maybe_derive_partial_ord_token_stream,
                        maybe_derive_serde_serialize_token_stream,
                        maybe_derive_serde_deserialize_token_stream
                    )
                }
                else {
                    (
                        proc_macro2::TokenStream::new(),
                        serde_serialize_comma_token_stream,
                        serde_deserialize_comma_token_stream
                    )
                };
                let maybe_derive_schemars_json_schema_token_stream: &dyn quote::ToTokens = match &schemars_json_schema {
                    SchemarsJsonSchema::Derive => &quote::quote! {schemars::JsonSchema,},
                    SchemarsJsonSchema::Impl(_) => &proc_macro2_token_stream_new,
                };
                quote::quote! {
                    #[derive(
                        Debug,
                        Clone,
                        PartialEq,
                        #maybe_derive_partial_ord_token_stream
                        Default,
                        #maybe_derive_serde_serialize_token_stream
                        #maybe_derive_serde_deserialize_token_stream
                        utoipa::ToSchema,
                        #maybe_derive_schemars_json_schema_token_stream
                    )]
                    pub struct #ident_origin_upper_camel_case(#field_type_handle);
                }
            };
            let maybe_impl_schemars_json_schema_for_ident_origin_token_stream: &dyn quote::ToTokens = match &schemars_json_schema {
                SchemarsJsonSchema::Derive => &proc_macro2_token_stream_new,
                SchemarsJsonSchema::Impl(schema_object_token_stream) => &{
                    let schema_id_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("postgersql_crud::postgersql_json_type::{ident_origin_upper_camel_case}"));
                    let metadata_token_stream = &schema_object_token_stream.metadata;
                    let instance_type_token_stream = &schema_object_token_stream.instance_type;
                    let format_token_stream = &schema_object_token_stream.format;
                    let enum_values_token_stream = &schema_object_token_stream.enum_values;
                    let const_value_token_stream = &schema_object_token_stream.const_value;
                    let subschemas_token_stream = &schema_object_token_stream.subschemas;
                    let number_token_stream = &schema_object_token_stream.number;
                    let string_token_stream = &schema_object_token_stream.string;
                    let array_token_stream = &schema_object_token_stream.array;
                    let object_token_stream = &schema_object_token_stream.object;
                    let reference_token_stream = &schema_object_token_stream.reference;
                    let extensions_token_stream = &schema_object_token_stream.extensions;
                    quote::quote! {
                        impl schemars::JsonSchema for #ident_origin_upper_camel_case {
                            fn schema_name() -> std::string::String {
                                #schema_name_format_handle_token_stream.to_owned()
                            }
                            fn schema_id() -> std::borrow::Cow<'static, str> {
                                std::borrow::Cow::Borrowed(#schema_id_format_handle_token_stream)
                            }
                            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
                                {
                                    schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                                        metadata: #metadata_token_stream,
                                        instance_type: #instance_type_token_stream,
                                        format: #format_token_stream,
                                        enum_values: #enum_values_token_stream,
                                        const_value: #const_value_token_stream,
                                        subschemas: #subschemas_token_stream,
                                        number: #number_token_stream,
                                        string: #string_token_stream,
                                        array: #array_token_stream,
                                        object: #object_token_stream,
                                        reference: #reference_token_stream,
                                        extensions: #extensions_token_stream,
                                    })
                                }
                            }
                        }
                    }
                }
            };
            let maybe_impl_is_string_empty_for_ident_origin_token_stream: &dyn quote::ToTokens = match (&not_null_or_nullable, &postgresql_json_type_pattern) {
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) => match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => &proc_macro2_token_stream_new,
                    PostgresqlJsonType::StdStringStringAsJsonbString => &postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_token_stream(&ident_origin_upper_camel_case),
                    PostgresqlJsonType::UuidUuidAsJsonbString => &postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_token_stream(&ident_origin_upper_camel_case),
                },
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::Standart) => match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                    | PostgresqlJsonType::StdStringStringAsJsonbString
                    | PostgresqlJsonType::UuidUuidAsJsonbString => &proc_macro2_token_stream_new,
                },
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension1 {..}) => match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber
                    | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean
                    | PostgresqlJsonType::StdStringStringAsJsonbString
                    | PostgresqlJsonType::UuidUuidAsJsonbString => &proc_macro2_token_stream_new,
                },
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension1 {..}) => &proc_macro2_token_stream_new,
            };
            let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                &ident_origin_upper_camel_case,
                &{
                    let content_token_stream = element.initialization_token_stream();
                    quote::quote! {
                        Self(#content_token_stream)
                    }
                }
            );
            let impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(
                &proc_macro2::TokenStream::new(),
                &ident_origin_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {format!("{self:#?}")}
            );
            let impl_sqlx_type_sqlx_postgres_for_ident_origin_token_stream = {
                let sqlx_types_json_type_field_type_token_stream = postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&field_type);
                quote::quote!{
                    impl sqlx::Type<sqlx::Postgres> for #ident_origin_upper_camel_case {
                        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                            <#sqlx_types_json_type_field_type_token_stream as sqlx::Type<sqlx::Postgres>>::type_info()
                        }
                        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
                            <#sqlx_types_json_type_field_type_token_stream as sqlx::Type<sqlx::Postgres>>::compatible(ty)
                        }
                    }
                }
            };
            let impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream = {
                quote::quote!{
                    impl sqlx::Encode<'_, sqlx::Postgres> for #ident_origin_upper_camel_case {
                        fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self.0), buf)
                        }
                    }
                }
            };
            quote::quote! {
                #ident_origin_token_stream
                #maybe_impl_schemars_json_schema_for_ident_origin_token_stream
                #maybe_impl_is_string_empty_for_ident_origin_token_stream
                #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream
                #impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_origin_token_stream
                #impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream
            }
        };

        let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
        let ident_select_token_stream = {
            let ident_select_token_stream = {
                let content_token_stream = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => quote::quote! {;},
                    PostgresqlJsonTypePattern::ArrayDimension1 {..} => quote::quote! {{ pagination: crate::pagination::Pagination }},
                };
                quote::quote! {
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
                    pub struct #ident_select_upper_camel_case #content_token_stream
                }
            };
            let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream =
                postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_select_upper_camel_case, &{
                    let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
                    match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => {
                            let core_default_default_default = token_patterns::CoreDefaultDefaultDefault;
                            quote::quote! {
                                #core_default_default_default
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension1 {..} => {
                            quote::quote! {
                                Self {
                                    pagination: #crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                }
                            }
                        }
                    }
                });
            quote::quote! {
                #ident_select_token_stream
                #impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream
            }
        };
        let ident_where_element_upper_camel_case = naming::parameter::SelfWhereElementUpperCamelCase::from_tokens(&ident);
        let ident_where_element_token_stream = {
            #[derive(Debug, Clone)]
            enum PostgresqlJsonTypeSpecific {
                Number,
                Bool,
                String,
            }
            impl std::convert::From<&PostgresqlJsonType> for PostgresqlJsonTypeSpecific {
                fn from(value: &PostgresqlJsonType) -> Self {
                    match value {
                        PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber
                        | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => Self::Number,
                        PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => Self::Bool,
                        PostgresqlJsonType::StdStringStringAsJsonbString | PostgresqlJsonType::UuidUuidAsJsonbString => Self::String,
                    }
                }
            }
            let postgresql_json_type_specific = PostgresqlJsonTypeSpecific::from(&element.postgresql_json_type);

            let is_vec_element_type = element.is_vec_element_type();
            let common_postgresql_json_type_filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = vec![&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal];
            let common_postgresql_json_type_vec_filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = {
                let mut vec: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = common_postgresql_json_type_filters_variants.clone();
                vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthMoreThan);
                if is_vec_element_type {
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::PositionEqual);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsAllElementsOfArray);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::OverlapsWithArray);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual);
                }
                vec
            };

            let generate_where_element_variants_types_generic_token_stream =
                |is_relevant_only_for_not_null: std::primitive::bool| -> &dyn naming::StdFmtDisplayPlusQuoteToTokens { if is_relevant_only_for_not_null { 
                    &ident_standart_not_null_origin_upper_camel_case
                } else {
                    &ident_origin_upper_camel_case
                }
            };
            let postgresql_json_type_where_element_number_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                &{
                    let mut vec = common_postgresql_json_type_filters_variants.clone();
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In);
                    vec
                },
                &generate_where_element_variants_types_generic_token_stream,
                &ident,
                &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                &postgresql_crud_macros_common::IsQueryBindMutable::False,
            );
            let postgresql_json_type_where_element_bool_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                &common_postgresql_json_type_filters_variants,
                &generate_where_element_variants_types_generic_token_stream,
                &ident,
                &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                &postgresql_crud_macros_common::IsQueryBindMutable::False,
            );
            let postgresql_json_type_where_element_string_token_stream = postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                &{
                    let mut vec = common_postgresql_json_type_filters_variants.clone();
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::CaseSensitiveRegularExpression);
                    vec.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::CaseInsensitiveRegularExpression);
                    vec
                },
                &generate_where_element_variants_types_generic_token_stream,
                &ident,
                &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                &postgresql_crud_macros_common::IsQueryBindMutable::False,
            );

            let postgresql_json_type_where_element_vec_number_token_stream = {
                let mut filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = common_postgresql_json_type_vec_filters_variants.clone();
                if is_vec_element_type {
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::PositionGreaterThan);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementGreaterThan);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsGreaterThan);
                }
                postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &filters_variants,
                    &generate_where_element_variants_types_generic_token_stream,
                    &ident,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                    &postgresql_crud_macros_common::IsQueryBindMutable::False,
                )
            };
            let postgresql_json_type_where_element_vec_bool_token_stream = {
                let filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = common_postgresql_json_type_vec_filters_variants.clone();
                postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &filters_variants,
                    &generate_where_element_variants_types_generic_token_stream,
                    &ident,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                    &postgresql_crud_macros_common::IsQueryBindMutable::False,
                )
            };
            let postgresql_json_type_where_element_vec_string_token_stream = {
                let mut filters_variants: std::vec::Vec<&dyn postgresql_crud_macros_common::PostgresqlFilter> = common_postgresql_json_type_vec_filters_variants.clone();
                if is_vec_element_type {
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::PositionCaseSensitiveRegularExpression);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::PositionCaseInsensitiveRegularExpression);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementCaseSensitiveRegularExpression);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementCaseInsensitiveRegularExpression);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsCaseSensitiveRegularExpression);
                    filters_variants.push(&postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsCaseInsensitiveRegularExpression);
                }
                postgresql_crud_macros_common::generate_postgresql_type_where_element_token_stream(
                    &filters_variants,
                    &generate_where_element_variants_types_generic_token_stream,
                    &ident,
                    &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                    &postgresql_crud_macros_common::IsQueryBindMutable::False,
                )
            };
            match &element.postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::Standart => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => postgresql_json_type_where_element_number_token_stream,
                    PostgresqlJsonTypeSpecific::Bool => postgresql_json_type_where_element_bool_token_stream,
                    PostgresqlJsonTypeSpecific::String => postgresql_json_type_where_element_string_token_stream,
                },
                PostgresqlJsonTypePattern::ArrayDimension1 {..} => match &postgresql_json_type_specific {
                    PostgresqlJsonTypeSpecific::Number => postgresql_json_type_where_element_vec_number_token_stream,
                    PostgresqlJsonTypeSpecific::Bool => postgresql_json_type_where_element_vec_bool_token_stream,
                    PostgresqlJsonTypeSpecific::String => postgresql_json_type_where_element_vec_string_token_stream,
                },
            }
        };
        let checked_add_upper_camel_case = naming::CheckedAddUpperCamelCase;
        let impl_crate_postgresql_json_type_for_ident_token_stream = postgresql_crud_macros_common::generate_postgresql_json_type_token_stream(
            &postgresql_crud_macros_common::ImportPath::Crate,
            &ident,
            &ident_origin_upper_camel_case,
            &{
                quote::quote! {
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!("${increment}"))
                        }
                        None => Err(crate::QueryPartErrorNamed::#checked_add_upper_camel_case {
                            code_occurence: error_occurence_lib::code_occurence!()
                        }),
                    }
                }
            },
            &postgresql_crud_macros_common::IsCreateQueryBindMutable::True,
            &{
                let value_snake_case = naming::ValueSnakeCase;
                quote::quote! {
                    query = query.bind(#value_snake_case);
                    query
                }
            },
            &ident_select_upper_camel_case,
            &ident_origin_upper_camel_case,
            &{
                let value_snake_case = naming::ValueSnakeCase;
                let postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream = |format_handle_token_stream: &dyn quote::ToTokens| {
                    let pagination_start_end_initialization_token_stream = macros_helpers::pagination_start_end_initialization_token_stream::pagination_start_end_initialization_token_stream(&value_snake_case);
                    quote::quote! {
                        #pagination_start_end_initialization_token_stream
                        format!(#format_handle_token_stream)
                    }
                };
                let column_name_and_maybe_field_getter_snake_case = naming::ColumnNameAndMaybeFieldGetterSnakeCase;

                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => {
                        let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_build_object('{{field_ident}}', jsonb_build_object('value', {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}'))"));
                        quote::quote! {
                            format!(#format_handle_token_stream)
                        }
                    }
                    PostgresqlJsonTypePattern::ArrayDimension1 {..} => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_quotes::double_quotes_token_stream(&format!(
                        "jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))"
                    ))),
                    // PostgresqlJsonTypePattern::StdVecVecStdOptionOptionStandart => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(
                    //     &generate_quotes::double_quotes_token_stream(
                    //         &format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value', case when jsonb_typeof({{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}') = 'array' then (select jsonb_agg(value) from jsonb_array_elements((select {{column_name_and_maybe_field_getter}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}}) else null end))")
                    //     )
                    // ),
                    // PostgresqlJsonTypePattern::StdVecVecVecStandart => postgresql_query_part_field_to_read_for_ident_with_limit_offset_start_end_token_stream(&generate_quotes::double_quotes_token_stream(&format!(
                    //     "jsonb_build_object('{{field_ident}}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {{{column_name_and_maybe_field_getter_snake_case}}}->'{{field_ident}}')) with ordinality where ordinality between {{start}} and {{end}})))"
                    // ))),
                }
            },
            &ident_where_element_upper_camel_case,
            &ident_origin_upper_camel_case,
            &{
                let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
                let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{jsonb_set_path}}}}}}',${{increment}})"));
                quote::quote! {
                    match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            Ok(format!(#format_handle_token_stream))
                        }
                        None => Err(crate::QueryPartErrorNamed::#checked_add_upper_camel_case { code_occurence: error_occurence_lib::code_occurence!() }),
                    }
                }
            },
            &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
            &{
                let value_snake_case = naming::ValueSnakeCase;
                quote::quote! {
                    query = query.bind(#value_snake_case);
                    query
                }
            },
        );
        let generated = quote::quote! {
            #ident_token_stream
            #ident_origin_token_stream

            #ident_select_token_stream

            #ident_where_element_token_stream
            #impl_crate_postgresql_json_type_for_ident_token_stream
        };
        // if ident == "" {
        //     println!("{generated}");
        //     println!("-------");
        //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file("GeneratePostgresqlJsonTypes", &generated);
        // }
        (
            {
                let field_ident = format!("column_{}", uuid::Uuid::new_v4()).replace("-", "_").parse::<proc_macro2::TokenStream>().unwrap();
                quote::quote!{
                    pub #field_ident: postgresql_crud::postgresql_json_type:: #ident,
                }.to_string()
            },
            generated.to_string()
        )
    })
    .collect::<(std::vec::Vec<String>, std::vec::Vec<String>)>();
    if false {
        let postgresql_crud_json_object_rust_struct_fields_token_stream = postgresql_crud_json_object_rust_struct_fields_token_stream
        .into_iter()
        .map(|element|{
            element.parse::<proc_macro2::TokenStream>().unwrap()
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
            "PostgresqlJsonTypeTokensExampleStruct",
            &quote::quote!{
                struct Example {
                    #(#postgresql_crud_json_object_rust_struct_fields_token_stream)*
                }
            },
        );
    }
    let generated = {
        let postgresql_json_type_array = postgresql_json_type_array
        .into_iter()
        .map(|element|{
            element.parse::<proc_macro2::TokenStream>().unwrap()
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#postgresql_json_type_array)*}
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "PostgresqlJsonTypeTokens",
    //     &generated,
    // );
    generated.into()
}
