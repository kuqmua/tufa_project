mod f;

#[derive(
    Debug,
    postgresql_crud::GeneratePostgresqlCrud
)]
#[postgresql_crud::create_many_additional_error_variants{enum CreateManyAdditionalErrorVariants{}}]
#[postgresql_crud::create_one_additional_error_variants{enum CreateOneAdditionalErrorVariants{}}]
#[postgresql_crud::read_many_additional_error_variants{enum ReadManyAdditionalErrorVariants{}}]
#[postgresql_crud::read_one_additional_error_variants{enum ReadOneAdditionalErrorVariants{}}]
#[postgresql_crud::update_many_additional_error_variants{enum UpdateManyAdditionalErrorVariants{}}]
#[postgresql_crud::update_one_additional_error_variants{enum UpdateOneAdditionalErrorVariants{}}]
#[postgresql_crud::delete_many_additional_error_variants{enum DeleteManyAdditionalErrorVariants{}}]
#[postgresql_crud::delete_one_additional_error_variants{enum DeleteOneAdditionalErrorVariants{}}]
#[postgresql_crud::common_additional_error_variants{
    enum CommonAdditionalErrorVariants {
        CheckCommit {
            #[eo_error_occurence]
            check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
}]
#[postgresql_crud::create_many_additional_route_logic{
    println!("kekw");
}]
#[postgresql_crud::create_one_additional_route_logic{}]
#[postgresql_crud::read_many_additional_route_logic{}]
#[postgresql_crud::read_one_additional_route_logic{}]
#[postgresql_crud::update_many_additional_route_logic{}]
#[postgresql_crud::update_one_additional_route_logic{}]
#[postgresql_crud::delete_many_additional_route_logic{}]
#[postgresql_crud::delete_one_additional_route_logic{}]
#[postgresql_crud::common_additional_route_logic{
    // if let Err(error) = postgresql_crud::check_commit::check_commit(
    //     *app_state.get_enable_api_git_commit_check(),
    //     &headers,
    // ) {
    //     let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
    //     //todo use reserved work instead of TryCreateManyRouteLogicErrorNamed
    //     let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
    //         check_commit: error,
    //         code_occurence: error_occurence_lib::code_occurence!(),
    //     };
    //     eprintln!("{error}");
    //     let mut response = axum::response::IntoResponse::into_response(axum::Json(
    //         TryCreateManyRouteLogicResponseVariants::from(error),
    //     ));
    //     *response.status_mut() = status_code;
    //     return response;
    // }
}]
pub struct Jsongeneric {
    // pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdPrimitiveBoolAsPostgresqlBool,
    pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    // pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt,
    // pub std_primitive_i16_as_postgresql_small_int_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallIntNotNull,
    // pub std_primitive_i16_as_postgresql_small_serial: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerial,
    // pub std_primitive_i16_as_postgresql_small_serial_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    // pub std_primitive_i16_as_postgresql_small_int2: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2,
    // pub std_primitive_i16_as_postgresql_small_int2_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2NotNull,

    // pub std_primitive_i32_as_postgresql_int: postgresql_crud::StdPrimitiveI32AsPostgresqlInt,
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

    //todo what to do with object?
    // pub sqlx_types_json_t_as_postgresql_json: postgresql_crud::SqlxTypesJsonAsPostgresqlJson::<Something>,//todo
    // postgresql_crud::StdPrimitiveBoolAsPostgresqlBool,//
    // pub sqlx_types_json_t_as_postgresql_json_not_null: postgresql_crud::SqlxTypesJsonAsPostgresqlJsonNotNull::<Something>
    // pub sqlx_types_json_t_as_postgresql_json_b: postgresql_crud::SqlxTypesJsonAsPostgresqlJsonB::<<Something>,//todo
    pub sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::SqlxTypesJsonAsPostgresqlJsonBNotNull<Something>, //todo

                                                                                                                            // pub serde_json_value_as_postgresql_json: postgresql_crud::SerdeJsonValueAsPostgresqlJson,
                                                                                                                            // pub serde_json_value_as_postgresql_json_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonNotNull,
                                                                                                                            // pub serde_json_value_as_postgresql_json_b: postgresql_crud::SerdeJsonValueAsPostgresqlJsonB,
                                                                                                                            // pub serde_json_value_as_postgresql_json_b_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonBNotNull,
}

