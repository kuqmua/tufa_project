generate_where_element_filters::generate_where_element_filters!();


#[derive(Debug, Clone)]
pub struct PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: crate::RegexRegex,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize
    for PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "logical_operator",
                &self.logical_operator,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "value",
                &self.value.to_string(),
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
impl PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression {
    fn new(logical_operator: crate::LogicalOperator, value: crate::RegexRegex) -> Self {
        Self { logical_operator, value }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                fn expecting(&self, __f: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__f, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
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
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression;
                fn expecting(&self, __f: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__f, "struct PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<crate::RegexRegex>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    Ok(PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression::new(__field0, __field1))
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<crate::RegexRegex> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::RegexRegex>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    Ok(PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression::new(__field0, __field1))
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema
    for PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression {
        fn schema_name() -> std::string::String {
            "PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed(
                "testing::PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression",
            )
        }
        fn json_schema(
            generator: &mut schemars::SchemaGenerator,
        ) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<
                        crate::LogicalOperator,
                    >(
                        object_validation,
                        "logical_operator",
                        false,
                        false,
                        generator.subschema_for::<crate::LogicalOperator>(),
                    );
                }
                {
                    schemars::_private::insert_object_property::<
                        std::string::String,
                    >(
                        object_validation,
                        "value",
                        false,
                        false,
                        generator.subschema_for::<std::string::String>(),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
// impl <'a> crate::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression {
//     fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
//         match increment.checked_add(1) {
//             Some(value) => {
//                 *increment = value;
//                 // println!("column {column}");
//                 // column animal_as_not_null_jsonb_object->'doggie_as_not_null_jsonb_object'->'column_113f3662_35a2_4a7a_9326_03bbd441815f'

//                 // SELECT 'cat'::jsonb->>'' ~* 'john';  -- false
//                 // SELECT '"johnny"'::jsonb->>'' ~* 'john';  -- true
//                 Ok(format!("{}(trim(both '\"' from ({})::text) ~* ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
//             }
//             None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         println!("bind");
//         query = query.bind(
//             // sqlx::types::Json(
//                 self.value.to_string()
//             // )
//         );
//         query
//     }
// }
