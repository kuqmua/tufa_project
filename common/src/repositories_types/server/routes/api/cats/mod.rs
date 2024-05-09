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
    // #[path(crate::server::extractors::commit_extractor::)]
    // enum CommitExtractorCheckErrorNamed {
    //     #[tvfrr_400_bad_request]
    //     CommitExtractorNotEqual {
    //         #[eo_to_std_string_string_serialize_deserialize]
    //         commit_not_equal: std::string::String,
    //         #[eo_to_std_string_string_serialize_deserialize]
    //         commit_to_use: std::string::String,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //     },
    //     #[tvfrr_400_bad_request]
    //     CommitExtractorToStrConversion {
    //         #[eo_to_std_string_string]
    //         commit_to_str_conversion: http::header::ToStrError,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //     },
    //     #[tvfrr_400_bad_request]
    //     NoCommitExtractorHeader {
    //         #[eo_to_std_string_string_serialize_deserialize]
    //         no_commit_header: std::string::String,
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
// pub struct TryCreateManyRouteLogicResponse {
//     status_code: axum::http::StatusCode,
//     body: TryCreateManyRouteLogicResponseVariants,
// }
// impl axum::response::IntoResponse for TryCreateManyRouteLogicResponse {
//     fn into_response(self) -> axum::response::Response {
//         let mut res = axum::Json(self.body).into_response(); 
//         *res.status_mut() = self.status_code;
//         res
//     }
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// pub enum TryCreateManyRouteLogicResponseVariants {
//     //
//     CheckCommit {
//         check_commit: route_validators::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     CheckBodySize {
//         check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     //
//     Desirable(std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>),
//     Configuration {
//         configuration: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Database {
//         database: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Io {
//         io: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Tls {
//         tls: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Protocol {
//         protocol: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     RowNotFound {
//         row_not_found: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     TypeNotFound {
//         type_not_found: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     ColumnIndexOutOfBounds {
//         column_index_out_of_bounds: usize,
//         len: usize,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     ColumnNotFound {
//         column_not_found: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     ColumnDecode {
//         column_decode_index: std::string::String,
//         source_handle: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Decode {
//         decode: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     PoolTimedOut {
//         pool_timed_out: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     PoolClosed {
//         pool_closed: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     WorkerCrashed {
//         worker_crashed: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Migrate {
//         migrate: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     JsonDataError {
//         json_data_error: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     JsonSyntaxError {
//         json_syntax_error: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     MissingJsonContentType {
//         missing_json_content_type: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     BytesRejection {
//         bytes_rejection: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     UnexpectedCase {
//         unexpected_case: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     BindQuery {
//         bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
//     {
//         operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
//             std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// impl std::convert::From<TryCreateManyRouteLogicErrorNamed> for TryCreateManyRouteLogicResponseVariants {
//     fn from(
//         value: TryCreateManyRouteLogicErrorNamed,
//     ) -> Self {
//         match value.into_serialize_deserialize_version() {
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit {
//                 check_commit,
//                 code_occurence,
//             } => Self::CheckCommit {
//                 check_commit: check_commit,
//                 code_occurence,
//             },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize {
//                 check_body_size,
//                 code_occurence,
//             } => Self::CheckBodySize {
//                 check_body_size: check_body_size,
//                 code_occurence,
//             },
//             //
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Configuration {
//                 configuration,
//                 code_occurence,
//             } => Self::Configuration { configuration, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Database {
//                 database,
//                 code_occurence,
//             } => Self::Database { database, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Io {
//                 io,
//                 code_occurence,
//             } => Self::Io { io: io.to_string(), code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Tls {
//                 tls,
//                 code_occurence,
//             } => Self::Tls { tls, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Protocol {
//                 protocol,
//                 code_occurence,
//             } => Self::Protocol { protocol, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::RowNotFound {
//                 row_not_found,
//                 code_occurence,
//             } => Self::RowNotFound { row_not_found, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::TypeNotFound {
//                 type_not_found,
//                 code_occurence,
//             } => Self::TypeNotFound { type_not_found, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
//                 column_index_out_of_bounds,
//                 len,
//                 code_occurence,
//             } => Self::ColumnIndexOutOfBounds { column_index_out_of_bounds, len, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::ColumnNotFound {
//                 column_not_found,
//                 code_occurence,
//             } => Self::ColumnNotFound { column_not_found, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::ColumnDecode {
//                 column_decode_index,
//                 source_handle,
//                 code_occurence,
//             } => Self::ColumnDecode { column_decode_index, source_handle, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Decode {
//                 decode,
//                 code_occurence,
//             } => Self::Decode { decode, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::PoolTimedOut {
//                 pool_timed_out,
//                 code_occurence,
//             } => Self::PoolTimedOut { pool_timed_out, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::PoolClosed {
//                 pool_closed,
//                 code_occurence,
//             } => Self::PoolClosed { pool_closed, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::WorkerCrashed {
//                 worker_crashed,
//                 code_occurence,
//             } => Self::WorkerCrashed { worker_crashed, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Migrate {
//                 migrate,
//                 code_occurence,
//             } => Self::Migrate { migrate: migrate.to_string(), code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::JsonDataError {
//                 json_data_error,
//                 code_occurence,
//             } => Self::JsonDataError { json_data_error: json_data_error.to_string(), code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::JsonSyntaxError {
//                 json_syntax_error,
//                 code_occurence,
//             } => Self::JsonSyntaxError { json_syntax_error: json_syntax_error.to_string(), code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::MissingJsonContentType {
//                 missing_json_content_type,
//                 code_occurence,
//             } => Self::MissingJsonContentType { missing_json_content_type, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::BytesRejection {
//                 bytes_rejection,
//                 code_occurence,
//             } => Self::BytesRejection { bytes_rejection, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedCase {
//                 unexpected_case,
//                 code_occurence,
//             } => Self::UnexpectedCase { unexpected_case, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery {
//                 bind_query,
//                 code_occurence,
//             } => Self::BindQuery { bind_query: bind_query, code_occurence },
//             TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer {
//                 operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
//                 code_occurence,
//             } => Self::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer { 
//                 operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server: operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server.to_string(), 
//                 code_occurence 
//             },
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum TryCreateManyRouteLogicErrorNamed {
//     CheckCommit {
//         #[eo_error_occurence]
//         check_commit: route_validators::check_commit::CheckCommitErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     CheckBodySize {
//         #[eo_error_occurence]
//         check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     //
//     Configuration {
//         #[eo_to_std_string_string_serialize_deserialize]
//         configuration: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Database {
//         #[eo_to_std_string_string_serialize_deserialize]
//         database: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Io {
//         #[eo_to_std_string_string]
//         io: std::io::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Tls {
//         #[eo_to_std_string_string_serialize_deserialize]
//         tls: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Protocol {
//         #[eo_to_std_string_string_serialize_deserialize]
//         protocol: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     RowNotFound {
//         #[eo_to_std_string_string_serialize_deserialize]
//         row_not_found: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     TypeNotFound {
//         #[eo_to_std_string_string_serialize_deserialize]
//         type_not_found: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     ColumnIndexOutOfBounds {
//         #[eo_to_std_string_string_serialize_deserialize]
//         column_index_out_of_bounds: usize,
//         #[eo_to_std_string_string_serialize_deserialize]
//         len: usize,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     ColumnNotFound {
//         #[eo_to_std_string_string_serialize_deserialize]
//         column_not_found: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     ColumnDecode {
//         #[eo_to_std_string_string_serialize_deserialize]
//         column_decode_index: std::string::String,
//         #[eo_to_std_string_string_serialize_deserialize]
//         source_handle: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Decode {
//         #[eo_to_std_string_string_serialize_deserialize]
//         decode: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     PoolTimedOut {
//         #[eo_to_std_string_string_serialize_deserialize]
//         pool_timed_out: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     PoolClosed {
//         #[eo_to_std_string_string_serialize_deserialize]
//         pool_closed: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     WorkerCrashed {
//         #[eo_to_std_string_string_serialize_deserialize]
//         worker_crashed: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Migrate {
//         #[eo_to_std_string_string]
//         migrate: sqlx::migrate::MigrateError,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     JsonDataError {
//         #[eo_to_std_string_string]
//         json_data_error: axum::extract::rejection::JsonDataError,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     JsonSyntaxError {
//         #[eo_to_std_string_string]
//         json_syntax_error: axum::extract::rejection::JsonSyntaxError,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     MissingJsonContentType {
//         #[eo_to_std_string_string_serialize_deserialize]
//         missing_json_content_type: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     BytesRejection {
//         #[eo_to_std_string_string_serialize_deserialize]
//         bytes_rejection: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     UnexpectedCase {
//         #[eo_to_std_string_string_serialize_deserialize]
//         unexpected_case: std::string::String,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     BindQuery {
//         #[eo_error_occurence]
//         bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
//     {
//         #[eo_to_std_string_string]
//         operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
//             sqlx::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     //
// }

// impl std::convert::From<TryCreateManyGeneratedRouteLogicErrorNamed> for TryCreateManyRouteLogicErrorNamed {
//     fn from(value: TryCreateManyGeneratedRouteLogicErrorNamed) -> Self {
//         match value {
//             TryCreateManyGeneratedRouteLogicErrorNamed :: Configuration
//             { configuration, code_occurence } => Self :: Configuration
//             { configuration, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: Database
//             { database, code_occurence } => Self :: Database
//             { database, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: Io { io, code_occurence }
//             => Self :: Io { io, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: Tls
//             { tls, code_occurence } => Self :: Tls { tls, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: Protocol
//             { protocol, code_occurence } => Self :: Protocol
//             { protocol, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: RowNotFound
//             { row_not_found, code_occurence } => Self :: RowNotFound
//             { row_not_found, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: TypeNotFound
//             { type_not_found, code_occurence } => Self :: TypeNotFound
//             { type_not_found, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence } => Self ::
//             ColumnIndexOutOfBounds
//             { column_index_out_of_bounds, len, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: ColumnNotFound
//             { column_not_found, code_occurence } => Self :: ColumnNotFound
//             { column_not_found, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: ColumnDecode
//             { column_decode_index, source_handle, code_occurence } => Self ::
//             ColumnDecode
//             { column_decode_index, source_handle, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: Decode
//             { decode, code_occurence } => Self :: Decode
//             { decode, code_occurence }, TryCreateManyGeneratedRouteLogicErrorNamed
//             :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
//             PoolTimedOut { pool_timed_out, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: PoolClosed
//             { pool_closed, code_occurence } => Self :: PoolClosed
//             { pool_closed, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: WorkerCrashed
//             { worker_crashed, code_occurence } => Self :: WorkerCrashed
//             { worker_crashed, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: Migrate
//             { migrate, code_occurence } => Self :: Migrate
//             { migrate, code_occurence }, TryCreateManyGeneratedRouteLogicErrorNamed
//             :: JsonDataError { json_data_error, code_occurence } => Self ::
//             JsonDataError { json_data_error, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: JsonSyntaxError
//             { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
//             { json_syntax_error, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: MissingJsonContentType
//             { missing_json_content_type, code_occurence } => Self ::
//             MissingJsonContentType
//             { missing_json_content_type, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: BytesRejection
//             { bytes_rejection, code_occurence } => Self :: BytesRejection
//             { bytes_rejection, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: UnexpectedCase
//             { unexpected_case, code_occurence } => Self :: UnexpectedCase
//             { unexpected_case, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed :: BindQuery
//             { bind_query, code_occurence } => Self :: BindQuery
//             { bind_query, code_occurence },
//             TryCreateManyGeneratedRouteLogicErrorNamed ::
//             OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
//             {
//                 operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
//                 code_occurence
//             } => Self ::
//             OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
//             {
//                 operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
//                 code_occurence
//             }
//         }
//     }
// }

pub async fn try_create_many_route_logic(
    app_state: axum::extract::State<
        crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits,
    >,
    request: axum::extract::Request
) -> TryCreateManyRouteLogicResponse {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        // error_occurence_lib::error_log::ErrorLog::error_log(&error, app_state.as_ref());
        eprintln!("{error}");
        return TryCreateManyRouteLogicResponse {
            status_code,
            body: TryCreateManyRouteLogicResponseVariants::from(error),
        };
    }
    let body_bytes = match route_validators::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error) => {
            let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
            let error = TryCreateManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error,
                code_occurence: error_occurence_lib::code_occurence!(),
            };
            // error_occurence_lib::error_log::ErrorLog::error_log(&error, app_state.as_ref());
            eprintln!("{error}");
            return TryCreateManyRouteLogicResponse {
                status_code,
                body: TryCreateManyRouteLogicResponseVariants::from(error),
            };
        }
    };
    todo!()
    // match try_create_many_generated_route_logic(app_state.as_ref(), body_bytes).await {
    //     Ok(value) => {
    //         let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&value);
    //         return TryCreateManyRouteLogicResponse {
    //             status_code,
    //             body: TryCreateManyRouteLogicResponseVariants::Desirable(value.0),
    //         };
    //     },
    //     Err(error) => {
    //         let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&e);
    //         let e = TryCreateManyRouteLogicErrorNamed::from(e);
    ////         error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
    //         eprintln!("{error}");
    //         return TryCreateManyRouteLogicResponse {
    //             status_code,
    //             body: TryCreateManyRouteLogicResponseVariants::from(e),
    //         };
    //     }
    // }
}

