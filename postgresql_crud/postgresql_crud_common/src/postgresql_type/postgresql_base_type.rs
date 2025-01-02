#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypePrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementNumber,
)]
pub struct StdPrimitiveI16(pub std::primitive::i16);
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypePrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementNumber,
)]
pub struct StdPrimitiveI32(pub std::primitive::i32);
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypePrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementNumber,
)]
pub struct StdPrimitiveI64(pub std::primitive::i64);
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementNumber,
)]
pub struct StdPrimitiveF32(pub std::primitive::f32);
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementNumber,
)]
pub struct StdPrimitiveF64(pub std::primitive::f64);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementBool,
)]
pub struct StdPrimitiveBool(pub std::primitive::bool); //todo maybe make it private? //todo column "std_primitive_bool_as_postgresql_bool" is of type boolean but expression is of type bigint
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementStdStringString,
)]
pub struct StdStringString(pub std::string::String);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensStdVecVecStdPrimitiveU8,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementStdVecVecStdPrimitiveU8,
)]
pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);


#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    //todo utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxPostgresTypesPgInterval,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgInterval,
)]
pub struct SqlxPostgresTypesPgInterval(pub sqlx::postgres::types::PgInterval);
impl serde::Serialize for SqlxPostgresTypesPgInterval {
    fn serialize<S>(&self, serializer: S) -> serde::__private::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serde_state = serde::Serializer::serialize_struct(serializer, "SqlxPostgresTypesPgInterval", false as usize + 1 + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "months", &self.0.months)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "days", &self.0.days)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "microseconds", &self.0.microseconds)?;
        serde::ser::SerializeStruct::end(serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Months,
            Days,
            Microseconds,
        }
        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl serde::de::Visitor<'_> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str("`months` or `days` or `microseconds`")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "months" => Ok(Field::Months),
                            "days" => Ok(Field::Days),
                            "microseconds" => Ok(Field::Microseconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct SqlxPostgresTypesPgIntervalVisitor;
        impl<'de> serde::de::Visitor<'de> for SqlxPostgresTypesPgIntervalVisitor {
            type Value = SqlxPostgresTypesPgInterval;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct SqlxPostgresTypesPgInterval")
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<SqlxPostgresTypesPgInterval, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let months = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let days = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let microseconds = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                Ok(SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval { months, days, microseconds }))
            }
            fn visit_map<V>(self, mut map: V) -> Result<SqlxPostgresTypesPgInterval, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut months = None;
                let mut days = None;
                let mut microseconds = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Months => {
                            if months.is_some() {
                                return Err(serde::de::Error::duplicate_field("months"));
                            }
                            months = Some(map.next_value()?);
                        }
                        Field::Days => {
                            if days.is_some() {
                                return Err(serde::de::Error::duplicate_field("days"));
                            }
                            days = Some(map.next_value()?);
                        }
                        Field::Microseconds => {
                            if microseconds.is_some() {
                                return Err(serde::de::Error::duplicate_field("microseconds"));
                            }
                            microseconds = Some(map.next_value()?);
                        }
                    }
                }
                let months = months.ok_or_else(|| serde::de::Error::missing_field("months"))?;
                let days = days.ok_or_else(|| serde::de::Error::missing_field("days"))?;
                let microseconds = microseconds.ok_or_else(|| serde::de::Error::missing_field("microseconds"))?;
                Ok(SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval { months, days, microseconds }))
            }
        }
        const FIELDS: &[&str] = &["months", "days", "microseconds"];
        deserializer.deserialize_struct("SqlxPostgresTypesPgInterval", FIELDS, SqlxPostgresTypesPgIntervalVisitor)
    }
}
////////////////////////////////////////////////////////////

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    //todo utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxPostgresTypesPgRange,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeStdPrimitiveI32,
)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32(pub sqlx::postgres::types::PgRange<std::primitive::i32>);
impl serde::Serialize for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn serialize<S>(&self, serializer: S) -> serde::__private::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serde_state = serde::Serializer::serialize_struct(serializer, "SqlxPostgresTypesPgRangeStdPrimitiveI32", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "start", &self.0.start)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "end", &self.0.end)?;
        serde::ser::SerializeStruct::end(serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Start,
            End,
        }
        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl serde::de::Visitor<'_> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str("`start` or `end`")
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
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct SqlxPostgresTypesPgRangeStdPrimitiveI32Visitor;
        impl<'de> serde::de::Visitor<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI32Visitor {
            type Value = SqlxPostgresTypesPgRangeStdPrimitiveI32;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct SqlxPostgresTypesPgRangeStdPrimitiveI32")
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<SqlxPostgresTypesPgRangeStdPrimitiveI32, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let start = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let end = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange { start, end }))
            }
            fn visit_map<V>(self, mut map: V) -> Result<SqlxPostgresTypesPgRangeStdPrimitiveI32, V::Error>
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
                let start = start.ok_or_else(|| serde::de::Error::missing_field("start"))?;
                let end = end.ok_or_else(|| serde::de::Error::missing_field("end"))?;
                Ok(SqlxPostgresTypesPgRangeStdPrimitiveI32(sqlx::postgres::types::PgRange { start, end }))
            }
        }
        const FIELDS: &[&str] = &["start", "end"];
        deserializer.deserialize_struct("SqlxPostgresTypesPgRangeStdPrimitiveI32", FIELDS, SqlxPostgresTypesPgRangeStdPrimitiveI32Visitor)
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    //todo utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxPostgresTypesPgRange,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeStdPrimitiveI64,
)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64(pub sqlx::postgres::types::PgRange<std::primitive::i64>);
impl serde::Serialize for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn serialize<S>(&self, serializer: S) -> serde::__private::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serde_state = serde::Serializer::serialize_struct(serializer, "SqlxPostgresTypesPgRangeStdPrimitiveI64", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "start", &self.0.start)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "end", &self.0.end)?;
        serde::ser::SerializeStruct::end(serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Start,
            End,
        }
        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl serde::de::Visitor<'_> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str("`start` or `end`")
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
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct SqlxPostgresTypesPgRangeStdPrimitiveI64Visitor;
        impl<'de> serde::de::Visitor<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI64Visitor {
            type Value = SqlxPostgresTypesPgRangeStdPrimitiveI64;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct SqlxPostgresTypesPgRangeStdPrimitiveI64")
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<SqlxPostgresTypesPgRangeStdPrimitiveI64, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let start = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let end = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange { start, end }))
            }
            fn visit_map<V>(self, mut map: V) -> Result<SqlxPostgresTypesPgRangeStdPrimitiveI64, V::Error>
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
                let start = start.ok_or_else(|| serde::de::Error::missing_field("start"))?;
                let end = end.ok_or_else(|| serde::de::Error::missing_field("end"))?;
                Ok(SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange { start, end }))
            }
        }
        const FIELDS: &[&str] = &["start", "end"];
        deserializer.deserialize_struct("SqlxPostgresTypesPgRangeStdPrimitiveI64", FIELDS, SqlxPostgresTypesPgRangeStdPrimitiveI64Visitor)
    }
}

