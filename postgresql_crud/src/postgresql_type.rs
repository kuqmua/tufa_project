// generate_postgresql_types::generate_postgresql_types!([
//     {
//         "postgresql_type": "StdPrimitiveI64AsInt8",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": "Standart"
//     },
//     {
//         "postgresql_type": "StdStringStringAsText",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": "Standart"
//     },
//     {
//         "postgresql_type": "StdPrimitiveI16AsInt2",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": "Standart"
//     },
//     {
//         "postgresql_type": "StdPrimitiveI32AsInt4",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": "Standart"
//     },
//     {
//         "postgresql_type": "StdPrimitiveI16AsInt2",
//         "not_null_or_nullable": "Nullable",
//         "postgresql_type_pattern": "Standart"
//     },
//     {
//         "postgresql_type": "StdPrimitiveI16AsInt2",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "NotNull"
//             }
//         }
//     },
//     {
//         "postgresql_type": "StdPrimitiveI16AsInt2",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "Nullable"
//             }
//         }
//     },
//     {
//         "postgresql_type": "StdPrimitiveI16AsInt2",
//         "not_null_or_nullable": "Nullable",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "NotNull"
//             }
//         }
//     },
//     {
//         "postgresql_type": "StdPrimitiveI16AsInt2",
//         "not_null_or_nullable": "Nullable",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "Nullable"
//             }
//         }
//     },
//     {
//         "postgresql_type": "StdPrimitiveI64AsBigSerialInitializedByPostgresql",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": "Standart"
//     },
//     {
//         "postgresql_type": "StdStringStringAsText",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "NotNull"
//             }
//         }
//     },
//     {
//         "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "NotNull"
//             }
//         }
//     },
//     {
//         "postgresql_type": "SqlxTypesTimeDateAsDate",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "NotNull"
//             }
//         }
//     },
//     {
//         "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "NotNull"
//             }
//         }
//     },
//     {
//         "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "NotNull"
//             }
//         }
//     },
//     {
//         "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "NotNull"
//             }
//         }
//     }
//     ,
//     {
//         "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "NotNull"
//             }
//         }
//     },
//     {
//         "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
//         "not_null_or_nullable": "NotNull",
//         "postgresql_type_pattern": {
//             "ArrayDimension1": {
//                 "dimension1_not_null_or_nullable": "NotNull"
//             }
//         }
//     }
// ]);

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
///////////
#[derive(Debug)]
pub struct StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin(std::primitive::i64);
impl StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin {
    pub fn new(value: std::primitive::i64) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::primitive::i64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::primitive::i64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <std::primitive::i64 as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} bigserial not null {}", crate::maybe_primary_key(is_primary_key))
    }
}
pub type StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlTableTypeDeclaration = StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlCreate(());
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
    GreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThan<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
    Between(crate::where_element_filters::PostgresqlTypeWhereElementBetween<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
    In(crate::where_element_filters::PostgresqlTypeWhereElementIn<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::In(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::In(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Between(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::In(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead(StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin);
impl StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead {
    pub fn new(value: std::primitive::i64) -> Self {
        Self(StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
impl crate::PostgresqlTypePrimaryKey for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql {
    type PrimaryKey = StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead;
}
pub type StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlReadInner = std::primitive::i64;
pub type StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlUpdate = StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin;
impl crate::PostgresqlType for StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql {
    type TableTypeDeclaration = StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlTableTypeDeclaration;
    type Create = StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        Ok(std::string::String::from("DEFAULT"))
    }
    fn create_query_bind(value: Self::Create, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query
    }
    type Select = StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlWhereElement;
    type Read = StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead;
    type ReadInner = StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0 .0
    }
    type Update = StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlUpdate;
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
#[derive(Debug)]
pub struct VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresql;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin(std::vec::Vec<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>);
impl VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    pub fn new(value: std::vec::Vec<std::primitive::i64>) -> Self {
        Self(value.into_iter().map(|element| StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin::new(element)).collect())
    }
}
impl std::fmt::Display for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::vec::Vec<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::vec::Vec<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::vec::Vec<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <std::primitive::i64 as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} bigserial[] not null,check (array_position({column},null) is null) {}", crate::maybe_primary_key(is_primary_key))
    }
}
pub type VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlTableTypeDeclaration = VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlCreate(());
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlSelect {
    dimension1_pagination: crate::PaginationStartsWithOne,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin>),
    DimensionOneEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
    DimensionOneLengthEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneGreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThan<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
    DimensionOneBetween(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneBetween<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
    DimensionOneIn(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneIn<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneIn(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneIn(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneBetween(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneIn(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlRead(VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin);
impl VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    pub fn new(value: std::vec::Vec<std::primitive::i64>) -> Self {
        Self(VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlReadInner = std::vec::Vec<std::primitive::i64>;
pub type VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlUpdate = VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin;
impl crate::PostgresqlType for VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresql {
    type TableTypeDeclaration = VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlTableTypeDeclaration;
    type Create = VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        Ok(std::string::String::from("DEFAULT"))
    }
    fn create_query_bind(value: Self::Create, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query
    }
    type Select = VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlWhereElement;
    type Read = VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlRead;
    type ReadInner = VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0 .0.into_iter().map(|element| element.0).collect()
    }
    type Update = VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlUpdate;
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
#[derive(Debug)]
pub struct OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresql;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin(std::option::Option<VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin>);
impl OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    pub fn new(value: std::option::Option<std::vec::Vec<std::primitive::i64>>) -> Self {
        Self(match value {
            Some(value) => Some(VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin::new(value)),
            None => None,
        })
    }
}
impl std::fmt::Display for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::option::Option<VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::option::Option<VecOfStdPrimitiveI64AsNotNullArrayOfNotNullBigSerialInitializedByPostgresqlOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <std::primitive::i64 as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} bigserial[],check (array_position({column},null) is null) {}", crate::maybe_primary_key(is_primary_key))
    }
}
pub type OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlTableTypeDeclaration = OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlCreate(());
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlSelect {
    dimension1_pagination: crate::PaginationStartsWithOne,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin>),
    DimensionOneEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
    DimensionOneLengthEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneGreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThan<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
    DimensionOneBetween(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneBetween<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
    DimensionOneIn(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneIn<StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneIn(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneIn(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneBetween(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneIn(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlRead(OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin);
impl OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    pub fn new(value: std::option::Option<std::vec::Vec<std::primitive::i64>>) -> Self {
        Self(OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlReadInner = std::option::Option<std::vec::Vec<std::primitive::i64>>;
pub type OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlUpdate = OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlOrigin;
impl crate::PostgresqlType for OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresql {
    type TableTypeDeclaration = OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlTableTypeDeclaration;
    type Create = OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        Ok(std::string::String::from("DEFAULT"))
    }
    fn create_query_bind(value: Self::Create, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query
    }
    type Select = OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlWhereElement;
    type Read = OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlRead;
    type ReadInner = OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        match value.0 .0 {
            Some(value) => Some(value.0.into_iter().map(|element| element.0).collect()),
            None => None,
        }
    }
    type Update = OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullBigSerialInitializedByPostgresqlUpdate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
}
#[derive(Debug)]
pub struct SqlxTypesBigDecimalAsNotNullNumeric;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SqlxTypesBigDecimalAsNotNullNumericOrigin(sqlx::types::BigDecimal);
impl SqlxTypesBigDecimalAsNotNullNumericOrigin {
    pub fn new(value: sqlx::types::BigDecimal) -> Self {
        Self(value)
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxTypesBigDecimalAsNotNullNumericOrigin {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let (bigint, exponent) = self.0.clone().into_bigint_and_exponent();
            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "SqlxTypesBigDecimalAsNotNullNumeric", false as std::primitive::usize + 1 + 1)?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "digits", &crate::postgresql_type::NumBigintBigInt(bigint))?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "scale", &exponent)?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxTypesBigDecimalAsNotNullNumericOrigin {
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
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "digits" => _serde::__private::Ok(__Field::__field0),
                        "scale" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
                where
                    __E: serde::de::Error,
                {
                    match __value {
                        b"digits" => serde::__private::Ok(__Field::__field0),
                        b"scale" => serde::__private::Ok(__Field::__field1),
                        _ => serde::__private::Ok(__Field::__ignore),
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
                marker: serde::__private::PhantomData<SqlxTypesBigDecimalAsNotNullNumericOrigin>,
                lifetime: serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesBigDecimalAsNotNullNumericOrigin;
                fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                    serde::__private::Formatter::write_str(__f, "struct SqlxTypesBigDecimalAsNumeric")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<crate::postgresql_type::NumBigintBigInt>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxTypesBigDecimalAsNumeric with 2 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxTypesBigDecimalAsNumeric with 2 elements"));
                        }
                    };
                    serde::__private::Ok(SqlxTypesBigDecimalAsNotNullNumericOrigin(sqlx::types::BigDecimal::new(__field0.0, __field1)))
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: serde::de::MapAccess<'de>,
                {
                    let mut __field0: serde::__private::Option<crate::postgresql_type::NumBigintBigInt> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::primitive::i64> = serde::__private::None;
                    while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("\"digits\""));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<crate::postgresql_type::NumBigintBigInt>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("\"scale\""));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
                            }
                            _ => {
                                let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("\"digits\"")?,
                    };
                    let __field1 = match __field1 {
                        serde::__private::Some(__field1) => __field1,
                        serde::__private::None => serde::__private::de::missing_field("\"scale\"")?,
                    };
                    serde::__private::Ok(SqlxTypesBigDecimalAsNotNullNumericOrigin(sqlx::types::BigDecimal::new(__field0.0, __field1)))
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["digits", "scale"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxTypesBigDecimalAsNotNullNumeric",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SqlxTypesBigDecimalAsNotNullNumericOrigin>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxTypesBigDecimalAsNotNullNumericOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesBigDecimalAsNotNullNumericOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesBigDecimalAsNotNullNumericOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesBigDecimalAsNotNullNumericOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::BigDecimal as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::BigDecimal as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesBigDecimalAsNotNullNumericOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesBigDecimalAsNotNullNumericOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::BigDecimal as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for SqlxTypesBigDecimalAsNotNullNumericOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::BigDecimal as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl SqlxTypesBigDecimalAsNotNullNumericOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numeric not null")
    }
}
pub type SqlxTypesBigDecimalAsNotNullNumericTableTypeDeclaration = SqlxTypesBigDecimalAsNotNullNumericOrigin;
pub type SqlxTypesBigDecimalAsNotNullNumericCreate = SqlxTypesBigDecimalAsNotNullNumericOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesBigDecimalAsNotNullNumericSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesBigDecimalAsNotNullNumericSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxTypesBigDecimalAsNotNullNumericWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
    GreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThan<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
    Between(crate::where_element_filters::PostgresqlTypeWhereElementBetween<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxTypesBigDecimalAsNotNullNumericWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesBigDecimalAsNotNullNumericWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesBigDecimalAsNotNullNumericWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Between(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesBigDecimalAsNotNullNumericRead(SqlxTypesBigDecimalAsNotNullNumericOrigin);
impl SqlxTypesBigDecimalAsNotNullNumericRead {
    pub fn new(value: sqlx::types::BigDecimal) -> Self {
        Self(SqlxTypesBigDecimalAsNotNullNumericOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesBigDecimalAsNotNullNumericRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesBigDecimalAsNotNullNumericRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesBigDecimalAsNotNullNumericRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesBigDecimalAsNotNullNumericRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <SqlxTypesBigDecimalAsNotNullNumericOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesBigDecimalAsNotNullNumericRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <SqlxTypesBigDecimalAsNotNullNumericOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <SqlxTypesBigDecimalAsNotNullNumericOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type SqlxTypesBigDecimalAsNotNullNumericReadInner = sqlx::types::BigDecimal;
pub type SqlxTypesBigDecimalAsNotNullNumericUpdate = SqlxTypesBigDecimalAsNotNullNumericOrigin;
impl crate::PostgresqlType for SqlxTypesBigDecimalAsNotNullNumeric {
    type TableTypeDeclaration = SqlxTypesBigDecimalAsNotNullNumericTableTypeDeclaration;
    type Create = SqlxTypesBigDecimalAsNotNullNumericCreate;
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
    type Select = SqlxTypesBigDecimalAsNotNullNumericSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxTypesBigDecimalAsNotNullNumericWhereElement;
    type Read = SqlxTypesBigDecimalAsNotNullNumericRead;
    type ReadInner = SqlxTypesBigDecimalAsNotNullNumericReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0 .0
    }
    type Update = SqlxTypesBigDecimalAsNotNullNumericUpdate;
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
#[derive(Debug)]
pub struct OptionSqlxTypesBigDecimalAsNullableNumeric;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionSqlxTypesBigDecimalAsNullableNumericOrigin(std::option::Option<SqlxTypesBigDecimalAsNotNullNumericOrigin>);
impl OptionSqlxTypesBigDecimalAsNullableNumericOrigin {
    pub fn new(value: std::option::Option<sqlx::types::BigDecimal>) -> Self {
        Self(match value {
            Some(value) => Some(SqlxTypesBigDecimalAsNotNullNumericOrigin::new(value)),
            None => None,
        })
    }
}
impl std::fmt::Display for OptionSqlxTypesBigDecimalAsNullableNumericOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionSqlxTypesBigDecimalAsNullableNumericOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionSqlxTypesBigDecimalAsNullableNumericOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionSqlxTypesBigDecimalAsNullableNumericOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<SqlxTypesBigDecimalAsNotNullNumericOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::option::Option<SqlxTypesBigDecimalAsNotNullNumericOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionSqlxTypesBigDecimalAsNullableNumericOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionSqlxTypesBigDecimalAsNullableNumericOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::option::Option<SqlxTypesBigDecimalAsNotNullNumericOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for OptionSqlxTypesBigDecimalAsNullableNumericOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::BigDecimal as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl OptionSqlxTypesBigDecimalAsNullableNumericOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numeric")
    }
}
pub type OptionSqlxTypesBigDecimalAsNullableNumericTableTypeDeclaration = OptionSqlxTypesBigDecimalAsNullableNumericOrigin;
pub type OptionSqlxTypesBigDecimalAsNullableNumericCreate = OptionSqlxTypesBigDecimalAsNullableNumericOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionSqlxTypesBigDecimalAsNullableNumericSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionSqlxTypesBigDecimalAsNullableNumericSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum OptionSqlxTypesBigDecimalAsNullableNumericWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<OptionSqlxTypesBigDecimalAsNullableNumericOrigin>),
    GreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThan<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
    Between(crate::where_element_filters::PostgresqlTypeWhereElementBetween<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for OptionSqlxTypesBigDecimalAsNullableNumericWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for OptionSqlxTypesBigDecimalAsNullableNumericWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionSqlxTypesBigDecimalAsNullableNumericWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::GreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Between(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionSqlxTypesBigDecimalAsNullableNumericRead(OptionSqlxTypesBigDecimalAsNullableNumericOrigin);
impl OptionSqlxTypesBigDecimalAsNullableNumericRead {
    pub fn new(value: std::option::Option<sqlx::types::BigDecimal>) -> Self {
        Self(OptionSqlxTypesBigDecimalAsNullableNumericOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for OptionSqlxTypesBigDecimalAsNullableNumericRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionSqlxTypesBigDecimalAsNullableNumericRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionSqlxTypesBigDecimalAsNullableNumericRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionSqlxTypesBigDecimalAsNullableNumericRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <OptionSqlxTypesBigDecimalAsNullableNumericOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionSqlxTypesBigDecimalAsNullableNumericRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionSqlxTypesBigDecimalAsNullableNumericOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <OptionSqlxTypesBigDecimalAsNullableNumericOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type OptionSqlxTypesBigDecimalAsNullableNumericReadInner = std::option::Option<sqlx::types::BigDecimal>;
pub type OptionSqlxTypesBigDecimalAsNullableNumericUpdate = OptionSqlxTypesBigDecimalAsNullableNumericOrigin;
impl crate::PostgresqlType for OptionSqlxTypesBigDecimalAsNullableNumeric {
    type TableTypeDeclaration = OptionSqlxTypesBigDecimalAsNullableNumericTableTypeDeclaration;
    type Create = OptionSqlxTypesBigDecimalAsNullableNumericCreate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
    type Select = OptionSqlxTypesBigDecimalAsNullableNumericSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = OptionSqlxTypesBigDecimalAsNullableNumericWhereElement;
    type Read = OptionSqlxTypesBigDecimalAsNullableNumericRead;
    type ReadInner = OptionSqlxTypesBigDecimalAsNullableNumericReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        match value.0 .0 {
            Some(value) => Some(value.0),
            None => None,
        }
    }
    type Update = OptionSqlxTypesBigDecimalAsNullableNumericUpdate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
}
#[derive(Debug)]
pub struct VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumeric;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin(std::vec::Vec<SqlxTypesBigDecimalAsNotNullNumericOrigin>);
impl VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin {
    pub fn new(value: std::vec::Vec<sqlx::types::BigDecimal>) -> Self {
        Self(value.into_iter().map(|element| SqlxTypesBigDecimalAsNotNullNumericOrigin::new(element)).collect())
    }
}
impl std::fmt::Display for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::vec::Vec<SqlxTypesBigDecimalAsNotNullNumericOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::vec::Vec<SqlxTypesBigDecimalAsNotNullNumericOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::vec::Vec<SqlxTypesBigDecimalAsNotNullNumericOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::BigDecimal as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numeric[] not null,check (array_position({column},null) is null)")
    }
}
pub type VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericTableTypeDeclaration = VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin;
pub type VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericCreate = VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericSelect {
    dimension1_pagination: crate::PaginationStartsWithOne,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin>),
    DimensionOneEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
    DimensionOneLengthEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneGreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThan<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
    DimensionOneBetween(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneBetween<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneBetween(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericRead(VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin);
impl VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericRead {
    pub fn new(value: std::vec::Vec<sqlx::types::BigDecimal>) -> Self {
        Self(VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericReadInner = std::vec::Vec<sqlx::types::BigDecimal>;
pub type VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericUpdate = VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin;
impl crate::PostgresqlType for VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumeric {
    type TableTypeDeclaration = VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericTableTypeDeclaration;
    type Create = VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericCreate;
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
    type Select = VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericWhereElement;
    type Read = VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericRead;
    type ReadInner = VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0 .0.into_iter().map(|element| element.0).collect()
    }
    type Update = VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericUpdate;
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
#[derive(Debug)]
pub struct OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumeric;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin(std::option::Option<VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin>);
impl OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin {
    pub fn new(value: std::option::Option<std::vec::Vec<sqlx::types::BigDecimal>>) -> Self {
        Self(match value {
            Some(value) => Some(VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin::new(value)),
            None => None,
        })
    }
}
impl std::fmt::Display for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::option::Option<VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::option::Option<VecOfSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumericOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::BigDecimal as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numeric[],check (array_position({column},null) is null)")
    }
}
pub type OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericTableTypeDeclaration = OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin;
pub type OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericCreate = OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericSelect {
    dimension1_pagination: crate::PaginationStartsWithOne,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin>),
    DimensionOneEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<OptionSqlxTypesBigDecimalAsNullableNumericOrigin>),
    DimensionOneLengthEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneGreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThan<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
    DimensionOneBetween(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneBetween<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneBetween(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericRead(OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin);
impl OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericRead {
    pub fn new(value: std::option::Option<std::vec::Vec<sqlx::types::BigDecimal>>) -> Self {
        Self(OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericReadInner = std::option::Option<std::vec::Vec<sqlx::types::BigDecimal>>;
pub type OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericUpdate = OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericOrigin;
impl crate::PostgresqlType for OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumeric {
    type TableTypeDeclaration = OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericTableTypeDeclaration;
    type Create = OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericCreate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
    type Select = OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericWhereElement;
    type Read = OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericRead;
    type ReadInner = OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        match value.0 .0 {
            Some(value) => Some(value.0.into_iter().map(|element| element.0).collect()),
            None => None,
        }
    }
    type Update = OptionVecOfSqlxTypesBigDecimalAsNullableArrayOfNotNullNumericUpdate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
}
#[derive(Debug)]
pub struct VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumeric;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin(std::vec::Vec<OptionSqlxTypesBigDecimalAsNullableNumericOrigin>);
impl VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin {
    pub fn new(value: std::vec::Vec<std::option::Option<sqlx::types::BigDecimal>>) -> Self {
        Self(value.into_iter().map(|element| OptionSqlxTypesBigDecimalAsNullableNumericOrigin::new(element)).collect())
    }
}
impl std::fmt::Display for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::vec::Vec<OptionSqlxTypesBigDecimalAsNullableNumericOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::vec::Vec<OptionSqlxTypesBigDecimalAsNullableNumericOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::vec::Vec<OptionSqlxTypesBigDecimalAsNullableNumericOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::BigDecimal as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numeric[] not null")
    }
}
pub type VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericTableTypeDeclaration = VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin;
pub type VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericCreate = VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericSelect {
    dimension1_pagination: crate::PaginationStartsWithOne,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin>),
    DimensionOneEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
    DimensionOneLengthEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneGreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThan<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
    DimensionOneBetween(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneBetween<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneBetween(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericRead(VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin);
impl VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericRead {
    pub fn new(value: std::vec::Vec<std::option::Option<sqlx::types::BigDecimal>>) -> Self {
        Self(VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericReadInner = std::vec::Vec<std::option::Option<sqlx::types::BigDecimal>>;
pub type VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericUpdate = VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin;
impl crate::PostgresqlType for VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumeric {
    type TableTypeDeclaration = VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericTableTypeDeclaration;
    type Create = VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericCreate;
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
    type Select = VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericWhereElement;
    type Read = VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericRead;
    type ReadInner = VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value
            .0
             .0
            .into_iter()
            .map(|element| match element.0 {
                Some(value) => Some(value.0),
                None => None,
            })
            .collect()
    }
    type Update = VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericUpdate;
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
#[derive(Debug)]
pub struct OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumeric;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin(std::option::Option<VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin>);
impl OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin {
    pub fn new(value: std::option::Option<std::vec::Vec<std::option::Option<sqlx::types::BigDecimal>>>) -> Self {
        Self(match value {
            Some(value) => Some(VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin::new(value)),
            None => None,
        })
    }
}
impl std::fmt::Display for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::option::Option<VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::option::Option<VecOfOptionSqlxTypesBigDecimalAsNotNullArrayOfNullableNumericOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::BigDecimal as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numeric[]")
    }
}
pub type OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericTableTypeDeclaration = OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin;
pub type OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericCreate = OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericSelect {
    dimension1_pagination: crate::PaginationStartsWithOne,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin>),
    DimensionOneEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<OptionSqlxTypesBigDecimalAsNullableNumericOrigin>),
    DimensionOneLengthEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneGreaterThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThan<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
    DimensionOneBetween(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneBetween<SqlxTypesBigDecimalAsNotNullNumericOrigin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthEqual(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneLengthMoreThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneBetween(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneLengthMoreThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneBetween(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericRead(OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin);
impl OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericRead {
    pub fn new(value: std::option::Option<std::vec::Vec<std::option::Option<sqlx::types::BigDecimal>>>) -> Self {
        Self(OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericReadInner = std::option::Option<std::vec::Vec<std::option::Option<sqlx::types::BigDecimal>>>;
pub type OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericUpdate = OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericOrigin;
impl crate::PostgresqlType for OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumeric {
    type TableTypeDeclaration = OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericTableTypeDeclaration;
    type Create = OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericCreate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
    type Select = OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericWhereElement;
    type Read = OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericRead;
    type ReadInner = OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        match value.0 .0 {
            Some(value) => Some(
                value
                    .0
                    .into_iter()
                    .map(|element| match element.0 {
                        Some(value) => Some(value.0),
                        None => None,
                    })
                    .collect(),
            ),
            None => None,
        }
    }
    type Update = OptionVecOfOptionSqlxTypesBigDecimalAsNullableArrayOfNullableNumericUpdate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
}
#[derive(Debug)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRange;
#[derive(Debug, Clone, PartialEq)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin(sqlx::postgres::types::PgRange<SqlxTypesBigDecimalAsNotNullNumericOrigin>);
impl SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin {
    pub fn new(value: sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>) -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: match value.start {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesBigDecimalAsNotNullNumericOrigin::new(value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesBigDecimalAsNotNullNumericOrigin::new(value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
            end: match value.end {
                std::ops::Bound::Included(value) => std::ops::Bound::Included(SqlxTypesBigDecimalAsNotNullNumericOrigin::new(value)),
                std::ops::Bound::Excluded(value) => std::ops::Bound::Excluded(SqlxTypesBigDecimalAsNotNullNumericOrigin::new(value)),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            },
        })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRange", false as std::primitive::usize + 1 + 1)?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "start",
                &match self.0.start.clone() {
                    std::collections::Bound::Included(value) => std::collections::Bound::Included(
                        // SqlxTypesBigDecimalAsNotNullNumericOrigin(value)
                        value//here
                    ),
                    std::collections::Bound::Excluded(value) => std::collections::Bound::Excluded(
                        // SqlxTypesBigDecimalAsNotNullNumericOrigin(value)
                        value
                    ),
                    std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                },
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "end",
                &match self.0.end.clone() {
                    std::collections::Bound::Included(value) => std::collections::Bound::Included(
                        // SqlxTypesBigDecimalAsNotNullNumericOrigin(value)
                        value
                    ),
                    std::collections::Bound::Excluded(value) => std::collections::Bound::Excluded(
                        // SqlxTypesBigDecimalAsNotNullNumericOrigin(value)
                        value
                    ),
                    std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                },
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin {
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
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "start" => _serde::__private::Ok(__Field::__field0),
                        "end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
                where
                    __E: serde::de::Error,
                {
                    match __value {
                        b"start" => serde::__private::Ok(__Field::__field0),
                        b"end" => serde::__private::Ok(__Field::__field1),
                        _ => serde::__private::Ok(__Field::__ignore),
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
                marker: serde::__private::PhantomData<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>,
                lifetime: serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin;
                fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                    serde::__private::Formatter::write_str(__f, "struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::collections::Bound<SqlxTypesBigDecimalAsNotNullNumericOrigin>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange with 2 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::collections::Bound<SqlxTypesBigDecimalAsNotNullNumericOrigin>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNumRange with 2 elements"));
                        }
                    };
                    serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin(sqlx::postgres::types::PgRange {
                        start: match __field0 {
                            std::collections::Bound::Included(value) => std::collections::Bound::Included(value),//here was .0
                            std::collections::Bound::Excluded(value) => std::collections::Bound::Excluded(value),
                            std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                        },
                        end: match __field1 {
                            std::collections::Bound::Included(value) => std::collections::Bound::Included(value),
                            std::collections::Bound::Excluded(value) => std::collections::Bound::Excluded(value),
                            std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                        },
                    }))
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: serde::de::MapAccess<'de>,
                {
                    let mut __field0: serde::__private::Option<std::collections::Bound<SqlxTypesBigDecimalAsNotNullNumericOrigin>> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::collections::Bound<SqlxTypesBigDecimalAsNotNullNumericOrigin>> = serde::__private::None;
                    while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("\"start\""));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::collections::Bound<SqlxTypesBigDecimalAsNotNullNumericOrigin>>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("\"end\""));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::collections::Bound<SqlxTypesBigDecimalAsNotNullNumericOrigin>>(&mut __map)?);
                            }
                            _ => {
                                let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("\"start\"")?,
                    };
                    let __field1 = match __field1 {
                        serde::__private::Some(__field1) => __field1,
                        serde::__private::None => serde::__private::de::missing_field("\"end\"")?,
                    };
                    serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin(sqlx::postgres::types::PgRange {
                        start: match __field0 {
                            std::collections::Bound::Included(value) => std::collections::Bound::Included(value),//here was .0
                            std::collections::Bound::Excluded(value) => std::collections::Bound::Excluded(value),
                            std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                        },
                        end: match __field1 {
                            std::collections::Bound::Included(value) => std::collections::Bound::Included(value),
                            std::collections::Bound::Excluded(value) => std::collections::Bound::Excluded(value),
                            std::collections::Bound::Unbounded => std::collections::Bound::Unbounded,
                        },
                    }))
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["start", "end"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRange",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgRange {
            start: std::ops::Bound::Included(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            end: std::ops::Bound::Excluded(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        })
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self::new(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numrange not null")
    }
}
pub type SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeTableTypeDeclaration = SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin;
pub type SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeCreate = SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    FindRangesWithinGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementFindRangesWithinGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    FindRangesThatFullyContainTheGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementFindRangesThatFullyContainTheGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    StrictlyToLeftOfRange(crate::where_element_filters::PostgresqlTypeWhereElementStrictlyToLeftOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    StrictlyToRightOfRange(crate::where_element_filters::PostgresqlTypeWhereElementStrictlyToRightOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    IncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementIncludedLowerBound<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    ExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementExcludedUpperBound<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    GreaterThanIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanIncludedLowerBound<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    GreaterThanExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanExcludedUpperBound<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    OverlapWithRange(crate::where_element_filters::PostgresqlTypeWhereElementOverlapWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    AdjacentWithRange(crate::where_element_filters::PostgresqlTypeWhereElementAdjacentWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    RangeLength(crate::where_element_filters::PostgresqlTypeWhereElementRangeLength),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeWhereElement {
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
impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeWhereElement {
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
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead(SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin);
impl SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead {
    pub fn new(value: sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>) -> Self {
        Self(SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeReadInner = sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>;
pub type SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeUpdate = SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin;
impl crate::PostgresqlType for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRange {
    type TableTypeDeclaration = SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeTableTypeDeclaration;
    type Create = SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeCreate;
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
    type Select = SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeWhereElement;
    type Read = SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead;
    type ReadInner = SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        <SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRange as crate::PostgresqlType>::into_inner(SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead(value.0))
    }
    type Update = SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeUpdate;
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
#[derive(Debug)]
pub struct OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRange;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin(std::option::Option<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>);
impl OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin {
    pub fn new(value: std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>) -> Self {
        Self(match value {
            Some(value) => Some(SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin::new(value)),
            None => None,
        })
    }
}
impl std::fmt::Display for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::option::Option<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::option::Option<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numrange")
    }
}
pub type OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeTableTypeDeclaration = OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin;
pub type OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeCreate = OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin>),
    FindRangesWithinGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementFindRangesWithinGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    FindRangesThatFullyContainTheGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementFindRangesThatFullyContainTheGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    StrictlyToLeftOfRange(crate::where_element_filters::PostgresqlTypeWhereElementStrictlyToLeftOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    StrictlyToRightOfRange(crate::where_element_filters::PostgresqlTypeWhereElementStrictlyToRightOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    IncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementIncludedLowerBound<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    ExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementExcludedUpperBound<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    GreaterThanIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanIncludedLowerBound<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    GreaterThanExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementGreaterThanExcludedUpperBound<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    OverlapWithRange(crate::where_element_filters::PostgresqlTypeWhereElementOverlapWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    AdjacentWithRange(crate::where_element_filters::PostgresqlTypeWhereElementAdjacentWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    RangeLength(crate::where_element_filters::PostgresqlTypeWhereElementRangeLength),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeWhereElement {
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
impl error_occurence_lib::ToStdStringString for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeWhereElement {
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
pub struct OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeRead(OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin);
impl OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeRead {
    pub fn new(value: std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>) -> Self {
        Self(OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeReadInner = std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>;
pub type OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeUpdate = OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin;
impl crate::PostgresqlType for OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRange {
    type TableTypeDeclaration = OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeTableTypeDeclaration;
    type Create = OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeCreate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
    type Select = OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeWhereElement;
    type Read = OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeRead;
    type ReadInner = OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        match value.0 .0 {
            Some(value) => Some(<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRange as crate::PostgresqlType>::into_inner(SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead(value))),
            None => None,
        }
    }
    type Update = OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeUpdate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
}
#[derive(Debug)]
pub struct VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRange;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin(std::vec::Vec<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>);
impl VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin {
    pub fn new(value: std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>) -> Self {
        Self(value.into_iter().map(|element| SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin::new(element)).collect())
    }
}
impl std::fmt::Display for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::vec::Vec<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::vec::Vec<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::vec::Vec<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numrange[] not null,check (array_position({column},null) is null)")
    }
}
pub type VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeTableTypeDeclaration = VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin;
pub type VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeCreate = VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeSelect {
    dimension1_pagination: crate::PaginationStartsWithOne,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin>),
    DimensionOneEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneLengthEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneFindRangesWithinGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneFindRangesWithinGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneFindRangesThatFullyContainTheGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneFindRangesThatFullyContainTheGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneStrictlyToLeftOfRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneStrictlyToLeftOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneStrictlyToRightOfRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneStrictlyToRightOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneIncludedLowerBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneExcludedUpperBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneGreaterThanIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThanIncludedLowerBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneGreaterThanExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThanExcludedUpperBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneOverlapWithRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneOverlapWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneAdjacentWithRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneAdjacentWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneRangeLength(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneRangeLength),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeWhereElement {
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
            Self::DimensionOneGreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
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
            Self::DimensionOneGreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneOverlapWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAdjacentWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneRangeLength(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeWhereElement {
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
            Self::DimensionOneGreaterThanIncludedLowerBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThanExcludedUpperBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneOverlapWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAdjacentWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneRangeLength(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeRead(VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin);
impl VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeRead {
    pub fn new(value: std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>) -> Self {
        Self(VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeReadInner = std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>;
pub type VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeUpdate = VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin;
impl crate::PostgresqlType for VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRange {
    type TableTypeDeclaration = VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeTableTypeDeclaration;
    type Create = VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeCreate;
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
    type Select = VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeWhereElement;
    type Read = VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeRead;
    type ReadInner = VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value
            .0
             .0
            .into_iter()
            .map(|element| <SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRange as crate::PostgresqlType>::into_inner(SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead(element)))
            .collect()
    }
    type Update = VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeUpdate;
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
#[derive(Debug)]
pub struct OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRange;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin(std::option::Option<VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin>);
impl OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin {
    pub fn new(value: std::option::Option<std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>>) -> Self {
        Self(match value {
            Some(value) => Some(VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin::new(value)),
            None => None,
        })
    }
}
impl std::fmt::Display for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::option::Option<VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::option::Option<VecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNotNullNumRangeOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numrange[],check (array_position({column},null) is null)")
    }
}
pub type OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeTableTypeDeclaration = OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin;
pub type OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeCreate = OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeSelect {
    dimension1_pagination: crate::PaginationStartsWithOne,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin>),
    DimensionOneEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin>),
    DimensionOneLengthEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneFindRangesWithinGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneFindRangesWithinGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneFindRangesThatFullyContainTheGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneFindRangesThatFullyContainTheGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneStrictlyToLeftOfRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneStrictlyToLeftOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneStrictlyToRightOfRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneStrictlyToRightOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneIncludedLowerBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneExcludedUpperBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneGreaterThanIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThanIncludedLowerBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneGreaterThanExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThanExcludedUpperBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneOverlapWithRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneOverlapWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneAdjacentWithRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneAdjacentWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneRangeLength(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneRangeLength),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeWhereElement {
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
            Self::DimensionOneGreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
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
            Self::DimensionOneGreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneOverlapWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAdjacentWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneRangeLength(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeWhereElement {
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
            Self::DimensionOneGreaterThanIncludedLowerBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThanExcludedUpperBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneOverlapWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAdjacentWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneRangeLength(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeRead(OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin);
impl OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeRead {
    pub fn new(value: std::option::Option<std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>>) -> Self {
        Self(OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeReadInner = std::option::Option<std::vec::Vec<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>>;
pub type OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeUpdate = OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeOrigin;
impl crate::PostgresqlType for OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRange {
    type TableTypeDeclaration = OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeTableTypeDeclaration;
    type Create = OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeCreate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
    type Select = OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeWhereElement;
    type Read = OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeRead;
    type ReadInner = OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        match value.0 .0 {
            Some(value) => Some(
                value
                    .0
                    .into_iter()
                    .map(|element| <SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRange as crate::PostgresqlType>::into_inner(SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead(element)))
                    .collect(),
            ),
            None => None,
        }
    }
    type Update = OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNotNullNumRangeUpdate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
}
#[derive(Debug)]
pub struct VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRange;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin(std::vec::Vec<OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin>);
impl VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin {
    pub fn new(value: std::vec::Vec<std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>>) -> Self {
        Self(value.into_iter().map(|element| OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin::new(element)).collect())
    }
}
impl std::fmt::Display for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::vec::Vec<OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::vec::Vec<OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::vec::Vec<OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numrange[] not null")
    }
}
pub type VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeTableTypeDeclaration = VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin;
pub type VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeCreate = VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeSelect {
    dimension1_pagination: crate::PaginationStartsWithOne,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin>),
    DimensionOneEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneLengthEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneFindRangesWithinGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneFindRangesWithinGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneFindRangesThatFullyContainTheGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneFindRangesThatFullyContainTheGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneStrictlyToLeftOfRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneStrictlyToLeftOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneStrictlyToRightOfRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneStrictlyToRightOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneIncludedLowerBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneExcludedUpperBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneGreaterThanIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThanIncludedLowerBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneGreaterThanExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThanExcludedUpperBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneOverlapWithRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneOverlapWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneAdjacentWithRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneAdjacentWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneRangeLength(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneRangeLength),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeWhereElement {
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
            Self::DimensionOneGreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
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
            Self::DimensionOneGreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneOverlapWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAdjacentWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneRangeLength(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeWhereElement {
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
            Self::DimensionOneGreaterThanIncludedLowerBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThanExcludedUpperBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneOverlapWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAdjacentWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneRangeLength(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeRead(VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin);
impl VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeRead {
    pub fn new(value: std::vec::Vec<std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>>) -> Self {
        Self(VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeReadInner = std::vec::Vec<std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>>;
pub type VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeUpdate = VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin;
impl crate::PostgresqlType for VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRange {
    type TableTypeDeclaration = VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeTableTypeDeclaration;
    type Create = VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeCreate;
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
    type Select = VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeWhereElement;
    type Read = VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeRead;
    type ReadInner = VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value
            .0
             .0
            .into_iter()
            .map(|element| match element.0 {
                Some(value) => Some(<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRange as crate::PostgresqlType>::into_inner(SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead(value))),
                None => None,
            })
            .collect()
    }
    type Update = VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeUpdate;
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
#[derive(Debug)]
pub struct OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRange;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin(std::option::Option<VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin>);
impl OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin {
    pub fn new(value: std::option::Option<std::vec::Vec<std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>>>) -> Self {
        Self(match value {
            Some(value) => Some(VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin::new(value)),
            None => None,
        })
    }
}
impl std::fmt::Display for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::option::Option<VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::option::Option<VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullArrayOfNullableNumRangeOrigin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::postgres::types::PgRange<sqlx::types::BigDecimal> as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} numrange[]")
    }
}
pub type OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeTableTypeDeclaration = OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin;
pub type OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeCreate = OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeSelect {
    dimension1_pagination: crate::PaginationStartsWithOne,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin>),
    DimensionOneEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneEqual<OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRangeOrigin>),
    DimensionOneLengthEqual(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthEqual),
    DimensionOneLengthMoreThan(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneLengthMoreThan),
    DimensionOneFindRangesWithinGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneFindRangesWithinGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneFindRangesThatFullyContainTheGivenRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneFindRangesThatFullyContainTheGivenRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneStrictlyToLeftOfRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneStrictlyToLeftOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneStrictlyToRightOfRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneStrictlyToRightOfRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneIncludedLowerBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneExcludedUpperBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneGreaterThanIncludedLowerBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThanIncludedLowerBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneGreaterThanExcludedUpperBound(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneGreaterThanExcludedUpperBound<<SqlxTypesBigDecimalAsNotNullNumeric as crate::PostgresqlType>::Read>),
    DimensionOneOverlapWithRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneOverlapWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneAdjacentWithRange(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneAdjacentWithRange<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeOrigin>),
    DimensionOneRangeLength(crate::where_element_filters::PostgresqlTypeWhereElementDimensionOneRangeLength),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeWhereElement {
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
            Self::DimensionOneGreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneGreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
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
            Self::DimensionOneGreaterThanIncludedLowerBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneGreaterThanExcludedUpperBound(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneOverlapWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneAdjacentWithRange(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneRangeLength(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeWhereElement {
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
            Self::DimensionOneGreaterThanIncludedLowerBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneGreaterThanExcludedUpperBound(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneOverlapWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneAdjacentWithRange(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneRangeLength(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeRead(OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin);
impl OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeRead {
    pub fn new(value: std::option::Option<std::vec::Vec<std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>>>) -> Self {
        Self(OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin::new(value))
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeReadInner = std::option::Option<std::vec::Vec<std::option::Option<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>>>>;
pub type OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeUpdate = OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeOrigin;
impl crate::PostgresqlType for OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRange {
    type TableTypeDeclaration = OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeTableTypeDeclaration;
    type Create = OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeCreate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
    type Select = OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeWhereElement;
    type Read = OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeRead;
    type ReadInner = OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        match value.0 .0 {
            Some(value) => Some(
                value
                    .0
                    .into_iter()
                    .map(|element| match element.0 {
                        Some(value) => Some(<SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRange as crate::PostgresqlType>::into_inner(SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRangeRead(value))),
                        None => None,
                    })
                    .collect(),
            ),
            None => None,
        }
    }
    type Update = OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableArrayOfNullableNumRangeUpdate;
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
        query = query.bind(match value.0 {
            Some(value) => Some(value),
            None => None,
        });
        query
    }
}