//todo enum tree support
//todo generate wrapper type for all possible json type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    //
    postgresql_crud::GeneratePostgresqlQueryPart
)] //user type must implement utoipa::ToSchema trait
pub struct Something {
    // pub id: postgresql_crud::json_types::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    pub std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8,
    // pub std_primitive_i16: postgresql_crud::json_types::StdPrimitiveI16,
    // pub std_primitive_i32: postgresql_crud::json_types::StdPrimitiveI32,
    // pub std_primitive_i64: postgresql_crud::json_types::StdPrimitiveI64,
    // pub std_primitive_u8: postgresql_crud::json_types::StdPrimitiveU8,
    // pub std_primitive_u16: postgresql_crud::json_types::StdPrimitiveU16,
    // pub std_primitive_u32: postgresql_crud::json_types::StdPrimitiveU32,
    // pub std_primitive_u64: postgresql_crud::json_types::StdPrimitiveU64,
    // pub std_primitive_f32: postgresql_crud::json_types::StdPrimitiveF32,
    // pub std_primitive_f64: postgresql_crud::json_types::StdPrimitiveF64,
    // pub std_primitive_bool: postgresql_crud::json_types::StdPrimitiveBool,
    // pub std_string_string: postgresql_crud::json_types::StdStringString,
    pub std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8,
    // pub std_option_option_std_primitive_i16: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_primitive_i32: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_primitive_i64: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_primitive_u8: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_primitive_u16: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_primitive_u32: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_primitive_u64: postgresql_crud::json_types::StdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_primitive_f32: postgresql_crud::json_types::StdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_primitive_f64: postgresql_crud::json_types::StdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_primitive_bool: postgresql_crud::json_types::StdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_string_string: postgresql_crud::json_types::StdOptionOptionStdStringString,
    pub std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdPrimitiveI8,
    // pub std_vec_vec_std_primitive_i16: postgresql_crud::json_types::StdVecVecStdPrimitiveI16,
    // pub std_vec_vec_std_primitive_i32: postgresql_crud::json_types::StdVecVecStdPrimitiveI32,
    // pub std_vec_vec_std_primitive_i64: postgresql_crud::json_types::StdVecVecStdPrimitiveI64,
    // pub std_vec_vec_std_primitive_u8: postgresql_crud::json_types::StdVecVecStdPrimitiveU8,
    // pub std_vec_vec_std_primitive_u16: postgresql_crud::json_types::StdVecVecStdPrimitiveU16,
    // pub std_vec_vec_std_primitive_u32: postgresql_crud::json_types::StdVecVecStdPrimitiveU32,
    // pub std_vec_vec_std_primitive_u64: postgresql_crud::json_types::StdVecVecStdPrimitiveU64,
    // pub std_vec_vec_std_primitive_f32: postgresql_crud::json_types::StdVecVecStdPrimitiveF32,
    // pub std_vec_vec_std_primitive_f64: postgresql_crud::json_types::StdVecVecStdPrimitiveF64,
    // pub std_vec_vec_std_primitive_bool: postgresql_crud::json_types::StdVecVecStdPrimitiveBool,
    // pub std_vec_vec_std_string_string: postgresql_crud::json_types::StdVecVecStdStringString,
    pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI8,
    // pub std_option_option_std_vec_vec_std_primitive_i16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_primitive_i32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_primitive_i64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_primitive_u8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_primitive_u16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_primitive_u32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_primitive_u64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_primitive_f32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_primitive_f64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_primitive_bool: postgresql_crud::json_types::StdOptionOptionStdVecVecStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_string_string: postgresql_crud::json_types::StdOptionOptionStdVecVecStdStringString,
    pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI8,
    // pub std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::json_types::StdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_vec_vec_std_option_option_std_string_string: postgresql_crud::json_types::StdVecVecStdOptionOptionStdStringString,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_option_option_std_string_string: postgresql_crud::json_types::StdOptionOptionStdVecVecStdOptionOptionStdStringString,

    // pub object: ObjectDoggie,
    // pub std_option_option_object: StdOptionOptionObjectDoggie,

    // pub std_vec_vec_object_with_id: StdVecVecObjectWithIdDoggie,
    // pub std_option_option_std_vec_vec_object_with_id: StdOptionOptionStdVecVecObjectWithIdDoggie
}

// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     // Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     //
//     postgresql_crud::GeneratePostgresqlQueryPart,
// )] //user type must implement utoipa::ToSchema trait
// pub struct Doggie {
//     pub std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8,
//     pub std_primitive_i16: postgresql_crud::json_types::StdPrimitiveI16,

//     // pub object: ObjectCat,
//     // pub std_option_option_object: StdOptionOptionObjectCat,

