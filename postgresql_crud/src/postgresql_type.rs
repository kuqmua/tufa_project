// generate_postgresql_types::generate_postgresql_types!("All");
// generate_postgresql_types::generate_postgresql_types!({
//     "Concrete": [
//         // {
//         //     "postgresql_type": "StdPrimitiveI64AsInt8",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         {
//             "postgresql_type": "StdStringStringAsText",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         }
//         ,
//         // {
//         //     "postgresql_type": "StdPrimitiveI32AsInt4",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI16AsInt2",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },

//         {
//             "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "Nullable",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "Nullable"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdPrimitiveI64AsBigSerialInitializedByPostgresql",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": "Standart"
//         // },
//         // {
//         //     "postgresql_type": "StdStringStringAsText",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesTimeDateAsDate",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // }
//         // ,
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         // {
//         //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
//         //     "not_null_or_nullable": "NotNull",
//         //     "postgresql_type_pattern": {
//         //         "ArrayDimension1": {
//         //             "dimension1_not_null_or_nullable": "NotNull"
//         //         }
//         //     }
//         // },
//         {
//             "postgresql_type": "SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql",
//             "not_null_or_nullable": "NotNull",
//             "postgresql_type_pattern": "Standart"
//         }
//     ]
// });


#[derive(Debug)]
pub struct StdStringStringAsNotNullText;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize)]
pub struct StdStringStringAsNotNullTextOrigin(std::string::String);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdStringStringAsNotNullTextOriginTryNewErrorNamed {
    ContainsNullByte {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdStringStringAsNotNullTextOrigin {
    pub fn try_new(value: std::string::String) -> Result<Self, StdStringStringAsNotNullTextOriginTryNewErrorNamed> {
        if value.find('\0').is_some() {
            Err(StdStringStringAsNotNullTextOriginTryNewErrorNamed::ContainsNullByte {
                value,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        } else {
            Ok(Self(value))
        }
    }
    pub fn new_or_try_new_unwraped_for_test(value: std::string::String) -> Self {
        Self::try_new(value).unwrap()
    }
}
impl crate::IsStringEmpty for StdStringStringAsNotNullTextOrigin {
    fn is_string_empty(&self) -> std::primitive::bool {
        self.0.to_string().is_empty()
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdStringStringAsNotNullTextOrigin {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<StdStringStringAsNotNullTextOrigin>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdStringStringAsNotNullTextOrigin;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct StdStringStringAsNotNullTextOrigin")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::string::String = <std::string::String as _serde::Deserialize>::deserialize(__e)?;
                    match StdStringStringAsNotNullTextOrigin::try_new(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct StdStringStringAsNotNullTextOrigin with 1 element"));
                        }
                    };
                    match StdStringStringAsNotNullTextOrigin::try_new(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "StdStringStringAsNotNullTextOrigin",
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdStringStringAsNotNullTextOrigin>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for StdStringStringAsNotNullTextOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for StdStringStringAsNotNullTextOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdStringStringAsNotNullTextOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for StdStringStringAsNotNullTextOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::string::String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::string::String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdStringStringAsNotNullTextOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdStringStringAsNotNullTextOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::string::String as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for StdStringStringAsNotNullTextOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <std::string::String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl StdStringStringAsNotNullTextOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} text not null")
    }
}
pub type StdStringStringAsNotNullTextTableTypeDeclaration = StdStringStringAsNotNullTextOrigin;
pub type StdStringStringAsNotNullTextCreate = StdStringStringAsNotNullTextOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdStringStringAsNotNullTextSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdStringStringAsNotNullTextSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum StdStringStringAsNotNullTextWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<StdStringStringAsNotNullTextOrigin>),
    RegularExpression(crate::where_element_filters::PostgresqlTypeWhereElementRegularExpression),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for StdStringStringAsNotNullTextWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::RegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::RegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for StdStringStringAsNotNullTextWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdStringStringAsNotNullTextWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::RegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdStringStringAsNotNullTextRead(StdStringStringAsNotNullTextOrigin);
impl StdStringStringAsNotNullTextRead {
    pub fn try_new(value: std::string::String) -> Result<Self, StdStringStringAsNotNullTextOriginTryNewErrorNamed> {
        match StdStringStringAsNotNullTextOrigin::try_new(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
    pub fn new_or_try_new_unwraped_for_test(value: std::string::String) -> Self {
        Self(StdStringStringAsNotNullTextOrigin::new_or_try_new_unwraped_for_test(value))
    }
}
impl error_occurence_lib::ToStdStringString for StdStringStringAsNotNullTextRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdStringStringAsNotNullTextRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdStringStringAsNotNullTextRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdStringStringAsNotNullTextRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <StdStringStringAsNotNullTextOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdStringStringAsNotNullTextRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <StdStringStringAsNotNullTextOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <StdStringStringAsNotNullTextOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type StdStringStringAsNotNullTextReadInner = std::string::String;
pub type StdStringStringAsNotNullTextUpdate = StdStringStringAsNotNullTextOrigin;
impl crate::PostgresqlType for StdStringStringAsNotNullText {
    type TableTypeDeclaration = StdStringStringAsNotNullTextTableTypeDeclaration;
    type Create = StdStringStringAsNotNullTextCreate;
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
    type Select = StdStringStringAsNotNullTextSelect;
    fn select_query_part(_: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = StdStringStringAsNotNullTextWhereElement;
    type Read = StdStringStringAsNotNullTextRead;
    type ReadInner = StdStringStringAsNotNullTextReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.0
    }
    type Update = StdStringStringAsNotNullTextUpdate;
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
impl crate::tests::PostgresqlTypeTestCases for StdStringStringAsNotNullText {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner> {
        vec![
            "".to_string(),
            "a".to_string(),
            "Hello, world!".to_string(),
            "   ".to_string(),
            "\n\r\t".to_string(),
            "1234567890".to_string(),
            "😀".to_string(),
            "こんにちは".to_string(),
            "🌍🚀✨ Rust 💖🦀".to_string(),
            "a".repeat(1024),
            "line1\nline2\nline3".to_string(),
            String::from_utf8_lossy(&[0xF0, 0x9F, 0x92, 0x96]).to_string(),
        ]
    }
}
#[derive(Debug)]
pub struct SqlxTypesChronoNaiveTimeAsNotNullTime;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize)]
pub struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin(sqlx::types::chrono::NaiveTime);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewErrorNamed {
    InvalidHourOrMinuteOrSecondOrMicrosecond {
        #[eo_to_std_string_string_serialize_deserialize]
        hour: crate::Hour,
        #[eo_to_std_string_string_serialize_deserialize]
        minute: crate::Minute,
        #[eo_to_std_string_string_serialize_deserialize]
        second: crate::Second,
        #[eo_to_std_string_string_serialize_deserialize]
        microsecond: crate::Microsecond,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    pub fn try_new(hour: crate::Hour, minute: crate::Minute, second: crate::Second, microsecond: crate::Microsecond) -> Result<Self, SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewErrorNamed> {
        match sqlx::types::chrono::NaiveTime::from_hms_micro_opt(hour.to_std_primitive_u32(), minute.to_std_primitive_u32(), second.to_std_primitive_u32(), microsecond.to_std_primitive_u32()) {
            Some(value) => Ok(Self(value)),
            None => Err(SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewErrorNamed::InvalidHourOrMinuteOrSecondOrMicrosecond {
                hour,
                minute,
                second,
                microsecond,
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
        }
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::chrono::NaiveTime) -> Self {
        Self::try_new(value).unwrap()
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "hour" => _serde::__private::Ok(__Field::__field0),
                        "minute" => _serde::__private::Ok(__Field::__field1),
                        "second" => _serde::__private::Ok(__Field::__field2),
                        "microsecond" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"hour" => _serde::__private::Ok(__Field::__field0),
                        b"minute" => _serde::__private::Ok(__Field::__field1),
                        b"second" => _serde::__private::Ok(__Field::__field2),
                        b"microsecond" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
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
                marker: _serde::__private::PhantomData<SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::Hour>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<crate::Minute>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<crate::Second>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements"));
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<crate::Microsecond>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(3usize, &"struct SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin with 4 elements"));
                        }
                    };
                    match SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::try_new(__field0, __field1, __field2, __field3) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::Hour> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<crate::Minute> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<crate::Second> = _serde::__private::None;
                    let mut __field3: _serde::__private::Option<crate::Microsecond> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("hour"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::Hour>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("minute"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::Minute>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("second"));
                                }
                                __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::Second>(&mut __map)?);
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("microsecond"));
                                }
                                __field3 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::Microsecond>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("hour")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("minute")?,
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("second")?,
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => _serde::__private::de::missing_field("microsecond")?,
                    };
                    match SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::try_new(__field0, __field1, __field2, __field3) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["hour", "minute", "second", "microsecond"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::NaiveTime as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::chrono::NaiveTime as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::chrono::NaiveTime as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::chrono::NaiveTime as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} time not null")
    }
}
pub type SqlxTypesChronoNaiveTimeAsNotNullTimeTableTypeDeclaration = SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin;
pub type SqlxTypesChronoNaiveTimeAsNotNullTimeCreate = SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesChronoNaiveTimeAsNotNullTimeSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveTimeAsNotNullTimeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxTypesChronoNaiveTimeAsNotNullTimeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin>),
    GreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThan<SqlxTypesChronoNaiveTimeAsNotNullTimeTableTypeDeclaration>),
    Between(crate::where_element_filters::PostgresqlTypeWhereElementBetween<SqlxTypesChronoNaiveTimeAsNotNullTimeTableTypeDeclaration>),
    CurrentTime(crate::where_element_filters::PostgresqlTypeWhereElementCurrentTime),
    GreaterThanCurrentTime(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanCurrentTime),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxTypesChronoNaiveTimeAsNotNullTimeWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CurrentTime(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThanCurrentTime(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::CurrentTime(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThanCurrentTime(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveTimeAsNotNullTimeWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveTimeAsNotNullTimeWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Between(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CurrentTime(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThanCurrentTime(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesChronoNaiveTimeAsNotNullTimeRead(SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin);
impl SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    pub fn try_new(value: sqlx::types::chrono::NaiveTime) -> Result<Self, SqlxTypesChronoNaiveTimeAsNotNullTimeOriginTryNewErrorNamed> {
        match SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::try_new(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::chrono::NaiveTime) -> Self {
        Self(SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin::new_or_try_new_unwraped_for_test(value))
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoNaiveTimeAsNotNullTimeRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type SqlxTypesChronoNaiveTimeAsNotNullTimeReadInner = sqlx::types::chrono::NaiveTime;
pub type SqlxTypesChronoNaiveTimeAsNotNullTimeUpdate = SqlxTypesChronoNaiveTimeAsNotNullTimeOrigin;
impl crate::PostgresqlType for SqlxTypesChronoNaiveTimeAsNotNullTime {
    type TableTypeDeclaration = SqlxTypesChronoNaiveTimeAsNotNullTimeTableTypeDeclaration;
    type Create = SqlxTypesChronoNaiveTimeAsNotNullTimeCreate;
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
    type Select = SqlxTypesChronoNaiveTimeAsNotNullTimeSelect;
    fn select_query_part(_: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxTypesChronoNaiveTimeAsNotNullTimeWhereElement;
    type Read = SqlxTypesChronoNaiveTimeAsNotNullTimeRead;
    type ReadInner = SqlxTypesChronoNaiveTimeAsNotNullTimeReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.0
    }
    type Update = SqlxTypesChronoNaiveTimeAsNotNullTimeUpdate;
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
impl crate::tests::PostgresqlTypeTestCases for SqlxTypesChronoNaiveTimeAsNotNullTime {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner> {
        vec![
            sqlx::types::chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            sqlx::types::chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            sqlx::types::chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
            sqlx::types::chrono::NaiveTime::from_hms_nano_opt(23, 59, 59, 999_999_999).unwrap(),
            sqlx::types::chrono::NaiveTime::from_hms_opt(6, 30, 0).unwrap(),
            sqlx::types::chrono::NaiveTime::from_hms_opt(18, 45, 15).unwrap(),
            sqlx::types::chrono::NaiveTime::from_hms_milli_opt(10, 5, 3, 250).unwrap(),
            sqlx::types::chrono::NaiveTime::from_hms_micro_opt(14, 22, 33, 123_456).unwrap(),
        ]
    }
}
#[derive(Debug)]
pub struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql;
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin(sqlx::types::uuid::Uuid);
impl SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    pub fn new(value: sqlx::types::uuid::Uuid) -> Self {
        Self(value)
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::uuid::Uuid) -> Self {
        Self::new(value)
    }
}
impl crate::IsStringEmpty for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn is_string_empty(&self) -> std::primitive::bool {
        self.0.to_string().is_empty()
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(__serializer, "SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin", &self.0.to_string())
        }
    }
};
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: serde::__private::PhantomData<SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin>,
                lifetime: serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin;
                fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                    serde::__private::Formatter::write_str(__f, "struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: serde::Deserializer<'de>,
                {
                    let __field0 = <std::string::String as serde::Deserialize>::deserialize(__e)?;
                    serde::__private::Ok(SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin(match sqlx::types::uuid::Uuid::try_parse(&__field0) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(serde::de::Error::custom(error));
                        }
                    }))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin with 1 elements"));
                        }
                    };
                    serde::__private::Ok(SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin(match sqlx::types::uuid::Uuid::try_parse(&__field0) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(serde::de::Error::custom(error));
                        }
                    }))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin",
                __Visitor {
                    marker: serde::__private::PhantomData::<SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin>,
                    lifetime: serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::uuid::Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::uuid::Uuid as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::uuid::Uuid as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::uuid::Uuid as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} uuid not null {}", crate::maybe_primary_key(is_primary_key))
    }
}
impl std::convert::From<SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin {
    fn from(value: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead) -> Self {
        Self::new(<SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as crate::PostgresqlType>::into_inner(value))
    }
}
pub type SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlCreate(());
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin>),
    RegularExpression(crate::where_element_filters::PostgresqlTypeWhereElementRegularExpression),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::RegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::RegularExpression(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::RegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead(SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin);
impl SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    pub fn new(value: sqlx::types::uuid::Uuid) -> Self {
        Self(SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(value))
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::uuid::Uuid) -> Self {
        Self(SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new_or_try_new_unwraped_for_test(value))
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, _: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("({} = ${})", column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self);
        query
    }
}
impl crate::PostgresqlTypePrimaryKey for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql {
    type PrimaryKey = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead;
}
pub type SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlReadInner = sqlx::types::uuid::Uuid;
pub type SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin;
impl crate::PostgresqlType for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql {
    type TableTypeDeclaration = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration;
    type Create = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlCreate;
    fn create_query_part(_: &Self::Create, _: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        Ok(std::string::String::from("uuid_generate_v4()"))
    }
    fn create_query_bind(_: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query
    }
    type Select = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlSelect;
    fn select_query_part(_: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement;
    type Read = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead;
    type ReadInner = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.0
    }
    type Update = SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate;
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
impl crate::tests::PostgresqlTypeTestCases for SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner> {
        vec![]
    }
}
