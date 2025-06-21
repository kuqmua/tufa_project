generate_postgresql_types::generate_postgresql_types!([
    {
        "postgresql_type": "StdStringStringAsText",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": "Standart"
    },
    {
        "postgresql_type": "StdPrimitiveI16AsInt2",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": "Standart"
    },
    {
        "postgresql_type": "StdPrimitiveI32AsInt4",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": "Standart"
    },
    {
        "postgresql_type": "StdPrimitiveI16AsInt2",
        "not_null_or_nullable": "Nullable",
        "postgresql_type_pattern": "Standart"
    },
    {
        "postgresql_type": "StdPrimitiveI16AsInt2",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    },
    {
        "postgresql_type": "StdPrimitiveI16AsInt2",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "Nullable"
            }
        }
    },
    {
        "postgresql_type": "StdPrimitiveI16AsInt2",
        "not_null_or_nullable": "Nullable",
        "postgresql_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    },
    {
        "postgresql_type": "StdPrimitiveI16AsInt2",
        "not_null_or_nullable": "Nullable",
        "postgresql_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "Nullable"
            }
        }
    },
    {
        "postgresql_type": "StdPrimitiveI64AsBigSerialInitializedByPostgresql",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": "Standart"
    },
    {
        "postgresql_type": "StdStringStringAsText",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    },
    {
        "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    },
    {
        "postgresql_type": "SqlxTypesTimeDateAsDate",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    },
    {
        "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    },
    {
        "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    },
    {
        "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    }
    // ,
    // {
    //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
    //     "not_null_or_nullable": "NotNull",
    //     "postgresql_type_pattern": {
    //         "ArrayDimension1": {
    //             "dimension1_not_null_or_nullable": "NotNull"
    //         }
    //     }
    // }
]);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum EncodeFormat {
    Base64,
    Hex,
    Escape,
}
impl std::fmt::Display for EncodeFormat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Base64 => write!(formatter, "base64"),
            Self::Hex => write!(formatter, "hex"),
            Self::Escape => write!(formatter, "escape"),
        }
    }
}
impl std::default::Default for EncodeFormat {
    fn default() -> Self {
        Self::Base64
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for EncodeFormat {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, utoipa::ToSchema)]
pub struct SqlxTypesTimeUtcOffset(pub sqlx::types::time::UtcOffset);
impl serde::Serialize for SqlxTypesTimeUtcOffset {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxTypesTimeUtcOffset", usize::from(false) + 1 + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "hours", &self.0.whole_hours())?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "minutes", &self.0.minutes_past_hour())?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "seconds", &self.0.seconds_past_minute())?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxTypesTimeUtcOffset {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[expect(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__f, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "hours" => serde::__private::Ok(__Field::__field0),
                    "minutes" => serde::__private::Ok(__Field::__field1),
                    "seconds" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"hours" => serde::__private::Ok(__Field::__field0),
                    b"minutes" => serde::__private::Ok(__Field::__field1),
                    b"seconds" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SqlxTypesTimeUtcOffset>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SqlxTypesTimeUtcOffset;
            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__f, "struct SqlxTypesTimeUtcOffset")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::primitive::i8>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxTypesTimeUtcOffset with 3 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::primitive::i8>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxTypesTimeUtcOffset with 3 elements"));
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::primitive::i8>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(2usize, &"struct SqlxTypesTimeUtcOffset with 3 elements"));
                    }
                };
                serde::__private::Ok(SqlxTypesTimeUtcOffset(match sqlx::types::time::UtcOffset::from_hms(__field0, __field1, __field2) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::primitive::i8> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::primitive::i8> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::primitive::i8> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("hours"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i8>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("minutes"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i8>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("seconds"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i8>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("hours")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("minutes")?,
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::de::missing_field("seconds")?,
                };
                serde::__private::Ok(SqlxTypesTimeUtcOffset(match sqlx::types::time::UtcOffset::from_hms(__field0, __field1, __field2) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
        }
        #[doc(hidden)]
        const FIELDS: &[&str] = &["hours", "minutes", "seconds"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxTypesTimeUtcOffset",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<Self>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, utoipa::ToSchema)]
pub struct NumBigintBigInt(pub num_bigint::BigInt);
impl serde::Serialize for NumBigintBigInt {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let (sign, digits) = self.0.to_u32_digits();
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "NumBigintBigInt", usize::from(false) + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "sign", &NumBigintSign(sign))?;
        serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "digits", &digits)?;
        serde::ser::SerializeStruct::end(__serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for NumBigintBigInt {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[expect(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__f, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "sign" => serde::__private::Ok(__Field::__field0),
                    "digits" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"sign" => serde::__private::Ok(__Field::__field0),
                    b"digits" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<NumBigintBigInt>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = NumBigintBigInt;
            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__f, "struct NumBigintBigInt")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<NumBigintSign>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct NumBigintBigInt with 2 elements")),
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::vec::Vec<std::primitive::u32>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct NumBigintBigInt with 2 elements")),
                };
                serde::__private::Ok(NumBigintBigInt(num_bigint::BigInt::new(__field0.0, __field1)))
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<NumBigintSign> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::vec::Vec<std::primitive::u32>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("sign"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<NumBigintSign>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("digits"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<std::primitive::u32>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("sign")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("digits")?,
                };
                serde::__private::Ok(NumBigintBigInt(num_bigint::BigInt::new(__field0.0, __field1)))
            }
        }
        #[doc(hidden)]
        const FIELDS: &[&str] = &["sign", "digits"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "NumBigintBigInt",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<Self>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, utoipa::ToSchema)]
pub struct NumBigintSign(pub num_bigint::Sign);
impl serde::Serialize for NumBigintSign {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        serde::Serializer::serialize_newtype_struct(
            __serializer,
            "NumBigintSign",
            match self.0 {
                num_bigint::Sign::Minus => "Minus",
                num_bigint::Sign::NoSign => "NoSign",
                num_bigint::Sign::Plus => "Plus",
            },
        )
    }
}
impl<'de> serde::Deserialize<'de> for NumBigintSign {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<NumBigintSign>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = NumBigintSign;
            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__f, "tuple struct NumBigintSign")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::string::String = <std::string::String as serde::Deserialize>::deserialize(__e)?;
                serde::__private::Ok(NumBigintSign(match __field0.as_str() {
                    "Minus" => num_bigint::Sign::Minus,
                    "NoSign" => num_bigint::Sign::NoSign,
                    "Plus" => num_bigint::Sign::Plus,
                    _ => {
                        return Err(serde::de::Error::custom("unsupported value, supported: Minus, NoSign, Plus"));
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
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct NumBigintSign with 1 element"));
                    }
                };
                serde::__private::Ok(NumBigintSign(match __field0.as_str() {
                    "Minus" => num_bigint::Sign::Minus,
                    "NoSign" => num_bigint::Sign::NoSign,
                    "Plus" => num_bigint::Sign::Plus,
                    _ => {
                        return Err(serde::de::Error::custom("unsupported value, supported: Minus, NoSign, Plus"));
                    }
                }))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "NumBigintSign",
            __Visitor {
                marker: serde::__private::PhantomData::<Self>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
/////////////////////////////
/////
///////
////////

/////
/////
/////
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4Range;
#[derive(Debug, Clone, PartialEq)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin(sqlx::postgres::types::PgRange<StdPrimitiveI32AsNotNullInt4Origin>);//here
impl SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
    pub fn new(value: sqlx::postgres::types::PgRange<std::primitive::i32>) -> Self {
        Self(
            sqlx::postgres::types::PgRange {
                start: match value.start {
                    std::ops::Bound::Included(value) => std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(value)),
                    std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(value)),
                    std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                },
                end: match value.end {
                    std::ops::Bound::Included(value) => std::ops::Bound::Included(StdPrimitiveI32AsNotNullInt4Origin::new(value)),
                    std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(StdPrimitiveI32AsNotNullInt4Origin::new(value)),
                    std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                }
            }
        )
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
            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4Range", false as std::primitive::usize + 1 + 1)?;
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
                    serde::__private::Formatter::write_str(__f, "struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range")
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
            //here
            start: std::ops::Bound::Included(
                // ::core::default::Default::default()
                crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
            ),
            end: std::ops::Bound::Excluded(
                // ::core::default::Default::default()
                crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
            ),
        })
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
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
            Ok(value) => Ok(Self::new(value)),//here
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
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} int4range not null")
    }
}
pub type SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin;
pub type SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeCreate = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin;
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde ::
Deserialize,
)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    FindRangesWithinGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementFindRangesWithinGivenRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    FindRangesThatFullyContainTheGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementFindRangesThatFullyContainTheGivenRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    StrictlyToLeftOfRange(crate::where_element_filters::PostgresqlTypeWhereElementStrictlyToLeftOfRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    StrictlyToRightOfRange(crate::where_element_filters::PostgresqlTypeWhereElementStrictlyToRightOfRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    IncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementIncludedLowerBound<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    ExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementExcludedUpperBound<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    GreaterThanLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanLowerBound<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    OverlapWithRange(crate::where_element_filters::PostgresqlTypeWhereElementOverlapWithRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    AdjacentWithRange(crate::where_element_filters::PostgresqlTypeWhereElementAdjacentWithRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
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
            Self::GreaterThanLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
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
            Self::GreaterThanLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
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
            Self::GreaterThanLowerBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
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
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeReadInner = sqlx::postgres::types::PgRange<std::primitive::i32>;
pub type SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeUpdate = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin;
impl crate::PostgresqlType for SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4Range {
    type TableTypeDeclaration = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeTableTypeDeclaration;
    type Create = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeWhereElement;
    type Read = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeRead;
    type ReadInner = SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        //here
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
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
//////
//////
//////
#[derive(Debug)]
pub struct VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4Range;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin(std::vec::Vec<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>);
impl VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin {
    pub fn new(value: std::vec::Vec<sqlx::postgres::types::PgRange<std::primitive::i32>>) -> Self {
        Self(value.into_iter().map(|element| SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin::new(element)).collect())
    }
}
impl std::fmt::Display for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::vec::Vec<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::vec::Vec<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::vec::Vec<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::postgres::types::PgRange<std::primitive::i32> as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} int4range[] not null,check (array_position({column},null) is null)")
    }
}
pub type VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeTableTypeDeclaration = VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin;
pub type VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeCreate = VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin;
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde ::
Deserialize,
)]
pub struct VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeSelect {
    dimension1_pagination: crate::PaginationStartsWithOne,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin>),
    DimensionOneEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    DimensionOneLengthEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneFindRangesWithinGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneFindRangesWithinGivenRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    DimensionOneFindRangesThatFullyContainTheGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneFindRangesThatFullyContainTheGivenRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    DimensionOneStrictlyToLeftOfRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneStrictlyToLeftOfRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    DimensionOneStrictlyToRightOfRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneStrictlyToRightOfRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    DimensionOneIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneIncludedLowerBound<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    DimensionOneExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneExcludedUpperBound<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    DimensionOneGreaterThanLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThanLowerBound<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    DimensionOneOverlapWithRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneOverlapWithRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    DimensionOneAdjacentWithRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneAdjacentWithRange<SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4RangeOrigin>),
    DimensionOneRangeLength(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneRangeLength),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneFindRangesWithinGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneFindRangesThatFullyContainTheGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneStrictlyToLeftOfRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneStrictlyToRightOfRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThanLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneOverlapWithRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneAdjacentWithRange(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneRangeLength(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneFindRangesWithinGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneFindRangesThatFullyContainTheGivenRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneStrictlyToLeftOfRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneStrictlyToRightOfRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThanLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneOverlapWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAdjacentWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneRangeLength(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneFindRangesWithinGivenRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneFindRangesThatFullyContainTheGivenRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneStrictlyToLeftOfRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneStrictlyToRightOfRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneIncludedLowerBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneExcludedUpperBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThanLowerBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneOverlapWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAdjacentWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneRangeLength(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeRead(VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin);
impl VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeRead {
    pub fn new(value: std::vec::Vec<sqlx::postgres::types::PgRange<std::primitive::i32>>) -> Self {
        Self(VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeReadInner = std::vec::Vec<sqlx::postgres::types::PgRange<std::primitive::i32>>;
pub type VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeUpdate = VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeOrigin;
impl crate::PostgresqlType for VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4Range {
    type TableTypeDeclaration = VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeTableTypeDeclaration;
    type Create = VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
    type Select = VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeWhereElement;
    type Read = VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeRead;
    type ReadInner = VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        //here
        value.0 .0.into_iter().map(|element| 
            sqlx::postgres::types::PgRange {
                start: match element.0.start {
                    std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                    std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                    std::ops::Bound::Unbounded => std::ops::Bound::Unbounded
                },
                end: match element.0.end {
                    std::ops::Bound::Included(value) => std::ops::Bound::Included(value.0),
                    std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(value.0),
                    std::ops::Bound::Unbounded => std::ops::Bound::Unbounded
                },
            }
        ).collect()
    }
    type Update = VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4RangeUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
