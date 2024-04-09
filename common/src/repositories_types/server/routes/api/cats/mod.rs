#[derive(Debug, postgresql_crud::GeneratePostgresqlCrud)]
#[postgresql_crud::create_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::create_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::read_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::read_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::update_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::update_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::delete_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::delete_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::additional_http_status_codes_error_variants{
    #[path(crate::server::extractors::commit_extractor::)]
    enum CommitExtractorCheckErrorNamed {
        #[tvfrr_400_bad_request]
        CommitExtractorNotEqual {
            #[eo_display_with_serialize_deserialize]
            commit_not_equal: std::string::String,
            #[eo_display_with_serialize_deserialize]
            commit_to_use: std::string::String,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
        #[tvfrr_400_bad_request]
        CommitExtractorToStrConversion {
            #[eo_display]
            commit_to_str_conversion: http::header::ToStrError,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
        #[tvfrr_400_bad_request]
        NoCommitExtractorHeader {
            #[eo_display_with_serialize_deserialize]
            no_commit_header: std::string::String,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
    // ;
    // enum SomethingErrorNamed {
    //     #[tvfrr_400_bad_request]
    //     SomethingVariant {
    //         #[eo_display_with_serialize_deserialize]
    //         something_field: std::string::String,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //     },
    // }
}]
pub struct Dog {
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdPrimitiveBoolAsPostgresqlBool,
    // pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt,
    // pub std_primitive_i16_as_postgresql_small_int_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallIntNotNull,
    // pub std_primitive_i16_as_postgresql_small_serial: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerial,
    // pub std_primitive_i16_as_postgresql_small_serial_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    // pub std_primitive_i16_as_postgresql_small_int2: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2,
    // pub std_primitive_i16_as_postgresql_small_int2_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2NotNull,

    pub std_primitive_i32_as_postgresql_int: postgresql_crud::StdPrimitiveI32AsPostgresqlInt,
    // pub std_primitive_i32_as_postgresql_int_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlIntNotNull,
    // pub std_primitive_i32_as_postgresql_serial: postgresql_crud::StdPrimitiveI32AsPostgresqlSerial,
    // pub std_primitive_i32_as_postgresql_serial_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlSerialNotNull,
    // pub std_primitive_i32_as_postgresql_int4: postgresql_crud::StdPrimitiveI32AsPostgresqlInt4,
    // pub std_primitive_i32_as_postgresql_int4_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlInt4NotNull,

    // pub std_primitive_i64_as_postgresql_big_int: postgresql_crud::StdPrimitiveI64AsPostgresqlBigInt,
    // pub std_primitive_i64_as_postgresql_big_int_not_null: postgresql_crud::StdPrimitiveI64AsPostgresqlBigIntNotNull,
    // pub std_primitive_i64_as_postgresql_big_serial: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerial,
    // pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
    // pub std_primitive_i64_as_postgresql_big_int8: postgresql_crud::StdPrimitiveI64AsPostgresqlInt8,
    // pub std_primitive_i64_as_postgresql_big_int8_not_null: postgresql_crud::StdPrimitiveI64AsPostgresqlInt8NotNull,

    // pub std_primitive_f32_as_postgresql_real: postgresql_crud::StdPrimitiveF32AsPostgresqlReal,
    // pub std_primitive_f32_as_postgresql_real_not_null: postgresql_crud::StdPrimitiveF32AsPostgresqlRealNotNull,
    // pub std_primitive_f32_as_postgresql_float4: postgresql_crud::StdPrimitiveF32AsPostgresqlFloat4,
    // pub std_primitive_f32_as_postgresql_float4_not_null: postgresql_crud::StdPrimitiveF32AsPostgresqlFloat4NotNull,

    // pub std_primitive_f64_as_postgresql_double_precision: postgresql_crud::StdPrimitiveF64AsPostgresqlDoublePrecision,
    // pub std_primitive_f64_as_postgresql_double_precision_not_null: postgresql_crud::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull,
    // pub std_primitive_f64_as_postgresql_float8: postgresql_crud::StdPrimitiveF64AsPostgresqlFloat8,
    // pub std_primitive_f64_as_postgresql_float8_not_null: postgresql_crud::StdPrimitiveF64AsPostgresqlFloat8NotNull,

    // pub std_string_string_as_postgresql_varchar: postgresql_crud::StdStringStringAsPostgresqlVarchar,
    // pub std_string_string_as_postgresql_varchar_not_null: postgresql_crud::StdStringStringAsPostgresqlVarcharNotNull,
    // pub std_string_string_as_postgresql_char_n: postgresql_crud::StdStringStringAsPostgresqlCharN,
    // pub std_string_string_as_postgresql_char_n_not_null: postgresql_crud::StdStringStringAsPostgresqlCharNNotNull,
    // pub std_string_string_as_postgresql_text: postgresql_crud::StdStringStringAsPostgresqlText,
    // pub std_string_string_as_postgresql_text_not_null: postgresql_crud::StdStringStringAsPostgresqlTextNotNull,
    // pub std_string_string_as_postgresql_ci_text: postgresql_crud::StdStringStringAsPostgresqlCiText,
    // pub std_string_string_as_postgresql_ci_text_not_null: postgresql_crud::StdStringStringAsPostgresqlCiTextNotNull,

    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea: postgresql_crud::StdVecVecStdPrimitiveU8AsPostgresqlBytea,
    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea_not_null: postgresql_crud::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,

    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval: postgresql_crud::SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval_not_null: postgresql_crud::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_money_as_postgresql_money: postgresql_crud::SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
    // pub sqlx_postgres_types_pg_money_as_postgresql_money_not_null: postgresql_crud::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,

    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text: postgresql_crud::SqlxPostgresTypesPgCiTextAsPostgresqlCiText,
    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text_not_null: postgresql_crud::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,

    // pub sqlx_types_big_decimal_as_postgresql_numeric: postgresql_crud::SqlxTypesBigDecimalAsPostgresqlNumeric,
    // pub sqlx_types_big_decimal_as_postgresql_numeric_not_null: postgresql_crud::SqlxTypesBigDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_decimal_as_postgresql_numeric: postgresql_crud::SqlxTypesDecimalAsPostgresqlNumeric,
    // pub sqlx_types_decimal_as_postgresql_numeric_not_null: postgresql_crud::SqlxTypesDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_not_null: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_not_null: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp: postgresql_crud::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_chrono_naive_date_as_postgresql_date: postgresql_crud::SqlxTypesChronoNaiveDateAsPostgresqlDate,
    // pub sqlx_types_chrono_naive_date_as_postgresql_date_not_null: postgresql_crud::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,

    // pub sqlx_types_chrono_naive_time_as_postgresql_time: postgresql_crud::SqlxTypesChronoNaiveTimeAsPostgresqlTime,
    // pub sqlx_types_chrono_naive_time_as_postgresql_time_not_null: postgresql_crud::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,

    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz: postgresql_crud::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz,
    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz_not_null: postgresql_crud::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull,

    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp: postgresql_crud::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz: postgresql_crud::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,
    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_not_null: postgresql_crud::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_time_date_as_postgresql_date: postgresql_crud::SqlxTypesTimeDateAsPostgresqlDate,
    // pub sqlx_types_time_date_as_postgresql_date_not_null: postgresql_crud::SqlxTypesTimeDateAsPostgresqlDateNotNull,

    // pub sqlx_types_time_time_as_postgresql_time: postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTime,
    // pub sqlx_types_time_time_as_postgresql_time_not_null: postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

    // pub sqlx_types_uuid_uuid_as_postgresql_uuid: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuid,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNull,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough //fails too but primary key is a different logic. need refactor it as different task 

    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet_not_null: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr_not_null: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

    // pub std_net_ip_addr_as_postgresql_inet: postgresql_crud::StdNetIpAddrAsPostgresqlInet,
    // pub std_net_ip_addr_as_postgresql_inet_not_null: postgresql_crud::StdNetIpAddrAsPostgresqlInetNotNull,
    // pub std_net_ip_addr_as_postgresql_cidr: postgresql_crud::StdNetIpAddrAsPostgresqlCidr,
    // pub std_net_ip_addr_as_postgresql_cidr_not_null: postgresql_crud::StdNetIpAddrAsPostgresqlCidrNotNull,

    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr: postgresql_crud::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr_not_null: postgresql_crud::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,

    // pub sqlx_types_bit_vec_as_postgresql_bit: postgresql_crud::SqlxTypesBitVecAsPostgresqlBit,
    // pub sqlx_types_bit_vec_as_postgresql_bit_not_null: postgresql_crud::SqlxTypesBitVecAsPostgresqlBitNotNull,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit: postgresql_crud::SqlxTypesBitVecAsPostgresqlVarBit,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit_not_null: postgresql_crud::SqlxTypesBitVecAsPostgresqlVarBitNotNull,

    //todo what to do with generic?
    // pub sqlx_types_json_t_as_postgresql_json: postgresql_crud::SqlxTypesJsonTAsPostgresqlJson::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_not_null: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonNotNull::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_b: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonB::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonBNotNull::<Something>,//todo

    // pub serde_json_value_as_postgresql_json: postgresql_crud::SerdeJsonValueAsPostgresqlJson,
    // pub serde_json_value_as_postgresql_json_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonNotNull,
    // pub serde_json_value_as_postgresql_json_b: postgresql_crud::SerdeJsonValueAsPostgresqlJsonB,
    // pub serde_json_value_as_postgresql_json_b_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonBNotNull,
}

// #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
// pub struct Something {
//     something: std::string::String,
// }

////////////////////////////////////////////////////////////////////////
pub const TABLE_NAME: &str = "dogs";
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DogOptions {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<
        postgresql_crud::Value<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>,
    >,
    pub std_primitive_bool_as_postgresql_bool: std::option::Option<
        postgresql_crud::Value<
            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
    >,
    pub std_primitive_i16_as_postgresql_small_int: std::option::Option<
        postgresql_crud::Value<
            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
        >,
    >,
    pub std_primitive_i32_as_postgresql_int: std::option::Option<
        postgresql_crud::Value<
            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
        >,
    >,
}
impl std::convert::From<Dog> for DogOptions {
    fn from(value: Dog) -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: Some(
                postgresql_crud::Value {
                    value: postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                        value
                            .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                            .0,
                    ),
                },
            ),
            std_primitive_bool_as_postgresql_bool: Some(postgresql_crud::Value {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                        value.std_primitive_bool_as_postgresql_bool.0,
                    ),
            }),
            std_primitive_i16_as_postgresql_small_int: Some(postgresql_crud::Value {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                        value.std_primitive_i16_as_postgresql_small_int.0,
                    ),
            }),
            std_primitive_i32_as_postgresql_int: Some(postgresql_crud::Value {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                        value.std_primitive_i32_as_postgresql_int.0,
                    ),
            }),
        }
    }
}
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    enum_extension ::
EnumExtension,
    strum_macros :: EnumIter,
    PartialEq,
    Eq,
    from_str :: FromStr,
)]
pub enum DogColumn {
    #[serde(rename(
        serialize = "std_primitive_bool_as_postgresql_bool",
        deserialize = "std_primitive_bool_as_postgresql_bool"
    ))]
    StdPrimitiveBoolAsPostgresqlBool,
    #[serde(rename(
        serialize = "std_primitive_i16_as_postgresql_small_int",
        deserialize = "std_primitive_i16_as_postgresql_small_int"
    ))]
    StdPrimitiveI16AsPostgresqlSmallInt,
    #[serde(rename(
        serialize = "std_primitive_i32_as_postgresql_int",
        deserialize = "std_primitive_i32_as_postgresql_int"
    ))]
    StdPrimitiveI32AsPostgresqlInt,
    #[serde(rename(
        serialize = "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
        deserialize = "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"
    ))]
    StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
}
impl std::fmt::Display for DogColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::StdPrimitiveBoolAsPostgresqlBool => {
                write!(f, "std_primitive_bool_as_postgresql_bool")
            }
            Self::StdPrimitiveI16AsPostgresqlSmallInt => {
                write!(f, "std_primitive_i16_as_postgresql_small_int")
            }
            Self::StdPrimitiveI32AsPostgresqlInt => {
                write!(f, "std_primitive_i32_as_postgresql_int")
            }
            Self::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => write!(
                f,
                "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"
            ),
        }
    }
}
//HERE
fn generate_query_vec_column(value: &std::vec::Vec<DogColumn>) -> std::string::String {
    let mut value = value
        .iter()
        .fold(std::string::String::from(""), |mut acc, element| {
            acc += match element {
                DogColumn::StdPrimitiveBoolAsPostgresqlBool => "std_primitive_bool_as_postgresql_bool",
                DogColumn::StdPrimitiveI16AsPostgresqlSmallInt => "std_primitive_i16_as_postgresql_small_int",
                DogColumn::StdPrimitiveI32AsPostgresqlInt => "std_primitive_i32_as_postgresql_int",
                DogColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
            };
            acc += ",";
            acc
        });
    value.pop();
    value
}
// impl postgresql_crud::GenerateQuery for DogColumn {
//     fn generate_query(&self) -> std::string::String {
//         match self {
//             Self::StdPrimitiveBoolAsPostgresqlBool => {
//                 std::string::String::from("std_primitive_bool_as_postgresql_bool")
//             }
//             Self::StdPrimitiveI16AsPostgresqlSmallInt => {
//                 std::string::String::from("std_primitive_i16_as_postgresql_small_int")
//             }
//             Self::StdPrimitiveI32AsPostgresqlInt => {
//                 std::string::String::from("std_primitive_i32_as_postgresql_int")
//             }
//             Self::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => {
//                 std::string::String::from(
//                     "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
//                 )
//             }
//         }
//     }
// }
// impl postgresql_crud::GenerateQuery for std::vec::Vec<DogColumn> {
//     fn generate_query(&self) -> std::string::String {
//         let mut value = self
//             .iter()
//             .fold(std::string::String::from(""), |mut acc, element| {
//                 acc += &postgresql_crud::GenerateQuery::generate_query(element);
//                 acc += ",";
//                 acc
//             });
//         value.pop();
//         value
//     }
// }
#[derive(Debug)]
struct WrapperVecColumn(std::vec::Vec<DogColumn>);
impl WrapperVecColumn {
    fn options_try_from_sqlx_row<'a, R: sqlx::Row>(&self, row: &'a R) -> sqlx::Result<DogOptions>
    where
        &'a std::primitive::str: sqlx::ColumnIndex<R>,
        std::option::Option<std::primitive::i64>: sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<std::primitive::i64>: sqlx::types::Type<R::Database>,
        std::option::Option<std::option::Option<std::primitive::bool>>:
            sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<std::option::Option<std::primitive::bool>>:
            sqlx::types::Type<R::Database>,
        std::option::Option<std::option::Option<std::primitive::i16>>:
            sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<std::option::Option<std::primitive::i16>>:
            sqlx::types::Type<R::Database>,
        std::option::Option<std::option::Option<std::primitive::i32>>:
            sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<std::option::Option<std::primitive::i32>>:
            sqlx::types::Type<R::Database>,
    {
        let mut
        std_primitive_i64_as_postgresql_big_serial_not_null_primary_key : std
        :: option :: Option < postgresql_crud :: Value <
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize > > = None ;
        let mut std_primitive_bool_as_postgresql_bool: std::option::Option<
            postgresql_crud::Value<
                postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
            >,
        > = None;
        let mut std_primitive_i16_as_postgresql_small_int: std::option::Option<
            postgresql_crud::Value<
                postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
            >,
        > = None;
        let mut std_primitive_i32_as_postgresql_int: std::option::Option<
            postgresql_crud::Value<
                postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
            >,
        > = None;
        for element in &self.0 {
            match element {
                DogColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => {
                    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = {
                        let value: std::option::Option<std::primitive::i64> = row.try_get(
                            "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                        )?;
                        value.map(|value| postgresql_crud::Value {
                            value: postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                                postgresql_crud::StdPrimitiveI64(value),
                            ),
                        })
                    };
                }
                DogColumn::StdPrimitiveBoolAsPostgresqlBool => {
                    std_primitive_bool_as_postgresql_bool = {
                        let value: std::option::Option<std::option::Option<std::primitive::bool>> =
                            row.try_get("std_primitive_bool_as_postgresql_bool")?;
                        value.map(|value| {
                            postgresql_crud :: Value
                        {
                            value :
                            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
                            ::
                            from(postgresql_crud::StdOptionOptionStdPrimitiveBool(value))
                        }
                        })
                    };
                }
                DogColumn::StdPrimitiveI16AsPostgresqlSmallInt => {
                    std_primitive_i16_as_postgresql_small_int = {
                        let value: std::option::Option<std::option::Option<std::primitive::i16>> =
                            row.try_get("std_primitive_i16_as_postgresql_small_int")?;
                        value.map(|value| {
                            postgresql_crud :: Value
                        {
                            value :
                            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize
                            ::
                            from(postgresql_crud::StdOptionOptionStdPrimitiveI16(value))
                        }
                        })
                    };
                }
                DogColumn::StdPrimitiveI32AsPostgresqlInt => {
                    std_primitive_i32_as_postgresql_int = {
                        let value: std::option::Option<std::option::Option<std::primitive::i32>> =
                            row.try_get("std_primitive_i32_as_postgresql_int")?;
                        value.map(|value| {
                            postgresql_crud :: Value
                        {
                            value :
                            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize
                            ::
                            from(postgresql_crud::StdOptionOptionStdPrimitiveI32(value))
                        }
                        })
                    };
                }
            }
        }
        Ok(DogOptions {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        })
    }
}
fn primary_key_try_from_sqlx_row<'a, R: sqlx::Row>(
    row: &'a R,
) -> sqlx::Result<postgresql_crud::StdPrimitiveI64>
where
    &'a std::primitive::str: sqlx::ColumnIndex<R>,
    std::primitive::i64: sqlx::decode::Decode<'a, R::Database>,
    std::primitive::i64: sqlx::types::Type<R::Database>,
{
    let primary_key: std::primitive::i64 =
        row.try_get("std_primitive_i64_as_postgresql_big_serial_not_null_primary_key")?;
    Ok(postgresql_crud::StdPrimitiveI64(primary_key))
}
fn deserialize_dog_order_by<'de, D>(
    deserializer: D,
) -> Result<crate::server::postgres::order_by::OrderBy<DogColumn>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let string_deserialized = {
        use serde::Deserialize;
        std::string::String::deserialize(deserializer)?
    };
    let split_inner_url_parameters_symbol = ',';
    let default_message = format!("Invalid DogOrderBy:");
    let column_equal_str = "column=";
    let order_equal_str = "order=";
    let column = match string_deserialized.find(column_equal_str) {
        Some(index) => match index.checked_add(column_equal_str.len()) {
            Some(offset) => match string_deserialized.get(offset..) {
                Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                    Some(offset_slice_next_comma_index) => {
                        match offset_slice.get(0..offset_slice_next_comma_index) {
                            Some(possible_column) => match {
                                use std::str::FromStr;
                                DogColumn::from_str(possible_column)
                            } {
                                Ok(column) => column,
                                Err(e) => {
                                    return Err(serde::de::Error::custom(&format!(
                                        "{default_message} {column_equal_str} {}",
                                        e
                                    )));
                                }
                            },
                            None => {
                                return
                                Err(serde :: de :: Error ::
                                custom(& format!
                                ("{default_message} {column_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)")))
                                ;
                            }
                        }
                    }
                    None => match offset_slice.get(0..) {
                        Some(possible_column) => match {
                            use std::str::FromStr;
                            DogColumn::from_str(possible_column)
                        } {
                            Ok(column) => column,
                            Err(e) => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {column_equal_str} {}",
                                    e
                                )));
                            }
                        },
                        None => {
                            return
                            Err(serde :: de :: Error ::
                            custom(& format!
                            ("{default_message} {column_equal_str} failed to offset_slice.get(0..)")))
                            ;
                        }
                    },
                },
                None => {
                    return
                    Err(serde :: de :: Error ::
                    custom(& format!
                    ("{default_message} {column_equal_str} failed to string_deserialized.get(offset..)")))
                    ;
                }
            },
            None => {
                return Err(serde::de::Error::custom(&format!(
                    "{default_message} {column_equal_str} index overflow"
                )));
            }
        },
        None => {
            return Err(serde::de::Error::custom(&format!(
                "{default_message} {column_equal_str} not found"
            )));
        }
    };
    let order = match string_deserialized.find(order_equal_str) {
        Some(index) => match index.checked_add(order_equal_str.len()) {
            Some(offset) => match string_deserialized.get(offset..) {
                Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                    Some(offset_slice_next_comma_index) => {
                        match offset_slice.get(0..offset_slice_next_comma_index) {
                            Some(possible_order) => match {
                                use std::str::FromStr;
                                crate::server::postgres::order::Order::from_str(possible_order)
                            } {
                                Ok(order) => Some(order),
                                Err(e) => {
                                    return Err(serde::de::Error::custom(&format!(
                                        "{default_message} {order_equal_str} {}",
                                        e
                                    )));
                                }
                            },
                            None => {
                                return
                                Err(serde :: de :: Error ::
                                custom(& format!
                                ("{default_message} {order_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)")))
                                ;
                            }
                        }
                    }
                    None => match offset_slice.get(0..) {
                        Some(possible_order) => match {
                            use std::str::FromStr;
                            crate::server::postgres::order::Order::from_str(possible_order)
                        } {
                            Ok(order) => Some(order),
                            Err(e) => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {order_equal_str} {}",
                                    e
                                )));
                            }
                        },
                        None => {
                            return Err(serde::de::Error::custom(
                                &format!
                            ("{default_message} {order_equal_str} failed to offset_slice.get(0..)"),
                            ));
                        }
                    },
                },
                None => {
                    return
                    Err(serde :: de :: Error ::
                    custom(& format!
                    ("{default_message} {order_equal_str} failed to string_deserialized.get(offset..)")))
                    ;
                }
            },
            None => {
                return Err(serde::de::Error::custom(&format!(
                    "{default_message} {order_equal_str} index overflow"
                )));
            }
        },
        None => None,
    };
    Ok(crate::server::postgres::order_by::OrderBy { column, order })
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DogOrderByWrapper(
    #[serde(deserialize_with = "deserialize_dog_order_by")]
    pub  crate::server::postgres::order_by::OrderBy<DogColumn>,
);
impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for DogOrderByWrapper {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        let column = &self.0.column;
        let order = self.0.order.unwrap_or_default();
        format!("column={column},order={order}")
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DogOrderByWrapperFromStrErrorNamed {
    ColumnFromStr {
        #[eo_display_with_serialize_deserialize]
        column_from_str: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNoOffsetValue {
        #[eo_display_with_serialize_deserialize]
        column_no_offset_value: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnOffsetSliceGet {
        #[eo_display_with_serialize_deserialize]
        column_offset_slice_get: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnStringDeserializedGet {
        #[eo_display_with_serialize_deserialize]
        column_string_deserialized_get: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexCheckedAdd {
        #[eo_display_with_serialize_deserialize]
        column_index_checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnStringDeserializedFind {
        #[eo_display_with_serialize_deserialize]
        column_string_deserialized_find: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OrderFromStr {
        #[eo_display_with_serialize_deserialize]
        order_from_str: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OrderOffsetSliceGetNone {
        #[eo_display_with_serialize_deserialize]
        order_offset_slice_get_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OrderStringDeserializedGetNone {
        #[eo_display_with_serialize_deserialize]
        order_string_deserialized_get_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OrderIndexCheckedAdd {
        #[eo_display_with_serialize_deserialize]
        order_index_checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::str::FromStr for DogOrderByWrapper {
    type Err = DogOrderByWrapperFromStrErrorNamed;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let string_deserialized = value.to_string();
        let split_inner_url_parameters_symbol = ',';
        let default_message = format!("Invalid DogOrderBy:");
        let column_equal_str = "column=";
        let order_equal_str = "order=";
        let column = match string_deserialized.find(column_equal_str) {
            Some(index) => match index.checked_add(column_equal_str.len()) {
                Some(offset) => {
                    match string_deserialized.get(offset..) {
                        Some(offset_slice) => match offset_slice
                            .find(split_inner_url_parameters_symbol)
                        {
                            Some(offset_slice_next_comma_index) => {
                                match offset_slice.get(0..offset_slice_next_comma_index) {
                                    Some(possible_column) => {
                                        match DogColumn::from_str(possible_column) {
                                            Ok(column) => column,
                                            Err(e) => {
                                                return
                                        Err(Self :: Err :: ColumnFromStr
                                        {
                                            column_from_str : e, code_occurence : error_occurence_lib ::
                                            code_occurence :: CodeOccurence ::
                                            new(file! ().to_string(), line! (), column! (),
                                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                            {
                                                file : std :: string :: String ::
                                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line : 950, column : 17,
                                            })),
                                        }) ;
                                            }
                                        }
                                    }
                                    None => {
                                        return
                                    Err(Self :: Err :: ColumnNoOffsetValue
                                    {
                                        column_no_offset_value : std :: string :: String ::
                                        from("no offset value"), code_occurence :
                                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                                        new(file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 956, column : 17,
                                        })),
                                    }) ;
                                    }
                                }
                            }
                            None => match offset_slice.get(0..) {
                                Some(possible_column) => match DogColumn::from_str(possible_column)
                                {
                                    Ok(column) => column,
                                    Err(e) => {
                                        return
                                    Err(Self :: Err :: ColumnFromStr
                                    {
                                        column_from_str : e, code_occurence : error_occurence_lib ::
                                        code_occurence :: CodeOccurence ::
                                        new(file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 962, column : 17,
                                        })),
                                    }) ;
                                    }
                                },
                                None => {
                                    return
                                Err(Self :: Err :: ColumnOffsetSliceGet
                                {
                                    column_offset_slice_get : std :: string :: String ::
                                    from("offset_slice_get"), code_occurence :
                                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 968, column : 17,
                                    })),
                                }) ;
                                }
                            },
                        },
                        None => {
                            return
                        Err(Self :: Err :: ColumnStringDeserializedGet
                        {
                            column_string_deserialized_get : std :: string :: String ::
                            from("string_deserialized_get"), code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_string(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 974, column : 17,
                            })),
                        }) ;
                        }
                    }
                }
                None => {
                    return Err(Self::Err::ColumnIndexCheckedAdd {
                        column_index_checked_add: std::string::String::from("index_checked_add"),
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 980,
                                column: 17,
                            }),
                        ),
                    });
                }
            },
            None => {
                return Err(Self::Err::ColumnStringDeserializedFind {
                    column_string_deserialized_find: std::string::String::from(
                        "string_deserialized_find",
                    ),
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 986,
                            column: 17,
                        }),
                    ),
                });
            }
        };
        let order = match string_deserialized.find(order_equal_str) {
            Some(index) => {
                match index.checked_add(order_equal_str.len()) {
                    Some(offset) => match string_deserialized.get(offset..) {
                        Some(offset_slice) => {
                            match offset_slice.find(split_inner_url_parameters_symbol) {
                                Some(offset_slice_next_comma_index) => {
                                    match offset_slice.get(0..offset_slice_next_comma_index) {
                                        Some(possible_order) => {
                                            match crate::server::postgres::order::Order::from_str(
                                                possible_order,
                                            ) {
                                                Ok(order) => Some(order),
                                                Err(e) => {
                                                    return
                                        Err(Self :: Err :: OrderFromStr
                                        {
                                            order_from_str : e, code_occurence : error_occurence_lib ::
                                            code_occurence :: CodeOccurence ::
                                            new(file! ().to_string(), line! (), column! (),
                                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                            {
                                                file : std :: string :: String ::
                                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line : 992, column : 17,
                                            })),
                                        }) ;
                                                }
                                            }
                                        }
                                        None => {
                                            return
                                    Err(Self :: Err :: OrderOffsetSliceGetNone
                                    {
                                        order_offset_slice_get_none : std :: string :: String ::
                                        from("order_offset_slice_get_none"), code_occurence :
                                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                                        new(file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 998, column : 17,
                                        })),
                                    }) ;
                                        }
                                    }
                                }
                                None => match offset_slice.get(0..) {
                                    Some(possible_order) => {
                                        match crate::server::postgres::order::Order::from_str(
                                            possible_order,
                                        ) {
                                            Ok(order) => Some(order),
                                            Err(e) => {
                                                return
                                    Err(Self :: Err :: OrderFromStr
                                    {
                                        order_from_str : e, code_occurence : error_occurence_lib ::
                                        code_occurence :: CodeOccurence ::
                                        new(file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 1004, column : 17,
                                        }))
                                    }) ;
                                            }
                                        }
                                    }
                                    None => {
                                        return
                                Err(Self :: Err :: OrderOffsetSliceGetNone
                                {
                                    order_offset_slice_get_none : std :: string :: String ::
                                    from("order_offset_slice_get_none"), code_occurence :
                                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1010, column : 17,
                                    })),
                                }) ;
                                    }
                                },
                            }
                        }
                        None => {
                            return
                        Err(Self :: Err :: OrderStringDeserializedGetNone
                        {
                            order_string_deserialized_get_none : std :: string :: String
                            :: from("string_deserialized_get_none"), code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_string(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1016, column : 17,
                            })),
                        }) ;
                        }
                    },
                    None => {
                        return Err(Self::Err::OrderIndexCheckedAdd {
                            order_index_checked_add: std::string::String::from(
                                "order_index_checked_add",
                            ),
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_string(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1022,
                                    column: 17,
                                }),
                            ),
                        });
                    }
                }
            }
            None => None,
        };
        Ok(Self(crate::server::postgres::order_by::OrderBy {
            column,
            order,
        }))
    }
}
pub const ALLOW_METHODS: [http::Method; 4] = [
    http::Method::GET,
    http::Method::POST,
    http::Method::PATCH,
    http::Method::DELETE,
];
pub struct DogColumnReadPermission {
    std_primitive_bool_as_postgresql_bool: std::primitive::bool,
    std_primitive_i16_as_postgresql_small_int: std::primitive::bool,
    std_primitive_i32_as_postgresql_int: std::primitive::bool,
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::primitive::bool,
}
#[test]
fn dog_emulate_crud_api_usage_test() {
    async fn find_out_if_it_works() {
        let api_location = std::string::String::from("http://127.0.0.1:8080");
        let limit = 1000;
        let offset = 0;
        println!("-------trycreate_many start-------");
        let primary_keys = match try_create_many(
            &api_location,
            CreateManyParameters {
                payload: CreateManyPayload(vec![CreateManyPayloadElement {
                    std_primitive_bool_as_postgresql_bool:
                        postgresql_crud::StdPrimitiveBoolAsPostgresqlBool::default(),
                    std_primitive_i16_as_postgresql_small_int:
                        postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt::default(),
                    std_primitive_i32_as_postgresql_int:
                        postgresql_crud::StdPrimitiveI32AsPostgresqlInt::default(),
                }]),
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------trycreate_many end-------");
        println!("-------tryread_many start-------");
        match
        try_read_many(& api_location, ReadManyParameters
        {
            payload : ReadManyPayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : Some(primary_keys.clone()),
                std_primitive_bool_as_postgresql_bool : None,
                std_primitive_i16_as_postgresql_small_int : None,
                std_primitive_i32_as_postgresql_int : None, select :
                DogColumnSelect ::
                StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
                order_by : crate :: server :: postgres :: order_by :: OrderBy
                {
                    column : DogColumn :: Name, order :
                    Some(crate :: server :: postgres :: order :: Order :: Desc),
                }, limit : crate :: server :: postgres :: postgres_bigint ::
                PostgresBigint(limit), offset : crate :: server :: postgres ::
                postgres_bigint :: PostgresBigint(offset),
            }
        },).await
        {
            Ok(value) => { println! ("{value:#?}") ; value }, Err(e) => panic!
            ("{}", e)
        } ;
        println!("-------tryread_many end-------");
        println!("-------tryupdate_many start-------");
        match try_update_many(
            &api_location,
            UpdateManyParameters {
                payload: UpdateManyPayload(
                    primary_keys
                        .clone()
                        .into_iter()
                        .map(|element| UpdateManyPayloadElement {
                            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
                                element,
                            std_primitive_bool_as_postgresql_bool:
                                postgresql_crud::StdPrimitiveBoolAsPostgresqlBool::default(),
                            std_primitive_i16_as_postgresql_small_int:
                                postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt::default(),
                            std_primitive_i32_as_postgresql_int:
                                postgresql_crud::StdPrimitiveI32AsPostgresqlInt::default(),
                        })
                        .collect(),
                ),
            },
        )
        .await
        {
            Ok(value) => println!("{value:#?}"),
            Err(e) => panic!("{}", e),
        }
        println!("-------tryupdate_many end-------");
        println!("-------tryread_many start-------");
        match
        try_read_many(& api_location, ReadManyParameters
        {
            payload : ReadManyPayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : Some(primary_keys.clone()),
                std_primitive_bool_as_postgresql_bool : None,
                std_primitive_i16_as_postgresql_small_int : None,
                std_primitive_i32_as_postgresql_int : None, select :
                DogColumnSelect ::
                StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
                order_by : crate :: server :: postgres :: order_by :: OrderBy
                {
                    column : DogColumn :: Name, order :
                    Some(crate :: server :: postgres :: order :: Order :: Desc),
                }, limit : crate :: server :: postgres :: postgres_bigint ::
                PostgresBigint(limit), offset : crate :: server :: postgres ::
                postgres_bigint :: PostgresBigint(offset),
            }
        },).await
        {
            Ok(value) => { println! ("{value:#?}") ; value }, Err(e) => panic!
            ("{}", e)
        } ;
        println!("-------tryread_many end-------");
        println!("-------trydelete_many start-------");
        match try_delete_many(
            &api_location,
            DeleteManyParameters {
                payload: DeleteManyPayload {
                    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: Some(
                        primary_keys.clone(),
                    ),
                    std_primitive_bool_as_postgresql_bool: None,
                    std_primitive_i16_as_postgresql_small_int: None,
                    std_primitive_i32_as_postgresql_int: None,
                },
            },
        )
        .await
        {
            Ok(value) => println!("{value:#?}"),
            Err(e) => panic!("{}", e),
        }
        println!("-------trydelete_many end-------");
        println!("-------tryread_many start-------");
        match
        try_read_many(& api_location, ReadManyParameters
        {
            payload : ReadManyPayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : Some(primary_keys.clone()),
                std_primitive_bool_as_postgresql_bool : None,
                std_primitive_i16_as_postgresql_small_int : None,
                std_primitive_i32_as_postgresql_int : None, select :
                DogColumnSelect ::
                StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
                order_by : crate :: server :: postgres :: order_by :: OrderBy
                {
                    column : DogColumn :: Name, order :
                    Some(crate :: server :: postgres :: order :: Order :: Desc),
                }, limit : crate :: server :: postgres :: postgres_bigint ::
                PostgresBigint(limit), offset : crate :: server :: postgres ::
                postgres_bigint :: PostgresBigint(offset),
            }
        },).await
        {
            Ok(value) => { println! ("{value:#?}") ; value }, Err(e) => panic!
            ("{}", e)
        } ;
        println!("-------tryread_many end-------");
        println!("-------trycreate_one start-------");
        let primary_key = match try_create_one(
            &api_location,
            CreateOneParameters {
                payload: CreateOnePayload {
                    std_primitive_bool_as_postgresql_bool:
                        postgresql_crud::StdPrimitiveBoolAsPostgresqlBool::default(),
                    std_primitive_i16_as_postgresql_small_int:
                        postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt::default(),
                    std_primitive_i32_as_postgresql_int:
                        postgresql_crud::StdPrimitiveI32AsPostgresqlInt::default(),
                },
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------trycreate_one end-------");
        println!("-------tryread_one start-------");
        match
        try_read_one(& api_location, ReadOneParameters
        {
            payload : ReadOnePayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : primary_key.clone(), select : DogColumnSelect ::
                StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
            }
        },).await
        { Ok(value) => println! ("{value:#?}"), Err(e) => panic! ("{}", e) } ;
        println!("-------tryread_one end-------");
        println!("-------tryupdate_one start-------");
        let primary_key = match try_update_one(
            &api_location,
            UpdateOneParameters {
                payload: UpdateOnePayload {
                    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: primary_key
                        .clone(),
                    std_primitive_bool_as_postgresql_bool: Some(
                        postgresql_crud::StdPrimitiveBoolAsPostgresqlBool::default(),
                    ),
                    std_primitive_i16_as_postgresql_small_int: Some(
                        postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt::default(),
                    ),
                    std_primitive_i32_as_postgresql_int: Some(
                        postgresql_crud::StdPrimitiveI32AsPostgresqlInt::default(),
                    ),
                },
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------tryupdate_one end-------");
        println!("-------tryread_one start-------");
        match
        try_read_one(& api_location, ReadOneParameters
        {
            payload : ReadOnePayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : primary_key.clone(), select : DogColumnSelect ::
                StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
            }
        },).await
        { Ok(value) => println! ("{value:#?}"), Err(e) => panic! ("{}", e) } ;
        println!("-------tryread_one end-------");
        println!("-------trydelete_one start-------");
        match try_delete_one(
            &api_location,
            DeleteOneParameters {
                payload: DeleteOnePayload {
                    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: primary_key
                        .clone(),
                },
            },
        )
        .await
        {
            Ok(value) => println!("{value:#?}"),
            Err(e) => panic!("{}", e),
        }
        println!("-------trydelete_one end-------");
        println!("-------tryread_one start-------");
        match
        try_read_one(& api_location, ReadOneParameters
        {
            payload : ReadOnePayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : primary_key.clone(), select : DogColumnSelect ::
                StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
            }
        },).await
        { Ok(value) => panic! ("{value:#?}"), Err(e) => println! ("{}", e) } ;
        println!("-------tryread_one end-------");
    }
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(e) => {
            panic!
            ("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {:#?}",
            e)
        }
        Ok(runtime) => {
            runtime.block_on(find_out_if_it_works());
        }
    }
}
#[derive(Debug)]
pub struct CreateManyPayloadElement {
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdOptionOptionStdPrimitiveBool,
    pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdOptionOptionStdPrimitiveI16,
    pub std_primitive_i32_as_postgresql_int: postgresql_crud::StdOptionOptionStdPrimitiveI32,
}
#[derive(Debug)]
pub struct CreateManyPayload(pub std::vec::Vec<CreateManyPayloadElement>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElementWithSerializeDeserialize {
    pub std_primitive_bool_as_postgresql_bool:
        postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
    pub std_primitive_i16_as_postgresql_small_int:
        postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
    pub std_primitive_i32_as_postgresql_int:
        postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct CreateManyPayloadWithSerializeDeserialize(
    std::vec::Vec<CreateManyPayloadElementWithSerializeDeserialize>,
);
impl std::convert::From<CreateManyPayloadElementWithSerializeDeserialize>
    for CreateManyPayloadElement
{
    fn from(value: CreateManyPayloadElementWithSerializeDeserialize) -> Self {
        let std_primitive_bool_as_postgresql_bool =
            postgresql_crud::StdOptionOptionStdPrimitiveBool::from(
                value.std_primitive_bool_as_postgresql_bool,
            );
        let std_primitive_i16_as_postgresql_small_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI16::from(
                value.std_primitive_i16_as_postgresql_small_int,
            );
        let std_primitive_i32_as_postgresql_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI32::from(
                value.std_primitive_i32_as_postgresql_int,
            );
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
        }
    }
}
impl std::convert::From<CreateManyPayloadWithSerializeDeserialize> for CreateManyPayload {
    fn from(value: CreateManyPayloadWithSerializeDeserialize) -> Self {
        let mut elements = std::vec::Vec::with_capacity(value.0.len());
        for element in value.0 {
            elements.push(CreateManyPayloadElement::from(element));
        }
        Self(elements)
    }
}
impl std::convert::From<CreateManyPayloadElement>
    for CreateManyPayloadElementWithSerializeDeserialize
{
    fn from(value: CreateManyPayloadElement) -> Self {
        let std_primitive_bool_as_postgresql_bool =
            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                value.std_primitive_bool_as_postgresql_bool,
            );
        let std_primitive_i16_as_postgresql_small_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                value.std_primitive_i16_as_postgresql_small_int,
            );
        let std_primitive_i32_as_postgresql_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                value.std_primitive_i32_as_postgresql_int,
            );
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
        }
    }
}
impl std::convert::From<CreateManyPayload> for CreateManyPayloadWithSerializeDeserialize {
    fn from(value: CreateManyPayload) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|element| CreateManyPayloadElementWithSerializeDeserialize::from(element))
                .collect::<std::vec::Vec<CreateManyPayloadElementWithSerializeDeserialize>>(),
        )
    }
}
#[derive(Debug)]
pub struct CreateManyParameters {
    pub payload: CreateManyPayload,
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryCreateMany {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_display]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateManyResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>),
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateMany> for TryCreateManyResponseVariants {
    fn from(value: TryCreateMany) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Io { io, code_occurence }
            => Self :: Io { io, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryCreateManyWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryCreateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryCreateManyWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateManyWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryCreateManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryCreateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryCreateManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryCreateManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryCreateManyWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryCreateManyResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryCreateManyResponseVariants) -> Self {
        match value
        {
            TryCreateManyResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Configuration { configuration : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Database { database : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            TypeNotFound { type_not_found : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            WorkerCrashed { worker_crashed : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Migrate { migrate : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            JsonDataError { json_data_error : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            BindQuery { bind_query : _, code_occurence : _ } => axum :: http
            :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr201Created {
    Desirable(std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>),
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr201Created>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr201Created) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr201Created::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr404NotFound>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr408RequestTimeout>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr500InternalServerError>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr400BadRequest {
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr400BadRequest>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr400BadRequest) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr400BadRequest::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            TryCreateManyResponseVariantsTvfrr400BadRequest::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            TryCreateManyResponseVariantsTvfrr400BadRequest::JsonDataError {
                json_data_error,
                code_occurence,
            } => Self::JsonDataError {
                json_data_error,
                code_occurence,
            },
            TryCreateManyResponseVariantsTvfrr400BadRequest::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            } => Self::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            },
            TryCreateManyResponseVariantsTvfrr400BadRequest::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            } => Self::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            },
            TryCreateManyResponseVariantsTvfrr400BadRequest::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            } => Self::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            },
            TryCreateManyResponseVariantsTvfrr400BadRequest::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            } => Self::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            },
            TryCreateManyResponseVariantsTvfrr400BadRequest::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            } => Self::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            },
        }
    }
}
impl TryFrom<TryCreateManyResponseVariants>
    for std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>
{
    type Error = TryCreateManyWithSerializeDeserialize;
    fn try_from(value: TryCreateManyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryCreateManyResponseVariants :: Desirable(i) => Ok(i),
            TryCreateManyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryCreateManyResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Database
            { database, code_occurence }), TryCreateManyResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryCreateManyResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryCreateManyResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryCreateManyResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryCreateManyResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryCreateManyResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryCreateManyResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryCreateManyResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryCreateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryCreateManyResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryCreateManyResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryCreateManyResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryCreateManyResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryCreateManyResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryCreateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryCreateManyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }),
            TryCreateManyResponseVariants :: UnexpectedCase
            { unexpected_case, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }),
            TryCreateManyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryCreateManyResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }), TryCreateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryCreateManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryCreateManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            NoCommitExtractorHeader { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub enum TryCreateManyStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    BindQueryTvfrr500InternalServerError,
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServerTvfrr500InternalServerError,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryCreateManyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryCreateManyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            } TryCreateManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many<'a>(
    server_location: &str,
    parameters: CreateManyParameters,
) -> Result<std::vec::Vec<postgresql_crud::StdPrimitiveI64>, TryCreateManyErrorNamed> {
    let payload = match serde_json::to_string(&CreateManyPayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryCreateManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1266,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/create_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(postgresql_crud::COMMIT, git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryCreateManyErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2142,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            return Err(TryCreateManyErrorNamed::FailedToGetResponseText {
                reqwest: e,
                status_code,
                headers,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2071,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::CREATED {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr201Created>(&response_text) {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr400BadRequest>(
            &response_text,
        ) {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr408RequestTimeout>(
            &response_text,
        ) {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr404NotFound>(&response_text)
        {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        return Err(TryCreateManyErrorNamed::UnexpectedStatusCode {
            status_code,
            headers,
            response_text_result:
                crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(
                    response_text,
                ),
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2036,
                    column: 13,
                }),
            ),
        });
    };
    match std::vec::Vec::<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>::try_from(
        variants,
    ) {
        Ok(value) => Ok(value
            .into_iter()
            .map(|element| postgresql_crud::StdPrimitiveI64::from(element))
            .collect()),
        Err(e) => {
            return Err(TryCreateManyErrorNamed::ExpectedType {
                expected_type: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1998,
                        column: 13,
                    }),
                ),
            });
        }
    }
}
#[utoipa ::
path(post, path = "/dogs/create_many", operation_id = "/dogs/create_many", tag
= "dogs",
request_body(content = CreateManyPayload, description =
"dogs create_many payload", content_type = "application/json"),
responses((status = 201, description = "created", body =
TryCreateManyResponseVariantsTvfrr201Created, content_type =
"application/json"),
(status = 500, description = "internal server error", body =
TryCreateManyResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryCreateManyResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryCreateManyResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryCreateManyResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn create_many(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<CreateManyPayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = CreateManyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                CreateManyPayloadWithSerializeDeserialize,
                TryCreateManyResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => CreateManyPayload::from(value),
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        let query_string = {
            "insert into dogs (std_primitive_bool_as_postgresql_bool, std_primitive_i16_as_postgresql_small_int, std_primitive_i32_as_postgresql_int) select std_primitive_bool_as_postgresql_bool, std_primitive_i16_as_postgresql_small_int, std_primitive_i32_as_postgresql_int from unnest($1, $2, $3) as a(std_primitive_bool_as_postgresql_bool, std_primitive_i16_as_postgresql_small_int, std_primitive_i32_as_postgresql_int) returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let current_vec_len = parameters.payload.0.len();
            let (
                std_primitive_bool_as_postgresql_bool_vec,
                std_primitive_i16_as_postgresql_small_int_vec,
                std_primitive_i32_as_postgresql_int_vec,
            ) = parameters.payload.0.into_iter().fold(
                (
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                ),
                |mut acc, element| {
                    acc.0.push(element.std_primitive_bool_as_postgresql_bool);
                    acc.1
                        .push(element.std_primitive_i16_as_postgresql_small_int);
                    acc.2.push(element.std_primitive_i32_as_postgresql_int);
                    acc
                },
            );
            query = query.bind(
                postgresql_crud::StdOptionOptionStdPrimitiveBool::into_inner_type_vec(
                    std_primitive_bool_as_postgresql_bool_vec,
                ),
            );
            query = query.bind(
                postgresql_crud::StdOptionOptionStdPrimitiveI16::into_inner_type_vec(
                    std_primitive_i16_as_postgresql_small_int_vec,
                ),
            );
            query = query.bind(
                postgresql_crud::StdOptionOptionStdPrimitiveI32::into_inner_type_vec(
                    std_primitive_i32_as_postgresql_int_vec,
                ),
            );
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let e = TryCreateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryCreateManyResponseVariants::from(e);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let e = TryCreateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryCreateManyResponseVariants::from(e);
            }
        };
        let mut rows = binded_query.fetch(pg_connection.as_mut());
        let mut vec_values = std::vec::Vec::new();
        while let Some(row) = {
            match {
                use futures::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => value,
                Err(e) => {
                    let e = TryCreateMany::from(e);
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryCreateManyResponseVariants::from(e);
                }
            }
        } {
            match {
                use sqlx::Row;
                row.try_get::<std::primitive::i64, &str>(
                    "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                )
            } {
                Ok(value) => {
                    vec_values.push(
                        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                            postgresql_crud::StdPrimitiveI64(value),
                        ),
                    );
                }
                Err(e) => {
                    let e = TryCreateMany ::
                    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
                    {
                        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1732, column : 13,
                        })),
                    } ;
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryCreateManyResponseVariants::from(e);
                }
            }
        }
        TryCreateManyResponseVariants::Desirable(vec_values)
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryCreateMany
{
    fn from(
        value: crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, utoipa :: ToSchema)]
pub struct CreateOnePayload {
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdOptionOptionStdPrimitiveBool,
    pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdOptionOptionStdPrimitiveI16,
    pub std_primitive_i32_as_postgresql_int: postgresql_crud::StdOptionOptionStdPrimitiveI32,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateOnePayloadWithSerializeDeserialize {
    pub std_primitive_bool_as_postgresql_bool:
        postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
    pub std_primitive_i16_as_postgresql_small_int:
        postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
    pub std_primitive_i32_as_postgresql_int:
        postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
}
impl std::convert::From<CreateOnePayloadWithSerializeDeserialize> for CreateOnePayload {
    fn from(value: CreateOnePayloadWithSerializeDeserialize) -> Self {
        let std_primitive_bool_as_postgresql_bool =
            postgresql_crud::StdOptionOptionStdPrimitiveBool::from(
                value.std_primitive_bool_as_postgresql_bool,
            );
        let std_primitive_i16_as_postgresql_small_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI16::from(
                value.std_primitive_i16_as_postgresql_small_int,
            );
        let std_primitive_i32_as_postgresql_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI32::from(
                value.std_primitive_i32_as_postgresql_int,
            );
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
        }
    }
}
impl std::convert::From<CreateOnePayload> for CreateOnePayloadWithSerializeDeserialize {
    fn from(value: CreateOnePayload) -> Self {
        let std_primitive_bool_as_postgresql_bool =
            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                value.std_primitive_bool_as_postgresql_bool,
            );
        let std_primitive_i16_as_postgresql_small_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                value.std_primitive_i16_as_postgresql_small_int,
            );
        let std_primitive_i32_as_postgresql_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                value.std_primitive_i32_as_postgresql_int,
            );
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
        }
    }
}
#[derive(Debug)]
pub struct CreateOneParameters {
    pub payload: CreateOnePayload,
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryCreateOne {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_display]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateOneResponseVariants {
    Desirable(postgresql_crud::StdPrimitiveI64WithSerializeDeserialize),
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOne> for TryCreateOneResponseVariants {
    fn from(value: TryCreateOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryCreateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryCreateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryCreateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryCreateOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryCreateOneWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryCreateOneWithSerializeDeserialize :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryCreateOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryCreateOneWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryCreateOneResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryCreateOneResponseVariants) -> Self {
        match value
        {
            TryCreateOneResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            Configuration { configuration : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            Database { database : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            TypeNotFound { type_not_found : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: Decode
            { decode : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            WorkerCrashed { worker_crashed : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            Migrate { migrate : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            JsonDataError { json_data_error : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateOneResponseVariantsTvfrr201Created {
    Desirable(postgresql_crud::StdPrimitiveI64WithSerializeDeserialize),
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr201Created>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr201Created) -> Self {
        match value {
            TryCreateOneResponseVariantsTvfrr201Created::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr408RequestTimeout>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryCreateOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateOneResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr500InternalServerError>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr404NotFound>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryCreateOneResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateOneResponseVariantsTvfrr400BadRequest {
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr400BadRequest>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value {
            TryCreateOneResponseVariantsTvfrr400BadRequest::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            TryCreateOneResponseVariantsTvfrr400BadRequest::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            TryCreateOneResponseVariantsTvfrr400BadRequest::JsonDataError {
                json_data_error,
                code_occurence,
            } => Self::JsonDataError {
                json_data_error,
                code_occurence,
            },
            TryCreateOneResponseVariantsTvfrr400BadRequest::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            } => Self::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            },
            TryCreateOneResponseVariantsTvfrr400BadRequest::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            } => Self::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            },
            TryCreateOneResponseVariantsTvfrr400BadRequest::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            } => Self::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            },
            TryCreateOneResponseVariantsTvfrr400BadRequest::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            } => Self::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            },
            TryCreateOneResponseVariantsTvfrr400BadRequest::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            } => Self::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            },
        }
    }
}
impl TryFrom<TryCreateOneResponseVariants>
    for postgresql_crud::StdPrimitiveI64WithSerializeDeserialize
{
    type Error = TryCreateOneWithSerializeDeserialize;
    fn try_from(value: TryCreateOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryCreateOneResponseVariants :: Desirable(i) => Ok(i),
            TryCreateOneResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryCreateOneResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Database
            { database, code_occurence }), TryCreateOneResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Io
            { io, code_occurence }), TryCreateOneResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryCreateOneResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryCreateOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryCreateOneResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryCreateOneResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryCreateOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryCreateOneResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryCreateOneResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryCreateOneWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryCreateOneResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryCreateOneResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryCreateOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryCreateOneResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryCreateOneResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }), TryCreateOneResponseVariants
            :: JsonSyntaxError { json_syntax_error, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryCreateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryCreateOneResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }), TryCreateOneResponseVariants
            :: UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryCreateOneResponseVariants
            ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }), TryCreateOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryCreateOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryCreateOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            NoCommitExtractorHeader { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateOneWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub enum TryCreateOneStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServerTvfrr500InternalServerError,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryCreateOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryCreateOneResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            } TryCreateOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneErrorNamed {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateOneWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_one<'a>(
    server_location: &str,
    parameters: CreateOneParameters,
) -> Result<postgresql_crud::StdPrimitiveI64, TryCreateOneErrorNamed> {
    let payload = match serde_json::to_string(&CreateOnePayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryCreateOneErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1266,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/create_one", server_location);
    let future = reqwest::Client::new()
        .post(&url)
        .header(postgresql_crud::COMMIT, git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryCreateOneErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2142,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            return Err(TryCreateOneErrorNamed::FailedToGetResponseText {
                reqwest: e,
                status_code,
                headers,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2071,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::CREATED {
        match serde_json::from_str::<TryCreateOneResponseVariantsTvfrr201Created>(&response_text) {
            Ok(value) => TryCreateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryCreateOneResponseVariantsTvfrr404NotFound>(&response_text) {
            Ok(value) => TryCreateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryCreateOneResponseVariantsTvfrr400BadRequest>(&response_text)
        {
            Ok(value) => TryCreateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match serde_json::from_str::<TryCreateOneResponseVariantsTvfrr408RequestTimeout>(
            &response_text,
        ) {
            Ok(value) => TryCreateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        return Err(TryCreateOneErrorNamed::UnexpectedStatusCode {
            status_code,
            headers,
            response_text_result:
                crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(
                    response_text,
                ),
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2036,
                    column: 13,
                }),
            ),
        });
    };
    match postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::try_from(variants) {
        Ok(value) => Ok(postgresql_crud::StdPrimitiveI64::from(value)),
        Err(e) => {
            return Err(TryCreateOneErrorNamed::ExpectedType {
                expected_type: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1998,
                        column: 13,
                    }),
                ),
            });
        }
    }
}
#[utoipa ::
path(post, path = "/dogs/create_one", operation_id = "/dogs/create_one", tag =
"dogs",
request_body(content = CreateOnePayloadWithSerializeDeserialize, description =
"dogs create_one payload", content_type = "application/json"),
responses((status = 201, description = "created", body =
TryCreateOneResponseVariantsTvfrr201Created, content_type =
"application/json"),
(status = 500, description = "internal server error", body =
TryCreateOneResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryCreateOneResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryCreateOneResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryCreateOneResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn create_one(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<CreateOnePayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = CreateOneParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                CreateOnePayloadWithSerializeDeserialize,
                TryCreateOneResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => CreateOnePayload::from(value),
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        let query_string = {
            "insert into dogs(std_primitive_bool_as_postgresql_bool, std_primitive_i16_as_postgresql_small_int, std_primitive_i32_as_postgresql_int) values ($1, $2, $3) returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            query = postgresql_crud::BindQuery::bind_value_to_query(
                parameters.payload.std_primitive_bool_as_postgresql_bool,
                query,
            );
            query = postgresql_crud::BindQuery::bind_value_to_query(
                parameters.payload.std_primitive_i16_as_postgresql_small_int,
                query,
            );
            query = postgresql_crud::BindQuery::bind_value_to_query(
                parameters.payload.std_primitive_i32_as_postgresql_int,
                query,
            );
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let e = TryCreateOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryCreateOneResponseVariants::from(e);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let e = TryCreateOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryCreateOneResponseVariants::from(e);
            }
        };
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(value) => match {
                use sqlx::Row;
                value.try_get::<std::primitive::i64, &str>(
                    "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                )
            } {
                Ok(value) => TryCreateOneResponseVariants::Desirable(
                    postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                        postgresql_crud::StdPrimitiveI64(value),
                    ),
                ),
                Err(e) => {
                    let e = TryCreateOne ::
                    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
                    {
                        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1732, column : 13,
                        })),
                    } ;
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryCreateOneResponseVariants::from(e);
                }
            },
            Err(e) => {
                let e = TryCreateOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryCreateOneResponseVariants::from(e);
            }
        }
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryCreateOne
{
    fn from(
        value: crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, utoipa :: ToSchema)]
