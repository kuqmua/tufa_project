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
    }
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