//tstzrange

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxPostgresTypesPgRange,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>);
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "start",
                &self.0.start,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "end",
                &self.0.end,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "start" => _serde::__private::Ok(__Field::__field0),
                        "end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"start" => _serde::__private::Ok(__Field::__field0),
                        b"end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<
                    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        std::collections::Bound<
                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
                        >,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        std::collections::Bound<
                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
                        >,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::postgres::types::PgRange {
                        start: __field0,
                        end: __field1,
                    }))
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        std::collections::Bound<
                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
                        >,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        std::collections::Bound<
                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
                        >,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::collections::Bound<
                                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::collections::Bound<
                                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("start")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("end")?
                        }
                    };
                    _serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(sqlx::postgres::types::PgRange {
                        start: __field0,
                        end: __field1,
                    }))
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["start", "end"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxPostgresTypesPgRange,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>);
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "start",
                &self.0.start,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "end",
                &self.0.end,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "start" => _serde::__private::Ok(__Field::__field0),
                        "end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"start" => _serde::__private::Ok(__Field::__field0),
                        b"end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<
                    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        std::collections::Bound<
                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
                        >,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        std::collections::Bound<
                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
                        >,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::postgres::types::PgRange {
                        start: __field0,
                        end: __field1,
                    }))
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        std::collections::Bound<
                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
                        >,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        std::collections::Bound<
                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
                        >,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::collections::Bound<
                                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::collections::Bound<
                                            sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("start")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("end")?
                        }
                    };
                    _serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(sqlx::postgres::types::PgRange {
                        start: __field0,
                        end: __field1,
                    }))
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["start", "end"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxPostgresTypesPgRange,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>);
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "start",
                &self.0.start,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "end",
                &self.0.end,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "start" => _serde::__private::Ok(__Field::__field0),
                        "end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"start" => _serde::__private::Ok(__Field::__field0),
                        b"end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<
                    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        std::collections::Bound<sqlx::types::chrono::NaiveDateTime>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        std::collections::Bound<sqlx::types::chrono::NaiveDateTime>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(sqlx::postgres::types::PgRange {
                        start: __field0,
                        end: __field1,
                    }))
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        std::collections::Bound<sqlx::types::chrono::NaiveDateTime>,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        std::collections::Bound<sqlx::types::chrono::NaiveDateTime>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::collections::Bound<sqlx::types::chrono::NaiveDateTime>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::collections::Bound<sqlx::types::chrono::NaiveDateTime>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("start")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("end")?
                        }
                    };
                    _serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(sqlx::postgres::types::PgRange {
                        start: __field0,
                        end: __field1,
                    }))
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["start", "end"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxPostgresTypesPgRange,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>);
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "start",
                &self.0.start,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "end",
                &self.0.end,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "start" => _serde::__private::Ok(__Field::__field0),
                        "end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"start" => _serde::__private::Ok(__Field::__field0),
                        b"end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<
                    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        std::collections::Bound<sqlx::types::chrono::NaiveDate>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        std::collections::Bound<sqlx::types::chrono::NaiveDate>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(sqlx::postgres::types::PgRange {
                        start: __field0,
                        end: __field1,
                    }))
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        std::collections::Bound<sqlx::types::chrono::NaiveDate>,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        std::collections::Bound<sqlx::types::chrono::NaiveDate>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::collections::Bound<sqlx::types::chrono::NaiveDate>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::collections::Bound<sqlx::types::chrono::NaiveDate>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("start")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("end")?
                        }
                    };
                    _serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(sqlx::postgres::types::PgRange {
                        start: __field0,
                        end: __field1,
                    }))
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["start", "end"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};