pub struct ReadManyPayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        std::option::Option<std::vec::Vec<postgresql_crud::StdPrimitiveI64>>,
    pub std_primitive_bool_as_postgresql_bool:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveBool>>,
    pub std_primitive_i16_as_postgresql_small_int:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16>>,
    pub std_primitive_i32_as_postgresql_int:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32>>,
    pub select: std::vec::Vec<DogColumn>,
    pub order_by: crate::server::postgres::order_by::OrderBy<DogColumn>,
    pub limit: postgresql_crud::StdPrimitiveI64,
    pub offset: postgresql_crud::StdPrimitiveI64,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct ReadManyPayloadWithSerializeDeserialize {
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<
        std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>,
    >,
    std_primitive_bool_as_postgresql_bool: std::option::Option<
        std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
    >,
    std_primitive_i16_as_postgresql_small_int: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize>,
    >,
    std_primitive_i32_as_postgresql_int: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize>,
    >,
    select: std::vec::Vec<DogColumn>,
    order_by: crate::server::postgres::order_by::OrderBy<DogColumn>,
    limit: postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
    offset: postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
}
impl std::convert::From<ReadManyPayloadWithSerializeDeserialize> for ReadManyPayload {
    fn from(value: ReadManyPayloadWithSerializeDeserialize) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = match value
            .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
        {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::StdPrimitiveI64::from(
                            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(element),
                        )
                    })
                    .collect(),
            ),
            None => None,
        };
        let std_primitive_bool_as_postgresql_bool =
            match value.std_primitive_bool_as_postgresql_bool {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveBool::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_i16_as_postgresql_small_int =
            match value.std_primitive_i16_as_postgresql_small_int {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI32::from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        let select = value.select;
        let order_by = value.order_by;
        let limit = postgresql_crud::StdPrimitiveI64::from(value.limit);
        let offset = postgresql_crud::StdPrimitiveI64::from(value.offset);
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            select,
            order_by,
            limit,
            offset,
        }
    }
}
impl std::convert::From<ReadManyPayload> for ReadManyPayloadWithSerializeDeserialize {
    fn from(value: ReadManyPayload) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
        match
        value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize ::
            from(element)).collect :: < std :: vec :: Vec <
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize >> (),),
            None => None,
        } ;
        let std_primitive_bool_as_postgresql_bool = match
        value.std_primitive_bool_as_postgresql_bool
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
            :: from(element)).collect()), None => None
        } ;
        let std_primitive_i16_as_postgresql_small_int = match value
            .std_primitive_i16_as_postgresql_small_int
        {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize
            :: from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize
            :: from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        let select = value.select;
        let order_by = value.order_by;
        let limit = postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(value.limit);
        let offset = postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(value.offset);
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            select,
            order_by,
            limit,
            offset,
        }
    }
}
#[derive(Debug)]
pub struct ReadManyParameters {
    pub payload: ReadManyPayload,
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryReadMany {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_std_primitive_bool_as_postgresql_bool_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_std_primitive_i16_as_postgresql_small_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI32AsPostgresqlIntVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_std_primitive_i32_as_postgresql_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec:
            std::vec::Vec<postgresql_crud::WhereStdPrimitiveI64WithSerializeDeserialize>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKeys {
        #[eo_vec_display]
        not_unique_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadManyResponseVariants {
    Desirable(std::vec::Vec<DogOptions>),
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec {
        not_unique_std_primitive_bool_as_postgresql_bool_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec {
        not_unique_std_primitive_i16_as_postgresql_small_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI32AsPostgresqlIntVec {
        not_unique_std_primitive_i32_as_postgresql_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec {
        not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec:
            std::vec::Vec<postgresql_crud::WhereStdPrimitiveI64WithSerializeDeserialize>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKeys {
        not_unique_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadMany> for TryReadManyResponseVariants {
    fn from(value: TryReadMany) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryReadManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryReadManyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryReadManyWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryReadManyWithSerializeDeserialize :: Tls { tls, code_occurence }
            => Self :: Tls { tls, code_occurence },
            TryReadManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryReadManyWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryReadManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryReadManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryReadManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryReadManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryReadManyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryReadManyWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryReadManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryReadManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryReadManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryReadManyWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryReadManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryReadManyWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryReadManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryReadManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryReadManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            }, TryReadManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            }, TryReadManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            }, TryReadManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            } => Self ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            }, TryReadManyWithSerializeDeserialize :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } => Self ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence },
            TryReadManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryReadManyWithSerializeDeserialize :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryReadManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryReadManyWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryReadManyResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryReadManyResponseVariants) -> Self {
        match value
        {
            TryReadManyResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: OK, TryReadManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadManyResponseVariants :: Database
            { database : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryReadManyResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryReadManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryReadManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryReadManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadManyResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryReadManyResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryReadManyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: OK, TryReadManyResponseVariants
            :: Decode { decode : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryReadManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadManyResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryReadManyResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryReadManyResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryReadManyResponseVariants ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec : _,
                code_occurence : _
            } => axum :: http :: StatusCode :: OK, TryReadManyResponseVariants
            :: NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec : _,
                code_occurence : _
            } => axum :: http :: StatusCode :: OK, TryReadManyResponseVariants
            :: NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec : _,
                code_occurence : _
            } => axum :: http :: StatusCode :: OK, TryReadManyResponseVariants
            ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK, TryReadManyResponseVariants
            :: NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryReadManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadManyResponseVariants ::
            CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryReadManyResponseVariants ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryReadManyResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadManyResponseVariantsTvfrr200Ok {
    Desirable(std::vec::Vec<DogOptions>),
}
impl std::convert::From<TryReadManyResponseVariantsTvfrr200Ok> for TryReadManyResponseVariants {
    fn from(value: TryReadManyResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryReadManyResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadManyResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadManyResponseVariantsTvfrr500InternalServerError>
    for TryReadManyResponseVariants
{
    fn from(value: TryReadManyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value {
            TryReadManyResponseVariantsTvfrr500InternalServerError::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            TryReadManyResponseVariantsTvfrr500InternalServerError::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::BytesRejection {
                bytes_rejection,
                code_occurence,
            } => Self::BytesRejection {
                bytes_rejection,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
            TryReadManyResponseVariantsTvfrr500InternalServerError::BindQuery {
                bind_query,
                code_occurence,
            } => Self::BindQuery {
                bind_query,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadManyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadManyResponseVariantsTvfrr408RequestTimeout>
    for TryReadManyResponseVariants
{
    fn from(value: TryReadManyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryReadManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadManyResponseVariantsTvfrr400BadRequest {
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec {
        not_unique_std_primitive_bool_as_postgresql_bool_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec {
        not_unique_std_primitive_i16_as_postgresql_small_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI32AsPostgresqlIntVec {
        not_unique_std_primitive_i32_as_postgresql_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec {
        not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec:
            std::vec::Vec<postgresql_crud::WhereStdPrimitiveI64WithSerializeDeserialize>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKeys {
        not_unique_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadManyResponseVariantsTvfrr400BadRequest>
    for TryReadManyResponseVariants
{
    fn from(value: TryReadManyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryReadManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryReadManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryReadManyResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryReadManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryReadManyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryReadManyResponseVariantsTvfrr400BadRequest ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            }, TryReadManyResponseVariantsTvfrr400BadRequest ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            }, TryReadManyResponseVariantsTvfrr400BadRequest ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            }, TryReadManyResponseVariantsTvfrr400BadRequest ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            } => Self ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            }, TryReadManyResponseVariantsTvfrr400BadRequest ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }
            => Self :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence },
            TryReadManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryReadManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryReadManyResponseVariantsTvfrr400BadRequest ::
            NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadManyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadManyResponseVariantsTvfrr404NotFound>
    for TryReadManyResponseVariants
{
    fn from(value: TryReadManyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryReadManyResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
impl TryFrom<TryReadManyResponseVariants> for std::vec::Vec<DogOptions> {
    type Error = TryReadManyWithSerializeDeserialize;
    fn try_from(value: TryReadManyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryReadManyResponseVariants :: Desirable(i) => Ok(i),
            TryReadManyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryReadManyResponseVariants ::
            Database { database, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: Database
            { database, code_occurence }), TryReadManyResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryReadManyResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryReadManyResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryReadManyResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryReadManyResponseVariants ::
            TypeNotFound { type_not_found, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryReadManyResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryReadManyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }), TryReadManyResponseVariants
            :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryReadManyResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryReadManyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryReadManyResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryReadManyResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryReadManyResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryReadManyResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryReadManyResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }), TryReadManyResponseVariants
            :: JsonSyntaxError { json_syntax_error, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryReadManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryReadManyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }), TryReadManyResponseVariants
            :: UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryReadManyResponseVariants
            :: NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            } =>
            Err(TryReadManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            }), TryReadManyResponseVariants ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            } =>
            Err(TryReadManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            }), TryReadManyResponseVariants ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            } =>
            Err(TryReadManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            }), TryReadManyResponseVariants ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            } =>
            Err(TryReadManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            }), TryReadManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence }),
            TryReadManyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryReadManyResponseVariants ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryReadManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryReadManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryReadManyWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryReadManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub enum TryReadManyStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolVecTvfrr400BadRequest,
    NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVecTvfrr400BadRequest,
    NotUniqueStdPrimitiveI32AsPostgresqlIntVecTvfrr400BadRequest,
    NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVecTvfrr400BadRequest,
    NotUniquePrimaryKeysTvfrr400BadRequest,
    BindQueryTvfrr500InternalServerError,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryReadManyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryReadManyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            } TryReadManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyErrorNamed {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryReadManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many<'a>(
    server_location: &str,
    parameters: ReadManyParameters,
) -> Result<std::vec::Vec<DogOptions>, TryReadManyErrorNamed> {
    let payload = match serde_json::to_string(&ReadManyPayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryReadManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1266,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/read_many", server_location);
    let future = reqwest::Client::new()
        .post(&url)
        .header(postgresql_crud::COMMIT, git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryReadManyErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2142,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            return Err(TryReadManyErrorNamed::FailedToGetResponseText {
                reqwest: e,
                status_code,
                headers,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2071,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::OK {
        match serde_json::from_str::<TryReadManyResponseVariantsTvfrr200Ok>(&response_text) {
            Ok(value) => TryReadManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryReadManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match serde_json::from_str::<TryReadManyResponseVariantsTvfrr408RequestTimeout>(
            &response_text,
        ) {
            Ok(value) => TryReadManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryReadManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match serde_json::from_str::<TryReadManyResponseVariantsTvfrr500InternalServerError>(
            &response_text,
        ) {
            Ok(value) => TryReadManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryReadManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryReadManyResponseVariantsTvfrr404NotFound>(&response_text) {
            Ok(value) => TryReadManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryReadManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        return Err(TryReadManyErrorNamed::UnexpectedStatusCode {
            status_code,
            headers,
            response_text_result:
                crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(
                    response_text,
                ),
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2036,
                    column: 13,
                }),
            ),
        });
    };
    match std::vec::Vec::<DogOptions>::try_from(variants) {
        Ok(value) => Ok(value),
        Err(e) => {
            return Err(TryReadManyErrorNamed::ExpectedType {
                expected_type: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1998,
                        column: 13,
                    }),
                ),
            });
        }
    }
}
#[utoipa ::
path(post, path = "/dogs/read_many", operation_id = "/dogs/read_many", tag =
"dogs",
request_body(content = ReadManyPayloadWithSerializeDeserialize, description =
"dogs read_many payload", content_type = "application/json"),
responses((status = 200, description = "ok", body =
TryReadManyResponseVariantsTvfrr200Ok, content_type = "application/json"),
(status = 500, description = "internal server error", body =
TryReadManyResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryReadManyResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryReadManyResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryReadManyResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn read_many(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<ReadManyPayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let mut parameters = ReadManyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                ReadManyPayloadWithSerializeDeserialize,
                TryReadManyResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => ReadManyPayload::from(value),
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        if let Some(std_primitive_i64_as_postgresql_big_serial_not_null_primary_key) =
            &mut parameters
                .payload
                .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
        {
            let not_unique_primary_keys = {
                let mut value = vec![];
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.sort_unstable();
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.dedup_by(|a, b| {
                    match a == b {
                        true => {
                            value.push(std::mem::take(a));
                            true
                        }
                        false => false,
                    }
                });
                value
            };
            if !not_unique_primary_keys.is_empty() {
                let e = TryReadMany::NotUniquePrimaryKeys {
                    not_unique_primary_keys,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1659,
                            column: 13,
                        }),
                    ),
                };
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryReadManyResponseVariants::from(e);
            }
        }
        let std_primitive_bool_as_postgresql_bool_handle = match parameters
            .payload
            .std_primitive_bool_as_postgresql_bool
        {
            Some(value) => {
                let is_unique = {
                    let mut vec = std::vec::Vec::with_capacity(value.len());
                    let mut is_unique = true;
                    for element in &value {
                        match vec.contains(&element) {
                            true => {
                                is_unique = false;
                                break;
                            }
                            false => {
                                vec.push(element);
                            }
                        }
                    }
                    is_unique
                };
                match is_unique {
                    true => Some(value),
                    false => {
                        let not_unique_std_primitive_bool_as_postgresql_bool_vec = {
                            let mut vec = std::vec::Vec::with_capacity(value.len());
                            let mut not_unique_std_primitive_bool_as_postgresql_bool_vec =
                                std::vec::Vec::with_capacity(value.len());
                            for element in value {
                                match vec.contains(&element) {
                                    true => {
                                        not_unique_std_primitive_bool_as_postgresql_bool_vec
                                            .push(element);
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            not_unique_std_primitive_bool_as_postgresql_bool_vec.into_iter().map(|
                            element |
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
                            :: from(element)).collect()
                        };
                        let e = TryReadMany::NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec {
                            not_unique_std_primitive_bool_as_postgresql_bool_vec,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_string(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 3928,
                                    column: 29,
                                }),
                            ),
                        };
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryReadManyResponseVariants::from(e);
                    }
                }
            }
            None => None,
        };
        let std_primitive_i16_as_postgresql_small_int_handle = match parameters
            .payload
            .std_primitive_i16_as_postgresql_small_int
        {
            Some(value) => {
                let is_unique = {
                    let mut vec = std::vec::Vec::with_capacity(value.len());
                    let mut is_unique = true;
                    for element in &value {
                        match vec.contains(&element) {
                            true => {
                                is_unique = false;
                                break;
                            }
                            false => {
                                vec.push(element);
                            }
                        }
                    }
                    is_unique
                };
                match is_unique {
                    true => Some(value),
                    false => {
                        let not_unique_std_primitive_i16_as_postgresql_small_int_vec = {
                            let mut vec = std::vec::Vec::with_capacity(value.len());
                            let mut not_unique_std_primitive_i16_as_postgresql_small_int_vec =
                                std::vec::Vec::with_capacity(value.len());
                            for element in value {
                                match vec.contains(&element) {
                                    true => {
                                        not_unique_std_primitive_i16_as_postgresql_small_int_vec
                                            .push(element);
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            not_unique_std_primitive_i16_as_postgresql_small_int_vec.into_iter().map(|
                            element |
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize
                            :: from(element)).collect()
                        };
                        let e = TryReadMany::NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec {
                            not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_string(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 3928,
                                    column: 29,
                                }),
                            ),
                        };
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryReadManyResponseVariants::from(e);
                    }
                }
            }
            None => None,
        };
        let std_primitive_i32_as_postgresql_int_handle = match parameters
            .payload
            .std_primitive_i32_as_postgresql_int
        {
            Some(value) => {
                let is_unique = {
                    let mut vec = std::vec::Vec::with_capacity(value.len());
                    let mut is_unique = true;
                    for element in &value {
                        match vec.contains(&element) {
                            true => {
                                is_unique = false;
                                break;
                            }
                            false => {
                                vec.push(element);
                            }
                        }
                    }
                    is_unique
                };
                match is_unique {
                    true => Some(value),
                    false => {
                        let not_unique_std_primitive_i32_as_postgresql_int_vec = {
                            let mut vec = std::vec::Vec::with_capacity(value.len());
                            let mut not_unique_std_primitive_i32_as_postgresql_int_vec =
                                std::vec::Vec::with_capacity(value.len());
                            for element in value {
                                match vec.contains(&element) {
                                    true => {
                                        not_unique_std_primitive_i32_as_postgresql_int_vec
                                            .push(element);
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            not_unique_std_primitive_i32_as_postgresql_int_vec.into_iter().map(|
                            element |
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize
                            :: from(element)).collect()
                        };
                        let e = TryReadMany::NotUniqueStdPrimitiveI32AsPostgresqlIntVec {
                            not_unique_std_primitive_i32_as_postgresql_int_vec,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_string(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 3928,
                                    column: 29,
                                }),
                            ),
                        };
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryReadManyResponseVariants::from(e);
                    }
                }
            }
            None => None,
        };
        let query_string = {
            format!(
                "select {} from dogs {}",
                //HERE
                generate_query_vec_column(&parameters.payload.select),
                {
                    let mut increment: u64 = 0;
                    let mut additional_parameters = std::string::String::default();
                    if let Some(value) = &parameters
                        .payload
                        .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                    {
                        let prefix = match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        };
                        match increment.checked_add(1) {
                            Some(value) => {
                                increment = value;
                            }
                            None => {
                                let e = postgresql_crud::TryGenerateBindIncrementsErrorNamed
                            :: CheckedAdd
                            {
                                checked_add : std :: string :: String ::
                                from("checked_add is None"), code_occurence :
                                error_occurence_lib :: code_occurence :: CodeOccurence ::
                                new(file! ().to_string(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1535, column : 13,
                                })),
                            } ;
                                return TryReadManyResponseVariants :: BindQuery
                            {
                                bind_query : e.into_serialize_deserialize_version(),
                                code_occurence : error_occurence_lib :: code_occurence ::
                                CodeOccurence ::
                                new(file! ().to_string(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1505, column : 13,
                                }))
                            } ;
                            }
                        }
                        additional_parameters.push_str(& format!
                    ("{} std_primitive_i64_as_postgresql_big_serial_not_null_primary_key in (select unnest(${}))",
                    prefix, increment)) ;
                    }
                    if let Some(value) = &std_primitive_bool_as_postgresql_bool_handle {
                        let prefix = match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        };
                        let bind_increments = {
                            let mut bind_increments = std::string::String::default();
                            for (index, element) in value.iter().enumerate() {
                                match postgresql_crud::BindQuery::try_generate_bind_increments(
                                    element,
                                    &mut increment,
                                ) {
                                    Ok(value) => {
                                        let handle = format!(
                                            "std_primitive_bool_as_postgresql_bool ~ {value} "
                                        );
                                        match index == 0 {
                                            true => {
                                                bind_increments.push_str(&handle);
                                            }
                                            false => {
                                                bind_increments.push_str(&format!(
                                                    "{} {handle}",
                                                    element.conjuctive_operator
                                                ));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        return TryReadManyResponseVariants :: BindQuery
                                    {
                                        bind_query : e.into_serialize_deserialize_version(),
                                        code_occurence : error_occurence_lib :: code_occurence ::
                                        CodeOccurence ::
                                        new(file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 1505, column : 13,
                                        }))
                                    } ;
                                    }
                                }
                            }
                            if let false = bind_increments.is_empty() {
                                bind_increments.pop();
                            }
                            bind_increments
                        };
                        additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
                    }
                    if let Some(value) = &std_primitive_i16_as_postgresql_small_int_handle {
                        let prefix = match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        };
                        let bind_increments = {
                            let mut bind_increments = std::string::String::default();
                            for (index, element) in value.iter().enumerate() {
                                match postgresql_crud::BindQuery::try_generate_bind_increments(
                                    element,
                                    &mut increment,
                                ) {
                                    Ok(value) => {
                                        let handle =
                                            format!
                                    ("std_primitive_i16_as_postgresql_small_int ~ {value} ");
                                        match index == 0 {
                                            true => {
                                                bind_increments.push_str(&handle);
                                            }
                                            false => {
                                                bind_increments.push_str(&format!(
                                                    "{} {handle}",
                                                    element.conjuctive_operator
                                                ));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        return TryReadManyResponseVariants :: BindQuery
                                    {
                                        bind_query : e.into_serialize_deserialize_version(),
                                        code_occurence : error_occurence_lib :: code_occurence ::
                                        CodeOccurence ::
                                        new(file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 1505, column : 13,
                                        }))
                                    } ;
                                    }
                                }
                            }
                            if let false = bind_increments.is_empty() {
                                bind_increments.pop();
                            }
                            bind_increments
                        };
                        additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
                    }
                    if let Some(value) = &std_primitive_i32_as_postgresql_int_handle {
                        let prefix = match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        };
                        let bind_increments = {
                            let mut bind_increments = std::string::String::default();
                            for (index, element) in value.iter().enumerate() {
                                match postgresql_crud::BindQuery::try_generate_bind_increments(
                                    element,
                                    &mut increment,
                                ) {
                                    Ok(value) => {
                                        let handle = format!(
                                            "std_primitive_i32_as_postgresql_int ~ {value} "
                                        );
                                        match index == 0 {
                                            true => {
                                                bind_increments.push_str(&handle);
                                            }
                                            false => {
                                                bind_increments.push_str(&format!(
                                                    "{} {handle}",
                                                    element.conjuctive_operator
                                                ));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        return TryReadManyResponseVariants :: BindQuery
                                    {
                                        bind_query : e.into_serialize_deserialize_version(),
                                        code_occurence : error_occurence_lib :: code_occurence ::
                                        CodeOccurence ::
                                        new(file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 1505, column : 13,
                                        }))
                                    } ;
                                    }
                                }
                            }
                            if let false = bind_increments.is_empty() {
                                bind_increments.pop();
                            }
                            bind_increments
                        };
                        additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
                    }
                    {
                        let prefix = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                        let value = &parameters.payload.order_by;
                        let order_stringified = match &value.order {
                            Some(order) => order.to_string(),
                            None => crate::server::postgres::order::Order::default().to_string(),
                        };
                        additional_parameters.push_str(&format!(
                            "{}order by {} {}",
                            prefix, value.column, order_stringified
                        ));
                    }
                    {
                        let prefix = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                        let value = match postgresql_crud::BindQuery::try_generate_bind_increments(
                            &parameters.payload.limit,
                            &mut increment,
                        ) {
                            Ok(value) => value,
                            Err(e) => {
                                return TryReadManyResponseVariants :: BindQuery
                            {
                                bind_query : e.into_serialize_deserialize_version(),
                                code_occurence : error_occurence_lib :: code_occurence ::
                                CodeOccurence ::
                                new(file! ().to_string(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1505, column : 13,
                                }))
                            } ;
                            }
                        };
                        additional_parameters.push_str(&format!("{}limit {}", prefix, value));
                    }
                    {
                        let prefix = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                        let value = match postgresql_crud::BindQuery::try_generate_bind_increments(
                            &parameters.payload.offset,
                            &mut increment,
                        ) {
                            Ok(value) => value,
                            Err(e) => {
                                return TryReadManyResponseVariants :: BindQuery
                            {
                                bind_query : e.into_serialize_deserialize_version(),
                                code_occurence : error_occurence_lib :: code_occurence ::
                                CodeOccurence ::
                                new(file! ().to_string(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1505, column : 13,
                                }))
                            } ;
                            }
                        };
                        additional_parameters.push_str(&format!("{}offset {}", prefix, value));
                    }
                    additional_parameters
                }
            )
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(value) = parameters
                .payload
                .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
            {
                query = query.bind(
                    value
                        .into_iter()
                        .map(|element| element.into_inner().clone())
                        .collect::<std::vec::Vec<std::primitive::i64>>(),
                );
            }
            if let Some(values) = std_primitive_bool_as_postgresql_bool_handle {
                for value in values {
                    query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
                }
            }
            if let Some(values) = std_primitive_i16_as_postgresql_small_int_handle {
                for value in values {
                    query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
                }
            }
            if let Some(values) = std_primitive_i32_as_postgresql_int_handle {
                for value in values {
                    query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
                }
            }
            query =
                postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.limit, query);
            query =
                postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.offset, query);
            query
        };
        let vec_values = {
            let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
                Ok(value) => value,
                Err(e) => {
                    let e = TryReadMany::from(e);
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryReadManyResponseVariants::from(e);
                }
            };
            let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                Ok(value) => value,
                Err(e) => {
                    let e = TryReadMany::from(e);
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryReadManyResponseVariants::from(e);
                }
            };
            let mut rows = binded_query.fetch(pg_connection.as_mut());
            let mut vec_values = std::vec::Vec::new();
            let wrapper_vec_column = WrapperVecColumn(parameters.payload.select);
            while let Some(row) = {
                match {
                    use futures::TryStreamExt;
                    rows.try_next()
                }
                .await
                {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryReadMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryReadManyResponseVariants::from(e);
                    }
                }
            } {
                match wrapper_vec_column.options_try_from_sqlx_row(&row) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        let e = TryReadMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryReadManyResponseVariants::from(e);
                    }
                }
            }
            vec_values
        };
        TryReadManyResponseVariants::Desirable(vec_values)
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryReadMany
{
    fn from(
        value: crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, utoipa :: ToSchema)]
