#[proc_macro]
pub fn generate_postgresql_json_types(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::EnumIter, enum_extension_lib::EnumExtension)]
    pub enum PostgresqlJsonType {
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
    impl PostgresqlJsonType {
        pub fn full_type_path_initialization_token_stream(&self) -> proc_macro2::TokenStream {
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
    }
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PostgresqlJsonTypeRecord {
        pub postgresql_json_type: PostgresqlJsonType,
        pub not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable,
        pub postgresql_json_type_pattern: PostgresqlJsonTypePattern,
    }
    impl PostgresqlJsonTypeRecord {
        //todo its not all variants
        pub fn all() -> std::vec::Vec<Self> {
            let mut acc = vec![];
            for postgresql_json_type in PostgresqlJsonType::into_array() {
                for postgresql_json_type_pattern in PostgresqlJsonTypePattern::into_array() {
                    acc.push(Self {
                        postgresql_json_type: postgresql_json_type.clone(),
                        not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::NotNull,//todo
                        postgresql_json_type_pattern
                    });
                }
            }
            acc
        }
        pub fn is_vec_element_type(&self) -> std::primitive::bool {
            match &self.postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::Standart => false,
                //todo maybe wrong
                PostgresqlJsonTypePattern::ArrayDimension1 {..} => true,
            }
        }
        pub fn postgresql_json_type_ident_wrapper(&self) -> proc_macro2::TokenStream {
            format!(
                "{}{}{}", 
                &self.not_null_or_nullable.prefix_stringified(),
                &self.postgresql_json_type_pattern.prefix_stringified(),
                self.postgresql_json_type
            ).parse::<proc_macro2::TokenStream>().unwrap()
        }

        pub fn handle_field_type(&self, is_wrapper: std::primitive::bool) -> proc_macro2::TokenStream {
            let postgresql_json_type = &self.postgresql_json_type;
            match (&self.not_null_or_nullable, &self.postgresql_json_type_pattern) {
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::Standart) => {
                    if is_wrapper {
                        quote::quote! {#postgresql_json_type}
                    } else {
                        match &postgresql_json_type {
                            PostgresqlJsonType::StdPrimitiveI8 => quote::quote!{std::primitive::i8},
                            PostgresqlJsonType::StdPrimitiveI16 => quote::quote!{std::primitive::i16},
                            PostgresqlJsonType::StdPrimitiveI32 => quote::quote!{std::primitive::i32},
                            PostgresqlJsonType::StdPrimitiveI64 => quote::quote!{std::primitive::i64},
                            PostgresqlJsonType::StdPrimitiveU8 => quote::quote!{std::primitive::u8},
                            PostgresqlJsonType::StdPrimitiveU16 => quote::quote!{std::primitive::u16},
                            PostgresqlJsonType::StdPrimitiveU32 => quote::quote!{std::primitive::u32},
                            PostgresqlJsonType::StdPrimitiveU64 => quote::quote!{std::primitive::u64},
                            PostgresqlJsonType::StdPrimitiveF32 => quote::quote!{std::primitive::f32},
                            PostgresqlJsonType::StdPrimitiveF64 => quote::quote!{std::primitive::f64},
                            PostgresqlJsonType::StdPrimitiveBool => quote::quote!{std::primitive::bool},
                            PostgresqlJsonType::StdStringString => quote::quote!{std::string::String},
                            PostgresqlJsonType::UuidUuid => quote::quote!{uuid::Uuid},
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
            let postgresql_json_type = &self.postgresql_json_type;
            match &self.postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::Standart => quote::quote! {#postgresql_json_type},
                PostgresqlJsonTypePattern::ArrayDimension1 {..} => quote::quote! {std::vec::Vec<#postgresql_json_type>},
            }
        }
        pub fn wrapper_initialization_token_stream(&self) -> proc_macro2::TokenStream {
            self.handle_initialization_token_stream(true)
        }
        pub fn wrapper_non_optional_initialization_token_stream(&self) -> proc_macro2::TokenStream {
            let crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream =
                token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
            match &self.postgresql_json_type_pattern {
                PostgresqlJsonTypePattern::Standart => quote::quote! {#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream},
                PostgresqlJsonTypePattern::ArrayDimension1 {..} => quote::quote! {vec![#crate_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream]},
            }
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
            PostgresqlJsonType::StdPrimitiveI8 => true,
            PostgresqlJsonType::StdPrimitiveI16 => true,
            PostgresqlJsonType::StdPrimitiveI32 => true,
            PostgresqlJsonType::StdPrimitiveI64 => true,
            PostgresqlJsonType::StdPrimitiveU8 => true,
            PostgresqlJsonType::StdPrimitiveU16 => true,
            PostgresqlJsonType::StdPrimitiveU32 => true,
            PostgresqlJsonType::StdPrimitiveU64 => true,
            PostgresqlJsonType::StdPrimitiveF32 => true,
            PostgresqlJsonType::StdPrimitiveF64 => true,
            PostgresqlJsonType::StdPrimitiveBool => true,
            PostgresqlJsonType::StdStringString => true,
            PostgresqlJsonType::UuidUuid => true,
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
    let postgresql_json_type_array = postgresql_json_type_record_vec
    .par_iter()
    // .into_iter()//just for console prints ordering
    .map(|postgresql_json_type_record|{
        let postgresql_json_type = &postgresql_json_type_record.postgresql_json_type;
        let postgresql_json_type_pattern = &postgresql_json_type_record.postgresql_json_type_pattern;
        let not_null_or_nullable = &postgresql_json_type_record.not_null_or_nullable;

        let ident: &dyn naming::StdFmtDisplayPlusQuoteToTokens = &postgresql_json_type_record.postgresql_json_type_ident_wrapper();
        let ident_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&ident);

        let field_type = &postgresql_json_type_record.field_type();

        let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
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
        let none_upper_camel_case = naming::NoneUpperCamelCase;
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
                    PostgresqlJsonType::StdPrimitiveI8 => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                    PostgresqlJsonType::StdPrimitiveI16 => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                    PostgresqlJsonType::StdPrimitiveI32 => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                    PostgresqlJsonType::StdPrimitiveI64 => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                    PostgresqlJsonType::StdPrimitiveU8 => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                    PostgresqlJsonType::StdPrimitiveU16 => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                    PostgresqlJsonType::StdPrimitiveU32 => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                    PostgresqlJsonType::StdPrimitiveU64 => SchemarsJsonSchema::Impl(schema_object_token_stream_integer),
                    PostgresqlJsonType::StdPrimitiveF32
                    | PostgresqlJsonType::StdPrimitiveF64
                    | PostgresqlJsonType::StdPrimitiveBool
                    | PostgresqlJsonType::StdStringString => SchemarsJsonSchema::Derive,
                    PostgresqlJsonType::UuidUuid => SchemarsJsonSchema::Impl(SchemaObjectTokenStream {
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

        let ident_token_stream = quote::quote! {
            #[derive(Debug)]
            pub struct #ident;
        };
        let ident_origin_token_stream = {
            let ident_origin_token_stream = {
                let maybe_derive_schemars_json_schema_token_stream: &dyn quote::ToTokens = match &schemars_json_schema {
                    SchemarsJsonSchema::Derive => &quote::quote! {schemars::JsonSchema,},
                    SchemarsJsonSchema::Impl(_) => &proc_macro2_token_stream_new,
                };
                quote::quote! {
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
                    pub struct #ident_origin_upper_camel_case(pub #field_type);
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
                    PostgresqlJsonType::StdPrimitiveI8
                    | PostgresqlJsonType::StdPrimitiveI16
                    | PostgresqlJsonType::StdPrimitiveI32
                    | PostgresqlJsonType::StdPrimitiveI64
                    | PostgresqlJsonType::StdPrimitiveU8
                    | PostgresqlJsonType::StdPrimitiveU16
                    | PostgresqlJsonType::StdPrimitiveU32
                    | PostgresqlJsonType::StdPrimitiveU64
                    | PostgresqlJsonType::StdPrimitiveF32
                    | PostgresqlJsonType::StdPrimitiveF64
                    | PostgresqlJsonType::StdPrimitiveBool => &proc_macro2_token_stream_new,
                    PostgresqlJsonType::StdStringString => &postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_token_stream(&ident_origin_upper_camel_case),
                    PostgresqlJsonType::UuidUuid => &postgresql_crud_macros_common::generate_impl_crate_is_string_empty_for_ident_token_stream(&ident_origin_upper_camel_case),
                },
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::Standart) => match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8
                    | PostgresqlJsonType::StdPrimitiveI16
                    | PostgresqlJsonType::StdPrimitiveI32
                    | PostgresqlJsonType::StdPrimitiveI64
                    | PostgresqlJsonType::StdPrimitiveU8
                    | PostgresqlJsonType::StdPrimitiveU16
                    | PostgresqlJsonType::StdPrimitiveU32
                    | PostgresqlJsonType::StdPrimitiveU64
                    | PostgresqlJsonType::StdPrimitiveF32
                    | PostgresqlJsonType::StdPrimitiveF64
                    | PostgresqlJsonType::StdPrimitiveBool
                    | PostgresqlJsonType::StdStringString
                    | PostgresqlJsonType::UuidUuid => &proc_macro2_token_stream_new,
                },
                (postgresql_crud_macros_common::NotNullOrNullable::NotNull, PostgresqlJsonTypePattern::ArrayDimension1 {..}) => match &postgresql_json_type {
                    PostgresqlJsonType::StdPrimitiveI8
                    | PostgresqlJsonType::StdPrimitiveI16
                    | PostgresqlJsonType::StdPrimitiveI32
                    | PostgresqlJsonType::StdPrimitiveI64
                    | PostgresqlJsonType::StdPrimitiveU8
                    | PostgresqlJsonType::StdPrimitiveU16
                    | PostgresqlJsonType::StdPrimitiveU32
                    | PostgresqlJsonType::StdPrimitiveU64
                    | PostgresqlJsonType::StdPrimitiveF32
                    | PostgresqlJsonType::StdPrimitiveF64
                    | PostgresqlJsonType::StdPrimitiveBool
                    | PostgresqlJsonType::StdStringString
                    | PostgresqlJsonType::UuidUuid => &proc_macro2_token_stream_new,
                },
                (postgresql_crud_macros_common::NotNullOrNullable::Nullable, PostgresqlJsonTypePattern::ArrayDimension1 {..}) => &proc_macro2_token_stream_new,
            };
            let impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_origin_token_stream = postgresql_crud_macros_common::generate_impl_crate_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
                &ident_origin_upper_camel_case,
                &{
                    let content_token_stream = postgresql_json_type_record.initialization_token_stream();
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
                        PostgresqlJsonType::StdPrimitiveI8
                        | PostgresqlJsonType::StdPrimitiveI16
                        | PostgresqlJsonType::StdPrimitiveI32
                        | PostgresqlJsonType::StdPrimitiveI64
                        | PostgresqlJsonType::StdPrimitiveU8
                        | PostgresqlJsonType::StdPrimitiveU16
                        | PostgresqlJsonType::StdPrimitiveU32
                        | PostgresqlJsonType::StdPrimitiveU64
                        | PostgresqlJsonType::StdPrimitiveF32
                        | PostgresqlJsonType::StdPrimitiveF64 => Self::Number,
                        PostgresqlJsonType::StdPrimitiveBool => Self::Bool,
                        PostgresqlJsonType::StdStringString | PostgresqlJsonType::UuidUuid => Self::String,
                    }
                }
            }
            let postgresql_json_type_specific = PostgresqlJsonTypeSpecific::from(&postgresql_json_type_record.postgresql_json_type);

            let is_vec_element_type = postgresql_json_type_record.is_vec_element_type();
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
            //this is for not nullable\not optionable filters like GreaterThan, Regular expression, etc.
            let postgresql_json_type_ident_wrapper_relevant_only_for_not_null = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&PostgresqlJsonTypeRecord {
                postgresql_json_type: postgresql_json_type_record.postgresql_json_type.clone(),
                not_null_or_nullable: postgresql_crud_macros_common::NotNullOrNullable::NotNull,
                postgresql_json_type_pattern: PostgresqlJsonTypePattern::Standart,//todo
            }
            .postgresql_json_type_ident_wrapper());
            let generate_where_element_variants_types_generic_token_stream =
                |is_relevant_only_for_not_null: std::primitive::bool| -> &dyn naming::StdFmtDisplayPlusQuoteToTokens { if is_relevant_only_for_not_null { 
                    &postgresql_json_type_ident_wrapper_relevant_only_for_not_null
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
            match &postgresql_json_type_record.postgresql_json_type_pattern {
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
        generated.to_string()
    })
    .collect::<std::vec::Vec<String>>();
    let generated = {
        let postgresql_json_type_array = postgresql_json_type_array
        .into_iter()
        .map(|element|{
            element.parse::<proc_macro2::TokenStream>().unwrap()
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {#(#postgresql_json_type_array)*}
    };
    // if ident == "" {
    //     println!("{generated}");
    //     println!("-------");
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file("GeneratePostgresqlJsonTypes", &generated);
    // }
    generated.into()
}