#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxPostgresTypesPgRange,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeSqlxTypesDecimal,
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimal(pub sqlx::postgres::types::PgRange<sqlx::types::Decimal>);
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SqlxPostgresTypesPgRangeSqlxTypesDecimal",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "start",
                &self.0.start,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "end",
                &self.0.end,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxPostgresTypesPgRangeSqlxTypesDecimal {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
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
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "start" => _serde::__private::Ok(__Field::__field0),
                        "end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"start" => _serde::__private::Ok(__Field::__field0),
                        b"end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<
                    SqlxPostgresTypesPgRangeSqlxTypesDecimal,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxPostgresTypesPgRangeSqlxTypesDecimal;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SqlxPostgresTypesPgRangeSqlxTypesDecimal",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        std::collections::Bound<sqlx::types::Decimal>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SqlxPostgresTypesPgRangeSqlxTypesDecimal with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        std::collections::Bound<sqlx::types::Decimal>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SqlxPostgresTypesPgRangeSqlxTypesDecimal with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesDecimal(sqlx::postgres::types::PgRange {
                        start: __field0,
                        end: __field1,
                    }))
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        std::collections::Bound<sqlx::types::Decimal>,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        std::collections::Bound<sqlx::types::Decimal>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::collections::Bound<sqlx::types::Decimal>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::collections::Bound<sqlx::types::Decimal>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("start")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("end")?
                        }
                    };
                    _serde::__private::Ok(SqlxPostgresTypesPgRangeSqlxTypesDecimal(sqlx::postgres::types::PgRange {
                        start: __field0,
                        end: __field1,
                    }))
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["start", "end"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxPostgresTypesPgRangeSqlxTypesDecimal",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        SqlxPostgresTypesPgRangeSqlxTypesDecimal,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};

//todo another ranges

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxTypesTimeDate,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeDate,
)]
pub struct SqlxTypesTimeDate(pub sqlx::types::time::Date);
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SqlxTypesTimeDateTryNewErrorNamed {
    FromCalendarDate {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    LessThanMinimumPostgresqlValue {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl SqlxTypesTimeDate {
    fn try_new(
        year: std::primitive::i32,
        month: time::Month,
        day: std::primitive::u8,
    ) -> Result<Self, SqlxTypesTimeDateTryNewErrorNamed> {
        match sqlx::types::time::Date::from_calendar_date(
            year,
            month,
            day,
        ) {
            Ok(value) => {
                //postgresql having minimum value "year": -4712, "month": 1, "day": 1. maximum "year": 5874897, "month": 12, "day": 31. but library type does not impl that correctly(in type max is 9999)
                let minimum = sqlx::types::time::Date::from_calendar_date(
                    -4713,
                    time::Month::December,
                    31,
                ).unwrap();
                if minimum > value {
                    Err(SqlxTypesTimeDateTryNewErrorNamed::LessThanMinimumPostgresqlValue {
                        value: format!("{value:?}"),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
                else {
                    Ok(Self(value))
                }
            },
            Err(error) => Err(SqlxTypesTimeDateTryNewErrorNamed::FromCalendarDate {
                value: format!("{error:?}"),
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxTypesTimeDate {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SqlxTypesTimeDate",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "year",
                &self.0.year(),
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "month",
                &self.0.month(),
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "day",
                &self.0.day(),
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
//todo default deserialize impl can cause an postgresql error "date of out range". pub const fn from_ordinal_date( do it too. if u want to check it just use sqlx::types::time::Date::MIN
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxTypesTimeDate {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
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
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "year" => _serde::__private::Ok(__Field::__field0),
                        "month" => _serde::__private::Ok(__Field::__field1),
                        "day" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"year" => _serde::__private::Ok(__Field::__field0),
                        b"month" => _serde::__private::Ok(__Field::__field1),
                        b"day" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SqlxTypesTimeDate>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesTimeDate;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SqlxTypesTimeDate",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        std::primitive::i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SqlxTypesTimeDate with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        time::Month,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SqlxTypesTimeDate with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        std::primitive::u8,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct SqlxTypesTimeDate with 3 elements",
                                ),
                            );
                        }
                    };
                    match SqlxTypesTimeDate::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                    }
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<std::primitive::i32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<time::Month> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<std::primitive::u8> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("year"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::primitive::i32,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("month"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        time::Month,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("day"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::primitive::u8,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("year")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("month")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("day")?
                        }
                    };
                    match SqlxTypesTimeDate::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["year", "month", "day"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxTypesTimeDate",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SqlxTypesTimeDate>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};


#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxTypesChronoNaiveTime,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxTypesChronoNaiveTime,
)]
pub struct SqlxTypesChronoNaiveTime(pub sqlx::types::chrono::NaiveTime);





// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxTypesTimeOffsetDateTime,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxTypesTimeOffsetDateTime,
// )]
// pub struct SqlxTypesTimeOffsetDateTime(pub sqlx::types::time::OffsetDateTime);
//todo initialize with this in deserialization
// pub const fn new_in_offset(
//     date: Date,
//     time: Time,
//     offset: UtcOffset,
// ) -> OffsetDateTime
// impl serde::Serialize for SqlxTypesTimeOffsetDateTime {
//     fn serialize<__S>(&self, __serializer: __S) -> serde::__private::Result<__S::Ok, __S::Error>
//     where
//         __S: serde::Serializer,
//     {
//         serde::Serializer::serialize_newtype_struct(__serializer, "SqlxTypesTimeOffsetDateTime", &self.0.unix_timestamp())
//     }
// }
// impl<'de> serde::Deserialize<'de> for SqlxTypesTimeOffsetDateTime {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<SqlxTypesTimeOffsetDateTime>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = SqlxTypesTimeOffsetDateTime;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "tuple struct SqlxTypesTimeOffsetDateTime")
//             }
//             #[inline]
//             fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
//             where
//                 __E: serde::Deserializer<'de>,
//             {
//                 let __field0: std::primitive::i64 = <std::primitive::i64 as serde::Deserialize>::deserialize(__e)?;
//                 serde::__private::Ok(SqlxTypesTimeOffsetDateTime(match sqlx::types::time::OffsetDateTime::from_unix_timestamp(__field0) {
//                     Ok(value) => value,
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(error));
//                     }
//                 }))
//             }
//             #[inline]
//             fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct SqlxTypesTimeOffsetDateTime with 1 element"));
//                     }
//                 };
//                 serde::__private::Ok(SqlxTypesTimeOffsetDateTime(match sqlx::types::time::OffsetDateTime::from_unix_timestamp(__field0) {
//                     Ok(value) => value,
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(error));
//                     }
//                 }))
//             }
//         }
//         serde::Deserializer::deserialize_newtype_struct(
//             __deserializer,
//             "SqlxTypesTimeOffsetDateTime",
//             __Visitor {
//                 marker: serde::__private::PhantomData::<SqlxTypesTimeOffsetDateTime>,
//                 lifetime: serde::__private::PhantomData,
//             },
//         )
//     }
// }
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxTypesTimeOffsetDateTime {
//     #[inline]
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(sqlx::types::time::OffsetDateTime::UNIX_EPOCH)
//     }
// }




// pub struct SqlxTypesTimePrimitiveDateTime(pub sqlx::types::time::PrimitiveDateTime);

// pub struct SqlxTypesBigDecimal(pub sqlx::types::BigDecimal);


// pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate(pub sqlx::postgres::types::PgRange<sqlx::types::time::Date>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(pub sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>);


// pub struct SqlxPostgresTypesPgMoney(pub sqlx::postgres::types::PgMoney);
// pub struct SqlxPostgresTypesPgCiText(pub sqlx::postgres::types::PgCiText);

// pub struct SqlxTypesDecimal(pub sqlx::types::Decimal);


// pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtc(pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>);
// pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocal(pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>);
// pub struct SqlxTypesChronoNaiveDateTime(pub sqlx::types::chrono::NaiveDateTime);
// pub struct SqlxTypesChronoNaiveDate(pub sqlx::types::chrono::NaiveDate);
// pub struct SqlxPostgresTypesPgTimeTz(pub sqlx::postgres::types::PgTimeTz);



// pub struct SqlxTypesTimeTime(pub sqlx::types::time::Time);


// pub struct SqlxTypesUuidUuid(pub sqlx::types::uuid::Uuid);
// pub struct SqlxTypesIpnetworkIpNetwork(pub sqlx::types::ipnetwork::IpNetwork);
// pub struct StdNetIpAddr(pub std::net::IpAddr);
// pub struct SqlxTypesMacAddressMacAddress(pub sqlx::types::mac_address::MacAddress);
// pub struct SqlxTypesBitVec(pub sqlx::types::BitVec);

// pub struct SerdeJsonValue(pub serde_json::Value);

////////////////////////////////
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EncodeFormat {
    Base64,
    Hex,
    Escape
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
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for EncodeFormat {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
///////////////////////////////////////////////////////