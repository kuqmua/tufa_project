#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct Pagination {
    limit: std::primitive::i64,
    offset: std::primitive::i64,
}
impl<'de> serde::Deserialize<'de> for Pagination {
    fn deserialize<__D>(
        __deserializer: __D,
    ) -> serde::__private::Result<Self, __D::Error>
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
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter<'_>,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    __formatter,
                    "field identifier",
                )
            }
            fn visit_u64<__E>(
                self,
                __value: u64,
            ) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(
                self,
                __value: &str,
            ) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "limit" => serde::__private::Ok(__Field::__field0),
                    "offset" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(
                self,
                __value: &[u8],
            ) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"limit" => serde::__private::Ok(__Field::__field0),
                    b"offset" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(
                    __deserializer,
                    __FieldVisitor,
                )
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<Pagination>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = Pagination;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter<'_>,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    __formatter,
                    "struct Pagination",
                )
            }
            #[inline]
            fn visit_seq<__A>(
                self,
                mut __seq: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<
                    std::primitive::i64,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                0usize,
                                &"struct Pagination with 2 elements",
                            ),
                        );
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<
                    std::primitive::i64,
                >(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                1usize,
                                &"struct Pagination with 2 elements",
                            ),
                        );
                    }
                };
                match Pagination::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
                }
            }
            #[inline]
            fn visit_map<__A>(
                self,
                mut __map: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::primitive::i64> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::primitive::i64> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                    __Field,
                >(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("limit"),
                                );
                            }
                            __field0 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::primitive::i64,
                                >(&mut __map)?,
                            );
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("offset"),
                                );
                            }
                            __field1 = serde::__private::Some(
                                serde::de::MapAccess::next_value::<
                                    std::primitive::i64,
                                >(&mut __map)?,
                            );
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<
                                serde::de::IgnoredAny,
                            >(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => {
                        serde::__private::de::missing_field("limit")?
                    }
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => {
                        serde::__private::de::missing_field("offset")?
                    }
                };
                match Pagination::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["limit", "offset"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "Pagination",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<Pagination>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum PaginationTryNewErrorNamed {
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: std::primitive::i64,
        #[eo_to_std_string_string_serialize_deserialize]
        offset: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    LimitIsLessThanOrEqualToZero {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OffsetIsLessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        offset: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Pagination {
    pub fn try_new(limit: std::primitive::i64, offset: std::primitive::i64) -> Result<Self, PaginationTryNewErrorNamed> {
        if limit <= 0 || offset < 0{
            if limit <= 0 {
                Err(PaginationTryNewErrorNamed::LimitIsLessThanOrEqualToZero {
                    limit,
                    code_occurence: error_occurence_lib::code_occurence!()
                })
            }
            else {
                Err(PaginationTryNewErrorNamed::OffsetIsLessThanZero {
                    offset,
                    code_occurence: error_occurence_lib::code_occurence!()
                })
            }
        }
        else {
            if offset.checked_add(limit).is_some() {
                Ok(Self{ limit, offset })
            }
            else {
                Err(PaginationTryNewErrorNamed::OffsetPlusLimitIsIntOverflow {
                    limit,
                    offset,
                    code_occurence: error_occurence_lib::code_occurence!()
                })
            }
        }
    }
    pub fn start(&self) -> std::primitive::i64 {
        self.offset
    }
    pub fn end(&self) -> std::primitive::i64 {
        self.offset + self.limit
    }
}
//for ReadMany in GeneratePostgresqlCrud
impl crate::BindQuery for Pagination {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(limit_increment) => {
                *increment = limit_increment;
                match increment.checked_add(1) {
                    Some(offset_increment) => {
                        *increment = offset_increment;
                        Ok(format!("limit ${limit_increment} offset ${offset_increment}"))
                    }
                    None => Err(crate::QueryPartErrorNamed::CheckedAdd {
                        code_occurence: error_occurence_lib::code_occurence!()
                    })
                }
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd {
                code_occurence: error_occurence_lib::code_occurence!()
            }),
        }
    }
    fn bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.limit);
        query = query.bind(self.offset);
        query
    }
}
impl crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for Pagination {
    #[inline]
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            limit: 3,
            offset: std::default::Default::default(),
        }
    }
}