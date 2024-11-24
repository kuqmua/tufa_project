#[derive(
    Debug,
    // postgresql_crud::GeneratePostgresqlCrudSecond
)]
// #[postgresql_crud::create_many_additional_error_variants{enum CreateManyAdditionalErrorVariants{}}]
// #[postgresql_crud::create_one_additional_error_variants{enum CreateOneAdditionalErrorVariants{}}]
// #[postgresql_crud::read_many_additional_error_variants{enum ReadManyAdditionalErrorVariants{}}]
// #[postgresql_crud::read_one_additional_error_variants{enum ReadOneAdditionalErrorVariants{}}]
// #[postgresql_crud::update_many_additional_error_variants{enum UpdateManyAdditionalErrorVariants{}}]
// #[postgresql_crud::update_one_additional_error_variants{enum UpdateOneAdditionalErrorVariants{}}]
// #[postgresql_crud::delete_many_additional_error_variants{enum DeleteManyAdditionalErrorVariants{}}]
// #[postgresql_crud::delete_one_additional_error_variants{enum DeleteOneAdditionalErrorVariants{}}]
// #[postgresql_crud::common_additional_error_variants{
//     enum CommonAdditionalErrorVariants {
//         CheckCommit {
//             #[eo_error_occurence]
//             check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
//             code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//         },
//     }
// }]
// #[postgresql_crud::create_many_additional_route_logic{
//     println!("kekw");
// }]
// #[postgresql_crud::create_one_additional_route_logic{}]
// #[postgresql_crud::read_many_additional_route_logic{}]
// #[postgresql_crud::read_one_additional_route_logic{}]
// #[postgresql_crud::update_many_additional_route_logic{}]
// #[postgresql_crud::update_one_additional_route_logic{}]
// #[postgresql_crud::delete_many_additional_route_logic{}]
// #[postgresql_crud::delete_one_additional_route_logic{}]
// #[postgresql_crud::common_additional_route_logic{
//     // if let Err(error) = postgresql_crud::check_commit::check_commit(
//     //     *app_state.get_enable_api_git_commit_check(),
//     //     &headers,
//     // ) {
//     //     let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
//     //     //todo use reserved work instead of TryCreateManyRouteLogicErrorNamed
//     //     let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
//     //         check_commit: error,
//     //         code_occurence: error_occurence_lib::code_occurence!(),
//     //     };
//     //     eprintln!("{error}");
//     //     let mut response = axum::response::IntoResponse::into_response(axum::Json(
//     //         TryCreateManyRouteLogicResponseVariants::from(error),
//     //     ));
//     //     *response.status_mut() = status_code;
//     //     return response;
//     // }
// }]
pub struct Example {
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBool,

    
    pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    // pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI16AsPostgresqlSmallInt,
    // pub std_primitive_i16_as_postgresql_small_int_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI16AsPostgresqlSmallIntNotNull,
    // pub std_primitive_i16_as_postgresql_small_serial: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI16AsPostgresqlSmallSerial,
    // pub std_primitive_i16_as_postgresql_small_serial_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    // pub std_primitive_i16_as_postgresql_small_int2: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI16AsPostgresqlInt2,
    // pub std_primitive_i16_as_postgresql_small_int2_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI16AsPostgresqlInt2NotNull,

    // pub std_primitive_i32_as_postgresql_int: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI32AsPostgresqlInt,
    // pub std_primitive_i32_as_postgresql_int_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI32AsPostgresqlIntNotNull,
    // pub std_primitive_i32_as_postgresql_serial: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI32AsPostgresqlSerial,
    // pub std_primitive_i32_as_postgresql_serial_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI32AsPostgresqlSerialNotNull,
    // pub std_primitive_i32_as_postgresql_int4: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI32AsPostgresqlInt4,
    // pub std_primitive_i32_as_postgresql_int4_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI32AsPostgresqlInt4NotNull,

    // pub std_primitive_i64_as_postgresql_big_int: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigInt,
    // pub std_primitive_i64_as_postgresql_big_int_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigIntNotNull,
    // pub std_primitive_i64_as_postgresql_big_serial: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerial,
    // pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNull,

    // #[generate_postgresql_crud_second_primary_key]
    pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
    // pub std_primitive_i64_as_postgresql_big_int8: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlInt8,
    // pub std_primitive_i64_as_postgresql_big_int8_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlInt8NotNull,

    // pub std_primitive_f32_as_postgresql_real: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveF32AsPostgresqlReal,
    // pub std_primitive_f32_as_postgresql_real_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveF32AsPostgresqlRealNotNull,
    // pub std_primitive_f32_as_postgresql_float4: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveF32AsPostgresqlFloat4,
    // pub std_primitive_f32_as_postgresql_float4_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveF32AsPostgresqlFloat4NotNull,

    // pub std_primitive_f64_as_postgresql_double_precision: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveF64AsPostgresqlDoublePrecision,
    // pub std_primitive_f64_as_postgresql_double_precision_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull,
    // pub std_primitive_f64_as_postgresql_float8: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveF64AsPostgresqlFloat8,
    // pub std_primitive_f64_as_postgresql_float8_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveF64AsPostgresqlFloat8NotNull,

    // pub std_string_string_as_postgresql_varchar: postgresql_crud::postgresql_types::base_wrap::StdStringStringAsPostgresqlVarchar,
    // pub std_string_string_as_postgresql_varchar_not_null: postgresql_crud::postgresql_types::base_wrap::StdStringStringAsPostgresqlVarcharNotNull,
    // pub std_string_string_as_postgresql_char_n: postgresql_crud::postgresql_types::base_wrap::StdStringStringAsPostgresqlCharN,
    // pub std_string_string_as_postgresql_char_n_not_null: postgresql_crud::postgresql_types::base_wrap::StdStringStringAsPostgresqlCharNNotNull,
    // pub std_string_string_as_postgresql_text: postgresql_crud::postgresql_types::base_wrap::StdStringStringAsPostgresqlText,
    // pub std_string_string_as_postgresql_text_not_null: postgresql_crud::postgresql_types::base_wrap::StdStringStringAsPostgresqlTextNotNull,
    // pub std_string_string_as_postgresql_ci_text: postgresql_crud::postgresql_types::base_wrap::StdStringStringAsPostgresqlCiText,
    // pub std_string_string_as_postgresql_ci_text_not_null: postgresql_crud::postgresql_types::base_wrap::StdStringStringAsPostgresqlCiTextNotNull,

    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea: postgresql_crud::postgresql_types::base_wrap::StdVecVecStdPrimitiveU8AsPostgresqlBytea,
    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea_not_null: postgresql_crud::postgresql_types::base_wrap::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,

    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_money_as_postgresql_money: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
    // pub sqlx_postgres_types_pg_money_as_postgresql_money_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,

    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgCiTextAsPostgresqlCiText,
    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,

    // pub sqlx_types_big_decimal_as_postgresql_numeric: postgresql_crud::postgresql_types::base_wrap::SqlxTypesBigDecimalAsPostgresqlNumeric,
    // pub sqlx_types_big_decimal_as_postgresql_numeric_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesBigDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_decimal_as_postgresql_numeric: postgresql_crud::postgresql_types::base_wrap::SqlxTypesDecimalAsPostgresqlNumeric,
    // pub sqlx_types_decimal_as_postgresql_numeric_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz: postgresql_crud::postgresql_types::base_wrap::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz: postgresql_crud::postgresql_types::base_wrap::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp: postgresql_crud::postgresql_types::base_wrap::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_chrono_naive_date_as_postgresql_date: postgresql_crud::postgresql_types::base_wrap::SqlxTypesChronoNaiveDateAsPostgresqlDate,
    // pub sqlx_types_chrono_naive_date_as_postgresql_date_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,

    // pub sqlx_types_chrono_naive_time_as_postgresql_time: postgresql_crud::postgresql_types::base_wrap::SqlxTypesChronoNaiveTimeAsPostgresqlTime,
    // pub sqlx_types_chrono_naive_time_as_postgresql_time_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,

    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz,
    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull,

    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp: postgresql_crud::postgresql_types::base_wrap::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz: postgresql_crud::postgresql_types::base_wrap::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,
    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_time_date_as_postgresql_date: postgresql_crud::postgresql_types::base_wrap::SqlxTypesTimeDateAsPostgresqlDate,
    // pub sqlx_types_time_date_as_postgresql_date_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesTimeDateAsPostgresqlDateNotNull,

    // pub sqlx_types_time_time_as_postgresql_time: postgresql_crud::postgresql_types::base_wrap::SqlxTypesTimeTimeAsPostgresqlTime,
    // pub sqlx_types_time_time_as_postgresql_time_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

    // pub sqlx_types_uuid_uuid_as_postgresql_uuid: postgresql_crud::postgresql_types::base_wrap::SqlxTypesUuidUuidAsPostgresqlUuid,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesUuidUuidAsPostgresqlUuidNotNull,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key: postgresql_crud::postgresql_types::base_wrap::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough //fails too but primary key is a different logic. need refactor it as different task

    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet: postgresql_crud::postgresql_types::base_wrap::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr: postgresql_crud::postgresql_types::base_wrap::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

