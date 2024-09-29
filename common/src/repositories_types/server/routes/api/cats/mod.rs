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

    //todo what to do with generic?
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
    // pub id: postgresql_crud::JsonUuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids
    // pub std_primitive_i8: postgresql_crud::JsonStdPrimitiveI8,
    // pub std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16,
    // pub std_primitive_i32: postgresql_crud::JsonStdPrimitiveI32,
    // pub std_primitive_i64: postgresql_crud::JsonStdPrimitiveI64,
    // pub std_primitive_u8: postgresql_crud::JsonStdPrimitiveU8,
    // pub std_primitive_u16: postgresql_crud::JsonStdPrimitiveU16,
    // pub std_primitive_u32: postgresql_crud::JsonStdPrimitiveU32,
    // pub std_primitive_u64: postgresql_crud::JsonStdPrimitiveU64,
    // pub std_primitive_f32: postgresql_crud::JsonStdPrimitiveF32,
    // pub std_primitive_f64: postgresql_crud::JsonStdPrimitiveF64,
    // pub std_primitive_bool: postgresql_crud::JsonStdPrimitiveBool,
    // pub std_string_string: postgresql_crud::JsonStdStringString,
    // pub std_option_option_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdPrimitiveI8,
    // pub std_option_option_std_primitive_i16: postgresql_crud::JsonStdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_primitive_i32: postgresql_crud::JsonStdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_primitive_i64: postgresql_crud::JsonStdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_primitive_u8: postgresql_crud::JsonStdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_primitive_u16: postgresql_crud::JsonStdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_primitive_u32: postgresql_crud::JsonStdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_primitive_u64: postgresql_crud::JsonStdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_primitive_f32: postgresql_crud::JsonStdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_primitive_f64: postgresql_crud::JsonStdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_primitive_bool: postgresql_crud::JsonStdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_string_string: postgresql_crud::JsonStdOptionOptionStdStringString,
    // pub std_vec_vec_std_primitive_i8: postgresql_crud::JsonStdVecVecStdPrimitiveI8,
    // pub std_vec_vec_std_primitive_i16: postgresql_crud::JsonStdVecVecStdPrimitiveI16,
    // pub std_vec_vec_std_primitive_i32: postgresql_crud::JsonStdVecVecStdPrimitiveI32,
    // pub std_vec_vec_std_primitive_i64: postgresql_crud::JsonStdVecVecStdPrimitiveI64,
    // pub std_vec_vec_std_primitive_u8: postgresql_crud::JsonStdVecVecStdPrimitiveU8,
    // pub std_vec_vec_std_primitive_u16: postgresql_crud::JsonStdVecVecStdPrimitiveU16,
    // pub std_vec_vec_std_primitive_u32: postgresql_crud::JsonStdVecVecStdPrimitiveU32,
    // pub std_vec_vec_std_primitive_u64: postgresql_crud::JsonStdVecVecStdPrimitiveU64,
    // pub std_vec_vec_std_primitive_f32: postgresql_crud::JsonStdVecVecStdPrimitiveF32,
    // pub std_vec_vec_std_primitive_f64: postgresql_crud::JsonStdVecVecStdPrimitiveF64,
    // pub std_vec_vec_std_primitive_bool: postgresql_crud::JsonStdVecVecStdPrimitiveBool,
    // pub std_vec_vec_std_string_string: postgresql_crud::JsonStdVecVecStdStringString,
    // pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8,
    // pub std_option_option_std_vec_vec_std_primitive_i16: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_primitive_i32: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_primitive_i64: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_primitive_u8: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_primitive_u16: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_primitive_u32: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_primitive_u64: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_primitive_f32: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_primitive_f64: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_primitive_bool: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_string_string: postgresql_crud::JsonStdOptionOptionStdVecVecStdStringString,
    // pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8,
    // pub std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_vec_vec_std_option_option_std_string_string: postgresql_crud::JsonStdVecVecStdOptionOptionStdStringString,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_option_option_std_string_string: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString,

    // pub generic: GenericCat,//postgresql_crud::JsonGeneric<Cat>,
    // pub std_option_option_generic: StdOptionOptionGenericCat,//postgresql_crud::JsonStdOptionOptionGeneric<Cat>,

    pub std_vec_vec_generic_with_id: StdVecVecGenericWithIdDoggie,//postgresql_crud::JsonStdVecVecGenericWithId<Doggie>,
    // pub std_option_option_std_vec_vec_generic_with_id: StdOptionOptionStdVecVecGenericWithIdDoggie//postgresql_crud::JsonStdOptionOptionStdVecVecGenericWithId<Doggie>,
}

// impl postgresql_crud::GeneratePostgresqlQueryPartFieldToRead for GenericCatFieldReader {
//     fn generate_postgresql_query_part_field_to_read(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
//         let mut acc = std::string::String::default();
//         for element in &self.0 {
//             match element {
//                 CatFieldToRead::StdPrimitiveI32(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('generic',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_primitive_i32",
//                             &format!("{column_name_and_maybe_field_getter}->'std_primitive_i32'"),
//                             &format!("{column_name_and_maybe_field_getter_for_error_message}.std_primitive_i32"),
//                         )
//                     ));
//                 }
//             }
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         format!("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object')) end")
//     }
// }

// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum SomethingFieldToRead {
//     #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
//     StdPrimitiveI8(postgresql_crud::JsonStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdPrimitiveI8(postgresql_crud::JsonStdOptionOptionStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
//     StdVecVecStdPrimitiveI8(postgresql_crud::JsonStdVecVecStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
//     // #[serde(rename(serialize = "generic", deserialize = "generic"))]
//     // Generic(GenericCatFieldReader),
//     // #[serde(rename(serialize = "std_option_option_generic", deserialize = "std_option_option_generic"))]
//     // StdOptionOptionGeneric(StdOptionOptionGenericCatFieldReader),
//     // #[serde(rename(serialize = "std_vec_vec_generic_with_id", deserialize = "std_vec_vec_generic_with_id"))]
//     // StdVecVecGenericWithId(StdVecVecGenericWithIdDoggieFieldReader),
//     // #[serde(rename(serialize = "std_option_option_std_vec_vec_generic_with_id", deserialize = "std_option_option_std_vec_vec_generic_with_id"))]
//     // StdOptionOptionStdVecVecGenericWithId(StdOptionOptionStdVecVecGenericWithIdDoggieFieldReader)
// }

#[derive(
    Debug,
    Clone,
    PartialEq,
    // Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    //
    postgresql_crud::GeneratePostgresqlQueryPart,
)] //user type must implement utoipa::ToSchema trait
pub struct Doggie {
    // pub id: postgresql_crud::JsonUuid, //todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    pub std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16,
    // pub generic: postgresql_crud::JsonGeneric<Cat>,
    // pub std_option_option_generic: postgresql_crud::JsonStdOptionOptionGeneric<Cat>,
    // pub std_vec_vec_generic_with_id: postgresql_crud::JsonStdVecVecGenericWithId<Cat>,
    // pub std_option_option_std_vec_vec_generic_with_id: postgresql_crud::JsonStdOptionOptionStdVecVecGenericWithId<Cat>,
}

// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum DoggieFieldToRead {
//     #[serde(rename(serialize = "id", deserialize = "id"))]
//     Id(postgresql_crud::JsonUuidFieldReader),
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32(postgresql_crud::JsonStdPrimitiveI32FieldReader),
// }

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    //
    postgresql_crud::GeneratePostgresqlQueryPart,
)] //user type must implement utoipa::ToSchema trait
pub struct Cat {
    // pub id: postgresql_crud::JsonUuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids
    pub std_primitive_i32: postgresql_crud::JsonStdPrimitiveI32,
}

// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum CatFieldToRead {
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32(postgresql_crud::JsonStdPrimitiveI32FieldReader),
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct GenericCatFieldReader(std::vec::Vec<CatFieldToRead>);

// impl postgresql_crud::GeneratePostgresqlQueryPartFieldToRead for GenericCatFieldReader {
//     fn generate_postgresql_query_part_field_to_read(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
//         let mut acc = std::string::String::default();
//         for element in &self.0 {
//             match element {
//                 CatFieldToRead::StdPrimitiveI32(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('generic',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_primitive_i32",
//                             &format!("{column_name_and_maybe_field_getter}->'std_primitive_i32'"),
//                             &format!("{column_name_and_maybe_field_getter_for_error_message}.std_primitive_i32"),
//                         )
//                     ));
//                 }
//             }
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         format!("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object')) end")
//     }
// }

// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct GenericCatFieldReader(std::vec::Vec<CatFieldToRead>);

// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdOptionOptionGenericCatFieldReader(std::vec::Vec<CatFieldToRead>);

// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdVecVecGenericWithIdCatFieldReader {
//     field_vec: std::vec::Vec<CatFieldToRead>,
//     pagination: postgresql_crud::Pagination,
// }

// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdOptionOptionStdVecVecGenericWithIdCatFieldReader {
//     field_vec: std::vec::Vec<CatFieldToRead>,
//     pagination: postgresql_crud::Pagination,
// }

// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum CatFieldToRead {
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32(postgresql_crud::JsonStdPrimitiveI32FieldReader),
// }


// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum SomethingFieldToRead {
//     #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
//     StdPrimitiveI8,
//     #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdPrimitiveI8,
//     #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
//     StdVecVecStdPrimitiveI8 { limit: std::primitive::u64, offset: std::primitive::u64 },
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdPrimitiveI8 { limit: std::primitive::u64, offset: std::primitive::u64 },
//     #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdVecVecStdOptionOptionStdPrimitiveI8 { limit: std::primitive::u64, offset: std::primitive::u64 },
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 { limit: std::primitive::u64, offset: std::primitive::u64 },
//     #[serde(rename(serialize = "generic", deserialize = "generic"))]
//     Generic(std::vec::Vec<CatFieldToRead>),
//     #[serde(rename(serialize = "std_option_option_generic", deserialize = "std_option_option_generic"))]
//     StdOptionOptionGeneric(std::vec::Vec<MouseFieldToRead>),
//     #[serde(rename(serialize = "std_vec_vec_generic_with_id", deserialize = "std_vec_vec_generic_with_id"))]
//     StdVecVecGenericWithId { field_vec: std::vec::Vec<DoggieFieldToRead>, limit: std::primitive::u64, offset: std::primitive::u64 },
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_generic_with_id", deserialize = "std_option_option_std_vec_vec_generic_with_id"))]
//     StdOptionOptionStdVecVecGenericWithId { field_vec: std::vec::Vec<DoggieFieldToRead>, limit: std::primitive::u64, offset: std::primitive::u64 },
// }

    // #[serde(rename(serialize = "generic", deserialize = "generic"))]
    // Generic(GenericCatFieldReader),
    // #[serde(rename(serialize = "std_option_option_generic", deserialize = "std_option_option_generic"))]
    // StdOptionOptionGeneric(StdOptionOptionGenericCatFieldReader),
    // #[serde(rename(serialize = "std_vec_vec_generic_with_id", deserialize = "std_vec_vec_generic_with_id"))]
    // StdVecVecGenericWithId(StdVecVecGenericWithIdDoggieFieldReader),
    // #[serde(rename(serialize = "std_option_option_std_vec_vec_generic_with_id", deserialize = "std_option_option_std_vec_vec_generic_with_id"))]
    // StdOptionOptionStdVecVecGenericWithId(StdOptionOptionStdVecVecGenericWithIdDoggieFieldReader)

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
//     // pub id: postgresql_crud::JsonUuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids
//     pub std_primitive_i32: postgresql_crud::JsonStdPrimitiveI32,
// }

#[test]
fn test_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() {
    // let default = postgresql_crud::JsonGeneric(Something{

    // });
    // println!("{default:#?}");
    // let default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element: postgresql_crud::JsonGeneric::<Something> = postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element();
    //     println!("{default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element:#?}");
    // let serialized = serde_json::to_string(&default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element).unwrap();
    //     println!("{serialized:#?}");
    //     let deserialized: postgresql_crud::JsonGeneric::<Something> = serde_json::from_str(&serialized).unwrap();
    //     println!("{deserialized:#?}");

    // let s = Something {
    //     std_primitive_i8: postgresql_crud::JsonStdPrimitiveI8(8),
    //     std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16(16),
    // };

    // #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
    // struct Thing {
    //     pub std_primitive_i8: postgresql_crud::JsonStdPrimitiveI8,
    // }
    // let schema = schemars::schema_for!(Thing);
    // println!("{}", serde_json::to_string_pretty(&schema).unwrap());

    let f = SomethingOptionsToUpdate(vec![
        // SomethingOptionToUpdate::StdPrimitiveI8(postgresql_crud::Value {
        //     value: std::primitive::i8::default()
        // }),
        //here
        SomethingOptionToUpdate::StdVecVecGenericWithId(postgresql_crud::Value {
            value: vec![
                DoggieOptionsToUpdate { id: postgresql_crud::JsonUuid(uuid::uuid!("d6a4aa72-b154-4699-889f-33ef34a8c7f2")), fields: vec![DoggieOptionToUpdate::StdPrimitiveI16(postgresql_crud::Value { value: std::primitive::i16::default() })] },
                // DoggieOptionsToUpdate(postgresql_crud::JsonArrayElementChange::Create(DoggieToCreate {
                //     std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16::default(),
                // })),
                // DoggieOptionsToUpdate(postgresql_crud::JsonArrayElementChange::Delete(postgresql_crud::JsonUuid(uuid::uuid!("d6a4aa72-b154-4699-889f-33ef34a8c7f2")))),

                // StdVecVecGenericWithId(postgresql_crud::Value<std::vec::Vec<DoggieJsonArrayElementChange>>),
                // #[serde(rename(
                //     serialize = "std_option_option_std_vec_vec_generic_with_id",
                //     deserialize = "std_option_option_std_vec_vec_generic_with_id"
                // ))]
                // StdOptionOptionStdVecVecGenericWithId(
                //     postgresql_crud::Value<std::vec::Vec<std::option::Option<DoggieJsonArrayElementChange>>>,
                // ),
            ],
        }),
    ]);
    println!("{f:#?}");
    let serialized = serde_json::to_string(&f).unwrap();
    println!("{serialized:#?}");
}

/////////start block code for trying implement partial
// #[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct UpdateOnePayload {
//     pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdPrimitiveI64,
//     pub sqlx_types_json_t_as_postgresql_json_b_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::SqlxTypesJson<SomethingOptionsToUpdate>>>,
// }
// #[derive(Debug)]
// pub struct UpdateOneParameters {
//     pub payload: UpdateOnePayload,
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
// pub enum TryUpdateOneRouteLogicResponseVariants {
//     Desirable(postgresql_crud::StdPrimitiveI64),
//     CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
//     Postgresql { postgresql: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
//     SerdeJson { serde_json: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
//     CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
//     BindQuery { bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
//     NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
//     RowAndRollback { row: std::string::String, rollback: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
// }
// impl std::convert::From<TryUpdateOneRouteLogicErrorNamed> for TryUpdateOneRouteLogicResponseVariants {
//     fn from(value: TryUpdateOneRouteLogicErrorNamed) -> Self {
//         match value.into_serialize_deserialize_version() {
//             TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
//             TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
//             TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
//             TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
//             TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence } => Self::BindQuery { bind_query, code_occurence },
//             TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence } => Self::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence },
//             TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum TryUpdateOneRouteLogicErrorNamed {
//     CheckBodySize {
//         #[eo_error_occurence]
//         check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Postgresql {
//         #[eo_to_std_string_string]
//         postgresql: sqlx::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     SerdeJson {
//         #[eo_to_std_string_string]
//         serde_json: serde_json::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     CheckCommit {
//         #[eo_error_occurence]
//         check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     BindQuery {
//         #[eo_error_occurence]
//         bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NoPayloadFieldsPrimaryKey {
//         #[eo_to_std_string_string]
//         no_payload_fields_primary_key: postgresql_crud::StdPrimitiveI64,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     RowAndRollback {
//         #[eo_to_std_string_string]
//         row: sqlx::Error,
//         #[eo_to_std_string_string]
//         rollback: sqlx::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// pub async fn try_update_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
//     let (parts, body) = request.into_parts();
//     let headers = parts.headers;
//     let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
//         Ok(value) => value,
//         Err(error_0) => {
//             let error = TryUpdateOneRouteLogicErrorNamed::CheckBodySize { check_body_size: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2577, column: 17 })) };
//             eprintln!("{error}");
//             let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//             *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
//             return response;
//         }
//     };
//     let parameters = UpdateOneParameters {
//         payload: match serde_json::from_slice::<UpdateOnePayload>(&body_bytes) {
//             Ok(value) => {
//                 let value = UpdateOnePayload::from(value);
//                 if let None = &value.sqlx_types_json_t_as_postgresql_json_b_not_null {
//                     let error_0 = value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;
//                     let error = TryUpdateOneRouteLogicErrorNamed::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2059, column: 13 })) };
//                     eprintln!("{error}");
//                     let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//                     *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
//                     return response;
//                 }
//                 value
//             }
//             Err(error_0) => {
//                 let error = TryUpdateOneRouteLogicErrorNamed::SerdeJson { serde_json: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2668, column: 13 })) };
//                 eprintln!("{error}");
//                 let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//                 *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
//                 return response;
//             }
//         },
//     };
//     println!("{:#?}", parameters);
//     let query_string = {
//         let mut increment: std::primitive::u64 = 0;
//         let mut query = std::string::String::from("update jsongeneric set ");
//         if let Some(value) = &parameters.payload.sqlx_types_json_t_as_postgresql_json_b_not_null {
//             // Json(
//             //     SomethingOptionsToUpdate(
//             //         [
//             //             StdPrimitiveI8(
//             //                 Value {
//             //                     value: 10,
//             //                 },
//             //             ),
//             //             Generic(
//             //                 Value {
//             //                     value: DoggieOptionsToUpdate(
//             //                         [
//             //                             StdStringString(
//             //                                 Value {
//             //                                     value: "fkkkkf",
//             //                                 },
//             //                             ),
//             //                         ],
//             //                     ),
//             //                 },
//             //             ),
//             //         ],
//             //     ),
//             // )