//     // pub std_vec_vec_object_with_id: StdVecVecObjectWithIdCat,
//     // pub std_option_option_std_vec_vec_object_with_id: StdOptionOptionStdVecVecObjectWithIdCat,
// }

// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     // Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     //
//     postgresql_crud::GeneratePostgresqlQueryPart,
// )] //user type must implement utoipa::ToSchema trait
// pub struct Cat {
//     // pub id: postgresql_crud::json_types::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids
//     pub std_primitive_i32: postgresql_crud::json_types::StdPrimitiveI32,
//     pub std_primitive_i64: postgresql_crud::json_types::StdPrimitiveI64,

//     pub object: ObjectBird,
//     pub std_option_option_object: StdOptionOptionObjectBird,

//     pub std_vec_vec_object_with_id: StdVecVecObjectWithIdBird,
//     pub std_option_option_std_vec_vec_object_with_id: StdOptionOptionStdVecVecObjectWithIdBird,
// }

// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     // Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     //
//     postgresql_crud::GeneratePostgresqlQueryPart,
// )] //user type must implement utoipa::ToSchema trait
// pub struct Bird {
//     // pub id: postgresql_crud::json_types::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids
//     pub std_primitive_u8: postgresql_crud::json_types::StdPrimitiveU8,
//     pub std_primitive_u16: postgresql_crud::json_types::StdPrimitiveU16,
// }

// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     //
//     // postgresql_crud::GeneratePostgresqlQueryPart,
// )] //user type must implement utoipa::ToSchema trait
// pub struct Mouse {
//     // pub id: postgresql_crud::json_types::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids
//     pub std_primitive_i32: postgresql_crud::json_types::StdPrimitiveI32,
// }

