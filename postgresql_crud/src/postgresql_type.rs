generate_postgresql_types::generate_postgresql_types!([
    {
        "postgresql_type": "StdPrimitiveI16AsInt2",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern_type": "Standart"
    },
    {
        "postgresql_type": "StdPrimitiveI64AsBigSerialInitializedByPostgresql",
        "not_null_or_nullable": "NotNull",
        "postgresql_type_pattern_type": "Standart"
    }
        // {
        //     "ArrayDimension1": {
        //         "dimension1_not_null_or_nullable": "Nullable"
        //     }
        // }
]);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
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
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, utoipa::ToSchema)]
pub struct SqlxTypesTimeUtcOffset(pub sqlx::types::time::UtcOffset);
impl serde::Serialize for SqlxTypesTimeUtcOffset {
    fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: serde::Serializer,
    {
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "SqlxTypesTimeUtcOffset", false as usize + 1 + 1 + 1)?;
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
        #[allow(non_camel_case_types)]
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
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
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
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SqlxTypesTimeUtcOffset")
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
        const FIELDS: &'static [&'static str] = &["hours", "minutes", "seconds"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SqlxTypesTimeUtcOffset",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SqlxTypesTimeUtcOffset>,
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
        let mut __serde_state = serde::Serializer::serialize_struct(__serializer, "NumBigintBigInt", false as usize + 1 + 1)?;
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
        #[allow(non_camel_case_types)]
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
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
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
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct NumBigintBigInt")
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
        const FIELDS: &'static [&'static str] = &["sign", "digits"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "NumBigintBigInt",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<NumBigintBigInt>,
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
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct NumBigintSign")
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
                marker: serde::__private::PhantomData::<NumBigintSign>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}