//             // {
//             //     "generic": {
//             //       "std_string_string": "sdsd"
//             //     },
//             //     "std_primitive_i8": 0
//             // }

//             query.push_str("sqlx_types_json_t_as_postgresql_json_b_not_null = ");
//             match postgresql_crud::GeneratePostgresqlQueryPartToUpdate::try_generate_bind_increments(
//                 &value.value.0 .0,
//                 "sqlx_types_json_t_as_postgresql_json_b_not_null",
//                 "sqlx_types_json_t_as_postgresql_json_b_not_null", //sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_generic'->0->'generic'
//                 "",                                                //{std_vec_vec_generic,0}
//                 &mut increment,
//                 postgresql_crud::ArrayObjectElementOrSimple::Simple,
//             ) {
//                 Ok(value) => {
//                     query.push_str(&value);
//                 }
//                 Err(error) => {
//                     println!("here {error:#?}");
//                     todo!()
//                 }
//             }
//             query.push_str(" ");

//             // UPDATE jsongeneric
//             // SET sqlx_types_json_t_as_postgresql_json_b_not_null =
//             //     jsonb_set(
//             //         jsonb_set(
//             //             jsonb_set(
//             //                 sqlx_types_json_t_as_postgresql_json_b_not_null,
//             //                 '{std_primitive_i8}',
//             //                 '44'
//             //             ),
//             //             '{generic,std_string_string}',
//             //             '"something"'
//             //         ),
//             //         '{std_option_option_generic}',
//             //         -- jsonb_build_object('std_option_option_generic', 'yeees')
//             //         'null'
//             //     )
//             // WHERE std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14
//             // RETURNING std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;
//         }
//         let _ = query.pop();
//         match postgresql_crud::BindQuery::try_increment(&parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, &mut increment) {
//             Ok(_) => {
//                 query.push_str(&format!(" where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = ${increment}"));
//             }
//             Err(error_0) => {
//                 let error = TryUpdateOneRouteLogicErrorNamed::BindQuery { bind_query: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 4822, column: 25 })) };
//                 eprintln!("{error}");
//                 let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//                 *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
//                 return response;
//             }
//         }
//         query.push_str(&format!(" returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"));
//         query
//     };
//     println!("{}", query_string);
//     let binded_query = {
//         let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
//         if let Some(value) = parameters.payload.sqlx_types_json_t_as_postgresql_json_b_not_null {
//             query = postgresql_crud::GeneratePostgresqlQueryPartToUpdate::bind_value_to_query(value.value.0 .0, query);
//             // for element in value.value.0.0.0 {
//             //     match element {
//             //         SomethingOptionToUpdate::StdPrimitiveI8(value) => {
//             //             query = query.bind(sqlx::types::Json(value.value));//sqlx::types::Json
//             //         }
//             //         SomethingOptionToUpdate::Generic(value) => {
//             //             for element in value.value.0 {
//             //                 match element {
//             //                     DoggieOptionToUpdate::StdStringString(value) => {
//             //                         query = query.bind(sqlx::types::Json(value.value));
//             //                     }
//             //                 }
//             //             }
//             //         }
//             //     }
//             // }

