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
    // pub id: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey, //todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX - check it
    // pub name: postgresql_crud::StdStringStringAsPostgresqlVarcharNotNull,
    // pub color: postgresql_crud::StdStringStringAsPostgresqlVarcharNotNull,

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
    // pub std_string_string_as_postgresql_name: postgresql_crud::StdStringStringAsPostgresqlName,
    // pub std_string_string_as_postgresql_name_not_null: postgresql_crud::StdStringStringAsPostgresqlNameNotNull,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        std::option::Option<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_primitive_bool_as_postgresql_bool: std::option::Option<
        postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_primitive_i16_as_postgresql_small_int: std::option::Option<
        postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_primitive_i32_as_postgresql_int: std::option::Option<
        postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
    >,
}
impl std::convert::From<Dog> for DogOptions {
    fn from(value: Dog) -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: Some(
                postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                    value
                        .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                        .0,
                ),
            ),
            std_primitive_bool_as_postgresql_bool: Some(
                postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                    value.std_primitive_bool_as_postgresql_bool.0,
                ),
            ),
            std_primitive_i16_as_postgresql_small_int: Some(
                postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                    value.std_primitive_i16_as_postgresql_small_int.0,
                ),
            ),
            std_primitive_i32_as_postgresql_int: Some(
                postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                    value.std_primitive_i32_as_postgresql_int.0,
                ),
            ),
        }
    }
}
//HERE start
// #[derive(Debug)]
// pub struct DogStdPrimitiveBoolAsPostgresqlBool {
//     pub std_primitive_bool_as_postgresql_bool:
//         postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveI16AsPostgresqlSmallInt {
//     pub std_primitive_i16_as_postgresql_small_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveI32AsPostgresqlInt {
//     pub std_primitive_i32_as_postgresql_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey {
//     pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
//         postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlInt {
//     pub std_primitive_bool_as_postgresql_bool:
//         postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
//     pub std_primitive_i32_as_postgresql_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     pub std_primitive_bool_as_postgresql_bool:
//         postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
//     pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
//         postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallInt {
//     pub std_primitive_bool_as_postgresql_bool:
//         postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
//     pub std_primitive_i16_as_postgresql_small_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt {
//     pub std_primitive_i16_as_postgresql_small_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
//     pub std_primitive_i32_as_postgresql_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     pub std_primitive_i16_as_postgresql_small_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
//     pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
//         postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey {
//     pub std_primitive_i32_as_postgresql_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
//     pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
//         postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     pub std_primitive_bool_as_postgresql_bool:
//         postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
//     pub std_primitive_i32_as_postgresql_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
//     pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
//         postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt
// {
//     pub std_primitive_bool_as_postgresql_bool:
//         postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
//     pub std_primitive_i16_as_postgresql_small_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
//     pub std_primitive_i32_as_postgresql_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     pub std_primitive_bool_as_postgresql_bool:
//         postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
//     pub std_primitive_i16_as_postgresql_small_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
//     pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
//         postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     pub std_primitive_i16_as_postgresql_small_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
//     pub std_primitive_i32_as_postgresql_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
//     pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
//         postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
// }
// #[derive(Debug)]
// pub struct DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     pub std_primitive_bool_as_postgresql_bool:
//         postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
//     pub std_primitive_i16_as_postgresql_small_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
//     pub std_primitive_i32_as_postgresql_int:
//         postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
//     pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
//         postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
// }
//HEREend
//HEREstart
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveBoolAsPostgresqlBoolTryFromDogOptionsErrorNamed {
//     StdPrimitiveBoolAsPostgresqlBoolIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_bool_as_postgresql_bool_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::TryFrom<DogOptions> for DogStdPrimitiveBoolAsPostgresqlBool {
//     type Error = DogStdPrimitiveBoolAsPostgresqlBoolTryFromDogOptionsErrorNamed;
//     fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
//         let std_primitive_bool_as_postgresql_bool =
//             match value.std_primitive_bool_as_postgresql_bool {
//                 Some(value) => value,
//                 None => {
//                     return Err(Self::Error::StdPrimitiveBoolAsPostgresqlBoolIsNone {
//                         std_primitive_bool_as_postgresql_bool_is_none: std::string::String::from(
//                             "std_primitive_bool_as_postgresql_bool is None",
//                         ),
//                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                             file!().to_string(),
//                             line!(),
//                             column!(),
//                             Some(error_occurence_lib::code_occurence::MacroOccurence {
//                                 file: std::string::String::from(
//                                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                                 ),
//                                 line: 584,
//                                 column: 29,
//                             }),
//                         ),
//                     });
//                 }
//             };
//         Ok(Self {
//             std_primitive_bool_as_postgresql_bool,
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveI16AsPostgresqlSmallIntTryFromDogOptionsErrorNamed {
//     StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i16_as_postgresql_small_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::TryFrom<DogOptions> for DogStdPrimitiveI16AsPostgresqlSmallInt {
//     type Error = DogStdPrimitiveI16AsPostgresqlSmallIntTryFromDogOptionsErrorNamed;
//     fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
//         let std_primitive_i16_as_postgresql_small_int = match value
//             .std_primitive_i16_as_postgresql_small_int
//         {
//             Some(value) => value,
//             None => {
//                 return Err(Self::Error::StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//                     std_primitive_i16_as_postgresql_small_int_is_none: std::string::String::from(
//                         "std_primitive_i16_as_postgresql_small_int is None",
//                     ),
//                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                         file!().to_string(),
//                         line!(),
//                         column!(),
//                         Some(error_occurence_lib::code_occurence::MacroOccurence {
//                             file: std::string::String::from(
//                                 "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                             ),
//                             line: 584,
//                             column: 29,
//                         }),
//                     ),
//                 });
//             }
//         };
//         Ok(Self {
//             std_primitive_i16_as_postgresql_small_int,
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveI32AsPostgresqlIntTryFromDogOptionsErrorNamed {
//     StdPrimitiveI32AsPostgresqlIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i32_as_postgresql_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::TryFrom<DogOptions> for DogStdPrimitiveI32AsPostgresqlInt {
//     type Error = DogStdPrimitiveI32AsPostgresqlIntTryFromDogOptionsErrorNamed;
//     fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
//         let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
//             Some(value) => value,
//             None => {
//                 return Err(Self::Error::StdPrimitiveI32AsPostgresqlIntIsNone {
//                     std_primitive_i32_as_postgresql_int_is_none: std::string::String::from(
//                         "std_primitive_i32_as_postgresql_int is None",
//                     ),
//                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                         file!().to_string(),
//                         line!(),
//                         column!(),
//                         Some(error_occurence_lib::code_occurence::MacroOccurence {
//                             file: std::string::String::from(
//                                 "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                             ),
//                             line: 584,
//                             column: 29,
//                         }),
//                     ),
//                 });
//             }
//         };
//         Ok(Self {
//             std_primitive_i32_as_postgresql_int,
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed {
//     StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none:
//             std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::TryFrom<DogOptions>
//     for DogStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     type Error =
//         DogStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed;
//     fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
//         let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = match value
//             .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         {
//             Some(value) => value,
//             None => {
//                 return
//                 Err(Self :: Error ::
//                 StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone
//                 {
//                     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none
//                     : std :: string :: String ::
//                     from("std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 521, column : 41,
//                     })),
//                 }) ;
//             }
//         };
//         Ok(Self {
//             std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlIntTryFromDogOptionsErrorNamed
// {
//     StdPrimitiveBoolAsPostgresqlBoolIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_bool_as_postgresql_bool_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI32AsPostgresqlIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i32_as_postgresql_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::TryFrom<DogOptions>
//     for DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlInt
// {
//     type Error =
//     DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlIntTryFromDogOptionsErrorNamed
//     ;
//     fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
//         let std_primitive_bool_as_postgresql_bool =
//             match value.std_primitive_bool_as_postgresql_bool {
//                 Some(value) => value,
//                 None => {
//                     return Err(Self::Error::StdPrimitiveBoolAsPostgresqlBoolIsNone {
//                         std_primitive_bool_as_postgresql_bool_is_none: std::string::String::from(
//                             "std_primitive_bool_as_postgresql_bool is None",
//                         ),
//                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                             file!().to_string(),
//                             line!(),
//                             column!(),
//                             Some(error_occurence_lib::code_occurence::MacroOccurence {
//                                 file: std::string::String::from(
//                                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                                 ),
//                                 line: 584,
//                                 column: 29,
//                             }),
//                         ),
//                     });
//                 }
//             };
//         let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
//             Some(value) => value,
//             None => {
//                 return Err(Self::Error::StdPrimitiveI32AsPostgresqlIntIsNone {
//                     std_primitive_i32_as_postgresql_int_is_none: std::string::String::from(
//                         "std_primitive_i32_as_postgresql_int is None",
//                     ),
//                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                         file!().to_string(),
//                         line!(),
//                         column!(),
//                         Some(error_occurence_lib::code_occurence::MacroOccurence {
//                             file: std::string::String::from(
//                                 "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                             ),
//                             line: 584,
//                             column: 29,
//                         }),
//                     ),
//                 });
//             }
//         };
//         Ok(Self {
//             std_primitive_bool_as_postgresql_bool,
//             std_primitive_i32_as_postgresql_int,
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
// {
//     StdPrimitiveBoolAsPostgresqlBoolIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_bool_as_postgresql_bool_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none:
//             std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::TryFrom<DogOptions>
//     for DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     type Error =
//     DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
//     ;
//     fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
//         let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = match value
//             .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         {
//             Some(value) => value,
//             None => {
//                 return
//                 Err(Self :: Error ::
//                 StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone
//                 {
//                     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none
//                     : std :: string :: String ::
//                     from("std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 521, column : 41,
//                     })),
//                 }) ;
//             }
//         };
//         let std_primitive_bool_as_postgresql_bool =
//             match value.std_primitive_bool_as_postgresql_bool {
//                 Some(value) => value,
//                 None => {
//                     return Err(Self::Error::StdPrimitiveBoolAsPostgresqlBoolIsNone {
//                         std_primitive_bool_as_postgresql_bool_is_none: std::string::String::from(
//                             "std_primitive_bool_as_postgresql_bool is None",
//                         ),
//                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                             file!().to_string(),
//                             line!(),
//                             column!(),
//                             Some(error_occurence_lib::code_occurence::MacroOccurence {
//                                 file: std::string::String::from(
//                                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                                 ),
//                                 line: 584,
//                                 column: 29,
//                             }),
//                         ),
//                     });
//                 }
//             };
//         Ok(Self {
//             std_primitive_bool_as_postgresql_bool,
//             std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntTryFromDogOptionsErrorNamed
// {
//     StdPrimitiveBoolAsPostgresqlBoolIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_bool_as_postgresql_bool_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i16_as_postgresql_small_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::TryFrom<DogOptions>
//     for DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallInt
// {
//     type Error =
//     DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntTryFromDogOptionsErrorNamed
//     ;
//     fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
//         let std_primitive_bool_as_postgresql_bool =
//             match value.std_primitive_bool_as_postgresql_bool {
//                 Some(value) => value,
//                 None => {
//                     return Err(Self::Error::StdPrimitiveBoolAsPostgresqlBoolIsNone {
//                         std_primitive_bool_as_postgresql_bool_is_none: std::string::String::from(
//                             "std_primitive_bool_as_postgresql_bool is None",
//                         ),
//                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                             file!().to_string(),
//                             line!(),
//                             column!(),
//                             Some(error_occurence_lib::code_occurence::MacroOccurence {
//                                 file: std::string::String::from(
//                                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                                 ),
//                                 line: 584,
//                                 column: 29,
//                             }),
//                         ),
//                     });
//                 }
//             };
//         let std_primitive_i16_as_postgresql_small_int = match value
//             .std_primitive_i16_as_postgresql_small_int
//         {
//             Some(value) => value,
//             None => {
//                 return Err(Self::Error::StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//                     std_primitive_i16_as_postgresql_small_int_is_none: std::string::String::from(
//                         "std_primitive_i16_as_postgresql_small_int is None",
//                     ),
//                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                         file!().to_string(),
//                         line!(),
//                         column!(),
//                         Some(error_occurence_lib::code_occurence::MacroOccurence {
//                             file: std::string::String::from(
//                                 "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                             ),
//                             line: 584,
//                             column: 29,
//                         }),
//                     ),
//                 });
//             }
//         };
//         Ok(Self {
//             std_primitive_bool_as_postgresql_bool,
//             std_primitive_i16_as_postgresql_small_int,
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntTryFromDogOptionsErrorNamed
// {
//     StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i16_as_postgresql_small_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI32AsPostgresqlIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i32_as_postgresql_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::TryFrom<DogOptions>
//     for DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt
// {
//     type Error =
//     DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntTryFromDogOptionsErrorNamed
//     ;
//     fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
//         let std_primitive_i16_as_postgresql_small_int = match value
//             .std_primitive_i16_as_postgresql_small_int
//         {
//             Some(value) => value,
//             None => {
//                 return Err(Self::Error::StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//                     std_primitive_i16_as_postgresql_small_int_is_none: std::string::String::from(
//                         "std_primitive_i16_as_postgresql_small_int is None",
//                     ),
//                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                         file!().to_string(),
//                         line!(),
//                         column!(),
//                         Some(error_occurence_lib::code_occurence::MacroOccurence {
//                             file: std::string::String::from(
//                                 "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                             ),
//                             line: 584,
//                             column: 29,
//                         }),
//                     ),
//                 });
//             }
//         };
//         let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
//             Some(value) => value,
//             None => {
//                 return Err(Self::Error::StdPrimitiveI32AsPostgresqlIntIsNone {
//                     std_primitive_i32_as_postgresql_int_is_none: std::string::String::from(
//                         "std_primitive_i32_as_postgresql_int is None",
//                     ),
//                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                         file!().to_string(),
//                         line!(),
//                         column!(),
//                         Some(error_occurence_lib::code_occurence::MacroOccurence {
//                             file: std::string::String::from(
//                                 "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                             ),
//                             line: 584,
//                             column: 29,
//                         }),
//                     ),
//                 });
//             }
//         };
//         Ok(Self {
//             std_primitive_i16_as_postgresql_small_int,
//             std_primitive_i32_as_postgresql_int,
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
// {
//     StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i16_as_postgresql_small_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none:
//             std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::TryFrom<DogOptions>
//     for DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     type Error =
//     DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
//     ;
//     fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
//         let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = match value
//             .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         {
//             Some(value) => value,
//             None => {
//                 return
//                 Err(Self :: Error ::
//                 StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone
//                 {
//                     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none
//                     : std :: string :: String ::
//                     from("std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 521, column : 41,
//                     })),
//                 }) ;
//             }
//         };
//         let std_primitive_i16_as_postgresql_small_int = match value
//             .std_primitive_i16_as_postgresql_small_int
//         {
//             Some(value) => value,
//             None => {
//                 return Err(Self::Error::StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//                     std_primitive_i16_as_postgresql_small_int_is_none: std::string::String::from(
//                         "std_primitive_i16_as_postgresql_small_int is None",
//                     ),
//                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                         file!().to_string(),
//                         line!(),
//                         column!(),
//                         Some(error_occurence_lib::code_occurence::MacroOccurence {
//                             file: std::string::String::from(
//                                 "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                             ),
//                             line: 584,
//                             column: 29,
//                         }),
//                     ),
//                 });
//             }
//         };
//         Ok(Self {
//             std_primitive_i16_as_postgresql_small_int,
//             std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
// {
//     StdPrimitiveI32AsPostgresqlIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i32_as_postgresql_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none:
//             std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std::convert::TryFrom<DogOptions>
//     for DogStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     type Error =
//     DogStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
//     ;
//     fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
//         let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = match value
//             .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         {
//             Some(value) => value,
//             None => {
//                 return
//                 Err(Self :: Error ::
//                 StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone
//                 {
//                     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none
//                     : std :: string :: String ::
//                     from("std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 521, column : 41,
//                     })),
//                 }) ;
//             }
//         };
//         let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
//             Some(value) => value,
//             None => {
//                 return Err(Self::Error::StdPrimitiveI32AsPostgresqlIntIsNone {
//                     std_primitive_i32_as_postgresql_int_is_none: std::string::String::from(
//                         "std_primitive_i32_as_postgresql_int is None",
//                     ),
//                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
//                         file!().to_string(),
//                         line!(),
//                         column!(),
//                         Some(error_occurence_lib::code_occurence::MacroOccurence {
//                             file: std::string::String::from(
//                                 "postgresql_crud/generate_postgresql_crud/src/lib.rs",
//                             ),
//                             line: 584,
//                             column: 29,
//                         }),
//                     ),
//                 });
//             }
//         };
//         Ok(Self {
//             std_primitive_i32_as_postgresql_int,
//             std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
// {
//     StdPrimitiveBoolAsPostgresqlBoolIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_bool_as_postgresql_bool_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI32AsPostgresqlIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i32_as_postgresql_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none:
//             std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std :: convert :: TryFrom < DogOptions > for
// DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     type Error =
//     DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
//     ; fn try_from(value : DogOptions) -> Result < Self, Self :: Error >
//     {
//         let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
//         match
//         value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error ::
//                 StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone
//                 {
//                     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none
//                     : std :: string :: String ::
//                     from("std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 521, column : 41,
//                     })),
//                 }) ;
//             }
//         } ; let std_primitive_bool_as_postgresql_bool = match
//         value.std_primitive_bool_as_postgresql_bool
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveBoolAsPostgresqlBoolIsNone
//                 {
//                     std_primitive_bool_as_postgresql_bool_is_none : std ::
//                     string :: String ::
//                     from("std_primitive_bool_as_postgresql_bool is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ; let std_primitive_i32_as_postgresql_int = match
//         value.std_primitive_i32_as_postgresql_int
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveI32AsPostgresqlIntIsNone
//                 {
//                     std_primitive_i32_as_postgresql_int_is_none : std :: string
//                     :: String ::
//                     from("std_primitive_i32_as_postgresql_int is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ;
//         Ok(Self
//         {
//             std_primitive_bool_as_postgresql_bool,
//             std_primitive_i32_as_postgresql_int,
//             std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntTryFromDogOptionsErrorNamed
// {
//     StdPrimitiveBoolAsPostgresqlBoolIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_bool_as_postgresql_bool_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i16_as_postgresql_small_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI32AsPostgresqlIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i32_as_postgresql_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std :: convert :: TryFrom < DogOptions > for
// DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt
// {
//     type Error =
//     DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntTryFromDogOptionsErrorNamed
//     ; fn try_from(value : DogOptions) -> Result < Self, Self :: Error >
//     {
//         let std_primitive_bool_as_postgresql_bool = match
//         value.std_primitive_bool_as_postgresql_bool
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveBoolAsPostgresqlBoolIsNone
//                 {
//                     std_primitive_bool_as_postgresql_bool_is_none : std ::
//                     string :: String ::
//                     from("std_primitive_bool_as_postgresql_bool is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ; let std_primitive_i16_as_postgresql_small_int = match
//         value.std_primitive_i16_as_postgresql_small_int
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveI16AsPostgresqlSmallIntIsNone
//                 {
//                     std_primitive_i16_as_postgresql_small_int_is_none : std ::
//                     string :: String ::
//                     from("std_primitive_i16_as_postgresql_small_int is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ; let std_primitive_i32_as_postgresql_int = match
//         value.std_primitive_i32_as_postgresql_int
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveI32AsPostgresqlIntIsNone
//                 {
//                     std_primitive_i32_as_postgresql_int_is_none : std :: string
//                     :: String ::
//                     from("std_primitive_i32_as_postgresql_int is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ;
//         Ok(Self
//         {
//             std_primitive_bool_as_postgresql_bool,
//             std_primitive_i16_as_postgresql_small_int,
//             std_primitive_i32_as_postgresql_int
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
// {
//     StdPrimitiveBoolAsPostgresqlBoolIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_bool_as_postgresql_bool_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i16_as_postgresql_small_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none:
//             std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std :: convert :: TryFrom < DogOptions > for
// DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     type Error =
//     DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
//     ; fn try_from(value : DogOptions) -> Result < Self, Self :: Error >
//     {
//         let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
//         match
//         value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error ::
//                 StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone
//                 {
//                     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none
//                     : std :: string :: String ::
//                     from("std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 521, column : 41,
//                     })),
//                 }) ;
//             }
//         } ; let std_primitive_bool_as_postgresql_bool = match
//         value.std_primitive_bool_as_postgresql_bool
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveBoolAsPostgresqlBoolIsNone
//                 {
//                     std_primitive_bool_as_postgresql_bool_is_none : std ::
//                     string :: String ::
//                     from("std_primitive_bool_as_postgresql_bool is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ; let std_primitive_i16_as_postgresql_small_int = match
//         value.std_primitive_i16_as_postgresql_small_int
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveI16AsPostgresqlSmallIntIsNone
//                 {
//                     std_primitive_i16_as_postgresql_small_int_is_none : std ::
//                     string :: String ::
//                     from("std_primitive_i16_as_postgresql_small_int is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ;
//         Ok(Self
//         {
//             std_primitive_bool_as_postgresql_bool,
//             std_primitive_i16_as_postgresql_small_int,
//             std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
// {
//     StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i16_as_postgresql_small_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI32AsPostgresqlIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i32_as_postgresql_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none:
//             std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std :: convert :: TryFrom < DogOptions > for
// DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     type Error =
//     DogStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
//     ; fn try_from(value : DogOptions) -> Result < Self, Self :: Error >
//     {
//         let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
//         match
//         value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error ::
//                 StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone
//                 {
//                     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none
//                     : std :: string :: String ::
//                     from("std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 521, column : 41,
//                     })),
//                 }) ;
//             }
//         } ; let std_primitive_i16_as_postgresql_small_int = match
//         value.std_primitive_i16_as_postgresql_small_int
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveI16AsPostgresqlSmallIntIsNone
//                 {
//                     std_primitive_i16_as_postgresql_small_int_is_none : std ::
//                     string :: String ::
//                     from("std_primitive_i16_as_postgresql_small_int is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ; let std_primitive_i32_as_postgresql_int = match
//         value.std_primitive_i32_as_postgresql_int
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveI32AsPostgresqlIntIsNone
//                 {
//                     std_primitive_i32_as_postgresql_int_is_none : std :: string
//                     :: String ::
//                     from("std_primitive_i32_as_postgresql_int is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ;
//         Ok(Self
//         {
//             std_primitive_i16_as_postgresql_small_int,
//             std_primitive_i32_as_postgresql_int,
//             std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         })
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
// {
//     StdPrimitiveBoolAsPostgresqlBoolIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_bool_as_postgresql_bool_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI16AsPostgresqlSmallIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i16_as_postgresql_small_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI32AsPostgresqlIntIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i32_as_postgresql_int_is_none: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone {
//         #[eo_display_with_serialize_deserialize]
//         std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none:
//             std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl std :: convert :: TryFrom < DogOptions > for
// DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
// {
//     type Error =
//     DogStdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyTryFromDogOptionsErrorNamed
//     ; fn try_from(value : DogOptions) -> Result < Self, Self :: Error >
//     {
//         let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
//         match
//         value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error ::
//                 StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeyIsNone
//                 {
//                     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none
//                     : std :: string :: String ::
//                     from("std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_is_none is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 521, column : 41,
//                     })),
//                 }) ;
//             }
//         } ; let std_primitive_bool_as_postgresql_bool = match
//         value.std_primitive_bool_as_postgresql_bool
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveBoolAsPostgresqlBoolIsNone
//                 {
//                     std_primitive_bool_as_postgresql_bool_is_none : std ::
//                     string :: String ::
//                     from("std_primitive_bool_as_postgresql_bool is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ; let std_primitive_i16_as_postgresql_small_int = match
//         value.std_primitive_i16_as_postgresql_small_int
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveI16AsPostgresqlSmallIntIsNone
//                 {
//                     std_primitive_i16_as_postgresql_small_int_is_none : std ::
//                     string :: String ::
//                     from("std_primitive_i16_as_postgresql_small_int is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ; let std_primitive_i32_as_postgresql_int = match
//         value.std_primitive_i32_as_postgresql_int
//         {
//             Some(value) => value, None =>
//             {
//                 return
//                 Err(Self :: Error :: StdPrimitiveI32AsPostgresqlIntIsNone
//                 {
//                     std_primitive_i32_as_postgresql_int_is_none : std :: string
//                     :: String ::
//                     from("std_primitive_i32_as_postgresql_int is None"),
//                     code_occurence : error_occurence_lib :: code_occurence ::
//                     CodeOccurence ::
//                     new(file! ().to_string(), line! (), column! (),
//                     Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                     {
//                         file : std :: string :: String ::
//                         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                         line : 584, column : 29,
//                     })),
//                 }) ;
//             }
//         } ;
//         Ok(Self
//         {
//             std_primitive_bool_as_postgresql_bool,
//             std_primitive_i16_as_postgresql_small_int,
//             std_primitive_i32_as_postgresql_int,
//             std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//         })
//     }
// }
//HEREend
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    enum_extension ::EnumExtension,
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
        write!(f, "{}", Self::to_snake_case(self))
    }
}
//modification
impl crate::server::postgres::generate_query::GenerateQuery for DogColumn {
    fn generate_query(&self) -> std::string::String {
        match self {
            Self::StdPrimitiveBoolAsPostgresqlBool => std::string::String::from("std_primitive_bool_as_postgresql_bool"),
            Self::StdPrimitiveI16AsPostgresqlSmallInt => std::string::String::from("std_primitive_i16_as_postgresql_small_int"),
            Self::StdPrimitiveI32AsPostgresqlInt => std::string::String::from("std_primitive_i32_as_postgresql_int"),
            Self::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => std::string::String::from("std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"),
        }
    }
}
//modification
impl crate::server::postgres::generate_query::GenerateQuery for std::vec::Vec<DogColumn> {
    fn generate_query(&self) -> std::string::String {
        let mut value = self.iter().fold(std::string::String::from(""), |mut acc, element| {
            acc += &crate::server::postgres::generate_query::GenerateQuery::generate_query(element);
            acc += ",";
            acc
        });
        value.pop();
        value
    }
}
//HEREstart
// #[derive(
//     Debug,
//     serde :: Serialize,
//     serde :: Deserialize,
//     Clone,
//     strum_macros
// :: Display,
// )]
// pub enum DogColumnSelect {
//     StdPrimitiveBoolAsPostgresqlBool,
//     StdPrimitiveI16AsPostgresqlSmallInt,
//     StdPrimitiveI32AsPostgresqlInt,
//     StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
//     StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlInt,
//     StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
//     StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallInt,
//     StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt,
//     StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
//     StdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
//     StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
//     StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt,
//     StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
//     StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
//     StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
// }
//HEREend
//HEREstart
// impl crate::server::postgres::generate_query::GenerateQuery for DogColumnSelect {
//     fn generate_query(&self) -> std::string::String {
//         match self
//         {
//             Self :: StdPrimitiveBoolAsPostgresqlBool => std :: string ::
//             String :: from("std_primitive_bool_as_postgresql_bool"), Self ::
//             StdPrimitiveI16AsPostgresqlSmallInt => std :: string :: String ::
//             from("std_primitive_i16_as_postgresql_small_int"), Self ::
//             StdPrimitiveI32AsPostgresqlInt => std :: string :: String ::
//             from("std_primitive_i32_as_postgresql_int"), Self ::
//             StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => std ::
//             string :: String ::
//             from("std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"),
//             Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlInt =>
//             std :: string :: String ::
//             from("std_primitive_bool_as_postgresql_bool,std_primitive_i32_as_postgresql_int"),
//             Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
//             => std :: string :: String ::
//             from("std_primitive_bool_as_postgresql_bool,std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"),
//             Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallInt
//             => std :: string :: String ::
//             from("std_primitive_bool_as_postgresql_bool,std_primitive_i16_as_postgresql_small_int"),
//             Self ::
//             StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt
//             => std :: string :: String ::
//             from("std_primitive_i16_as_postgresql_small_int,std_primitive_i32_as_postgresql_int"),
//             Self ::
//             StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
//             => std :: string :: String ::
//             from("std_primitive_i16_as_postgresql_small_int,std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"),
//             Self ::
//             StdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
//             => std :: string :: String ::
//             from("std_primitive_i32_as_postgresql_int,std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"),
//             Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
//             => std :: string :: String ::
//             from("std_primitive_bool_as_postgresql_bool,std_primitive_i32_as_postgresql_int,std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"),
//             Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt
//             => std :: string :: String ::
//             from("std_primitive_bool_as_postgresql_bool,std_primitive_i16_as_postgresql_small_int,std_primitive_i32_as_postgresql_int"),
//             Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
//             => std :: string :: String ::
//             from("std_primitive_bool_as_postgresql_bool,std_primitive_i16_as_postgresql_small_int,std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"),
//             Self ::
//             StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
//             => std :: string :: String ::
//             from("std_primitive_i16_as_postgresql_small_int,std_primitive_i32_as_postgresql_int,std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"),
//             Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
//             => std :: string :: String ::
//             from("std_primitive_bool_as_postgresql_bool,std_primitive_i16_as_postgresql_small_int,std_primitive_i32_as_postgresql_int,std_primitive_i64_as_postgresql_big_serial_not_null_primary_key")
//         }
//     }
// }
//
//HEREend
// impl std::default::Default for DogColumnSelect {
//     fn default() -> Self {
//         Self ::
//         StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
//     }
// }
// impl std::convert::From<std::option::Option<Self>> for DogColumnSelect {
//     fn from(option_value: std::option::Option<Self>) -> Self {
//         match option_value {
//             Some(value) => value,
//             None => Self::default(),
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DogColumnSelectFromStrErrorNamed {
//     NotCorrect {
//         #[eo_display_with_serialize_deserialize]
//         not_correct_value: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         supported_values: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
//HERE start
// impl std::str::FromStr for DogColumnSelect {
//     type Err = DogColumnSelectFromStrErrorNamed;
//     fn from_str(value: &str) -> Result<Self, Self::Err> {
//         match value
//         {
//             "StdPrimitiveBoolAsPostgresqlBool" =>
//             Ok(Self :: StdPrimitiveBoolAsPostgresqlBool),
//             "StdPrimitiveI16AsPostgresqlSmallInt" =>
//             Ok(Self :: StdPrimitiveI16AsPostgresqlSmallInt),
//             "StdPrimitiveI32AsPostgresqlInt" =>
//             Ok(Self :: StdPrimitiveI32AsPostgresqlInt),
//             "StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey" =>
//             Ok(Self :: StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
//             "StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlInt"
//             =>
//             Ok(Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlInt),
//             "StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey"
//             =>
//             Ok(Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
//             "StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallInt"
//             =>
//             Ok(Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallInt),
//             "StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt"
//             =>
//             Ok(Self ::
//             StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt),
//             "StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey"
//             =>
//             Ok(Self ::
//             StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
//             "StdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey"
//             =>
//             Ok(Self ::
//             StdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
//             "StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey"
//             =>
//             Ok(Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
//             "StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt"
//             =>
//             Ok(Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt),
//             "StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey"
//             =>
//             Ok(Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
//             "StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey"
//             =>
//             Ok(Self ::
//             StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
//             "StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey"
//             =>
//             Ok(Self ::
//             StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey),
//             _ =>
//             Err(Self :: Err :: NotCorrect
//             {
//                 not_correct_value : std :: string :: String :: from(value),
//                 supported_values : std :: string :: String ::
//                 from("\"StdPrimitiveBoolAsPostgresqlBool\",\"StdPrimitiveI16AsPostgresqlSmallInt\",\"StdPrimitiveI32AsPostgresqlInt\",\"StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey\",\"StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlInt\",\"StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey\",\"StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallInt\",\"StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt\",\"StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey\",\"StdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey\",\"StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey\",\"StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlInt\",\"StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey\",\"StdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey\",\"StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey\""),
//                 code_occurence : error_occurence_lib :: code_occurence ::
//                 CodeOccurence ::
//                 new(file! ().to_string(), line! (), column! (),
//                 Some(error_occurence_lib :: code_occurence :: MacroOccurence
//                 {
//                     file : std :: string :: String ::
//                     from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
//                     line : 852, column : 17,
//                 })),
//             }),
//         }
//     }
// }
//HERE end
// impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for DogColumnSelect {
//     fn serde_urlencoded_parameter(self) -> std::string::String {
//         self.to_string()
//     }
// }
//HEREstart

//modification
struct WrapperVecColumn(std::vec::Vec<DogColumn>);
//modification
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
        let mut std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize
        > = None;
        let mut std_primitive_bool_as_postgresql_bool: std::option::Option<
            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        > = None;
        let mut std_primitive_i16_as_postgresql_small_int: std::option::Option<
            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
        > = None;
        let mut std_primitive_i32_as_postgresql_int: std::option::Option<
            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
        > = None;
        todo!()
        
        // Ok(DogOptions {
        //     std_primitive_bool_as_postgresql_bool,
        //     std_primitive_i16_as_postgresql_small_int,
        //     std_primitive_i32_as_postgresql_int,
        //     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        // })
    }
}
//HERE end
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
                                                line : 1278, column : 17,
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
                                            line : 1284, column : 17,
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
                                            line : 1290, column : 17,
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
                                        line : 1296, column : 17,
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
                                line : 1302, column : 17,
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
                                line: 1308,
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
                            line: 1314,
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
                                                line : 1320, column : 17,
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
                                            line : 1326, column : 17,
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
                                            line : 1332, column : 17,
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
                                        line : 1338, column : 17,
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
                                line : 1344, column : 17,
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
                                    line: 1350,
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
// #[test]
// fn dog_emulate_crud_api_usage_test() {
//     async fn find_out_if_it_works() {
//         let api_location = std::string::String::from("http://127.0.0.1:8080");
//         let limit = 1000;
//         let offset = 0;
//         println!("-------trycreate_many start-------");
//         let primary_keys = match try_create_many(
//             &api_location,
//             CreateManyParameters {
//                 payload: CreateManyPayload(vec![CreateManyPayloadElement {
//                     std_primitive_bool_as_postgresql_bool:
//                         postgresql_crud::StdPrimitiveBoolAsPostgresqlBool::default(),
//                     std_primitive_i16_as_postgresql_small_int:
//                         postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt::default(),
//                     std_primitive_i32_as_postgresql_int:
//                         postgresql_crud::StdPrimitiveI32AsPostgresqlInt::default(),
//                 }]),
//             },
//         )
//         .await
//         {
//             Ok(value) => {
//                 println!("{value:#?}");
//                 value
//             }
//             Err(e) => panic!("{}", e),
//         };
//         println!("-------trycreate_many end-------");
//         println!("-------tryread_many start-------");
//         match
//         try_read_many(& api_location, ReadManyParameters
//         {
//             payload : ReadManyPayload
//             {
//                 std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//                 : Some(primary_keys.clone()),
//                 std_primitive_bool_as_postgresql_bool : None,
//                 std_primitive_i16_as_postgresql_small_int : None,
//                 std_primitive_i32_as_postgresql_int : None, select :
//                 DogColumnSelect ::
//                 StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
//                 order_by : crate :: server :: postgres :: order_by :: OrderBy
//                 {
//                     column : DogColumn :: Name, order :
//                     Some(crate :: server :: postgres :: order :: Order :: Desc),
//                 }, limit : crate :: server :: postgres :: postgres_bigint ::
//                 PostgresBigint(limit), offset : crate :: server :: postgres ::
//                 postgres_bigint :: PostgresBigint(offset),
//             }
//         },).await
//         {
//             Ok(value) => { println! ("{value:#?}") ; value }, Err(e) => panic!
//             ("{}", e)
//         } ;
//         println!("-------tryread_many end-------");
//         println!("-------tryupdate_many start-------");
//         match try_update_many(
//             &api_location,
//             UpdateManyParameters {
//                 payload: UpdateManyPayload(
//                     primary_keys
//                         .clone()
//                         .into_iter()
//                         .map(|element| UpdateManyPayloadElement {
//                             std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
//                                 element,
//                             std_primitive_bool_as_postgresql_bool:
//                                 postgresql_crud::StdPrimitiveBoolAsPostgresqlBool::default(),
//                             std_primitive_i16_as_postgresql_small_int:
//                                 postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt::default(),
//                             std_primitive_i32_as_postgresql_int:
//                                 postgresql_crud::StdPrimitiveI32AsPostgresqlInt::default(),
//                         })
//                         .collect(),
//                 ),
//             },
//         )
//         .await
//         {
//             Ok(value) => println!("{value:#?}"),
//             Err(e) => panic!("{}", e),
//         }
//         println!("-------tryupdate_many end-------");
//         println!("-------tryread_many start-------");
//         match
//         try_read_many(& api_location, ReadManyParameters
//         {
//             payload : ReadManyPayload
//             {
//                 std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//                 : Some(primary_keys.clone()),
//                 std_primitive_bool_as_postgresql_bool : None,
//                 std_primitive_i16_as_postgresql_small_int : None,
//                 std_primitive_i32_as_postgresql_int : None, select :
//                 DogColumnSelect ::
//                 StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
//                 order_by : crate :: server :: postgres :: order_by :: OrderBy
//                 {
//                     column : DogColumn :: Name, order :
//                     Some(crate :: server :: postgres :: order :: Order :: Desc),
//                 }, limit : crate :: server :: postgres :: postgres_bigint ::
//                 PostgresBigint(limit), offset : crate :: server :: postgres ::
//                 postgres_bigint :: PostgresBigint(offset),
//             }
//         },).await
//         {
//             Ok(value) => { println! ("{value:#?}") ; value }, Err(e) => panic!
//             ("{}", e)
//         } ;
//         println!("-------tryread_many end-------");
//         println!("-------trydelete_many start-------");
//         match try_delete_many(
//             &api_location,
//             DeleteManyParameters {
//                 payload: DeleteManyPayload {
//                     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: Some(
//                         primary_keys.clone(),
//                     ),
//                     std_primitive_bool_as_postgresql_bool: None,
//                     std_primitive_i16_as_postgresql_small_int: None,
//                     std_primitive_i32_as_postgresql_int: None,
//                 },
//             },
//         )
//         .await
//         {
//             Ok(value) => println!("{value:#?}"),
//             Err(e) => panic!("{}", e),
//         }
//         println!("-------trydelete_many end-------");
//         println!("-------tryread_many start-------");
//         match
//         try_read_many(& api_location, ReadManyParameters
//         {
//             payload : ReadManyPayload
//             {
//                 std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//                 : Some(primary_keys.clone()),
//                 std_primitive_bool_as_postgresql_bool : None,
//                 std_primitive_i16_as_postgresql_small_int : None,
//                 std_primitive_i32_as_postgresql_int : None, select :
//                 DogColumnSelect ::
//                 StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
//                 order_by : crate :: server :: postgres :: order_by :: OrderBy
//                 {
//                     column : DogColumn :: Name, order :
//                     Some(crate :: server :: postgres :: order :: Order :: Desc),
//                 }, limit : crate :: server :: postgres :: postgres_bigint ::
//                 PostgresBigint(limit), offset : crate :: server :: postgres ::
//                 postgres_bigint :: PostgresBigint(offset),
//             }
//         },).await
//         {
//             Ok(value) => { println! ("{value:#?}") ; value }, Err(e) => panic!
//             ("{}", e)
//         } ;
//         println!("-------tryread_many end-------");
//         println!("-------trycreate_one start-------");
//         let primary_key = match try_create_one(
//             &api_location,
//             CreateOneParameters {
//                 payload: CreateOnePayload {
//                     std_primitive_bool_as_postgresql_bool:
//                         postgresql_crud::StdPrimitiveBoolAsPostgresqlBool::default(),
//                     std_primitive_i16_as_postgresql_small_int:
//                         postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt::default(),
//                     std_primitive_i32_as_postgresql_int:
//                         postgresql_crud::StdPrimitiveI32AsPostgresqlInt::default(),
//                 },
//             },
//         )
//         .await
//         {
//             Ok(value) => {
//                 println!("{value:#?}");
//                 value
//             }
//             Err(e) => panic!("{}", e),
//         };
//         println!("-------trycreate_one end-------");
//         println!("-------tryread_one start-------");
//         match
//         try_read_one(& api_location, ReadOneParameters
//         {
//             payload : ReadOnePayload
//             {
//                 std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//                 : primary_key.clone(), select : DogColumnSelect ::
//                 StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
//             }
//         },).await
//         { Ok(value) => println! ("{value:#?}"), Err(e) => panic! ("{}", e) } ;
//         println!("-------tryread_one end-------");
//         println!("-------tryupdate_one start-------");
//         let primary_key = match try_update_one(
//             &api_location,
//             UpdateOneParameters {
//                 payload: UpdateOnePayload {
//                     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: primary_key
//                         .clone(),
//                     std_primitive_bool_as_postgresql_bool: Some(
//                         postgresql_crud::StdPrimitiveBoolAsPostgresqlBool::default(),
//                     ),
//                     std_primitive_i16_as_postgresql_small_int: Some(
//                         postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt::default(),
//                     ),
//                     std_primitive_i32_as_postgresql_int: Some(
//                         postgresql_crud::StdPrimitiveI32AsPostgresqlInt::default(),
//                     ),
//                 },
//             },
//         )
//         .await
//         {
//             Ok(value) => {
//                 println!("{value:#?}");
//                 value
//             }
//             Err(e) => panic!("{}", e),
//         };
//         println!("-------tryupdate_one end-------");
//         println!("-------tryread_one start-------");
//         match
//         try_read_one(& api_location, ReadOneParameters
//         {
//             payload : ReadOnePayload
//             {
//                 std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//                 : primary_key.clone(), select : DogColumnSelect ::
//                 StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
//             }
//         },).await
//         { Ok(value) => println! ("{value:#?}"), Err(e) => panic! ("{}", e) } ;
//         println!("-------tryread_one end-------");
//         println!("-------trydelete_one start-------");
//         match try_delete_one(
//             &api_location,
//             DeleteOneParameters {
//                 payload: DeleteOnePayload {
//                     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: primary_key
//                         .clone(),
//                 },
//             },
//         )
//         .await
//         {
//             Ok(value) => println!("{value:#?}"),
//             Err(e) => panic!("{}", e),
//         }
//         println!("-------trydelete_one end-------");
//         println!("-------tryread_one start-------");
//         match
//         try_read_one(& api_location, ReadOneParameters
//         {
//             payload : ReadOnePayload
//             {
//                 std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
//                 : primary_key.clone(), select : DogColumnSelect ::
//                 StdPrimitiveBoolAsPostgresqlBoolStdPrimitiveI16AsPostgresqlSmallIntStdPrimitiveI32AsPostgresqlIntStdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
//             }
//         },).await
//         { Ok(value) => panic! ("{value:#?}"), Err(e) => println! ("{}", e) } ;
//         println!("-------tryread_one end-------");
//     }
//     match tokio::runtime::Builder::new_multi_thread()
//         .worker_threads(num_cpus::get())
//         .enable_all()
//         .build()
//     {
//         Err(e) => {
//             panic!
//             ("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {:#?}",
//             e)
//         }
//         Ok(runtime) => {
//             runtime.block_on(find_out_if_it_works());
//         }
//     }
// }
//
#[derive(Debug, utoipa :: ToSchema)]
pub struct ReadOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64,
    pub select: std::vec::Vec<DogColumn>,//modification
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct ReadOnePayloadWithSerializeDeserialize {
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
    select: std::vec::Vec<DogColumn>,//modification
}
//modification
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
    NotUniqueColumn {
        #[eo_display_with_serialize_deserialize]
        not_unique_column: DogColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
//modification
impl std::convert::TryFrom<ReadOnePayloadWithSerializeDeserialize> for ReadOnePayload {
    type Error = ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize;//modification
    fn try_from(value: ReadOnePayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let select = {
            let mut vec = std::vec::Vec::with_capacity(4);
            for element in value.select {
                if vec.contains(&element) {
                    return Err(Self::Error::NotUniqueColumn {//modification
                        not_unique_column: element,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1588,
                                column: 13,
                            }),
                        ),
                    });
                }
                else {
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

//modification
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayload {
    NotUniqueColumn {
        #[eo_display_with_serialize_deserialize]
        not_unique_column: DogColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
//modification
impl std::convert::TryFrom<ReadOnePayload> for ReadOnePayloadWithSerializeDeserialize {
    type Error = ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayload;
    fn try_from(value: ReadOnePayload) -> Result<Self, Self::Error> {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let select = {
            let mut vec = std::vec::Vec::with_capacity(4);
            for element in value.select {
                if vec.contains(&element) {
                    return Err(Self::Error::NotUniqueColumn {//modification
                        not_unique_column: element,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1588,
                                column: 13,
                            }),
                        ),
                    });
                }
                else {
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
    //modification
    ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        read_one_payload_try_from_read_one_payload_with_serialize_deserialize: ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    //
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadOneResponseVariants {
    Desirable(DogOptions),
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
    //modification
    ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
        read_one_payload_try_from_read_one_payload_with_serialize_deserialize: ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}
impl std::convert::From<TryReadOne> for TryReadOneResponseVariants {
    fn from(value: TryReadOne) -> Self {
        match value.into_serialize_deserialize_version() {
            TryReadOneWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            TryReadOneWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::JsonDataError {
                json_data_error,
                code_occurence,
            } => Self::JsonDataError {
                json_data_error,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            } => Self::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            } => Self::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::BytesRejection {
                bytes_rejection,
                code_occurence,
            } => Self::BytesRejection {
                bytes_rejection,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            } => Self::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            } => Self::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            },
            TryReadOneWithSerializeDeserialize::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            } => Self::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            },
            //modification
            TryReadOneWithSerializeDeserialize::ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence,
            } => Self::ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence,
            }
        }
    }
}
impl std::convert::From<&TryReadOneResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryReadOneResponseVariants) -> Self {
        match value {
            TryReadOneResponseVariants::Desirable(_) => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Configuration {
                configuration: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Database {
                database: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Io {
                io: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Tls {
                tls: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Protocol {
                protocol: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Decode {
                decode: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::Migrate {
                migrate: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::JsonDataError {
                json_data_error: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::JsonSyntaxError {
                json_syntax_error: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::MissingJsonContentType {
                missing_json_content_type: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::BytesRejection {
                bytes_rejection: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::CommitExtractorNotEqual {
                commit_not_equal: _,
                commit_to_use: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::CommitExtractorToStrConversion {
                commit_to_str_conversion: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            TryReadOneResponseVariants::NoCommitExtractorHeader {
                no_commit_header: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
            //modification
            TryReadOneResponseVariants::ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize: _,
                code_occurence: _,
            } => axum::http::StatusCode::OK,
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
pub enum TryReadOneResponseVariantsTvfrr400BadRequest {
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
impl std::convert::From<TryReadOneResponseVariantsTvfrr400BadRequest>
    for TryReadOneResponseVariants
{
    fn from(value: TryReadOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value {
            TryReadOneResponseVariantsTvfrr400BadRequest::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr400BadRequest::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr400BadRequest::JsonDataError {
                json_data_error,
                code_occurence,
            } => Self::JsonDataError {
                json_data_error,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr400BadRequest::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            } => Self::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr400BadRequest::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            } => Self::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr400BadRequest::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            } => Self::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr400BadRequest::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            } => Self::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            },
            TryReadOneResponseVariantsTvfrr400BadRequest::NoCommitExtractorHeader {
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
        match value {
            TryReadOneResponseVariants::Desirable(i) => Ok(i),
            TryReadOneResponseVariants::Configuration {
                configuration,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            }),
            TryReadOneResponseVariants::Database {
                database,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::Database {
                database,
                code_occurence,
            }),
            TryReadOneResponseVariants::Io { io, code_occurence } => {
                Err(TryReadOneWithSerializeDeserialize::Io { io, code_occurence })
            }
            TryReadOneResponseVariants::Tls {
                tls,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            }),
            TryReadOneResponseVariants::Protocol {
                protocol,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            }),
            TryReadOneResponseVariants::RowNotFound {
                row_not_found,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            }),
            TryReadOneResponseVariants::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            }),
            TryReadOneResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            }),
            TryReadOneResponseVariants::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            }),
            TryReadOneResponseVariants::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            }),
            TryReadOneResponseVariants::Decode {
                decode,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            }),
            TryReadOneResponseVariants::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            }),
            TryReadOneResponseVariants::PoolClosed {
                pool_closed,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            }),
            TryReadOneResponseVariants::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            }),
            TryReadOneResponseVariants::Migrate {
                migrate,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            }),
            TryReadOneResponseVariants::JsonDataError {
                json_data_error,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::JsonDataError {
                json_data_error,
                code_occurence,
            }),
            TryReadOneResponseVariants::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::JsonSyntaxError {
                json_syntax_error,
                code_occurence,
            }),
            TryReadOneResponseVariants::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::MissingJsonContentType {
                missing_json_content_type,
                code_occurence,
            }),
            TryReadOneResponseVariants::BytesRejection {
                bytes_rejection,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::BytesRejection {
                bytes_rejection,
                code_occurence,
            }),
            TryReadOneResponseVariants::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Err(TryReadOneWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            }),
            TryReadOneResponseVariants::CommitExtractorNotEqual {
                commit_not_equal,
                commit_to_use,
                code_occurence,
            } => Err(
                TryReadOneWithSerializeDeserialize::CommitExtractorNotEqual {
                    commit_not_equal,
                    commit_to_use,
                    code_occurence,
                },
            ),
            TryReadOneResponseVariants::CommitExtractorToStrConversion {
                commit_to_str_conversion,
                code_occurence,
            } => Err(
                TryReadOneWithSerializeDeserialize::CommitExtractorToStrConversion {
                    commit_to_str_conversion,
                    code_occurence,
                },
            ),
            TryReadOneResponseVariants::NoCommitExtractorHeader {
                no_commit_header,
                code_occurence,
            } => Err(
                TryReadOneWithSerializeDeserialize::NoCommitExtractorHeader {
                    no_commit_header,
                    code_occurence,
                },
            ),
            //modification
            TryReadOneResponseVariants::ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence,
            } => Err(
                TryReadOneWithSerializeDeserialize::ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
                    read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                    code_occurence,
                },
            ),
            //
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
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryReadOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match &self {
            TryReadOneResponseVariants::Desirable(_) => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Configuration {
                configuration: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Database {
                database: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Io {
                io: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Tls {
                tls: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Protocol {
                protocol: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Decode {
                decode: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::Migrate {
                migrate: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::JsonDataError {
                json_data_error: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::JsonSyntaxError {
                json_syntax_error: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::MissingJsonContentType {
                missing_json_content_type: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::BytesRejection {
                bytes_rejection: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::CommitExtractorNotEqual {
                commit_not_equal: _,
                commit_to_use: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::CommitExtractorToStrConversion {
                commit_to_str_conversion: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            TryReadOneResponseVariants::NoCommitExtractorHeader {
                no_commit_header: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            //modification
            TryReadOneResponseVariants::ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize: _,
                code_occurence: _,
            } => {
                let mut res = axum::Json(self).into_response();
                *res.status_mut() = axum::http::StatusCode::OK;
                res
            }
            //
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
    //modification
    ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayload {
        #[eo_error_occurence]
        read_one_payload_with_serialize_deserialize_try_from_read_one_payload: ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayload,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    //
}
pub async fn try_read_one<'a>(
    server_location: &str,
    parameters: ReadOneParameters,
) -> Result<DogOptions, TryReadOneErrorNamed> {
    //modification
    let payload = match ReadOnePayloadWithSerializeDeserialize::try_from(
        parameters.payload,
    ) {
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
                            line: 1588,
                            column: 13,
                        }),
                    ),
                });
            }
        },
        Err(e) => {
            return Err(TryReadOneErrorNamed::ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayload {
                read_one_payload_with_serialize_deserialize_try_from_read_one_payload: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1588,
                        column: 13,
                    }),
                ),
            });
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
                        line: 2442,
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
                        line: 2371,
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
                            line: 2408,
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
                            line: 2408,
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
                            line: 2408,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match serde_json::from_str::<TryReadOneResponseVariantsTvfrr500InternalServerError>(
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
                            line: 2408,
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
                    line: 2336,
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
                        line: 2298,
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
                        let e = TryReadOne::from(e);
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
            crate :: server :: postgres :: generate_query :: GenerateQuery ::
            generate_query(& select),)
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
            //modification
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
//modification
impl std::convert::From<ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize>
    for TryReadOne
{
    fn from(
        value: ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize,
    ) -> Self {
        //modification
        Self::ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
            read_one_payload_try_from_read_one_payload_with_serialize_deserialize: value,
            //todo is it that need to have two instances of code_occurence? refactor it
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2298,
                    column: 13,
                }),
            ),
        }
    }
}
//

    