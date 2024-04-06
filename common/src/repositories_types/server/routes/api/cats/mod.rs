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
//HERE end
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum TryReadOneErrorNamed {
//     SerdeJsonToString {
//         #[eo_display]
//         serde_json_to_string: serde_json::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     ExpectedType {
//         #[eo_display_with_serialize_deserialize]
//         expected_type: TryReadOneWithSerializeDeserialize,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     UnexpectedStatusCode {
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_foreign_type]
//         response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_display]
//         serde: serde_json::Error,
//         #[eo_display]
//         status_code: http::StatusCode,
//         #[eo_display_foreign_type]
//         headers: reqwest::header::HeaderMap,
//         #[eo_display_with_serialize_deserialize]
//         response_text: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_display_foreign_type]
//         reqwest: reqwest::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     //modification
//     ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayload {
//         #[eo_error_occurence]
//         read_one_payload_with_serialize_deserialize_try_from_read_one_payload: ReadOnePayloadWithSerializeDeserializeTryFromReadOnePayloadErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     //
// }
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
impl std::convert::From<ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamed>
    for TryReadOne
{
    fn from(
        value: ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamed,
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

    