//             // query = query.bind(sqlx::types::Json(value.value));
//             // query = postgresql_crud::BindQuery::bind_value_to_query(value.value.0.0, query);
//             // query = postgresql_crud::BindQuery::bind_value_to_query(sqlx::types::Json(postgresql_crud::JsonStdStringString(std::string::String::new("cat"))), query);
//             // query = postgresql_crud::BindQuery::bind_value_to_query(sqlx::types::Json(postgresql_crud::JsonStdPrimitiveI8(42)), query);
//         }
//         query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, query);
//         query
//     };
//     let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
//         Ok(value) => value,
//         Err(error_0) => {
//             let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2609, column: 17 })) };
//             eprintln!("{error}");
//             let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//             *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
//             return response;
//         }
//     };
//     let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
//         Ok(value) => value,
//         Err(error_0) => {
//             let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2609, column: 17 })) };
//             eprintln!("{error}");
//             let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//             *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
//             return response;
//         }
//     };
//     let value = {
//         let mut executor = match sqlx::Acquire::begin(executor).await {
//             Ok(value) => value,
//             Err(error_0) => {
//                 let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2289, column: 17 })) };
//                 eprintln!("{error}");
//                 let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//                 *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
//                 return response;
//             }
//         };
//         let value = {
//             match binded_query.fetch_one(executor.as_mut()).await {
//                 Ok(value) => match sqlx::Row::try_get::<std::primitive::i64, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
//                     Ok(value) => postgresql_crud::StdPrimitiveI64(value),
//                     Err(error_0) => match executor.rollback().await {
//                         Ok(_) => {
//                             let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 3058, column: 17 })) };
//                             eprintln!("{error}");
//                             let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//                             *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
//                             return response;
//                         }
//                         Err(error_1) => {
//                             let error = TryUpdateOneRouteLogicErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 3061, column: 17 })) };
//                             eprintln!("{error}");
//                             let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//                             *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
//                             return response;
//                         }
//                     },
//                 },
//                 Err(error_0) => match executor.rollback().await {
//                     Ok(_) => {
//                         let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 3068, column: 13 })) };
//                         eprintln!("{error}");
//                         let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//                         *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
//                         return response;
//                     }
//                     Err(error_1) => {
//                         let error = TryUpdateOneRouteLogicErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 3071, column: 13 })) };
//                         eprintln!("{error}");
//                         let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//                         *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
//                         return response;
//                     }
//                 },
//             }
//         };
//         if let Err(error_0) = executor.commit().await {
//             let error = TryUpdateOneRouteLogicErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line: 2307, column: 17 })) };
//             eprintln!("{error}");
//             let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
//             *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
//             return response;
//         }
//         value
//     };
//     let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::Desirable(value)));
//     *response.status_mut() = axum::http::StatusCode::OK;
//     return response;
// }
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
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for UpdateOnePayload {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self { std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }
//     }
// }
// pub async fn update_one_payload_example_route_logic() -> axum::response::Response {
//     let mut response = axum::response::IntoResponse::into_response(axum::Json(<UpdateOnePayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()));
//     *response.status_mut() = axum::http::StatusCode::OK;
//     return response;
// }
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
// impl std::fmt::Display for Something {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum SomethingFieldToRead {
//     #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
//     StdPrimitiveI8,
//     #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdPrimitiveI8,
//     #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
//     StdVecVecStdPrimitiveI8 { limit: std::primitive::u64, offset: std::primitive::u64 },
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdPrimitiveI8 { limit: std::primitive::u64, offset: std::primitive::u64 },
//     #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdVecVecStdOptionOptionStdPrimitiveI8 { limit: std::primitive::u64, offset: std::primitive::u64 },
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 { limit: std::primitive::u64, offset: std::primitive::u64 },
//     #[serde(rename(serialize = "generic", deserialize = "generic"))]
//     Generic(std::vec::Vec<CatFieldToRead>),
//     #[serde(rename(serialize = "std_option_option_generic", deserialize = "std_option_option_generic"))]
//     StdOptionOptionGeneric(std::vec::Vec<MouseFieldToRead>),
//     #[serde(rename(serialize = "std_vec_vec_generic_with_id", deserialize = "std_vec_vec_generic_with_id"))]
//     StdVecVecGenericWithId { field_vec: std::vec::Vec<DoggieFieldToRead>, limit: std::primitive::u64, offset: std::primitive::u64 },
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_generic_with_id", deserialize = "std_option_option_std_vec_vec_generic_with_id"))]
//     StdOptionOptionStdVecVecGenericWithId { field_vec: std::vec::Vec<DoggieFieldToRead>, limit: std::primitive::u64, offset: std::primitive::u64 },
// }
// impl error_occurence_lib::ToStdStringString for SomethingFieldToRead {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed {
//     FieldsFilterIsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUniqueFieldFilter {
//         #[eo_to_std_string_string_serialize_deserialize]
//         field: SomethingFieldToRead,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     GeneratePostgresqlQueryPart {
//         #[eo_error_occurence]
//         error: SomethingGeneratePostgresqlQueryPartToReadErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl error_occurence_lib::ToStdStringString for SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum SomethingGeneratePostgresqlQueryPartToReadErrorNamed {
//     StdVecVecStdPrimitiveI8OffsetPlusLimitIsIntOverflow {
//         #[eo_to_std_string_string_serialize_deserialize]
//         limit: std::primitive::u64,
//         #[eo_to_std_string_string_serialize_deserialize]
//         offset: std::primitive::u64,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdOptionOptionStdVecVecStdPrimitiveI8OffsetPlusLimitIsIntOverflow {
//         #[eo_to_std_string_string_serialize_deserialize]
//         limit: std::primitive::u64,
//         #[eo_to_std_string_string_serialize_deserialize]
//         offset: std::primitive::u64,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdVecVecStdOptionOptionStdPrimitiveI8OffsetPlusLimitIsIntOverflow {
//         #[eo_to_std_string_string_serialize_deserialize]
//         limit: std::primitive::u64,
//         #[eo_to_std_string_string_serialize_deserialize]
//         offset: std::primitive::u64,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OffsetPlusLimitIsIntOverflow {
//         #[eo_to_std_string_string_serialize_deserialize]
//         limit: std::primitive::u64,
//         #[eo_to_std_string_string_serialize_deserialize]
//         offset: std::primitive::u64,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdVecVecGenericWithIdOffsetPlusLimitIsIntOverflow {
//         #[eo_to_std_string_string_serialize_deserialize]
//         limit: std::primitive::u64,
//         #[eo_to_std_string_string_serialize_deserialize]
//         offset: std::primitive::u64,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdOptionOptionStdVecVecGenericWithIdOffsetPlusLimitIsIntOverflow {
//         #[eo_to_std_string_string_serialize_deserialize]
//         limit: std::primitive::u64,
//         #[eo_to_std_string_string_serialize_deserialize]
//         offset: std::primitive::u64,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     CatGeneratePostgresqlQueryPartToReadFromSelfVec {
//         #[eo_error_occurence]
//         field: CatGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     MouseGeneratePostgresqlQueryPartToReadFromSelfVec {
//         #[eo_error_occurence]
//         field: MouseGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     DoggieGeneratePostgresqlQueryPartToReadFromSelfVec {
//         #[eo_error_occurence]
//         field: DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl postgresql_crud::GeneratePostgresqlQueryPartToRead<SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed, SomethingGeneratePostgresqlQueryPartToReadErrorNamed> for SomethingFieldToRead {
//     fn generate_postgresql_query_part_to_read_from_self_vec(value: &std::vec::Vec<Self>, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_optional: std::primitive::bool) -> Result<std::string::String, SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed> {
//         if value.is_empty() {
//             return Err(SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
//         }
//         let mut unique = vec![];
//         for element in value {
//             if unique.contains(&element) {
//                 return Err(SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed::NotUniqueFieldFilter { field: element.clone(), code_occurence: error_occurence_lib::code_occurence!() });
//             } else {
//                 unique.push(&element);
//             }
//         }
//         let mut acc = std::string::String::default();
//         for element in value {
//             match element.generate_postgresql_query_part_to_read(column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message) {
//                 Ok(value) => {
//                     acc.push_str(&format!("{value}||"));
//                 }
//                 Err(error) => {
//                     return Err(SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed::GeneratePostgresqlQueryPart { error, code_occurence: error_occurence_lib::code_occurence!() });
//                 }
//             }
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         let is_optional_query_part = match is_optional {
//             true => format!("when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null)"),
//             false => std::string::String::default(),
//         };
//         Ok({
//             let space_and_not_null = if is_optional { " and not null" } else { "" };
//             format!("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}){is_optional_query_part} else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object{space_and_not_null}')) end")
//         })
//     }
//     fn generate_postgresql_query_part_to_read(&self, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> Result<std::string::String, SomethingGeneratePostgresqlQueryPartToReadErrorNamed> {
//         match self {
//             Self::StdPrimitiveI8 => Ok(format!(
//                 "jsonb_build_object('std_primitive_i8',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i8') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i8') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_primitive_i8 is not number')) end )"
//             )),
//             Self::StdOptionOptionStdPrimitiveI8 => Ok(format!("jsonb_build_object('std_option_option_std_primitive_i8',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_primitive_i8') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_option_option_std_primitive_i8') when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_primitive_i8') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_primitive_i8 is not number and not null')) end )")),
//             Self::StdVecVecStdPrimitiveI8 { limit, offset } => {
//                 let start = offset;
//                 let end = match offset.checked_add(*limit) {
//                     Some(value) => value,
//                     None => {
//                         return Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed::StdVecVecStdPrimitiveI8OffsetPlusLimitIsIntOverflow { limit: *limit, offset: *offset, code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 };
//                 Ok(format!("jsonb_build_object('std_vec_vec_std_primitive_i8',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_vec_vec_std_primitive_i8') = 'array' then jsonb_build_object('Ok',(select jsonb_agg(case when jsonb_typeof(value) = 'number' then jsonb_build_object('Ok',value) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_std_primitive_i8[array element] is not number')) end) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_vec_vec_std_primitive_i8')) with ordinality where ordinality between {start} and {end})) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_std_primitive_i8 is not array')) end)"))
//             }
//             Self::StdOptionOptionStdVecVecStdPrimitiveI8 { limit, offset } => {
//                 let start = offset;
//                 let end = match offset.checked_add(*limit) {
//                     Some(value) => value,
//                     None => {
//                         return Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed::StdOptionOptionStdVecVecStdPrimitiveI8OffsetPlusLimitIsIntOverflow { limit: *limit, offset: *offset, code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 };
//                 Ok(format!("jsonb_build_object('std_option_option_std_vec_vec_std_primitive_i8',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_primitive_i8') = 'array' then jsonb_build_object('Ok',(select jsonb_agg(case when jsonb_typeof(value) = 'number' then jsonb_build_object('Ok',value) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_std_primitive_i8[array element] is not number')) end) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_primitive_i8')) with ordinality where ordinality between {start} and {end})) when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_primitive_i8') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_std_primitive_i8 is not array and not null')) end)"))
//             }
//             Self::StdVecVecStdOptionOptionStdPrimitiveI8 { limit, offset } => {
//                 let start = offset;
//                 let end = match offset.checked_add(*limit) {
//                     Some(value) => value,
//                     None => {
//                         return Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed::StdVecVecStdOptionOptionStdPrimitiveI8OffsetPlusLimitIsIntOverflow { limit: *limit, offset: *offset, code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 };
//                 Ok(format!("jsonb_build_object('std_vec_vec_std_option_option_std_primitive_i8',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_vec_vec_std_option_option_std_primitive_i8') = 'array' then jsonb_build_object('Ok',(select jsonb_agg(case when jsonb_typeof(value) = 'number' then jsonb_build_object('Ok',value) when jsonb_typeof(value) = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_std_option_option_std_primitive_i8[array element] is not number and not null')) end) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_vec_vec_std_option_option_std_primitive_i8')) with ordinality where ordinality between {start} and {end})) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_std_option_option_std_primitive_i8 is not array')) end)"))
//             }
//             Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 { limit, offset } => {
//                 let start = offset;
//                 let end = match offset.checked_add(*limit) {
//                     Some(value) => value,
//                     None => {
//                         return Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OffsetPlusLimitIsIntOverflow { limit: *limit, offset: *offset, code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 };
//                 Ok(format!("jsonb_build_object('std_option_option_std_vec_vec_std_option_option_std_primitive_i8',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_option_option_std_primitive_i8') = 'array' then jsonb_build_object('Ok',(select jsonb_agg(case when jsonb_typeof(value) = 'number' then jsonb_build_object('Ok',value) when jsonb_typeof(value) = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_std_option_option_std_primitive_i8[array element] is not number and not null')) end) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_option_option_std_primitive_i8')) with ordinality where ordinality between {start} and {end}))when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_option_option_std_primitive_i8') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_std_option_option_std_primitive_i8 is not array and not null')) end)"))
//             }
//             Self::Generic(fields_vec) => match postgresql_crud::GeneratePostgresqlQueryPartToRead::generate_postgresql_query_part_to_read_from_self_vec(fields_vec, &format!("{column_name_and_maybe_field_getter}->'generic'"), &format!("{column_name_and_maybe_field_getter_for_error_message}.generic"), false) {
//                 Ok(value) => Ok(format!("jsonb_build_object('generic',{value})")),
//                 Err(error) => {
//                     return Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed::CatGeneratePostgresqlQueryPartToReadFromSelfVec { field: error, code_occurence: error_occurence_lib::code_occurence!() });
//                 }
//             },
//             Self::StdOptionOptionGeneric(fields_vec) => match postgresql_crud::GeneratePostgresqlQueryPartToRead::generate_postgresql_query_part_to_read_from_self_vec(fields_vec, &format!("{column_name_and_maybe_field_getter}->'std_option_option_generic'"), &format!("{column_name_and_maybe_field_getter_for_error_message}.std_option_option_generic"), true) {
//                 Ok(value) => Ok(format!("jsonb_build_object('std_option_option_generic',{value})")),
//                 Err(error) => {
//                     return Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed::MouseGeneratePostgresqlQueryPartToReadFromSelfVec { field: error, code_occurence: error_occurence_lib::code_occurence!() });
//                 }
//             },
//             Self::StdVecVecGenericWithId { field_vec, limit, offset } => match postgresql_crud::GeneratePostgresqlQueryPartToRead::generate_postgresql_query_part_to_read_from_self_vec(field_vec, &format!("value"), &format!("{column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_generic_with_id[array element]"), false) {
//                 Ok(value) => {
//                     let start = offset;
//                     let end = match offset.checked_add(*limit) {
//                         Some(value) => value,
//                         None => {
//                             return Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed::StdVecVecGenericWithIdOffsetPlusLimitIsIntOverflow { limit: *limit, offset: *offset, code_occurence: error_occurence_lib::code_occurence!() });
//                         }
//                     };
//                     Ok(format!("jsonb_build_object('std_vec_vec_generic_with_id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_vec_vec_generic_with_id') = 'array' then jsonb_build_object('Ok',(select jsonb_agg({value}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_vec_vec_generic_with_id')) with ordinality where ordinality between {start} and {end})) else jsonb_build_object(jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}} is not array')) end)"))
//                 }
//                 Err(error) => {
//                     return Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed::DoggieGeneratePostgresqlQueryPartToReadFromSelfVec { field: error, code_occurence: error_occurence_lib::code_occurence!() });
//                 }
//             },
//             Self::StdOptionOptionStdVecVecGenericWithId { field_vec, limit, offset } => match postgresql_crud::GeneratePostgresqlQueryPartToRead::generate_postgresql_query_part_to_read_from_self_vec(field_vec, &format!("value"), &format!("{column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_generic_with_id[array element]"), false) {
//                 Ok(value) => {
//                     let start = offset;
//                     let end = match offset.checked_add(*limit) {
//                         Some(value) => value,
//                         None => {
//                             return Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed::StdOptionOptionStdVecVecGenericWithIdOffsetPlusLimitIsIntOverflow { limit: *limit, offset: *offset, code_occurence: error_occurence_lib::code_occurence!() });
//                         }
//                     };
//                     Ok(format!("jsonb_build_object('std_option_option_std_vec_vec_generic_with_id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic_with_id') = 'array' then jsonb_build_object('Ok',(select jsonb_agg({value}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic_with_id')) with ordinality where ordinality between {start} and {end}))when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic_with_id') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object(jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}} is not array and not null')) end)"))
//                 }
//                 Err(error) => {
//                     return Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed::DoggieGeneratePostgresqlQueryPartToReadFromSelfVec { field: error, code_occurence: error_occurence_lib::code_occurence!() });
//                 }
//             },
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
// pub struct SomethingOptionsToRead {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::JsonStdPrimitiveI8OptionsToRead>>,//here
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::JsonStdOptionOptionStdPrimitiveI8OptionsToRead>>,
    
    
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::JsonStdVecVecStdPrimitiveI8OptionsToRead>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_option_option_std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_option_option_std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead>>,

//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<std::vec::Vec<std::primitive::i8>>>,
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // std_option_option_std_vec_vec_std_primitive_i8: std::option::Option<postgresql_crud::Value<std::option::Option<std::vec::Vec<std::primitive::i8>>>>,
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<std::vec::Vec<std::option::Option<std::primitive::i8>>>>,
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // std_option_option_std_vec_vec_std_option_option_std_primitive_i8: std::option::Option<postgresql_crud::Value<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>,

//     #[serde(skip_serializing_if = "Option::is_none")]
//     generic: std::option::Option<postgresql_crud::Value<CatOptionsToRead>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_option_option_generic: std::option::Option<postgresql_crud::Value<MouseOptionsToRead>>,//std::option::Option<postgresql_crud::Value<std::option::Option<MouseOptionsToRead>>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_vec_vec_generic_with_id: std::option::Option<postgresql_crud::Value<std::vec::Vec<DoggieOptionsToRead>>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_option_option_std_vec_vec_generic_with_id: std::option::Option<postgresql_crud::Value<std::option::Option<std::vec::Vec<DoggieOptionsToRead>>>>,//maybe here remove option too
// }
// // impl std::convert::From<Something> for SomethingOptionsToRead {
// //     fn from(value: Something) -> Self {
// //         Self {
// //             std_primitive_i8: Some(postgresql_crud::Value {
// //                 value: postgresql_crud::JsonStdPrimitiveI8OptionsToRead::from(value.std_primitive_i8)
// //             }),
// //             std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
// //                 value: postgresql_crud::JsonStdOptionOptionStdPrimitiveI8OptionsToRead::from(value.std_option_option_std_primitive_i8)
// //             }),
// //             std_vec_vec_std_primitive_i8: Some(postgresql_crud::Value {
// //                 value: postgresql_crud::JsonStdVecVecStdPrimitiveI8OptionsToRead::from(value.std_vec_vec_std_primitive_i8)
// //             }),
// //             std_option_option_std_vec_vec_std_primitive_i8: Some(postgresql_crud::Value {
// //                 value: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead::from(value.std_option_option_std_vec_vec_std_primitive_i8),
// //             }),
// //             std_vec_vec_std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
// //                 value: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead::from(value.std_vec_vec_std_option_option_std_primitive_i8),
// //             }),
// //             std_option_option_std_vec_vec_std_option_option_std_primitive_i8: Some(postgresql_crud::Value {
// //                 value: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead::from(value.std_option_option_std_vec_vec_std_option_option_std_primitive_i8),
// //             }),
// //             generic: Some(postgresql_crud::Value {
// //                 value: 
// //                 CatOptionsToRead::from(value.generic.0)
// //                 // postgresql_crud::JsonGenericOptionsToRead<CatOptionsToRead>::from(value.generic)

// //                 // CatOptionsToRead::from(value.generic)
// //                 // CatOptionsToRead::from(value.generic.0)
// //             }),
// //             std_option_option_generic: Some(postgresql_crud::Value {
// //                 value: match value.std_option_option_generic.0 {
// //                     Some(value) => Some(MouseOptionsToRead::from(value)),
// //                     None => None,
// //                 }
// //                 // std::option::Option::<MouseOptionsToRead>::from(value.std_option_option_generic),
                

// //                 // postgresql_crud::JsonStdOptionOptionGeneric<Mouse>

// //                 // std::option::Option<MouseOptionsToRead>>
// //             }),
// //             std_vec_vec_generic_with_id: Some(postgresql_crud::Value { value: value.std_vec_vec_generic_with_id.0.into_iter().map(|element| DoggieOptionsToRead::from(element)).collect::<std::vec::Vec<DoggieOptionsToRead>>() }),
// //             std_option_option_std_vec_vec_generic_with_id: Some(postgresql_crud::Value {
// //                 value: match value.std_option_option_std_vec_vec_generic_with_id.0 {
// //                     Some(value) => Some(value.into_iter().map(|element| DoggieOptionsToRead::from(element)).collect::<std::vec::Vec<DoggieOptionsToRead>>()),
// //                     None => None,
// //                 },
// //             }),
// //         }
// //     }
// // }
// impl<'de> serde::Deserialize<'de> for SomethingOptionsToRead {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[allow(non_camel_case_types)]
//         #[doc(hidden)]
//         enum __Field {
//             __field0,
//             __field1,
//             __field2,
//             __field3,
//             __field4,
//             __field5,
//             __field6,
//             __field7,
//             __field8,
//             __field9,
//             __ignore,
//         }
//         #[doc(hidden)]
//         struct __FieldVisitor;
//         impl serde::de::Visitor<'_> for __FieldVisitor {
//             type Value = __Field;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "field identifier")
//             }
//             fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     0u64 => serde::__private::Ok(__Field::__field0),
//                     1u64 => serde::__private::Ok(__Field::__field1),
//                     2u64 => serde::__private::Ok(__Field::__field2),
//                     3u64 => serde::__private::Ok(__Field::__field3),
//                     4u64 => serde::__private::Ok(__Field::__field4),
//                     5u64 => serde::__private::Ok(__Field::__field5),
//                     6u64 => serde::__private::Ok(__Field::__field6),
//                     7u64 => serde::__private::Ok(__Field::__field7),
//                     8u64 => serde::__private::Ok(__Field::__field8),
//                     9u64 => serde::__private::Ok(__Field::__field9),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//             fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     "std_primitive_i8" => serde::__private::Ok(__Field::__field0),
//                     "std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field1),
//                     "std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field2),
//                     "std_option_option_std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field3),
//                     "std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field4),
//                     "std_option_option_std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field5),
//                     "generic" => serde::__private::Ok(__Field::__field6),
//                     "std_option_option_generic" => serde::__private::Ok(__Field::__field7),
//                     "std_vec_vec_generic_with_id" => serde::__private::Ok(__Field::__field8),
//                     "std_option_option_std_vec_vec_generic_with_id" => serde::__private::Ok(__Field::__field9),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//             fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     b"std_primitive_i8" => serde::__private::Ok(__Field::__field0),
//                     b"std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field1),
//                     b"std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field2),
//                     b"std_option_option_std_vec_vec_std_primitive_i8" => serde::__private::Ok(__Field::__field3),
//                     b"std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field4),
//                     b"std_option_option_std_vec_vec_std_option_option_std_primitive_i8" => serde::__private::Ok(__Field::__field5),
//                     b"generic" => serde::__private::Ok(__Field::__field6),
//                     b"std_option_option_generic" => serde::__private::Ok(__Field::__field7),
//                     b"std_vec_vec_generic_with_id" => serde::__private::Ok(__Field::__field8),
//                     b"std_option_option_std_vec_vec_generic_with_id" => serde::__private::Ok(__Field::__field9),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//         }
//         impl<'de> serde::Deserialize<'de> for __Field {
//             #[inline]
//             fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//             where
//                 __D: serde::Deserializer<'de>,
//             {
//                 serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
//             }
//         }
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<SomethingOptionsToRead>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = SomethingOptionsToRead;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "struct SomethingOptionsToRead")
//             }
//             #[inline]
//             fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<std::primitive::i8, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SomethingOptionsToRead with 10 elements"));
//                     }
//                 };
//                 let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<std::option::Option<std::primitive::i8>, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct SomethingOptionsToRead with 10 elements"));
//                     }
//                 };
//                 let __field2 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::primitive::i8, std::string::String>>, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(2usize, &"struct SomethingOptionsToRead with 10 elements"));
//                     }
//                 };
//                 let __field3 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i8, std::string::String>>>, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(3usize, &"struct SomethingOptionsToRead with 10 elements"));
//                     }
//                 };
//                 let __field4 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>, std::string::String>>, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(4usize, &"struct SomethingOptionsToRead with 10 elements"));
//                     }
//                 };
//                 let __field5 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>, std::string::String>>>, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(5usize, &"struct SomethingOptionsToRead with 10 elements"));
//                     }
//                 };
//                 let __field6 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<CatOptionsToRead, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(6usize, &"struct SomethingOptionsToRead with 10 elements"));
//                     }
//                 };
//                 //here remove option
//                 let __field7 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<MouseOptionsToRead, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(7usize, &"struct SomethingOptionsToRead with 10 elements"));
//                     }
//                 };
//                 let __field8 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<DoggieOptionsToRead, std::string::String>>, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(8usize, &"struct SomethingOptionsToRead with 10 elements"));
//                     }
//                 };
//                 let __field9 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<std::option::Option<std::vec::Vec<std::result::Result<DoggieOptionsToRead, std::string::String>>>, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(9usize, &"struct SomethingOptionsToRead with 10 elements"));
//                     }
//                 };
//                 serde::__private::Ok(SomethingOptionsToRead {
//                     std_primitive_i8: match __field0 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: postgresql_crud::JsonStdPrimitiveI8OptionsToRead(postgresql_crud::JsonStdPrimitiveI8(value)) }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_option_option_std_primitive_i8: match __field1 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: postgresql_crud::JsonStdOptionOptionStdPrimitiveI8OptionsToRead(postgresql_crud::JsonStdOptionOptionStdPrimitiveI8(match value {
//                                 Some(value) => Some(postgresql_crud::JsonStdPrimitiveI8(value)), 
//                                 None => None
//                             })) }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_vec_vec_std_primitive_i8: match __field2 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: postgresql_crud::JsonStdVecVecStdPrimitiveI8OptionsToRead(postgresql_crud::JsonStdVecVecStdPrimitiveI8({
//                                     let mut acc = vec![];
//                                     for element in value {
//                                         match element {
//                                             Ok(value) => {
//                                                 acc.push(postgresql_crud::JsonStdPrimitiveI8(value));
//                                             }
//                                             Err(error) => {
//                                                 return Err(serde::de::Error::custom(error));
//                                             }
//                                         }
//                                     }
//                                     acc
//                                 })),
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_option_option_std_vec_vec_std_primitive_i8: match __field3 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead(postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8(match value {
//                                     Some(value) => {
//                                         let mut acc = vec![];
//                                         for element in value {
//                                             match element {
//                                                 Ok(value) => {
//                                                     acc.push(postgresql_crud::JsonStdPrimitiveI8(value));
//                                                 }
//                                                 Err(error) => {
//                                                     return Err(serde::de::Error::custom(error));
//                                                 }
//                                             }
//                                         }
//                                         Some(acc)
//                                     }
//                                     None => None,
//                                 })),
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_vec_vec_std_option_option_std_primitive_i8: match __field4 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead(postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8({
//                                     let mut acc = vec![];
//                                     for element in value {
//                                         match element {
//                                             Ok(value) => {
//                                                 acc.push(match value {
//                                                     Some(value) => Some(postgresql_crud::JsonStdPrimitiveI8(value)),
//                                                     None => None,
//                                                 });
//                                             }
//                                             Err(error) => {
//                                                 return Err(serde::de::Error::custom(error));
//                                             }
//                                         }
//                                     }
//                                     acc
//                                 })),
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_option_option_std_vec_vec_std_option_option_std_primitive_i8: match __field5 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead(postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(match value {
//                                     Some(value) => {
//                                         let mut acc = vec![];
//                                         for element in value {
//                                             match element {
//                                                 Ok(value) => {
//                                                     acc.push(match value {
//                                                         Some(value) => Some(postgresql_crud::JsonStdPrimitiveI8(value)),
//                                                         None => None
//                                                     });
//                                                 }
//                                                 Err(error) => {
//                                                     return Err(serde::de::Error::custom(error));
//                                                 }
//                                             }
//                                         }
//                                         Some(acc)
//                                     }
//                                     None => None,
//                                 })),
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     generic: match __field6 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_option_option_generic: match __field7 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_vec_vec_generic_with_id: match __field8 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: {
//                                     let mut acc = vec![];
//                                     for element in value {
//                                         match element {
//                                             Ok(value) => {
//                                                 acc.push(value);
//                                             }
//                                             Err(error) => {
//                                                 return Err(serde::de::Error::custom(error));
//                                             }
//                                         }
//                                     }
//                                     acc
//                                 },
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_option_option_std_vec_vec_generic_with_id: match __field9 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: match value {
//                                     Some(value) => {
//                                         let mut acc = vec![];
//                                         for element in value {
//                                             match element {
//                                                 Ok(value) => {
//                                                     acc.push(value);
//                                                 }
//                                                 Err(error) => {
//                                                     return Err(serde::de::Error::custom(error));
//                                                 }
//                                             }
//                                         }
//                                         Some(acc)
//                                     }
//                                     None => None,
//                                 },
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                 })
//             }
//             #[inline]
//             fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::MapAccess<'de>,
//             {
//                 let mut __field0: serde::__private::Option<std::option::Option<std::result::Result<std::primitive::i8, std::string::String>>> = serde::__private::None;
//                 let mut __field1: serde::__private::Option<std::option::Option<std::result::Result<std::option::Option<std::primitive::i8>, std::string::String>>> = serde::__private::None;
//                 let mut __field2: serde::__private::Option<std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::primitive::i8, std::string::String>>, std::string::String>>> = serde::__private::None;
//                 let mut __field3: serde::__private::Option<std::option::Option<std::result::Result<std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i8, std::string::String>>>, std::string::String>>> = serde::__private::None;
//                 let mut __field4: serde::__private::Option<std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>, std::string::String>>, std::string::String>>> = serde::__private::None;
//                 let mut __field5: serde::__private::Option<std::option::Option<std::result::Result<std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>, std::string::String>>>, std::string::String>>> = serde::__private::None;
//                 let mut __field6: serde::__private::Option<std::option::Option<std::result::Result<CatOptionsToRead, std::string::String>>> = serde::__private::None;
//                 //here remove option
//                 let mut __field7: serde::__private::Option<std::option::Option<std::result::Result<MouseOptionsToRead, std::string::String>>> = serde::__private::None;
//                 let mut __field8: serde::__private::Option<std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<DoggieOptionsToRead, std::string::String>>, std::string::String>>> = serde::__private::None;
//                 let mut __field9: serde::__private::Option<std::option::Option<std::result::Result<std::option::Option<std::vec::Vec<std::result::Result<DoggieOptionsToRead, std::string::String>>>, std::string::String>>> = serde::__private::None;
//                 while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
//                     match __key {
//                         __Field::__field0 => {
//                             if serde::__private::Option::is_some(&__field0) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i8"));
//                             }
//                             __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<std::primitive::i8, std::string::String>>>(&mut __map)?);
//                         }
//                         __Field::__field1 => {
//                             if serde::__private::Option::is_some(&__field1) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_primitive_i8"));
//                             }
//                             __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<std::option::Option<std::primitive::i8>, std::string::String>>>(&mut __map)?);
//                         }
//                         __Field::__field2 => {
//                             if serde::__private::Option::is_some(&__field2) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_vec_vec_std_primitive_i8"));
//                             }
//                             __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::primitive::i8, std::string::String>>, std::string::String>>>(&mut __map)?);
//                         }
//                         __Field::__field3 => {
//                             if serde::__private::Option::is_some(&__field3) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_vec_vec_std_primitive_i8"));
//                             }
//                             __field3 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<std::option::Option<std::vec::Vec<std::result::Result<std::primitive::i8, std::string::String>>>, std::string::String>>>(&mut __map)?);
//                         }
//                         __Field::__field4 => {
//                             if serde::__private::Option::is_some(&__field4) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_vec_vec_std_option_option_std_primitive_i8"));
//                             }
//                             __field4 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>, std::string::String>>, std::string::String>>>(&mut __map)?);
//                         }
//                         __Field::__field5 => {
//                             if serde::__private::Option::is_some(&__field5) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8"));
//                             }
//                             __field5 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<std::primitive::i8>, std::string::String>>>, std::string::String>>>(&mut __map)?);
//                         }
//                         __Field::__field6 => {
//                             if serde::__private::Option::is_some(&__field6) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("generic"));
//                             }
//                             __field6 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<CatOptionsToRead, std::string::String>>>(&mut __map)?);
//                         }
//                         __Field::__field7 => {
//                             if serde::__private::Option::is_some(&__field7) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_generic"));
//                             }
//                             //here remove option
//                             __field7 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<MouseOptionsToRead, std::string::String>>>(&mut __map)?);
//                         }
//                         __Field::__field8 => {
//                             if serde::__private::Option::is_some(&__field8) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_vec_vec_generic_with_id"));
//                             }
//                             __field8 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<DoggieOptionsToRead, std::string::String>>, std::string::String>>>(&mut __map)?);
//                         }
//                         __Field::__field9 => {
//                             if serde::__private::Option::is_some(&__field9) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_option_option_std_vec_vec_generic_with_id"));
//                             }
//                             __field9 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<std::option::Option<std::vec::Vec<std::result::Result<DoggieOptionsToRead, std::string::String>>>, std::string::String>>>(&mut __map)?);
//                         }
//                         _ => {
//                             let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
//                         }
//                     }
//                 }
//                 let __field0 = match __field0 {
//                     serde::__private::Some(__field0) => __field0,
//                     serde::__private::None => serde::__private::de::missing_field("std_primitive_i8")?,
//                 };
//                 let __field1 = match __field1 {
//                     serde::__private::Some(__field1) => __field1,
//                     serde::__private::None => serde::__private::de::missing_field("std_option_option_std_primitive_i8")?,
//                 };
//                 let __field2 = match __field2 {
//                     serde::__private::Some(__field2) => __field2,
//                     serde::__private::None => serde::__private::de::missing_field("std_vec_vec_std_primitive_i8")?,
//                 };
//                 let __field3 = match __field3 {
//                     serde::__private::Some(__field3) => __field3,
//                     serde::__private::None => serde::__private::de::missing_field("std_option_option_std_vec_vec_std_primitive_i8")?,
//                 };
//                 let __field4 = match __field4 {
//                     serde::__private::Some(__field4) => __field4,
//                     serde::__private::None => serde::__private::de::missing_field("std_vec_vec_std_option_option_std_primitive_i8")?,
//                 };
//                 let __field5 = match __field5 {
//                     serde::__private::Some(__field5) => __field5,
//                     serde::__private::None => serde::__private::de::missing_field("std_option_option_std_vec_vec_std_option_option_std_primitive_i8")?,
//                 };
//                 let __field6 = match __field6 {
//                     serde::__private::Some(__field6) => __field6,
//                     serde::__private::None => serde::__private::de::missing_field("generic")?,
//                 };
//                 let __field7 = match __field7 {
//                     serde::__private::Some(__field7) => __field7,
//                     serde::__private::None => serde::__private::de::missing_field("std_option_option_generic")?,
//                 };
//                 let __field8 = match __field8 {
//                     serde::__private::Some(__field8) => __field8,
//                     serde::__private::None => serde::__private::de::missing_field("std_vec_vec_generic_with_id")?,
//                 };
//                 let __field9 = match __field9 {
//                     serde::__private::Some(__field9) => __field9,
//                     serde::__private::None => serde::__private::de::missing_field("std_option_option_std_vec_vec_generic_with_id")?,
//                 };
//                 serde::__private::Ok(SomethingOptionsToRead {
//                     std_primitive_i8: match __field0 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: postgresql_crud::JsonStdPrimitiveI8OptionsToRead(postgresql_crud::JsonStdPrimitiveI8(value)) }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_option_option_std_primitive_i8: match __field1 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: postgresql_crud::JsonStdOptionOptionStdPrimitiveI8OptionsToRead(postgresql_crud::JsonStdOptionOptionStdPrimitiveI8(match value {
//                                 Some(value) => Some(postgresql_crud::JsonStdPrimitiveI8(value)),
//                                 None => None,
//                             })) }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_vec_vec_std_primitive_i8: match __field2 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: postgresql_crud::JsonStdVecVecStdPrimitiveI8OptionsToRead({
//                                     let mut acc = vec![];
//                                     for element in value {
//                                         match element {
//                                             Ok(value) => {
//                                                 acc.push(postgresql_crud::JsonStdPrimitiveI8(value));
//                                             }
//                                             Err(error) => {
//                                                 return Err(serde::de::Error::custom(error));
//                                             }
//                                         }
//                                     }
//                                     postgresql_crud::JsonStdVecVecStdPrimitiveI8(acc)
//                                 }),
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_option_option_std_vec_vec_std_primitive_i8: match __field3 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8OptionsToRead(postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8(match value {
//                                     Some(value) => {
//                                         let mut acc = vec![];
//                                         for element in value {
//                                             match element {
//                                                 Ok(value) => {
//                                                     acc.push(postgresql_crud::JsonStdPrimitiveI8(value));
//                                                 }
//                                                 Err(error) => {
//                                                     return Err(serde::de::Error::custom(error));
//                                                 }
//                                             }
//                                         }
//                                         Some(acc)
//                                     }
//                                     None => None,
//                                 })),
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_vec_vec_std_option_option_std_primitive_i8: match __field4 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead(postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8({
//                                     let mut acc = vec![];
//                                     for element in value {
//                                         match element {
//                                             Ok(value) => {
//                                                 acc.push(match value {
//                                                     Some(value) => Some(postgresql_crud::JsonStdPrimitiveI8(value)),
//                                                     None => None
//                                                 });
//                                             }
//                                             Err(error) => {
//                                                 return Err(serde::de::Error::custom(error));
//                                             }
//                                         }
//                                     }
//                                     acc
//                                 })),
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_option_option_std_vec_vec_std_option_option_std_primitive_i8: match __field5 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8OptionsToRead(postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(match value {
//                                     Some(value) => {
//                                         let mut acc = vec![];
//                                         for element in value {
//                                             match element {
//                                                 Ok(value) => {
//                                                     acc.push(match value {
//                                                         Some(value) => Some(postgresql_crud::JsonStdPrimitiveI8(value)),
//                                                         None => None
//                                                     });
//                                                 }
//                                                 Err(error) => {
//                                                     return Err(serde::de::Error::custom(error));
//                                                 }
//                                             }
//                                         }
//                                         Some(acc)
//                                     }
//                                     None => None,
//                                 })),
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     generic: match __field6 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_option_option_generic: match __field7 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_vec_vec_generic_with_id: match __field8 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: {
//                                     let mut acc = vec![];
//                                     for element in value {
//                                         match element {
//                                             Ok(value) => {
//                                                 acc.push(value);
//                                             }
//                                             Err(error) => {
//                                                 return Err(serde::de::Error::custom(error));
//                                             }
//                                         }
//                                     }
//                                     acc
//                                 },
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_option_option_std_vec_vec_generic_with_id: match __field9 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value {
//                                 value: match value {
//                                     Some(value) => {
//                                         let mut acc = vec![];
//                                         for element in value {
//                                             match element {
//                                                 Ok(value) => {
//                                                     acc.push(value);
//                                                 }
//                                                 Err(error) => {
//                                                     return Err(serde::de::Error::custom(error));
//                                                 }
//                                             }
//                                         }
//                                         Some(acc)
//                                     }
//                                     None => None,
//                                 },
//                             }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                 })
//             }
//         }
//         #[doc(hidden)]
//         const FIELDS: &'static [&'static str] = &["std_primitive_i8", "std_option_option_std_primitive_i8", "std_vec_vec_std_primitive_i8", "std_option_option_std_vec_vec_std_primitive_i8", "std_vec_vec_std_option_option_std_primitive_i8", "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", "generic", "std_option_option_generic", "std_vec_vec_generic_with_id", "std_option_option_std_vec_vec_generic_with_id"];
//         serde::Deserializer::deserialize_struct(__deserializer, "SomethingOptionsToRead", FIELDS, __Visitor { marker: serde::__private::PhantomData::<SomethingOptionsToRead>, lifetime: serde::__private::PhantomData })
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
// pub struct SomethingReader(pub SomethingOptionsToRead);
// impl<'de> serde::Deserialize<'de> for SomethingReader {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<SomethingReader>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = SomethingReader;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "tuple struct SomethingReader")
//             }
//             #[inline]
//             fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
//             where
//                 __E: serde::Deserializer<'de>,
//             {
//                 let __field0: Result<SomethingOptionsToRead, std::string::String> = <Result<SomethingOptionsToRead, std::string::String> as serde::Deserialize>::deserialize(__e)?;
//                 serde::__private::Ok(SomethingReader(match __field0 {
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
//                 let __field0 = match serde::de::SeqAccess::next_element::<Result<SomethingOptionsToRead, std::string::String>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct SomethingReader with 1 element"));
//                     }
//                 };
//                 serde::__private::Ok(SomethingReader(match __field0 {
//                     Ok(value) => value,
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(error));
//                     }
//                 }))
//             }
//         }
//         serde::Deserializer::deserialize_newtype_struct(__deserializer, "SomethingReader", __Visitor { marker: serde::__private::PhantomData::<SomethingReader>, lifetime: serde::__private::PhantomData })
//     }
// }
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for Something {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self { std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), generic: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_option_option_generic: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_vec_vec_generic_with_id: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_option_option_std_vec_vec_generic_with_id: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum SomethingFieldToUpdate {
//     #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
//     StdPrimitiveI8,
//     #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdPrimitiveI8,
//     #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
//     StdVecVecStdPrimitiveI8,
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdPrimitiveI8,
//     #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdVecVecStdOptionOptionStdPrimitiveI8,
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
//     #[serde(rename(serialize = "generic", deserialize = "generic"))]
//     Generic,
//     #[serde(rename(serialize = "std_option_option_generic", deserialize = "std_option_option_generic"))]
//     StdOptionOptionGeneric,
//     #[serde(rename(serialize = "std_vec_vec_generic_with_id", deserialize = "std_vec_vec_generic_with_id"))]
//     StdVecVecGenericWithId,
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_generic_with_id", deserialize = "std_option_option_std_vec_vec_generic_with_id"))]
//     StdOptionOptionStdVecVecGenericWithId,
// }
// impl error_occurence_lib::ToStdStringString for SomethingFieldToUpdate {
//     fn to_std_string_string(&self) -> std::string::String {
//         match &self {
//             Self::StdPrimitiveI8 => "std_primitive_i8".to_owned(),
//             Self::StdOptionOptionStdPrimitiveI8 => "std_option_option_std_primitive_i8".to_owned(),
//             Self::StdVecVecStdPrimitiveI8 => "std_vec_vec_std_primitive_i8".to_owned(),
//             Self::StdOptionOptionStdVecVecStdPrimitiveI8 => "std_option_option_std_vec_vec_std_primitive_i8".to_owned(),
//             Self::StdVecVecStdOptionOptionStdPrimitiveI8 => "std_vec_vec_std_option_option_std_primitive_i8".to_owned(),
//             Self::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => "std_option_option_std_vec_vec_std_option_option_std_primitive_i8".to_owned(),
//             Self::Generic => "generic".to_owned(),
//             Self::StdOptionOptionGeneric => "std_option_option_generic".to_owned(),
//             Self::StdVecVecGenericWithId => "std_vec_vec_generic_with_id".to_owned(),
//             Self::StdOptionOptionStdVecVecGenericWithId => "std_option_option_std_vec_vec_generic_with_id".to_owned(),
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub enum SomethingOptionToUpdate {
//     #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
//     StdPrimitiveI8(postgresql_crud::Value<std::primitive::i8>),
//     #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdPrimitiveI8(postgresql_crud::Value<std::option::Option<std::primitive::i8>>),
//     #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
//     StdVecVecStdPrimitiveI8(postgresql_crud::Value<std::vec::Vec<std::primitive::i8>>),
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::Value<std::option::Option<std::vec::Vec<std::primitive::i8>>>),
//     #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value<std::vec::Vec<std::option::Option<std::primitive::i8>>>),
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>),
//     #[serde(rename(serialize = "generic", deserialize = "generic"))]
//     Generic(postgresql_crud::Value<CatOptionsToUpdate>),
//     #[serde(rename(serialize = "std_option_option_generic", deserialize = "std_option_option_generic"))]
//     StdOptionOptionGeneric(postgresql_crud::Value<std::option::Option<MouseOptionsToUpdate>>),
//     #[serde(rename(serialize = "std_vec_vec_generic_with_id", deserialize = "std_vec_vec_generic_with_id"))]
//     StdVecVecGenericWithId(postgresql_crud::Value<postgresql_crud::JsonArrayChange<DoggieToCreate, DoggieOptionsToUpdate>>),
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_generic_with_id", deserialize = "std_option_option_std_vec_vec_generic_with_id"))]
//     StdOptionOptionStdVecVecGenericWithId(postgresql_crud::Value<std::option::Option<postgresql_crud::JsonArrayChange<DoggieToCreate, DoggieOptionsToUpdate>>>),
// }
// impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SomethingOptionToUpdate {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<SomethingOptionToUpdate> {
//         vec![
//             SomethingOptionToUpdate::StdPrimitiveI8(postgresql_crud::Value { value: <postgresql_crud::JsonStdPrimitiveI8 as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0 }),
//             SomethingOptionToUpdate::StdOptionOptionStdPrimitiveI8(postgresql_crud::Value {
//                 value: match <postgresql_crud::JsonStdOptionOptionStdPrimitiveI8 as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0 {
//                     Some(value) => Some(value.0),
//                     None => None,
//                 },
//             }),
//             SomethingOptionToUpdate::StdVecVecStdPrimitiveI8(postgresql_crud::Value { value: <postgresql_crud::JsonStdVecVecStdPrimitiveI8 as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0.into_iter().map(|element| element.0).collect() }),
//             SomethingOptionToUpdate::StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::Value {
//                 value: match <postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8 as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0 {
//                     Some(value) => Some(value.into_iter().map(|element| element.0).collect()),
//                     None => None,
//                 },
//             }),
//             SomethingOptionToUpdate::StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value {
//                 value: <postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
//                     .0
//                     .into_iter()
//                     .map(|element| match element {
//                         Some(value) => Some(value.0),
//                         None => None,
//                     })
//                     .collect(),
//             }),
//             SomethingOptionToUpdate::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::Value {
//                 value: match <postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0 {
//                     Some(value) => Some(
//                         value
//                             .into_iter()
//                             .map(|element| match element {
//                                 Some(value) => Some(value.0),
//                                 None => None,
//                             })
//                             .collect(),
//                     ),
//                     None => None,
//                 },
//             }),
//             SomethingOptionToUpdate::Generic(postgresql_crud::Value { value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }),
//             SomethingOptionToUpdate::StdOptionOptionGeneric(postgresql_crud::Value { value: Some(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()) }),
//             SomethingOptionToUpdate::StdVecVecGenericWithId(postgresql_crud::Value { value: postgresql_crud::JsonArrayChange { create: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()], update: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()], delete: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()] } }),
//             SomethingOptionToUpdate::StdOptionOptionStdVecVecGenericWithId(postgresql_crud::Value { value: Some(postgresql_crud::JsonArrayChange { create: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()], update: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()], delete: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()] }) }),
//         ]
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct SomethingOptionsToUpdate(pub std::vec::Vec<SomethingOptionToUpdate>);
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SomethingOptionsToUpdate {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed {
//     FieldsIsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUniqueField {
//         #[eo_to_std_string_string_serialize_deserialize]
//         field: SomethingFieldToUpdate,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     CheckedAdd {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Cat {
//         #[eo_error_occurence]
//         cat: CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     Mouse {
//         #[eo_error_occurence]
//         mouse: MouseOptionsToUpdateTryGenerateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdVecVecGenericWithIdDoggieNotUniqueId {
//         #[eo_to_std_string_string_serialize_deserialize]
//         std_vec_vec_generic_with_id_doggie_not_unique_id: postgresql_crud::JsonUuid,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements {
//         #[eo_error_occurence]
//         std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments: DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements {
//         #[eo_error_occurence]
//         std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments: postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements {
//         #[eo_error_occurence]
//         std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments: postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdOptionOptionStdVecVecGenericWithIdDoggieNotUniqueId {
//         #[eo_to_std_string_string_serialize_deserialize]
//         std_option_option_std_vec_vec_generic_with_id_doggie_not_unique_id: postgresql_crud::JsonUuid,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements {
//         #[eo_error_occurence]
//         std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments: DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements {
//         #[eo_error_occurence]
//         std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments: postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements {
//         #[eo_error_occurence]
//         std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments: postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl postgresql_crud::GeneratePostgresqlQueryPartToUpdate<SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed> for SomethingOptionsToUpdate {
//     fn try_generate_bind_increments(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64, is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple) -> Result<std::string::String, SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed> {
//         if self.0.is_empty() {
//             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::FieldsIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
//         }
//         {
//             let mut acc = vec![];
//             for element in &self.0 {
//                 match element {
//                     SomethingOptionToUpdate::StdPrimitiveI8(_) => {
//                         let value = SomethingFieldToUpdate::StdPrimitiveI8;
//                         if acc.contains(&value) {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                     SomethingOptionToUpdate::StdOptionOptionStdPrimitiveI8(_) => {
//                         let value = SomethingFieldToUpdate::StdOptionOptionStdPrimitiveI8;
//                         if acc.contains(&value) {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                     SomethingOptionToUpdate::StdVecVecStdPrimitiveI8(_) => {
//                         let value = SomethingFieldToUpdate::StdVecVecStdPrimitiveI8;
//                         if acc.contains(&value) {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                     SomethingOptionToUpdate::StdOptionOptionStdVecVecStdPrimitiveI8(_) => {
//                         let value = SomethingFieldToUpdate::StdOptionOptionStdVecVecStdPrimitiveI8;
//                         if acc.contains(&value) {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                     SomethingOptionToUpdate::StdVecVecStdOptionOptionStdPrimitiveI8(_) => {
//                         let value = SomethingFieldToUpdate::StdVecVecStdOptionOptionStdPrimitiveI8;
//                         if acc.contains(&value) {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                     SomethingOptionToUpdate::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(_) => {
//                         let value = SomethingFieldToUpdate::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8;
//                         if acc.contains(&value) {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                     SomethingOptionToUpdate::Generic(_) => {
//                         let value = SomethingFieldToUpdate::Generic;
//                         if acc.contains(&value) {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                     SomethingOptionToUpdate::StdOptionOptionGeneric(_) => {
//                         let value = SomethingFieldToUpdate::StdOptionOptionGeneric;
//                         if acc.contains(&value) {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                     SomethingOptionToUpdate::StdVecVecGenericWithId(_) => {
//                         let value = SomethingFieldToUpdate::StdVecVecGenericWithId;
//                         if acc.contains(&value) {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                     SomethingOptionToUpdate::StdOptionOptionStdVecVecGenericWithId(_) => {
//                         let value = SomethingFieldToUpdate::StdOptionOptionStdVecVecGenericWithId;
//                         if acc.contains(&value) {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                 }
//             }
//         }
//         let mut acc = std::string::String::from(jsonb_set_accumulator);
//         let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
//             true => std::string::String::default(),
//             false => format!("{jsonb_set_path},"),
//         };
//         for element in &self.0 {
//             match &element {
//                 SomethingOptionToUpdate::StdPrimitiveI8(_) => match increment.checked_add(1) {
//                     Some(value) => {
//                         *increment = value;
//                         acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_primitive_i8}}',${increment})");
//                     }
//                     None => {
//                         return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 },
//                 SomethingOptionToUpdate::StdOptionOptionStdPrimitiveI8(_) => match increment.checked_add(1) {
//                     Some(value) => {
//                         *increment = value;
//                         acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_option_option_std_primitive_i8}}',${increment})");
//                     }
//                     None => {
//                         return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 },
//                 SomethingOptionToUpdate::StdVecVecStdPrimitiveI8(_) => match increment.checked_add(1) {
//                     Some(value) => {
//                         *increment = value;
//                         acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_vec_vec_std_primitive_i8}}',${increment})");
//                     }
//                     None => {
//                         return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 },
//                 SomethingOptionToUpdate::StdOptionOptionStdVecVecStdPrimitiveI8(_) => match increment.checked_add(1) {
//                     Some(value) => {
//                         *increment = value;
//                         acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_option_option_std_vec_vec_std_primitive_i8}}',${increment})");
//                     }
//                     None => {
//                         return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 },
//                 SomethingOptionToUpdate::StdVecVecStdOptionOptionStdPrimitiveI8(_) => match increment.checked_add(1) {
//                     Some(value) => {
//                         *increment = value;
//                         acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_vec_vec_std_option_option_std_primitive_i8}}',${increment})");
//                     }
//                     None => {
//                         return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 },
//                 SomethingOptionToUpdate::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(_) => match increment.checked_add(1) {
//                     Some(value) => {
//                         *increment = value;
//                         acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_option_option_std_vec_vec_std_option_option_std_primitive_i8}}',${increment})");
//                     }
//                     None => {
//                         return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 },
//                 SomethingOptionToUpdate::Generic(value) => match value.value.try_generate_bind_increments(&jsonb_set_accumulator, &jsonb_set_target, &jsonb_set_path, increment, is_array_object_element.clone()) {
//                     Ok(value) => {
//                         acc = value;
//                     }
//                     Err(error) => {
//                         return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::Cat { cat: error, code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 },
//                 SomethingOptionToUpdate::StdOptionOptionGeneric(value) => match &value.value {
//                     Some(value) => match value.try_generate_bind_increments(&jsonb_set_accumulator, &jsonb_set_target, &format!("{previous_jsonb_set_path}std_option_option_generic"), increment, is_array_object_element.clone()) {
//                         Ok(value) => {
//                             acc = value;
//                         }
//                         Err(error) => {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::Mouse { mouse: error, code_occurence: error_occurence_lib::code_occurence!() });
//                         }
//                     },
//                     None => match increment.checked_add(1) {
//                         Some(value) => {
//                             *increment = value;
//                             acc = format!("jsonb_set({acc},'{previous_jsonb_set_path}std_option_option_generic',${increment})");
//                         }
//                         None => {
//                             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                         }
//                     },
//                 },
//                 SomethingOptionToUpdate::StdVecVecGenericWithId(value) => {
//                     {
//                         let mut ids: std::vec::Vec<&postgresql_crud::JsonUuid> = vec![];
//                         for element in &value.value.update {
//                             if ids.contains(&&element.id) {
//                                 return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdVecVecGenericWithIdDoggieNotUniqueId { std_vec_vec_generic_with_id_doggie_not_unique_id: element.id, code_occurence: error_occurence_lib::code_occurence!() });
//                             } else {
//                                 ids.push(&element.id);
//                             }
//                         }
//                         for element in &value.value.delete {
//                             if ids.contains(&element) {
//                                 return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdVecVecGenericWithIdDoggieNotUniqueId { std_vec_vec_generic_with_id_doggie_not_unique_id: *element, code_occurence: error_occurence_lib::code_occurence!() });
//                             } else {
//                                 ids.push(&element);
//                             }
//                         }
//                     }
//                     let current_jsonb_set_target = format!("{jsonb_set_target}->'std_vec_vec_generic_with_id'");
//                     let mut update_query_part_acc = std::string::String::default();
//                     for (index, element) in &value.value.update.iter().enumerate().collect::<std::vec::Vec<(usize, &DoggieOptionsToUpdate)>>() {
//                         match postgresql_crud::JsonArrayElementUpdateBindQuery::try_generate_update_bind_increments(*element, &jsonb_set_accumulator, &jsonb_set_target, &jsonb_set_path, increment, is_array_object_element.clone()) {
//                             Ok(value) => {
//                                 if let Some(value) = value {
//                                     update_query_part_acc.push_str(&value);
//                                 }
//                             }
//                             Err(error) => {
//                                 return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements { std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments: error, code_occurence: error_occurence_lib::code_occurence!() });
//                             }
//                         }
//                     }
//                     let delete_query_part_acc = {
//                         if value.value.delete.is_empty() {
//                             std::string::String::default()
//                         } else {
//                             let mut delete_query_part_acc = std::string::String::default();
//                             for (index, element) in &value.value.delete.iter().enumerate().collect::<std::vec::Vec<(usize, &postgresql_crud::JsonUuid)>>() {
//                                 match increment.checked_add(1) {
//                                     Some(value) => {
//                                         *increment = value;
//                                         let maybe_space_and_space = if delete_query_part_acc.is_empty() { "" } else { " and " };
//                                         delete_query_part_acc.push_str(&format!("{maybe_space_and_space}elem->>'id' <> ${increment}"));
//                                     }
//                                     None => {
//                                         return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements { std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments: postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }, code_occurence: error_occurence_lib::code_occurence!() });
//                                     }
//                                 }
//                             }
//                             delete_query_part_acc
//                         }
//                     };
//                     let mut create_query_part_acc = std::string::String::default();
//                     for (index, element) in &value.value.create.iter().enumerate().collect::<std::vec::Vec<(usize, &DoggieToCreate)>>() {
//                         match postgresql_crud::JsonArrayElementCreateBindQuery::try_generate_create_bind_increments(*element, increment) {
//                             Ok(value) => {
//                                 if let Some(value) = value {
//                                     create_query_part_acc.push_str(&format!("{value},"));
//                                 }
//                             }
//                             Err(error) => {
//                                 return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements { std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments: error, code_occurence: error_occurence_lib::code_occurence!() });
//                             }
//                         }
//                     }
//                     let _ = create_query_part_acc.pop();
//                     let maybe_jsonb_agg_case = if update_query_part_acc.is_empty() { std::string::String::from("elem") } else { format!("case {update_query_part_acc} else elem end") };
//                     let maybe_where = if delete_query_part_acc.is_empty() { std::string::String::default() } else { format!(" where {delete_query_part_acc}") };
//                     let maybe_jsonb_build_array = if create_query_part_acc.is_empty() { std::string::String::default() } else { format!(" || jsonb_build_array({create_query_part_acc})") };
//                     acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_vec_vec_generic_with_id}}',(select jsonb_agg({maybe_jsonb_agg_case}) from jsonb_array_elements({current_jsonb_set_target}) as elem {maybe_where}){maybe_jsonb_build_array})");
//                 }
//                 SomethingOptionToUpdate::StdOptionOptionStdVecVecGenericWithId(value) => {
//                     {
//                         let mut ids: std::vec::Vec<&postgresql_crud::JsonUuid> = vec![];
//                         if let Some(value) = &value.value {
//                             for element in &value.update {
//                                 if ids.contains(&&element.id) {
//                                     return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdOptionOptionStdVecVecGenericWithIdDoggieNotUniqueId { std_option_option_std_vec_vec_generic_with_id_doggie_not_unique_id: element.id, code_occurence: error_occurence_lib::code_occurence!() });
//                                 } else {
//                                     ids.push(&element.id);
//                                 }
//                             }
//                             for element in &value.delete {
//                                 if ids.contains(&element) {
//                                     return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdOptionOptionStdVecVecGenericWithIdDoggieNotUniqueId { std_option_option_std_vec_vec_generic_with_id_doggie_not_unique_id: *element, code_occurence: error_occurence_lib::code_occurence!() });
//                                 } else {
//                                     ids.push(&element);
//                                 }
//                             }
//                         }
//                     }
//                     let current_jsonb_set_target = format!("{jsonb_set_target}->'std_option_option_std_vec_vec_generic_with_id'");
//                     match &value.value {
//                         Some(value) => {
//                             let mut update_query_part_acc = std::string::String::default();
//                             for (index, element) in &value.update.iter().enumerate().collect::<std::vec::Vec<(usize, &DoggieOptionsToUpdate)>>() {
//                                 match postgresql_crud::JsonArrayElementUpdateBindQuery::try_generate_update_bind_increments(*element, &jsonb_set_accumulator, &jsonb_set_target, &jsonb_set_path, increment, is_array_object_element.clone()) {
//                                     Ok(value) => {
//                                         if let Some(value) = value {
//                                             update_query_part_acc.push_str(&value);
//                                         }
//                                     }
//                                     Err(error) => {
//                                         return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements { std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments: error, code_occurence: error_occurence_lib::code_occurence!() });
//                                     }
//                                 }
//                             }
//                             let delete_query_part_acc = {
//                                 if value.delete.is_empty() {
//                                     std::string::String::default()
//                                 } else {
//                                     let mut delete_query_part_acc = std::string::String::default();
//                                     for (index, element) in &value.delete.iter().enumerate().collect::<std::vec::Vec<(usize, &postgresql_crud::JsonUuid)>>() {
//                                         match increment.checked_add(1) {
//                                             Some(value) => {
//                                                 *increment = value;
//                                                 let maybe_space_and_space = if delete_query_part_acc.is_empty() { "" } else { " and " };
//                                                 delete_query_part_acc.push_str(&format!("{maybe_space_and_space}elem->>'id' <> ${increment}"));
//                                             }
//                                             None => {
//                                                 return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements { std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments: postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }, code_occurence: error_occurence_lib::code_occurence!() });
//                                             }
//                                         }
//                                     }
//                                     delete_query_part_acc
//                                 }
//                             };
//                             let mut create_query_part_acc = std::string::String::default();
//                             for (index, element) in &value.create.iter().enumerate().collect::<std::vec::Vec<(usize, &DoggieToCreate)>>() {
//                                 match postgresql_crud::JsonArrayElementCreateBindQuery::try_generate_create_bind_increments(*element, increment) {
//                                     Ok(value) => {
//                                         if let Some(value) = value {
//                                             create_query_part_acc.push_str(&format!("{value},"));
//                                         }
//                                     }
//                                     Err(error) => {
//                                         return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements { std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments: error, code_occurence: error_occurence_lib::code_occurence!() });
//                                     }
//                                 }
//                             }
//                             let _ = create_query_part_acc.pop();
//                             let maybe_jsonb_agg_case = if update_query_part_acc.is_empty() { std::string::String::from("elem") } else { format!("case {update_query_part_acc} else elem end") };
//                             let maybe_where = if delete_query_part_acc.is_empty() { std::string::String::default() } else { format!(" where {delete_query_part_acc}") };
//                             let maybe_jsonb_build_array = if create_query_part_acc.is_empty() { std::string::String::default() } else { format!(" || jsonb_build_array({create_query_part_acc})") };
//                             let maybe_jsonb_build_array_in_case_of_null = if create_query_part_acc.is_empty() { current_jsonb_set_target.clone() } else { format!("jsonb_build_array({create_query_part_acc})") };
//                             acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_option_option_std_vec_vec_generic_with_id}}',case when {jsonb_set_target}->'std_option_option_std_vec_vec_generic_with_id' = 'null' then {maybe_jsonb_build_array_in_case_of_null} else (select jsonb_agg({maybe_jsonb_agg_case}) from jsonb_array_elements({current_jsonb_set_target}) as elem {maybe_where}) {maybe_jsonb_build_array} end)");
//                         }
//                         None => match increment.checked_add(1) {
//                             Some(value) => {
//                                 *increment = value;
//                                 acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_option_option_std_vec_vec_generic_with_id}}',${increment})");
//                             }
//                             None => {
//                                 return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                             }
//                         },
//                     }
//                 }
//             }
//         }
//         Ok(acc)
//     }
//     fn bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         for element in self.0 {
//             match element {
//                 SomethingOptionToUpdate::StdPrimitiveI8(value) => {
//                     query = query.bind(sqlx::types::Json(value.value));
//                 }
//                 SomethingOptionToUpdate::StdOptionOptionStdPrimitiveI8(value) => {
//                     query = query.bind(sqlx::types::Json(value.value));
//                 }
//                 SomethingOptionToUpdate::StdVecVecStdPrimitiveI8(value) => {
//                     query = query.bind(sqlx::types::Json(value.value));
//                 }
//                 SomethingOptionToUpdate::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
//                     query = query.bind(sqlx::types::Json(value.value));
//                 }
//                 SomethingOptionToUpdate::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
//                     query = query.bind(sqlx::types::Json(value.value));
//                 }
//                 SomethingOptionToUpdate::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
//                     query = query.bind(sqlx::types::Json(value.value));
//                 }
//                 SomethingOptionToUpdate::Generic(value) => {
//                     query = value.value.bind_value_to_query(query);
//                 }
//                 SomethingOptionToUpdate::StdOptionOptionGeneric(value) => match value.value {
//                     Some(value) => {
//                         query = value.bind_value_to_query(query);
//                     }
//                     None => {
//                         query = query.bind(sqlx::types::Json(None::<std::option::Option<MouseToCreate>>));
//                     }
//                 },
//                 SomethingOptionToUpdate::StdVecVecGenericWithId(value) => {
//                     for element in &value.value.update {
//                         query = postgresql_crud::JsonArrayElementUpdateBindQuery::bind_update_value_to_query(element.clone(), query);
//                     }
//                     for element in &value.value.delete {
//                         query = query.bind(element.0.to_string());
//                     }
//                     for element in &value.value.create {
//                         query = postgresql_crud::JsonArrayElementCreateBindQuery::bind_create_value_to_query(element.clone(), query);
//                     }
//                 }
//                 SomethingOptionToUpdate::StdOptionOptionStdVecVecGenericWithId(value) => match &value.value {
//                     Some(value) => {
//                         for element in &value.update {
//                             query = postgresql_crud::JsonArrayElementUpdateBindQuery::bind_update_value_to_query(element.clone(), query);
//                         }
//                         for element in &value.delete {
//                             query = query.bind(element.0.to_string());
//                         }
//                         for element in &value.create {
//                             query = postgresql_crud::JsonArrayElementCreateBindQuery::bind_create_value_to_query(element.clone(), query);
//                         }
//                     }
//                     None => {
//                         query = query.bind(sqlx::types::Json(None::<std::option::Option<postgresql_crud::JsonArrayChange<DoggieToCreate, DoggieOptionsToUpdate>>>));
//                     }
//                 },
//             }
//         }
//         query
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde :: Serialize,
//     serde ::
// Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
// pub struct SomethingToCreate {
//     pub std_primitive_i8: postgresql_crud::JsonStdPrimitiveI8,
//     pub std_option_option_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdPrimitiveI8,
//     pub std_vec_vec_std_primitive_i8: postgresql_crud::JsonStdVecVecStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8,
//     pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
//     pub generic: postgresql_crud::JsonGeneric<CatToCreate>,
//     pub std_option_option_generic: postgresql_crud::JsonStdOptionOptionGeneric<MouseToCreate>,
//     pub std_vec_vec_generic_with_id: postgresql_crud::JsonStdVecVecGenericWithId<DoggieToCreate>,
//     pub std_option_option_std_vec_vec_generic_with_id: postgresql_crud::JsonStdOptionOptionStdVecVecGenericWithId<DoggieToCreate>,
// }
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SomethingToCreate {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self { std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), generic: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_option_option_generic: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_vec_vec_generic_with_id: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_option_option_std_vec_vec_generic_with_id: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }
//     }
// }
// impl<'a> postgresql_crud::BindQuery<'a> for SomethingToCreate {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         todo!()
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         let mut increments = std::string::String::from("");
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("'std_primitive_i8',${increment},"));
//             }
//             None => {
//                 return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("'std_option_option_std_primitive_i8',${increment},"));
//             }
//             None => {
//                 return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("'std_vec_vec_std_primitive_i8',${increment},"));
//             }
//             None => {
//                 return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("'std_option_option_std_vec_vec_std_primitive_i8',${increment},"));
//             }
//             None => {
//                 return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("'std_vec_vec_std_option_option_std_primitive_i8',${increment},"));
//             }
//             None => {
//                 return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("'std_option_option_std_vec_vec_std_option_option_std_primitive_i8',${increment},"));
//             }
//             None => {
//                 return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         match self.generic.0.try_generate_bind_increments(increment) {
//             Ok(value) => {
//                 increments.push_str(&format!("'generic',{value},"));
//             }
//             Err(error) => {
//                 return Err(error);
//             }
//         }
//         match &self.std_option_option_generic.0 {
//             Some(value) => match value.try_generate_bind_increments(increment) {
//                 Ok(value) => {
//                     increments.push_str(&format!("'std_option_option_generic',{value},"));
//                 }
//                 Err(error) => {
//                     return Err(error);
//                 }
//             },
//             None => match increment.checked_add(1) {
//                 Some(value) => {
//                     *increment = value;
//                     increments.push_str(&format!("'std_option_option_generic',${increment},"));
//                 }
//                 None => {
//                     return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                 }
//             },
//         }
//         {
//             let mut acc = std::string::String::default();
//             for element in &self.std_vec_vec_generic_with_id.0 {
//                 match element.try_generate_bind_increments(increment) {
//                     Ok(value) => {
//                         acc.push_str(&format!("{value},"));
//                     }
//                     Err(error) => {
//                         return Err(error);
//                     }
//                 }
//             }
//             let _ = acc.pop();
//             increments.push_str(&format!("'std_vec_vec_generic_with_id',jsonb_build_array({acc}),"));
//         }
//         {
//             let maybe_jsonb_build_array_stringified: std::string::String;
//             match &self.std_option_option_std_vec_vec_generic_with_id.0 {
//                 Some(value) => {
//                     let mut acc = std::string::String::default();
//                     for element in value {
//                         match element.try_generate_bind_increments(increment) {
//                             Ok(value) => {
//                                 acc.push_str(&format!("{value},"));
//                             }
//                             Err(error) => {
//                                 return Err(error);
//                             }
//                         }
//                     }
//                     let _ = acc.pop();
//                     maybe_jsonb_build_array_stringified = format!("jsonb_build_array({acc})");
//                 }
//                 None => match increment.checked_add(1) {
//                     Some(value) => {
//                         *increment = value;
//                         maybe_jsonb_build_array_stringified = format!("${increment}");
//                     }
//                     None => {
//                         return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 },
//             }
//             increments.push_str(&format!("'std_option_option_std_vec_vec_generic_with_id',{maybe_jsonb_build_array_stringified},"));
//         }
//         let _ = increments.pop();
//         Ok(format!("jsonb_build_object({increments})"))
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(sqlx::types::Json(self.std_primitive_i8.0));
//         query = query.bind(sqlx::types::Json(self.std_option_option_std_primitive_i8.0));
//         query = query.bind(sqlx::types::Json(self.std_vec_vec_std_primitive_i8.0));
//         query = query.bind(sqlx::types::Json(self.std_option_option_std_vec_vec_std_primitive_i8.0));
//         query = query.bind(sqlx::types::Json(self.std_vec_vec_std_option_option_std_primitive_i8.0));
//         query = query.bind(sqlx::types::Json(self.std_option_option_std_vec_vec_std_option_option_std_primitive_i8.0));
//         query = self.generic.0.bind_value_to_query(query);
//         match self.std_option_option_generic.0 {
//             Some(value) => {
//                 query = value.bind_value_to_query(query);
//             }
//             None => {
//                 query = query.bind(sqlx::types::Json(None::<std::option::Option<MouseToCreate>>));
//             }
//         }
//         for element in self.std_vec_vec_generic_with_id.0 {
//             query = element.bind_value_to_query(query);
//         }
//         match self.std_option_option_std_vec_vec_generic_with_id.0 {
//             Some(value) => {
//                 for element in value {
//                     query = element.bind_value_to_query(query);
//                 }
//             }
//             None => {
//                 query = query.bind(sqlx::types::Json(None::<std::option::Option<std::vec::Vec<DoggieToCreate>>>));
//             }
//         }
//         query
//     }
// }
// impl postgresql_crud::CheckIdExistsInJsonGenericFields for Something {
//     fn check_id_exists_in_json_generic_fields(&self) {
//         let _: () = postgresql_crud::CheckIdExistsInJsonStdVecVecGenericWithId::check_id_exists_in_json_std_vec_vec_generic_with_id(&self.std_vec_vec_generic_with_id);
//         let _: () = postgresql_crud::CheckIdExistsInJsonStdOptionOptionStdVecVecGenericWithId::check_id_exists_in_json_std_option_option_std_vec_vec_generic_with_id(&self.std_option_option_std_vec_vec_generic_with_id);
//     }
// }
// ////////////
// impl std::fmt::Display for Doggie {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum DoggieFieldToRead {
//     #[serde(rename(serialize = "id", deserialize = "id"))]
//     Id,
//     #[serde(rename(serialize = "std_primitive_i16", deserialize = "std_primitive_i16"))]
//     StdPrimitiveI16,
// }
// impl error_occurence_lib::ToStdStringString for DoggieFieldToRead {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed {
//     FieldsFilterIsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUniqueFieldFilter {
//         #[eo_to_std_string_string_serialize_deserialize]
//         field: DoggieFieldToRead,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl error_occurence_lib::ToStdStringString for DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// impl postgresql_crud::GeneratePostgresqlQueryPartToRead<DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed, ()> for DoggieFieldToRead {
//     fn generate_postgresql_query_part_to_read_from_self_vec(value: &std::vec::Vec<Self>, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_optional: std::primitive::bool) -> Result<std::string::String, DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed> {
//         if value.is_empty() {
//             return Err(DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
//         }
//         let mut unique = vec![];
//         for element in value {
//             if unique.contains(&element) {
//                 return Err(DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed::NotUniqueFieldFilter { field: element.clone(), code_occurence: error_occurence_lib::code_occurence!() });
//             } else {
//                 unique.push(&element);
//             }
//         }
//         let mut acc = std::string::String::default();
//         for element in value {
//             acc.push_str(&format!(
//                 "{}||",
//                 match element {
//                     Self::Id => format!("jsonb_build_object('id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'id') = 'string' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'id') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.id is not string')) end )"),
//                     Self::StdPrimitiveI16 => format!("jsonb_build_object('std_primitive_i16',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i16') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i16') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_primitive_i16 is not number')) end )"),
//                 }
//             ));
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         let is_optional_query_part = match is_optional {
//             true => format!("when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null)"),
//             false => std::string::String::default(),
//         };
//         Ok({
//             let space_and_not_null = if is_optional { " and not null" } else { "" };
//             format!("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}){is_optional_query_part} else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object{space_and_not_null}')) end")
//         })
//     }
//     fn generate_postgresql_query_part_to_read(&self, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> Result<std::string::String, ()> {
//         match self {
//             Self::Id => Ok(format!("jsonb_build_object('id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'id') = 'string' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'id') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.id is not string')) end )")),
//             Self::StdPrimitiveI16 => Ok(format!("jsonb_build_object('std_primitive_i16',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i16') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i16') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_primitive_i16 is not number')) end )")),
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
// pub struct DoggieOptionsToRead {
//     id: std::option::Option<postgresql_crud::Uuid>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_primitive_i16: std::option::Option<postgresql_crud::Value<std::primitive::i16>>,
// }
// impl std::convert::From<Doggie> for DoggieOptionsToRead {
//     fn from(value: Doggie) -> Self {
//         Self { id: Some(value.id.0), std_primitive_i16: Some(postgresql_crud::Value { value: value.std_primitive_i16.0 }) }
//     }
// }
// impl<'de> serde::Deserialize<'de> for DoggieOptionsToRead {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[allow(non_camel_case_types)]
//         #[doc(hidden)]
//         enum __Field {
//             __field0,
//             __field1,
//             __ignore,
//         }
//         #[doc(hidden)]
//         struct __FieldVisitor;
//         impl serde::de::Visitor<'_> for __FieldVisitor {
//             type Value = __Field;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "field identifier")
//             }
//             fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     0u64 => serde::__private::Ok(__Field::__field0),
//                     1u64 => serde::__private::Ok(__Field::__field1),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//             fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     "id" => serde::__private::Ok(__Field::__field0),
//                     "std_primitive_i16" => serde::__private::Ok(__Field::__field1),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//             fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     b"id" => serde::__private::Ok(__Field::__field0),
//                     b"std_primitive_i16" => serde::__private::Ok(__Field::__field1),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//         }
//         impl<'de> serde::Deserialize<'de> for __Field {
//             #[inline]
//             fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//             where
//                 __D: serde::Deserializer<'de>,
//             {
//                 serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
//             }
//         }
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<DoggieOptionsToRead>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = DoggieOptionsToRead;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "struct DoggieOptionsToRead")
//             }
//             #[inline]
//             fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<postgresql_crud::Uuid, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct DoggieOptionsToRead with 2 elements"));
//                     }
//                 };
//                 let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<std::primitive::i16, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(1usize, &"struct DoggieOptionsToRead with 2 elements"));
//                     }
//                 };
//                 serde::__private::Ok(DoggieOptionsToRead {
//                     id: match __field0 {
//                         Some(value) => match value {
//                             Ok(value) => Some(value),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_primitive_i16: match __field1 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                 })
//             }
//             #[inline]
//             fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::MapAccess<'de>,
//             {
//                 let mut __field0: serde::__private::Option<std::option::Option<std::result::Result<postgresql_crud::Uuid, std::string::String>>> = serde::__private::None;
//                 let mut __field1: serde::__private::Option<std::option::Option<std::result::Result<std::primitive::i16, std::string::String>>> = serde::__private::None;
//                 while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
//                     match __key {
//                         __Field::__field0 => {
//                             if serde::__private::Option::is_some(&__field0) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("id"));
//                             }
//                             __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<postgresql_crud::Uuid, std::string::String>>>(&mut __map)?);
//                         }
//                         __Field::__field1 => {
//                             if serde::__private::Option::is_some(&__field1) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i16"));
//                             }
//                             __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<std::primitive::i16, std::string::String>>>(&mut __map)?);
//                         }
//                         _ => {
//                             let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
//                         }
//                     }
//                 }
//                 let __field0 = match __field0 {
//                     serde::__private::Some(__field0) => __field0,
//                     serde::__private::None => serde::__private::de::missing_field("id")?,
//                 };
//                 let __field1 = match __field1 {
//                     serde::__private::Some(__field1) => __field1,
//                     serde::__private::None => serde::__private::de::missing_field("std_primitive_i16")?,
//                 };
//                 serde::__private::Ok(DoggieOptionsToRead {
//                     id: match __field0 {
//                         Some(value) => match value {
//                             Ok(value) => Some(value),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                     std_primitive_i16: match __field1 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                 })
//             }
//         }
//         #[doc(hidden)]
//         const FIELDS: &'static [&'static str] = &["id", "std_primitive_i16"];
//         serde::Deserializer::deserialize_struct(__deserializer, "DoggieOptionsToRead", FIELDS, __Visitor { marker: serde::__private::PhantomData::<DoggieOptionsToRead>, lifetime: serde::__private::PhantomData })
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
// pub struct DoggieReader(pub DoggieOptionsToRead);
// impl<'de> serde::Deserialize<'de> for DoggieReader {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<DoggieReader>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = DoggieReader;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "tuple struct DoggieReader")
//             }
//             #[inline]
//             fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
//             where
//                 __E: serde::Deserializer<'de>,
//             {
//                 let __field0: Result<DoggieOptionsToRead, std::string::String> = <Result<DoggieOptionsToRead, std::string::String> as serde::Deserialize>::deserialize(__e)?;
//                 serde::__private::Ok(DoggieReader(match __field0 {
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
//                 let __field0 = match serde::de::SeqAccess::next_element::<Result<DoggieOptionsToRead, std::string::String>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct DoggieReader with 1 element"));
//                     }
//                 };
//                 serde::__private::Ok(DoggieReader(match __field0 {
//                     Ok(value) => value,
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(error));
//                     }
//                 }))
//             }
//         }
//         serde::Deserializer::deserialize_newtype_struct(__deserializer, "DoggieReader", __Visitor { marker: serde::__private::PhantomData::<DoggieReader>, lifetime: serde::__private::PhantomData })
//     }
// }
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for Doggie {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self { id: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), std_primitive_i16: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum DoggieFieldToUpdate {
//     #[serde(rename(serialize = "std_primitive_i16", deserialize = "std_primitive_i16"))]
//     StdPrimitiveI16,
// }
// impl error_occurence_lib::ToStdStringString for DoggieFieldToUpdate {
//     fn to_std_string_string(&self) -> std::string::String {
//         match &self {
//             Self::StdPrimitiveI16 => "std_primitive_i16".to_owned(),
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub enum DoggieOptionToUpdate {
//     #[serde(rename(serialize = "std_primitive_i16", deserialize = "std_primitive_i16"))]
//     StdPrimitiveI16(postgresql_crud::Value<std::primitive::i16>),
// }
// impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for DoggieOptionToUpdate {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<DoggieOptionToUpdate> {
//         vec![DoggieOptionToUpdate::StdPrimitiveI16(postgresql_crud::Value { value: <postgresql_crud::JsonStdPrimitiveI16 as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0 })]
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct DoggieOptionsToUpdate {
//     pub id: postgresql_crud::JsonUuid,
//     pub fields: std::vec::Vec<DoggieOptionToUpdate>,
// }
// impl postgresql_crud::GetJsonId for DoggieOptionsToUpdate {
//     fn get_json_id(&self) -> &postgresql_crud::JsonUuid {
//         &self.id
//     }
// }
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for DoggieOptionsToUpdate {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self { id: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(), fields: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed {
//     FieldsIsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUniqueField {
//         #[eo_to_std_string_string_serialize_deserialize]
//         field: DoggieFieldToUpdate,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     CheckedAdd {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed {
//     CheckedAdd {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     FieldsIsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUniqueField {
//         #[eo_to_std_string_string_serialize_deserialize]
//         field: DoggieFieldToUpdate,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl postgresql_crud::JsonArrayElementUpdateBindQuery<DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed> for DoggieOptionsToUpdate {
//     fn try_generate_update_bind_increments(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64, is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple) -> Result<std::option::Option<std::string::String>, DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(new_increment_value) => {
//                 *increment = new_increment_value;
//                 if self.fields.is_empty() {
//                     return Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed::FieldsIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
//                 }
//                 {
//                     let mut acc = vec![];
//                     for element in &self.fields {
//                         match element {
//                             DoggieOptionToUpdate::StdPrimitiveI16(_) => {
//                                 let value = DoggieFieldToUpdate::StdPrimitiveI16;
//                                 if acc.contains(&value) {
//                                     return Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                                 } else {
//                                     acc.push(value);
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 let id_increment = format!("${increment}");
//                 let mut acc = std::string::String::default();
//                 for element in &self.fields {
//                     match &element {
//                         DoggieOptionToUpdate::StdPrimitiveI16(_) => match increment.checked_add(1) {
//                             Some(value) => {
//                                 *increment = value;
//                                 acc.push_str(&format!("'{{std_primitive_i16}}',${increment}"));
//                             }
//                             None => {
//                                 return Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                             }
//                         },
//                     }
//                     acc.push_str(",");
//                 }
//                 let _ = acc.pop();
//                 Ok(Some(format!("when elem->>'id' = {id_increment} then jsonb_set(elem,{acc})")))
//             }
//             None => Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn bind_update_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(self.id.0.to_string());
//         for element in self.fields {
//             match element {
//                 DoggieOptionToUpdate::StdPrimitiveI16(value) => {
//                     query = query.bind(sqlx::types::Json(value.value));
//                 }
//             }
//         }
//         query
//     }
// }
// impl postgresql_crud::JsonArrayElementCreateBindQuery for DoggieToCreate {
//     fn try_generate_create_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::option::Option<std::string::String>, postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed> {
//         match postgresql_crud::BindQuery::try_generate_bind_increments(self, increment) {
//             Ok(value) => Ok(Some(value)),
//             Err(error) => Err(postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed::TryGenerateBindIncrements { error: error, code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn bind_create_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = postgresql_crud::BindQuery::bind_value_to_query(self, query);
//         query
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde :: Serialize,
//     serde ::
// Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
// pub struct DoggieToCreate {
//     pub std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16,
// }
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for DoggieToCreate {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self { std_primitive_i16: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }
//     }
// }
// impl<'a> postgresql_crud::BindQuery<'a> for DoggieToCreate {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         todo!()
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         let mut increments = std::string::String::from("'id', to_jsonb(gen_random_uuid()),");
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("'std_primitive_i16',${increment},"));
//             }
//             None => {
//                 return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         let _ = increments.pop();
//         Ok(format!("jsonb_build_object({increments})"))
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(sqlx::types::Json(self.std_primitive_i16.0));
//         query
//     }
// }
// impl postgresql_crud::GetJsonId for Doggie {
//     fn get_json_id(&self) -> &postgresql_crud::JsonUuid {
//         &self.id
//     }
// }
// impl postgresql_crud::CheckIdExistsInJsonGenericFields for Doggie {
//     fn check_id_exists_in_json_generic_fields(&self) {}
// }
// ////////
// impl std::fmt::Display for Cat {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum CatFieldToRead {
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32,
// }
// impl error_occurence_lib::ToStdStringString for CatFieldToRead {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum CatGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed {
//     FieldsFilterIsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUniqueFieldFilter {
//         #[eo_to_std_string_string_serialize_deserialize]
//         field: CatFieldToRead,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl error_occurence_lib::ToStdStringString for CatGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// impl postgresql_crud::GeneratePostgresqlQueryPartToRead<CatGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed, ()> for CatFieldToRead {
//     fn generate_postgresql_query_part_to_read_from_self_vec(value: &std::vec::Vec<Self>, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_optional: std::primitive::bool) -> Result<std::string::String, CatGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed> {
//         if value.is_empty() {
//             return Err(CatGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
//         }
//         let mut unique = vec![];
//         for element in value {
//             if unique.contains(&element) {
//                 return Err(CatGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed::NotUniqueFieldFilter { field: element.clone(), code_occurence: error_occurence_lib::code_occurence!() });
//             } else {
//                 unique.push(&element);
//             }
//         }
//         let mut acc = std::string::String::default();
//         for element in value {
//             acc.push_str(&format!(
//                 "{}||",
//                 match element {
//                     Self::StdPrimitiveI32 => format!("jsonb_build_object('std_primitive_i32',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i32') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i32') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_primitive_i32 is not number')) end )"),
//                 }
//             ));
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         let is_optional_query_part = match is_optional {
//             true => format!("when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null)"),
//             false => std::string::String::default(),
//         };
//         Ok({
//             let space_and_not_null = if is_optional { " and not null" } else { "" };
//             format!("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}){is_optional_query_part} else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object{space_and_not_null}')) end")
//         })
//     }
//     fn generate_postgresql_query_part_to_read(&self, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> Result<std::string::String, ()> {
//         match self {
//             Self::StdPrimitiveI32 => Ok(format!("jsonb_build_object('std_primitive_i32',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i32') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i32') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_primitive_i32 is not number')) end )")),
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
// pub struct CatOptionsToRead {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_primitive_i32: std::option::Option<postgresql_crud::Value<std::primitive::i32>>,
// }
// impl std::convert::From<Cat> for CatOptionsToRead {
//     fn from(value: Cat) -> Self {
//         Self { std_primitive_i32: Some(postgresql_crud::Value { value: value.std_primitive_i32.0 }) }
//     }
// }
// //here
// // impl std::convert::From<postgresql_crud::JsonGeneric<Cat>> for CatOptionsToRead {
// //     fn from(value: postgresql_crud::JsonGeneric<Cat>) -> Self {
// //         Self {
// //             std_primitive_i32: Some(postgresql_crud::Value { value: value.0.std_primitive_i32.0 })
// //         }
// //     }
// // }
// // impl std::convert::From<postgresql_crud::JsonStdOptionOptionGeneric<Cat>> for std::option::Option<CatOptionsToRead> {
// //     fn from(value: postgresql_crud::JsonStdOptionOptionGeneric<Cat>) -> Self {
// //         // match 
// //         todo!()
// //         // Self {
// //         //     std_primitive_i32: Some(postgresql_crud::Value { value: value.0.std_primitive_i32.0 })
// //         // }
// //     }
// // }

//             // generic: Some(postgresql_crud::Value {
//             //     value: CatOptionsToRead::from(value.generic)
//             // }),
//             // std_option_option_generic: Some(postgresql_crud::Value {
//             //     value:{
//             //         match value.std_option_option_generic.0 {
//             //             Some(value) => Some(MouseOptionsToRead::from(value)),
//             //             None => None,
//             //         }
//             //     }
//             // }),
//             // std_vec_vec_generic_with_id: Some(postgresql_crud::Value { value: value.std_vec_vec_generic_with_id.0.into_iter().map(|element| DoggieOptionsToRead::from(element)).collect::<std::vec::Vec<DoggieOptionsToRead>>() }),
//             // std_option_option_std_vec_vec_generic_with_id: Some(postgresql_crud::Value {
//             //     value: match value.std_option_option_std_vec_vec_generic_with_id.0 {
//             //         Some(value) => Some(value.into_iter().map(|element| DoggieOptionsToRead::from(element)).collect::<std::vec::Vec<DoggieOptionsToRead>>()),
//             //         None => None,
//             //     },
//             // }),

//     //Cat
//     // pub generic: postgresql_crud::JsonGeneric<Cat>,
//     // pub std_option_option_generic: postgresql_crud::JsonStdOptionOptionGeneric<Mouse>,
//     // pub std_vec_vec_generic_with_id: postgresql_crud::JsonStdVecVecGenericWithId<Doggie>,
//     // pub std_option_option_std_vec_vec_generic_with_id: postgresql_crud::JsonStdOptionOptionStdVecVecGenericWithId<Doggie>,

//     //OptionsToRead
//     // generic: std::option::Option<postgresql_crud::Value<CatOptionsToRead>>,
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // std_option_option_generic: std::option::Option<postgresql_crud::Value<std::option::Option<MouseOptionsToRead>>>,
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // std_vec_vec_generic_with_id: std::option::Option<postgresql_crud::Value<std::vec::Vec<DoggieOptionsToRead>>>,
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // std_option_option_std_vec_vec_generic_with_id: std::option::Option<postgresql_crud::Value<std::option::Option<std::vec::Vec<DoggieOptionsToRead>>>>,
//     //

// //
// // From<JsonStdOptionOptionGeneric<Mouse>>` is not implemented for `std::option::Option<MouseOptionsToRead>`
// // impl std::convert::From<postgresql_crud::JsonStdOptionOptionGeneric<Cat>> for CatOptionsToRead {
// //     fn from(value: postgresql_crud::JsonStdOptionOptionGeneric<Cat>) -> Self {

// //         // std_primitive_i32: std::option::Option<postgresql_crud::Value<std::primitive::i32>>,

// // //

// // // pub struct Cat {
// // //     // pub id: postgresql_crud::JsonUuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids
// // //     pub std_primitive_i32: postgresql_crud::JsonStdPrimitiveI32,
// // // }
// // //

// //         Self {
// //             std_primitive_i32: Some(postgresql_crud::Value { value:  })
// //         }
// //     }
// // }
// //
// impl<'de> serde::Deserialize<'de> for CatOptionsToRead {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[allow(non_camel_case_types)]
//         #[doc(hidden)]
//         enum __Field {
//             __field0,
//             __ignore,
//         }
//         #[doc(hidden)]
//         struct __FieldVisitor;
//         impl serde::de::Visitor<'_> for __FieldVisitor {
//             type Value = __Field;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "field identifier")
//             }
//             fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     0u64 => serde::__private::Ok(__Field::__field0),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//             fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     "std_primitive_i32" => serde::__private::Ok(__Field::__field0),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//             fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     b"std_primitive_i32" => serde::__private::Ok(__Field::__field0),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//         }
//         impl<'de> serde::Deserialize<'de> for __Field {
//             #[inline]
//             fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//             where
//                 __D: serde::Deserializer<'de>,
//             {
//                 serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
//             }
//         }
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<CatOptionsToRead>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = CatOptionsToRead;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "struct CatOptionsToRead")
//             }
//             #[inline]
//             fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<std::primitive::i32, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct CatOptionsToRead with 1 elements"));
//                     }
//                 };
//                 serde::__private::Ok(CatOptionsToRead {
//                     std_primitive_i32: match __field0 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                 })
//             }
//             #[inline]
//             fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::MapAccess<'de>,
//             {
//                 let mut __field0: serde::__private::Option<std::option::Option<std::result::Result<std::primitive::i32, std::string::String>>> = serde::__private::None;
//                 while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
//                     match __key {
//                         __Field::__field0 => {
//                             if serde::__private::Option::is_some(&__field0) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i32"));
//                             }
//                             __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<std::primitive::i32, std::string::String>>>(&mut __map)?);
//                         }
//                         _ => {
//                             let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
//                         }
//                     }
//                 }
//                 let __field0 = match __field0 {
//                     serde::__private::Some(__field0) => __field0,
//                     serde::__private::None => serde::__private::de::missing_field("std_primitive_i32")?,
//                 };
//                 serde::__private::Ok(CatOptionsToRead {
//                     std_primitive_i32: match __field0 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                 })
//             }
//         }
//         #[doc(hidden)]
//         const FIELDS: &'static [&'static str] = &["std_primitive_i32"];
//         serde::Deserializer::deserialize_struct(__deserializer, "CatOptionsToRead", FIELDS, __Visitor { marker: serde::__private::PhantomData::<CatOptionsToRead>, lifetime: serde::__private::PhantomData })
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
// pub struct CatReader(pub CatOptionsToRead);
// impl<'de> serde::Deserialize<'de> for CatReader {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<CatReader>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = CatReader;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "tuple struct CatReader")
//             }
//             #[inline]
//             fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
//             where
//                 __E: serde::Deserializer<'de>,
//             {
//                 let __field0: Result<CatOptionsToRead, std::string::String> = <Result<CatOptionsToRead, std::string::String> as serde::Deserialize>::deserialize(__e)?;
//                 serde::__private::Ok(CatReader(match __field0 {
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
//                 let __field0 = match serde::de::SeqAccess::next_element::<Result<CatOptionsToRead, std::string::String>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct CatReader with 1 element"));
//                     }
//                 };
//                 serde::__private::Ok(CatReader(match __field0 {
//                     Ok(value) => value,
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(error));
//                     }
//                 }))
//             }
//         }
//         serde::Deserializer::deserialize_newtype_struct(__deserializer, "CatReader", __Visitor { marker: serde::__private::PhantomData::<CatReader>, lifetime: serde::__private::PhantomData })
//     }
// }
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for Cat {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self { std_primitive_i32: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum CatFieldToUpdate {
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32,
// }
// impl error_occurence_lib::ToStdStringString for CatFieldToUpdate {
//     fn to_std_string_string(&self) -> std::string::String {
//         match &self {
//             Self::StdPrimitiveI32 => "std_primitive_i32".to_owned(),
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub enum CatOptionToUpdate {
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32(postgresql_crud::Value<std::primitive::i32>),
// }
// impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CatOptionToUpdate {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<CatOptionToUpdate> {
//         vec![CatOptionToUpdate::StdPrimitiveI32(postgresql_crud::Value { value: <postgresql_crud::JsonStdPrimitiveI32 as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0 })]
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct CatOptionsToUpdate(pub std::vec::Vec<CatOptionToUpdate>);
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CatOptionsToUpdate {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed {
//     FieldsIsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUniqueField {
//         #[eo_to_std_string_string_serialize_deserialize]
//         field: CatFieldToUpdate,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     CheckedAdd {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl postgresql_crud::GeneratePostgresqlQueryPartToUpdate<CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed> for CatOptionsToUpdate {
//     fn try_generate_bind_increments(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64, is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple) -> Result<std::string::String, CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed> {
//         if self.0.is_empty() {
//             return Err(CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed::FieldsIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
//         }
//         {
//             let mut acc = vec![];
//             for element in &self.0 {
//                 match element {
//                     CatOptionToUpdate::StdPrimitiveI32(_) => {
//                         let value = CatFieldToUpdate::StdPrimitiveI32;
//                         if acc.contains(&value) {
//                             return Err(CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                 }
//             }
//         }
//         let mut acc = std::string::String::from(jsonb_set_accumulator);
//         let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
//             true => std::string::String::default(),
//             false => format!("{jsonb_set_path},"),
//         };
//         for element in &self.0 {
//             match &element {
//                 CatOptionToUpdate::StdPrimitiveI32(_) => match increment.checked_add(1) {
//                     Some(value) => {
//                         *increment = value;
//                         acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_primitive_i32}}',${increment})");
//                     }
//                     None => {
//                         return Err(CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 },
//             }
//         }
//         Ok(acc)
//     }
//     fn bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         for element in self.0 {
//             match element {
//                 CatOptionToUpdate::StdPrimitiveI32(value) => {
//                     query = query.bind(sqlx::types::Json(value.value));
//                 }
//             }
//         }
//         query
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde :: Serialize,
//     serde ::
// Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
// pub struct CatToCreate {
//     pub std_primitive_i32: postgresql_crud::JsonStdPrimitiveI32,
// }
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CatToCreate {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self { std_primitive_i32: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }
//     }
// }
// impl<'a> postgresql_crud::BindQuery<'a> for CatToCreate {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         todo!()
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         let mut increments = std::string::String::from("");
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("'std_primitive_i32',${increment},"));
//             }
//             None => {
//                 return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         let _ = increments.pop();
//         Ok(format!("jsonb_build_object({increments})"))
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(sqlx::types::Json(self.std_primitive_i32.0));
//         query
//     }
// }
// impl postgresql_crud::CheckIdExistsInJsonGenericFields for Cat {
//     fn check_id_exists_in_json_generic_fields(&self) {}
// }
// ///////
// impl std::fmt::Display for Mouse {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum MouseFieldToRead {
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32,
// }
// impl error_occurence_lib::ToStdStringString for MouseFieldToRead {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// #[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum MouseGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed {
//     FieldsFilterIsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUniqueFieldFilter {
//         #[eo_to_std_string_string_serialize_deserialize]
//         field: MouseFieldToRead,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl error_occurence_lib::ToStdStringString for MouseGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// impl postgresql_crud::GeneratePostgresqlQueryPartToRead<MouseGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed, ()> for MouseFieldToRead {
//     fn generate_postgresql_query_part_to_read_from_self_vec(value: &std::vec::Vec<Self>, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_optional: std::primitive::bool) -> Result<std::string::String, MouseGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed> {
//         if value.is_empty() {
//             return Err(MouseGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
//         }
//         let mut unique = vec![];
//         for element in value {
//             if unique.contains(&element) {
//                 return Err(MouseGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed::NotUniqueFieldFilter { field: element.clone(), code_occurence: error_occurence_lib::code_occurence!() });
//             } else {
//                 unique.push(&element);
//             }
//         }
//         let mut acc = std::string::String::default();
//         for element in value {
//             acc.push_str(&format!(
//                 "{}||",
//                 match element {
//                     Self::StdPrimitiveI32 => format!("jsonb_build_object('std_primitive_i32',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i32') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i32') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_primitive_i32 is not number')) end )"),
//                 }
//             ));
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         let is_optional_query_part = match is_optional {
//             true => format!("when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null)"),
//             false => std::string::String::default(),
//         };
//         Ok({
//             let space_and_not_null = if is_optional { " and not null" } else { "" };
//             format!("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}){is_optional_query_part} else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object{space_and_not_null}')) end")
//         })
//     }
//     fn generate_postgresql_query_part_to_read(&self, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> Result<std::string::String, ()> {
//         match self {
//             Self::StdPrimitiveI32 => Ok(format!("jsonb_build_object('std_primitive_i32',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i32') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i32') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_primitive_i32 is not number')) end )")),
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
// pub struct MouseOptionsToRead {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_primitive_i32: std::option::Option<postgresql_crud::Value<std::primitive::i32>>,
// }
// impl std::convert::From<Mouse> for MouseOptionsToRead {
//     fn from(value: Mouse) -> Self {
//         Self { std_primitive_i32: Some(postgresql_crud::Value { value: value.std_primitive_i32.0 }) }
//     }
// }
// impl<'de> serde::Deserialize<'de> for MouseOptionsToRead {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[allow(non_camel_case_types)]
//         #[doc(hidden)]
//         enum __Field {
//             __field0,
//             __ignore,
//         }
//         #[doc(hidden)]
//         struct __FieldVisitor;
//         impl serde::de::Visitor<'_> for __FieldVisitor {
//             type Value = __Field;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "field identifier")
//             }
//             fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     0u64 => serde::__private::Ok(__Field::__field0),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//             fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     "std_primitive_i32" => serde::__private::Ok(__Field::__field0),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//             fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     b"std_primitive_i32" => serde::__private::Ok(__Field::__field0),
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//         }
//         impl<'de> serde::Deserialize<'de> for __Field {
//             #[inline]
//             fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//             where
//                 __D: serde::Deserializer<'de>,
//             {
//                 serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
//             }
//         }
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<MouseOptionsToRead>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = MouseOptionsToRead;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "struct MouseOptionsToRead")
//             }
//             #[inline]
//             fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<std::primitive::i32, std::string::String>>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct MouseOptionsToRead with 1 elements"));
//                     }
//                 };
//                 serde::__private::Ok(MouseOptionsToRead {
//                     std_primitive_i32: match __field0 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                 })
//             }
//             #[inline]
//             fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::MapAccess<'de>,
//             {
//                 let mut __field0: serde::__private::Option<std::option::Option<std::result::Result<std::primitive::i32, std::string::String>>> = serde::__private::None;
//                 while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
//                     match __key {
//                         __Field::__field0 => {
//                             if serde::__private::Option::is_some(&__field0) {
//                                 return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i32"));
//                             }
//                             __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<std::result::Result<std::primitive::i32, std::string::String>>>(&mut __map)?);
//                         }
//                         _ => {
//                             let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
//                         }
//                     }
//                 }
//                 let __field0 = match __field0 {
//                     serde::__private::Some(__field0) => __field0,
//                     serde::__private::None => serde::__private::de::missing_field("std_primitive_i32")?,
//                 };
//                 serde::__private::Ok(MouseOptionsToRead {
//                     std_primitive_i32: match __field0 {
//                         Some(value) => match value {
//                             Ok(value) => Some(postgresql_crud::Value { value: value }),
//                             Err(error) => {
//                                 return Err(serde::de::Error::custom(error));
//                             }
//                         },
//                         None => None,
//                     },
//                 })
//             }
//         }
//         #[doc(hidden)]
//         const FIELDS: &'static [&'static str] = &["std_primitive_i32"];
//         serde::Deserializer::deserialize_struct(__deserializer, "MouseOptionsToRead", FIELDS, __Visitor { marker: serde::__private::PhantomData::<MouseOptionsToRead>, lifetime: serde::__private::PhantomData })
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
// pub struct MouseReader(pub MouseOptionsToRead);
// impl<'de> serde::Deserialize<'de> for MouseReader {
//     fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
//     where
//         __D: serde::Deserializer<'de>,
//     {
//         #[doc(hidden)]
//         struct __Visitor<'de> {
//             marker: serde::__private::PhantomData<MouseReader>,
//             lifetime: serde::__private::PhantomData<&'de ()>,
//         }
//         impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
//             type Value = MouseReader;
//             fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "tuple struct MouseReader")
//             }
//             #[inline]
//             fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
//             where
//                 __E: serde::Deserializer<'de>,
//             {
//                 let __field0: Result<MouseOptionsToRead, std::string::String> = <Result<MouseOptionsToRead, std::string::String> as serde::Deserialize>::deserialize(__e)?;
//                 serde::__private::Ok(MouseReader(match __field0 {
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
//                 let __field0 = match serde::de::SeqAccess::next_element::<Result<MouseOptionsToRead, std::string::String>>(&mut __seq)? {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct MouseReader with 1 element"));
//                     }
//                 };
//                 serde::__private::Ok(MouseReader(match __field0 {
//                     Ok(value) => value,
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(error));
//                     }
//                 }))
//             }
//         }
//         serde::Deserializer::deserialize_newtype_struct(__deserializer, "MouseReader", __Visitor { marker: serde::__private::PhantomData::<MouseReader>, lifetime: serde::__private::PhantomData })
//     }
// }
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for Mouse {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self { std_primitive_i32: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum MouseFieldToUpdate {
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32,
// }
// impl error_occurence_lib::ToStdStringString for MouseFieldToUpdate {
//     fn to_std_string_string(&self) -> std::string::String {
//         match &self {
//             Self::StdPrimitiveI32 => "std_primitive_i32".to_owned(),
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub enum MouseOptionToUpdate {
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32(postgresql_crud::Value<std::primitive::i32>),
// }
// impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for MouseOptionToUpdate {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<MouseOptionToUpdate> {
//         vec![MouseOptionToUpdate::StdPrimitiveI32(postgresql_crud::Value { value: <postgresql_crud::JsonStdPrimitiveI32 as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().0 })]
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct MouseOptionsToUpdate(pub std::vec::Vec<MouseOptionToUpdate>);
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for MouseOptionsToUpdate {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
// #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
// pub enum MouseOptionsToUpdateTryGenerateBindIncrementsErrorNamed {
//     FieldsIsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUniqueField {
//         #[eo_to_std_string_string_serialize_deserialize]
//         field: MouseFieldToUpdate,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     CheckedAdd {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl postgresql_crud::GeneratePostgresqlQueryPartToUpdate<MouseOptionsToUpdateTryGenerateBindIncrementsErrorNamed> for MouseOptionsToUpdate {
//     fn try_generate_bind_increments(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64, is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple) -> Result<std::string::String, MouseOptionsToUpdateTryGenerateBindIncrementsErrorNamed> {
//         if self.0.is_empty() {
//             return Err(MouseOptionsToUpdateTryGenerateBindIncrementsErrorNamed::FieldsIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
//         }
//         {
//             let mut acc = vec![];
//             for element in &self.0 {
//                 match element {
//                     MouseOptionToUpdate::StdPrimitiveI32(_) => {
//                         let value = MouseFieldToUpdate::StdPrimitiveI32;
//                         if acc.contains(&value) {
//                             return Err(MouseOptionsToUpdateTryGenerateBindIncrementsErrorNamed::NotUniqueField { field: value, code_occurence: error_occurence_lib::code_occurence!() });
//                         } else {
//                             acc.push(value);
//                         }
//                     }
//                 }
//             }
//         }
//         let mut acc = std::string::String::from(jsonb_set_accumulator);
//         let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
//             true => std::string::String::default(),
//             false => format!("{jsonb_set_path},"),
//         };
//         for element in &self.0 {
//             match &element {
//                 MouseOptionToUpdate::StdPrimitiveI32(_) => match increment.checked_add(1) {
//                     Some(value) => {
//                         *increment = value;
//                         acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_primitive_i32}}',${increment})");
//                     }
//                     None => {
//                         return Err(MouseOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//                     }
//                 },
//             }
//         }
//         Ok(acc)
//     }
//     fn bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         for element in self.0 {
//             match element {
//                 MouseOptionToUpdate::StdPrimitiveI32(value) => {
//                     query = query.bind(sqlx::types::Json(value.value));
//                 }
//             }
//         }
//         query
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde :: Serialize,
//     serde ::
// Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
// pub struct MouseToCreate {
//     pub std_primitive_i32: postgresql_crud::JsonStdPrimitiveI32,
// }
// impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for MouseToCreate {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self { std_primitive_i32: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() }
//     }
// }
// impl<'a> postgresql_crud::BindQuery<'a> for MouseToCreate {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         todo!()
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         let mut increments = std::string::String::from("");
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("'std_primitive_i32',${increment},"));
//             }
//             None => {
//                 return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         let _ = increments.pop();
//         Ok(format!("jsonb_build_object({increments})"))
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(sqlx::types::Json(self.std_primitive_i32.0));
//         query
//     }
// }
// impl postgresql_crud::CheckIdExistsInJsonGenericFields for Mouse {
//     fn check_id_exists_in_json_generic_fields(&self) {}
// }

// /////////////////////////////////
//todo this need for old version of update_many. later need to refactor update many and remove this