const CHAR_AND_VARCHAR_MAX_LENGTH: std::primitive::u32 = 10_485_760;
fn generate_must_be_between_1_and_length_message(length: &dyn std::fmt::Display) -> std::string::String {
    format!("value must be between 1(included) and {length}(included)")
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum StdStringStringAsCharNLengthTryFromStdPrimitiveU32ErrorNamed {
    NotValid {
        #[eo_to_std_string_string_serialize_deserialize]
        error_message: std::string::String,
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::u32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct StdStringStringAsCharNLength(std::primitive::u32);
impl std::convert::TryFrom<std::primitive::u32> for StdStringStringAsCharNLength {
    type Error = StdStringStringAsCharNLengthTryFromStdPrimitiveU32ErrorNamed;
    fn try_from(value: std::primitive::u32) -> Result<Self, Self::Error> {
        if (value == 0) || (value > CHAR_AND_VARCHAR_MAX_LENGTH) {
            Err(Self::Error::NotValid {
                error_message: generate_must_be_between_1_and_length_message(&CHAR_AND_VARCHAR_MAX_LENGTH),
                value,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        } else {
            Ok(Self(value))
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdStringStringAsCharNLength {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<StdStringStringAsCharNLength>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdStringStringAsCharNLength;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct StdStringStringAsCharNLength")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::primitive::u32 = <std::primitive::u32 as _serde::Deserialize>::deserialize(__e)?;
                    match StdStringStringAsCharNLength::try_from(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::primitive::u32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct StdStringStringAsCharNLength with 1 element"));
                        }
                    };
                    match StdStringStringAsCharNLength::try_from(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "StdStringStringAsCharNLength",
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdStringStringAsCharNLength>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for StdStringStringAsCharNLength {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum StdStringStringAsVarcharLengthTryFromStdPrimitiveU32ErrorNamed {
    NotValid {
        #[eo_to_std_string_string_serialize_deserialize]
        error_message: std::string::String,
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::u32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct StdStringStringAsVarcharLength(std::primitive::u32);
impl std::convert::TryFrom<std::primitive::u32> for StdStringStringAsVarcharLength {
    type Error = StdStringStringAsVarcharLengthTryFromStdPrimitiveU32ErrorNamed;
    fn try_from(value: std::primitive::u32) -> Result<Self, Self::Error> {
        if (value == 0) || (value > CHAR_AND_VARCHAR_MAX_LENGTH) {
            Err(Self::Error::NotValid {
                error_message: generate_must_be_between_1_and_length_message(&CHAR_AND_VARCHAR_MAX_LENGTH),
                value,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        } else {
            Ok(Self(value))
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdStringStringAsVarcharLength {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<StdStringStringAsVarcharLength>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdStringStringAsVarcharLength;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct StdStringStringAsVarcharLength")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::primitive::u32 = <std::primitive::u32 as _serde::Deserialize>::deserialize(__e)?;
                    match StdStringStringAsVarcharLength::try_from(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::primitive::u32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct StdStringStringAsVarcharLength with 1 element"));
                        }
                    };
                    match StdStringStringAsVarcharLength::try_from(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "StdStringStringAsVarcharLength",
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdStringStringAsVarcharLength>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for StdStringStringAsVarcharLength {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesBitVecAsBitLengthTryFromStdPrimitiveU64ErrorNamed {
    NotValid {
        #[eo_to_std_string_string_serialize_deserialize]
        error_message: std::string::String,
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::u64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct SqlxTypesBitVecAsBitLength(std::primitive::u64);
impl std::convert::TryFrom<std::primitive::u64> for SqlxTypesBitVecAsBitLength {
    type Error = SqlxTypesBitVecAsBitLengthTryFromStdPrimitiveU64ErrorNamed;
    fn try_from(value: std::primitive::u64) -> Result<Self, Self::Error> {
        let max_length = 8_589_934_592;
        if (value == 0) || (value > max_length) {
            Err(Self::Error::NotValid {
                error_message: generate_must_be_between_1_and_length_message(&max_length),
                value,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        } else {
            Ok(Self(value))
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxTypesBitVecAsBitLength {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SqlxTypesBitVecAsBitLength>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesBitVecAsBitLength;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct SqlxTypesBitVecAsBitLength")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::primitive::u64 = <std::primitive::u64 as _serde::Deserialize>::deserialize(__e)?;
                    match SqlxTypesBitVecAsBitLength::try_from(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::primitive::u64>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct SqlxTypesBitVecAsBitLength with 1 element"));
                        }
                    };
                    match SqlxTypesBitVecAsBitLength::try_from(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "SqlxTypesBitVecAsBitLength",
                __Visitor {
                    marker: _serde::__private::PhantomData::<SqlxTypesBitVecAsBitLength>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxTypesBitVecAsBitLength {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesBitVecAsVarbitLengthTryFromStdPrimitiveU32ErrorNamed {
    NotValid {
        #[eo_to_std_string_string_serialize_deserialize]
        error_message: std::string::String,
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::u32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct SqlxTypesBitVecAsVarbitLength(std::primitive::u32);
impl std::convert::TryFrom<std::primitive::u32> for SqlxTypesBitVecAsVarbitLength {
    type Error = SqlxTypesBitVecAsVarbitLengthTryFromStdPrimitiveU32ErrorNamed;
    fn try_from(value: std::primitive::u32) -> Result<Self, Self::Error> {
        let max_length = 83_886_080;
        if (value == 0) || (value > max_length) {
            Err(Self::Error::NotValid {
                error_message: generate_must_be_between_1_and_length_message(&max_length),
                value,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        } else {
            Ok(Self(value))
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxTypesBitVecAsVarbitLength {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SqlxTypesBitVecAsVarbitLength>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesBitVecAsVarbitLength;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct SqlxTypesBitVecAsVarbitLength")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::primitive::u32 = <std::primitive::u32 as _serde::Deserialize>::deserialize(__e)?;
                    match SqlxTypesBitVecAsVarbitLength::try_from(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::primitive::u32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct SqlxTypesBitVecAsVarbitLength with 1 element"));
                        }
                    };
                    match SqlxTypesBitVecAsVarbitLength::try_from(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "SqlxTypesBitVecAsVarbitLength",
                __Visitor {
                    marker: _serde::__private::PhantomData::<SqlxTypesBitVecAsVarbitLength>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxTypesBitVecAsVarbitLength {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/////////////////////////////
#[derive(Debug)]
pub struct VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin(std::vec::Vec<StdPrimitiveI16AsNotNullInt2Origin>);
impl VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin {
    pub fn new(value: std::vec::Vec<std::primitive::i16>) -> Self {
        Self(value.into_iter().map(|element| StdPrimitiveI16AsNotNullInt2Origin::new(element)).collect())
    }
}
impl std::fmt::Display for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::vec::Vec<StdPrimitiveI16AsNotNullInt2Origin> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::vec::Vec<StdPrimitiveI16AsNotNullInt2Origin> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::vec::Vec<StdPrimitiveI16AsNotNullInt2Origin> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <std::primitive::i16 as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} int2[] not null")
    }
}
pub type VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2TableTypeDeclaration = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin;
pub type VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Create = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Select {
    dimension1_pagination: crate::Pagination,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Select {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            dimension1_pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2WhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2WhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2WhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2WhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
pub type VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Read = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin;
pub type VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Update = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Origin;
impl crate::PostgresqlType for VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 {
    type TableTypeDeclaration = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2TableTypeDeclaration;
    type Create = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Create;
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
    type Select = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Select;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{column}[{}:{}]", value.dimension1_pagination.start(), value.dimension1_pagination.end(),)
    }
    type WhereElement = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2WhereElement;
    type Read = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Read;
    type Update = VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2Update;
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