#[proc_macro]
pub fn generate_postgresql_json_types(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
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
    impl From<&PostgresqlJsonType> for RustTypeName {
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
    #[derive(Debug)]
    enum PostgresqlJsonTypeName {
        Number,
        Boolean,
        String,
    }
    impl std::fmt::Display for PostgresqlJsonTypeName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                naming::parameter::JsonbSelfUpperCamelCase::from_display(match &self {
                    Self::Number => &naming::NumberUpperCamelCase,
                    Self::Boolean => &naming::BooleanUpperCamelCase,
                    Self::String => &naming::StringUpperCamelCase,
                })
            )
        }
    }
    impl From<&PostgresqlJsonType> for PostgresqlJsonTypeName {
        fn from(value: &PostgresqlJsonType) -> Self {
            match &value {
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
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => Self::Boolean,
                PostgresqlJsonType::StdStringStringAsJsonbString
                | PostgresqlJsonType::UuidUuidAsJsonbString => Self::String,
            }
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        Hash,
        serde::Serialize,
        serde::Deserialize,
        strum_macros::Display,
        strum_macros::EnumIter,
        enum_extension_lib::EnumExtension,
    )]
    enum PostgresqlJsonType {
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
    impl quote::ToTokens for PostgresqlJsonType {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            self.to_string()
                .parse::<proc_macro2::TokenStream>()
                .expect("eb6cafe0-ad0d-4108-8b0e-c062b155efbb")
                .to_tokens(tokens);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        Hash,
        serde::Serialize,
        serde::Deserialize,
        strum_macros::Display,
        strum_macros::EnumIter,
        enum_extension_lib::EnumExtension,
    )]
    enum PostgresqlJsonTypePattern {
        Standart,
        ArrayDimension1 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension2 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension3 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension3_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension4 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension3_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension4_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
    }
    impl PostgresqlJsonTypePattern {
        const fn down_by_1(&self) -> Option<Self> {
            match &self {
                Self::Standart => None,
                Self::ArrayDimension1 { .. } => Some(Self::Standart),
                Self::ArrayDimension2 {
                    dimension2_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension1 {
                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                }),
                Self::ArrayDimension3 {
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension2 {
                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                }),
                Self::ArrayDimension4 {
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension3 {
                    dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                    dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                }),
            }
        }
        const fn down_by_2(&self) -> Option<Self> {
            match &self {
                Self::Standart | Self::ArrayDimension1 { .. } => None,
                Self::ArrayDimension2 { .. } => Some(Self::Standart),
                Self::ArrayDimension3 {
                    dimension3_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension1 {
                    dimension1_not_null_or_nullable: *dimension3_not_null_or_nullable,
                }),
                Self::ArrayDimension4 {
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension2 {
                    dimension1_not_null_or_nullable: *dimension3_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension4_not_null_or_nullable,
                }),
            }
        }
        const fn down_by_3(&self) -> Option<Self> {
            match &self {
                Self::Standart | Self::ArrayDimension1 { .. } | Self::ArrayDimension2 { .. } => {
                    None
                }
                Self::ArrayDimension3 { .. } => Some(Self::Standart),
                Self::ArrayDimension4 {
                    dimension4_not_null_or_nullable,
                    ..
                } => Some(Self::ArrayDimension1 {
                    dimension1_not_null_or_nullable: *dimension4_not_null_or_nullable,
                }),
            }
        }
        const fn down_by_4(&self) -> Option<Self> {
            match &self {
                Self::Standart
                | Self::ArrayDimension1 { .. }
                | Self::ArrayDimension2 { .. }
                | Self::ArrayDimension3 { .. } => None,
                Self::ArrayDimension4 { .. } => Some(Self::Standart),
            }
        }
    }
    enum ArrayDimension {
        ArrayDimension1,
        ArrayDimension2 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension3 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
        ArrayDimension4 {
            dimension1_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension2_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
            dimension3_not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        },
    }
    impl ArrayDimension {
        const fn to_usize(&self) -> usize {
            match &self {
                Self::ArrayDimension1 { .. } => 1,
                Self::ArrayDimension2 { .. } => 2,
                Self::ArrayDimension3 { .. } => 3,
                Self::ArrayDimension4 { .. } => 4,
            }
        }
    }
    impl TryFrom<&PostgresqlJsonTypePattern> for ArrayDimension {
        type Error = ();
        fn try_from(value: &PostgresqlJsonTypePattern) -> Result<Self, Self::Error> {
            match &value {
                PostgresqlJsonTypePattern::Standart => Err(()),
                PostgresqlJsonTypePattern::ArrayDimension1 { .. } => Ok(Self::ArrayDimension1),
                PostgresqlJsonTypePattern::ArrayDimension2 {
                    dimension1_not_null_or_nullable,
                    ..
                } => Ok(Self::ArrayDimension2 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                }),
                PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    ..
                } => Ok(Self::ArrayDimension3 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                }),
                PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    ..
                } => Ok(Self::ArrayDimension4 {
                    dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable: *dimension3_not_null_or_nullable,
                }),
            }
        }
    }
    #[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    struct PostgresqlJsonTypeRecord {
        postgresql_json_type: PostgresqlJsonType,
        not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        postgresql_json_type_pattern: PostgresqlJsonTypePattern,
    }
    #[derive(Debug, serde::Deserialize)]
    enum GeneratePostgresqlJsonTypesConfigVariant {
        All,
        WithoutDimensions,
        WithDimensionOne,
        WithDimensionTwo,
        WithDimensionThree,
        WithDimensionFour,
        Concrete(Vec<PostgresqlJsonTypeRecord>),
    }
    #[derive(Debug, serde::Deserialize)]
    struct GeneratePostgresqlJsonTypesConfig {
        postgresql_table_columns_content_write_into_postgresql_table_columns_using_postgresql_json_types:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        whole_content_write_into_generate_postgresql_json_types:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        variant: GeneratePostgresqlJsonTypesConfigVariant,
    }
    use rayon::iter::IntoParallelRefIterator as _;
    use rayon::iter::ParallelIterator as _;
    panic_location::panic_location();
    let generate_postgresql_json_types_config =
        serde_json::from_str::<GeneratePostgresqlJsonTypesConfig>(&input_token_stream.to_string())
            .expect("1123f78f-9c84-4001-b619-b534dd55a835");
    let (fields_token_stream, postgresql_json_type_array) = {
        {
            let generate_standart = |acc_29796d99: &mut Vec<PostgresqlJsonTypeRecord>, postgresql_json_type: PostgresqlJsonType|{
                for element_2f39e657 in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                    acc_29796d99.push(PostgresqlJsonTypeRecord {
                        postgresql_json_type: postgresql_json_type.clone(),
                        not_null_or_nullable: element_2f39e657,
                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::Standart,
                    });
                }
            };
            let generate_array_dimension1 = |acc_5b22a398: &mut Vec<PostgresqlJsonTypeRecord>, postgresql_json_type: PostgresqlJsonType|{
                for element_29854486 in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                    for element_6440cc9c in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                        acc_5b22a398.push(PostgresqlJsonTypeRecord {
                            postgresql_json_type: postgresql_json_type.clone(),
                            not_null_or_nullable: element_29854486,
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable: element_6440cc9c },
                        });
                    }
                }
            };
            let generate_array_dimension2 = |acc_e59f7158: &mut Vec<PostgresqlJsonTypeRecord>, postgresql_json_type: PostgresqlJsonType|{
                for element_a6ba4c3e in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                    for element_4b5a815d in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                        for element_2e4896dd in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                            acc_e59f7158.push(PostgresqlJsonTypeRecord {
                                postgresql_json_type: postgresql_json_type.clone(),
                                not_null_or_nullable: element_a6ba4c3e,
                                postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension2 {
                                    dimension1_not_null_or_nullable: element_4b5a815d,
                                    dimension2_not_null_or_nullable: element_2e4896dd
                                },
                            });
                        }
                    }
                }
            };
            let generate_array_dimension3 = |acc_77498dc3: &mut Vec<PostgresqlJsonTypeRecord>, postgresql_json_type: PostgresqlJsonType|{
                for element_8f03b1c2 in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                    for element_a27b642f in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                        for element_dc57e9b7 in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                            for element_4361fee5 in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                acc_77498dc3.push(PostgresqlJsonTypeRecord {
                                    postgresql_json_type: postgresql_json_type.clone(),
                                    not_null_or_nullable: element_8f03b1c2,
                                    postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension3 {
                                        dimension1_not_null_or_nullable: element_a27b642f,
                                        dimension2_not_null_or_nullable: element_dc57e9b7,
                                        dimension3_not_null_or_nullable: element_4361fee5,
                                    },
                                });
                            }
                        }
                    }
                }
            };
            let generate_array_dimension4 = |acc_7c8a3329: &mut Vec<PostgresqlJsonTypeRecord>, postgresql_json_type: PostgresqlJsonType|{
                for element_daf10957 in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                    for element_fc5a53dd in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                        for element_69b59c5c in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                            for element_d7efbd09 in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                for element_c16cb65b in postgresql_crud_macros_common::NotNullOrNullable::into_array() {
                                    acc_7c8a3329.push(PostgresqlJsonTypeRecord {
                                        postgresql_json_type: postgresql_json_type.clone(),
                                        not_null_or_nullable: element_daf10957,
                                        postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension4 {
                                            dimension1_not_null_or_nullable: element_fc5a53dd,
                                            dimension2_not_null_or_nullable: element_69b59c5c,
                                            dimension3_not_null_or_nullable: element_d7efbd09,
                                            dimension4_not_null_or_nullable: element_c16cb65b,
                                        },
                                    });
                                }
                            }
                        }
                    }
                }
            };
            let acc_d97120ed = match generate_postgresql_json_types_config.variant {
                GeneratePostgresqlJsonTypesConfigVariant::All => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_e2f65a79, postgresql_json_type| {
                    for element_644643cd in PostgresqlJsonTypePattern::into_array() {
                        match &element_644643cd {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_e2f65a79, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } => generate_array_dimension1(&mut acc_e2f65a79, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } => generate_array_dimension2(&mut acc_e2f65a79, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } => generate_array_dimension3(&mut acc_e2f65a79, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => generate_array_dimension4(&mut acc_e2f65a79, postgresql_json_type.clone()),
                        }
                    }
                    acc_e2f65a79
                }),
                GeneratePostgresqlJsonTypesConfigVariant::WithoutDimensions => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_3d95645c, postgresql_json_type| {
                    for element_fccf1979 in PostgresqlJsonTypePattern::into_array() {
                        match &element_fccf1979 {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_3d95645c, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => (),
                        }
                    }
                    acc_3d95645c
                }),
                GeneratePostgresqlJsonTypesConfigVariant::WithDimensionOne => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_66a17cae, postgresql_json_type| {
                    for element_e69bd1fc in PostgresqlJsonTypePattern::into_array() {
                        match &element_e69bd1fc {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_66a17cae, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } => generate_array_dimension1(&mut acc_66a17cae, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => (),
                        }
                    }
                    acc_66a17cae
                }),
                GeneratePostgresqlJsonTypesConfigVariant::WithDimensionTwo => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_c5ffb796, postgresql_json_type| {
                    for element_345fd6bd in PostgresqlJsonTypePattern::into_array() {
                        match &element_345fd6bd {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_c5ffb796, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } => generate_array_dimension1(&mut acc_c5ffb796, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } => generate_array_dimension2(&mut acc_c5ffb796, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } |
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => (),
                        }
                    }
                    acc_c5ffb796
                }),
                GeneratePostgresqlJsonTypesConfigVariant::WithDimensionThree => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_78b27c00, postgresql_json_type| {
                    for element_88e3b8fe in PostgresqlJsonTypePattern::into_array() {
                        match &element_88e3b8fe {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_78b27c00, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } => generate_array_dimension1(&mut acc_78b27c00, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } => generate_array_dimension2(&mut acc_78b27c00, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } => generate_array_dimension3(&mut acc_78b27c00, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => (),
                        }
                    }
                    acc_78b27c00
                }),
                GeneratePostgresqlJsonTypesConfigVariant::WithDimensionFour => PostgresqlJsonType::into_array().into_iter().fold(Vec::new(), |mut acc_172c62ad, postgresql_json_type| {
                    for element_80434642 in PostgresqlJsonTypePattern::into_array() {
                        match &element_80434642 {
                            PostgresqlJsonTypePattern::Standart => generate_standart(&mut acc_172c62ad, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension1 { .. } => generate_array_dimension1(&mut acc_172c62ad, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension2 { .. } => generate_array_dimension2(&mut acc_172c62ad, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension3 { .. } => generate_array_dimension3(&mut acc_172c62ad, postgresql_json_type.clone()),
                            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => generate_array_dimension4(&mut acc_172c62ad, postgresql_json_type.clone()),
                        }
                    }
                    acc_172c62ad
                }),
                GeneratePostgresqlJsonTypesConfigVariant::Concrete(value) => value,
            };
            let mut seen = std::collections::HashSet::new();
            assert!(
                acc_d97120ed
                    .iter()
                    .all(|element_8ef63a77| seen.insert(element_8ef63a77)),
                "c2d37017-229c-4259-bcee-c434852dca1b"
            );
            acc_d97120ed
        }.into_iter().fold(Vec::new(), |mut acc_5e43269c, element_c4f9bf8f| {
            use postgresql_crud_macros_common::NotNullOrNullable;
            for element_7ae8d2ae in {
                #[derive(Clone)]
                struct PostgresqlJsonTypeRecordHandle {
                    not_null_or_nullable: NotNullOrNullable,
                    postgresql_json_type_pattern: PostgresqlJsonTypePattern,
                }
                fn generate_postgresql_json_type_record_handle_vec(postgresql_json_type_record_handle: PostgresqlJsonTypeRecordHandle) -> Vec<PostgresqlJsonTypeRecordHandle> {
                    let generate_vec = |current_postgresql_json_type_record_handle: PostgresqlJsonTypeRecordHandle| generate_postgresql_json_type_record_handle_vec(
                        current_postgresql_json_type_record_handle
                    ).into_iter().chain(std::iter::once(postgresql_json_type_record_handle.clone())).collect();
                    match (&postgresql_json_type_record_handle.not_null_or_nullable, &postgresql_json_type_record_handle.postgresql_json_type_pattern) {
                        (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) => vec![postgresql_json_type_record_handle],
                        (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::Standart) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: NotNullOrNullable::NotNull,
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::Standart,
                        }),
                        (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: *dimension1_not_null_or_nullable,
                            postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.down_by_1().expect("0e970a4f-eeec-421e-aa30-f90fc536d388"),
                        }),
                        (NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: *dimension1_not_null_or_nullable,
                            postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.down_by_1().expect("85f8ed83-aa1f-4f52-8f8f-80aeb86f8083"),
                        }),
                        (
                            NotNullOrNullable::NotNull,
                            PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                            },
                        ) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: *dimension1_not_null_or_nullable,
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension2 {
                                dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                            },
                        }),
                        (
                            NotNullOrNullable::NotNull,
                            PostgresqlJsonTypePattern::ArrayDimension4 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                                dimension4_not_null_or_nullable,
                            },
                        ) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: *dimension1_not_null_or_nullable,
                            postgresql_json_type_pattern: PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                dimension2_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                dimension3_not_null_or_nullable: *dimension4_not_null_or_nullable,
                            },
                        }),
                        (NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. }) => generate_vec(PostgresqlJsonTypeRecordHandle {
                            not_null_or_nullable: NotNullOrNullable::NotNull,
                            postgresql_json_type_pattern: postgresql_json_type_record_handle.postgresql_json_type_pattern.clone(),
                        }),
                    }
                }
                generate_postgresql_json_type_record_handle_vec(PostgresqlJsonTypeRecordHandle {
                    not_null_or_nullable: element_c4f9bf8f.not_null_or_nullable,
                    postgresql_json_type_pattern: element_c4f9bf8f.postgresql_json_type_pattern,
                })
            } {
                let postgresql_json_type_record = PostgresqlJsonTypeRecord {
                    postgresql_json_type: element_c4f9bf8f.postgresql_json_type.clone(),
                    not_null_or_nullable: element_7ae8d2ae.not_null_or_nullable,
                    postgresql_json_type_pattern: element_7ae8d2ae.postgresql_json_type_pattern,
                };
                if !acc_5e43269c.contains(&postgresql_json_type_record) {
                    acc_5e43269c.push(postgresql_json_type_record);
                }
            }
            acc_5e43269c
        })
    }
    .into_iter()
    .enumerate()
    .collect::<Vec<(usize, PostgresqlJsonTypeRecord)>>()
    .par_iter()
    // .into_iter() //just for console prints ordering
    .map(|(index, element_1d376874)| {
        enum IsStandartNotNull {
            True,
            False,
        }
        enum IsStandartNotNullUuid {
            True,
            False,
        }
        enum ConstFn {
            True,
            False
        }
        // struct SchemaObjectTokenStream<'lifetime> {
        //     metadata: &'lifetime dyn quote::ToTokens,
        //     instance_type: &'lifetime dyn quote::ToTokens,
        //     format: &'lifetime dyn quote::ToTokens,
        //     enum_values: &'lifetime dyn quote::ToTokens,
        //     const_value: &'lifetime dyn quote::ToTokens,
        //     subschemas: &'lifetime dyn quote::ToTokens,
        //     number: &'lifetime dyn quote::ToTokens,
        //     string: &'lifetime dyn quote::ToTokens,
        //     array: &'lifetime dyn quote::ToTokens,
        //     object: &'lifetime dyn quote::ToTokens,
        //     reference: &'lifetime dyn quote::ToTokens,
        //     extensions: &'lifetime dyn quote::ToTokens,
        // }
        // enum SchemarsJsonSchema<'schema_object_token_stream_tifetime> {
        //     Derive,
        //     Impl(SchemaObjectTokenStream<'schema_object_token_stream_tifetime>),
        // }
        let postgresql_json_type = &element_1d376874.postgresql_json_type;
        let not_null_or_nullable = &element_1d376874.not_null_or_nullable;
        let postgresql_json_type_pattern = &element_1d376874.postgresql_json_type_pattern;
        let rust_type_name = RustTypeName::from(postgresql_json_type);
        let postgresql_json_type_name = PostgresqlJsonTypeName::from(postgresql_json_type);
        let is_standart_not_null = if let (PostgresqlJsonTypePattern::Standart, postgresql_crud_macros_common::NotNullOrNullable::NotNull) = (&postgresql_json_type_pattern, &not_null_or_nullable) {
            IsStandartNotNull::True
        } else {
            IsStandartNotNull::False
        };
        let is_standart_not_null_uuid = if let (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart, PostgresqlJsonType::UuidUuidAsJsonbString) = (&not_null_or_nullable, &postgresql_json_type_pattern, &postgresql_json_type) {
            IsStandartNotNullUuid::True
        } else {
            IsStandartNotNullUuid::False
        };
        let value_snake_case = naming::ValueSnakeCase;
        let as_upper_camel_case = naming::AsUpperCamelCase;
        let new_snake_case = naming::NewSnakeCase;
        let self_upper_camel_case = naming::SelfUpperCamelCase;
        let increment_snake_case = naming::IncrementSnakeCase;
        let query_snake_case = naming::QuerySnakeCase;
        let read_snake_case = naming::ReadSnakeCase;
        let error_snake_case = naming::ErrorSnakeCase;
        let option_vec_create_snake_case = naming::OptionVecCreateSnakeCase;
        let option_update_snake_case = naming::OptionUpdateSnakeCase;
        let read_only_ids_to_two_dimensional_vec_read_inner_snake_case = naming::ReadOnlyIdsToTwoDimensionalVecReadInnerSnakeCase;
        let create_snake_case = naming::CreateSnakeCase;
        let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
        let postgresql_json_type_upper_camel_case = naming::PostgresqlJsonTypeUpperCamelCase;
        let import_path = postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon;
        let create_for_query_upper_camel_case = naming::CreateForQueryUpperCamelCase;
        let update_for_query_upper_camel_case = naming::UpdateForQueryUpperCamelCase;
        let update_upper_camel_case = naming::UpdateUpperCamelCase;
        let self_snake_case = naming::SelfSnakeCase;
        let equal_upper_camel_case = naming::EqualUpperCamelCase;
        let read_inner_upper_camel_case = naming::ReadInnerUpperCamelCase;
        let read_only_ids_merged_with_create_into_read_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoReadSnakeCase;
        let read_only_ids_merged_with_create_into_where_equal_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoWhereEqualSnakeCase;
        let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSnakeCase;

        let std_primitive_i8_token_stream = token_patterns::StdPrimitiveI8;
        let std_primitive_i16_token_stream = token_patterns::StdPrimitiveI16;
        let std_primitive_i32_token_stream = token_patterns::StdPrimitiveI32;
        let std_primitive_i64_token_stream = token_patterns::StdPrimitiveI64;
        let std_primitive_u8_token_stream = token_patterns::StdPrimitiveU8;
        let std_primitive_u16_token_stream = token_patterns::StdPrimitiveU16;
        let std_primitive_u32_token_stream = token_patterns::StdPrimitiveU32;
        let std_primitive_u64_token_stream = token_patterns::StdPrimitiveU64;
        let std_primitive_f32_token_stream = token_patterns::StdPrimitiveF32;
        let std_primitive_f64_token_stream = token_patterns::StdPrimitiveF64;
        let std_primitive_bool_token_stream = token_patterns::StdPrimitiveBool;
        let std_string_string_token_stream = token_patterns::StdStringString;
        let uuid_uuid_token_stream = token_patterns::UuidUuid;
        // let schemars_json_schema_token_stream = token_patterns::SchemarsJsonSchema;

        let none_token_stream = quote::quote! {None};

        let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
        let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
        let postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream = token_patterns::PostgresqlCrudCommonDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSizeCall;

        let generate_import_path_value_initialization_token_stream = |content_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_value_initialization_token_stream(&import_path, &content_token_stream);
        let generate_ident_token_stream = |current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
            let vec_of_upper_camel_case = naming::VecOfUpperCamelCase;
            let array_of_upper_camel_case = naming::ArrayOfUpperCamelCase;
            let not_null_or_nullable_rust = current_not_null_or_nullable.rust();
            let (rust_part, postgresql_part) = match &current_postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::Standart => (rust_type_name.to_string(), postgresql_json_type_name.to_string()),
                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    (format!("{vec_of_upper_camel_case}{d1_rust}{rust_type_name}"), format!("{array_of_upper_camel_case}{d1}{postgresql_json_type_name}"))
                }
                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    let d2 = dimension2_not_null_or_nullable;
                    let d2_rust = dimension2_not_null_or_nullable.rust();
                    (format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{rust_type_name}"), format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{postgresql_json_type_name}"))
                }
                PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    let d2 = dimension2_not_null_or_nullable;
                    let d2_rust = dimension2_not_null_or_nullable.rust();
                    let d3 = dimension3_not_null_or_nullable;
                    let d3_rust = dimension3_not_null_or_nullable.rust();
                    (
                        format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{rust_type_name}"),
                        format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{postgresql_json_type_name}"),
                    )
                }
                PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                } => {
                    let d1 = dimension1_not_null_or_nullable;
                    let d1_rust = dimension1_not_null_or_nullable.rust();
                    let d2 = dimension2_not_null_or_nullable;
                    let d2_rust = dimension2_not_null_or_nullable.rust();
                    let d3 = dimension3_not_null_or_nullable;
                    let d3_rust = dimension3_not_null_or_nullable.rust();
                    let d4 = dimension4_not_null_or_nullable;
                    let d4_rust = dimension4_not_null_or_nullable.rust();
                    (
                        format!("{vec_of_upper_camel_case}{d1_rust}{vec_of_upper_camel_case}{d2_rust}{vec_of_upper_camel_case}{d3_rust}{vec_of_upper_camel_case}{d4_rust}{rust_type_name}"),
                        format!("{array_of_upper_camel_case}{d1}{array_of_upper_camel_case}{d2}{array_of_upper_camel_case}{d3}{array_of_upper_camel_case}{d4}{postgresql_json_type_name}"),
                    )
                }
            };
            format!("{not_null_or_nullable_rust}{rust_part}{as_upper_camel_case}{current_not_null_or_nullable}{postgresql_part}").parse::<proc_macro2::TokenStream>().expect("998d1471-be98-4669-8bd3-ca6c4a1a5853")
        };
        let ident = &generate_ident_token_stream(not_null_or_nullable, postgresql_json_type_pattern);
        let ident_standart_not_null_upper_camel_case = &generate_ident_token_stream(&postgresql_crud_macros_common::NotNullOrNullable::NotNull, &PostgresqlJsonTypePattern::Standart);
        let ident_standart_not_null_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
        let ident_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident);
        let ident_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
        let ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&ident);
        let ident_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident);
        let ident_not_null_token_stream = generate_ident_token_stream(&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern);
        let ident_token_stream = {
            let ident_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy()
                .build_struct(
                    &ident,
                    &quote::quote!{;}
                );
            quote::quote! {
                #ident_token_stream
            }
        };
        let ident_standart_not_null_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
        let ident_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident);
        let ident_read_inner_standart_not_null_alias_token_stream = {
            let content_token_stream: &dyn quote::ToTokens = match &postgresql_json_type {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => &std_primitive_i8_token_stream,
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => &std_primitive_i16_token_stream,
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => &std_primitive_i32_token_stream,
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => &std_primitive_i64_token_stream,
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => &std_primitive_u8_token_stream,
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => &std_primitive_u16_token_stream,
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => &std_primitive_u32_token_stream,
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => &std_primitive_u64_token_stream,
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => &std_primitive_f32_token_stream,
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => &std_primitive_f64_token_stream,
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => &std_primitive_bool_token_stream,
                PostgresqlJsonType::StdStringStringAsJsonbString => &std_string_string_token_stream,
                PostgresqlJsonType::UuidUuidAsJsonbString => &uuid_uuid_token_stream,
            };
            quote::quote! {#content_token_stream}
        };
        let ident_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&ident);
        let value_ident_read_inner_token_stream = quote::quote! {#value_snake_case: #ident_read_inner_upper_camel_case};
        let generate_pub_fn_new_value_ident_read_inner_content_token_stream = |content_token_stream: &dyn quote::ToTokens| macros_helpers::generate_pub_new_token_stream(&value_ident_read_inner_token_stream, &content_token_stream);
        let generate_pub_const_fn_new_value_ident_read_inner_content_token_stream = |content_token_stream: &dyn quote::ToTokens| macros_helpers::generate_pub_const_new_token_stream(&value_ident_read_inner_token_stream, &content_token_stream);
        let self_ident_origin_new_value_token_stream = quote::quote! {Self(#ident_origin_upper_camel_case::new(#value_snake_case))};
        let maybe_const_fn = match &postgresql_json_type_pattern {
            PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                postgresql_crud_macros_common::NotNullOrNullable::NotNull => ConstFn::True,
                postgresql_crud_macros_common::NotNullOrNullable::Nullable => ConstFn::False,
            },
            PostgresqlJsonTypePattern::ArrayDimension1 { .. } |
            PostgresqlJsonTypePattern::ArrayDimension2 { .. } |
            PostgresqlJsonTypePattern::ArrayDimension3 { .. } |
            PostgresqlJsonTypePattern::ArrayDimension4 { .. } => ConstFn::False,
        };
        let generate_pub_new_or_fn_new_token_stream = |const_new_token_stream: &dyn quote::ToTokens, new_token_stream: &dyn quote::ToTokens|match maybe_const_fn {
            ConstFn::True => generate_pub_const_fn_new_value_ident_read_inner_content_token_stream(
                &const_new_token_stream
            ),
            ConstFn::False => generate_pub_fn_new_value_ident_read_inner_content_token_stream(
                &new_token_stream
            ),
        };
        let pub_new_or_const_new_self_ident_origin_new_value_token_stream = generate_pub_new_or_fn_new_token_stream(
            &self_ident_origin_new_value_token_stream,
            &self_ident_origin_new_value_token_stream
        );
        let ident_create_for_query_upper_camel_case = naming::parameter::SelfCreateForQueryUpperCamelCase::from_tokens(&ident);
        let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
        let ident_update_for_query_upper_camel_case = naming::parameter::SelfUpdateForQueryUpperCamelCase::from_tokens(&ident);
        let maybe_derive_copy = match &postgresql_json_type_pattern {
            PostgresqlJsonTypePattern::Standart => match &postgresql_json_type {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber |
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                PostgresqlJsonType::UuidUuidAsJsonbString => macros_helpers::DeriveCopy::True,
                PostgresqlJsonType::StdStringStringAsJsonbString => macros_helpers::DeriveCopy::False,
            },
            PostgresqlJsonTypePattern::ArrayDimension1 {..} |
            PostgresqlJsonTypePattern::ArrayDimension2 {..} |
            PostgresqlJsonTypePattern::ArrayDimension3 {..} |
            PostgresqlJsonTypePattern::ArrayDimension4 {..} => macros_helpers::DeriveCopy::False,
        };
        let ident_origin_token_stream = {
            let generate_current_ident_origin_non_wrapping = |
                current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable,
                current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern
            | naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_ident_token_stream(current_not_null_or_nullable, current_postgresql_json_type_pattern));
            // let schema_name_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&ident_origin_upper_camel_case);
            //todo json schema logic
            // let metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream = quote::quote! {
            //     Some(Box::new(schemars::schema::Metadata {
            //         id: None,
            //         title: Some(#schema_name_format_handle_token_stream.to_owned()),
            //         description: None,
            //         default: None,
            //         deprecated: false,
            //         read_only: false,
            //         write_only: false,
            //         examples: Vec::default(),
            //     }))
            // };
            // let extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream = quote::quote! {schemars::Map::default()};
            // let (instance_type_number_token_stream, instance_type_string_token_stream) = {
            //     let generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type = |instance_type: &schemars::schema::InstanceType| {
            //         let instance_type_token_stream: &dyn quote::ToTokens = match &instance_type {
            //             schemars::schema::InstanceType::Null => &naming::NullUpperCamelCase,
            //             schemars::schema::InstanceType::Boolean => &naming::BooleanUpperCamelCase,
            //             schemars::schema::InstanceType::Object => &naming::ObjectUpperCamelCase,
            //             schemars::schema::InstanceType::Array => &naming::ArrayUpperCamelCase,
            //             schemars::schema::InstanceType::Number => &naming::NumberUpperCamelCase,
            //             schemars::schema::InstanceType::String => &naming::StringUpperCamelCase,
            //             schemars::schema::InstanceType::Integer => &naming::IntegerUpperCamelCase,
            //         };
            //         quote::quote! {Some(schemars::schema::SingleOrVec::Single(Box::new(schemars::schema::InstanceType::#instance_type_token_stream)))}
            //     };
            //     (
            //         generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type(&schemars::schema::InstanceType::Number),
            //         generate_instance_type_some_schemars_schema_single_or_vec_single_box_new_schemars_schema_instance_type(&schemars::schema::InstanceType::String),
            //     )
            // };
            // let schemars_json_schema = if let IsStandartNotNull::True = &is_standart_not_null {
            //     match &postgresql_json_type {
            //         PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber
            //         | PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => SchemarsJsonSchema::Impl(SchemaObjectTokenStream {
            //             metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream,
            //             instance_type: &instance_type_number_token_stream,
            //             format: &none_upper_camel_case,
            //             enum_values: &none_upper_camel_case,
            //             const_value: &none_upper_camel_case,
            //             subschemas: &none_upper_camel_case,
            //             number: &quote::quote! {Some(Box::new(schemars::schema::NumberValidation {
            //                 multiple_of: None,
            //                 maximum: Some(#ident_read_inner_standart_not_null_alias_token_stream ::MAX as #std_primitive_f64_token_stream),
            //                 exclusive_maximum: None,
            //                 minimum: Some(#ident_read_inner_standart_not_null_alias_token_stream ::MIN as #std_primitive_f64_token_stream),
            //                 exclusive_minimum: None,
            //             }))},
            //             string: &none_upper_camel_case,
            //             array: &none_upper_camel_case,
            //             object: &none_upper_camel_case,
            //             reference: &none_upper_camel_case,
            //             extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream,
            //         }),
            //         PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber | PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean | PostgresqlJsonType::StdStringStringAsJsonbString => SchemarsJsonSchema::Derive,
            //         PostgresqlJsonType::UuidUuidAsJsonbString => SchemarsJsonSchema::Impl(SchemaObjectTokenStream {
            //             metadata: &metadata_4167ee5c_732b_4787_9b37_e0060b0aa8de_token_stream,
            //             instance_type: &instance_type_string_token_stream,
            //             format: &none_upper_camel_case,
            //             enum_values: &none_upper_camel_case,
            //             const_value: &none_upper_camel_case,
            //             subschemas: &none_upper_camel_case,
            //             number: &none_upper_camel_case,
            //             string: &quote::quote! {Some(Box::new(schemars::schema::StringValidation {
            //                 max_length: Some(36),
            //                 min_length: Some(36),
            //                 pattern: None,
            //             }))},
            //             array: &none_upper_camel_case,
            //             object: &none_upper_camel_case,
            //             reference: &none_upper_camel_case,
            //             extensions: &extensions_8dbfea73_88f6_41db_b095_61f59b1002fd_token_stream,
            //         }),
            //     }
            // } else {
            //     SchemarsJsonSchema::Derive
            // };
            let ident_origin_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_partial_ord()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema_if(
                    //todo
                    // match &schemars_json_schema {
                    //     SchemarsJsonSchema::Derive => macros_helpers::DeriveSchemarsJsonSchema::True,
                    //     SchemarsJsonSchema::Impl(_) => macros_helpers::DeriveSchemarsJsonSchema::False,
                    // }
                    if let IsStandartNotNull::True = &is_standart_not_null {
                        match &postgresql_json_type {
                            PostgresqlJsonType::UuidUuidAsJsonbString => macros_helpers::DeriveSchemarsJsonSchema::False,
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
                            | PostgresqlJsonType::StdStringStringAsJsonbString => macros_helpers::DeriveSchemarsJsonSchema::True,
                        }
                    } else {
                        macros_helpers::DeriveSchemarsJsonSchema::True
                    }
                )
                .build_struct(
                    &ident_origin_upper_camel_case,
                    &{
                        let content_token_stream: &dyn quote::ToTokens = {
                            let generate_current_ident_origin = |current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable, current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern| {
                                let value = generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern);
                                match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&value),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&value),
                                }
                            };
                            match &postgresql_json_type_pattern {
                                PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => &ident_read_inner_standart_not_null_alias_token_stream,
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_standart_not_null_origin_upper_camel_case),
                                },
                                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                                    let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("e994797d-7334-4e30-b180-af24c16b68b1")),
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                    };
                                    generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                },
                                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => &{
                                    let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("76eb44e3-3099-4c9a-a935-da3e6f6e4210")),
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                    };
                                    generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                },
                                PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => &{
                                    let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("1b996c86-0b08-476a-b963-373dd6838496")),
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                    };
                                    generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                },
                                PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => &{
                                    let (current_not_null_or_nullable, current_postgresql_json_type_pattern): (&postgresql_crud_macros_common::NotNullOrNullable, &PostgresqlJsonTypePattern) = match &not_null_or_nullable {
                                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => (dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("d24b7481-27d9-40c0-8476-344a16d08f27")),
                                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => (&postgresql_crud_macros_common::NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                                    };
                                    generate_current_ident_origin(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                                },
                            }
                        };
                        quote::quote!{(#content_token_stream);}
                    }
                );
            let ident_origin_impl_new_self_content_token_stream = {
                let generate_value_map_type_new_token_stream = |type_token_stream: &dyn quote::ToTokens| quote::quote! {#value_snake_case.map(#type_token_stream::#new_snake_case)};
                let generate_array_dimensions_initialization_token_stream = |type_token_stream: &dyn quote::ToTokens| match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#value_snake_case.into_iter().map(#type_token_stream::#new_snake_case).collect()},
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_value_map_type_new_token_stream(&type_token_stream),
                };
                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {value},
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_value_map_type_new_token_stream(&ident_standart_not_null_origin_upper_camel_case),
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_array_dimensions_initialization_token_stream(&{
                        let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("1160d3df-2e2b-4a33-a297-6b48546b9ca8"), dimension1_not_null_or_nullable),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                        };
                        generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    }),
                    PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_array_dimensions_initialization_token_stream(&{
                        let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("8ab62f4e-611b-4228-a295-3e731e934b9c"), dimension1_not_null_or_nullable),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                        };
                        generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    }),
                    PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_array_dimensions_initialization_token_stream(&{
                        let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("ed64919d-4679-45da-9d14-22ddee84716b"), dimension1_not_null_or_nullable),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                        };
                        generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    }),
                    PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_array_dimensions_initialization_token_stream(&{
                        let (current_postgresql_json_type_pattern, current_not_null_or_nullable): (&PostgresqlJsonTypePattern, &postgresql_crud_macros_common::NotNullOrNullable) = match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => (&postgresql_json_type_pattern.down_by_1().expect("25646d29-5a30-49fb-b91a-7b49ed8c5b0a"), dimension1_not_null_or_nullable),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => (postgresql_json_type_pattern, &postgresql_crud_macros_common::NotNullOrNullable::NotNull),
                        };
                        generate_current_ident_origin_non_wrapping(current_not_null_or_nullable, current_postgresql_json_type_pattern)
                    }),
                }
            };
            let impl_ident_origin_token_stream = {
                let pub_fn_new_token_stream = {
                    let self_ident_origin_impl_new_self_content_token_stream = quote::quote!{
                        Self(#ident_origin_impl_new_self_content_token_stream)
                    };
                    generate_pub_new_or_fn_new_token_stream(
                        &self_ident_origin_impl_new_self_content_token_stream,
                        &self_ident_origin_impl_new_self_content_token_stream
                    )
                };
                quote::quote! {
                    impl #ident_origin_upper_camel_case {
                        #pub_fn_new_token_stream
                    }
                }
            };
            let impl_std_convert_from_ident_create_for_ident_origin_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream(&ident_create_upper_camel_case, &ident_origin_upper_camel_case, &quote::quote! {#value_snake_case.0});
            let impl_std_convert_from_ident_update_for_ident_origin_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream(&ident_update_upper_camel_case, &ident_origin_upper_camel_case, &quote::quote! {#value_snake_case.0});
            //todo
            let maybe_impl_schemars_json_schema_for_ident_origin_token_stream = if let IsStandartNotNull::True = &is_standart_not_null {
                match &postgresql_json_type {
                    PostgresqlJsonType::UuidUuidAsJsonbString => {
                        let ident_standart_not_null_origin_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                            &ident_standart_not_null_origin_upper_camel_case
                        );
                        let text_ident_standart_not_null_origin_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                            &format!("tests::{ident_standart_not_null_origin_upper_camel_case}")
                        );
                        quote::quote!{
                            const _: () = {
                                #[automatically_derived]
                                #[allow(unused_braces)]
                                impl schemars::JsonSchema for #ident_standart_not_null_origin_upper_camel_case {
                                    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                                        schemars::_private::alloc::borrow::Cow::Borrowed(#ident_standart_not_null_origin_double_quotes_token_stream)
                                    }
                                    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                                        schemars::_private::alloc::borrow::Cow::Borrowed(#text_ident_standart_not_null_origin_double_quotes_token_stream)
                                    }
                                    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                                        { generator.subschema_for::<String>() }
                                    }
                                    fn inline_schema() -> bool {
                                        false
                                    }
                                }
                            };
                        }
                    },
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
                    | PostgresqlJsonType::StdStringStringAsJsonbString => proc_macro2::TokenStream::new(),
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            // match &schemars_json_schema {
            //     SchemarsJsonSchema::Derive => &proc_macro2::TokenStream::new(),
            //     SchemarsJsonSchema::Impl(schema_object_token_stream) => &{
            //         let schema_id_format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("postgresql_crud::postgersql_json_type::{ident_origin_upper_camel_case}"));
            //         let metadata_token_stream = &schema_object_token_stream.metadata;
            //         let instance_type_token_stream = &schema_object_token_stream.instance_type;
            //         let format_token_stream = &schema_object_token_stream.format;
            //         let enum_values_token_stream = &schema_object_token_stream.enum_values;
            //         let const_value_token_stream = &schema_object_token_stream.const_value;
            //         let subschemas_token_stream = &schema_object_token_stream.subschemas;
            //         let number_token_stream = &schema_object_token_stream.number;
            //         let string_token_stream = &schema_object_token_stream.string;
            //         let array_token_stream = &schema_object_token_stream.array;
            //         let object_token_stream = &schema_object_token_stream.object;
            //         let reference_token_stream = &schema_object_token_stream.reference;
            //         let extensions_token_stream = &schema_object_token_stream.extensions;
            //         //todo maybe reuse
            //         quote::quote! {
            //             impl #schemars_json_schema_token_stream for #ident_origin_upper_camel_case {
            //                 fn schema_name() -> String {
            //                     #schema_name_format_handle_token_stream.to_owned()
            //                 }
            //                 fn schema_id() -> std::borrow::Cow<'static, str> {
            //                     std::borrow::Cow::Borrowed(#schema_id_format_handle_token_stream)
            //                 }
            //                 fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
            //                     schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            //                         metadata: #metadata_token_stream,
            //                         instance_type: #instance_type_token_stream,
            //                         format: #format_token_stream,
            //                         enum_values: #enum_values_token_stream,
            //                         const_value: #const_value_token_stream,
            //                         subschemas: #subschemas_token_stream,
            //                         number: #number_token_stream,
            //                         string: #string_token_stream,
            //                         array: #array_token_stream,
            //                         object: #object_token_stream,
            //                         reference: #reference_token_stream,
            //                         extensions: #extensions_token_stream,
            //                     })
            //                 }
            //             }
            //         }
            //     },
            // };
            let maybe_impl_is_string_empty_for_ident_origin_token_stream = if let IsStandartNotNull::True = &is_standart_not_null {
                match &postgresql_json_type {
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
                    | PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => proc_macro2::TokenStream::new(),
                    PostgresqlJsonType::StdStringStringAsJsonbString => postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_content_token_stream(
                        &ident_origin_upper_camel_case,
                        &quote::quote!{self.0.clone().is_empty()}
                    ),
                    PostgresqlJsonType::UuidUuidAsJsonbString => postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_content_token_stream(
                        &ident_origin_upper_camel_case,
                        &quote::quote!{self.0.to_string().is_empty()}
                    ),
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let impl_std_fmt_display_for_ident_origin_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(&proc_macro2::TokenStream::new(), &ident_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {write!(f, "{self:?}")});
            let impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {format!("{self:#?}")});
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_origin_upper_camel_case, &{
                let content_token_stream = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => match &postgresql_json_type {
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
                            | PostgresqlJsonType::StdStringStringAsJsonbString => quote::quote! {Default::default()},
                            PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {uuid::Uuid::new_v4()},
                        },
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {Some(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {vec![#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]},
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {Some(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)},
                    },
                };
                quote::quote! {Self(#content_token_stream)}
            });
            let impl_sqlx_type_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_origin_upper_camel_case, &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&ident_read_inner_upper_camel_case));
            let impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_origin_upper_camel_case, &quote::quote! {sqlx::types::Json(&#self_snake_case.0)});
            quote::quote! {
                #ident_origin_token_stream
                #impl_ident_origin_token_stream
                #impl_std_convert_from_ident_create_for_ident_origin_token_stream
                #impl_std_convert_from_ident_update_for_ident_origin_token_stream
                #maybe_impl_schemars_json_schema_for_ident_origin_token_stream
                #maybe_impl_is_string_empty_for_ident_origin_token_stream
                #impl_std_fmt_display_for_ident_origin_token_stream
                #impl_error_occurence_lib_to_std_string_string_for_ident_origin_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_origin_token_stream
                #impl_sqlx_encode_sqlx_postgres_for_ident_origin_token_stream
            }
        };
        let ident_origin_struct_content_token_stream = quote::quote!{(#ident_origin_upper_camel_case);};
        let ident_table_type_declaration_token_stream = {
            let ident_table_type_declaration_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_partial_ord()//maybe add it to the trait?
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema()
                .build_struct(
                    &ident_table_type_declaration_upper_camel_case,
                    &ident_origin_struct_content_token_stream
                );
            let impl_ident_table_type_declaration_token_stream = {
                quote::quote!{
                    impl #ident_table_type_declaration_upper_camel_case {
                        #pub_new_or_const_new_self_ident_origin_new_value_token_stream
                    }
                }
            };
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_table_type_declaration_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
            //todo maybe add to trait?
            let impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_table_type_declaration_upper_camel_case, &quote::quote! {&#self_snake_case.0});
            //todo maybe add to trait?
            let impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_table_type_declaration_upper_camel_case, &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&ident_read_inner_upper_camel_case));
            quote::quote! {
                #ident_table_type_declaration_token_stream
                #impl_ident_table_type_declaration_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_table_type_declaration_token_stream
                #impl_sqlx_encode_sqlx_postgres_for_ident_table_type_declaration_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_table_type_declaration_token_stream
            }
        };
        let ident_create_token_stream = {
            let ident_create_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema()
                .build_struct(
                    &ident_create_upper_camel_case,
                    &ident_origin_struct_content_token_stream
                );
            let impl_ident_create_token_stream = {
                quote::quote!{
                    impl #ident_create_upper_camel_case {
                        #pub_new_or_const_new_self_ident_origin_new_value_token_stream
                    }
                }
            };
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_create_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
            quote::quote! {
                #ident_create_token_stream
                #impl_ident_create_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream
            }
        };
        let ident_create_for_query_token_stream = {
            let ident_create_for_query_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .build_struct(
                    &ident_create_for_query_upper_camel_case,
                    &ident_origin_struct_content_token_stream
                );
            let impl_ident_create_for_query_token_stream = {
                quote::quote! {
                    impl #ident_create_for_query_upper_camel_case {
                        #pub_new_or_const_new_self_ident_origin_new_value_token_stream
                    }
                }
            };
            let impl_sqlx_encode_sqlx_postgres_for_ident_create_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_create_for_query_upper_camel_case, &quote::quote! {sqlx::types::Json(&#self_snake_case.0)});
            let impl_sqlx_type_sqlx_postgres_for_ident_create_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_create_for_query_upper_camel_case, &ident_origin_upper_camel_case);
            let impl_std_convert_from_ident_create_for_ident_create_for_query_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream(&ident_create_upper_camel_case, &ident_create_for_query_upper_camel_case, &quote::quote! {Self(#value_snake_case.0)});
            let maybe_impl_std_convert_from_ident_update_for_ident_create_for_query_token_stream = if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
                macros_helpers::generate_impl_std_convert_from_token_stream(&ident_update_upper_camel_case, &ident_create_for_query_upper_camel_case, &quote::quote! {Self(#value_snake_case.0)})
            } else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #ident_create_for_query_token_stream
                #impl_ident_create_for_query_token_stream
                #impl_sqlx_encode_sqlx_postgres_for_ident_create_for_query_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_create_for_query_token_stream
                #impl_std_convert_from_ident_create_for_ident_create_for_query_token_stream
                #maybe_impl_std_convert_from_ident_update_for_ident_create_for_query_token_stream
            }
        };
        let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
        let ident_select_token_stream = {
            let ident_select_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy()
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema()
                .build_struct(
                    &ident_select_upper_camel_case,
                    &ArrayDimension::try_from(postgresql_json_type_pattern).map_or_else(
                        |()| quote::quote! {;},
                        |array_dimension| {
                            let mut arguments_token_stream = Vec::new();
                            for element_419a74e5 in 1..=array_dimension.to_usize() {
                                let dimension_number_pagination_token_stream = format!("dimension{element_419a74e5}_pagination").parse::<proc_macro2::TokenStream>()
                                .expect("2ad1faf7-57a8-4cfb-8b71-0082b06436ea");
                                arguments_token_stream.push(quote::quote! {
                                    #dimension_number_pagination_token_stream: #import_path::PaginationStartsWithZero
                                });
                            }
                            quote::quote! {{#(#arguments_token_stream),*}}
                        }
                    )
                );
            let generate_default_some_one_content_token_stream = |default_some_one_or_default_some_one_with_max_page_size: &postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize| {
                let content_token_stream = ArrayDimension::try_from(postgresql_json_type_pattern).map_or_else(
                    |()| proc_macro2::TokenStream::new(),
                    |array_dimension| {
                        let content_token_stream: &dyn quote::ToTokens = match &default_some_one_or_default_some_one_with_max_page_size {
                            postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                            postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_call_token_stream,
                        };
                        let mut arguments_token_stream = Vec::new();
                        for element_d56aec99 in 1..=array_dimension.to_usize() {
                            let dimension_number_pagination_token_stream = format!("dimension{element_d56aec99}_pagination").parse::<proc_macro2::TokenStream>().expect("26ca29fb-fd98-466a-a380-974a6c5d4166");
                            arguments_token_stream.push(quote::quote! {
                                #dimension_number_pagination_token_stream: #content_token_stream
                            });
                        }
                        quote::quote! {#(#arguments_token_stream),*}
                    }
                );
                quote::quote! {Self{#content_token_stream}}
            };
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_select_upper_camel_case, &generate_default_some_one_content_token_stream(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne));
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_postgresql_json_type_ident_select_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_tokens_token_stream(&ident_select_upper_camel_case, &generate_default_some_one_content_token_stream(&postgresql_crud_macros_common::DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize));
            quote::quote! {
                #ident_select_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_postgresql_json_type_ident_select_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size_for_postgresql_json_type_ident_select_token_stream
            }
        };
        let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
        let ident_where_token_stream = match &not_null_or_nullable {
            postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_crud_macros_common::generate_postgresql_type_where_token_stream(
                &{
                    #[derive(Debug, Clone)]
                    enum PostgresqlJsonTypeSpecific {
                        Number,
                        Bool,
                        String,
                    }
                    impl From<&PostgresqlJsonType> for PostgresqlJsonTypeSpecific {
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
                    let postgresql_json_type_specific = PostgresqlJsonTypeSpecific::from(postgresql_json_type);
                    let common_postgresql_json_type_filters = vec![postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Equal { ident: quote::quote! {#ident_table_type_declaration_upper_camel_case} }];
                    let ident_table_type_declaration_upper_camel_case_token_stream = quote::quote! {#ident_table_type_declaration_upper_camel_case};
                    match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => {
                            let common_standart_postgresql_json_type_filters = {
                                let mut vec = common_postgresql_json_type_filters;
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::In {
                                    ident: ident_table_type_declaration_upper_camel_case_token_stream.clone(),
                                });
                                vec
                            };
                            match &postgresql_json_type_specific {
                                PostgresqlJsonTypeSpecific::Number => {
                                    let mut vec = common_standart_postgresql_json_type_filters;
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::GreaterThan {
                                        ident: ident_table_type_declaration_upper_camel_case_token_stream.clone(),
                                    });
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::Between { ident: ident_table_type_declaration_upper_camel_case_token_stream });
                                    vec
                                }
                                PostgresqlJsonTypeSpecific::Bool => common_standart_postgresql_json_type_filters,
                                PostgresqlJsonTypeSpecific::String => {
                                    let mut vec = common_standart_postgresql_json_type_filters;
                                    vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::RegularExpression);
                                    vec
                                }
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                            let array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case = {
                                let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("21eaebaf-cd7a-4625-9232-0e23788a5432")));
                                quote::quote! {#value}
                            };
                            let common_array_dimension1_postgresql_json_type_filters = {
                                let mut vec = common_postgresql_json_type_filters;
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsAllElementsOfArray {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneOverlapsWithArray {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec
                            };
                            match &postgresql_json_type_specific {
                                PostgresqlJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dimension1_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneGreaterThan {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneBetween {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementGreaterThan {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsGreaterThan {
                                        ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case,
                                    });
                                    filters
                                }
                                PostgresqlJsonTypeSpecific::Bool => common_array_dimension1_postgresql_json_type_filters,
                                PostgresqlJsonTypeSpecific::String => {
                                    let mut filters = common_array_dimension1_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::ContainsElementRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                            let array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case = {
                                let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("0c4491c4-8364-4c27-9478-227aefadb086")));
                                quote::quote! {#value}
                            };
                            let array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case = {
                                let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension2_not_null_or_nullable, &postgresql_json_type_pattern.down_by_2().expect("2d4ee5d4-490e-4503-91a7-ed29f73e6219")));
                                quote::quote! {#value}
                            };
                            let common_array_dimension2_postgresql_json_type_filters = {
                                let mut vec = common_postgresql_json_type_filters;
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                    ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsAllElementsOfArray {
                                    ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoOverlapsWithArray {
                                    ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                    ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                    ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec
                            };
                            match &postgresql_json_type_specific {
                                PostgresqlJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dimension2_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoGreaterThan {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoBetween {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElementGreaterThan {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsGreaterThan {
                                        ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case,
                                    });
                                    filters
                                }
                                PostgresqlJsonTypeSpecific::Bool => common_array_dimension2_postgresql_json_type_filters,
                                PostgresqlJsonTypeSpecific::String => {
                                    let mut filters = common_array_dimension2_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneContainsElementRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension3 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                        } => {
                            let array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case = {
                                let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("3450bef4-e5f3-4bcd-b2de-4e4c67143336")));
                                quote::quote! {#value}
                            };
                            let array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case = {
                                let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension2_not_null_or_nullable, &postgresql_json_type_pattern.down_by_2().expect("3c0d10f4-6d7d-45d0-b929-5e307c7d79b1")));
                                quote::quote! {#value}
                            };
                            let array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case = {
                                let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension3_not_null_or_nullable, &postgresql_json_type_pattern.down_by_3().expect("9aaf9e82-0a92-4848-bfd4-de49013972a5")));
                                quote::quote! {#value}
                            };
                            let common_array_dimension3_postgresql_json_type_filters = {
                                let mut vec = common_postgresql_json_type_filters;
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                    ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual {
                                    ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsAllElementsOfArray {
                                    ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeOverlapsWithArray {
                                    ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                    ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual {
                                    ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                    ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn {
                                    ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec
                            };
                            match &postgresql_json_type_specific {
                                PostgresqlJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dimension3_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeGreaterThan {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeBetween {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElementGreaterThan {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsGreaterThan {
                                        ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case,
                                    });
                                    filters
                                }
                                PostgresqlJsonTypeSpecific::Bool => common_array_dimension3_postgresql_json_type_filters,
                                PostgresqlJsonTypeSpecific::String => {
                                    let mut filters = common_array_dimension3_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoContainsElementRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                        PostgresqlJsonTypePattern::ArrayDimension4 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                            dimension4_not_null_or_nullable,
                        } => {
                            let array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case = {
                                let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("550d313b-e925-496d-8a57-87931c573155")));
                                quote::quote! {#value}
                            };
                            let array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case = {
                                let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension2_not_null_or_nullable, &postgresql_json_type_pattern.down_by_2().expect("7bda1424-64c0-402e-9bf8-44d5fb3b9903")));
                                quote::quote! {#value}
                            };
                            let array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case = {
                                let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension3_not_null_or_nullable, &postgresql_json_type_pattern.down_by_3().expect("b43aa5bd-9bba-4f3e-b93b-a41f108262ff")));
                                quote::quote! {#value}
                            };
                            let array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case = {
                                let value = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(dimension4_not_null_or_nullable, &postgresql_json_type_pattern.down_by_4().expect("a246885a-ca72-4e37-a396-b7220e237c7e")));
                                quote::quote! {#value}
                            };
                            let common_array_dimension4_postgresql_json_type_filters = {
                                let mut vec = common_postgresql_json_type_filters;
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneEqual {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoEqual {
                                    ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeEqual {
                                    ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourEqual {
                                    ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthEqual);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::LengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourLengthGreaterThan);
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourContainsAllElementsOfArray {
                                    ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourOverlapsWithArray {
                                    ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::AllElementsEqual {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneAllElementsEqual {
                                    ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoAllElementsEqual {
                                    ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsEqual {
                                    ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionOneIn {
                                    ident: array_dimension1_inner_element_ident_table_type_declaration_upper_camel_case,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionTwoIn {
                                    ident: array_dimension2_inner_element_ident_table_type_declaration_upper_camel_case,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeIn {
                                    ident: array_dimension3_inner_element_ident_table_type_declaration_upper_camel_case,
                                });
                                vec.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourIn {
                                    ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                });
                                vec
                            };
                            match &postgresql_json_type_specific {
                                PostgresqlJsonTypeSpecific::Number => {
                                    let mut filters = common_array_dimension4_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourGreaterThan {
                                        ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourBetween {
                                        ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElementGreaterThan {
                                        ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case.clone(),
                                    });
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsGreaterThan {
                                        ident: array_dimension4_inner_element_ident_table_type_declaration_upper_camel_case,
                                    });
                                    filters
                                }
                                PostgresqlJsonTypeSpecific::Bool => common_array_dimension4_postgresql_json_type_filters,
                                PostgresqlJsonTypeSpecific::String => {
                                    let mut filters = common_array_dimension4_postgresql_json_type_filters;
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionFourRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeContainsElementRegularExpression);
                                    filters.push(postgresql_crud_macros_common::PostgresqlJsonTypeFilter::DimensionThreeAllElementsRegularExpression);
                                    filters
                                }
                            }
                        }
                    }
                }
                .iter()
                .map(|element_f992f593| {
                    let handle: &dyn postgresql_crud_macros_common::PostgresqlFilter = element_f992f593;
                    handle
                })
                .collect(),
                &ident,
                &postgresql_crud_macros_common::ShouldDeriveUtoipaToSchema::True,
                &postgresql_crud_macros_common::ShouldDeriveSchemarsJsonSchema::True,
                &postgresql_crud_macros_common::IsQueryBindMutable::False,
            ),
            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                pub type #ident_where_upper_camel_case = #import_path::NullableJsonObjectPostgresqlTypeWhereFilter<
                    <#ident_not_null_token_stream as #import_path::PostgresqlJsonType>::Where
                >;
            }
        };
        //exists because need to implement .into_inner() for fields (only for read subtype)
        let ident_read_token_stream = {
            //todo maybe add some derive\impl to trait
            let ident_read_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_partial_ord()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema()
                .build_struct(
                    &ident_read_upper_camel_case,
                    &ident_origin_struct_content_token_stream
                );
            let impl_ident_read_token_stream = {
                quote::quote!{
                    impl #ident_read_upper_camel_case {
                        #pub_new_or_const_new_self_ident_origin_new_value_token_stream
                    }
                }
            };
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_read_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
            let impl_sqlx_encode_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_read_upper_camel_case, &quote::quote! {&#self_snake_case.0});
            let impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_read_upper_camel_case, &postgresql_crud_macros_common::generate_sqlx_types_json_type_declaration_token_stream(&ident_read_inner_upper_camel_case));
            quote::quote! {
                #ident_read_token_stream
                #impl_ident_read_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_read_token_stream
                #impl_sqlx_encode_sqlx_postgres_for_ident_read_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_read_token_stream
            }
        };
        let ident_read_only_ids_standart_not_null_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident_standart_not_null_upper_camel_case);
        let ident_read_only_ids_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_partial_eq()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .build_struct(
                &ident_read_only_ids_upper_camel_case,
                &{
                    use postgresql_crud_macros_common::NotNullOrNullable;
                    let std_option_option_unit_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&quote::quote! {()});
                    let vec_token_stream = |value: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&value);
                    let content_token_stream = if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                        match &postgresql_json_type_pattern {
                            PostgresqlJsonTypePattern::Standart => {
                                let token_stream1 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                                };
                                quote::quote! {#token_stream1}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                                let token_stream1 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                                });
                                let token_stream2 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                };
                                quote::quote! {#token_stream2}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                                let token_stream1 = vec_token_stream(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                                });
                                let token_stream2 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                });
                                let token_stream3 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream2,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream2),
                                };
                                quote::quote! {#token_stream3}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension3 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                            } => {
                                let token_stream1 = vec_token_stream(&match &dimension3_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                                });
                                let token_stream2 = vec_token_stream(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                });
                                let token_stream3 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream2,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream2),
                                });
                                let token_stream4 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream3,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream3),
                                };
                                quote::quote! {#token_stream4}
                            }
                            PostgresqlJsonTypePattern::ArrayDimension4 {
                                dimension1_not_null_or_nullable,
                                dimension2_not_null_or_nullable,
                                dimension3_not_null_or_nullable,
                                dimension4_not_null_or_nullable,
                            } => {
                                let token_stream1 = vec_token_stream(&match &dimension4_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream},
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                                });
                                let token_stream2 = vec_token_stream(&match &dimension3_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream1,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream1),
                                });
                                let token_stream3 = vec_token_stream(&match &dimension2_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream2,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream2),
                                });
                                let token_stream4 = vec_token_stream(&match &dimension1_not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream3,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream3),
                                });
                                let token_stream5 = match &not_null_or_nullable {
                                    NotNullOrNullable::NotNull => token_stream4,
                                    NotNullOrNullable::Nullable => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_stream4),
                                };
                                quote::quote! {#token_stream5}
                            }
                        }
                    } else {
                        std_option_option_unit_token_stream
                    };
                    quote::quote!{(pub #import_path::Value<#content_token_stream>);}
                }
            );
        let ident_read_inner_token_stream = {
            let type_token_stream = match &postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => &ident_read_inner_standart_not_null_alias_token_stream,
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => &postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_read_inner_standart_not_null_alias_token_stream),
                },
                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => &{
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream});
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                },
                PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => &{
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream});
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension2_type));
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                },
                PostgresqlJsonTypePattern::ArrayDimension3 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                } => &{
                    let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream});
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension3_type));
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension2_type));
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                },
                PostgresqlJsonTypePattern::ArrayDimension4 {
                    dimension1_not_null_or_nullable,
                    dimension2_not_null_or_nullable,
                    dimension3_not_null_or_nullable,
                    dimension4_not_null_or_nullable,
                } => &{
                    let dimension4_type = dimension4_not_null_or_nullable.maybe_option_wrap(quote::quote! {#ident_read_inner_standart_not_null_alias_token_stream});
                    let dimension3_type = dimension3_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension4_type));
                    let dimension2_type = dimension2_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension3_type));
                    let dimension1_type = dimension1_not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension2_type));
                    not_null_or_nullable.maybe_option_wrap(postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&dimension1_type))
                },
            };
            let impl_std_convert_from_ident_origin_for_ident_read_inner_token_stream = {
                use postgresql_crud_macros_common::NotNullOrNullable;
                let value_dot_zero_token_stream = quote::quote!{#value_snake_case.0};
                let nullable_token_stream = quote::quote!{#value_dot_zero_token_stream.map(Into::into)};
                let into_inner_content_token_stream = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => value_dot_zero_token_stream,
                        NotNullOrNullable::Nullable => nullable_token_stream,
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 {..} |
                    PostgresqlJsonTypePattern::ArrayDimension2 {..} |
                    PostgresqlJsonTypePattern::ArrayDimension3 {..} |
                    PostgresqlJsonTypePattern::ArrayDimension4 {..} => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote::quote!{#value_dot_zero_token_stream.into_iter().map(Into::into).collect()},
                        NotNullOrNullable::Nullable => nullable_token_stream
                    },
                };
                quote::quote! {
                    impl From<#ident_origin_upper_camel_case> for #ident_read_inner_upper_camel_case {
                        fn from(#value_snake_case: #ident_origin_upper_camel_case) -> Self {
                            #into_inner_content_token_stream
                        }
                    }
                }
            };
            quote::quote! {
                pub type #ident_read_inner_upper_camel_case = #type_token_stream;
                #impl_std_convert_from_ident_origin_for_ident_read_inner_token_stream
            }
        };
        let ident_update_token_stream = {
            let ident_update_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .derive_utoipa_to_schema()
                .derive_schemars_json_schema()
                .build_struct(
                    &ident_update_upper_camel_case,
                    &ident_origin_struct_content_token_stream
                );
            let impl_ident_update_token_stream = {
                quote::quote!{
                    impl #ident_update_upper_camel_case {
                        #pub_new_or_const_new_self_ident_origin_new_value_token_stream
                    }
                }
            };
            let impl_error_occurence_lib_to_std_string_string_for_ident_update_token_stream = if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
                macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_update_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {format!("{self:?}")})
            } else {
                proc_macro2::TokenStream::new()
            };
            let impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream =
                postgresql_crud_macros_common::generate_impl_postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_update_upper_camel_case, &quote::quote! {Self(#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)});
            quote::quote! {
                #ident_update_token_stream
                #impl_ident_update_token_stream
                #impl_error_occurence_lib_to_std_string_string_for_ident_update_token_stream
                #impl_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream
            }
        };
        let ident_update_for_query_token_stream = {
            let ident_update_for_query_token_stream = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_copy_if(maybe_derive_copy)
                .derive_partial_eq()
                .derive_serde_serialize()
                .build_struct(
                    &ident_update_for_query_upper_camel_case,
                    &ident_origin_struct_content_token_stream
                );
            let impl_ident_update_for_query_token_stream = {
                quote::quote! {
                    impl #ident_update_for_query_upper_camel_case {
                        #pub_new_or_const_new_self_ident_origin_new_value_token_stream
                    }
                }
            };
            let impl_std_convert_from_ident_update_for_ident_update_for_query_token_stream = macros_helpers::generate_impl_std_convert_from_token_stream(&ident_update_upper_camel_case, &ident_update_for_query_upper_camel_case, &quote::quote! {Self(#value_snake_case.0)});
            //its only for primitive json types
            let impl_sqlx_encode_sqlx_postgres_for_ident_update_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_encode_sqlx_postgres_for_ident_token_stream(&ident_update_for_query_upper_camel_case, &quote::quote! {sqlx::types::Json(&#self_snake_case.0)});
            let impl_sqlx_type_sqlx_postgres_for_ident_update_for_query_token_stream = postgresql_crud_macros_common::generate_impl_sqlx_type_sqlx_postgres_for_ident_token_stream(&ident_update_for_query_upper_camel_case, &ident_origin_upper_camel_case);
            quote::quote! {
                #ident_update_for_query_token_stream
                #impl_ident_update_for_query_token_stream
                #impl_std_convert_from_ident_update_for_ident_update_for_query_token_stream
                #impl_sqlx_encode_sqlx_postgres_for_ident_update_for_query_token_stream
                #impl_sqlx_type_sqlx_postgres_for_ident_update_for_query_token_stream
            }
        };
        let postgresql_crud_macros_common_import_path_postgresql_crud_common = postgresql_crud_macros_common::ImportPath::PostgresqlCrudCommon;
        let impl_postgresql_json_type_for_ident_token_stream = {
            let generate_dimension_number_stringified = |dimensions_number: usize| format!("dimension{dimensions_number}");
            let generate_dimension_number_start_stringified = |dimensions_number: usize| format!("{}_start", generate_dimension_number_stringified(dimensions_number));
            let generate_dimension_number_end_stringified = |dimensions_number: usize| format!("{}_end", generate_dimension_number_stringified(dimensions_number));
            let select_only_created_or_updated_ids_query_part_token_stream = if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                quote::quote! {
                    match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                        Ok(value_f06128be) => Ok(format!("'{field_ident}',jsonb_build_object('value',${value_f06128be}),")),
                        Err(#error_snake_case) => Err(#error_snake_case),
                    }
                }
            } else {
                quote::quote! {Ok(field_ident_jsonb_build_object_value(field_ident))}
            };
            let select_only_created_or_updated_ids_query_bind_token_stream = if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                quote::quote! {
                    if let Err(#error_snake_case) = #query_snake_case.try_bind(#value_snake_case) {
                        return Err(#error_snake_case.to_string());
                    }
                    Ok(#query_snake_case)
                }
            } else {
                quote::quote! {Ok(#query_snake_case)}
            };
            postgresql_crud_macros_common::generate_impl_postgresql_json_type_token_stream(
                &postgresql_crud_macros_common_import_path_postgresql_crud_common,
                &ident,
                &ident_table_type_declaration_upper_camel_case,
                &ident_create_upper_camel_case,
                &ident_create_for_query_upper_camel_case,
                &ident_select_upper_camel_case,
                &match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::False,
                    PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => postgresql_crud_macros_common::IsSelectQueryPartSelfSelectUsed::True,
                },
                &postgresql_crud_macros_common::IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed::False,
                &postgresql_crud_macros_common::IsSelectQueryPartIsPostgresqlTypeUsed::False,
                &{
                    let format_handle = {
                        //last child dimension value does not matter - null or type - works both good
                        use postgresql_crud_macros_common::NotNullOrNullable;
                        let column_name_and_maybe_field_getter_field_ident = format!("{{{}}}->'{{field_ident}}'", naming::ColumnNameAndMaybeFieldGetterSnakeCase);
                        let format_handle = ArrayDimension::try_from(postgresql_json_type_pattern).map_or_else(
                            |()| column_name_and_maybe_field_getter_field_ident.clone(),
                            |array_dimension| {
                                enum ArrayDimensionSelectPattern {
                                    ArrayDimension2 {
                                        dimension1_not_null_or_nullable: NotNullOrNullable,
                                    },
                                    ArrayDimension3 {
                                        dimension1_not_null_or_nullable: NotNullOrNullable,
                                        dimension2_not_null_or_nullable: NotNullOrNullable,
                                    },
                                    ArrayDimension4 {
                                        dimension1_not_null_or_nullable: NotNullOrNullable,
                                        dimension2_not_null_or_nullable: NotNullOrNullable,
                                        dimension3_not_null_or_nullable: NotNullOrNullable,
                                    },
                                }
                                impl TryFrom<&ArrayDimension> for ArrayDimensionSelectPattern {
                                    type Error = ();
                                    fn try_from(value: &ArrayDimension) -> Result<Self, Self::Error> {
                                        match &value {
                                            ArrayDimension::ArrayDimension1 => Err(()),
                                            ArrayDimension::ArrayDimension2 {
                                                dimension1_not_null_or_nullable,
                                            } => Ok(Self::ArrayDimension2 {
                                                dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                                            }),
                                            ArrayDimension::ArrayDimension3 {
                                                dimension1_not_null_or_nullable,
                                                dimension2_not_null_or_nullable,
                                            } => Ok(Self::ArrayDimension3 {
                                                dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                                                dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                            }),
                                            ArrayDimension::ArrayDimension4 {
                                                dimension1_not_null_or_nullable,
                                                dimension2_not_null_or_nullable,
                                                dimension3_not_null_or_nullable,
                                            } => Ok(Self::ArrayDimension4 {
                                                dimension1_not_null_or_nullable: *dimension1_not_null_or_nullable,
                                                dimension2_not_null_or_nullable: *dimension2_not_null_or_nullable,
                                                dimension3_not_null_or_nullable: *dimension3_not_null_or_nullable,
                                            }),
                                        }
                                    }
                                }
                                let generate_jsonb_agg = |jsonb_agg_content: &str, jsonb_array_elements_content: &str, ordinality_content: &str, dimensions_number: usize| {
                                    let dimension_number_start = generate_dimension_number_start_stringified(dimensions_number);
                                    let dimension_number_end = generate_dimension_number_end_stringified(dimensions_number);
                                    format!(
                                        "select jsonb_agg(({jsonb_agg_content})) from jsonb_array_elements(({jsonb_array_elements_content})) with ordinality {ordinality_content} between {{{dimension_number_start}}} and {{{dimension_number_end}}}"
                                    )
                                };
                                ArrayDimensionSelectPattern::try_from(&array_dimension).map_or_else(
                                    |()| generate_jsonb_agg(
                                        "value",
                                        &format!("select {column_name_and_maybe_field_getter_field_ident}"),
                                        "where ordinality",
                                        1,
                                    ),
                                    |array_dimension_select_pattern| {
                                        // Dimension1 does not fit into pattern. its only for 2+ dimensions
                                        let generate_d_number_elem = |content: usize| format!("d{content}_elem");
                                        let generate_d_number_ord = |content: usize| format!("d{content}_elem");
                                        let generate_dot_value = |content: &str| format!("{content}.value");
                                        let generate_as_value_where = |
                                            first_content: &str,
                                            second_content: &str
                                        | format!("as {first_content}(value, {second_content}) where {second_content}");
                                        let one = 1;
                                        generate_jsonb_agg(
                                            &{
                                                let mut current_usize_value = match &array_dimension_select_pattern {
                                                    ArrayDimensionSelectPattern::ArrayDimension2 { .. } => 2,
                                                    ArrayDimensionSelectPattern::ArrayDimension3 { .. } => 3,
                                                    ArrayDimensionSelectPattern::ArrayDimension4 { .. } => 4,
                                                };
                                                match &array_dimension_select_pattern {
                                                    ArrayDimensionSelectPattern::ArrayDimension2 {
                                                        dimension1_not_null_or_nullable,
                                                    } => vec![dimension1_not_null_or_nullable],
                                                    ArrayDimensionSelectPattern::ArrayDimension3 {
                                                        dimension1_not_null_or_nullable,
                                                        dimension2_not_null_or_nullable,
                                                    } => vec![
                                                        dimension2_not_null_or_nullable,
                                                        dimension1_not_null_or_nullable,
                                                    ],
                                                    ArrayDimensionSelectPattern::ArrayDimension4 {
                                                        dimension1_not_null_or_nullable,
                                                        dimension2_not_null_or_nullable,
                                                        dimension3_not_null_or_nullable,
                                                    } => vec![
                                                        dimension3_not_null_or_nullable,
                                                        dimension2_not_null_or_nullable,
                                                        dimension1_not_null_or_nullable,
                                                    ],
                                                }
                                                .into_iter()
                                                .fold(generate_dot_value(&generate_d_number_elem(current_usize_value)), |mut acc_64e08e3a, current_not_null_or_nullable| {
                                                    let current_usize_value_minus_one = current_usize_value.checked_sub(one).expect("a35e873e-a2a1-4a25-8de1-c35dbb0b65af");
                                                    let d_usize_minus_one_elem_value = generate_dot_value(&generate_d_number_elem(current_usize_value_minus_one));
                                                    let value = generate_jsonb_agg(
                                                        &acc_64e08e3a,
                                                        &d_usize_minus_one_elem_value,
                                                        &generate_as_value_where(&generate_d_number_elem(current_usize_value), &generate_d_number_ord(current_usize_value)),
                                                        current_usize_value,
                                                    );
                                                    acc_64e08e3a = match &current_not_null_or_nullable {
                                                        NotNullOrNullable::NotNull => value,
                                                        NotNullOrNullable::Nullable => {
                                                            format!("case when jsonb_typeof({d_usize_minus_one_elem_value})='array' then ({value}) else null end")
                                                        }
                                                    };
                                                    current_usize_value = current_usize_value_minus_one;
                                                    acc_64e08e3a
                                                })
                                            },
                                            &column_name_and_maybe_field_getter_field_ident,
                                            &generate_as_value_where(&generate_d_number_elem(one), &generate_d_number_ord(one)),
                                            one,
                                        )
                                    },
                                )
                            },
                        );
                        match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => format_handle,
                            NotNullOrNullable::Nullable => format!("case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident})='null' then 'null'::jsonb else ({format_handle}) end"),
                        }
                    };
                    let maybe_dimensions_start_end_initialization = ArrayDimension::try_from(postgresql_json_type_pattern).ok().into_iter().flat_map(|array_dimension| {
                        (1..=array_dimension.to_usize()).map(|element_8d56d66d| {
                            let dimension_number_start_token_stream =
                                generate_dimension_number_start_stringified(element_8d56d66d)
                                    .parse::<proc_macro2::TokenStream>()
                                    .expect("77ec13b9-710a-460f-9239-ac9c3680244d");
                            let dimension_number_end_token_stream =
                                generate_dimension_number_end_stringified(element_8d56d66d)
                                    .parse::<proc_macro2::TokenStream>()
                                    .expect("24acbb5e-c0fe-4257-b299-8746887ce198");
                            let dimension_number_pagination_token_stream =
                                format!(
                                    "{}_pagination",
                                    generate_dimension_number_stringified(element_8d56d66d)
                                )
                                .parse::<proc_macro2::TokenStream>()
                                .expect("745c99b3-4e24-46c2-a671-9c7b4dce76f4");
                            quote::quote! {
                                let #dimension_number_start_token_stream = #value_snake_case.#dimension_number_pagination_token_stream.start();
                                let #dimension_number_end_token_stream = #value_snake_case.#dimension_number_pagination_token_stream.end();
                            }
                        })
                    });
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_build_object('{{field_ident}}',jsonb_build_object('value',({format_handle})))"));
                    quote::quote! {
                        #(#maybe_dimensions_start_end_initialization)*
                        Ok(format!(#format_handle_token_stream))
                    }
                },
                &ident_where_upper_camel_case,
                &ident_read_upper_camel_case,
                &ident_read_only_ids_upper_camel_case,
                &{
                    let content_token_stream = if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                        quote::quote! {format!("jsonb_build_object('value',{column_name_and_maybe_field_getter})")}
                    } else {
                        quote::quote! {"jsonb_build_object('value','null'::jsonb)".to_owned()}
                    };
                    quote::quote! {Ok(#content_token_stream)}
                },
                &ident_read_inner_upper_camel_case,
                &{
                    let content_token_stream = quote::quote! {#value_snake_case.0.0};
                    let generate_match_element_zero_token_stream = |
                        match_token_stream: &dyn quote::ToTokens,
                        value_token_stream: &dyn quote::ToTokens,
                        current_content_token_stream: &dyn quote::ToTokens
                    | {
                        quote::quote! {#match_token_stream.map(|#value_token_stream| #value_token_stream.0 #current_content_token_stream)}
                    };
                    let generate_into_iter_map_element_collect_token_stream = |
                        element_token_stream: &dyn quote::ToTokens,
                        current_content_token_stream: &dyn quote::ToTokens
                    | {
                        quote::quote! {.into_iter().map(|#element_token_stream|#current_content_token_stream).collect()}
                    };
                    let generate_into_iter_map_element_collect_not_null_or_nullable_token_stream = |
                        element_token_stream: &dyn quote::ToTokens,
                        current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable
                    | {
                        generate_into_iter_map_element_collect_token_stream(
                            &element_token_stream,
                            &match &current_not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#element_token_stream.0},
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_element_zero_token_stream(
                                    &quote::quote! {#element_token_stream.0},
                                    &quote::quote! {value_f8b0b01d},
                                    &proc_macro2::TokenStream::new()
                                )
                            }
                        )
                    };
                    let generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream = |
                        element_token_stream: &dyn quote::ToTokens,
                        value_token_stream: &dyn quote::ToTokens,
                        current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable,
                        current_content_token_stream: &dyn quote::ToTokens
                    | {
                        match &current_not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => generate_into_iter_map_element_collect_token_stream(
                                &element_token_stream,
                                &quote::quote! {#element_token_stream.0 #current_content_token_stream}
                            ),
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let match_element_zero_token_stream = generate_match_element_zero_token_stream(
                                    &quote::quote! {#element_token_stream.0},
                                    &value_token_stream,
                                    &current_content_token_stream
                                );
                                quote::quote! {.into_iter().map(|#element_token_stream|#match_element_zero_token_stream).collect()}
                            }
                        }
                    };
                    let into_inner_content_token_stream = match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => proc_macro2::TokenStream::new(),
                        PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(
                            &quote::quote!{element_0fdb74a5},
                            dimension1_not_null_or_nullable,
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => {
                            let dimension2_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(
                                &quote::quote!{element_dac5ba56},
                                dimension2_not_null_or_nullable
                            );
                            generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(
                                &quote::quote!{element_cf5646e9},
                                &quote::quote!{value_1c90c80c},
                                dimension1_not_null_or_nullable,
                                &dimension2_not_null_or_nullable_content_token_stream
                            )
                        }
                        PostgresqlJsonTypePattern::ArrayDimension3 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                        } => {
                            let dimension3_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(
                                &quote::quote!{element_c935a865},
                                dimension3_not_null_or_nullable
                            );
                            let dimension2_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(
                                &quote::quote!{element_dc9e788b},
                                &quote::quote!{value_3d1307e8},
                                dimension2_not_null_or_nullable,
                                &dimension3_not_null_or_nullable_content_token_stream
                            );
                            generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(
                                &quote::quote!{element_bf67606b},
                                &quote::quote!{value_721e4164},
                                dimension1_not_null_or_nullable,
                                &dimension2_not_null_or_nullable_content_token_stream
                            )
                        }
                        PostgresqlJsonTypePattern::ArrayDimension4 {
                            dimension1_not_null_or_nullable,
                            dimension2_not_null_or_nullable,
                            dimension3_not_null_or_nullable,
                            dimension4_not_null_or_nullable,
                        } => {
                            let dimension4_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_token_stream(
                                &quote::quote!{element_98961cb7},
                                dimension4_not_null_or_nullable
                            );
                            let dimension3_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(
                                &quote::quote!{element_98961cb7},
                                &quote::quote!{value_995a5fbe},
                                dimension3_not_null_or_nullable,
                                &dimension4_not_null_or_nullable_content_token_stream
                            );
                            let dimension2_not_null_or_nullable_content_token_stream = generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(
                                &quote::quote!{element_34e95172},
                                &quote::quote!{},
                                dimension2_not_null_or_nullable,
                                &dimension3_not_null_or_nullable_content_token_stream
                            );
                            generate_into_iter_map_element_collect_not_null_or_nullable_content_token_stream(
                                &quote::quote!{element_f64124e2},
                                &quote::quote!{value_7fc1b146},
                                dimension1_not_null_or_nullable,
                                &dimension2_not_null_or_nullable_content_token_stream
                            )
                        }
                    };
                    match &not_null_or_nullable {
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#content_token_stream #into_inner_content_token_stream},
                        postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_match_element_zero_token_stream(
                            &content_token_stream,
                            &quote::quote!{value_3432e728},
                            &into_inner_content_token_stream
                        ),
                    }
                },
                &ident_update_upper_camel_case,
                &ident_update_for_query_upper_camel_case,
                &{
                    let jsonb_set_accumulator_snake_case = naming::JsonbSetAccumulatorSnakeCase;
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("jsonb_set({{{jsonb_set_accumulator_snake_case}}},'{{{{{{jsonb_set_path}}}}}}',${{value_26526e0f}})"));
                    quote::quote! {
                        match #import_path::increment_checked_add_one_returning_increment(#increment_snake_case) {
                            Ok(value_26526e0f) => Ok(format!(#format_handle_token_stream)),
                            Err(#error_snake_case) => Err(#error_snake_case),
                        }
                    }
                },
                &postgresql_crud_macros_common::IsUpdateQueryPartSelfUpdateUsed::False,
                &postgresql_crud_macros_common::IsUpdateQueryPartJsonbSetTargetUsed::False,
                &postgresql_crud_macros_common::IsUpdateQueryBindMutable::True,
                &quote::quote! {
                    if let Err(error) = query.try_bind(#value_snake_case) {
                        return Err(error.to_string());
                    }
                    Ok(query)
                },
                &select_only_created_or_updated_ids_query_part_token_stream,
                &if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                    postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::True
                } else {
                    postgresql_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMutable::False
                },
                &select_only_created_or_updated_ids_query_bind_token_stream,
                &select_only_created_or_updated_ids_query_part_token_stream,
                &if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                    postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::True
                } else {
                    postgresql_crud_macros_common::IsSelectOnlyCreatedIdsQueryBindMutable::False
                },
                &select_only_created_or_updated_ids_query_bind_token_stream,
            )
        };
        let maybe_impl_postgresql_json_type_object_vec_element_id_for_ident_origin_token_stream = if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
            let (query_bind_string_as_postgresql_text_create_for_query_token_stream, query_bind_string_as_postgresql_text_update_for_query_token_stream) = {
                enum CreateForQueryOrUpdateForQuery {
                    CreateForQuery,
                    UpdateForQuery,
                }
                let generate_query_bind_string_as_postgresql_text_token_stream = |create_for_query_or_update_for_query: &CreateForQueryOrUpdateForQuery| {
                    let name_token_stream = format!(
                        "query_bind_string_as_postgresql_text_{}_for_query",
                        match &create_for_query_or_update_for_query {
                            CreateForQueryOrUpdateForQuery::CreateForQuery => "create",
                            CreateForQueryOrUpdateForQuery::UpdateForQuery => "update",
                        }
                    )
                    .parse::<proc_macro2::TokenStream>()
                    .expect("f1bcde08-085f-4c98-9a1e-1fb583c9fb6e");
                    let type_token_stream: &dyn quote::ToTokens = match &create_for_query_or_update_for_query {
                        CreateForQueryOrUpdateForQuery::CreateForQuery => &create_for_query_upper_camel_case,
                        CreateForQueryOrUpdateForQuery::UpdateForQuery => &update_for_query_upper_camel_case,
                    };
                    quote::quote! {
                        fn #name_token_stream(
                            #value_snake_case: <Self::PostgresqlJsonType as #import_path::PostgresqlJsonType>::#type_token_stream,
                            mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
                        ) -> Result<
                            sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                            #std_string_string_token_stream
                        > {
                            if let Err(#error_snake_case) = #query_snake_case.try_bind(#value_snake_case.0.0.to_string()) {
                                return Err(#error_snake_case.to_string())
                            }
                            Ok(#query_snake_case)
                        }
                    }
                };
                (generate_query_bind_string_as_postgresql_text_token_stream(&CreateForQueryOrUpdateForQuery::CreateForQuery), generate_query_bind_string_as_postgresql_text_token_stream(&CreateForQueryOrUpdateForQuery::UpdateForQuery))
            };
            quote::quote! {
                impl #import_path::PostgresqlJsonTypeObjectVecElementId for #ident {
                    type PostgresqlJsonType = Self;
                    type #create_for_query_upper_camel_case = #ident_create_for_query_upper_camel_case;
                    type #update_upper_camel_case = #ident_update_upper_camel_case;
                    type #read_inner_upper_camel_case = #ident_read_inner_upper_camel_case;
                    #query_bind_string_as_postgresql_text_create_for_query_token_stream
                    #query_bind_string_as_postgresql_text_update_for_query_token_stream
                    fn get_inner(#value_snake_case: &<Self::PostgresqlJsonType as #import_path::PostgresqlJsonType>::#create_for_query_upper_camel_case) -> &Self::#read_inner_upper_camel_case {
                        &#value_snake_case.0.0
                    }
                    fn increment_checked_add_one(#increment_snake_case: &mut #std_primitive_u64_token_stream) -> Result<#std_primitive_u64_token_stream, #import_path::QueryPartErrorNamed> {
                        #import_path::increment_checked_add_one_returning_increment(#increment_snake_case)
                    }
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let impl_postgresql_json_type_test_cases_for_ident_token_stream = {
            let generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream = |read_or_update: &postgresql_crud_macros_common::ReadOrUpdate| {
                let read_or_update_upper_camel_case = read_or_update.upper_camel_case();
                quote::quote! {<#self_upper_camel_case::#postgresql_json_type_upper_camel_case
                    as
                    #postgresql_crud_macros_common_import_path_postgresql_crud_common::#postgresql_json_type_upper_camel_case
                >::#read_or_update_upper_camel_case::#new_snake_case(#value_snake_case)}
            };
            let standart_not_null_test_cases_vec_name_token_stream = match &postgresql_json_type {
                PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber => quote::quote! {std_primitive_i8_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber => quote::quote! {std_primitive_i16_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber => quote::quote! {std_primitive_i32_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber => quote::quote! {std_primitive_i64_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber => quote::quote! {std_primitive_u8_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber => quote::quote! {std_primitive_u16_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber => quote::quote! {std_primitive_u32_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => quote::quote! {std_primitive_u64_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber => quote::quote! {std_primitive_f32_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => quote::quote! {std_primitive_f64_test_cases_vec},
                PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean => quote::quote! {std_primitive_bool_test_cases_vec},
                PostgresqlJsonType::StdStringStringAsJsonbString => quote::quote! {std_string_string_test_cases_vec},
                PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {uuid_uuid_test_cases_vec},
            };
            let generate_array_dimension_equal_token_stream = |dimension: &postgresql_crud_macros_common::Dimension| {
                use postgresql_crud_macros_common::NotNullOrNullable;
                let dimension_index_number_max = postgresql_crud_macros_common::DimensionIndexNumber::from(dimension);
                let index_number_to_std_primitive_u8 = |dimension_index_number: &postgresql_crud_macros_common::DimensionIndexNumber| -> u8 {
                    match dimension_index_number {
                        postgresql_crud_macros_common::DimensionIndexNumber::Zero => 0,
                        postgresql_crud_macros_common::DimensionIndexNumber::One => 1,
                        postgresql_crud_macros_common::DimensionIndexNumber::Two => 2,
                        postgresql_crud_macros_common::DimensionIndexNumber::Three => 3,
                    }
                };
                let generate_for_index_element_into_iter_enumerate_token_stream = |
                    dimension_index_number: &postgresql_crud_macros_common::DimensionIndexNumber,
                    in_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens,
                    value_token_stream: &dyn quote::ToTokens,
                | {
                    let index_number_token_stream = format!("index_{}", index_number_to_std_primitive_u8(dimension_index_number)).parse::<proc_macro2::TokenStream>().expect("dd0a2fb8-40a5-4d63-95bc-f47a3656f652");
                    quote::quote! {
                        for (#index_number_token_stream, #value_token_stream) in #in_token_stream.into_iter().enumerate() {
                            #content_token_stream
                        }
                    }
                };
                let create_dot_zero_dot_zero_token_stream = quote::quote! {#create_snake_case.0.0};
                //
                //
                //
                let generate_index_increment_token_stream = |index_c1128a3e|format!("index_{index_c1128a3e}").parse::<proc_macro2::TokenStream>().expect("afbe7252-745f-40ad-9bf4-1bb20377b5a5");
                let generate_value_increment_token_stream = |index_0abe6039|format!("value{index_0abe6039}").parse::<proc_macro2::TokenStream>().expect("568d8eb6-df23-4f57-afdd-ef392e3b7f72");
                enum NotNullOrNullableTokenStream {
                    NotNull(proc_macro2::TokenStream),
                    Nullable {
                        some_token_stream: proc_macro2::TokenStream,
                        content_token_stream: proc_macro2::TokenStream,
                    }
                }
                let generate_maybe_if_some_token_stream_alt = |
                    not_null_or_nullable_token_stream: NotNullOrNullableTokenStream,
                    index_a08f4550: u64,
                | {
                    let value_index_token_stream = generate_value_increment_token_stream(index_a08f4550);
                    let value_index_incremented_token_stream = generate_value_increment_token_stream(index_a08f4550.checked_add(1).expect("41a4b7fd-dacf-4cb9-842f-19c2ac90df38"));
                    match not_null_or_nullable_token_stream {
                        NotNullOrNullableTokenStream::NotNull(token_stream_bbcbf9bb) => quote::quote! {#value_index_token_stream #token_stream_bbcbf9bb},
                        NotNullOrNullableTokenStream::Nullable {
                            some_token_stream,
                            content_token_stream,
                        } => quote::quote! {
                            if let Some(#value_index_incremented_token_stream) = #value_index_token_stream #some_token_stream {
                                #content_token_stream
                            }
                        },
                    }
                };






                let generate_for_dot_zero_into_iter_enumerate_token_stream = |
                    index: u64,
                    value_index: u64,
                    enumerate_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens,
                |{
                    let index_increment_token_stream = generate_index_increment_token_stream(index);
                    let value_increment_checked_add_token_stream = generate_value_increment_token_stream(value_index.checked_add(1).expect("565378ff-e076-4594-8a0b-a9dab327fe8c"));
                    quote::quote!{
                        for (#index_increment_token_stream, #value_increment_checked_add_token_stream) in #enumerate_token_stream.0.into_iter().enumerate() {
                            #content_token_stream
                        }
                    }
                };
                let generate_for_value_index_dot_zero_into_iter_enumerate_token_stream = |
                    index: u64,
                    value_index: u64,
                    content_token_stream: &dyn quote::ToTokens,
                |{
                    let index_increment_token_stream = generate_index_increment_token_stream(index);
                    let value_increment_checked_add_token_stream = generate_value_increment_token_stream(value_index.checked_add(1).expect("8ded0128-4027-4755-a1ba-49f7bff874f2"));
                    let value_increment_token_stream = generate_value_increment_token_stream(value_index);
                    quote::quote!{
                        for (#index_increment_token_stream, #value_increment_checked_add_token_stream) in #value_increment_token_stream.0.into_iter().enumerate() {
                            #content_token_stream
                        }
                    }
                };
                let generate_if_let_some_equals_dot_zero_token_stream = |
                    value_index: u64,
                    equal_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens,
                |{
                    let value_increment_checked_add_token_stream = generate_value_increment_token_stream(value_index.checked_add(1).expect("69c567fb-35e9-4269-8c57-0a67c038530e"));
                    quote::quote!{
                        if let Some(#value_increment_checked_add_token_stream) = #equal_token_stream.0 {
                            #content_token_stream
                        }
                    }
                };
                let generate_if_let_some_equals_value_index_dot_zero_token_stream = |
                    value_index: u64,
                    content_token_stream: &dyn quote::ToTokens,
                |{
                    let value_increment_checked_add_token_stream = generate_value_increment_token_stream(value_index.checked_add(1).expect("e691b835-bf9d-4dcc-800f-ad33a276b26a"));
                    let value_increment_token_stream = generate_value_increment_token_stream(value_index);
                    quote::quote!{
                        if let Some(#value_increment_checked_add_token_stream) = #value_increment_token_stream.0 {
                            #content_token_stream
                        }
                    }
                };
                let generate_acc_token_stream_alt = |content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        Some(#import_path::NotEmptyUniqueEnumVec::try_new({
                            let mut acc_049ff0b3 = Vec::new();
                            #content_token_stream
                            acc_049ff0b3
                        }).expect("e99ecd08-0aec-4a25-931d-163319bb8179"))
                    }
                };
                //
                //
                //
                let generate_maybe_if_some_token_stream = |
                    current_not_null_or_nullable: &NotNullOrNullable,
                    some_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens,
                    value_token_stream: &dyn quote::ToTokens,
                | match current_not_null_or_nullable {
                    NotNullOrNullable::NotNull => quote::quote! {#content_token_stream},
                    NotNullOrNullable::Nullable => quote::quote! {
                        if let Some(#value_token_stream) = #some_token_stream {
                            #content_token_stream
                        }
                    },
                };
                let generate_acc_token_stream = |content_token_stream_11dd247a: &dyn quote::ToTokens| {
                    let value_zero_token_stream = generate_value_increment_token_stream(0);
                    let content_token_stream_96de95ec = match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote::quote! {#content_token_stream_11dd247a},
                        NotNullOrNullable::Nullable => quote::quote! {
                            if let Some(#value_zero_token_stream) = #create_dot_zero_dot_zero_token_stream {
                                #content_token_stream_11dd247a
                            }
                        },
                    };
                    quote::quote! {
                        Some(#import_path::NotEmptyUniqueEnumVec::try_new({
                            let mut acc_049ff0b3 = Vec::new();
                            #content_token_stream_96de95ec
                            acc_049ff0b3
                        }).expect("e99ecd08-0aec-4a25-931d-163319bb8179"))
                    }
                };
                let generate_dimension_equal_initialization_token_stream = |
                    current_value_ident_not_null_or_nullable: &NotNullOrNullable,
                    current_value_ident_postgresql_json_type_pattern: &PostgresqlJsonTypePattern,
                    value_token_stream: &dyn quote::ToTokens
                | {
                    let to_number_starting_with_one_word_str = |dimension_index_number: &postgresql_crud_macros_common::DimensionIndexNumber| match dimension_index_number {
                        postgresql_crud_macros_common::DimensionIndexNumber::Zero => "One",
                        postgresql_crud_macros_common::DimensionIndexNumber::One => "Two",
                        postgresql_crud_macros_common::DimensionIndexNumber::Two => "Three",
                        postgresql_crud_macros_common::DimensionIndexNumber::Three => "Four",
                    };
                    let dimension_number_starting_with_one_equal_token_stream = format!("Dimension{}Equal", to_number_starting_with_one_word_str(&dimension_index_number_max)).parse::<proc_macro2::TokenStream>().expect("52fa34ac-5cd1-4ae9-8a1d-832e73a505d7");
                    let postgresql_json_type_where_dimension_number_starting_with_one_equal_token_stream = format!("PostgresqlJsonTypeWhereDimension{}Equal", to_number_starting_with_one_word_str(&dimension_index_number_max)).parse::<proc_macro2::TokenStream>().expect("15d769b0-0767-473c-a2c5-3d0f6e221ced");
                    let current_where_ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&generate_ident_token_stream(&NotNullOrNullable::NotNull, postgresql_json_type_pattern));
                    let current_value_ident_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&generate_ident_token_stream(current_value_ident_not_null_or_nullable, current_value_ident_postgresql_json_type_pattern));
                    let vec_content_token_stream = {
                        let mut content_token_stream = Vec::new();
                        for element_db559599 in 0..=index_number_to_std_primitive_u8(&dimension_index_number_max) {
                            let index_number_token_stream = format!("index_{element_db559599}").parse::<proc_macro2::TokenStream>().expect("f0ce7e73-6d15-4de8-8f15-ce00334ed410");
                            content_token_stream.push(quote::quote! {
                                postgresql_crud_common::UnsignedPartOfStdPrimitiveI32::try_from(
                                    i32::try_from(#index_number_token_stream).expect("5a1818e7-3865-4222-bf6b-31486bd721d2")
                                ).expect("ad1ab73f-fd3b-4162-adb0-bb09a19d31a0")
                            });
                        }
                        quote::quote! {#(#content_token_stream),*}
                    };
                    quote::quote! {
                        #current_where_ident_where_upper_camel_case::#dimension_number_starting_with_one_equal_token_stream(
                            where_filters::#postgresql_json_type_where_dimension_number_starting_with_one_equal_token_stream {
                                logical_operator: #import_path::LogicalOperator::And,
                                dimensions: where_filters::BoundedStdVecVec::try_from(
                                    vec![#vec_content_token_stream]
                                ).expect("82cc0a3c-3e8d-47c4-b317-2795362a9b37"),
                                #value_snake_case: #current_value_ident_table_type_declaration_upper_camel_case::new(#value_token_stream.into()),
                            }
                        )
                    }
                };
                let generate_maybe_if_some_value_dot_zero_token_stream = |
                    current_not_null_or_nullable: &NotNullOrNullable,
                    content_token_stream: &dyn quote::ToTokens,
                    value0_token_stream: &dyn quote::ToTokens,
                    value1_token_stream: &dyn quote::ToTokens
                | generate_maybe_if_some_token_stream(
                    current_not_null_or_nullable,
                    &quote::quote! {#value0_token_stream.0},
                    &content_token_stream,
                    &value1_token_stream
                );
                let generate_not_null_or_nullable_token_stream = |
                    current_not_null_or_nullable: &NotNullOrNullable,
                    current_postgresql_json_type_pattern: &PostgresqlJsonTypePattern,
                    value_token_stream: &dyn quote::ToTokens
                | {
                    let content_token_stream = generate_dimension_equal_initialization_token_stream(
                        current_not_null_or_nullable,
                        current_postgresql_json_type_pattern,
                        &value_token_stream
                    );
                    match not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote::quote! {acc_049ff0b3.push(#content_token_stream);},
                        NotNullOrNullable::Nullable => quote::quote! {
                            match #import_path::NotEmptyUniqueEnumVec::try_new(vec![#content_token_stream]) {
                                Ok(value_9328b66f) => {
                                    acc_049ff0b3.push(#import_path::NullableJsonObjectPostgresqlTypeWhereFilter(Some(value_9328b66f)));
                                },
                                Err(error) => match error {
                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => (),
                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("2f5f648a-4dc6-4699-8656-33870b2c629f")
                                }
                            }
                        },
                    }
                };
                let generate_for_index_element_into_iter_enumerate_three_token_stream = |
                    in_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens,
                    value_token_stream: &dyn quote::ToTokens
                | generate_for_index_element_into_iter_enumerate_token_stream(
                    &postgresql_crud_macros_common::DimensionIndexNumber::Three,
                    &in_token_stream,
                    &content_token_stream,
                    &value_token_stream
                );
                let generate_for_index_element_into_iter_enumerate_two_token_stream = |
                    in_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens,
                    value_token_stream: &dyn quote::ToTokens
                | generate_for_index_element_into_iter_enumerate_token_stream(
                    &postgresql_crud_macros_common::DimensionIndexNumber::Two,
                    &in_token_stream,
                    &content_token_stream,
                    &value_token_stream
                );
                let generate_for_index_element_into_iter_enumerate_one_token_stream = |
                    in_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens,
                    value_token_stream: &dyn quote::ToTokens
                | generate_for_index_element_into_iter_enumerate_token_stream(
                    &postgresql_crud_macros_common::DimensionIndexNumber::One,
                    &in_token_stream,
                    &content_token_stream,
                    &value_token_stream
                );
                let generate_for_index_element_into_iter_enumerate_zero_token_stream = |
                    in_token_stream: &dyn quote::ToTokens,
                    content_token_stream: &dyn quote::ToTokens,
                    value_token_stream: &dyn quote::ToTokens
                | generate_for_index_element_into_iter_enumerate_token_stream(
                    &postgresql_crud_macros_common::DimensionIndexNumber::Zero,
                    &in_token_stream,
                    &content_token_stream,
                    &value_token_stream
                );
                let generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream = |
                    content_token_stream: &dyn quote::ToTokens,
                    index_88b9ffc8: u64,
                | generate_for_index_element_into_iter_enumerate_zero_token_stream(
                    &match not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote::quote!{#create_dot_zero_dot_zero_token_stream},
                        NotNullOrNullable::Nullable => {
                            let value_index_token_stream = generate_value_increment_token_stream(index_88b9ffc8);
                            quote::quote! {#value_index_token_stream.0}
                        },
                    },
                    &content_token_stream,
                    &generate_value_increment_token_stream(index_88b9ffc8.checked_add(1).expect("53943ead-83bb-4f61-9461-6688f78e3817"))
                );
                let generate_for_maybe_if_some_token_stream = |
                    dimension_index_number: &postgresql_crud_macros_common::DimensionIndexNumber,
                    current_not_null_or_nullable: &NotNullOrNullable,
                    content_token_stream: &dyn quote::ToTokens,
                    value0_token_stream: &dyn quote::ToTokens,
                    value1_token_stream: &dyn quote::ToTokens,
                    value2_token_stream: &dyn quote::ToTokens,
                    value3_token_stream: &dyn quote::ToTokens
                | {
                    generate_maybe_if_some_value_dot_zero_token_stream(
                        current_not_null_or_nullable,
                        &generate_for_index_element_into_iter_enumerate_token_stream(
                            dimension_index_number,
                            &quote::quote! {#value0_token_stream.0},
                            &content_token_stream,
                            &value1_token_stream
                        ),
                        &value2_token_stream,
                        &value3_token_stream
                    )
                };
                let generate_down_postgresql_json_type_pattern = || match dimension_index_number_max {
                    postgresql_crud_macros_common::DimensionIndexNumber::Zero => postgresql_json_type_pattern.down_by_1(),
                    postgresql_crud_macros_common::DimensionIndexNumber::One => postgresql_json_type_pattern.down_by_2(),
                    postgresql_crud_macros_common::DimensionIndexNumber::Two => postgresql_json_type_pattern.down_by_3(),
                    postgresql_crud_macros_common::DimensionIndexNumber::Three => postgresql_json_type_pattern.down_by_4(),
                };
                let generate_dimension_index_number_zero_token_stream = |dimension1_not_null_or_nullable: &NotNullOrNullable|
                {
                    generate_acc_token_stream_alt(&{
                        let create_dot_zero_token_stream = quote::quote!{create.0};
                        let content_token_stream = generate_not_null_or_nullable_token_stream(
                            dimension1_not_null_or_nullable,
                            &generate_down_postgresql_json_type_pattern().expect("63f3476d-e0e0-471c-9faa-0a626c8ba75e"),
                            &match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                                (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value1},
                                (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value1},
                                (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value1},
                                (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value1},
                            }
                        );
                        // let dimension1_not_null_or_nullable_token_stream = match &dimension1_not_null_or_nullable {
                        //     NotNullOrNullable::NotNull => generate_for_dot_zero_into_iter_enumerate_token_stream(
                        //         0,
                        //         0,
                        //         &create_dot_zero_token_stream,
                        //         &content_token_stream,
                        //     ),
                        //     NotNullOrNullable::Nullable => generate_for_dot_zero_into_iter_enumerate_token_stream(
                        //         0,
                        //         0,
                        //         &create_dot_zero_token_stream,
                        //         &content_token_stream,
                        //     ),
                        //     // generate_for_dot_zero_into_iter_enumerate_token_stream(
                        //     //     0,
                        //     //     0,
                        //     //     &create_dot_zero_token_stream,
                        //     //     &generate_if_let_some_equals_value_index_dot_zero_token_stream(
                        //     //         1,
                        //     //         &content_token_stream,
                        //     //     )
                        //     // )
                        //     // generate_if_let_some_equals_dot_zero_token_stream(
                        //     //     0,
                        //     //     &create_dot_zero_token_stream,
                        //     //     &generate_for_value_index_dot_zero_into_iter_enumerate_token_stream(
                        //     //         0,
                        //     //         1,
                        //     //         &content_token_stream,
                        //     //     )
                        //     // )
                        // };
                        // println!("not_null_or_nullable {not_null_or_nullable:#?} dimension1_not_null_or_nullable {dimension1_not_null_or_nullable:#?} \n---{dimension1_not_null_or_nullable_token_stream}\n\n\n");
                        // // match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                        // //     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => dimension1_not_null_or_nullable_token_stream,
                        // //     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => dimension1_not_null_or_nullable_token_stream,
                        // //     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => generate_if_let_some_equals_dot_zero_token_stream(
                        // //         1,
                        // //         &quote::quote!{create.0},
                        // //         &dimension1_not_null_or_nullable_token_stream
                        // //     ),
                        // //     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => generate_if_let_some_equals_dot_zero_token_stream(
                        // //         1,
                        // //         &quote::quote!{create.0},
                        // //         &dimension1_not_null_or_nullable_token_stream
                        // //     ),
                        // //     // generate_if_let_some_equals_value_index_dot_zero_token_stream(
                        // //     //     1,
                        // //     //     &dimension1_not_null_or_nullable_token_stream
                        // //     // ),
                        // // }
                        // match &not_null_or_nullable {
                        //     NotNullOrNullable::NotNull => dimension1_not_null_or_nullable_token_stream,
                        //     NotNullOrNullable::Nullable => {
                        //         let f = generate_if_let_some_equals_value_index_dot_zero_token_stream(
                        //             0,
                        //             &dimension1_not_null_or_nullable_token_stream
                        //         );
                        //         println!("@@@{f}");
                        //         f
                        //     }
                        // }
                        match (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                            (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => {
                                let dimension1_not_null_or_nullable_token_stream = generate_for_value_index_dot_zero_into_iter_enumerate_token_stream(
                                    1,
                                    &content_token_stream,
                                );
                            },
                            (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => todo!(),
                            (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => todo!(),
                            (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => todo!(),
                        }
                    })
                };
                // generate_acc_token_stream(
                //     &generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(
                //         &generate_not_null_or_nullable_token_stream(
                //             dimension1_not_null_or_nullable,
                //             &generate_down_postgresql_json_type_pattern().expect("63f3476d-e0e0-471c-9faa-0a626c8ba75e"),
                //             &quote::quote!{value1},
                //         ),
                //         0,
                //     )
                // );
                // let generate_dimension_index_number_one_token_stream = |
                //     dimension1_not_null_or_nullable: &NotNullOrNullable,
                //     dimension2_not_null_or_nullable: &NotNullOrNullable
                // |generate_acc_token_stream(
                //     &{
                //         let dimension2_token_stream = generate_for_index_element_into_iter_enumerate_one_token_stream(
                //             &match &dimension1_not_null_or_nullable {
                //                 NotNullOrNullable::NotNull => quote::quote! {value1.0},
                //                 NotNullOrNullable::Nullable => quote::quote! {value2.0},
                //             },
                //             &generate_not_null_or_nullable_token_stream(
                //                 dimension2_not_null_or_nullable,
                //                 &generate_down_postgresql_json_type_pattern().expect("23f9b122-9788-4673-b996-5d437b363f7e"),
                //                 &quote::quote!{value3},
                //             ),
                //             &quote::quote!{value3}
                //         );
                //         let maybe_if_some_dimension2_token_stream = generate_maybe_if_some_value_dot_zero_token_stream(
                //             dimension1_not_null_or_nullable,
                //             &dimension2_token_stream,
                //             &quote::quote!{value1},
                //             &quote::quote!{value2}
                //         );
                //         let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(
                //             &maybe_if_some_dimension2_token_stream,
                //             0
                //         );
                //         quote::quote! {#dimension1_token_stream}
                //     },
                // );
                // let generate_dimension_index_number_two_token_stream = |
                //     dimension1_not_null_or_nullable: &NotNullOrNullable,
                //     dimension2_not_null_or_nullable: &NotNullOrNullable,
                //     dimension3_not_null_or_nullable: &NotNullOrNullable
                // |generate_acc_token_stream(
                //     &{
                //         let dimension3_token_stream = generate_for_index_element_into_iter_enumerate_two_token_stream(
                //             &match (
                //                 &dimension1_not_null_or_nullable,
                //                 &dimension2_not_null_or_nullable,
                //                 &dimension3_not_null_or_nullable,
                //             ) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2.0},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2.0},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3.0},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3.0},
                //             },
                //             &generate_not_null_or_nullable_token_stream(
                //                 dimension3_not_null_or_nullable,
                //                 &generate_down_postgresql_json_type_pattern().expect("55896a34-0056-48f1-b79b-69391daa149a"),
                //                 &match (
                //                     &dimension1_not_null_or_nullable,
                //                     &dimension2_not_null_or_nullable,
                //                     &dimension3_not_null_or_nullable,
                //                 ) {
                //                     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                     (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                     (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                     (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                     (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 },
                //             ),
                //             &match (
                //                 &dimension1_not_null_or_nullable,
                //                 &dimension2_not_null_or_nullable,
                //                 &dimension3_not_null_or_nullable,
                //             ) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //             },
                //         );
                //         let maybe_if_some_dimension3_token_stream = generate_maybe_if_some_value_dot_zero_token_stream(
                //             dimension2_not_null_or_nullable,
                //             &dimension3_token_stream,
                //             &match (
                //                 &dimension1_not_null_or_nullable,
                //                 &dimension2_not_null_or_nullable,
                //                 &dimension3_not_null_or_nullable,
                //             ) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //             },
                //             &match (
                //                 &dimension1_not_null_or_nullable,
                //                 &dimension2_not_null_or_nullable,
                //                 &dimension3_not_null_or_nullable,
                //             ) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //             },
                //         );
                //         let maybe_if_some_dimension2_token_stream = generate_for_maybe_if_some_token_stream(
                //             &postgresql_crud_macros_common::DimensionIndexNumber::One,
                //             dimension1_not_null_or_nullable,
                //             &maybe_if_some_dimension3_token_stream,
                //             &match (
                //                 &dimension1_not_null_or_nullable,
                //                 &dimension2_not_null_or_nullable,
                //                 &dimension3_not_null_or_nullable,
                //             ) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value1},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value1},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value1},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value1},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //             },
                //             &match (
                //                 &dimension1_not_null_or_nullable,
                //                 &dimension2_not_null_or_nullable,
                //                 &dimension3_not_null_or_nullable,
                //             ) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //             },
                //             &quote::quote!{value1},
                //             &quote::quote!{value2}
                //         );
                //         let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(
                //             &maybe_if_some_dimension2_token_stream,
                //             0
                //         );
                //         quote::quote! {#dimension1_token_stream}
                //     },
                // );
                // let generate_dimension_index_number_three_token_stream = |
                //     dimension1_not_null_or_nullable: &NotNullOrNullable,
                //     dimension2_not_null_or_nullable: &NotNullOrNullable,
                //     dimension3_not_null_or_nullable: &NotNullOrNullable,
                //     dimension4_not_null_or_nullable: &NotNullOrNullable
                // |generate_acc_token_stream(
                //     &{
                //         let dimension4_token_stream = generate_for_index_element_into_iter_enumerate_three_token_stream(
                //             &match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3.0},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3.0},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4.0},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4.0},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4.0},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value4.0},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value5.0},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value5.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value5.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value6.0},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value6.0},
                //             },
                //             &generate_not_null_or_nullable_token_stream(
                //                 dimension4_not_null_or_nullable,
                //                 &generate_down_postgresql_json_type_pattern().expect("9048d6b1-5312-4c91-b48f-7f2adb197135"),
                //                 &quote::quote!{value7},
                //             ),
                //             &quote::quote!{value7}
                //         );
                //         let maybe_if_some_dimension4_token_stream = generate_maybe_if_some_value_dot_zero_token_stream(
                //             dimension3_not_null_or_nullable,
                //             &dimension4_token_stream,
                //             &match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value5},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value5},
                //             },
                //             &match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value5},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value5},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value5},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value6},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value6},
                //             },
                //         );
                //         let maybe_if_some_dimension3_token_stream = generate_for_maybe_if_some_token_stream(
                //             &postgresql_crud_macros_common::DimensionIndexNumber::Two,
                //             dimension2_not_null_or_nullable,
                //             &maybe_if_some_dimension4_token_stream,
                //             &match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //             },
                //             &match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value5},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value5},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value5},
                //             },
                //             &match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //             },
                //             &match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value4},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value4},
                //             },
                //         );
                //         let maybe_if_some_dimension2_token_stream = generate_for_maybe_if_some_token_stream(
                //             &postgresql_crud_macros_common::DimensionIndexNumber::One,
                //             dimension1_not_null_or_nullable,
                //             &maybe_if_some_dimension3_token_stream,
                //             &match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value1},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value1},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value1},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value1},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value1},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value1},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value1},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value1},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //             },
                //             &match (&dimension1_not_null_or_nullable, &dimension2_not_null_or_nullable, &dimension3_not_null_or_nullable, &dimension4_not_null_or_nullable) {
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::NotNull) => quote::quote!{value2},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::NotNull) => quote::quote!{value3},
                //                 (NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable, NotNullOrNullable::Nullable) => quote::quote!{value3},
                //             },
                //             &quote::quote!{value1},
                //             &quote::quote!{value2}
                //         );
                //         let dimension1_token_stream = generate_for_index_element_into_iter_enumerate_zero_starting_value_token_stream(
                //             &maybe_if_some_dimension2_token_stream,
                //             0
                //         );
                //         quote::quote! {#dimension1_token_stream}
                //     },
                // );
                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => none_token_stream.clone(),
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => match dimension_index_number_max {
                        postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_dimension_index_number_zero_token_stream(dimension1_not_null_or_nullable),
                        postgresql_crud_macros_common::DimensionIndexNumber::One | postgresql_crud_macros_common::DimensionIndexNumber::Two | postgresql_crud_macros_common::DimensionIndexNumber::Three => none_token_stream.clone(),
                    },
                    PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, dimension2_not_null_or_nullable } => todo!(),
                    // match dimension_index_number_max {
                    //     postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_dimension_index_number_zero_token_stream(dimension1_not_null_or_nullable),
                    //     postgresql_crud_macros_common::DimensionIndexNumber::One => generate_dimension_index_number_one_token_stream(
                    //         dimension1_not_null_or_nullable,
                    //         dimension2_not_null_or_nullable
                    //     ),
                    //     postgresql_crud_macros_common::DimensionIndexNumber::Two | postgresql_crud_macros_common::DimensionIndexNumber::Three => none_token_stream.clone(),
                    // },
                    PostgresqlJsonTypePattern::ArrayDimension3 {
                        dimension1_not_null_or_nullable,
                        dimension2_not_null_or_nullable,
                        dimension3_not_null_or_nullable,
                    } => todo!(),
                    // match dimension_index_number_max {
                    //     postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_dimension_index_number_zero_token_stream(dimension1_not_null_or_nullable),
                    //     postgresql_crud_macros_common::DimensionIndexNumber::One => generate_dimension_index_number_one_token_stream(
                    //         dimension1_not_null_or_nullable,
                    //         dimension2_not_null_or_nullable
                    //     ),
                    //     postgresql_crud_macros_common::DimensionIndexNumber::Two => generate_dimension_index_number_two_token_stream(
                    //         dimension1_not_null_or_nullable,
                    //         dimension2_not_null_or_nullable,
                    //         dimension3_not_null_or_nullable
                    //     ),
                    //     postgresql_crud_macros_common::DimensionIndexNumber::Three => none_token_stream.clone(),
                    // },
                    PostgresqlJsonTypePattern::ArrayDimension4 {
                        dimension1_not_null_or_nullable,
                        dimension2_not_null_or_nullable,
                        dimension3_not_null_or_nullable,
                        dimension4_not_null_or_nullable,
                    } => todo!(),
                    // {
                    //     match dimension_index_number_max {
                    //         postgresql_crud_macros_common::DimensionIndexNumber::Zero => generate_dimension_index_number_zero_token_stream(dimension1_not_null_or_nullable),
                    //         postgresql_crud_macros_common::DimensionIndexNumber::One => generate_dimension_index_number_one_token_stream(
                    //             dimension1_not_null_or_nullable,
                    //             dimension2_not_null_or_nullable
                    //         ),
                    //         postgresql_crud_macros_common::DimensionIndexNumber::Two => generate_dimension_index_number_two_token_stream(
                    //             dimension1_not_null_or_nullable,
                    //             dimension2_not_null_or_nullable,
                    //             dimension3_not_null_or_nullable
                    //         ),
                    //         postgresql_crud_macros_common::DimensionIndexNumber::Three => generate_dimension_index_number_three_token_stream(
                    //             dimension1_not_null_or_nullable,
                    //             dimension2_not_null_or_nullable,
                    //             dimension3_not_null_or_nullable,
                    //             dimension4_not_null_or_nullable
                    //         )
                    //     }
                    // }
                }
            };
            let option_vec_create_token_stream = {
                use postgresql_crud_macros_common::NotNullOrNullable;
                let generate_some_acc_content_token_stream = |current_not_null_or_nullable: &NotNullOrNullable, current_ident_token_stream: &dyn quote::ToTokens| {
                    let (new_content_token_stream, maybe_acc_push_none_token_stream) = match &current_not_null_or_nullable {
                        NotNullOrNullable::NotNull => (quote::quote! {vec![element_88131059.0.into()]}, proc_macro2::TokenStream::new()),
                        NotNullOrNullable::Nullable => (quote::quote! {Some(element_88131059.0.into())}, quote::quote! {acc_50e99088.push(<Self as #import_path::PostgresqlJsonType>::Create::new(None));}),
                    };
                    //todo check - maybe need to add something here
                    let maybe_acc_push_long_vec_token_stream = match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote::quote! {
                            if let Some(value_a25f1343) = <#current_ident_token_stream as #import_path::PostgresqlJsonTypeTestCases>::#option_vec_create_snake_case() {
                                let mut acc_27624e5e = Vec::new();
                                for element_0dcb405a in value_a25f1343 {
                                    acc_27624e5e.push(element_0dcb405a.0.into());
                                }
                                if !acc_27624e5e.is_empty() {
                                    acc_50e99088.push(<Self as #import_path::PostgresqlJsonType>::Create::new(acc_27624e5e));
                                }
                            }
                        },
                        NotNullOrNullable::Nullable => proc_macro2::TokenStream::new(),
                    };
                    quote::quote! {Some({
                        let mut acc_50e99088 = Vec::new();
                        if let Some(value_8de026a4) = <#current_ident_token_stream as #import_path::PostgresqlJsonTypeTestCases>::#option_vec_create_snake_case() {
                            for element_88131059 in value_8de026a4 {
                                acc_50e99088.push(<Self as #import_path::PostgresqlJsonType>::Create::new(#new_content_token_stream));
                            }
                        }
                        #maybe_acc_push_long_vec_token_stream
                        #maybe_acc_push_none_token_stream
                        acc_50e99088
                    })}
                };
                let content_token_stream = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote::quote! {
                            Some(
                                #import_path::#standart_not_null_test_cases_vec_name_token_stream().into_iter().map(<Self as #import_path::PostgresqlJsonType>::Create::new).collect()
                            )
                        },
                        NotNullOrNullable::Nullable => generate_some_acc_content_token_stream(not_null_or_nullable, &generate_ident_token_stream(&NotNullOrNullable::NotNull, &PostgresqlJsonTypePattern::Standart)),
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_some_acc_content_token_stream(
                        not_null_or_nullable,
                        &match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("dec468c0-09fd-4db3-98e7-7fa9cd565909")),
                            NotNullOrNullable::Nullable => generate_ident_token_stream(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        },
                    ),
                    PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_some_acc_content_token_stream(
                        not_null_or_nullable,
                        &match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("4010ebf7-d6e2-4d6e-a562-a299201c92ec")),
                            NotNullOrNullable::Nullable => generate_ident_token_stream(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        },
                    ),
                    PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_some_acc_content_token_stream(
                        not_null_or_nullable,
                        &match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("acdbb564-b169-40db-9815-2653c0150a4c")),
                            NotNullOrNullable::Nullable => generate_ident_token_stream(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        },
                    ),
                    PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_some_acc_content_token_stream(
                        not_null_or_nullable,
                        &match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("5abf9504-cde0-4c6c-adb9-145b385918a5")),
                            NotNullOrNullable::Nullable => generate_ident_token_stream(&NotNullOrNullable::NotNull, postgresql_json_type_pattern),
                        },
                    ),
                };
                match &postgresql_json_type {
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
                    | PostgresqlJsonType::StdStringStringAsJsonbString => content_token_stream,
                    PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {None},
                }
            };
            let read_only_ids_to_two_dimensional_vec_read_inner_token_stream = {
                use postgresql_crud_macros_common::NotNullOrNullable;
                let (has_len_greater_than_one_token_stream, has_len_greater_than_one_for_for_token_stream) = {
                    let generate_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                        quote::quote! {let has_len_greater_than_one = #content_token_stream;}
                    };
                    (
                        generate_token_stream(&quote::quote! {read_only_ids_to_two_dimensional_vec_read_inner.len() > 1}),
                        generate_token_stream(&quote::quote! {{
                            let mut has_len_greater_than_one = false;
                            for element_4a00ab02 in &read_only_ids_to_two_dimensional_vec_read_inner {
                                if element_4a00ab02.len() > 1 {
                                    has_len_greater_than_one = true;
                                    break;
                                }
                            }
                            has_len_greater_than_one
                        }}),
                    )
                };
                let generate_acc_content_handle_token_stream = |current_ident_token_stream: &dyn quote::ToTokens, has_len_greater_than_one_content_token_stream: &dyn quote::ToTokens| {
                    let current_ident_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&current_ident_token_stream);
                    let option_additional_content_token_stream = {
                        let element_82c7dc0a_clone_token_stream = quote::quote! {element_82c7dc0a.clone()};
                        let first = quote::quote! {vec![#element_82c7dc0a_clone_token_stream]};
                        let second = quote::quote! {vec![#element_82c7dc0a_clone_token_stream, #element_82c7dc0a_clone_token_stream]};
                        let (first_content_token_stream, second_content_token_stream) = match &not_null_or_nullable {
                            NotNullOrNullable::NotNull => (first, second),
                            NotNullOrNullable::Nullable => {
                                let generate_token_stream = |content_token_stream: &dyn quote::ToTokens| quote::quote! {Some(#content_token_stream)};
                                (generate_token_stream(&first), generate_token_stream(&second))
                            }
                        };
                        quote::quote! {
                            let option_additional = {
                                let mut option_additional = None;
                                for element_c4f9bf8f in &read_only_ids_to_two_dimensional_vec_read_inner {
                                    if option_additional.is_some() {
                                        break;
                                    }
                                    for element_82c7dc0a in element_c4f9bf8f {
                                        if option_additional.is_none() {
                                            option_additional = Some((vec![#first_content_token_stream], vec![#second_content_token_stream]));
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                }
                                option_additional
                            };
                        }
                    };
                    let acc_push_vec_content_token_stream = {
                        let content_token_stream = {
                            let inner_content_token_stream = quote::quote! {{
                                let mut acc_6cd5b60a = Vec::new();
                                for element_640f58e8 in read_only_ids_to_two_dimensional_vec_read_inner {
                                    for element_d251d1f6 in element_640f58e8 {
                                        acc_6cd5b60a.push(element_d251d1f6);
                                    }
                                }
                                acc_6cd5b60a
                            }};
                            match &not_null_or_nullable {
                                NotNullOrNullable::NotNull => inner_content_token_stream,
                                NotNullOrNullable::Nullable => quote::quote! {Some(#inner_content_token_stream)},
                            }
                        };
                        quote::quote! {acc_0a07db18.push(vec![#content_token_stream]);}
                    };
                    let maybe_acc_push_vec_none_token_stream = match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => proc_macro2::TokenStream::new(),
                        NotNullOrNullable::Nullable => quote::quote! {acc_0a07db18.push(vec![None]);},
                    };
                    quote::quote! {
                        let mut acc_0a07db18 = Vec::new();
                        let read_only_ids_to_two_dimensional_vec_read_inner = <
                            #current_ident_token_stream
                            as
                            #import_path::PostgresqlJsonTypeTestCases
                        >::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(
                            &#current_ident_read_only_ids_upper_camel_case(read_only_ids.0.clone())
                        );
                        #option_additional_content_token_stream
                        #has_len_greater_than_one_content_token_stream
                        #acc_push_vec_content_token_stream
                        #maybe_acc_push_vec_none_token_stream
                        if let Some(value_3de7fba4) = option_additional {
                            if has_len_greater_than_one {
                                acc_0a07db18.push(value_3de7fba4.0);
                            }
                            if !has_len_greater_than_one {
                                acc_0a07db18.push(value_3de7fba4.1);
                            }
                        }
                        acc_0a07db18
                    }
                };
                let content_token_stream = match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                        NotNullOrNullable::NotNull => quote::quote! {vec![#import_path::#standart_not_null_test_cases_vec_name_token_stream().into()]},
                        NotNullOrNullable::Nullable => quote::quote! {
                            let mut acc_97242d4d = Vec::new();
                            for element_8f3646f9 in <#ident_standart_not_null_upper_camel_case as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_to_two_dimensional_vec_read_inner_snake_case(&#ident_read_only_ids_standart_not_null_upper_camel_case(read_only_ids.0.clone())) {
                                for element_35a4dba9 in element_8f3646f9 {
                                    acc_97242d4d.push(vec![Some(element_35a4dba9)]);
                                }
                            }
                            acc_97242d4d.push(vec![None]);
                            acc_97242d4d
                        },
                    },
                    PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_acc_content_handle_token_stream(
                        &generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("d6f89137-9a47-4f74-afce-0e1959d3dc59")),
                        &match &dimension1_not_null_or_nullable {
                            NotNullOrNullable::NotNull => &has_len_greater_than_one_for_for_token_stream,
                            NotNullOrNullable::Nullable => &has_len_greater_than_one_token_stream,
                        },
                    ),
                    PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_acc_content_handle_token_stream(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("38774398-d485-4c14-84e8-92e06c36c23b")), &has_len_greater_than_one_token_stream),
                    PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_acc_content_handle_token_stream(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("053f4bab-0a8e-457f-9176-50e519b312bb")), &has_len_greater_than_one_token_stream),
                    PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_acc_content_handle_token_stream(&generate_ident_token_stream(dimension1_not_null_or_nullable, &postgresql_json_type_pattern.down_by_1().expect("860f8f15-72ac-4557-a2c6-87b1aa958eb4")), &has_len_greater_than_one_token_stream),
                };
                match &postgresql_json_type {
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
                    | PostgresqlJsonType::StdStringStringAsJsonbString => content_token_stream,
                    PostgresqlJsonType::UuidUuidAsJsonbString => quote::quote! {Vec::new()},
                }
            };
            let read_inner_into_read_with_new_or_try_new_unwraped_token_stream = generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream(&postgresql_crud_macros_common::ReadOrUpdate::Read);
            let read_inner_into_update_with_new_or_try_new_unwraped_token_stream = generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream(&postgresql_crud_macros_common::ReadOrUpdate::Update);
            let read_only_ids_into_option_value_read_inner_token_stream = {
                let content_token_stream = generate_import_path_value_initialization_token_stream(&if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
                    quote::quote! {#value_snake_case.0.#value_snake_case}
                } else {
                    quote::quote! {
                        <Self as #import_path::PostgresqlJsonType>::into_inner(
                            <
                                <Self as #import_path::PostgresqlJsonType>::Read
                                as
                                #postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream
                            >::default_but_option_is_always_some_and_vec_always_contains_one_element()
                        )
                    }
                });
                quote::quote! {Some(#content_token_stream)}
            };
            let update_to_read_only_ids_token_stream = {
                let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                    let generate_iter_or_match_token_stream = |
                        current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable,
                        current_ident_token_stream: &dyn quote::ToTokens,
                        update_current_not_null_or_nullable: &postgresql_crud_macros_common::NotNullOrNullable
                    | {
                        let value_zero_zero_token_stream = quote::quote! {#value_snake_case.0.0};
                        let content_token_stream = {
                            let current_ident_update_token_stream = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&current_ident_token_stream);
                            let content_token_stream = {
                                let content_token_stream = match &update_current_not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {element_aa999306.clone()},
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {value_92de91cc.clone()},
                                };
                                quote::quote! {#current_ident_update_token_stream(#content_token_stream)}
                            };
                            quote::quote! {
                                <
                                    #current_ident_token_stream
                                    as
                                    #import_path::PostgresqlJsonTypeTestCases
                                >::update_to_read_only_ids(&#content_token_stream).0.#value_snake_case
                            }
                        };
                        match &current_not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {
                                #value_zero_zero_token_stream.iter().map(|element_aa999306|#content_token_stream).collect()
                            },
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => quote::quote! {
                                #value_zero_zero_token_stream.as_ref().map(|value_92de91cc| #content_token_stream)
                            },
                        }
                    };
                    match &postgresql_json_type_pattern {
                        PostgresqlJsonTypePattern::Standart => match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#value_snake_case.0.clone().into()},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => generate_iter_or_match_token_stream(
                                not_null_or_nullable,
                                &ident_not_null_token_stream,
                                not_null_or_nullable
                            ),
                        },
                        PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => generate_iter_or_match_token_stream(
                            not_null_or_nullable,
                            &generate_ident_token_stream(
                                &match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                },
                                &match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("e84064c3-5c31-4fa6-8dbc-ba454128c9da"),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                },
                            ),
                            not_null_or_nullable,
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension2 { dimension1_not_null_or_nullable, .. } => generate_iter_or_match_token_stream(
                            not_null_or_nullable,
                            &generate_ident_token_stream(
                                &match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                },
                                &match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("226c6318-6be3-4b85-b2cd-c0b53a2d6bf9"),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                },
                            ),
                            not_null_or_nullable,
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension3 { dimension1_not_null_or_nullable, .. } => generate_iter_or_match_token_stream(
                            not_null_or_nullable,
                            &generate_ident_token_stream(
                                &match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                },
                                &match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("3ae1e9f8-84ec-4369-a633-eca188d9b10a"),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                },
                            ),
                            not_null_or_nullable,
                        ),
                        PostgresqlJsonTypePattern::ArrayDimension4 { dimension1_not_null_or_nullable, .. } => generate_iter_or_match_token_stream(
                            not_null_or_nullable,
                            &generate_ident_token_stream(
                                &match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => *dimension1_not_null_or_nullable,
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                                },
                                &match &not_null_or_nullable {
                                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => postgresql_json_type_pattern.down_by_1().expect("44d51dc5-1b15-4807-ad63-c4fcfb01251c"),
                                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => postgresql_json_type_pattern.clone(),
                                },
                            ),
                            not_null_or_nullable,
                        ),
                    }
                } else {
                    none_token_stream.clone()
                });
                quote::quote! {#ident_read_only_ids_upper_camel_case(#value_initialization_token_stream)}
            };
            let read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream = {
                let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&if let PostgresqlJsonType::UuidUuidAsJsonbString = &postgresql_json_type {
                    quote::quote! {#ident_read_upper_camel_case::new(#value_snake_case.0.#value_snake_case.clone())}
                } else {
                    quote::quote! {#postgresql_crud_common_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream}
                });
                quote::quote! {Some(#value_initialization_token_stream)}
            };
            let previous_read_merged_with_option_update_into_read_token_stream = quote::quote! {
                #option_update_snake_case.map_or(#read_snake_case, |value_f6e37412| #ident_read_upper_camel_case(value_f6e37412.into()))
            };
            let read_only_ids_merged_with_create_into_read_token_stream = {
                let content_token_stream = if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
                    quote::quote! {#ident_origin_upper_camel_case::new(#read_only_ids_snake_case.0.#value_snake_case)}
                } else {
                    quote::quote! {#create_snake_case.into()}
                };
                quote::quote! {#ident_read_upper_camel_case(#content_token_stream)}
            };
            let read_only_ids_merged_with_create_into_option_value_read_token_stream = {
                let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&quote::quote! {
                    <Self as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_read_snake_case(
                        #read_only_ids_snake_case,
                        #create_snake_case
                    )
                });
                quote::quote! {Some(#value_initialization_token_stream)}
            };
            let read_only_ids_merged_with_create_into_table_type_declaration_token_stream = {
                let content_token_stream = if let IsStandartNotNullUuid::True = &is_standart_not_null_uuid {
                    quote::quote! {#ident_origin_upper_camel_case::new(#read_only_ids_snake_case.0.#value_snake_case)}
                } else {
                    quote::quote! {#create_snake_case.into()}
                };
                quote::quote! {#ident_table_type_declaration_upper_camel_case(#content_token_stream)}
            };
            let read_only_ids_merged_with_create_into_where_equal_token_stream = {
                let generate_equal_token_stream = |content_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        where_filters::PostgresqlJsonTypeWhereEqual {
                            logical_operator: #import_path::LogicalOperator::Or,
                            #value_snake_case: #content_token_stream
                        }
                    }
                };
                match &not_null_or_nullable {
                    postgresql_crud_macros_common::NotNullOrNullable::NotNull => {
                        let equal_token_stream = generate_equal_token_stream(&quote::quote! {#ident_table_type_declaration_upper_camel_case::new(#create_snake_case.0.into())});
                        quote::quote! {#ident_where_upper_camel_case::#equal_upper_camel_case(#equal_token_stream)}
                    }
                    postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                        let current_ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&ident_not_null_token_stream);
                        let current_ident_table_type_declaration_upper_camel_case = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_tokens(&ident_not_null_token_stream);
                        let equal_token_stream = generate_equal_token_stream(&quote::quote! {#current_ident_table_type_declaration_upper_camel_case::new(value_18544acf.into())});
                        quote::quote! {
                            #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                #create_snake_case.0.0.map(|value_18544acf| postgresql_crud_common::NotEmptyUniqueEnumVec::try_new(
                                    vec![#current_ident_where_upper_camel_case::#equal_upper_camel_case(#equal_token_stream)]
                                ).expect("88bfa095-a3ab-4d0c-be71-af63c3acd50f"))
                            )
                        }
                    }
                }
            };
            let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream = quote::quote! {
                #import_path::NotEmptyUniqueEnumVec::try_new(vec![
                    <Self as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_where_equal_snake_case(
                        #read_only_ids_snake_case,
                        #create_snake_case
                    )
                ]).expect("56eb9ad4-8f4f-4833-9618-7327f42b0014")
            };
            let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream = quote::quote! {<Self as #import_path::PostgresqlJsonTypeTestCases>::#read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case(
                #read_only_ids_snake_case,
                #create_snake_case
            )};
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream = generate_array_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::One);
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream = generate_array_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Two);
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream = generate_array_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Three);
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream = generate_array_dimension_equal_token_stream(&postgresql_crud_macros_common::Dimension::Four);
            //todo maybe reuse LengthEqual and LengthGreaterThan
            let create_into_postgresql_json_type_option_vec_where_length_equal_token_stream = {
                let generate_token_stream = || {
                    let content_token_stream = {
                        let create_dot_zero_dot_zero = quote::quote! {#create_snake_case.0.0};
                        let content_token_stream = {
                            let content_token_stream: &dyn quote::ToTokens = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => &create_dot_zero_dot_zero,
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => &quote::quote! {value_1bbf74bc.0},
                            };
                            quote::quote! {
                                ::LengthEqual(
                                    where_filters::PostgresqlJsonTypeWhereLengthEqual {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        #value_snake_case: postgresql_crud_common::UnsignedPartOfStdPrimitiveI32::try_from(
                                            i32::try_from(#content_token_stream.len()).expect("64d3424f-86fb-4b44-a437-75aea9997f47")
                                        ).expect("081f4463-0430-4901-8a76-83afcfb3f856"),
                                    }
                                )
                            }
                        };
                        match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#ident_where_upper_camel_case #content_token_stream},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let current_ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&ident_not_null_token_stream);
                                quote::quote! {
                                    #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(
                                        match #create_dot_zero_dot_zero {
                                            Some(value_1bbf74bc) => match #import_path::NotEmptyUniqueEnumVec::try_new(
                                                vec![#current_ident_where_upper_camel_case #content_token_stream]
                                            ) {
                                                Ok(value_d82bbdbe) => Some(value_d82bbdbe),
                                                Err(error) => match error {
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => {
                                                        return None;
                                                    },
                                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("3d7ce854-db39-493c-bde3-a3dec5a8a9c3")
                                                }
                                            },
                                            None => None,
                                        }
                                    )
                                }
                            }
                        }
                    };
                    quote::quote! {
                        match #import_path::NotEmptyUniqueEnumVec::try_new(vec![
                            #content_token_stream
                        ]) {
                            Ok(value_e196e86d) => Some(value_e196e86d),
                            Err(error) => match error {
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("e9f9b021-e884-412e-bc02-7c1dafd35fdc")
                            }
                        }
                    }
                };
                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => none_token_stream.clone(),
                    PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => generate_token_stream(),
                }
            };
            let create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream = {
                let generate_token_stream = || {
                    let content_token_stream = {
                        let create_dot_zero_dot_zero = quote::quote! {#create_snake_case.0.0};
                        let content_token_stream = {
                            let content_token_stream: &dyn quote::ToTokens = match &not_null_or_nullable {
                                postgresql_crud_macros_common::NotNullOrNullable::NotNull => &create_dot_zero_dot_zero,
                                postgresql_crud_macros_common::NotNullOrNullable::Nullable => &quote::quote! {value_68880991.0},
                            };
                            quote::quote! {
                                ::LengthGreaterThan(
                                    where_filters::PostgresqlJsonTypeWhereLengthGreaterThan {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        #value_snake_case: if let Ok(value_762dae1f) = postgresql_crud_common::UnsignedPartOfStdPrimitiveI32::try_from(
                                            if let Ok(value_9dca0200) = i32::try_from(
                                                //todo temp code. make it better checking all cases
                                                match #content_token_stream.len().checked_sub(1) {
                                                    Some(value_92860143) => value_92860143,
                                                    None => {
                                                        return None;
                                                    }
                                                }
                                            ) {
                                                value_9dca0200
                                            }
                                            else {
                                                return None;
                                            }
                                        ) {
                                            value_762dae1f
                                        }
                                        else {
                                            return None;
                                        }
                                    }
                                )
                            }
                        };
                        match &not_null_or_nullable {
                            postgresql_crud_macros_common::NotNullOrNullable::NotNull => quote::quote! {#ident_where_upper_camel_case #content_token_stream},
                            postgresql_crud_macros_common::NotNullOrNullable::Nullable => {
                                let current_ident_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&ident_not_null_token_stream);
                                quote::quote! {
                                    #import_path::NullableJsonObjectPostgresqlTypeWhereFilter(match #create_dot_zero_dot_zero {
                                        Some(value_68880991) => match #import_path::NotEmptyUniqueEnumVec::try_new(
                                            vec![#current_ident_where_upper_camel_case #content_token_stream]
                                        ) {
                                            Ok(value_cdc120a8) => Some(value_cdc120a8),
                                            Err(error) => match error {
                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => {
                                                    return None;
                                                },
                                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("584f801e-e26d-486b-8814-758cd421bee4")
                                            }
                                        },
                                        None => None,
                                    })
                                }
                            }
                        }
                    };
                    quote::quote! {
                        match #import_path::NotEmptyUniqueEnumVec::try_new(vec![#content_token_stream]) {
                            Ok(value_cee8d0ab) => Some(value_cee8d0ab),
                            Err(error) => match error {
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("497359a5-49fb-4152-a9f4-5d1bbda2f926")
                            },
                        }
                    }
                };
                match &postgresql_json_type_pattern {
                    PostgresqlJsonTypePattern::Standart => none_token_stream.clone(),
                    PostgresqlJsonTypePattern::ArrayDimension1 { .. } | PostgresqlJsonTypePattern::ArrayDimension2 { .. } | PostgresqlJsonTypePattern::ArrayDimension3 { .. } | PostgresqlJsonTypePattern::ArrayDimension4 { .. } => generate_token_stream(),
                }
            };
            let generate_dot_checked_sub_one_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote!{#content_token_stream.checked_sub(1)};
            let generate_minus_one_is_finite_then_some_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote!{{
                let value = #content_token_stream - 1.0;
                //The correct way to compare floating point numbers is to define an allowed error margin
                if (#content_token_stream - value).abs() < 0.1 {
                    None
                }
                else {
                    value.is_finite().then_some(value)
                }
            }};
            //todo additonal logic for Option<value> and element of array? optional element of array?
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream = if let PostgresqlJsonTypePattern::Standart = &postgresql_json_type_pattern &&
                let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
            {
                let (
                    int_greater_than_one_less_token_stream,
                    float_greater_than_one_less_token_stream,
                ) = {
                    let generate_greater_than_one_less_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote!{
                        let value_7aa498e8 = #content_token_stream?;
                        match #import_path::NotEmptyUniqueEnumVec::try_new(
                            vec![
                                #import_path::SingleOrMultiple::Single(#ident_where_upper_camel_case::GreaterThan(
                                    where_filters::PostgresqlJsonTypeWhereGreaterThan {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        value: #ident_table_type_declaration_upper_camel_case(
                                            #ident_origin_upper_camel_case(value_7aa498e8)
                                        ),
                                    }
                                ))
                            ]
                        ) {
                            Ok(value_6f3e23b5) => Some(value_6f3e23b5),
                            Err(error) => match error {
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => panic!("11287f54-f840-4076-a16b-1a59a74e6dee"),
                            },
                        }
                    };
                    let create_dot_zero_dot_zero_token_stream = quote::quote!{create.0.0};
                    (
                        generate_greater_than_one_less_token_stream(&generate_dot_checked_sub_one_token_stream(
                            &create_dot_zero_dot_zero_token_stream
                        )),
                        generate_greater_than_one_less_token_stream(&generate_minus_one_is_finite_then_some_token_stream(
                            &create_dot_zero_dot_zero_token_stream
                        )),
                    )
                };
                match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => int_greater_than_one_less_token_stream,
                    PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => float_greater_than_one_less_token_stream,
                    PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                    PostgresqlJsonType::StdStringStringAsJsonbString |
                    PostgresqlJsonType::UuidUuidAsJsonbString => none_token_stream.clone(),
                }
            }
            else {
                none_token_stream.clone()
            };
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_token_stream = if let PostgresqlJsonTypePattern::Standart = &postgresql_json_type_pattern &&
                let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
            {
                let (
                    between_one_less_and_one_more_int_token_stream,
                    between_one_less_and_one_more_float_token_stream
                ) = {
                    let generate_between_one_less_and_one_more_token_stream = |
                        less_token_stream: &dyn quote::ToTokens,
                        more_token_stream: &dyn quote::ToTokens
                    |quote::quote!{
                        if let (Some(start), Some(end)) = (#less_token_stream, #more_token_stream) {
                            match where_filters::Between::try_new(
                                #ident_table_type_declaration_upper_camel_case::new(start),
                                #ident_table_type_declaration_upper_camel_case::new(end)
                            ) {
                                Ok(value_cdde02cc) => match postgresql_crud_common::NotEmptyUniqueEnumVec::try_new(vec![
                                    #import_path::SingleOrMultiple::Single(
                                        #ident_where_upper_camel_case::Between(
                                            where_filters::PostgresqlJsonTypeWhereBetween {
                                                logical_operator: postgresql_crud_common::LogicalOperator::Or,
                                                value: value_cdde02cc,
                                            }
                                        )
                                    )
                                ]) {
                                    Ok(value_41af48fb) => Some(value_41af48fb),
                                    Err(error) => match error {
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("5edabfcc-fcbe-419c-a2be-5bd7e961e6e0")
                                    }
                                },
                                Err(error) => None
                            }
                        }
                        else {
                            None
                        }
                    };
                    (
                        {
                            let generate_content_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote!{create.0.0.#content_token_stream(1)};
                            generate_between_one_less_and_one_more_token_stream(
                                &generate_content_token_stream(&quote::quote!{checked_sub}),
                                &generate_content_token_stream(&quote::quote!{checked_add})
                            )
                        },
                        {
                            let generate_content_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote!{{
                                let value = create.0.0 #content_token_stream 1.0;
                                value.is_finite().then_some(value)
                            }};
                            generate_between_one_less_and_one_more_token_stream(
                                &generate_content_token_stream(&quote::quote!{-}),
                                &generate_content_token_stream(&quote::quote!{+})
                            )
                        }
                    )
                };
                match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => between_one_less_and_one_more_int_token_stream,
                    PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => between_one_less_and_one_more_float_token_stream,
                    PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                    PostgresqlJsonType::StdStringStringAsJsonbString |
                    PostgresqlJsonType::UuidUuidAsJsonbString => none_token_stream.clone()
                }
            }
            else {
                none_token_stream.clone()
            };
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_token_stream = if let PostgresqlJsonTypePattern::Standart = &postgresql_json_type_pattern &&
                let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
            {
                match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => {
                        //todo additional variants to test
                        quote::quote!{
                            match #import_path::NotEmptyUniqueEnumVec::try_new(vec![
                                #import_path::SingleOrMultiple::Single(
                                    #ident_where_upper_camel_case::In(
                                        where_filters::PostgresqlJsonTypeWhereIn {
                                            logical_operator: #import_path::LogicalOperator::Or,
                                            value: where_filters::PostgresqlJsonTypeNotEmptyUniqueVec::try_new(
                                                vec![#ident_table_type_declaration_upper_camel_case::new(#create_snake_case.0.0)]
                                            ).expect("2737c0ed-cf4a-4aba-b749-dc7c4e37ff2e")
                                        }
                                    ),
                                )
                            ]) {
                                Ok(value_1c4f89a4) => Some(value_1c4f89a4),
                                Err(error) => match error {
                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                    #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("16ae359d-3869-421b-b37d-85b0b24835bd")
                                }
                            }
                        }
                    },
                    PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                    PostgresqlJsonType::StdStringStringAsJsonbString |
                    PostgresqlJsonType::UuidUuidAsJsonbString => none_token_stream.clone()
                }
            }
            else {
                none_token_stream.clone()
            };
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_token_stream = if let PostgresqlJsonTypePattern::Standart = &postgresql_json_type_pattern &&
                let postgresql_crud_macros_common::NotNullOrNullable::NotNull = &not_null_or_nullable
            {
                match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber |
                    PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                    PostgresqlJsonType::UuidUuidAsJsonbString => none_token_stream.clone(),
                    PostgresqlJsonType::StdStringStringAsJsonbString => quote::quote!{
                        match #import_path::NotEmptyUniqueEnumVec::try_new(vec![
                            #import_path::SingleOrMultiple::Single(
                                #ident_where_upper_camel_case::RegularExpression(
                                    where_filters::PostgresqlJsonTypeWhereRegularExpression {
                                        logical_operator: #import_path::LogicalOperator::Or,
                                        regular_expression_case: where_filters::RegularExpressionCase::Sensitive,
                                        value: where_filters::RegexRegex(regex::Regex::new(&format!("^{}$", regex::escape(&#create_snake_case.0.0))).expect("3814ff38-0e4d-4173-bf0e-971372b888f6")),
                                    }
                                ),
                            )
                        ]) {
                            Ok(value_75ae8964) => Some(value_75ae8964),
                            Err(error) => match error {
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("b9713787-7160-4a8b-b82c-72617d446184")
                            }
                        }
                    },
                }
            }
            else {
                none_token_stream.clone()
            };
            //todo add contains_element_greater_than for dimension 2,3,4
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than_token_stream = match &postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                    if let (
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull
                    ) = (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                        let (
                            int_greater_than_one_less_token_stream,
                            float_greater_than_one_less_token_stream,
                        ) = {
                            let generate_greater_than_one_less_token_stream = |content_token_stream: &dyn quote::ToTokens|quote::quote!{
                                match #import_path::NotEmptyUniqueEnumVec::try_new({
                                    let mut acc_f95ec4f2 = vec![];
                                    for element_ba78af60 in create.0.0 {
                                        match #content_token_stream {
                                            Some(value) => {
                                                acc_f95ec4f2.push(
                                                    #import_path::SingleOrMultiple::Single(
                                                        #ident_where_upper_camel_case::ContainsElementGreaterThan(
                                                            where_filters::PostgresqlJsonTypeWhereContainsElementGreaterThan {
                                                                logical_operator: #import_path::LogicalOperator::Or,
                                                                value: #ident_standart_not_null_table_type_declaration_upper_camel_case(
                                                                    #ident_standart_not_null_origin_upper_camel_case(value)
                                                                )
                                                            }
                                                        )
                                                    )
                                                );
                                            },
                                            None => {
                                                return None;
                                            },
                                        }
                                    }
                                    acc_f95ec4f2
                                }) {
                                    Ok(value_69c93ec5) => Some(value_69c93ec5),
                                    Err(error) => match error {
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("47e44ecd-d1c9-4d2b-9d9e-4191cad34be9")
                                    }
                                }
                            };
                            let element_dot_zero_token_stream = quote::quote!{element_ba78af60.0};
                            (
                                generate_greater_than_one_less_token_stream(&generate_dot_checked_sub_one_token_stream(
                                    &element_dot_zero_token_stream
                                )),
                                generate_greater_than_one_less_token_stream(&generate_minus_one_is_finite_then_some_token_stream(
                                    &element_dot_zero_token_stream
                                )),
                            )
                        };
                        match &postgresql_json_type {
                            PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber => int_greater_than_one_less_token_stream,
                            PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber => float_greater_than_one_less_token_stream,
                            PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                            PostgresqlJsonType::StdStringStringAsJsonbString |
                            PostgresqlJsonType::UuidUuidAsJsonbString => none_token_stream.clone(),
                        }
                    }
                    else {
                        none_token_stream.clone()
                    }
                },
                PostgresqlJsonTypePattern::Standart |
                PostgresqlJsonTypePattern::ArrayDimension2 {..} |
                PostgresqlJsonTypePattern::ArrayDimension3 {..} |
                PostgresqlJsonTypePattern::ArrayDimension4 {..} => none_token_stream.clone()
            };
            //todo add contains_element_regular_expression for dimension 2,3,4
            let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression_token_stream = match &postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::ArrayDimension1 { dimension1_not_null_or_nullable } => {
                    if let (
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                        postgresql_crud_macros_common::NotNullOrNullable::NotNull
                    ) = (&not_null_or_nullable, &dimension1_not_null_or_nullable) {
                        match &postgresql_json_type {
                            PostgresqlJsonType::StdPrimitiveI8AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI16AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI32AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveI64AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU8AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU16AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU32AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveU64AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveF32AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveF64AsJsonbNumber |
                            PostgresqlJsonType::StdPrimitiveBoolAsJsonbBoolean |
                            PostgresqlJsonType::UuidUuidAsJsonbString => none_token_stream.clone(),
                            PostgresqlJsonType::StdStringStringAsJsonbString => quote::quote!{
                                match #import_path::NotEmptyUniqueEnumVec::try_new(create.0.0.into_iter().map(|element_590fca71| {
                                    #import_path::SingleOrMultiple::Single(
                                        #ident_where_upper_camel_case::ContainsElementRegularExpression(
                                            where_filters::PostgresqlJsonTypeWhereContainsElementRegularExpression {
                                                logical_operator: #import_path::LogicalOperator::Or,
                                                regular_expression_case:
                                                    where_filters::RegularExpressionCase::Sensitive,
                                                value: where_filters::RegexRegex(
                                                    regex::Regex::new(&format!(
                                                        "^{}$",
                                                        regex::escape(&element_590fca71.0)
                                                    ))
                                                    .expect("7d01653a-e82f-4615-bbef-a8c899491f34"),
                                                ),
                                            },
                                        ),
                                    )
                                })
                                .collect()) {
                                    Ok(value_0363f494) => Some(value_0363f494),
                                    Err(error) => match error {
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {..} => None,
                                        #import_path::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {..} => panic!("415a73d9-3665-4dde-9120-662a51626586")
                                    }
                                }
                            }
                        }
                    }
                    else {
                        none_token_stream.clone()
                    }
                },
                PostgresqlJsonTypePattern::Standart |
                PostgresqlJsonTypePattern::ArrayDimension2 {..} |
                PostgresqlJsonTypePattern::ArrayDimension3 {..} |
                PostgresqlJsonTypePattern::ArrayDimension4 {..} => none_token_stream.clone()
            };
            postgresql_crud_macros_common::generate_impl_postgresql_json_type_test_cases_for_ident_token_stream(
                &quote::quote! {#[cfg(feature = "test-utils")]},
                &postgresql_crud_macros_common_import_path_postgresql_crud_common,
                &ident_read_inner_upper_camel_case,
                &ident,
                &option_vec_create_token_stream,
                &read_only_ids_to_two_dimensional_vec_read_inner_token_stream,
                &read_inner_into_read_with_new_or_try_new_unwraped_token_stream,
                &read_inner_into_update_with_new_or_try_new_unwraped_token_stream,
                &read_only_ids_into_option_value_read_inner_token_stream,
                &update_to_read_only_ids_token_stream,
                &read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element_token_stream,
                &previous_read_merged_with_option_update_into_read_token_stream,
                &read_only_ids_merged_with_create_into_read_token_stream,
                &read_only_ids_merged_with_create_into_option_value_read_token_stream,
                &read_only_ids_merged_with_create_into_table_type_declaration_token_stream,
                &read_only_ids_merged_with_create_into_where_equal_token_stream,
                &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream,
                &read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream,
                &create_into_postgresql_json_type_option_vec_where_length_equal_token_stream,
                &create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than_token_stream,
                &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression_token_stream,
            )
        };
        let generated = quote::quote! {
            #ident_token_stream
            #ident_origin_token_stream
            #ident_table_type_declaration_token_stream
            #ident_create_token_stream
            #ident_create_for_query_token_stream
            #ident_select_token_stream
            #ident_where_token_stream
            #ident_read_token_stream
            #ident_read_only_ids_token_stream
            #ident_read_inner_token_stream
            #ident_update_token_stream
            #ident_update_for_query_token_stream
            #impl_postgresql_json_type_for_ident_token_stream
            #maybe_impl_postgresql_json_type_object_vec_element_id_for_ident_origin_token_stream
            #impl_postgresql_json_type_test_cases_for_ident_token_stream
        };
        (
            {
                let field_ident = format!("field_{index}").parse::<proc_macro2::TokenStream>().expect("f992f797-a4df-40d0-9984-3a3a3ad439d7");
                quote::quote! {pub #field_ident: #ident,}.to_string()
            },
            generated.to_string(),
        )
    })
    .collect::<(Vec<String>, Vec<String>)>();
    macros_helpers::maybe_write_token_stream_into_file(
        generate_postgresql_json_types_config.postgresql_table_columns_content_write_into_postgresql_table_columns_using_postgresql_json_types,
        "postgresql_table_columns_using_postgresql_json_types",
        &{
            let fields_content_token_stream = fields_token_stream
                .into_iter()
                .map(|element_7ec253fa| element_7ec253fa.parse::<proc_macro2::TokenStream>().expect("1d8cd8e4-5f51-4aed-a626-79d759d86ebf"))
                .collect::<Vec<proc_macro2::TokenStream>>();
            quote::quote! {
                pub struct PostgresqlTableColumnsUsingPostgresqlJsonTypes {
                    #(#fields_content_token_stream)*
                }
            }
        },
        &macros_helpers::FormatWithCargofmt::True
    );
    let generated = {
        let content_token_stream = postgresql_json_type_array
            .into_iter()
            .map(|element_af9caefa| {
                element_af9caefa
                    .parse::<proc_macro2::TokenStream>()
                    .expect("84e21b40-b5a4-4f4c-86d3-8f6ecfbe1f6e")
            })
            .collect::<Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#content_token_stream)*}
    };
    macros_helpers::maybe_write_token_stream_into_file(
        generate_postgresql_json_types_config
            .whole_content_write_into_generate_postgresql_json_types,
        "generate_postgresql_json_types",
        &generated,
        &macros_helpers::FormatWithCargofmt::True,
    );
    generated.into()
}