    // pub std_net_ip_addr_as_postgresql_inet: postgresql_crud::postgresql_types::base_wrap::StdNetIpAddrAsPostgresqlInet,
    // pub std_net_ip_addr_as_postgresql_inet_not_null: postgresql_crud::postgresql_types::base_wrap::StdNetIpAddrAsPostgresqlInetNotNull,
    // pub std_net_ip_addr_as_postgresql_cidr: postgresql_crud::postgresql_types::base_wrap::StdNetIpAddrAsPostgresqlCidr,
    // pub std_net_ip_addr_as_postgresql_cidr_not_null: postgresql_crud::postgresql_types::base_wrap::StdNetIpAddrAsPostgresqlCidrNotNull,

    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr: postgresql_crud::postgresql_types::base_wrap::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,

    // pub sqlx_types_bit_vec_as_postgresql_bit: postgresql_crud::postgresql_types::base_wrap::SqlxTypesBitVecAsPostgresqlBit,
    // pub sqlx_types_bit_vec_as_postgresql_bit_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesBitVecAsPostgresqlBitNotNull,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit: postgresql_crud::postgresql_types::base_wrap::SqlxTypesBitVecAsPostgresqlVarBit,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesBitVecAsPostgresqlVarBitNotNull,

    //todo what to do with object?
    // pub sqlx_types_json_t_as_postgresql_json: postgresql_crud::postgresql_types::base_wrap::SqlxTypesJsonAsPostgresqlJson::<Something>,//todo
    // postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBool,//
    // pub sqlx_types_json_t_as_postgresql_json_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesJsonAsPostgresqlJsonNotNull::<Something>
    // pub sqlx_types_json_t_as_postgresql_json_b: postgresql_crud::postgresql_types::base_wrap::SqlxTypesJsonAsPostgresqlJsonB::<<Something>,//todo



    // pub sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::postgresql_types::base_wrap::SqlxTypesJsonAsPostgresqlJsonBNotNull<Something>, //todo

                                                                                                                            // pub serde_json_value_as_postgresql_json: postgresql_crud::postgresql_types::base_wrap::SerdeJsonValueAsPostgresqlJson,
                                                                                                                            // pub serde_json_value_as_postgresql_json_not_null: postgresql_crud::postgresql_types::base_wrap::SerdeJsonValueAsPostgresqlJsonNotNull,
                                                                                                                            // pub serde_json_value_as_postgresql_json_b: postgresql_crud::postgresql_types::base_wrap::SerdeJsonValueAsPostgresqlJsonB,
                                                                                                                            // pub serde_json_value_as_postgresql_json_b_not_null: postgresql_crud::postgresql_types::base_wrap::SerdeJsonValueAsPostgresqlJsonBNotNull,
}

//todo enum tree support
//todo generate wrapper type for all possible json type
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     //
//     postgresql_crud::GeneratePostgresqlQueryPart
// )] //user type must implement utoipa::ToSchema trait
// pub struct Something {
//     // pub id: postgresql_crud::json_types::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

//     pub std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8,
//     // pub std_primitive_i16: postgresql_crud::json_types::StdPrimitiveI16,
//     // pub std_primitive_i32: postgresql_crud::json_types::StdPrimitiveI32,
//     // pub std_primitive_i64: postgresql_crud::json_types::StdPrimitiveI64,
//     // pub std_primitive_u8: postgresql_crud::json_types::StdPrimitiveU8,
//     // pub std_primitive_u16: postgresql_crud::json_types::StdPrimitiveU16,
//     // pub std_primitive_u32: postgresql_crud::json_types::StdPrimitiveU32,
//     // pub std_primitive_u64: postgresql_crud::json_types::StdPrimitiveU64,
//     // pub std_primitive_f32: postgresql_crud::json_types::StdPrimitiveF32,
//     // pub std_primitive_f64: postgresql_crud::json_types::StdPrimitiveF64,
//     // pub std_primitive_bool: postgresql_crud::json_types::StdPrimitiveBool,
//     // pub std_string_string: postgresql_crud::json_types::StdStringString,
//     pub std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8,
//     // pub std_option_option_std_primitive_i16: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI16,
//     // pub std_option_option_std_primitive_i32: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI32,
//     // pub std_option_option_std_primitive_i64: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI64,
//     // pub std_option_option_std_primitive_u8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU8,
//     // pub std_option_option_std_primitive_u16: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU16,
//     // pub std_option_option_std_primitive_u32: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU32,
//     // pub std_option_option_std_primitive_u64: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU64,
//     // pub std_option_option_std_primitive_f32: postgresql_crud::json_types::StdOptionOptionStdPrimitiveF32,
//     // pub std_option_option_std_primitive_f64: postgresql_crud::json_types::StdOptionOptionStdPrimitiveF64,
//     // pub std_option_option_std_primitive_bool: postgresql_crud::json_types::StdOptionOptionStdPrimitiveBool,
//     // pub std_option_option_std_string_string: postgresql_crud::json_types::StdOptionOptionStdStringString,
//     pub std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdPrimitiveI8,
//     // pub std_vec_vec_std_primitive_i16: postgresql_crud::json_types::StdVecVecStdPrimitiveI16,
//     // pub std_vec_vec_std_primitive_i32: postgresql_crud::json_types::StdVecVecStdPrimitiveI32,
//     // pub std_vec_vec_std_primitive_i64: postgresql_crud::json_types::StdVecVecStdPrimitiveI64,
//     // pub std_vec_vec_std_primitive_u8: postgresql_crud::json_types::StdVecVecStdPrimitiveU8,
//     // pub std_vec_vec_std_primitive_u16: postgresql_crud::json_types::StdVecVecStdPrimitiveU16,
//     // pub std_vec_vec_std_primitive_u32: postgresql_crud::json_types::StdVecVecStdPrimitiveU32,
//     // pub std_vec_vec_std_primitive_u64: postgresql_crud::json_types::StdVecVecStdPrimitiveU64,
//     // pub std_vec_vec_std_primitive_f32: postgresql_crud::json_types::StdVecVecStdPrimitiveF32,
//     // pub std_vec_vec_std_primitive_f64: postgresql_crud::json_types::StdVecVecStdPrimitiveF64,
//     // pub std_vec_vec_std_primitive_bool: postgresql_crud::json_types::StdVecVecStdPrimitiveBool,
//     // pub std_vec_vec_std_string_string: postgresql_crud::json_types::StdVecVecStdStringString,
//     pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8,
//     // pub std_option_option_std_vec_vec_std_primitive_i16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI16,
//     // pub std_option_option_std_vec_vec_std_primitive_i32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI32,
//     // pub std_option_option_std_vec_vec_std_primitive_i64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI64,
//     // pub std_option_option_std_vec_vec_std_primitive_u8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU8,
//     // pub std_option_option_std_vec_vec_std_primitive_u16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU16,
//     // pub std_option_option_std_vec_vec_std_primitive_u32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU32,
//     // pub std_option_option_std_vec_vec_std_primitive_u64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU64,
//     // pub std_option_option_std_vec_vec_std_primitive_f32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveF32,
//     // pub std_option_option_std_vec_vec_std_primitive_f64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveF64,
//     // pub std_option_option_std_vec_vec_std_primitive_bool: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveBool,
//     // pub std_option_option_std_vec_vec_std_string_string: postgresql_crud::json_types::StdOptionOptionStdVecVecStdStringString,
//     pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8,
//     // pub std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI16,
//     // pub std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI32,
//     // pub std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI64,
//     // pub std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU8,
//     // pub std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU16,
//     // pub std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU32,
//     // pub std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU64,
//     // pub std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveF32,
//     // pub std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveF64,
//     // pub std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveBool,
//     // pub std_vec_vec_std_option_option_std_string_string: postgresql_crud::json_types::StdVecVecStdOptionOptionStdStringString,
//     pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
//     // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
//     // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
//     // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
//     // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
//     // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
//     // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
//     // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
//     // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
//     // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
//     // pub std_option_option_std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
//     // pub std_option_option_std_vec_vec_std_option_option_std_string_string: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdStringString,

//     // pub object: ObjectDoggie,
//     // pub std_option_option_object: StdOptionOptionObjectDoggie,