#[test]
fn test_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() {
    // let default = postgresql_crud::json_types::Object(Something{

    // });
    // println!("{default:#?}");
    // let default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element: postgresql_crud::json_types::Object::<Something> = postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element();
    //     println!("{default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element:#?}");
    // let serialized = serde_json::to_string(&default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element).unwrap();
    //     println!("{serialized:#?}");
    //     let deserialized: postgresql_crud::json_types::Object::<Something> = serde_json::from_str(&serialized).unwrap();
    //     println!("{deserialized:#?}");

    // let s = Something {
    //     std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8(8),
    //     std_primitive_i16: postgresql_crud::json_types::StdPrimitiveI16(16),
    // };

    // #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
    // struct Thing {
    //     pub std_primitive_i8: postgresql_crud::json_types::StdPrimitiveI8,
    // }
    // let schema = schemars::schema_for!(Thing);
    // println!("{}", serde_json::to_string_pretty(&schema).unwrap());

    // let f = ReadOnePayload {
    //     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdPrimitiveI64(42),
    //     // select: std::vec::Vec<JsonobjectColumn>,
    //     select: vec![
    //         JsonobjectColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull,
    //         JsonobjectColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
    //         JsonobjectColumn::SqlxTypesJsonTAsPostgresqlJsonBNotNull {
    //             filter: vec![
    //                 SomethingFieldToRead::StdPrimitiveI8(
    //                     postgresql_crud::json_types::StdPrimitiveI8FieldReader{}
    //                 ),
    //                 SomethingFieldToRead::StdOptionOptionStdPrimitiveI8(
    //                     postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8FieldReader{}
    //                 )
    //             ]
    //         }
    //     ]
    // };
    
    // SomethingOptionsToUpdate(vec![
    //     // SomethingOptionToUpdate::StdPrimitiveI8(postgresql_crud::Value {
    //     //     value: std::primitive::i8::default()
    //     // }),
    //     //here
    //     SomethingOptionToUpdate::StdVecVecObjectWithId(postgresql_crud::Value {
    //         value: vec![
    //             DoggieOptionsToUpdate { id: postgresql_crud::json_types::Uuid(uuid::uuid!("d6a4aa72-b154-4699-889f-33ef34a8c7f2")), fields: vec![DoggieOptionToUpdate::StdPrimitiveI16(postgresql_crud::Value { value: std::primitive::i16::default() })] },
    //             // DoggieOptionsToUpdate(postgresql_crud::json_types::ArrayElementChange::Create(DoggieToCreate {
    //             //     std_primitive_i16: postgresql_crud::json_types::StdPrimitiveI16::default(),
    //             // })),
    //             // DoggieOptionsToUpdate(postgresql_crud::json_types::ArrayElementChange::Delete(postgresql_crud::json_types::Uuid(uuid::uuid!("d6a4aa72-b154-4699-889f-33ef34a8c7f2")))),

    //             // StdVecVecObjectWithId(postgresql_crud::Value<std::vec::Vec<DoggieJsonArrayElementChange>>),
    //             // #[serde(rename(
    //             //     serialize = "std_option_option_std_vec_vec_object_with_id",
    //             //     deserialize = "std_option_option_std_vec_vec_object_with_id"
    //             // ))]
    //             // StdOptionOptionStdVecVecObjectWithId(
    //             //     postgresql_crud::Value<std::vec::Vec<std::option::Option<DoggieJsonArrayElementChange>>>,
    //             // ),
    //         ],
    //     }),
    // ]);
    // println!("{f:#?}");
    // let serialized = serde_json::to_string(&f).unwrap();
    // println!("{serialized:#?}");
    println!("---------------");
    let g = SomethingReader(SomethingOptionsToRead {
        // std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdPrimitiveI8OptionsToRead>>,
        // std_primitive_i8: Some(postgresql_crud::Value { value: postgresql_crud::json_types::StdPrimitiveI8OptionsToRead(1) }),
        // std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead>>,
        // std_option_option_std_primitive_i8: Some(postgresql_crud::Value { value: postgresql_crud::json_types::StdOptionOptionStdPrimitiveI8OptionsToRead(Some(1)) }),

        //here

        // std_vec_vec_std_primitive_i8: Some(postgresql_crud::Value { value: postgresql_crud::json_types::StdVecVecStdPrimitiveI8OptionsToRead(vec![1]) }),
        // object: Some(postgresql_crud::Value { value: ObjectCatOptionsToRead {
        //     std_primitive_i32: Some(postgresql_crud::Value {
        //         value: postgresql_crud::json_types::StdPrimitiveI32OptionsToRead(1)
        //     })
        // }}),
        // std_option_option_object: Some(postgresql_crud::Value { value: StdOptionOptionObjectCatOptionsToRead(None)

        // std_vec_vec_object_with_id: Some(postgresql_crud::Value { value: StdVecVecObjectWithIdDoggieOptionsToRead(vec![
        //     StdVecVecObjectWithIdDoggieOptionsToReadOrigin {
        //         id: Some(postgresql_crud::Value {
        //             value: postgresql_crud::json_types::UuidOptionsToRead::default()
        //         }),
        //         std_primitive_i16: Some(postgresql_crud::Value {
        //             value: postgresql_crud::json_types::StdPrimitiveI16OptionsToRead(1)
        //         })
        //     }
        // ])}),
        std_option_option_std_vec_vec_object_with_id: Some(postgresql_crud::Value { value: StdOptionOptionStdVecVecObjectWithIdDoggieOptionsToRead(Some(vec![
            StdOptionOptionStdVecVecObjectWithIdDoggieOptionsToReadOrigin {
                id: Some(postgresql_crud::Value {
                    value: postgresql_crud::json_types::UuidOptionsToRead::default()
                }),
                std_primitive_i16: Some(postgresql_crud::Value {
                    value: postgresql_crud::json_types::StdPrimitiveI16OptionsToRead(1)
                })
            }
        ]))}),
            
            // {
            // std_primitive_i32: Some(postgresql_crud::Value {
            //     value: postgresql_crud::json_types::StdPrimitiveI32OptionsToRead(1)
            // })
            // }
        // }),
        
        // postgresql_crud::json_types::StdVecVecStdPrimitiveI8
    });
    println!("{g:#?}");
    let serialized = serde_json::to_string(&g).unwrap();
    println!("{serialized:#?}");

}