/////////////////////////////////////////



#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    // ExpectedType {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     expected_type: TryCreateManyGeneratedRouteLogicErrorNamedWithSerializeDeserialize,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    UnexpectedStatusCode {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },

    //
    // CheckCommit {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     check_commit: route_validators::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // CheckBodySize {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // //
    // Configuration {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     configuration: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // Database {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     database: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // Io {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     io: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // Tls {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     tls: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // Protocol {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     protocol: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // RowNotFound {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     row_not_found: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // TypeNotFound {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     type_not_found: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // ColumnIndexOutOfBounds {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     column_index_out_of_bounds: usize,
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     len: usize,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // ColumnNotFound {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     column_not_found: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // ColumnDecode {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     column_decode_index: std::string::String,
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     source_handle: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // Decode {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     decode: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // PoolTimedOut {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     pool_timed_out: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // PoolClosed {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     pool_closed: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // WorkerCrashed {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     worker_crashed: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // Migrate {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     migrate: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // JsonDataError {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     json_data_error: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // JsonSyntaxError {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     json_syntax_error: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // MissingJsonContentType {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     missing_json_content_type: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // BytesRejection {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     bytes_rejection: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // UnexpectedCase {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     unexpected_case: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // BindQuery {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    // OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
    //         std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
}

pub async fn try_create_many(
    server_location: &str,//todo rename as endpoint location
    parameters: CreateManyParameters,
) -> Result<std::vec::Vec<postgresql_crud::StdPrimitiveI64>, TryCreateManyErrorNamed> {
    let payload = match serde_json::to_string(&CreateManyPayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(error) => {
            return Err(TryCreateManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 796,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{server_location}/dogs/create_many");
    let future = reqwest::Client::new()
        .post(&url)
        .header(
            <naming_constants::Commit as naming_constants::Naming>::snake_case_stringified(),
            git_info::PROJECT_GIT_INFO.commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(error) => {
            return Err(TryCreateManyErrorNamed::Reqwest {
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1678,
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
        Err(error) => {
            return Err(TryCreateManyErrorNamed::FailedToGetResponseText {
                reqwest: error,
                status_code,
                headers,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1607,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryCreateManyRouteLogicResponseVariants>(&response_text) {
        Ok(value) => value,
        Err(error) => {
            return Err(TryCreateManyErrorNamed::DeserializeResponse {
                serde: error,
                status_code,
                headers,
                response_text,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1644,
                        column: 13,
                    }),
                ),
            });
        }
    };
    // match expected_response {
    //     TryCreateManyRouteLogicResponseVariants::Desirable(value) => Ok(
    //         value
    //         .into_iter()
    //         .map(|element| postgresql_crud::StdPrimitiveI64::from(element))
    //         .collect()
    //     ),
    //     //
    //     TryCreateManyRouteLogicResponseVariants::CheckCommit {
    //         check_commit,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::CheckCommit{
    //         check_commit,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::CheckBodySize {
    //         check_body_size,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::CheckBodySize{
    //         check_body_size,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     //
    //     TryCreateManyRouteLogicResponseVariants::Configuration {
    //         configuration,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::Configuration{
    //         configuration,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::Database {
    //         database,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::Database{
    //         database,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::Io {
    //         io,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::Io{
    //         io,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::Tls {
    //         tls,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::Tls{
    //         tls,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::Protocol {
    //         protocol,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::Protocol{
    //         protocol,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::RowNotFound {
    //         row_not_found,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::RowNotFound{
    //         row_not_found,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::TypeNotFound {
    //         type_not_found,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::TypeNotFound{
    //         type_not_found,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::ColumnIndexOutOfBounds {
    //         column_index_out_of_bounds,
    //         len,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::ColumnIndexOutOfBounds{
    //         column_index_out_of_bounds,
    //         len,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::ColumnNotFound {
    //         column_not_found,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::ColumnNotFound{
    //         column_not_found,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::ColumnDecode {
    //         column_decode_index,
    //         source_handle,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::ColumnDecode{
    //         column_decode_index,
    //         source_handle,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::Decode {
    //         decode,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::Decode{
    //         decode,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::PoolTimedOut {
    //         pool_timed_out,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::PoolTimedOut{
    //         pool_timed_out,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::PoolClosed {
    //         pool_closed,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::PoolClosed{
    //         pool_closed,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::WorkerCrashed {
    //         worker_crashed,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::WorkerCrashed{
    //         worker_crashed,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::Migrate {
    //         migrate,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::Migrate{
    //         migrate,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::JsonDataError {
    //         json_data_error,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::JsonDataError{
    //         json_data_error,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::JsonSyntaxError {
    //         json_syntax_error,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::JsonSyntaxError{
    //         json_syntax_error,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::MissingJsonContentType {
    //         missing_json_content_type,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::MissingJsonContentType{
    //         missing_json_content_type,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::BytesRejection {
    //         bytes_rejection,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::BytesRejection{
    //         bytes_rejection,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::UnexpectedCase {
    //         unexpected_case,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::UnexpectedCase{
    //         unexpected_case,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::BindQuery {
    //         bind_query,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::BindQuery{
    //         bind_query,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    //     TryCreateManyRouteLogicResponseVariants::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer {
    //         operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
    //         code_occurence,
    //     } => Err(TryCreateManyErrorNamed::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer{
    //         operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //             file!().to_owned(),
    //             line!(),
    //             column!(),
    //             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                 file: std::string::String::from(
    //                     "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                 ),
    //                 line: 1644,
    //                 column: 13,
    //             }),
    //         ),
    //     }),
    // }
    todo!()
}
 
/////////////////////////////////
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
    Clone, 
    Copy,
    serde :: Serialize,
    serde :: Deserialize,
    enum_extension_lib
:: EnumExtension,
    postgresql_crud :: EnumIter,
    PartialEq,
    Eq,
    from_str ::
FromStr,
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StdPrimitiveBoolAsPostgresqlBool => {
                write!(formatter, "std_primitive_bool_as_postgresql_bool")
            }
            Self::StdPrimitiveI16AsPostgresqlSmallInt => {
                write!(formatter, "std_primitive_i16_as_postgresql_small_int")
            }
            Self::StdPrimitiveI32AsPostgresqlInt => {
                write!(formatter, "std_primitive_i32_as_postgresql_int")
            }
            Self::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => write!(
                formatter,
                "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"
            ),
        }
    }
}
fn generate_query_vec_column(value: &[DogColumn]) -> std::string::String {
    let mut value = value
        .iter()
        .fold(std::string::String::from(""), |mut acc, element| {
            acc += match element {
                DogColumn::StdPrimitiveBoolAsPostgresqlBool => {
                    "std_primitive_bool_as_postgresql_bool"
                }
                DogColumn::StdPrimitiveI16AsPostgresqlSmallInt => {
                    "std_primitive_i16_as_postgresql_small_int"
                }
                DogColumn::StdPrimitiveI32AsPostgresqlInt => "std_primitive_i32_as_postgresql_int",
                DogColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => {
                    "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"
                }
            };
            acc += ",";
            acc
        });
    let _: std::option::Option<std::primitive::char> = value.pop();
    value
}
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
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize > > = None;
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
pub const ALLOW_METHODS: [http::Method; 4] = [
    http::Method::GET,
    http::Method::POST,
    http::Method::PATCH,
    http::Method::DELETE,
];
#[derive(Debug, Clone, Copy)]
pub struct DogColumnReadPermission {
    std_primitive_bool_as_postgresql_bool: std::primitive::bool,
    std_primitive_i16_as_postgresql_small_int: std::primitive::bool,
    std_primitive_i32_as_postgresql_int: std::primitive::bool,
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::primitive::bool,
}
pub use postgresql_crud::StdOptionOptionStdPrimitiveBool;
pub use postgresql_crud::StdOptionOptionStdPrimitiveI16;
pub use postgresql_crud::StdOptionOptionStdPrimitiveI32;
pub use postgresql_crud::StdPrimitiveI64;
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
                order_by : postgresql_crud :: OrderBy
                {
                    column : DogColumn :: Name, order :
                    Some(postgresql_crud :: Order :: Desc),
                }, limit : postgresql_crud::StdPrimitiveI64(limit), offset :
                postgresql_crud::StdPrimitiveI64(offset),
            }
        },).await
        {
            Ok(value) => { println! ("{value:#?}"); value }, Err(e) => panic!
            ("{}", e)
        };
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
                order_by : postgresql_crud :: OrderBy
                {
                    column : DogColumn :: Name, order :
                    Some(postgresql_crud :: Order :: Desc),
                }, limit : postgresql_crud::StdPrimitiveI64(limit), offset :
                postgresql_crud::StdPrimitiveI64(offset),
            }
        },).await
        {
            Ok(value) => { println! ("{value:#?}"); value }, Err(e) => panic!
            ("{}", e)
        };
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
                order_by : postgresql_crud :: OrderBy
                {
                    column : DogColumn :: Name, order :
                    Some(postgresql_crud :: Order :: Desc),
                }, limit : postgresql_crud::StdPrimitiveI64(limit), offset :
                postgresql_crud::StdPrimitiveI64(offset),
            }
        },).await
        {
            Ok(value) => { println! ("{value:#?}"); value }, Err(e) => panic!
            ("{}", e)
        };
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
        { Ok(value) => println! ("{value:#?}"), Err(e) => panic! ("{}", e) };
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
        { Ok(value) => println! ("{value:#?}"), Err(e) => panic! ("{}", e) };
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
        { Ok(value) => panic! ("{value:#?}"), Err(e) => println! ("{}", e) };
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
pub enum TryCreateManyGeneratedRouteLogicErrorNamed {
    Configuration {
        #[eo_to_std_string_string_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_to_std_string_string_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_to_std_string_string]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_to_std_string_string_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_to_std_string_string_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_to_std_string_string_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_to_std_string_string_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_to_std_string_string_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_to_std_string_string_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_to_std_string_string_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_to_std_string_string_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_to_std_string_string_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_to_std_string_string_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_to_std_string_string_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_to_std_string_string_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_to_std_string_string_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_to_std_string_string]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_to_std_string_string]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_to_std_string_string]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_to_std_string_string_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_to_std_string_string_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_to_std_string_string_serialize_deserialize]
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
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl postgresql_crud::GetAxumHttpStatusCode for TryCreateManyGeneratedRouteLogicErrorNamed {
    fn get_axum_http_status_code(&self) -> axum::http::StatusCode {
        match self
        {
            Self :: Configuration { configuration, code_occurence } => axum ::
            http :: StatusCode :: CREATED, Self :: Database
            { database, code_occurence } => axum :: http :: StatusCode ::
            CREATED, Self :: Io { io, code_occurence } => axum :: http ::
            StatusCode :: CREATED, Self :: Tls { tls, code_occurence } => axum
            :: http :: StatusCode :: CREATED, Self :: Protocol
            { protocol, code_occurence } => axum :: http :: StatusCode ::
            CREATED, Self :: RowNotFound { row_not_found, code_occurence } =>
            axum :: http :: StatusCode :: CREATED, Self :: TypeNotFound
            { type_not_found, code_occurence } => axum :: http :: StatusCode
            :: CREATED, Self :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => axum ::
            http :: StatusCode :: CREATED, Self :: ColumnNotFound
            { column_not_found, code_occurence } => axum :: http :: StatusCode
            :: CREATED, Self :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => axum ::
            http :: StatusCode :: CREATED, Self :: Decode
            { decode, code_occurence } => axum :: http :: StatusCode ::
            CREATED, Self :: PoolTimedOut { pool_timed_out, code_occurence }
            => axum :: http :: StatusCode :: CREATED, Self :: PoolClosed
            { pool_closed, code_occurence } => axum :: http :: StatusCode ::
            CREATED, Self :: WorkerCrashed { worker_crashed, code_occurence }
            => axum :: http :: StatusCode :: CREATED, Self :: Migrate
            { migrate, code_occurence } => axum :: http :: StatusCode ::
            CREATED, Self :: JsonDataError { json_data_error, code_occurence }
            => axum :: http :: StatusCode :: CREATED, Self :: JsonSyntaxError
            { json_syntax_error, code_occurence } => axum :: http ::
            StatusCode :: CREATED, Self :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => axum :: http ::
            StatusCode :: CREATED, Self :: BytesRejection
            { bytes_rejection, code_occurence } => axum :: http :: StatusCode
            :: CREATED, Self :: UnexpectedCase
            { unexpected_case, code_occurence } => axum :: http :: StatusCode
            :: CREATED, Self :: BindQuery { bind_query, code_occurence } =>
            axum :: http :: StatusCode :: CREATED, Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => axum :: http :: StatusCode :: CREATED
        }
    }
}
#[derive(Debug)]
pub struct TryCreateManyGeneratedRouteLogicDesirable(
    std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>,
);
impl postgresql_crud::GetAxumHttpStatusCode for TryCreateManyGeneratedRouteLogicDesirable {
    fn get_axum_http_status_code(&self) -> axum::http::StatusCode {
        axum::http::StatusCode::CREATED
    }
}
pub async fn try_create_many_generated_route_logic(
    app_state: &dyn postgresql_crud::CombinationOfTraitsForPostgresqlCrudLogic,
    body_bytes: bytes::Bytes,
) -> Result<TryCreateManyGeneratedRouteLogicDesirable, TryCreateManyGeneratedRouteLogicErrorNamed> {
    let parameters =
        CreateManyParameters {
            payload: match axum::Json::<CreateManyPayloadWithSerializeDeserialize>::from_bytes(
                &body_bytes,
            ) {
                Ok(axum::Json(value)) => CreateManyPayload::from(value),
                Err(e) => {
                    match e {
                        axum::extract::rejection::JsonRejection::JsonDataError(value) => {
                            return
                    Err(TryCreateManyGeneratedRouteLogicErrorNamed ::
                    JsonDataError
                    {
                        json_data_error : value, code_occurence :
                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2678, column : 21,
                        })),
                    });
                        }
                        axum::extract::rejection::JsonRejection::JsonSyntaxError(value) => {
                            return
                    Err(TryCreateManyGeneratedRouteLogicErrorNamed ::
                    JsonSyntaxError
                    {
                        json_syntax_error : value, code_occurence :
                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2684, column : 21,
                        })),
                    });
                        }
                        axum::extract::rejection::JsonRejection::MissingJsonContentType(value) => {
                            return
                    Err(TryCreateManyGeneratedRouteLogicErrorNamed ::
                    MissingJsonContentType
                    {
                        missing_json_content_type : value.to_string(),
                        code_occurence : error_occurence_lib :: code_occurence ::
                        CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2690, column : 21,
                        })),
                    });
                        }
                        axum::extract::rejection::JsonRejection::BytesRejection(value) => {
                            return
                    Err(TryCreateManyGeneratedRouteLogicErrorNamed ::
                    BytesRejection
                    {
                        bytes_rejection : value.to_string(), code_occurence :
                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2696, column : 21,
                        })),
                    });
                        }
                        _ => {
                            return
                    Err(TryCreateManyGeneratedRouteLogicErrorNamed ::
                    UnexpectedCase
                    {
                        unexpected_case : std :: string :: String ::
                        from("Unknown error"), code_occurence : error_occurence_lib
                        :: code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2708, column : 21,
                        })),
                    });
                        }
                    }
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
                return Err(TryCreateManyGeneratedRouteLogicErrorNamed::from(e));
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                return Err(TryCreateManyGeneratedRouteLogicErrorNamed::from(e));
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
                    return Err(TryCreateManyGeneratedRouteLogicErrorNamed::from(e));
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
                Err(error) => {
                    return Err(TryCreateManyGeneratedRouteLogicErrorNamed::from(error));
                }
            }
        }
        Ok(TryCreateManyGeneratedRouteLogicDesirable(vec_values))
    }
}
#[derive(Debug)]
pub struct TryCreateManyRouteLogicResponse {
    status_code: axum::http::StatusCode,
    body: TryCreateManyRouteLogicResponseVariants,
}
impl axum::response::IntoResponse for TryCreateManyRouteLogicResponse {
    fn into_response(self) -> axum::response::Response {
        let mut res = axum::Json(self.body).into_response();
        *res.status_mut() = self.status_code;
        res
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateManyRouteLogicResponseVariants {
    CheckCommit {
        check_commit: route_validators::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckBodySize {
        check_body_size:
            route_validators::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
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
}
impl std::convert::From<TryCreateManyRouteLogicErrorNamed>
    for TryCreateManyRouteLogicResponseVariants
{
    fn from(value: TryCreateManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence, } => Self ::
            CheckCommit { check_commit : check_commit, code_occurence, },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence, } => Self ::
            CheckBodySize
            { check_body_size : check_body_size, code_occurence, },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            TypeNotFound { type_not_found, code_occurence } => Self ::
            TypeNotFound { type_not_found, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence } => Self ::
            ColumnNotFound { column_not_found, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Decode { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            JsonSyntaxError { json_syntax_error, code_occurence } => Self ::
            JsonSyntaxError { json_syntax_error, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
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
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryCreateManyRouteLogicErrorNamed {
    CheckCommit {
        #[eo_error_occurence]
        check_commit: route_validators::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Configuration {
        #[eo_to_std_string_string_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_to_std_string_string_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_to_std_string_string]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_to_std_string_string_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_to_std_string_string_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_to_std_string_string_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_to_std_string_string_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_to_std_string_string_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_to_std_string_string_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_to_std_string_string_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_to_std_string_string_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_to_std_string_string_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_to_std_string_string_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_to_std_string_string_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_to_std_string_string_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_to_std_string_string_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_to_std_string_string]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_to_std_string_string]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_to_std_string_string]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_to_std_string_string_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_to_std_string_string_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_to_std_string_string_serialize_deserialize]
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
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyGeneratedRouteLogicErrorNamed>
    for TryCreateManyRouteLogicErrorNamed
{
    fn from(value: TryCreateManyGeneratedRouteLogicErrorNamed) -> Self {
        match value
        {
            TryCreateManyGeneratedRouteLogicErrorNamed :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: PoolTimedOut
            { pool_timed_out, code_occurence } => Self :: PoolTimedOut
            { pool_timed_out, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyGeneratedRouteLogicErrorNamed ::
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