pub struct ReadOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64,
    pub select: std::vec::Vec<DogColumn>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct ReadOnePayloadWithSerializeDeserialize {
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
    select: std::vec::Vec<DogColumn>,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamed {
    NotUniqueColumn {
        #[eo_display_with_serialize_deserialize]
        not_unique_column: DogColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<ReadOnePayloadWithSerializeDeserialize> for ReadOnePayload {
    type Error = ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: ReadOnePayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let select = {
            let mut vec = std::vec::Vec::with_capacity(4);
            for element in value.select {
                if vec.contains(&element) {
                    return Err(Self::Error::NotUniqueColumn {
                        not_unique_column: element,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 4449,
                                column: 21,
                            }),
                        ),
                    });
                } else {
                    vec.push(element);
                }
            }
            vec
        };
        Ok(Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            select,
        })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayloadErrorNamed {
    NotUniqueColumn {
        #[eo_display_with_serialize_deserialize]
        not_unique_column: DogColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<ReadOnePayload> for ReadOnePayloadWithSerializeDeserialize {
    type Error = ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayloadErrorNamed;
    fn try_from(value: ReadOnePayload) -> Result<Self, Self::Error> {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let select = {
            let mut vec = std::vec::Vec::with_capacity(4);
            for element in value.select {
                if vec.contains(&element) {
                    return Err(Self::Error::NotUniqueColumn {
                        not_unique_column: element,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 4449,
                                column: 21,
                            }),
                        ),
                    });
                } else {
                    vec.push(element);
                }
            }
            vec
        };
        Ok(Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            select,
        })
    }
}
#[derive(Debug)]
pub struct ReadOneParameters {
    pub payload: ReadOnePayload,
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryReadOne {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        read_one_payload_try_from_read_one_payload_with_serialize_deserialize:
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadOneResponseVariants {
    Desirable(DogOptions), Configuration
    {
        configuration : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Database
    {
        database : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Tls
    {
        tls : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Protocol
    {
        protocol : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, RowNotFound
    {
        row_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, TypeNotFound
    {
        type_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize<>, len : usize<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std::string::String<>, source_handle :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Decode
    {
        decode : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, PoolTimedOut
    {
        pool_timed_out : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, PoolClosed
    {
        pool_closed : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
    {
        read_one_payload_try_from_read_one_payload_with_serialize_deserialize
        :
        ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorNotEqual
    {
        commit_not_equal : std::string::String<>, commit_to_use :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorToStrConversion
    {
        commit_to_str_conversion : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NoCommitExtractorHeader
    {
        no_commit_header : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryReadOne> for TryReadOneResponseVariants {
    fn from(value: TryReadOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryReadOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryReadOneWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryReadOneWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryReadOneWithSerializeDeserialize :: Tls { tls, code_occurence }
            => Self :: Tls { tls, code_occurence },
            TryReadOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryReadOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryReadOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryReadOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryReadOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryReadOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryReadOneWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryReadOneWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryReadOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryReadOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryReadOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryReadOneWithSerializeDeserialize ::
            JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryReadOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryReadOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryReadOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryReadOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryReadOneWithSerializeDeserialize ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            }, TryReadOneWithSerializeDeserialize :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryReadOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryReadOneWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryReadOneResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryReadOneResponseVariants) -> Self {
        match value
        {
            TryReadOneResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: OK, TryReadOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadOneResponseVariants :: Database
            { database : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryReadOneResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryReadOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryReadOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryReadOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadOneResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryReadOneResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryReadOneResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: OK, TryReadOneResponseVariants ::
            Decode { decode : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryReadOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryReadOneResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryReadOneResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryReadOneResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryReadOneResponseVariants ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK, TryReadOneResponseVariants
            :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryReadOneResponseVariants ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryReadOneResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadOneResponseVariantsTvfrr200Ok {
    Desirable(DogOptions),
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr200Ok> for TryReadOneResponseVariants {
    fn from(value: TryReadOneResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadOneResponseVariantsTvfrr400BadRequest {
    TypeNotFound
    {
        type_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
    {
        read_one_payload_try_from_read_one_payload_with_serialize_deserialize
        :
        ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorNotEqual
    {
        commit_not_equal : std::string::String<>, commit_to_use :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorToStrConversion
    {
        commit_to_str_conversion : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NoCommitExtractorHeader
    {
        no_commit_header : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr400BadRequest>
    for TryReadOneResponseVariants
{
    fn from(value: TryReadOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryReadOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            }, TryReadOneResponseVariantsTvfrr400BadRequest ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryReadOneResponseVariantsTvfrr400BadRequest ::
            NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr408RequestTimeout>
    for TryReadOneResponseVariants
{
    fn from(value: TryReadOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadOneResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr500InternalServerError>
    for TryReadOneResponseVariants
{
    fn from(value: TryReadOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr500InternalServerError::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            TryReadOneResponseVariantsTvfrr500InternalServerError::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::BytesRejection {
                bytes_rejection,
                code_occurence,
            } => Self::BytesRejection {
                bytes_rejection,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr500InternalServerError::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryReadOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneResponseVariantsTvfrr404NotFound> for TryReadOneResponseVariants {
    fn from(value: TryReadOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
impl TryFrom<TryReadOneResponseVariants> for DogOptions {
    type Error = TryReadOneWithSerializeDeserialize;
    fn try_from(value: TryReadOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryReadOneResponseVariants :: Desirable(i) => Ok(i),
            TryReadOneResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryReadOneResponseVariants ::
            Database { database, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Database
            { database, code_occurence }), TryReadOneResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Io
            { io, code_occurence }), TryReadOneResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryReadOneResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryReadOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryReadOneResponseVariants ::
            TypeNotFound { type_not_found, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryReadOneResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryReadOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }), TryReadOneResponseVariants
            :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryReadOneResponseVariants :: Decode { decode, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryReadOneResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryReadOneResponseVariants ::
            PoolClosed { pool_closed, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryReadOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryReadOneResponseVariants ::
            Migrate { migrate, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryReadOneResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }), TryReadOneResponseVariants
            :: JsonSyntaxError { json_syntax_error, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }), TryReadOneResponseVariants
            :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryReadOneResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }), TryReadOneResponseVariants
            :: UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryReadOneResponseVariants
            :: ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryReadOneWithSerializeDeserialize ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            }), TryReadOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryReadOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryReadOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryReadOneWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryReadOneWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub enum TryReadOneStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeTvfrr400BadRequest,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryReadOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryReadOneResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            } TryReadOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: Io { io : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryReadOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneErrorNamed {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryReadOneWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayload {
        #[eo_error_occurence]
        read_one_payload_with_serialize_deserialize_try_from_read_one_payload:
            ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayloadErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_one<'a>(
    server_location: &str,
    parameters: ReadOneParameters,
) -> Result<DogOptions, TryReadOneErrorNamed> {
    let payload =
        match ReadOnePayloadWithSerializeDeserialize::try_from(parameters.payload) {
            Ok(value) => match serde_json::to_string(&value) {
                Ok(value) => value,
                Err(e) => {
                    return Err(TryReadOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: e,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1266,
                                column: 13,
                            }),
                        ),
                    });
                }
            },
            Err(e) => {
                return
            Err(TryReadOneErrorNamed ::
            ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayload
            {
                read_one_payload_with_serialize_deserialize_try_from_read_one_payload
                : e, code_occurence : error_occurence_lib :: code_occurence ::
                CodeOccurence ::
                new(file! ().to_string(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 4649, column : 25,
                }))
            }) ;
            }
        };
    let url = format!("{}/dogs/read_one", server_location);
    let future = reqwest::Client::new()
        .post(&url)
        .header(postgresql_crud::COMMIT, git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryReadOneErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2142,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            return Err(TryReadOneErrorNamed::FailedToGetResponseText {
                reqwest: e,
                status_code,
                headers,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2071,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::OK {
        match serde_json::from_str::<TryReadOneResponseVariantsTvfrr200Ok>(&response_text) {
            Ok(value) => TryReadOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryReadOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryReadOneResponseVariantsTvfrr404NotFound>(&response_text) {
            Ok(value) => TryReadOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryReadOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match serde_json::from_str::<TryReadOneResponseVariantsTvfrr408RequestTimeout>(
            &response_text,
        ) {
            Ok(value) => TryReadOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryReadOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryReadOneResponseVariantsTvfrr400BadRequest>(&response_text) {
            Ok(value) => TryReadOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryReadOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        return Err(TryReadOneErrorNamed::UnexpectedStatusCode {
            status_code,
            headers,
            response_text_result:
                crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(
                    response_text,
                ),
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2036,
                    column: 13,
                }),
            ),
        });
    };
    match DogOptions::try_from(variants) {
        Ok(value) => Ok(value),
        Err(e) => {
            return Err(TryReadOneErrorNamed::ExpectedType {
                expected_type: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1998,
                        column: 13,
                    }),
                ),
            });
        }
    }
}
#[utoipa ::
path(post, path = "/dogs/read_one", operation_id = "/dogs/read_one", tag =
"dogs",
request_body(content = ReadOnePayloadWithSerializeDeserialize, description =
"dogs read_one payload", content_type = "application/json"),
responses((status = 200, description = "ok", body =
TryReadOneResponseVariantsTvfrr200Ok, content_type = "application/json"),
(status = 500, description = "internal server error", body =
TryReadOneResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryReadOneResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryReadOneResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryReadOneResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn read_one(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<ReadOnePayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = ReadOneParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                ReadOnePayloadWithSerializeDeserialize,
                TryReadOneResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => match ReadOnePayload::try_from(value) {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryReadOne ::
                    ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
                    {
                        read_one_payload_try_from_read_one_payload_with_serialize_deserialize
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 4813, column : 17,
                        })),
                    } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryReadOneResponseVariants::from(e);
                    }
                },
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        let select = parameters.payload.select;
        let query_string = {
            format!
            ("select {} from dogs where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = $1",
            //HERE
            generate_query_vec_column(& select),)
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let query = postgresql_crud::BindQuery::bind_value_to_query(
                parameters
                    .payload
                    .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                query,
            );
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let e = TryReadOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryReadOneResponseVariants::from(e);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let e = TryReadOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryReadOneResponseVariants::from(e);
            }
        };
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(row) => match WrapperVecColumn(select).options_try_from_sqlx_row(&row) {
                Ok(value) => TryReadOneResponseVariants::Desirable(value),
                Err(e) => {
                    let e = TryReadOne::from(e);
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryReadOneResponseVariants::from(e);
                }
            },
            Err(e) => {
                let e = TryReadOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryReadOneResponseVariants::from(e);
            }
        }
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryReadOne
{
    fn from(
        value: crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, utoipa :: ToSchema)]
pub struct UpdateManyPayloadElement {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64,
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdOptionOptionStdPrimitiveBool,
    pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdOptionOptionStdPrimitiveI16,
    pub std_primitive_i32_as_postgresql_int: postgresql_crud::StdOptionOptionStdPrimitiveI32,
}
#[derive(Debug, utoipa :: ToSchema)]
pub struct UpdateManyPayload(pub std::vec::Vec<UpdateManyPayloadElement>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadElementWithSerializeDeserialize {
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
    pub std_primitive_bool_as_postgresql_bool:
        postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
    pub std_primitive_i16_as_postgresql_small_int:
        postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
    pub std_primitive_i32_as_postgresql_int:
        postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadWithSerializeDeserialize(
    std::vec::Vec<UpdateManyPayloadElementWithSerializeDeserialize>,
);
impl std::convert::From<UpdateManyPayloadElementWithSerializeDeserialize>
    for UpdateManyPayloadElement
{
    fn from(value: UpdateManyPayloadElementWithSerializeDeserialize) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let std_primitive_bool_as_postgresql_bool =
            postgresql_crud::StdOptionOptionStdPrimitiveBool::from(
                value.std_primitive_bool_as_postgresql_bool,
            );
        let std_primitive_i16_as_postgresql_small_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI16::from(
                value.std_primitive_i16_as_postgresql_small_int,
            );
        let std_primitive_i32_as_postgresql_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI32::from(
                value.std_primitive_i32_as_postgresql_int,
            );
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        }
    }
}
impl std::convert::From<UpdateManyPayloadWithSerializeDeserialize> for UpdateManyPayload {
    fn from(value: UpdateManyPayloadWithSerializeDeserialize) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|element| UpdateManyPayloadElement::from(element))
                .collect(),
        )
    }
}
impl std::convert::From<UpdateManyPayloadElement>
    for UpdateManyPayloadElementWithSerializeDeserialize
{
    fn from(value: UpdateManyPayloadElement) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let std_primitive_bool_as_postgresql_bool =
            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                value.std_primitive_bool_as_postgresql_bool,
            );
        let std_primitive_i16_as_postgresql_small_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                value.std_primitive_i16_as_postgresql_small_int,
            );
        let std_primitive_i32_as_postgresql_int =
            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                value.std_primitive_i32_as_postgresql_int,
            );
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        }
    }
}
impl std::convert::From<UpdateManyPayload> for UpdateManyPayloadWithSerializeDeserialize {
    fn from(value: UpdateManyPayload) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|element| UpdateManyPayloadElementWithSerializeDeserialize::from(element))
                .collect(),
        )
    }
}
#[derive(Debug)]
pub struct UpdateManyParameters {
    pub payload: UpdateManyPayload,
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryUpdateMany {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKeys {
        #[eo_vec_display]
        not_unique_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        #[eo_display_with_serialize_deserialize]
        checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        #[eo_display]
        commit_failed: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        #[eo_vec_display]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        #[eo_display]
        primary_key_from_row: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndFailedRollback {
        #[eo_vec_display]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        #[eo_display]
        query_error: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateManyResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>),
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKeys {
        not_unique_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        no_payload_fields: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        commit_failed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        primary_key_from_row: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndFailedRollback {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        query_error: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateMany> for TryUpdateManyResponseVariants {
    fn from(value: TryUpdateMany) -> Self {
        match value.into_serialize_deserialize_version() {
            TryUpdateManyWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            TryUpdateManyWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::JsonDataError {
                json_data_error,
                code_occurence,
            } => Self::JsonDataError {
                json_data_error,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            } => Self::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            } => Self::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::BytesRejection {
                bytes_rejection,
                code_occurence,
            } => Self::BytesRejection {
                bytes_rejection,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::NotUniquePrimaryKeys {
                not_unique_primary_keys,
                code_occurence,
            } => Self::NotUniquePrimaryKeys {
                not_unique_primary_keys,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::BindQuery {
                bind_query,
                code_occurence,
            } => Self::BindQuery {
                bind_query,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::CheckedAdd {
                checked_add,
                code_occurence,
            } => Self::CheckedAdd {
                checked_add,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::NoPayloadFields {
                no_payload_fields,
                code_occurence,
            } => Self::NoPayloadFields {
                no_payload_fields,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::CommitFailed {
                commit_failed,
                code_occurence,
            } => Self::CommitFailed {
                commit_failed,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::NonExistingPrimaryKeys {
                non_existing_primary_keys,
                code_occurence,
            } => Self::NonExistingPrimaryKeys {
                non_existing_primary_keys,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::PrimaryKeyFromRowAndFailedRollback {
                primary_key_from_row,
                rollback_error,
                code_occurence,
            } => Self::PrimaryKeyFromRowAndFailedRollback {
                primary_key_from_row,
                rollback_error,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::NonExistingPrimaryKeysAndFailedRollback {
                non_existing_primary_keys,
                rollback_error,
                code_occurence,
            } => Self::NonExistingPrimaryKeysAndFailedRollback {
                non_existing_primary_keys,
                rollback_error,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::QueryAndRollbackFailed {
                query_error,
                rollback_error,
                code_occurence,
            } => Self::QueryAndRollbackFailed {
                query_error,
                rollback_error,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            } => Self::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            } => Self::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            },
            TryUpdateManyWithSerializeDeserialize::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            } => Self::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            },
        }
    }
}
impl std::convert::From<&TryUpdateManyResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryUpdateManyResponseVariants) -> Self {
        match value {
            TryUpdateManyResponseVariants::Desirable(_) => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::Configuration {
                configuration: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::Database {
                database: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::Io {
                io: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::Tls {
                tls: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::Protocol {
                protocol: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::Decode {
                decode: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::Migrate {
                migrate: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::JsonDataError {
                json_data_error: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::JsonSyntaxError {
                json_syntax_error: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::MissingJsonContentType {
                missing_json_content_type: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::BytesRejection {
                bytes_rejection: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::NotUniquePrimaryKeys {
                not_unique_primary_keys: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::BindQuery {
                bind_query: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::CheckedAdd {
                checked_add: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::NoPayloadFields {
                no_payload_fields: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::CommitFailed {
                commit_failed: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::NonExistingPrimaryKeys {
                non_existing_primary_keys: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::PrimaryKeyFromRowAndFailedRollback {
                primary_key_from_row: _,
                rollback_error: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::NonExistingPrimaryKeysAndFailedRollback {
                non_existing_primary_keys: _,
                rollback_error: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::QueryAndRollbackFailed {
                query_error: _,
                rollback_error: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::CommitExtractorNotEqual {
                commit_not_equal: _,
                commit_to_use: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::CommitExtractorToStrConversion {
                commit_to_str_conversion: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryUpdateManyResponseVariants::NoCommitExtractorHeader {
                no_commit_header: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateManyResponseVariantsTvfrr200Ok {
    Desirable(std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>),
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr200Ok> for TryUpdateManyResponseVariants {
    fn from(value: TryUpdateManyResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryUpdateManyResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateManyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr404NotFound>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryUpdateManyResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateManyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr408RequestTimeout>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryUpdateManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateManyResponseVariantsTvfrr400BadRequest {
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKeys {
        not_unique_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        no_payload_fields: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndFailedRollback {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr400BadRequest>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }
            => Self :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Self :: NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateManyResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        commit_failed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        primary_key_from_row: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        query_error: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr500InternalServerError>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            CheckedAdd { checked_add, code_occurence } => Self :: CheckedAdd
            { checked_add, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            CommitFailed { commit_failed, code_occurence } => Self ::
            CommitFailed { commit_failed, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } => Self
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } => Self ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence }
        }
    }
}
impl TryFrom<TryUpdateManyResponseVariants>
    for std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>
{
    type Error = TryUpdateManyWithSerializeDeserialize;
    fn try_from(value: TryUpdateManyResponseVariants) -> Result<Self, Self::Error> {
        match value {
            TryUpdateManyResponseVariants::Desirable(i) => Ok(i),
            TryUpdateManyResponseVariants::Configuration {
                configuration,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::Database {
                database,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::Database {
                database,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::Io { io, code_occurence } => {
                Err(TryUpdateManyWithSerializeDeserialize::Io { io, code_occurence })
            }
            TryUpdateManyResponseVariants::Tls {
                tls,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::Protocol {
                protocol,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::RowNotFound {
                row_not_found,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Err(
                TryUpdateManyWithSerializeDeserialize::ColumnIndexOutOfBounds {
                    column_index_out_of_bounds,
                    len,
                    code_occurence,
                },
            ),
            TryUpdateManyResponseVariants::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::Decode {
                decode,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::PoolClosed {
                pool_closed,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::Migrate {
                migrate,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::JsonDataError {
                json_data_error,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::JsonDataError {
                json_data_error,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            } => Err(
                TryUpdateManyWithSerializeDeserialize::MissingJsonContentType {
                    missing_json_content_type,
                    code_occurence,
                },
            ),
            TryUpdateManyResponseVariants::BytesRejection {
                bytes_rejection,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::BytesRejection {
                bytes_rejection,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::NotUniquePrimaryKeys {
                not_unique_primary_keys,
                code_occurence,
            } => Err(
                TryUpdateManyWithSerializeDeserialize::NotUniquePrimaryKeys {
                    not_unique_primary_keys,
                    code_occurence,
                },
            ),
            TryUpdateManyResponseVariants::BindQuery {
                bind_query,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::BindQuery {
                bind_query,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::CheckedAdd {
                checked_add,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::CheckedAdd {
                checked_add,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::NoPayloadFields {
                no_payload_fields,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::NoPayloadFields {
                no_payload_fields,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::CommitFailed {
                commit_failed,
                code_occurence,
            } => Err(TryUpdateManyWithSerializeDeserialize::CommitFailed {
                commit_failed,
                code_occurence,
            }),
            TryUpdateManyResponseVariants::NonExistingPrimaryKeys {
                non_existing_primary_keys,
                code_occurence,
            } => Err(
                TryUpdateManyWithSerializeDeserialize::NonExistingPrimaryKeys {
                    non_existing_primary_keys,
                    code_occurence,
                },
            ),
            TryUpdateManyResponseVariants::PrimaryKeyFromRowAndFailedRollback {
                primary_key_from_row,
                rollback_error,
                code_occurence,
            } => Err(
                TryUpdateManyWithSerializeDeserialize::PrimaryKeyFromRowAndFailedRollback {
                    primary_key_from_row,
                    rollback_error,
                    code_occurence,
                },
            ),
            TryUpdateManyResponseVariants::NonExistingPrimaryKeysAndFailedRollback {
                non_existing_primary_keys,
                rollback_error,
                code_occurence,
            } => Err(
                TryUpdateManyWithSerializeDeserialize::NonExistingPrimaryKeysAndFailedRollback {
                    non_existing_primary_keys,
                    rollback_error,
                    code_occurence,
                },
            ),
            TryUpdateManyResponseVariants::QueryAndRollbackFailed {
                query_error,
                rollback_error,
                code_occurence,
            } => Err(
                TryUpdateManyWithSerializeDeserialize::QueryAndRollbackFailed {
                    query_error,
                    rollback_error,
                    code_occurence,
                },
            ),
            TryUpdateManyResponseVariants::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            } => Err(
                TryUpdateManyWithSerializeDeserialize::CommitExtractorNotEqual {
                    commit_not_equal,
                    commit_to_use,
                    code_occurence,
                },
            ),
            TryUpdateManyResponseVariants::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            } => Err(
                TryUpdateManyWithSerializeDeserialize::CommitExtractorToStrConversion {
                    commit_to_str_conversion,
                    code_occurence,
                },
            ),
            TryUpdateManyResponseVariants::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            } => Err(
                TryUpdateManyWithSerializeDeserialize::NoCommitExtractorHeader {
                    no_commit_header,
                    code_occurence,
                },
            ),
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryUpdateManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub enum TryUpdateManyStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    NotUniquePrimaryKeysTvfrr400BadRequest,
    BindQueryTvfrr500InternalServerError,
    CheckedAddTvfrr500InternalServerError,
    NoPayloadFieldsTvfrr400BadRequest,
    CommitFailedTvfrr500InternalServerError,
    NonExistingPrimaryKeysTvfrr400BadRequest,
    PrimaryKeyFromRowAndFailedRollbackTvfrr500InternalServerError,
    NonExistingPrimaryKeysAndFailedRollbackTvfrr400BadRequest,
    QueryAndRollbackFailedTvfrr500InternalServerError,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryUpdateManyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match &self {
            TryUpdateManyResponseVariants::Desirable(_) => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::Configuration {
                configuration: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::Database {
                database: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::Io {
                io: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::Tls {
                tls: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::Protocol {
                protocol: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::Decode {
                decode: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::Migrate {
                migrate: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::JsonDataError {
                json_data_error: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::JsonSyntaxError {
                json_syntax_error: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::MissingJsonContentType {
                missing_json_content_type: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::BytesRejection {
                bytes_rejection: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::NotUniquePrimaryKeys {
                not_unique_primary_keys: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::BindQuery {
                bind_query: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::CheckedAdd {
                checked_add: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::NoPayloadFields {
                no_payload_fields: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::CommitFailed {
                commit_failed: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::NonExistingPrimaryKeys {
                non_existing_primary_keys: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::PrimaryKeyFromRowAndFailedRollback {
                primary_key_from_row: _,
                rollback_error: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::NonExistingPrimaryKeysAndFailedRollback {
                non_existing_primary_keys: _,
                rollback_error: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::QueryAndRollbackFailed {
                query_error: _,
                rollback_error: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::CommitExtractorNotEqual {
                commit_not_equal: _,
                commit_to_use: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::CommitExtractorToStrConversion {
                commit_to_str_conversion: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryUpdateManyResponseVariants::NoCommitExtractorHeader {
                no_commit_header: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyErrorNamed {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryUpdateManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_many<'a>(
    server_location: &str,
    parameters: UpdateManyParameters,
) -> Result<std::vec::Vec<postgresql_crud::StdPrimitiveI64>, TryUpdateManyErrorNamed> {
    let payload = match serde_json::to_string(&UpdateManyPayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryUpdateManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1266,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/update_many", server_location,);
    let future = reqwest::Client::new()
        .patch(&url)
        .header(postgresql_crud::COMMIT, git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryUpdateManyErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2142,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            return Err(TryUpdateManyErrorNamed::FailedToGetResponseText {
                reqwest: e,
                status_code,
                headers,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2071,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::OK {
        match serde_json::from_str::<TryUpdateManyResponseVariantsTvfrr200Ok>(&response_text) {
            Ok(value) => TryUpdateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match serde_json::from_str::<TryUpdateManyResponseVariantsTvfrr408RequestTimeout>(
            &response_text,
        ) {
            Ok(value) => TryUpdateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match serde_json::from_str::<TryUpdateManyResponseVariantsTvfrr500InternalServerError>(
            &response_text,
        ) {
            Ok(value) => TryUpdateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryUpdateManyResponseVariantsTvfrr400BadRequest>(
            &response_text,
        ) {
            Ok(value) => TryUpdateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        return Err(TryUpdateManyErrorNamed::UnexpectedStatusCode {
            status_code,
            headers,
            response_text_result:
                crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(
                    response_text,
                ),
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2036,
                    column: 13,
                }),
            ),
        });
    };
    match std::vec::Vec::<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>::try_from(
        variants,
    ) {
        Ok(value) => Ok(value
            .into_iter()
            .map(|element| postgresql_crud::StdPrimitiveI64::from(element))
            .collect()),
        Err(e) => {
            return Err(TryUpdateManyErrorNamed::ExpectedType {
                expected_type: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1998,
                        column: 13,
                    }),
                ),
            });
        }
    }
}
#[utoipa ::
path(patch, path = "/dogs/update_many", operation_id = "/dogs/update_many",
tag = "dogs",
request_body(content = UpdateManyPayloadWithSerializeDeserialize, description
= "dogs update_many payload", content_type = "application/json"),
responses((status = 200, description = "ok", body =
TryUpdateManyResponseVariantsTvfrr200Ok, content_type = "application/json"),
(status = 500, description = "internal server error", body =
TryUpdateManyResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryUpdateManyResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryUpdateManyResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryUpdateManyResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn update_many<'a>(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<UpdateManyPayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = UpdateManyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                UpdateManyPayloadWithSerializeDeserialize,
                TryUpdateManyResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => UpdateManyPayload::from(value),
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        {
            let not_unique_primary_keys = {
                let mut vec = std::vec::Vec::with_capacity(parameters.payload.0.len());
                let mut not_unique_primary_keys =
                    std::vec::Vec::with_capacity(parameters.payload.0.len());
                for element in &parameters.payload.0 {
                    let handle =
                        &element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;
                    match vec.contains(&handle) {
                        true => {
                            not_unique_primary_keys.push(
                                element
                                    .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                                    .clone(),
                            );
                        }
                        false => {
                            vec.push(&
                            element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key)
                            ;
                        }
                    }
                }
                not_unique_primary_keys
            };
            if let false = not_unique_primary_keys.is_empty() {
                let e = TryUpdateMany::NotUniquePrimaryKeys {
                    not_unique_primary_keys,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1659,
                            column: 13,
                        }),
                    ),
                };
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryUpdateManyResponseVariants::from(e);
            }
        }
        let expected_updated_primary_keys = {
            parameters
                .payload
                .0
                .iter()
                .map(|element| {
                    element
                        .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                        .clone()
                })
                .collect::<std::vec::Vec<postgresql_crud::StdPrimitiveI64>>()
        };
        let binded_query = {
            let query_string = {
                "update dogs as t set std_primitive_bool_as_postgresql_bool = data.std_primitive_bool_as_postgresql_bool, std_primitive_i16_as_postgresql_small_int = data.std_primitive_i16_as_postgresql_small_int, std_primitive_i32_as_postgresql_int = data.std_primitive_i32_as_postgresql_int from (select * from unnest($1, $2, $3, $4)) as data(std_primitive_bool_as_postgresql_bool, std_primitive_i16_as_postgresql_small_int, std_primitive_i32_as_postgresql_int, std_primitive_i64_as_postgresql_big_serial_not_null_primary_key) where t.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = data.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key returning data.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"
            };
            println!("{}", query_string);
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let current_vec_len = parameters.payload.0.len();
            let (
                std_primitive_bool_as_postgresql_bool_vec,
                std_primitive_i16_as_postgresql_small_int_vec,
                std_primitive_i32_as_postgresql_int_vec,
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
            ) = parameters.payload.0.into_iter().fold(
                (
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                ),
                |mut acc, element| {
                    acc.0.push(element.std_primitive_bool_as_postgresql_bool);
                    acc.1
                        .push(element.std_primitive_i16_as_postgresql_small_int);
                    acc.2.push(element.std_primitive_i32_as_postgresql_int);
                    acc.3.push(
                        element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                    );
                    acc
                },
            );
            query = query.bind(
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec
                    .into_iter()
                    .map(|element| element.into_inner())
                    .collect::<std::vec::Vec<std::primitive::i64>>(),
            );
            query = query.bind(
                std_primitive_bool_as_postgresql_bool_vec
                    .into_iter()
                    .map(|element| element.into_inner())
                    .collect::<std::vec::Vec<std::option::Option<std::primitive::bool>>>(),
            );
            query = query.bind(
                std_primitive_i16_as_postgresql_small_int_vec
                    .into_iter()
                    .map(|element| element.into_inner())
                    .collect::<std::vec::Vec<std::option::Option<std::primitive::i16>>>(),
            );
            query = query.bind(
                std_primitive_i32_as_postgresql_int_vec
                    .into_iter()
                    .map(|element| element.into_inner())
                    .collect::<std::vec::Vec<std::option::Option<std::primitive::i32>>>(),
            );
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let e = TryUpdateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryUpdateManyResponseVariants::from(e);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let e = TryUpdateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryUpdateManyResponseVariants::from(e);
            }
        };
        let mut postgres_transaction = match {
            use sqlx::Acquire;
            pg_connection.begin()
        }
        .await
        {
            Ok(value) => value,
            Err(e) => {
                let e = TryUpdateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryUpdateManyResponseVariants::from(e);
            }
        };
        let results_vec = {
            let mut results_vec = std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
            let mut option_error: Option<sqlx::Error> = None;
            {
                let mut rows = binded_query.fetch(postgres_transaction.as_mut());
                while let (Some(Some(row)), None) = (
                    match {
                        use futures::TryStreamExt;
                        rows.try_next()
                    }
                    .await
                    {
                        Ok(value) => Some(value),
                        Err(e) => {
                            option_error = Some(e);
                            None
                        }
                    },
                    &option_error,
                ) {
                    results_vec.push(row);
                }
            }
            if let Some(e) = option_error {
                match postgres_transaction.rollback().await {
                    Ok(_) => {
                        let e = TryUpdateMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryUpdateManyResponseVariants::from(e);
                    }
                    Err(rollback_error) => {
                        let e = TryUpdateMany :: QueryAndRollbackFailed
                        {
                            query_error : e, rollback_error, code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_string(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                line : 37, column : 13,
                            })),
                        } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryUpdateManyResponseVariants::from(e);
                    }
                }
            }
            results_vec
        };
        let primary_key_vec = {
            let mut primary_key_vec =
                std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
            for element in results_vec {
                match primary_key_try_from_sqlx_row(&element) {
                    Ok(primary_key) => {
                        primary_key_vec.push(primary_key);
                    }
                    Err(e) => match postgres_transaction.rollback().await {
                        Ok(_) => {
                            let e = TryUpdateMany::from(e);
                            error_occurence_lib::error_log::ErrorLog::error_log(
                                &e,
                                app_state.as_ref(),
                            );
                            return TryUpdateManyResponseVariants::from(e);
                        }
                        Err(rollback_error) => {
                            let e = TryUpdateMany :: PrimaryKeyFromRowAndFailedRollback
                            {
                                primary_key_from_row : e, rollback_error, code_occurence :
                                error_occurence_lib :: code_occurence :: CodeOccurence ::
                                new(file! ().to_string(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                    line : 52, column : 13,
                                })),
                            } ;
                            error_occurence_lib::error_log::ErrorLog::error_log(
                                &e,
                                app_state.as_ref(),
                            );
                            return TryUpdateManyResponseVariants::from(e);
                        }
                    },
                }
            }
            primary_key_vec
        };
        {
            let non_existing_primary_keys = {
                let len = expected_updated_primary_keys.len();
                expected_updated_primary_keys.into_iter().fold(
                    std::vec::Vec::with_capacity(len),
                    |mut acc, element| {
                        if let false = primary_key_vec.contains(&element) {
                            acc.push(element);
                        }
                        acc
                    },
                )
            };
            if let false = non_existing_primary_keys.is_empty() {
                match postgres_transaction.rollback().await {
                    Ok(_) => {
                        let e = TryUpdateMany :: NonExistingPrimaryKeys
                        {
                            non_existing_primary_keys, code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_string(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                line : 67, column : 13,
                            })),
                        } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryUpdateManyResponseVariants::from(e);
                    }
                    Err(e) => {
                        let e = TryUpdateMany ::
                        NonExistingPrimaryKeysAndFailedRollback
                        {
                            non_existing_primary_keys, rollback_error : e,
                            code_occurence : error_occurence_lib :: code_occurence ::
                            CodeOccurence ::
                            new(file! ().to_string(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                line : 81, column : 13,
                            })),
                        } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryUpdateManyResponseVariants::from(e);
                    }
                }
            }
        }
        match postgres_transaction.commit().await {
            Ok(_) => TryUpdateManyResponseVariants::Desirable(
                primary_key_vec
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(element)
                    })
                    .collect(),
            ),
            Err(e) => {
                let e = TryUpdateMany :: CommitFailed
                {
                    commit_failed : e, code_occurence : error_occurence_lib ::
                    code_occurence :: CodeOccurence ::
                    new(file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                        line : 96, column : 13,
                    })),
                } ;
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                TryUpdateManyResponseVariants::from(e)
            }
        }
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryUpdateMany
{
    fn from(
        value: crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, utoipa :: ToSchema)]
pub struct UpdateOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64,
    pub std_primitive_bool_as_postgresql_bool:
        std::option::Option<postgresql_crud::StdOptionOptionStdPrimitiveBool>,
    pub std_primitive_i16_as_postgresql_small_int:
        std::option::Option<postgresql_crud::StdOptionOptionStdPrimitiveI16>,
    pub std_primitive_i32_as_postgresql_int:
        std::option::Option<postgresql_crud::StdOptionOptionStdPrimitiveI32>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateOnePayloadWithSerializeDeserialize {
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
    std_primitive_bool_as_postgresql_bool: std::option::Option<
        postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
    >,
    std_primitive_i16_as_postgresql_small_int: std::option::Option<
        postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
    >,
    std_primitive_i32_as_postgresql_int: std::option::Option<
        postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
    >,
}
impl std::convert::From<UpdateOnePayloadWithSerializeDeserialize> for UpdateOnePayload {
    fn from(value: UpdateOnePayloadWithSerializeDeserialize) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let std_primitive_bool_as_postgresql_bool =
            match value.std_primitive_bool_as_postgresql_bool {
                Some(value) => Some(postgresql_crud::StdOptionOptionStdPrimitiveBool::from(
                    value,
                )),
                None => None,
            };
        let std_primitive_i16_as_postgresql_small_int =
            match value.std_primitive_i16_as_postgresql_small_int {
                Some(value) => Some(postgresql_crud::StdOptionOptionStdPrimitiveI16::from(value)),
                None => None,
            };
        let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
            Some(value) => Some(postgresql_crud::StdOptionOptionStdPrimitiveI32::from(value)),
            None => None,
        };
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
        }
    }
}
impl std::convert::From<UpdateOnePayload> for UpdateOnePayloadWithSerializeDeserialize {
    fn from(value: UpdateOnePayload) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let std_primitive_bool_as_postgresql_bool =
            match value.std_primitive_bool_as_postgresql_bool {
                Some(value) => Some(
                    postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                        value,
                    ),
                ),
                None => None,
            };
        let std_primitive_i16_as_postgresql_small_int =
            match value.std_primitive_i16_as_postgresql_small_int {
                Some(value) => Some(
                    postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                        value,
                    ),
                ),
                None => None,
            };
        let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
            Some(value) => Some(
                postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                    value,
                ),
            ),
            None => None,
        };
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
        }
    }
}
#[derive(Debug)]
pub struct UpdateOneParameters {
    pub payload: UpdateOnePayload,
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryUpdateOne {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_display]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateOneResponseVariants {
    Desirable(postgresql_crud::StdPrimitiveI64WithSerializeDeserialize),
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        no_payload_fields: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateOne> for TryUpdateOneResponseVariants {
    fn from(value: TryUpdateOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryUpdateOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryUpdateOneWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryUpdateOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryUpdateOneWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryUpdateOneWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryUpdateOneWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryUpdateOneWithSerializeDeserialize :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryUpdateOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryUpdateOneWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryUpdateOneResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryUpdateOneResponseVariants) -> Self {
        match value
        {
            TryUpdateOneResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: Database
            { database : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryUpdateOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryUpdateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateOneResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: OK, TryUpdateOneResponseVariants
            :: Decode { decode : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryUpdateOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateOneResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            BindQuery { bind_query : _, code_occurence : _ } => axum :: http
            :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            NoPayloadFields { no_payload_fields : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryUpdateOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryUpdateOneResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateOneResponseVariantsTvfrr200Ok {
    Desirable(postgresql_crud::StdPrimitiveI64WithSerializeDeserialize),
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr200Ok> for TryUpdateOneResponseVariants {
    fn from(value: TryUpdateOneResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryUpdateOneResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateOneResponseVariantsTvfrr400BadRequest {
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        no_payload_fields: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr400BadRequest>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value {
            TryUpdateOneResponseVariantsTvfrr400BadRequest::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            TryUpdateOneResponseVariantsTvfrr400BadRequest::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            TryUpdateOneResponseVariantsTvfrr400BadRequest::JsonDataError {
                json_data_error,
                code_occurence,
            } => Self::JsonDataError {
                json_data_error,
                code_occurence,
            },
            TryUpdateOneResponseVariantsTvfrr400BadRequest::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            } => Self::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            },
            TryUpdateOneResponseVariantsTvfrr400BadRequest::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            } => Self::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            },
            TryUpdateOneResponseVariantsTvfrr400BadRequest::NoPayloadFields {
                no_payload_fields,
                code_occurence,
            } => Self::NoPayloadFields {
                no_payload_fields,
                code_occurence,
            },
            TryUpdateOneResponseVariantsTvfrr400BadRequest::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            } => Self::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            },
            TryUpdateOneResponseVariantsTvfrr400BadRequest::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            } => Self::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            },
            TryUpdateOneResponseVariantsTvfrr400BadRequest::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            } => Self::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateOneResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr500InternalServerError>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateOneResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr408RequestTimeout>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryUpdateOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateOneResponseVariantsTvfrr404NotFound>
    for TryUpdateOneResponseVariants
{
    fn from(value: TryUpdateOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryUpdateOneResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
impl TryFrom<TryUpdateOneResponseVariants>
    for postgresql_crud::StdPrimitiveI64WithSerializeDeserialize
{
    type Error = TryUpdateOneWithSerializeDeserialize;
    fn try_from(value: TryUpdateOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryUpdateOneResponseVariants :: Desirable(i) => Ok(i),
            TryUpdateOneResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryUpdateOneResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Database
            { database, code_occurence }), TryUpdateOneResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Io
            { io, code_occurence }), TryUpdateOneResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryUpdateOneResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryUpdateOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryUpdateOneResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryUpdateOneResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryUpdateOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryUpdateOneResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryUpdateOneResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryUpdateOneWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryUpdateOneResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryUpdateOneResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryUpdateOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryUpdateOneResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryUpdateOneResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }), TryUpdateOneResponseVariants
            :: JsonSyntaxError { json_syntax_error, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryUpdateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryUpdateOneResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }), TryUpdateOneResponseVariants
            :: UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryUpdateOneResponseVariants
            :: BindQuery { bind_query, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryUpdateOneResponseVariants ::
            NoPayloadFields { no_payload_fields, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence }),
            TryUpdateOneResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }), TryUpdateOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryUpdateOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryUpdateOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryUpdateOneWithSerializeDeserialize ::
            NoCommitExtractorHeader { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryUpdateOneWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub enum TryUpdateOneStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    BindQueryTvfrr500InternalServerError,
    NoPayloadFieldsTvfrr400BadRequest,
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServerTvfrr500InternalServerError,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryUpdateOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryUpdateOneResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            } TryUpdateOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateOneErrorNamed {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryUpdateOneWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_one<'a>(
    server_location: &str,
    parameters: UpdateOneParameters,
) -> Result<postgresql_crud::StdPrimitiveI64, TryUpdateOneErrorNamed> {
    let payload = match serde_json::to_string(&UpdateOnePayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryUpdateOneErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1266,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/update_one", server_location);
    let future = reqwest::Client::new()
        .patch(&url)
        .header(postgresql_crud::COMMIT, git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryUpdateOneErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2142,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            return Err(TryUpdateOneErrorNamed::FailedToGetResponseText {
                reqwest: e,
                status_code,
                headers,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2071,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::OK {
        match serde_json::from_str::<TryUpdateOneResponseVariantsTvfrr200Ok>(&response_text) {
            Ok(value) => TryUpdateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryUpdateOneResponseVariantsTvfrr404NotFound>(&response_text) {
            Ok(value) => TryUpdateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match serde_json::from_str::<TryUpdateOneResponseVariantsTvfrr500InternalServerError>(
            &response_text,
        ) {
            Ok(value) => TryUpdateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryUpdateOneResponseVariantsTvfrr400BadRequest>(&response_text)
        {
            Ok(value) => TryUpdateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        return Err(TryUpdateOneErrorNamed::UnexpectedStatusCode {
            status_code,
            headers,
            response_text_result:
                crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(
                    response_text,
                ),
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2036,
                    column: 13,
                }),
            ),
        });
    };
    match postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::try_from(variants) {
        Ok(value) => Ok(postgresql_crud::StdPrimitiveI64::from(value)),
        Err(e) => {
            return Err(TryUpdateOneErrorNamed::ExpectedType {
                expected_type: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1998,
                        column: 13,
                    }),
                ),
            });
        }
    }
}
#[utoipa ::
path(patch, path = "/dogs/update_one", operation_id = "/dogs/update_one", tag
= "dogs",
request_body(content = UpdateOnePayloadWithSerializeDeserialize, description =
"dogs update_one payload", content_type = "application/json"),
responses((status = 200, description = "ok", body =
TryUpdateOneResponseVariantsTvfrr200Ok, content_type = "application/json"),
(status = 500, description = "internal server error", body =
TryUpdateOneResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryUpdateOneResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryUpdateOneResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryUpdateOneResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn update_one<'a>(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<UpdateOnePayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = UpdateOneParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                UpdateOnePayloadWithSerializeDeserialize,
                TryUpdateOneResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => UpdateOnePayload::from(value),
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        if let (None, None, None) = (
            &parameters.payload.std_primitive_bool_as_postgresql_bool,
            &parameters.payload.std_primitive_i16_as_postgresql_small_int,
            &parameters.payload.std_primitive_i32_as_postgresql_int,
        ) {
            return TryUpdateOneResponseVariants::NoPayloadFields {
                no_payload_fields: std::string::String::from("no payload fields"),
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/check_for_none.rs",
                        ),
                        line: 49,
                        column: 13,
                    }),
                ),
            };
        }
        let query_string = {
            let mut increment: u64 = 0;
            let mut query = std::string::String::from("update dogs set ");
            if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool {
                match postgresql_crud::BindQuery::try_increment(value, &mut increment) {
                    Ok(_) => {
                        query.push_str(&format!(
                            "std_primitive_bool_as_postgresql_bool = ${increment}, "
                        ));
                    }
                    Err(e) => {
                        return TryUpdateOneResponseVariants::BindQuery {
                            bind_query: e.into_serialize_deserialize_version(),
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_string(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1505,
                                    column: 13,
                                }),
                            ),
                        };
                    }
                }
            }
            if let Some(value) = &parameters.payload.std_primitive_i16_as_postgresql_small_int {
                match postgresql_crud::BindQuery::try_increment(value, &mut increment) {
                    Ok(_) => {
                        query.push_str(&format!(
                            "std_primitive_i16_as_postgresql_small_int = ${increment}, "
                        ));
                    }
                    Err(e) => {
                        return TryUpdateOneResponseVariants::BindQuery {
                            bind_query: e.into_serialize_deserialize_version(),
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_string(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1505,
                                    column: 13,
                                }),
                            ),
                        };
                    }
                }
            }
            if let Some(value) = &parameters.payload.std_primitive_i32_as_postgresql_int {
                match postgresql_crud::BindQuery::try_increment(value, &mut increment) {
                    Ok(_) => {
                        query.push_str(&format!(
                            "std_primitive_i32_as_postgresql_int = ${increment}"
                        ));
                    }
                    Err(e) => {
                        return TryUpdateOneResponseVariants::BindQuery {
                            bind_query: e.into_serialize_deserialize_version(),
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_string(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1505,
                                    column: 13,
                                }),
                            ),
                        };
                    }
                }
            }
            match postgresql_crud::BindQuery::try_increment(
                &parameters
                    .payload
                    .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                &mut increment,
            ) {
                Ok(_) => {
                    query.push_str(& format!
                    (" where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = ${increment}"))
                    ;
                }
                Err(e) => {
                    return TryUpdateOneResponseVariants::BindQuery {
                        bind_query: e.into_serialize_deserialize_version(),
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1505,
                                column: 13,
                            }),
                        ),
                    };
                }
            }
            query.push_str(&format!(
                " returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"
            ));
            query
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(value) = parameters.payload.std_primitive_bool_as_postgresql_bool {
                query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
            }
            if let Some(value) = parameters.payload.std_primitive_i16_as_postgresql_small_int {
                query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
            }
            if let Some(value) = parameters.payload.std_primitive_i32_as_postgresql_int {
                query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
            }
            query = postgresql_crud::BindQuery::bind_value_to_query(
                parameters
                    .payload
                    .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                query,
            );
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let e = TryUpdateOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryUpdateOneResponseVariants::from(e);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let e = TryUpdateOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryUpdateOneResponseVariants::from(e);
            }
        };
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(value) => match {
                use sqlx::Row;
                value.try_get::<std::primitive::i64, &str>(
                    "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                )
            } {
                Ok(value) => TryUpdateOneResponseVariants::Desirable(
                    postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                        postgresql_crud::StdPrimitiveI64(value),
                    ),
                ),
                Err(e) => {
                    let e = TryUpdateOne ::
                    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
                    {
                        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1732, column : 13,
                        })),
                    } ;
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryUpdateOneResponseVariants::from(e);
                }
            },
            Err(e) => {
                let e = TryUpdateOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryUpdateOneResponseVariants::from(e);
            }
        }
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryUpdateOne
{
    fn from(
        value: crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, utoipa :: ToSchema)]
pub struct DeleteManyPayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        std::option::Option<std::vec::Vec<postgresql_crud::StdPrimitiveI64>>,
    pub std_primitive_bool_as_postgresql_bool:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveBool>>,
    pub std_primitive_i16_as_postgresql_small_int:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16>>,
    pub std_primitive_i32_as_postgresql_int:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DeleteManyPayloadWithSerializeDeserialize {
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<
        std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>,
    >,
    std_primitive_bool_as_postgresql_bool: std::option::Option<
        std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
    >,
    std_primitive_i16_as_postgresql_small_int: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize>,
    >,
    std_primitive_i32_as_postgresql_int: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize>,
    >,
}
impl std::convert::From<DeleteManyPayloadWithSerializeDeserialize> for DeleteManyPayload {
    fn from(value: DeleteManyPayloadWithSerializeDeserialize) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            match value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| postgresql_crud::StdPrimitiveI64::from(element))
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_bool_as_postgresql_bool =
            match value.std_primitive_bool_as_postgresql_bool {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveBool::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_i16_as_postgresql_small_int =
            match value.std_primitive_i16_as_postgresql_small_int {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI32::from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        }
    }
}
impl std::convert::From<DeleteManyPayload> for DeleteManyPayloadWithSerializeDeserialize {
    fn from(value: DeleteManyPayload) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
        match
        value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize ::
            from(element)).collect :: < std :: vec :: Vec <
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize >> ()),
            None => None,
        } ;
        let std_primitive_bool_as_postgresql_bool = match
        value.std_primitive_bool_as_postgresql_bool
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
            :: from(element)).collect()), None => None
        } ;
        let std_primitive_i16_as_postgresql_small_int = match value
            .std_primitive_i16_as_postgresql_small_int
        {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize
            :: from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize
            :: from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        }
    }
}
#[derive(Debug)]
pub struct DeleteManyParameters {
    pub payload: DeleteManyPayload,
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryDeleteMany {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_std_primitive_bool_as_postgresql_bool_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_std_primitive_i16_as_postgresql_small_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI32AsPostgresqlIntVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_std_primitive_i32_as_postgresql_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec:
            std::vec::Vec<postgresql_crud::WhereStdPrimitiveI64WithSerializeDeserialize>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKeys {
        #[eo_vec_display]
        not_unique_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadParameters {
        #[eo_display_with_serialize_deserialize]
        no_payload_parameters: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        #[eo_vec_display]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndFailedRollback {
        #[eo_vec_display]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        #[eo_display]
        primary_key_from_row: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        #[eo_display]
        commit_failed: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        #[eo_display]
        query_error: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_display]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteManyResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>),
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec {
        not_unique_std_primitive_bool_as_postgresql_bool_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec {
        not_unique_std_primitive_i16_as_postgresql_small_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI32AsPostgresqlIntVec {
        not_unique_std_primitive_i32_as_postgresql_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec {
        not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec:
            std::vec::Vec<postgresql_crud::WhereStdPrimitiveI64WithSerializeDeserialize>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKeys {
        not_unique_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        no_payload_fields: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadParameters {
        no_payload_parameters: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndFailedRollback {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        primary_key_from_row: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        commit_failed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        query_error: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteMany> for TryDeleteManyResponseVariants {
    fn from(value: TryDeleteMany) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryDeleteManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Io { io, code_occurence }
            => Self :: Io { io, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryDeleteManyWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryDeleteManyWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryDeleteManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            }, TryDeleteManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            }, TryDeleteManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            }, TryDeleteManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            } => Self ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            }, TryDeleteManyWithSerializeDeserialize :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } => Self ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: NoPayloadParameters
            { no_payload_parameters, code_occurence } => Self ::
            NoPayloadParameters { no_payload_parameters, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryDeleteManyWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Self :: NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence },
            TryDeleteManyWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } => Self
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: CommitFailed
            { commit_failed, code_occurence } => Self :: CommitFailed
            { commit_failed, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } => Self ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence },
            TryDeleteManyWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryDeleteManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryDeleteManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryDeleteManyResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryDeleteManyResponseVariants) -> Self {
        match value
        {
            TryDeleteManyResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: Database
            { database : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteManyResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryDeleteManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryDeleteManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants
            :: Decode { decode : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec : _,
                code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec : _,
                code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec : _,
                code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            BindQuery { bind_query : _, code_occurence : _ } => axum :: http
            :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NoPayloadFields { no_payload_fields : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NoPayloadParameters
            { no_payload_parameters : _, code_occurence : _ } => axum :: http
            :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            {
                non_existing_primary_keys : _, rollback_error : _,
                code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            {
                primary_key_from_row : _, rollback_error : _, code_occurence :
                _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants :: CommitFailed
            { commit_failed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants ::
            QueryAndRollbackFailed
            { query_error : _, rollback_error : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteManyResponseVariantsTvfrr200Ok {
    Desirable(std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>),
}
impl std::convert::From<TryDeleteManyResponseVariantsTvfrr200Ok> for TryDeleteManyResponseVariants {
    fn from(value: TryDeleteManyResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryDeleteManyResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteManyResponseVariantsTvfrr400BadRequest {
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec {
        not_unique_std_primitive_bool_as_postgresql_bool_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec {
        not_unique_std_primitive_i16_as_postgresql_small_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI32AsPostgresqlIntVec {
        not_unique_std_primitive_i32_as_postgresql_int_vec: std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
        >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec {
        not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec:
            std::vec::Vec<postgresql_crud::WhereStdPrimitiveI64WithSerializeDeserialize>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKeys {
        not_unique_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        no_payload_fields: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadParameters {
        no_payload_parameters: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndFailedRollback {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteManyResponseVariantsTvfrr400BadRequest>
    for TryDeleteManyResponseVariants
{
    fn from(value: TryDeleteManyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryDeleteManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            }, TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            }, TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            }, TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            } => Self ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            }, TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }
            => Self :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NoPayloadParameters { no_payload_parameters, code_occurence } =>
            Self :: NoPayloadParameters
            { no_payload_parameters, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Self :: NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteManyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteManyResponseVariantsTvfrr404NotFound>
    for TryDeleteManyResponseVariants
{
    fn from(value: TryDeleteManyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryDeleteManyResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteManyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteManyResponseVariantsTvfrr408RequestTimeout>
    for TryDeleteManyResponseVariants
{
    fn from(value: TryDeleteManyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryDeleteManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteManyResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        primary_key_from_row: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        commit_failed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        query_error: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteManyResponseVariantsTvfrr500InternalServerError>
    for TryDeleteManyResponseVariants
{
    fn from(value: TryDeleteManyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } => Self
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            CommitFailed { commit_failed, code_occurence } => Self ::
            CommitFailed { commit_failed, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } => Self ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }
        }
    }
}
impl TryFrom<TryDeleteManyResponseVariants>
    for std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>
{
    type Error = TryDeleteManyWithSerializeDeserialize;
    fn try_from(value: TryDeleteManyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryDeleteManyResponseVariants :: Desirable(i) => Ok(i),
            TryDeleteManyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryDeleteManyResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Database
            { database, code_occurence }), TryDeleteManyResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryDeleteManyResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryDeleteManyResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryDeleteManyResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryDeleteManyResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryDeleteManyResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryDeleteManyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryDeleteManyResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryDeleteManyResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryDeleteManyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryDeleteManyResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryDeleteManyResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryDeleteManyResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryDeleteManyResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryDeleteManyResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryDeleteManyResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryDeleteManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryDeleteManyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }),
            TryDeleteManyResponseVariants :: UnexpectedCase
            { unexpected_case, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }),
            TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec,
                code_occurence
            }), TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                code_occurence
            }), TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec,
                code_occurence
            }), TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
                code_occurence
            }), TryDeleteManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence }),
            TryDeleteManyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryDeleteManyResponseVariants ::
            NoPayloadFields { no_payload_fields, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence }),
            TryDeleteManyResponseVariants :: NoPayloadParameters
            { no_payload_parameters, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: NoPayloadParameters
            { no_payload_parameters, code_occurence }),
            TryDeleteManyResponseVariants :: NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence }),
            TryDeleteManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence }),
            TryDeleteManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence }),
            TryDeleteManyResponseVariants :: CommitFailed
            { commit_failed, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: CommitFailed
            { commit_failed, code_occurence }), TryDeleteManyResponseVariants
            :: QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence }),
            TryDeleteManyResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }), TryDeleteManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryDeleteManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryDeleteManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            NoCommitExtractorHeader { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteManyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryDeleteManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub enum TryDeleteManyStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolVecTvfrr400BadRequest,
    NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVecTvfrr400BadRequest,
    NotUniqueStdPrimitiveI32AsPostgresqlIntVecTvfrr400BadRequest,
    NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVecTvfrr400BadRequest,
    NotUniquePrimaryKeysTvfrr400BadRequest,
    BindQueryTvfrr500InternalServerError,
    NoPayloadFieldsTvfrr400BadRequest,
    NoPayloadParametersTvfrr400BadRequest,
    NonExistingPrimaryKeysTvfrr400BadRequest,
    NonExistingPrimaryKeysAndFailedRollbackTvfrr400BadRequest,
    PrimaryKeyFromRowAndFailedRollbackTvfrr500InternalServerError,
    CommitFailedTvfrr500InternalServerError,
    QueryAndRollbackFailedTvfrr500InternalServerError,
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServerTvfrr500InternalServerError,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryDeleteManyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryDeleteManyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            } TryDeleteManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
            {
                not_unique_std_primitive_bool_as_postgresql_bool_vec : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_vec : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntVec
            {
                not_unique_std_primitive_i32_as_postgresql_int_vec : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            NotUniqueStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyVec
            {
                not_unique_std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: NoPayloadParameters
            { no_payload_parameters : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: NonExistingPrimaryKeys
            { non_existing_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            {
                non_existing_primary_keys : _, rollback_error : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            {
                primary_key_from_row : _, rollback_error : _, code_occurence :
                _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: CommitFailed
            { commit_failed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: QueryAndRollbackFailed
            { query_error : _, rollback_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteManyErrorNamed {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryDeleteManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_many<'a>(
    server_location: &str,
    parameters: DeleteManyParameters,
) -> Result<std::vec::Vec<postgresql_crud::StdPrimitiveI64>, TryDeleteManyErrorNamed> {
    let payload = match serde_json::to_string(&DeleteManyPayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryDeleteManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1266,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/delete_many", server_location,);
    let future = reqwest::Client::new()
        .delete(&url)
        .header(postgresql_crud::COMMIT, git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryDeleteManyErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2142,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            return Err(TryDeleteManyErrorNamed::FailedToGetResponseText {
                reqwest: e,
                status_code,
                headers,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2071,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::OK {
        match serde_json::from_str::<TryDeleteManyResponseVariantsTvfrr200Ok>(&response_text) {
            Ok(value) => TryDeleteManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryDeleteManyResponseVariantsTvfrr404NotFound>(&response_text)
        {
            Ok(value) => TryDeleteManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match serde_json::from_str::<TryDeleteManyResponseVariantsTvfrr408RequestTimeout>(
            &response_text,
        ) {
            Ok(value) => TryDeleteManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryDeleteManyResponseVariantsTvfrr400BadRequest>(
            &response_text,
        ) {
            Ok(value) => TryDeleteManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        return Err(TryDeleteManyErrorNamed::UnexpectedStatusCode {
            status_code,
            headers,
            response_text_result:
                crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(
                    response_text,
                ),
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2036,
                    column: 13,
                }),
            ),
        });
    };
    match std::vec::Vec::<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>::try_from(
        variants,
    ) {
        Ok(value) => Ok(value
            .into_iter()
            .map(|element| postgresql_crud::StdPrimitiveI64::from(element))
            .collect()),
        Err(e) => {
            return Err(TryDeleteManyErrorNamed::ExpectedType {
                expected_type: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1998,
                        column: 13,
                    }),
                ),
            });
        }
    }
}
#[utoipa ::
path(delete, path = "/dogs/delete_many", operation_id = "/dogs/delete_many",
tag = "dogs",
request_body(content = DeleteManyPayloadWithSerializeDeserialize, description
= "dogs delete_many payload", content_type = "application/json"),
responses((status = 200, description = "ok", body =
TryDeleteManyResponseVariantsTvfrr200Ok, content_type = "application/json"),
(status = 500, description = "internal server error", body =
TryDeleteManyResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryDeleteManyResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryDeleteManyResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryDeleteManyResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn delete_many<'a>(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<DeleteManyPayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = DeleteManyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                DeleteManyPayloadWithSerializeDeserialize,
                TryDeleteManyResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => DeleteManyPayload::from(value),
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        if let (None, None, None, None) = (
            &parameters.payload.std_primitive_bool_as_postgresql_bool,
            &parameters.payload.std_primitive_i16_as_postgresql_small_int,
            &parameters.payload.std_primitive_i32_as_postgresql_int,
            &parameters
                .payload
                .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        ) {
            return TryDeleteManyResponseVariants::NoPayloadFields {
                no_payload_fields: std::string::String::from("no payload fields"),
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/check_for_none.rs",
                        ),
                        line: 49,
                        column: 13,
                    }),
                ),
            };
        }
        match (
            &parameters.payload.std_primitive_bool_as_postgresql_bool,
            &parameters.payload.std_primitive_i16_as_postgresql_small_int,
            &parameters.payload.std_primitive_i32_as_postgresql_int,
            &parameters
                .payload
                .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        ) {
            (
                None,
                None,
                None,
                Some(std_primitive_i64_as_postgresql_big_serial_not_null_primary_key),
            ) => {
                let not_unique_primary_keys = {
                    let mut vec = std::vec::Vec::with_capacity(
                        std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.len(),
                    );
                    let mut not_unique_primary_keys = std::vec::Vec::with_capacity(
                        std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.len(),
                    );
                    for element in std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
                        let handle = element;
                        match vec.contains(&handle) {
                            true => {
                                not_unique_primary_keys.push(element.clone());
                            }
                            false => {
                                vec.push(element);
                            }
                        }
                    }
                    not_unique_primary_keys
                };
                if let false = not_unique_primary_keys.is_empty() {
                    let e = TryDeleteMany::NotUniquePrimaryKeys {
                        not_unique_primary_keys,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1659,
                                column: 13,
                            }),
                        ),
                    };
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryDeleteManyResponseVariants::from(e);
                }
                let expected_updated_primary_keys = {
                    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                        .iter()
                        .map(|element| element.clone())
                        .collect::<std::vec::Vec<postgresql_crud::StdPrimitiveI64>>()
                };
                let binded_query = {
                    let query_string = {
                        "delete from dogs where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key in (select unnest($1)) returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"
                    };
                    println!("{}", query_string);
                    let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                    query = query.bind(
                        std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                            .into_iter()
                            .map(|element| element.clone().into_inner())
                            .collect::<std::vec::Vec<std::primitive::i64>>(),
                    );
                    query
                };
                let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryDeleteMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                };
                let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryDeleteMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                };
                let mut postgres_transaction = match {
                    use sqlx::Acquire;
                    pg_connection.begin()
                }
                .await
                {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryDeleteMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                };
                let results_vec = {
                    let mut results_vec =
                        std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
                    let mut option_error: Option<sqlx::Error> = None;
                    {
                        let mut rows = binded_query.fetch(postgres_transaction.as_mut());
                        while let (Some(Some(row)), None) = (
                            match {
                                use futures::TryStreamExt;
                                rows.try_next()
                            }
                            .await
                            {
                                Ok(value) => Some(value),
                                Err(e) => {
                                    option_error = Some(e);
                                    None
                                }
                            },
                            &option_error,
                        ) {
                            results_vec.push(row);
                        }
                    }
                    if let Some(e) = option_error {
                        match postgres_transaction.rollback().await {
                            Ok(_) => {
                                let e = TryDeleteMany::from(e);
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                            Err(rollback_error) => {
                                let e = TryDeleteMany :: QueryAndRollbackFailed
                                {
                                    query_error : e, rollback_error, code_occurence :
                                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                        line : 37, column : 13,
                                    })),
                                } ;
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                        }
                    }
                    results_vec
                };
                let primary_key_vec = {
                    let mut primary_key_vec =
                        std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
                    for element in results_vec {
                        match primary_key_try_from_sqlx_row(&element) {
                            Ok(primary_key) => {
                                primary_key_vec.push(primary_key);
                            }
                            Err(e) => match postgres_transaction.rollback().await {
                                Ok(_) => {
                                    let e = TryDeleteMany::from(e);
                                    error_occurence_lib::error_log::ErrorLog::error_log(
                                        &e,
                                        app_state.as_ref(),
                                    );
                                    return TryDeleteManyResponseVariants::from(e);
                                }
                                Err(rollback_error) => {
                                    let e = TryDeleteMany :: PrimaryKeyFromRowAndFailedRollback
                                    {
                                        primary_key_from_row : e, rollback_error, code_occurence :
                                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                                        new(file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                            line : 52, column : 13,
                                        })),
                                    } ;
                                    error_occurence_lib::error_log::ErrorLog::error_log(
                                        &e,
                                        app_state.as_ref(),
                                    );
                                    return TryDeleteManyResponseVariants::from(e);
                                }
                            },
                        }
                    }
                    primary_key_vec
                };
                {
                    let non_existing_primary_keys = {
                        let len = expected_updated_primary_keys.len();
                        expected_updated_primary_keys.into_iter().fold(
                            std::vec::Vec::with_capacity(len),
                            |mut acc, element| {
                                if let false = primary_key_vec.contains(&element) {
                                    acc.push(element);
                                }
                                acc
                            },
                        )
                    };
                    if let false = non_existing_primary_keys.is_empty() {
                        match postgres_transaction.rollback().await {
                            Ok(_) => {
                                let e = TryDeleteMany :: NonExistingPrimaryKeys
                                {
                                    non_existing_primary_keys, code_occurence :
                                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                        line : 67, column : 13,
                                    })),
                                } ;
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                            Err(e) => {
                                let e = TryDeleteMany ::
                                NonExistingPrimaryKeysAndFailedRollback
                                {
                                    non_existing_primary_keys, rollback_error : e,
                                    code_occurence : error_occurence_lib :: code_occurence ::
                                    CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                        line : 81, column : 13,
                                    })),
                                } ;
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                        }
                    }
                }
                match postgres_transaction.commit().await {
                    Ok(_) => TryDeleteManyResponseVariants::Desirable(
                        primary_key_vec
                            .into_iter()
                            .map(|element| {
                                postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                                    element,
                                )
                            })
                            .collect(),
                    ),
                    Err(e) => {
                        let e = TryDeleteMany :: CommitFailed
                        {
                            commit_failed : e, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_string(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                line : 96, column : 13,
                            })),
                        } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        TryDeleteManyResponseVariants::from(e)
                    }
                }
            }
            _ => {
                if let Some(std_primitive_i64_as_postgresql_big_serial_not_null_primary_key) =
                    &parameters
                        .payload
                        .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                {
                    let not_unique_primary_keys = {
                        let mut vec = std::vec::Vec::with_capacity(
                            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.len(),
                        );
                        let mut not_unique_primary_keys = std::vec::Vec::with_capacity(
                            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.len(),
                        );
                        for element in
                            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                        {
                            let handle = element;
                            match vec.contains(&handle) {
                                true => {
                                    not_unique_primary_keys.push(element.clone());
                                }
                                false => {
                                    vec.push(element);
                                }
                            }
                        }
                        not_unique_primary_keys
                    };
                    if let false = not_unique_primary_keys.is_empty() {
                        let e = TryDeleteMany::NotUniquePrimaryKeys {
                            not_unique_primary_keys,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_string(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1659,
                                    column: 13,
                                }),
                            ),
                        };
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                }
                let std_primitive_bool_as_postgresql_bool_handle = match parameters
                    .payload
                    .std_primitive_bool_as_postgresql_bool
                {
                    Some(value) => {
                        let is_unique = {
                            let mut vec = std::vec::Vec::with_capacity(value.len());
                            let mut is_unique = true;
                            for element in &value {
                                match vec.contains(&element) {
                                    true => {
                                        is_unique = false;
                                        break;
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            is_unique
                        };
                        match is_unique {
                            true => Some(value),
                            false => {
                                let not_unique_std_primitive_bool_as_postgresql_bool_vec = {
                                    let mut vec = std::vec::Vec::with_capacity(value.len());
                                    let mut not_unique_std_primitive_bool_as_postgresql_bool_vec =
                                        std::vec::Vec::with_capacity(value.len());
                                    for element in value {
                                        match vec.contains(&element) {
                                            true => {
                                                not_unique_std_primitive_bool_as_postgresql_bool_vec.push(element)
                                                ;
                                            }
                                            false => {
                                                vec.push(element);
                                            }
                                        }
                                    }
                                    not_unique_std_primitive_bool_as_postgresql_bool_vec.into_iter().map(|
                                    element |
                                    postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
                                    :: from(element)).collect()
                                };
                                let e = TryDeleteMany ::
                                NotUniqueStdPrimitiveBoolAsPostgresqlBoolVec
                                {
                                    not_unique_std_primitive_bool_as_postgresql_bool_vec,
                                    code_occurence : error_occurence_lib :: code_occurence ::
                                    CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 6848, column : 33,
                                    })),
                                } ;
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                        }
                    }
                    None => None,
                };
                let std_primitive_i16_as_postgresql_small_int_handle = match parameters
                    .payload
                    .std_primitive_i16_as_postgresql_small_int
                {
                    Some(value) => {
                        let is_unique = {
                            let mut vec = std::vec::Vec::with_capacity(value.len());
                            let mut is_unique = true;
                            for element in &value {
                                match vec.contains(&element) {
                                    true => {
                                        is_unique = false;
                                        break;
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            is_unique
                        };
                        match is_unique {
                            true => Some(value),
                            false => {
                                let not_unique_std_primitive_i16_as_postgresql_small_int_vec = {
                                    let mut vec = std::vec::Vec::with_capacity(value.len());
                                    let mut
                                    not_unique_std_primitive_i16_as_postgresql_small_int_vec =
                                        std::vec::Vec::with_capacity(value.len());
                                    for element in value {
                                        match vec.contains(&element) {
                                            true => {
                                                not_unique_std_primitive_i16_as_postgresql_small_int_vec.push(element)
                                                ;
                                            }
                                            false => {
                                                vec.push(element);
                                            }
                                        }
                                    }
                                    not_unique_std_primitive_i16_as_postgresql_small_int_vec.into_iter().map(|
                                    element |
                                    postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize
                                    :: from(element)).collect()
                                };
                                let e = TryDeleteMany ::
                                NotUniqueStdPrimitiveI16AsPostgresqlSmallIntVec
                                {
                                    not_unique_std_primitive_i16_as_postgresql_small_int_vec,
                                    code_occurence : error_occurence_lib :: code_occurence ::
                                    CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 6848, column : 33,
                                    })),
                                } ;
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                        }
                    }
                    None => None,
                };
                let std_primitive_i32_as_postgresql_int_handle = match parameters
                    .payload
                    .std_primitive_i32_as_postgresql_int
                {
                    Some(value) => {
                        let is_unique = {
                            let mut vec = std::vec::Vec::with_capacity(value.len());
                            let mut is_unique = true;
                            for element in &value {
                                match vec.contains(&element) {
                                    true => {
                                        is_unique = false;
                                        break;
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            is_unique
                        };
                        match is_unique {
                            true => Some(value),
                            false => {
                                let not_unique_std_primitive_i32_as_postgresql_int_vec = {
                                    let mut vec = std::vec::Vec::with_capacity(value.len());
                                    let mut not_unique_std_primitive_i32_as_postgresql_int_vec =
                                        std::vec::Vec::with_capacity(value.len());
                                    for element in value {
                                        match vec.contains(&element) {
                                            true => {
                                                not_unique_std_primitive_i32_as_postgresql_int_vec
                                                    .push(element);
                                            }
                                            false => {
                                                vec.push(element);
                                            }
                                        }
                                    }
                                    not_unique_std_primitive_i32_as_postgresql_int_vec.into_iter().map(|
                                    element |
                                    postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize
                                    :: from(element)).collect()
                                };
                                let e = TryDeleteMany ::
                                NotUniqueStdPrimitiveI32AsPostgresqlIntVec
                                {
                                    not_unique_std_primitive_i32_as_postgresql_int_vec,
                                    code_occurence : error_occurence_lib :: code_occurence ::
                                    CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 6848, column : 33,
                                    })),
                                } ;
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                        }
                    }
                    None => None,
                };
                let query_string = {
                    format!
                    ("delete from dogs where {} returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                    {
                        let mut increment : u64 = 0 ; let mut additional_parameters
                        = std :: string :: String :: default() ; if let Some(value)
                        = & std_primitive_bool_as_postgresql_bool_handle
                        {
                            for element in value
                            {
                                match postgresql_crud :: BindQuery ::
                                try_increment(element, & mut increment)
                                {
                                    Ok(_) =>
                                    {
                                        let handle = format!
                                        ("std_primitive_bool_as_postgresql_bool = ${increment}") ;
                                        match additional_parameters.is_empty()
                                        {
                                            true => { additional_parameters.push_str(& handle) ; },
                                            false =>
                                            {
                                                additional_parameters.push_str(& format! (" AND {handle}"))
                                                ;
                                            },
                                        }
                                    }, Err(e) =>
                                    {
                                        return TryDeleteManyResponseVariants :: BindQuery
                                        {
                                            bind_query : e.into_serialize_deserialize_version(),
                                            code_occurence : error_occurence_lib :: code_occurence ::
                                            CodeOccurence ::
                                            new(file! ().to_string(), line! (), column! (),
                                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                            {
                                                file : std :: string :: String ::
                                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line : 1505, column : 13,
                                            }))
                                        } ;
                                    },
                                }
                            }
                        } if let Some(value) = &
                        std_primitive_i16_as_postgresql_small_int_handle
                        {
                            for element in value
                            {
                                match postgresql_crud :: BindQuery ::
                                try_increment(element, & mut increment)
                                {
                                    Ok(_) =>
                                    {
                                        let handle = format!
                                        ("std_primitive_i16_as_postgresql_small_int = ${increment}")
                                        ; match additional_parameters.is_empty()
                                        {
                                            true => { additional_parameters.push_str(& handle) ; },
                                            false =>
                                            {
                                                additional_parameters.push_str(& format! (" AND {handle}"))
                                                ;
                                            },
                                        }
                                    }, Err(e) =>
                                    {
                                        return TryDeleteManyResponseVariants :: BindQuery
                                        {
                                            bind_query : e.into_serialize_deserialize_version(),
                                            code_occurence : error_occurence_lib :: code_occurence ::
                                            CodeOccurence ::
                                            new(file! ().to_string(), line! (), column! (),
                                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                            {
                                                file : std :: string :: String ::
                                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line : 1505, column : 13,
                                            }))
                                        } ;
                                    },
                                }
                            }
                        } if let Some(value) = &
                        std_primitive_i32_as_postgresql_int_handle
                        {
                            for element in value
                            {
                                match postgresql_crud :: BindQuery ::
                                try_increment(element, & mut increment)
                                {
                                    Ok(_) =>
                                    {
                                        let handle = format!
                                        ("std_primitive_i32_as_postgresql_int = ${increment}") ;
                                        match additional_parameters.is_empty()
                                        {
                                            true => { additional_parameters.push_str(& handle) ; },
                                            false =>
                                            {
                                                additional_parameters.push_str(& format! (" AND {handle}"))
                                                ;
                                            },
                                        }
                                    }, Err(e) =>
                                    {
                                        return TryDeleteManyResponseVariants :: BindQuery
                                        {
                                            bind_query : e.into_serialize_deserialize_version(),
                                            code_occurence : error_occurence_lib :: code_occurence ::
                                            CodeOccurence ::
                                            new(file! ().to_string(), line! (), column! (),
                                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                            {
                                                file : std :: string :: String ::
                                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line : 1505, column : 13,
                                            }))
                                        } ;
                                    },
                                }
                            }
                        } if let
                        Some(std_primitive_i64_as_postgresql_big_serial_not_null_primary_key)
                        = &
                        parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                        {
                            if let false = additional_parameters.is_empty()
                            { additional_parameters.push_str(" and") ; }
                            additional_parameters.push_str(& format!
                            (" std_primitive_i64_as_postgresql_big_serial_not_null_primary_key in ({})",
                            {
                                let mut additional_parameters = std :: string :: String ::
                                default() ; for element in
                                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                                {
                                    match postgresql_crud :: BindQuery ::
                                    try_increment(element, & mut increment,)
                                    {
                                        Ok(_) =>
                                        {
                                            additional_parameters.push_str(& format! ("${increment},"))
                                            ;
                                        } Err(e) =>
                                        {
                                            return TryDeleteManyResponseVariants :: BindQuery
                                            {
                                                bind_query : e.into_serialize_deserialize_version(),
                                                code_occurence : error_occurence_lib :: code_occurence ::
                                                CodeOccurence ::
                                                new(file! ().to_string(), line! (), column! (),
                                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                                {
                                                    file : std :: string :: String ::
                                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                    line : 1505, column : 13,
                                                }))
                                            } ;
                                        }
                                    }
                                } additional_parameters.pop() ; additional_parameters
                            })) ;
                        } additional_parameters
                    })
                };
                println!("{}", query_string);
                let binded_query = {
                    let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                    if let Some(value) = std_primitive_bool_as_postgresql_bool_handle {
                        for element in value {
                            query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
                        }
                    }
                    if let Some(value) = std_primitive_i16_as_postgresql_small_int_handle {
                        for element in value {
                            query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
                        }
                    }
                    if let Some(value) = std_primitive_i32_as_postgresql_int_handle {
                        for element in value {
                            query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
                        }
                    }
                    if let Some(std_primitive_i64_as_postgresql_big_serial_not_null_primary_key) =
                        parameters
                            .payload
                            .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                    {
                        for element in
                            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                        {
                            query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
                        }
                    }
                    query
                };
                let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryDeleteMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                };
                let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryDeleteMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                };
                let mut rows = binded_query.fetch(pg_connection.as_mut());
                let mut vec_values = std::vec::Vec::new();
                while let Some(row) = {
                    match {
                        use futures::TryStreamExt;
                        rows.try_next()
                    }
                    .await
                    {
                        Ok(value) => value,
                        Err(e) => {
                            let e = TryDeleteMany::from(e);
                            error_occurence_lib::error_log::ErrorLog::error_log(
                                &e,
                                app_state.as_ref(),
                            );
                            return TryDeleteManyResponseVariants::from(e);
                        }
                    }
                } {
                    match {
                        use sqlx::Row;
                        row.try_get::<std::primitive::i64, &str>(
                            "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                        )
                    } {
                        Ok(value) => {
                            vec_values.push(
                                postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                                    postgresql_crud::StdPrimitiveI64(value),
                                ),
                            );
                        }
                        Err(e) => {
                            let e = TryDeleteMany ::
                            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
                            {
                                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                                : e, code_occurence : error_occurence_lib :: code_occurence
                                :: CodeOccurence ::
                                new(file! ().to_string(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1732, column : 13,
                                })),
                            } ;
                            error_occurence_lib::error_log::ErrorLog::error_log(
                                &e,
                                app_state.as_ref(),
                            );
                            return TryDeleteManyResponseVariants::from(e);
                        }
                    }
                }
                TryDeleteManyResponseVariants::Desirable(vec_values)
            }
        }
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryDeleteMany
{
    fn from(
        value: crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, utoipa :: ToSchema)]
pub struct DeleteOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DeleteOnePayloadWithSerializeDeserialize {
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
}
impl std::convert::From<DeleteOnePayloadWithSerializeDeserialize> for DeleteOnePayload {
    fn from(value: DeleteOnePayloadWithSerializeDeserialize) -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
                postgresql_crud::StdPrimitiveI64::from(
                    value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                ),
        }
    }
}
impl std::convert::From<DeleteOnePayload> for DeleteOnePayloadWithSerializeDeserialize {
    fn from(value: DeleteOnePayload) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        }
    }
}
#[derive(Debug)]
pub struct DeleteOneParameters {
    pub payload: DeleteOnePayload,
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryDeleteOne {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_display]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteOneResponseVariants {
    Desirable(postgresql_crud::StdPrimitiveI64WithSerializeDeserialize),
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOne> for TryDeleteOneResponseVariants {
    fn from(value: TryDeleteOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryDeleteOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryDeleteOneWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryDeleteOneWithSerializeDeserialize :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryDeleteOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryDeleteOneResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryDeleteOneResponseVariants) -> Self {
        match value
        {
            TryDeleteOneResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: Database
            { database : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteOneResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryDeleteOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryDeleteOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: OK, TryDeleteOneResponseVariants
            :: Decode { decode : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteOneResponseVariantsTvfrr200Ok {
    Desirable(postgresql_crud::StdPrimitiveI64WithSerializeDeserialize),
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr200Ok> for TryDeleteOneResponseVariants {
    fn from(value: TryDeleteOneResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryDeleteOneResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteOneResponseVariantsTvfrr400BadRequest {
    TypeNotFound {
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        commit_not_equal: std::string::String,
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        commit_to_str_conversion: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr400BadRequest>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value {
            TryDeleteOneResponseVariantsTvfrr400BadRequest::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            TryDeleteOneResponseVariantsTvfrr400BadRequest::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            TryDeleteOneResponseVariantsTvfrr400BadRequest::JsonDataError {
                json_data_error,
                code_occurence,
            } => Self::JsonDataError {
                json_data_error,
                code_occurence,
            },
            TryDeleteOneResponseVariantsTvfrr400BadRequest::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            } => Self::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            },
            TryDeleteOneResponseVariantsTvfrr400BadRequest::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            } => Self::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            },
            TryDeleteOneResponseVariantsTvfrr400BadRequest::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            } => Self::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            },
            TryDeleteOneResponseVariantsTvfrr400BadRequest::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            } => Self::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            },
            TryDeleteOneResponseVariantsTvfrr400BadRequest::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            } => Self::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr408RequestTimeout>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryDeleteOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr404NotFound>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryDeleteOneResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteOneResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr500InternalServerError>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }
        }
    }
}
impl TryFrom<TryDeleteOneResponseVariants>
    for postgresql_crud::StdPrimitiveI64WithSerializeDeserialize
{
    type Error = TryDeleteOneWithSerializeDeserialize;
    fn try_from(value: TryDeleteOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryDeleteOneResponseVariants :: Desirable(i) => Ok(i),
            TryDeleteOneResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryDeleteOneResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Database
            { database, code_occurence }), TryDeleteOneResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Io
            { io, code_occurence }), TryDeleteOneResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryDeleteOneResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryDeleteOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryDeleteOneResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryDeleteOneResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryDeleteOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryDeleteOneResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryDeleteOneResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryDeleteOneWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryDeleteOneResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryDeleteOneResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryDeleteOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryDeleteOneResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryDeleteOneResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }), TryDeleteOneResponseVariants
            :: JsonSyntaxError { json_syntax_error, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryDeleteOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryDeleteOneResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }), TryDeleteOneResponseVariants
            :: UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryDeleteOneResponseVariants
            ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }), TryDeleteOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryDeleteOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryDeleteOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            NoCommitExtractorHeader { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryDeleteOneWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub enum TryDeleteOneStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServerTvfrr500InternalServerError,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryDeleteOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryDeleteOneResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            } TryDeleteOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteOneErrorNamed {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryDeleteOneWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_one<'a>(
    server_location: &str,
    parameters: DeleteOneParameters,
) -> Result<postgresql_crud::StdPrimitiveI64, TryDeleteOneErrorNamed> {
    let payload = match serde_json::to_string(&DeleteOnePayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryDeleteOneErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1266,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/delete_one", server_location);
    let future = reqwest::Client::new()
        .delete(&url)
        .header(postgresql_crud::COMMIT, git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryDeleteOneErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2142,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            return Err(TryDeleteOneErrorNamed::FailedToGetResponseText {
                reqwest: e,
                status_code,
                headers,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2071,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::OK {
        match serde_json::from_str::<TryDeleteOneResponseVariantsTvfrr200Ok>(&response_text) {
            Ok(value) => TryDeleteOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryDeleteOneResponseVariantsTvfrr404NotFound>(&response_text) {
            Ok(value) => TryDeleteOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match serde_json::from_str::<TryDeleteOneResponseVariantsTvfrr408RequestTimeout>(
            &response_text,
        ) {
            Ok(value) => TryDeleteOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryDeleteOneResponseVariantsTvfrr400BadRequest>(&response_text)
        {
            Ok(value) => TryDeleteOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteOneErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2108,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        return Err(TryDeleteOneErrorNamed::UnexpectedStatusCode {
            status_code,
            headers,
            response_text_result:
                crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(
                    response_text,
                ),
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2036,
                    column: 13,
                }),
            ),
        });
    };
    match postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::try_from(variants) {
        Ok(value) => Ok(postgresql_crud::StdPrimitiveI64::from(value)),
        Err(e) => {
            return Err(TryDeleteOneErrorNamed::ExpectedType {
                expected_type: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1998,
                        column: 13,
                    }),
                ),
            });
        }
    }
}
#[utoipa ::
path(delete, path = "/dogs/delete_one", operation_id = "/dogs/delete_one", tag
= "dogs",
request_body(content = DeleteOnePayloadWithSerializeDeserialize, description =
"dogs delete_one payload", content_type = "application/json"),
responses((status = 200, description = "ok", body =
TryDeleteOneResponseVariantsTvfrr200Ok, content_type = "application/json"),
(status = 500, description = "internal server error", body =
TryDeleteOneResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryDeleteOneResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryDeleteOneResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryDeleteOneResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn delete_one<'a>(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<DeleteOnePayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = DeleteOneParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                DeleteOnePayloadWithSerializeDeserialize,
                TryDeleteOneResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => DeleteOnePayload::from(value),
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        let query_string = {
            let mut query = format!("delete from dogs where");
            query.push_str(&format!(
                " std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = $1"
            ));
            query.push_str(&format!(
                " returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"
            ));
            query
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            query = postgresql_crud::BindQuery::bind_value_to_query(
                parameters
                    .payload
                    .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                query,
            );
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let e = TryDeleteOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryDeleteOneResponseVariants::from(e);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let e = TryDeleteOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryDeleteOneResponseVariants::from(e);
            }
        };
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(value) => match {
                use sqlx::Row;
                value.try_get::<std::primitive::i64, &str>(
                    "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                )
            } {
                Ok(value) => TryDeleteOneResponseVariants::Desirable(
                    postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                        postgresql_crud::StdPrimitiveI64(value),
                    ),
                ),
                Err(e) => {
                    let e = TryDeleteOne ::
                    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
                    {
                        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1732, column : 13,
                        })),
                    } ;
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryDeleteOneResponseVariants::from(e);
                }
            },
            Err(e) => {
                let e = TryDeleteOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryDeleteOneResponseVariants::from(e);
            }
        }
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryDeleteOne
{
    fn from(
        value: crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