//     // pub std_vec_vec_object_with_id: StdVecVecObjectWithIdDoggie,
//     // pub std_option_option_std_vec_vec_object_with_id: StdOptionOptionStdVecVecObjectWithIdDoggie
// }
/////////////////////////////////////////
impl Example {
    pub fn table_name() -> &'static str {
        "example"
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct ExampleOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_primitive_i64_as_postgresql_big_serial_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_primitive_bool_as_postgresql_bool: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullToRead>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, PartialEq, Clone)]
pub enum ExampleColumn {
    #[serde(rename(serialize = "std_primitive_bool_as_postgresql_bool", deserialize = "std_primitive_bool_as_postgresql_bool"))]
    StdPrimitiveBoolAsPostgresqlBool(std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolColumn>),
    #[serde(rename(serialize = "std_primitive_bool_as_postgresql_bool_not_null", deserialize = "std_primitive_bool_as_postgresql_bool_not_null"))]
    StdPrimitiveBoolAsPostgresqlBoolNotNull(std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullColumn>),
    #[serde(rename(serialize = "std_primitive_i64_as_postgresql_big_serial_not_null", deserialize = "std_primitive_i64_as_postgresql_big_serial_not_null"))]
    StdPrimitiveI64AsPostgresqlBigSerialNotNull(std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullColumn>),
}
impl std::fmt::Display for ExampleColumn {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", serde_json::to_string(&self).unwrap_or_else(|e| format!("cannot serialize into json: {e:?}")))
    }
}
impl error_occurence_lib::ToStdStringString for ExampleColumn {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ExampleColumn {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            ExampleColumn::StdPrimitiveBoolAsPostgresqlBool(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            ExampleColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull(
                postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            ),
            ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNull(
                postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            ),
        ]
    }
}
impl ExampleColumn {
    fn pick_column(&self) -> std::string::String {
        match &self {
            Self::StdPrimitiveBoolAsPostgresqlBool(_) => "std_primitive_bool_as_postgresql_bool".to_string(),
            Self::StdPrimitiveBoolAsPostgresqlBoolNotNull(_) => "std_primitive_bool_as_postgresql_bool_not_null".to_string(),
            Self::StdPrimitiveI64AsPostgresqlBigSerialNotNull(_) => "std_primitive_i64_as_postgresql_big_serial_not_null".to_string(),
        }
    }
}
pub const ALLOW_METHODS: [http::Method; 4] = [http::Method::GET, http::Method::POST, http::Method::PATCH, http::Method::DELETE];
#[derive(Debug, Clone, Copy)]
pub struct ExampleColumnReadPermission {
    std_primitive_bool_as_postgresql_bool: std::primitive::bool,
    std_primitive_bool_as_postgresql_bool_not_null: std::primitive::bool,
    std_primitive_i64_as_postgresql_big_serial_not_null: std::primitive::bool,
}
pub async fn create_table_if_not_exists(pool: &sqlx::Pool<sqlx::Postgres>) {
    let create_extension_if_not_exists_pg_jsonschema_query_stringified = "create extension if not exists pg_jsonschema";
    println!("{create_extension_if_not_exists_pg_jsonschema_query_stringified}");
    let _ = sqlx::query(create_extension_if_not_exists_pg_jsonschema_query_stringified).execute(pool).await.unwrap();
    let create_table_if_not_exists_query_stringified = format!(
        "CREATE TABLE IF NOT EXISTS example (std_primitive_i64_as_postgresql_big_serial_not_null {} PRIMARY KEY,std_primitive_bool_as_postgresql_bool {},std_primitive_bool_as_postgresql_bool_not_null {})",
        <postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNull as postgresql_crud::CreateTableQueryPart>::create_table_query_part(),
        <postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBool as postgresql_crud::CreateTableQueryPart>::create_table_query_part(),
        <postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNull as postgresql_crud::CreateTableQueryPart>::create_table_query_part()
    );
    println!("{create_table_if_not_exists_query_stringified}");
    let _ = sqlx::query(&create_table_if_not_exists_query_stringified).execute(pool).await.unwrap();
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElement {
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolToCreate,
    pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullToCreate,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayload(pub std::vec::Vec<CreateManyPayloadElement>);
#[derive(Debug)]
pub struct CreateManyParameters {
    pub payload: CreateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLength {
        expected_length: std::string::String,
        got_length: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLengthAndRollback {
        expected_length: std::string::String,
        got_length: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyRouteLogicErrorNamed> for TryCreateManyRouteLogicResponseVariants {
    fn from(value: TryCreateManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence } => Self::BindQuery { bind_query, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLength { expected_length, got_length, code_occurence } => Self::UnexpectedRowsLength { expected_length, got_length, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence } => Self::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLength {
        #[eo_to_std_string_string]
        expected_length: std::primitive::usize,
        #[eo_to_std_string_string]
        got_length: std::primitive::usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLengthAndRollback {
        #[eo_to_std_string_string]
        expected_length: std::primitive::usize,
        #[eo_to_std_string_string]
        got_length: std::primitive::usize,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2545,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    println!("kekw");
    let parameters = CreateManyParameters {
        payload: match serde_json::from_slice::<CreateManyPayload>(&body_bytes) {
            Ok(value) => {
                let value = CreateManyPayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryCreateManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2617,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let error_0 = parameters.payload.0.len();
    let query_string = {
        let mut increment: std::primitive::u64 = 0;
        let mut values = std::string::String::default();
        for element in &parameters.payload.0 {
            let mut acc = std::string::String::default();
            match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&element.std_primitive_bool_as_postgresql_bool, &mut increment) {
                Ok(value) => {
                    acc.push_str(&format!("{value},"));
                }
                Err(error_0) => {
                    let error = TryCreateManyRouteLogicErrorNamed::BindQuery {
                        bind_query: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 3132,
                                column: 258,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&element.std_primitive_bool_as_postgresql_bool_not_null, &mut increment) {
                Ok(value) => {
                    acc.push_str(&format!("{value},"));
                }
                Err(error_0) => {
                    let error = TryCreateManyRouteLogicErrorNamed::BindQuery {
                        bind_query: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 3132,
                                column: 258,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            let _ = acc.pop();
            values.push_str(&format!("({acc}),"));
        }
        let _ = values.pop();
        format!("insert into example (std_primitive_bool_as_postgresql_bool,std_primitive_bool_as_postgresql_bool_not_null) values {values}  returning std_primitive_i64_as_postgresql_big_serial_not_null")
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        for element in parameters.payload.0 {
            query = postgresql_crud::BindQuerySecond::bind_value_to_query(element.std_primitive_bool_as_postgresql_bool, query);
            query = postgresql_crud::BindQuerySecond::bind_value_to_query(element.std_primitive_bool_as_postgresql_bool_not_null, query);
        }
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2302,
                            column: 246,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut rows = binded_query.fetch(executor.as_mut());
            let mut acc = std::vec::Vec::new();
            while let Some(value) = match {
                use postgresql_crud::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => match value {
                    Some(value) => match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null") {
                        Ok(value) => Some(value),
                        Err(error_0) => {
                            drop(rows);
                            match executor.rollback().await {
                                Ok(_) => {
                                    let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 2965,
                                                column: 129,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                                Err(error_1) => {
                                    let error = TryCreateManyRouteLogicErrorNamed::RowAndRollback {
                                        row: error_0,
                                        rollback: error_1,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 2965,
                                                column: 158,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        }
                    },
                    None => None,
                },
                Err(error_0) => {
                    drop(rows);
                    match executor.rollback().await {
                        Ok(_) => {
                            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2967,
                                        column: 125,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryCreateManyRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2967,
                                        column: 154,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            } {
                acc.push(value);
            }
            acc
        };
        {
            let error_1 = value.len();
            if error_0 != error_1 {
                match executor.rollback().await {
                    Ok(_) => {
                        let error = TryCreateManyRouteLogicErrorNamed::UnexpectedRowsLength {
                            expected_length: error_0,
                            got_length: error_1,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 3186,
                                    column: 25,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    Err(error_2) => {
                        let error = TryCreateManyRouteLogicErrorNamed::UnexpectedRowsLengthAndRollback {
                            expected_length: error_0,
                            got_length: error_1,
                            rollback: error_2,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 3193,
                                    column: 25,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
        }
        if let Err(error_0) = executor.commit().await {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2314,
                        column: 246,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::CREATED;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_create_many_route_logic_error_named_with_serialize_deserialize: TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many(server_location: &std::primitive::str, parameters: CreateManyParameters) -> Result<std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate>, TryCreateManyErrorNamed> {
    let payload = {
        let value = CreateManyPayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryCreateManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2669,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/create_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryCreateManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2721,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryCreateManyErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2738,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryCreateManyRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryCreateManyErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2751,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_create_many_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryCreateManyRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryCreateManyRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryCreateManyRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryCreateManyRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryCreateManyRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryCreateManyRouteLogicResponseVariants::BindQuery { bind_query, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence },
        TryCreateManyRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
        TryCreateManyRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
        TryCreateManyRouteLogicResponseVariants::UnexpectedRowsLength { expected_length, got_length, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLength { expected_length, got_length, code_occurence },
        TryCreateManyRouteLogicResponseVariants::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence },
    };
    Err(TryCreateManyErrorNamed::TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_create_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                line: 2789,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CreateManyPayloadElement {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_bool_as_postgresql_bool: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CreateManyPayload {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(vec![
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ])
    }
}
pub async fn create_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <CreateManyPayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateOnePayload {
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolToCreate,
    pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullToCreate,
}
#[derive(Debug)]
pub struct CreateOneParameters {
    pub payload: CreateOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateOneRouteLogicResponseVariants {
    Desirable(postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneRouteLogicErrorNamed> for TryCreateOneRouteLogicResponseVariants {
    fn from(value: TryCreateOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence } => Self::BindQuery { bind_query, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2545,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = CreateOneParameters {
        payload: match serde_json::from_slice::<CreateOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = CreateOnePayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryCreateOneRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2617,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = {
        let mut increment: std::primitive::u64 = 0;
        format!(
            "insert into example (std_primitive_bool_as_postgresql_bool,std_primitive_bool_as_postgresql_bool_not_null) values ({},{}) returning std_primitive_i64_as_postgresql_big_serial_not_null",
            match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&parameters.payload.std_primitive_bool_as_postgresql_bool, &mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TryCreateOneRouteLogicErrorNamed::BindQuery {
                        bind_query: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 3415,
                                column: 258,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            },
            match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&parameters.payload.std_primitive_bool_as_postgresql_bool_not_null, &mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TryCreateOneRouteLogicErrorNamed::BindQuery {
                        bind_query: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 3415,
                                column: 258,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        )
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        query = postgresql_crud::BindQuerySecond::bind_value_to_query(parameters.payload.std_primitive_bool_as_postgresql_bool, query);
        query = postgresql_crud::BindQuerySecond::bind_value_to_query(parameters.payload.std_primitive_bool_as_postgresql_bool_not_null, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2302,
                            column: 246,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null") {
                    Ok(value) => value,
                    Err(error_0) => match executor.rollback().await {
                        Ok(_) => {
                            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2981,
                                        column: 112,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryCreateOneRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2981,
                                        column: 141,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                },
                Err(error_0) => match executor.rollback().await {
                    Ok(_) => {
                        let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 2983,
                                    column: 108,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    Err(error_1) => {
                        let error = TryCreateOneRouteLogicErrorNamed::RowAndRollback {
                            row: error_0,
                            rollback: error_1,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 2983,
                                    column: 137,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                },
            }
        };
        if let Err(error_0) = executor.commit().await {
            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2314,
                        column: 246,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::CREATED;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_create_one_route_logic_error_named_with_serialize_deserialize: TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_one(server_location: &std::primitive::str, parameters: CreateOneParameters) -> Result<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate, TryCreateOneErrorNamed> {
    let payload = {
        let value = CreateOnePayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryCreateOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2669,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/create_one", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryCreateOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2721,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryCreateOneErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2738,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryCreateOneRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryCreateOneErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2751,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_create_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryCreateOneRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryCreateOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryCreateOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryCreateOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryCreateOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryCreateOneRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
        TryCreateOneRouteLogicResponseVariants::BindQuery { bind_query, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence },
        TryCreateOneRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
    };
    Err(TryCreateOneErrorNamed::TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize {
        try_create_one_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                line: 2789,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CreateOnePayload {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_bool_as_postgresql_bool: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn create_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <CreateOnePayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadManyPayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null: std::option::Option<std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNull>>,
    pub std_primitive_bool_as_postgresql_bool: std::option::Option<std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolWhere>>,
    pub std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullWhere>>,
    pub select: std::vec::Vec<ExampleColumn>,
    pub order_by: postgresql_crud::OrderBy<ExampleColumn>,
    pub limit: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
    pub offset: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
}
#[derive(Debug)]
pub struct ReadManyParameters {
    pub payload: ReadManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<ExampleOptions>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        not_unique_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        not_unique_column: ExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBool {
        not_unique_std_primitive_bool_as_postgresql_bool: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
        not_unique_std_primitive_bool_as_postgresql_bool_not_null: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadManyRouteLogicErrorNamed> for TryReadManyRouteLogicResponseVariants {
    fn from(value: TryReadManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence } => Self::BindQuery { bind_query, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => Self::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence } => Self::NotUniqueColumn { not_unique_column, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBool {
                not_unique_std_primitive_bool_as_postgresql_bool,
                code_occurence,
            } => Self::NotUniqueStdPrimitiveBoolAsPostgresqlBool {
                not_unique_std_primitive_bool_as_postgresql_bool,
                code_occurence,
            },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                not_unique_std_primitive_bool_as_postgresql_bool_not_null,
                code_occurence,
            } => Self::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                not_unique_std_primitive_bool_as_postgresql_bool_not_null,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: ExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBool {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBool,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2545,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = ReadManyParameters {
        payload: match serde_json::from_slice::<ReadManyPayload>(&body_bytes) {
            Ok(value) => {
                let value = ReadManyPayload::from(value);
                if let Some(value) = &value.std_primitive_i64_as_postgresql_big_serial_not_null {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if !acc.contains(&element) {
                            acc.push(&element);
                        } else {
                            let error_0 = *element;
                            let error = TryReadManyRouteLogicErrorNamed::NotUniquePrimaryKey {
                                not_unique_primary_key: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2853,
                                        column: 173,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                if let Some(value) = &value.std_primitive_bool_as_postgresql_bool {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if !acc.contains(&element) {
                            acc.push(&element);
                        } else {
                            let error_0 = element.value.clone();
                            let error = TryReadManyRouteLogicErrorNamed::NotUniqueStdPrimitiveBoolAsPostgresqlBool {
                                not_unique_std_primitive_bool_as_postgresql_bool: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2900,
                                        column: 17,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                if let Some(value) = &value.std_primitive_bool_as_postgresql_bool_not_null {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if !acc.contains(&element) {
                            acc.push(&element);
                        } else {
                            let error_0 = element.value.clone();
                            let error = TryReadManyRouteLogicErrorNamed::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                                not_unique_std_primitive_bool_as_postgresql_bool_not_null: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2900,
                                        column: 17,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                let mut acc = std::vec::Vec::new();
                for element in &value.select {
                    if acc.contains(&element) {
                        let error_0 = element.clone();
                        let error = TryReadManyRouteLogicErrorNamed::NotUniqueColumn {
                            not_unique_column: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 1977,
                                    column: 264,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    } else {
                        acc.push(element);
                    }
                }
                value
            }
            Err(error_0) => {
                let error = TryReadManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2617,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = {
        format!(
            "select {} from example {}",
            {
                let mut value = std::string::String::default();
                for element in &parameters.payload.select {
                    value.push_str(&match element {
                        ExampleColumn::StdPrimitiveBoolAsPostgresqlBool(_) => "std_primitive_bool_as_postgresql_bool".to_string(),
                        ExampleColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull(_) => "std_primitive_bool_as_postgresql_bool_not_null".to_string(),
                        ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNull(_) => "std_primitive_i64_as_postgresql_big_serial_not_null".to_string(),
                    });
                    value.push_str(",");
                }
                let _ = value.pop();
                value
            },
            {
                let mut increment: std::primitive::u64 = 0;
                let mut additional_parameters = std::string::String::default();
                if let Some(_) = &parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null {
                    let prefix = match additional_parameters.is_empty() {
                        true => "where",
                        false => " and",
                    };
                    match increment.checked_add(1) {
                        Some(value) => {
                            increment = value;
                        }
                        None => {
                            let error = TryReadManyRouteLogicErrorNamed::CheckedAdd {
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 3613,
                                        column: 268,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                    additional_parameters.push_str(&format!("{} std_primitive_i64_as_postgresql_big_serial_not_null in (select unnest(${}))", prefix, increment));
                }
                if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool {
                    additional_parameters.push_str(&format!(
                        "{} {}",
                        match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        },
                        {
                            let mut acc = std::string::String::default();
                            for (index, element) in value.iter().enumerate() {
                                match postgresql_crud::BindQuerySecond::try_generate_bind_increments(element, &mut increment) {
                                    Ok(value) => {
                                        let handle = format!("std_primitive_bool_as_postgresql_bool ~ {value} ");
                                        match index == 0 {
                                            true => {
                                                acc.push_str(&handle);
                                            }
                                            false => {
                                                acc.push_str(&format!("{} {handle}", element.conjuctive_operator));
                                            }
                                        }
                                    }
                                    Err(error_0) => {
                                        let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                            bind_query: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                    line: 3639,
                                                    column: 266,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                }
                            }
                            if let false = acc.is_empty() {
                                let _ = acc.pop();
                            }
                            acc
                        }
                    ));
                }
                if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
                    additional_parameters.push_str(&format!(
                        "{} {}",
                        match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        },
                        {
                            let mut acc = std::string::String::default();
                            for (index, element) in value.iter().enumerate() {
                                match postgresql_crud::BindQuerySecond::try_generate_bind_increments(element, &mut increment) {
                                    Ok(value) => {
                                        let handle = format!("std_primitive_bool_as_postgresql_bool_not_null ~ {value} ");
                                        match index == 0 {
                                            true => {
                                                acc.push_str(&handle);
                                            }
                                            false => {
                                                acc.push_str(&format!("{} {handle}", element.conjuctive_operator));
                                            }
                                        }
                                    }
                                    Err(error_0) => {
                                        let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                            bind_query: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                    line: 3639,
                                                    column: 266,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                }
                            }
                            if let false = acc.is_empty() {
                                let _ = acc.pop();
                            }
                            acc
                        }
                    ));
                }
                {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = &parameters.payload.order_by;
                    let order = match &value.order {
                        Some(value) => value.to_snake_case_stringified(),
                        None => postgresql_crud::Order::default().to_snake_case_stringified(),
                    };
                    additional_parameters.push_str(&format!("{}order by {} {}", prefix, value.column.pick_column(), order,));
                }
                {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&parameters.payload.limit, &mut increment) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                bind_query: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 3687,
                                        column: 168,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    };
                    additional_parameters.push_str(&format!("{}limit {}", prefix, value));
                }
                {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&parameters.payload.offset, &mut increment) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                bind_query: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 3689,
                                        column: 168,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
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
        if let Some(value) = parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null {
            query = query.bind(value.into_iter().map(|element| element.clone()).collect::<std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNull>>());
        }
        if let Some(value) = parameters.payload.std_primitive_bool_as_postgresql_bool {
            for value in value {
                query = postgresql_crud::BindQuerySecond::bind_value_to_query(value, query);
            }
        }
        if let Some(value) = parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
            for value in value {
                query = postgresql_crud::BindQuerySecond::bind_value_to_query(value, query);
            }
        }
        query = postgresql_crud::BindQuerySecond::bind_value_to_query(parameters.payload.limit, query);
        query = postgresql_crud::BindQuerySecond::bind_value_to_query(parameters.payload.offset, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let value = {
            let mut rows = binded_query.fetch(executor.as_mut());
            let mut acc = std::vec::Vec::new();
            while let Some(value) = match {
                use postgresql_crud::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => match value {
                    Some(value) => Some({
                        let mut std_primitive_i64_as_postgresql_big_serial_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToRead>> = None;
                        let mut std_primitive_bool_as_postgresql_bool: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolToRead>> = None;
                        let mut std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullToRead>> = None;
                        for element in &parameters.payload.select {
                            match element {
                                ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNull(_) => {
                                    match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToRead, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null") {
                                        Ok(value) => {
                                            std_primitive_i64_as_postgresql_big_serial_not_null = Some(postgresql_crud::Value { value: value });
                                        }
                                        Err(error_0) => {
                                            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                                postgresql: error_0,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!(),
                                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                        line: 1235,
                                                        column: 246,
                                                    }),
                                                ),
                                            };
                                            eprintln!("{error}");
                                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                            return response;
                                        }
                                    }
                                }
                                ExampleColumn::StdPrimitiveBoolAsPostgresqlBool(_) => match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolToRead, &std::primitive::str>(&value, "std_primitive_bool_as_postgresql_bool") {
                                    Ok(value) => {
                                        std_primitive_bool_as_postgresql_bool = Some(postgresql_crud::Value { value: value });
                                    }
                                    Err(error_0) => {
                                        let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                            postgresql: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                    line: 1270,
                                                    column: 246,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                },
                                ExampleColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull(_) => match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullToRead, &std::primitive::str>(&value, "std_primitive_bool_as_postgresql_bool_not_null") {
                                    Ok(value) => {
                                        std_primitive_bool_as_postgresql_bool_not_null = Some(postgresql_crud::Value { value: value });
                                    }
                                    Err(error_0) => {
                                        let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                            postgresql: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                    line: 1270,
                                                    column: 246,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                },
                            }
                        }
                        ExampleOptions {
                            std_primitive_bool_as_postgresql_bool,
                            std_primitive_bool_as_postgresql_bool_not_null,
                            std_primitive_i64_as_postgresql_big_serial_not_null,
                        }
                    }),
                    None => None,
                },
                Err(error_0) => {
                    let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 3810,
                                column: 169,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            } {
                acc.push(value);
            }
            acc
        };
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBool {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBool,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: ExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_read_many_route_logic_error_named_with_serialize_deserialize: TryReadManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many(server_location: &std::primitive::str, parameters: ReadManyParameters) -> Result<std::vec::Vec<ExampleOptions>, TryReadManyErrorNamed> {
    let payload = {
        if let Some(value) = &parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if !acc.contains(&element) {
                    acc.push(&element);
                } else {
                    let error_0 = *element;
                    return Err(TryReadManyErrorNamed::NotUniquePrimaryKey {
                        not_unique_primary_key: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 3862,
                                column: 202,
                            }),
                        ),
                    });
                }
            }
        }
        if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if !acc.contains(&&element.value) {
                    acc.push(&element.value);
                } else {
                    let error_0 = element.value.clone();
                    return Err(TryReadManyErrorNamed::NotUniqueStdPrimitiveBoolAsPostgresqlBool {
                        not_unique_std_primitive_bool_as_postgresql_bool: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 3880,
                                column: 37,
                            }),
                        ),
                    });
                }
            }
        }
        if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if !acc.contains(&&element.value) {
                    acc.push(&element.value);
                } else {
                    let error_0 = element.value.clone();
                    return Err(TryReadManyErrorNamed::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                        not_unique_std_primitive_bool_as_postgresql_bool_not_null: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 3880,
                                column: 37,
                            }),
                        ),
                    });
                }
            }
        }
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.select {
            if acc.contains(&element) {
                let error_0 = element.clone();
                return Err(TryReadManyErrorNamed::NotUniqueColumn {
                    not_unique_column: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2050,
                            column: 176,
                        }),
                    ),
                });
            } else {
                acc.push(element);
            }
        }
        let value = ReadManyPayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryReadManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2669,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/read_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryReadManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2721,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryReadManyErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2738,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryReadManyRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryReadManyErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2751,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_read_many_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryReadManyRouteLogicResponseVariants::Desirable(value) => {
            let value = value.into_iter().fold(std::vec::Vec::new(), |mut acc, element| {
                acc.push(element);
                acc
            });
            return Ok(value);
        }
        TryReadManyRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryReadManyRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryReadManyRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryReadManyRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryReadManyRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
        TryReadManyRouteLogicResponseVariants::BindQuery { bind_query, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence },
        TryReadManyRouteLogicResponseVariants::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryReadManyRouteLogicResponseVariants::NotUniqueColumn { not_unique_column, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence },
        TryReadManyRouteLogicResponseVariants::NotUniqueStdPrimitiveBoolAsPostgresqlBool {
            not_unique_std_primitive_bool_as_postgresql_bool,
            code_occurence,
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBool {
            not_unique_std_primitive_bool_as_postgresql_bool,
            code_occurence,
        },
        TryReadManyRouteLogicResponseVariants::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
            not_unique_std_primitive_bool_as_postgresql_bool_not_null,
            code_occurence,
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
            not_unique_std_primitive_bool_as_postgresql_bool_not_null,
            code_occurence,
        },
    };
    Err(TryReadManyErrorNamed::TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_read_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                line: 2789,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ReadManyPayload {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_bool_as_postgresql_bool: Some(vec![
                postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            ]),
            std_primitive_bool_as_postgresql_bool_not_null: Some(vec![
                postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            ]),
            std_primitive_i64_as_postgresql_big_serial_not_null: Some(vec![
                postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            ]),
            select: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            order_by: postgresql_crud::OrderBy {
                column: ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNull(vec![]),
                order: Some(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            },
            limit: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            offset: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn read_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <ReadManyPayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToRead,
    pub select: std::vec::Vec<ExampleColumn>,
}
#[derive(Debug)]
pub struct ReadOneParameters {
    pub payload: ReadOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadOneRouteLogicResponseVariants {
    Desirable(ExampleOptions),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        not_unique_column: ExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneRouteLogicErrorNamed> for TryReadOneRouteLogicResponseVariants {
    fn from(value: TryReadOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence } => Self::NotUniqueColumn { not_unique_column, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: ExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2545,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = ReadOneParameters {
        payload: match serde_json::from_slice::<ReadOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = ReadOnePayload::from(value);
                let mut acc = std::vec::Vec::new();
                for element in &value.select {
                    if acc.contains(&element) {
                        let error_0 = element.clone();
                        let error = TryReadOneRouteLogicErrorNamed::NotUniqueColumn {
                            not_unique_column: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 1977,
                                    column: 264,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    } else {
                        acc.push(element);
                    }
                }
                value
            }
            Err(error_0) => {
                let error = TryReadOneRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2617,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = format!("select {} from example where std_primitive_i64_as_postgresql_big_serial_not_null = $1", {
        let mut value = std::string::String::default();
        for element in &parameters.payload.select {
            value.push_str(&match element {
                ExampleColumn::StdPrimitiveBoolAsPostgresqlBool(_) => "std_primitive_bool_as_postgresql_bool".to_string(),
                ExampleColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull(_) => "std_primitive_bool_as_postgresql_bool_not_null".to_string(),
                ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNull(_) => "std_primitive_i64_as_postgresql_big_serial_not_null".to_string(),
            });
            value.push_str(",");
        }
        let _ = value.pop();
        value
    },);
    println!("{}", query_string);
    let binded_query = {
        let query = sqlx::query::<sqlx::Postgres>(&query_string);
        let query = postgresql_crud::BindQuerySecond::bind_value_to_query(parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => {
                    let mut std_primitive_i64_as_postgresql_big_serial_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToRead>> = None;
                    let mut std_primitive_bool_as_postgresql_bool: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolToRead>> = None;
                    let mut std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullToRead>> = None;
                    for element in &parameters.payload.select {
                        match element {
                            ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNull(_) => {
                                match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToRead, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null") {
                                    Ok(value) => {
                                        std_primitive_i64_as_postgresql_big_serial_not_null = Some(postgresql_crud::Value { value: value });
                                    }
                                    Err(error_0) => {
                                        let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                            postgresql: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                    line: 1235,
                                                    column: 246,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                }
                            }
                            ExampleColumn::StdPrimitiveBoolAsPostgresqlBool(_) => match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolToRead, &std::primitive::str>(&value, "std_primitive_bool_as_postgresql_bool") {
                                Ok(value) => {
                                    std_primitive_bool_as_postgresql_bool = Some(postgresql_crud::Value { value: value });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 1270,
                                                column: 246,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                            ExampleColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull(_) => match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullToRead, &std::primitive::str>(&value, "std_primitive_bool_as_postgresql_bool_not_null") {
                                Ok(value) => {
                                    std_primitive_bool_as_postgresql_bool_not_null = Some(postgresql_crud::Value { value: value });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 1270,
                                                column: 246,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                        }
                    }
                    ExampleOptions {
                        std_primitive_bool_as_postgresql_bool,
                        std_primitive_bool_as_postgresql_bool_not_null,
                        std_primitive_i64_as_postgresql_big_serial_not_null,
                    }
                }
                Err(error_0) => {
                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 4132,
                                column: 169,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        };
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: ExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryReadOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_read_one_route_logic_error_named_with_serialize_deserialize: TryReadOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_one(server_location: &std::primitive::str, parameters: ReadOneParameters) -> Result<ExampleOptions, TryReadOneErrorNamed> {
    let payload = {
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.select {
            if acc.contains(&element) {
                let error_0 = element.clone();
                return Err(TryReadOneErrorNamed::NotUniqueColumn {
                    not_unique_column: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2050,
                            column: 176,
                        }),
                    ),
                });
            } else {
                acc.push(element);
            }
        }
        let value = ReadOnePayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryReadOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2669,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/read_one", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryReadOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2721,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryReadOneErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2738,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryReadOneRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryReadOneErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2751,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_read_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryReadOneRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryReadOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryReadOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryReadOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryReadOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryReadOneRouteLogicResponseVariants::NotUniqueColumn { not_unique_column, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence },
    };
    Err(TryReadOneErrorNamed::TryReadOneRouteLogicErrorNamedWithSerializeDeserialize {
        try_read_one_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                line: 2789,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ReadOnePayload {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            select: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn read_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <ReadOnePayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadElement {
    pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate,
    pub std_primitive_bool_as_postgresql_bool: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolToUpdate>>,
    pub std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayload(pub std::vec::Vec<UpdateManyPayloadElement>);
#[derive(Debug)]
pub struct UpdateManyParameters {
    pub payload: UpdateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndRollback {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        not_unique_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        no_payload_fields_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyRouteLogicErrorNamed> for TryUpdateManyRouteLogicResponseVariants {
    fn from(value: TryUpdateManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence } => Self::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence } => Self::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => Self::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence } => Self::BindQuery { bind_query, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence } => Self::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndRollback {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate>,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        #[eo_to_std_string_string]
        no_payload_fields_primary_key: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_many_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2545,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = UpdateManyParameters {
        payload: match serde_json::from_slice::<UpdateManyPayload>(&body_bytes) {
            Ok(value) => {
                let value = UpdateManyPayload::from(value);
                {
                    let mut acc = std::vec::Vec::new();
                    for element in &value.0 {
                        if !acc.contains(&&element.std_primitive_i64_as_postgresql_big_serial_not_null) {
                            acc.push(&element.std_primitive_i64_as_postgresql_big_serial_not_null);
                        } else {
                            let error_0 = element.std_primitive_i64_as_postgresql_big_serial_not_null;
                            let error = TryUpdateManyRouteLogicErrorNamed::NotUniquePrimaryKey {
                                not_unique_primary_key: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 4341,
                                        column: 185,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                for element in &value.0 {
                    if let (None, None) = (&element.std_primitive_bool_as_postgresql_bool, &element.std_primitive_bool_as_postgresql_bool_not_null) {
                        let error_0 = element.std_primitive_i64_as_postgresql_big_serial_not_null;
                        let error = TryUpdateManyRouteLogicErrorNamed::NoPayloadFieldsPrimaryKey {
                            no_payload_fields_primary_key: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 2083,
                                    column: 288,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
                value
            }
            Err(error_0) => {
                let error = TryUpdateManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2617,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let expected_primary_keys = parameters
        .payload
        .0
        .iter()
        .map(|element| element.std_primitive_i64_as_postgresql_big_serial_not_null.clone())
        .collect::<std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate>>();
    let query_string = {
        let mut query = std::string::String::from("update example set ");
        let mut increment: std::primitive::u64 = 0;
        {
            let mut is_std_primitive_bool_as_postgresql_bool_update_exist = false;
            for element in &parameters.payload.0 {
                if element.std_primitive_bool_as_postgresql_bool.is_some() {
                    is_std_primitive_bool_as_postgresql_bool_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_bool_as_postgresql_bool_update_exist {
                let mut acc = std::string::String::from("std_primitive_bool_as_postgresql_bool = case ");
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_bool_as_postgresql_bool {
                        acc.push_str(&format!(
                            "when std_primitive_i64_as_postgresql_big_serial_not_null = {} then {} ",
                            match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&element.std_primitive_i64_as_postgresql_big_serial_not_null, &mut increment) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    let error = TryUpdateManyRouteLogicErrorNamed::BindQuery {
                                        bind_query: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 4369,
                                                column: 254,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                            match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&value.value, &mut increment) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    let error = TryUpdateManyRouteLogicErrorNamed::BindQuery {
                                        bind_query: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 4369,
                                                column: 254,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        ));
                    }
                }
                query.push_str(&format!("{}{}", acc, "else std_primitive_bool_as_postgresql_bool end,"));
            }
        }
        {
            let mut is_std_primitive_bool_as_postgresql_bool_not_null_update_exist = false;
            for element in &parameters.payload.0 {
                if element.std_primitive_bool_as_postgresql_bool_not_null.is_some() {
                    is_std_primitive_bool_as_postgresql_bool_not_null_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_bool_as_postgresql_bool_not_null_update_exist {
                let mut acc = std::string::String::from("std_primitive_bool_as_postgresql_bool_not_null = case ");
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_bool_as_postgresql_bool_not_null {
                        acc.push_str(&format!(
                            "when std_primitive_i64_as_postgresql_big_serial_not_null = {} then {} ",
                            match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&element.std_primitive_i64_as_postgresql_big_serial_not_null, &mut increment) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    let error = TryUpdateManyRouteLogicErrorNamed::BindQuery {
                                        bind_query: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 4369,
                                                column: 254,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                            match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&value.value, &mut increment) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    let error = TryUpdateManyRouteLogicErrorNamed::BindQuery {
                                        bind_query: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 4369,
                                                column: 254,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        ));
                    }
                }
                query.push_str(&format!("{}{}", acc, "else std_primitive_bool_as_postgresql_bool_not_null end,"));
            }
        }
        let _ = query.pop();
        query.push_str(&format!(" where std_primitive_i64_as_postgresql_big_serial_not_null in ({}) returning std_primitive_i64_as_postgresql_big_serial_not_null;", {
            let mut acc = std::string::String::default();
            for element in &parameters.payload.0 {
                match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&element.std_primitive_i64_as_postgresql_big_serial_not_null, &mut increment) {
                    Ok(value) => {
                        acc.push_str(&format!("{value},"));
                    }
                    Err(error_0) => {
                        let error = TryUpdateManyRouteLogicErrorNamed::BindQuery {
                            bind_query: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 4369,
                                    column: 254,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            let _ = acc.pop();
            acc
        }));
        query
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        {
            let mut is_std_primitive_bool_as_postgresql_bool_update_exist = false;
            for element in &parameters.payload.0 {
                if element.std_primitive_bool_as_postgresql_bool.is_some() {
                    is_std_primitive_bool_as_postgresql_bool_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_bool_as_postgresql_bool_update_exist {
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_bool_as_postgresql_bool {
                        query = query.bind(element.std_primitive_i64_as_postgresql_big_serial_not_null);
                        query = postgresql_crud::BindQuerySecond::bind_value_to_query(value.value.clone(), query);
                    }
                }
            }
        }
        {
            let mut is_std_primitive_bool_as_postgresql_bool_not_null_update_exist = false;
            for element in &parameters.payload.0 {
                if element.std_primitive_bool_as_postgresql_bool_not_null.is_some() {
                    is_std_primitive_bool_as_postgresql_bool_not_null_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_bool_as_postgresql_bool_not_null_update_exist {
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_bool_as_postgresql_bool_not_null {
                        query = query.bind(element.std_primitive_i64_as_postgresql_big_serial_not_null);
                        query = postgresql_crud::BindQuerySecond::bind_value_to_query(value.value.clone(), query);
                    }
                }
            }
        }
        {
            for element in &parameters.payload.0 {
                query = query.bind(element.std_primitive_i64_as_postgresql_big_serial_not_null);
            }
        }
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2302,
                            column: 246,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut rows = binded_query.fetch(executor.as_mut());
            let mut acc = std::vec::Vec::new();
            while let Some(value) = match {
                use postgresql_crud::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => match value {
                    Some(value) => match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null") {
                        Ok(value) => Some(value),
                        Err(error_0) => {
                            drop(rows);
                            match executor.rollback().await {
                                Ok(_) => {
                                    let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 2965,
                                                column: 129,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                                Err(error_1) => {
                                    let error = TryUpdateManyRouteLogicErrorNamed::RowAndRollback {
                                        row: error_0,
                                        rollback: error_1,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 2965,
                                                column: 158,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        }
                    },
                    None => None,
                },
                Err(error_0) => {
                    drop(rows);
                    match executor.rollback().await {
                        Ok(_) => {
                            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2967,
                                        column: 125,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryUpdateManyRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2967,
                                        column: 154,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            } {
                acc.push(value);
            }
            acc
        };
        {
            let error_0 = expected_primary_keys.into_iter().fold(std::vec::Vec::new(), |mut acc, element| {
                if let false = value.contains(&element) {
                    acc.push(element);
                }
                acc
            });
            if let false = error_0.is_empty() {
                match executor.rollback().await {
                    Ok(_) => {
                        let error = TryUpdateManyRouteLogicErrorNamed::NonExistingPrimaryKeys {
                            non_existing_primary_keys: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 2165,
                                    column: 13,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                    Err(error_1) => {
                        let error = TryUpdateManyRouteLogicErrorNamed::NonExistingPrimaryKeysAndRollback {
                            non_existing_primary_keys: error_0,
                            rollback: error_1,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 2176,
                                    column: 13,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
            }
        }
        if let Err(error_0) = executor.commit().await {
            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2314,
                        column: 246,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_update_many_route_logic_error_named_with_serialize_deserialize: TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_many(server_location: &std::primitive::str, parameters: UpdateManyParameters) -> Result<std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate>, TryUpdateManyErrorNamed> {
    let payload = {
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.0 {
            if !acc.contains(&&element.std_primitive_i64_as_postgresql_big_serial_not_null) {
                acc.push(&element.std_primitive_i64_as_postgresql_big_serial_not_null);
            } else {
                let error_0 = element.std_primitive_i64_as_postgresql_big_serial_not_null;
                return Err(TryUpdateManyErrorNamed::NotUniquePrimaryKey {
                    not_unique_primary_key: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 4552,
                            column: 208,
                        }),
                    ),
                });
            }
        }
        let value = UpdateManyPayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryUpdateManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2669,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/update_many", server_location,);
    let future = reqwest::Client::new()
        .patch(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryUpdateManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2721,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryUpdateManyErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2738,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryUpdateManyRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryUpdateManyErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2751,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_update_many_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryUpdateManyRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryUpdateManyRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::BindQuery { bind_query, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence },
    };
    Err(TryUpdateManyErrorNamed::TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_update_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                line: 2789,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for UpdateManyPayloadElement {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_primitive_bool_as_postgresql_bool: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_primitive_bool_as_postgresql_bool_not_null: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        }
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for UpdateManyPayload {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(vec![
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ])
    }
}
pub async fn update_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <UpdateManyPayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate,
    pub std_primitive_bool_as_postgresql_bool: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolToUpdate>>,
    pub std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate>>,
}
#[derive(Debug)]
pub struct UpdateOneParameters {
    pub payload: UpdateOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateOneRouteLogicResponseVariants {
    Desirable(postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        no_payload_fields_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateOneRouteLogicErrorNamed> for TryUpdateOneRouteLogicResponseVariants {
    fn from(value: TryUpdateOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence } => Self::BindQuery { bind_query, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence } => Self::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateOneRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        #[eo_to_std_string_string]
        no_payload_fields_primary_key: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2545,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = UpdateOneParameters {
        payload: match serde_json::from_slice::<UpdateOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = UpdateOnePayload::from(value);
                if let (None, None) = (&value.std_primitive_bool_as_postgresql_bool, &value.std_primitive_bool_as_postgresql_bool_not_null) {
                    let error_0 = value.std_primitive_i64_as_postgresql_big_serial_not_null;
                    let error = TryUpdateOneRouteLogicErrorNamed::NoPayloadFieldsPrimaryKey {
                        no_payload_fields_primary_key: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 2083,
                                column: 288,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
                value
            }
            Err(error_0) => {
                let error = TryUpdateOneRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2617,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = {
        let mut increment: std::primitive::u64 = 0;
        let mut query = std::string::String::from("update example set ");
        if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool {
            //here
            match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&value.value, &mut increment) {
                Ok(value) => {
                    query.push_str(&format!("std_primitive_bool_as_postgresql_bool = {value},"));
                }
                Err(error_0) => {
                    let error = TryUpdateOneRouteLogicErrorNamed::BindQuery {
                        bind_query: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 4695,
                                column: 254,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }
        if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
            //here
            match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&value.value, &mut increment) {
                Ok(value) => {
                    query.push_str(&format!("std_primitive_bool_as_postgresql_bool_not_null = {value},"));
                }
                Err(error_0) => {
                    let error = TryUpdateOneRouteLogicErrorNamed::BindQuery {
                        bind_query: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 4695,
                                column: 254,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }
        let _ = query.pop();
        //here
        match postgresql_crud::BindQuerySecond::try_generate_bind_increments(&parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null, &mut increment) {
            Ok(value) => {
                query.push_str(&format!(" where std_primitive_i64_as_postgresql_big_serial_not_null = {value}"));
            }
            Err(error_0) => {
                let error = TryUpdateOneRouteLogicErrorNamed::BindQuery {
                    bind_query: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 4695,
                            column: 254,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        }
        query.push_str(&format!(" returning std_primitive_i64_as_postgresql_big_serial_not_null"));
        query
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters.payload.std_primitive_bool_as_postgresql_bool {
            query = postgresql_crud::BindQuerySecond::bind_value_to_query(value.value, query);
        }
        if let Some(value) = parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
            query = postgresql_crud::BindQuerySecond::bind_value_to_query(value.value, query);
        }
        query = postgresql_crud::BindQuerySecond::bind_value_to_query(parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2302,
                            column: 246,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null") {
                    Ok(value) => value,
                    Err(error_0) => match executor.rollback().await {
                        Ok(_) => {
                            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2981,
                                        column: 112,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryUpdateOneRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2981,
                                        column: 141,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                },
                Err(error_0) => match executor.rollback().await {
                    Ok(_) => {
                        let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 2983,
                                    column: 108,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    Err(error_1) => {
                        let error = TryUpdateOneRouteLogicErrorNamed::RowAndRollback {
                            row: error_0,
                            rollback: error_1,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 2983,
                                    column: 137,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                },
            }
        };
        if let Err(error_0) = executor.commit().await {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2314,
                        column: 246,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_update_one_route_logic_error_named_with_serialize_deserialize: TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_one(server_location: &std::primitive::str, parameters: UpdateOneParameters) -> Result<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate, TryUpdateOneErrorNamed> {
    let payload = {
        let value = UpdateOnePayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryUpdateOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2669,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/update_one", server_location,);
    let future = reqwest::Client::new()
        .patch(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryUpdateOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2721,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryUpdateOneErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2738,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryUpdateOneRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryUpdateOneErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2751,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_update_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryUpdateOneRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryUpdateOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::BindQuery { bind_query, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
    };
    Err(TryUpdateOneErrorNamed::TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize {
        try_update_one_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                line: 2789,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for UpdateOnePayload {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_primitive_bool_as_postgresql_bool: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_primitive_bool_as_postgresql_bool_not_null: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        }
    }
}
pub async fn update_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <UpdateOnePayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DeleteManyPayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null: std::option::Option<std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete>>,
    pub std_primitive_bool_as_postgresql_bool: std::option::Option<std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolWhere>>,
    pub std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNullWhere>>,
}
#[derive(Debug)]
pub struct DeleteManyParameters {
    pub payload: DeleteManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPrimaryKeys {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndRollback {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        not_unique_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBool {
        not_unique_std_primitive_bool_as_postgresql_bool: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
        not_unique_std_primitive_bool_as_postgresql_bool_not_null: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteManyRouteLogicErrorNamed> for TryDeleteManyRouteLogicResponseVariants {
    fn from(value: TryDeleteManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence } => Self::BindQuery { bind_query, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFields { code_occurence } => Self::NoPayloadFields { code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NoPrimaryKeys { code_occurence } => Self::NoPrimaryKeys { code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence } => Self::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence } => Self::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => Self::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBool {
                not_unique_std_primitive_bool_as_postgresql_bool,
                code_occurence,
            } => Self::NotUniqueStdPrimitiveBoolAsPostgresqlBool {
                not_unique_std_primitive_bool_as_postgresql_bool,
                code_occurence,
            },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                not_unique_std_primitive_bool_as_postgresql_bool_not_null,
                code_occurence,
            } => Self::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                not_unique_std_primitive_bool_as_postgresql_bool_not_null,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPrimaryKeys {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndRollback {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete>,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBool {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBool,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveBoolAsPostgresqlBoolNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_many_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2545,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = DeleteManyParameters {
        payload: match serde_json::from_slice::<DeleteManyPayload>(&body_bytes) {
            Ok(value) => {
                let value = DeleteManyPayload::from(value);
                if let (None, None, None) = (&value.std_primitive_bool_as_postgresql_bool, &value.std_primitive_bool_as_postgresql_bool_not_null, &value.std_primitive_i64_as_postgresql_big_serial_not_null) {
                    let error = TryDeleteManyRouteLogicErrorNamed::NoPayloadFields {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                line: 4917,
                                column: 272,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
                if let Some(value) = &value.std_primitive_i64_as_postgresql_big_serial_not_null {
                    if value.is_empty() {
                        let error = TryDeleteManyRouteLogicErrorNamed::NoPrimaryKeys {
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 4925,
                                    column: 268,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                if let Some(value) = &value.std_primitive_i64_as_postgresql_big_serial_not_null {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if !acc.contains(&element) {
                            acc.push(&element);
                        } else {
                            let error_0 = *element;
                            let error = TryDeleteManyRouteLogicErrorNamed::NotUniquePrimaryKey {
                                not_unique_primary_key: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2853,
                                        column: 173,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                if let Some(value) = &value.std_primitive_bool_as_postgresql_bool {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if !acc.contains(&element) {
                            acc.push(&element);
                        } else {
                            let error_0 = element.value.clone();
                            let error = TryDeleteManyRouteLogicErrorNamed::NotUniqueStdPrimitiveBoolAsPostgresqlBool {
                                not_unique_std_primitive_bool_as_postgresql_bool: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2900,
                                        column: 17,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                if let Some(value) = &value.std_primitive_bool_as_postgresql_bool_not_null {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if !acc.contains(&element) {
                            acc.push(&element);
                        } else {
                            let error_0 = element.value.clone();
                            let error = TryDeleteManyRouteLogicErrorNamed::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                                not_unique_std_primitive_bool_as_postgresql_bool_not_null: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2900,
                                        column: 17,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                value
            }
            Err(error_0) => {
                let error = TryDeleteManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2617,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let expected_primary_keys = parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null.clone();
    let query_string = format!("delete from example where {} returning std_primitive_i64_as_postgresql_big_serial_not_null", {
        let mut increment: std::primitive::u64 = 0;
        let mut additional_parameters = std::string::String::default();
        if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool {
            for element in value {
                //here
                match postgresql_crud::BindQuerySecond::try_generate_bind_increments(element, &mut increment) {
                    Ok(value) => {
                        let handle = format!("std_primitive_bool_as_postgresql_bool = {value}");
                        match additional_parameters.is_empty() {
                            true => {
                                additional_parameters.push_str(&handle);
                            }
                            false => {
                                additional_parameters.push_str(&format!(" AND {handle}"));
                            }
                        }
                    }
                    Err(error_0) => {
                        let error = TryDeleteManyRouteLogicErrorNamed::BindQuery {
                            bind_query: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 4948,
                                    column: 258,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
        }
        if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
            for element in value {
                //here
                match postgresql_crud::BindQuerySecond::try_generate_bind_increments(element, &mut increment) {
                    Ok(value) => {
                        let handle = format!("std_primitive_bool_as_postgresql_bool_not_null = {value}");
                        match additional_parameters.is_empty() {
                            true => {
                                additional_parameters.push_str(&handle);
                            }
                            false => {
                                additional_parameters.push_str(&format!(" AND {handle}"));
                            }
                        }
                    }
                    Err(error_0) => {
                        let error = TryDeleteManyRouteLogicErrorNamed::BindQuery {
                            bind_query: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 4948,
                                    column: 258,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
        }
        if let Some(std_primitive_i64_as_postgresql_big_serial_not_null) = &parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null {
            if let false = additional_parameters.is_empty() {
                additional_parameters.push_str(" and");
            }
            additional_parameters.push_str(&format!(" std_primitive_i64_as_postgresql_big_serial_not_null in ({})", {
                let mut additional_parameters = std::string::String::default();
                for element in std_primitive_i64_as_postgresql_big_serial_not_null {
                    //here
                    match postgresql_crud::BindQuerySecond::try_generate_bind_increments(element, &mut increment) {
                        Ok(value) => {
                            additional_parameters.push_str(&format!("{value},"));
                        }
                        Err(error_0) => {
                            let error = TryDeleteManyRouteLogicErrorNamed::BindQuery {
                                bind_query: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 4982,
                                        column: 258,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
                let _ = additional_parameters.pop();
                additional_parameters
            }));
        }
        additional_parameters
    });
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters.payload.std_primitive_bool_as_postgresql_bool {
            for element in value {
                query = postgresql_crud::BindQuerySecond::bind_value_to_query(element, query);
            }
        }
        if let Some(value) = parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
            for element in value {
                query = postgresql_crud::BindQuerySecond::bind_value_to_query(element, query);
            }
        }
        if let Some(std_primitive_i64_as_postgresql_big_serial_not_null) = parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null {
            for element in std_primitive_i64_as_postgresql_big_serial_not_null {
                query = postgresql_crud::BindQuerySecond::bind_value_to_query(element, query);
            }
        }
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2302,
                            column: 246,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut rows = binded_query.fetch(executor.as_mut());
            let mut acc = std::vec::Vec::new();
            while let Some(value) = match {
                use postgresql_crud::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => match value {
                    Some(value) => match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null") {
                        Ok(value) => Some(value),
                        Err(error_0) => {
                            drop(rows);
                            match executor.rollback().await {
                                Ok(_) => {
                                    let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 2965,
                                                column: 129,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                                Err(error_1) => {
                                    let error = TryDeleteManyRouteLogicErrorNamed::RowAndRollback {
                                        row: error_0,
                                        rollback: error_1,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                                line: 2965,
                                                column: 158,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        }
                    },
                    None => None,
                },
                Err(error_0) => {
                    drop(rows);
                    match executor.rollback().await {
                        Ok(_) => {
                            let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2967,
                                        column: 125,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryDeleteManyRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2967,
                                        column: 154,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            } {
                acc.push(value);
            }
            acc
        };
        {
            if let Some(error) = expected_primary_keys {
                let error_0 = error.into_iter().fold(std::vec::Vec::new(), |mut acc, element| {
                    if let false = value.contains(&element) {
                        acc.push(element);
                    }
                    acc
                });
                if let false = error_0.is_empty() {
                    match executor.rollback().await {
                        Ok(_) => {
                            let error = TryDeleteManyRouteLogicErrorNamed::NonExistingPrimaryKeys {
                                non_existing_primary_keys: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2165,
                                        column: 13,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryDeleteManyRouteLogicErrorNamed::NonExistingPrimaryKeysAndRollback {
                                non_existing_primary_keys: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2176,
                                        column: 13,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
            }
        }
        if let Err(error_0) = executor.commit().await {
            let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2314,
                        column: 246,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_delete_many_route_logic_error_named_with_serialize_deserialize: TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_many(server_location: &std::primitive::str, parameters: DeleteManyParameters) -> Result<std::vec::Vec<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete>, TryDeleteManyErrorNamed> {
    let payload = {
        let value = DeleteManyPayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryDeleteManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2669,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/delete_many", server_location,);
    let future = reqwest::Client::new()
        .delete(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryDeleteManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2721,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryDeleteManyErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2738,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryDeleteManyRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryDeleteManyErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2751,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_delete_many_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryDeleteManyRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryDeleteManyRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::BindQuery { bind_query, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::NoPayloadFields { code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFields { code_occurence },
        TryDeleteManyRouteLogicResponseVariants::NoPrimaryKeys { code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NoPrimaryKeys { code_occurence },
        TryDeleteManyRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::NotUniqueStdPrimitiveBoolAsPostgresqlBool {
            not_unique_std_primitive_bool_as_postgresql_bool,
            code_occurence,
        } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBool {
            not_unique_std_primitive_bool_as_postgresql_bool,
            code_occurence,
        },
        TryDeleteManyRouteLogicResponseVariants::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
            not_unique_std_primitive_bool_as_postgresql_bool_not_null,
            code_occurence,
        } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
            not_unique_std_primitive_bool_as_postgresql_bool_not_null,
            code_occurence,
        },
    };
    Err(TryDeleteManyErrorNamed::TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_delete_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                line: 2789,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for DeleteManyPayload {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null: Some(vec![
                postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            ]),
            std_primitive_bool_as_postgresql_bool: Some(vec![
                postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            ]),
            std_primitive_bool_as_postgresql_bool_not_null: Some(vec![
                postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            ]),
        }
    }
}
pub async fn delete_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <DeleteManyPayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DeleteOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete,
}
#[derive(Debug)]
pub struct DeleteOneParameters {
    pub payload: DeleteOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteOneRouteLogicResponseVariants {
    Desirable(postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneRouteLogicErrorNamed> for TryDeleteOneRouteLogicResponseVariants {
    fn from(value: TryDeleteOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteOneRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2545,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = DeleteOneParameters {
        payload: match serde_json::from_slice::<DeleteOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = DeleteOnePayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryDeleteOneRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2617,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = format!("delete from example where std_primitive_i64_as_postgresql_big_serial_not_null = $1 returning std_primitive_i64_as_postgresql_big_serial_not_null");
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        query = postgresql_crud::BindQuerySecond::bind_value_to_query(parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2567,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2302,
                            column: 246,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => match sqlx::Row::try_get::<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null") {
                    Ok(value) => value,
                    Err(error_0) => match executor.rollback().await {
                        Ok(_) => {
                            let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2981,
                                        column: 112,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryDeleteOneRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                        line: 2981,
                                        column: 141,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                },
                Err(error_0) => match executor.rollback().await {
                    Ok(_) => {
                        let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 2983,
                                    column: 108,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    Err(error_1) => {
                        let error = TryDeleteOneRouteLogicErrorNamed::RowAndRollback {
                            row: error_0,
                            rollback: error_1,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                                    line: 2983,
                                    column: 137,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                },
            }
        };
        if let Err(error_0) = executor.commit().await {
            let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2314,
                        column: 246,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_delete_one_route_logic_error_named_with_serialize_deserialize: TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_one(server_location: &std::primitive::str, parameters: DeleteOneParameters) -> Result<postgresql_crud::postgresql_types::base_wrap::StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete, TryDeleteOneErrorNamed> {
    let payload = {
        let value = DeleteOnePayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryDeleteOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                            line: 2669,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/delete_one", server_location,);
    let future = reqwest::Client::new()
        .delete(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryDeleteOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2721,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryDeleteOneErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2738,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryDeleteOneRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryDeleteOneErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                        line: 2751,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_delete_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryDeleteOneRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryDeleteOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryDeleteOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryDeleteOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryDeleteOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryDeleteOneRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
    };
    Err(TryDeleteOneErrorNamed::TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize {
        try_delete_one_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud_second/src/lib.rs"),
                line: 2789,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for DeleteOnePayload {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn delete_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <DeleteOnePayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