/////////start block code for trying implement partial
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdPrimitiveI64,
    pub sqlx_types_json_t_as_postgresql_json_b_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::SqlxTypesJson<SomethingOptionToUpdate>>>,
}
#[derive(Debug)]
pub struct UpdateOneParameters {
    pub payload: UpdateOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateOneRouteLogicResponseVariants {
    Desirable(postgresql_crud::StdPrimitiveI64),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    BindQuery { bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: std::string::String, rollback: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
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
        no_payload_fields_primary_key: postgresql_crud::StdPrimitiveI64,
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
            let error = TryUpdateOneRouteLogicErrorNamed::CheckBodySize { check_body_size: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2577, column: 17 })) };
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
                if let None = &value.sqlx_types_json_t_as_postgresql_json_b_not_null {
                    let error_0 = value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;
                    let error = TryUpdateOneRouteLogicErrorNamed::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2059, column: 13 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
                value
            }
            Err(error_0) => {
                let error = TryUpdateOneRouteLogicErrorNamed::SerdeJson { serde_json: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2668, column: 13 })) };
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
        let mut query = std::string::String::from("update jsongeneric set ");
        if let Some(value) = &parameters.payload.sqlx_types_json_t_as_postgresql_json_b_not_null {
            // Json(
            //     SomethingOptionsToUpdate(
            //         [
            //             StdPrimitiveI8(
            //                 Value {
            //                     value: 10,
            //                 },
            //             ),
            //             Object(
            //                 Value {
            //                     value: DoggieOptionsToUpdate(
            //                         [
            //                             StdStringString(
            //                                 Value {
            //                                     value: "fkkkkf",
            //                                 },
            //                             ),
            //                         ],
            //                     ),
            //                 },
            //             ),
            //         ],
            //     ),
            // )

            // {
            //     "object": {
            //       "std_string_string": "sdsd"
            //     },
            //     "std_primitive_i8": 0
            // }

            query.push_str("sqlx_types_json_t_as_postgresql_json_b_not_null = ");
            match value.value.0.0.try_generate_postgresql_query_part_to_update(
                "sqlx_types_json_t_as_postgresql_json_b_not_null",
                "sqlx_types_json_t_as_postgresql_json_b_not_null", //sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_object'->0->'object'
                "",                                                //{std_vec_vec_object,0}
                &mut increment,
            ) {
                Ok(value) => {
                    query.push_str(&value);
                }
                Err(error) => {
                    println!("here {error:#?}");
                    todo!()
                }
            }
            query.push_str(" ");

            // UPDATE jsongeneric
            // SET sqlx_types_json_t_as_postgresql_json_b_not_null =
            //     jsonb_set(
            //         jsonb_set(
            //             jsonb_set(
            //                 sqlx_types_json_t_as_postgresql_json_b_not_null,
            //                 '{std_primitive_i8}',
            //                 '44'
            //             ),
            //             '{object,std_string_string}',
            //             '"something"'
            //         ),
            //         '{std_option_option_object}',
            //         -- jsonb_build_object('std_option_option_object', 'yeees')
            //         'null'
            //     )
            // WHERE std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14
            // RETURNING std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;
        }
        let _ = query.pop();
        match postgresql_crud::BindQuery::try_increment(&parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, &mut increment) {
            Ok(_) => {
                query.push_str(&format!(" where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = ${increment}"));
            }
            Err(error_0) => {
                let error = TryUpdateOneRouteLogicErrorNamed::BindQuery { bind_query: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 4822, column: 25 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        }
        query.push_str(&format!(" returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"));
        query
    };
    //update jsongeneric set sqlx_types_json_t_as_postgresql_json_b_not_null = jsonb_set(sqlx_types_json_t_as_postgresql_json_b_not_null,'{}',$2) where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = $3 returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters.payload.sqlx_types_json_t_as_postgresql_json_b_not_null {
            query = value.value.0.0.bind_value_to_postgresql_query_part_to_update(query);
            // for element in value.value.0.0.0 {
            //     match element {
            //         SomethingOptionToUpdate::StdPrimitiveI8(value) => {
            //             query = query.bind(sqlx::types::Json(value.value));//sqlx::types::Json
            //         }
            //         SomethingOptionToUpdate::Object(value) => {
            //             for element in value.value.0 {
            //                 match element {
            //                     DoggieOptionToUpdate::StdStringString(value) => {
            //                         query = query.bind(sqlx::types::Json(value.value));
            //                     }
            //                 }
            //             }
            //         }
            //     }
            // }

            // query = query.bind(sqlx::types::Json(value.value));
            // query = postgresql_crud::BindQuery::bind_value_to_query(value.value.0.0, query);
            // query = postgresql_crud::BindQuery::bind_value_to_query(sqlx::types::Json(postgresql_crud::json_types::StdStringString(std::string::String::new("cat"))), query);
            // query = postgresql_crud::BindQuery::bind_value_to_query(sqlx::types::Json(postgresql_crud::json_types::StdPrimitiveI8(42)), query);
        }
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2609, column: 17 })) };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2609, column: 17 })) };
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
                let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2289, column: 17 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => match sqlx::Row::try_get::<std::primitive::i64, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
                    Ok(value) => postgresql_crud::StdPrimitiveI64(value),
                    Err(error_0) => match executor.rollback().await {
                        Ok(_) => {
                            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 3058, column: 17 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryUpdateOneRouteLogicErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 3061, column: 17 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                },
                Err(error_0) => match executor.rollback().await {
                    Ok(_) => {
                        let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 3068, column: 13 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    Err(error_1) => {
                        let error = TryUpdateOneRouteLogicErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 3071, column: 13 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                },
            }
        };
        if let Err(error_0) = executor.commit().await {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2307, column: 17 })) };
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
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum TryUpdateOneErrorNamed {
//     SerdeJsonToString {
//         #[eo_to_std_string_string]
//         serde_json_to_string: serde_json::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     FailedToGetResponseText {
//         #[eo_to_std_string_string]
//         status_code: http::StatusCode,
//         #[eo_to_std_string_string]
//         headers: reqwest::header::HeaderMap,
//         #[eo_to_std_string_string]
//         reqwest: reqwest::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     DeserializeResponse {
//         #[eo_to_std_string_string]
//         status_code: http::StatusCode,
//         #[eo_to_std_string_string]
//         headers: reqwest::header::HeaderMap,
//         #[eo_to_std_string_string_serialize_deserialize]
//         response_text: std::string::String,
//         #[eo_to_std_string_string]
//         serde: serde_json::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Reqwest {
//         #[eo_to_std_string_string]
//         reqwest: reqwest::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize {
//         #[eo_to_std_string_string]
//         try_update_one_route_logic_error_named_with_serialize_deserialize: TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// pub async fn try_update_one(server_location: &std::primitive::str, parameters: UpdateOneParameters) -> Result<postgresql_crud::StdPrimitiveI64, TryUpdateOneErrorNamed> {
//     let payload = {
//         let value = UpdateOnePayload::from(parameters.payload);
//         match serde_json::to_string(&value) {
//             Ok(value) => value,
//             Err(error_0) => {
//                 return Err(TryUpdateOneErrorNamed::SerdeJsonToString { serde_json_to_string: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2711, column: 17 })) });
//             }
//         }
//     };
//     let url = format!("{}/jsongeneric/update_one", server_location,);
//     let future = reqwest::Client::new().patch(&url).header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
//     let response = match future.await {
//         Ok(value) => value,
//         Err(error_0) => {
//             return Err(TryUpdateOneErrorNamed::Reqwest { reqwest: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2774, column: 17 })) });
//         }
//     };
//     let error_0 = response.status();
//     let error_1 = response.headers().clone();
//     let error_2 = match response.text().await {
//         Ok(value) => value,
//         Err(error_2) => {
//             return Err(TryUpdateOneErrorNamed::FailedToGetResponseText { status_code: error_0, headers: error_1, reqwest: error_2, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2796, column: 17 })) });
//         }
//     };
//     let expected_response = match serde_json::from_str::<TryUpdateOneRouteLogicResponseVariants>(&error_2) {
//         Ok(value) => value,
//         Err(error_3) => {
//             return Err(TryUpdateOneErrorNamed::DeserializeResponse { status_code: error_0, headers: error_1, response_text: error_2, serde: error_3, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2814, column: 17 })) });
//         }
//     };
//     let try_update_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
//         TryUpdateOneRouteLogicResponseVariants::Desirable(value) => {
//             let value = postgresql_crud::StdPrimitiveI64::from(value);
//             return Ok(value);
//         }
//         TryUpdateOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
//         TryUpdateOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
//         TryUpdateOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
//         TryUpdateOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
//         TryUpdateOneRouteLogicResponseVariants::BindQuery { bind_query, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence },
//         TryUpdateOneRouteLogicResponseVariants::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence },
//         TryUpdateOneRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
//     };
//     Err(TryUpdateOneErrorNamed::TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize { try_update_one_route_logic_error_named_with_serialize_deserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2857, column: 17 })) })
// }
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for UpdateOnePayload {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self { std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }
    }
}
pub async fn update_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(<UpdateOnePayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
/////

#[test]
fn test_dd() {
    // let d = SomethingOptionsToUpdate(vec![
    //     SomethingOptionToUpdate::StdPrimitiveI8(postgresql_crud::Value{ value: 4 }),
    //     SomethingOptionToUpdate::StdPrimitiveI16(postgresql_crud::Value{ value: 5 })
    // ]);
    // println!("{d:#?}");
    // let serialized = serde_json::to_string(&d).unwrap();
    // println!("{serialized:#?}");
    // let deserialized: SomethingOptionsToUpdate = serde_json::from_str(&serialized).unwrap();
    // println!("{deserialized:#?}");
}
/////////