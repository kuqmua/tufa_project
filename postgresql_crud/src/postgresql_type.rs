// generate_postgresql_types::generate_postgresql_types!("All");
generate_postgresql_types::generate_postgresql_types!({
    "Concrete": [
        // {
        //     "postgresql_type": "StdPrimitiveI64AsInt8",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdStringStringAsText",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // }
        // ,
        //todo make ranges generate this dependent types automatically
        {
            "postgresql_type": "StdPrimitiveI32AsInt4",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimeDateAsDate",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgIntervalAsInterval",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateAsDate",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI64AsBigSerialInitializedByPostgresql",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdStringStringAsText",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimeDateAsDate",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimePrimitiveDateTimeAsTimestamp",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        {
            "postgresql_type": "SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        }
    ]
});


#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4Range;
#[derive(Debug, Clone, PartialEq)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin(sqlx::postgres::types::PgRange<StdPrimitiveI32AsNotNullInt4Origin>);
impl SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
    pub fn new(value: sqlx::postgres::types::PgRange<std::primitive::i32>) -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: match value.start {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
            end: match value.end {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        })
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::postgres::types::PgRange<std::primitive::i32>) -> Self {
        Self::new(value)
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin", false as std::primitive::usize + 1 + 1)?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "start", &self.0.start)?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "end", &self.0.end)?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            enum Field {
                Start,
                End,
            }
            impl<'de> serde::Deserialize<'de> for Field {
                fn deserialize<D>(__deserializer: D) -> Result<Field, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = Field;
                        fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(__f, "`start` or `end`")
                        }
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
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4RangeVisitor;
            impl<'de> _serde::de::Visitor<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4RangeVisitor {
                type Value = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin;
                fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                    serde::__private::Formatter::write_str(__f, "struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: serde::de::SeqAccess<'de>,
                {
                    let __field0 = __seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                    let __field1 = __seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                    serde::__private::Ok(SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin(sqlx::postgres::types::PgRange { start: __field0, end: __field1 }))
                }
                #[inline]
                fn visit_map<V>(self, mut map: V) -> Result<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
                {
                    let mut start = None;
                    let mut end = None;
                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::Start => {
                                if start.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                start = Some(map.next_value()?);
                            }
                            Field::End => {
                                if end.is_some() {
                                    return Err(serde::de::Error::duplicate_field("end"));
                                }
                                end = Some(map.next_value()?);
                            }
                        }
                    }
                    let __field0 = start.ok_or_else(|| serde::de::Error::missing_field("\"start\""))?;
                    let __field1 = end.ok_or_else(|| serde::de::Error::missing_field("\"end\""))?;
                    serde::__private::Ok(SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin(sqlx::postgres::types::PgRange { start: __field0, end: __field1 }))
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["start", "end"];
            _serde::Deserializer::deserialize_struct(__deserializer, "SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4Range", FIELDS, SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4RangeVisitor)
        }
    }
};
impl std::fmt::Display for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            end: std::ops::Bound::Excluded(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        })
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self::new(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} int4range not null")
    }
}
pub type SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin;
pub type SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeCreate = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    FindRangesWithinGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementFindRangesWithinGivenRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration>),
    FindRangesThatFullyContainTheGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementFindRangesThatFullyContainTheGivenRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration>),
    StrictlyToLeftOfRange(crate::where_element_filters::PostgresqlTypeWhereElementStrictlyToLeftOfRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration>),
    StrictlyToRightOfRange(crate::where_element_filters::PostgresqlTypeWhereElementStrictlyToRightOfRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration>),
    IncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementIncludedLowerBound<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration>),
    ExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementExcludedUpperBound<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration>),
    GreaterThanIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanIncludedLowerBound<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration>),
    GreaterThanExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanExcludedUpperBound<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration>),
    OverlapWithRange(crate::where_element_filters::PostgresqlTypeWhereElementOverlapWithRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration>),
    AdjacentWithRange(crate::where_element_filters::PostgresqlTypeWhereElementAdjacentWithRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration>),
    RangeLength(crate::where_element_filters::PostgresqlTypeWhereElementRangeLength),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::FindRangesWithinGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::FindRangesThatFullyContainTheGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::StrictlyToLeftOfRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::StrictlyToRightOfRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::IncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::ExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::OverlapWithRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AdjacentWithRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::RangeLength(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::FindRangesWithinGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::FindRangesThatFullyContainTheGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::StrictlyToLeftOfRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::StrictlyToRightOfRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::IncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::ExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::OverlapWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::AdjacentWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::RangeLength(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::FindRangesWithinGivenRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::FindRangesThatFullyContainTheGivenRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::StrictlyToLeftOfRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::StrictlyToRightOfRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::IncludedLowerBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::ExcludedUpperBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThanIncludedLowerBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThanExcludedUpperBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::OverlapWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AdjacentWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::RangeLength(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeRead(SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin);
impl SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeRead {
    pub fn new(value: sqlx::postgres::types::PgRange<std::primitive::i32>) -> Self {
        Self(SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin::new(value))
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::postgres::types::PgRange<std::primitive::i32>) -> Self {
        Self(SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin::new_or_try_new_unwraped_for_test(value))
    }
}
impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeReadInner = sqlx::postgres::types::PgRange<std::primitive::i32>;
pub type SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeUpdate = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin;
impl crate::PostgresqlType for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4Range {
    type TableTypeDeclaration = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration;
    type Create = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeCreate;
    fn create_query_part(_: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                acc.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(acc)
    }
    fn create_query_bind(value: Self::Create, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(value);
        query
    }
    type Select = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeSelect;
    fn select_query_part(_: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeWhereElement;
    type Read = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeRead;
    type ReadInner = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        sqlx::postgres::types::PgRange {
            start: match value.0.0.start {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
            end: match value.0.0.end {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        }
    }
    type Update = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeUpdate;
    fn update_query_part(_: &Self::Update, _: &std::primitive::str, _: &std::primitive::str, _: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                acc.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(acc)
    }
    fn update_query_bind<'a>(value: Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(value);
        query
    }
}
impl crate::tests::PostgresqlTypeTestCases for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4Range {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner> {
        let mut acc = vec![];
        for element in <StdPrimitiveI32AsNotNullInt4 as crate::tests::PostgresqlTypeTestCases>::test_cases() {
            // acc.push(sqlx::postgres::types::PgRange {
            //     start: std::ops::Bound::Included(element.clone()),
            //     end: std::ops::Bound::Included(element.clone()),
            // });
            acc.push(sqlx::postgres::types::PgRange {
                start: std::ops::Bound::Included(element.clone()),
                end: std::ops::Bound::Excluded(element.clone()),
            });
            // acc.push(sqlx::postgres::types::PgRange {
            //     start: std::ops::Bound::Included(element.clone()),
            //     end: std::ops::Bound::Unbounded,
            // });
            // acc.push(sqlx::postgres::types::PgRange {
            //     start: std::ops::Bound::Excluded(element.clone()),
            //     end: std::ops::Bound::Included(element.clone()),
            // });
            // acc.push(sqlx::postgres::types::PgRange {
            //     start: std::ops::Bound::Excluded(element.clone()),
            //     end: std::ops::Bound::Excluded(element.clone()),
            // });
            // acc.push(sqlx::postgres::types::PgRange {
            //     start: std::ops::Bound::Excluded(element.clone()),
            //     end: std::ops::Bound::Unbounded,
            // });
            // acc.push(sqlx::postgres::types::PgRange {
            //     start: std::ops::Bound::Unbounded,
            //     end: std::ops::Bound::Included(element.clone()),
            // });
            // acc.push(sqlx::postgres::types::PgRange {
            //     start: std::ops::Bound::Unbounded,
            //     end: std::ops::Bound::Excluded(element.clone()),
            // });
            // acc.push(sqlx::postgres::types::PgRange {
            //     start: std::ops::Bound::Unbounded,
            //     end: std::ops::Bound::Unbounded,
            // });
        }
        acc
    }
}
