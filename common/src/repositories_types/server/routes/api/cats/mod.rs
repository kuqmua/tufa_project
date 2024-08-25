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
    // pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    // pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt,
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
    // pub sqlx_types_json_t_as_postgresql_json: postgresql_crud::SqlxTypesJsonAsPostgresqlJson::<Something>,//todo
    // postgresql_crud::StdPrimitiveBoolAsPostgresqlBool,//
    // pub sqlx_types_json_t_as_postgresql_json_not_null: postgresql_crud::SqlxTypesJsonAsPostgresqlJsonNotNull::<Something>
    // pub sqlx_types_json_t_as_postgresql_json_b: postgresql_crud::SqlxTypesJsonAsPostgresqlJsonB::<<Something>,//todo
    pub sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::SqlxTypesJsonAsPostgresqlJsonBNotNull::<Something>,//todo

    // pub serde_json_value_as_postgresql_json: postgresql_crud::SerdeJsonValueAsPostgresqlJson,
    // pub serde_json_value_as_postgresql_json_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonNotNull,
    // pub serde_json_value_as_postgresql_json_b: postgresql_crud::SerdeJsonValueAsPostgresqlJsonB,
    // pub serde_json_value_as_postgresql_json_b_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonBNotNull,
}

//todo enum tree support
//todo generate wrapper type for all possible json type
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,
     postgresql_crud::GeneratePostgresqlQueryPart
)] //user type must implement utoipa::ToSchema trait
pub struct Something {
    pub std_primitive_i8: postgresql_crud::JsonStdPrimitiveI8,
    pub std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16,
    pub std_primitive_i32: postgresql_crud::JsonStdPrimitiveI32,
    pub std_primitive_i64: postgresql_crud::JsonStdPrimitiveI64,
    pub std_primitive_i128: postgresql_crud::JsonStdPrimitiveI128,
    pub std_primitive_u8: postgresql_crud::JsonStdPrimitiveU8,
    pub std_primitive_u16: postgresql_crud::JsonStdPrimitiveU16,
    pub std_primitive_u32: postgresql_crud::JsonStdPrimitiveU32,
    pub std_primitive_u64: postgresql_crud::JsonStdPrimitiveU64,
    pub std_primitive_u128: postgresql_crud::JsonStdPrimitiveU128,
    pub std_primitive_f32: postgresql_crud::JsonStdPrimitiveF32,
    pub std_primitive_f64: postgresql_crud::JsonStdPrimitiveF64,
    pub std_primitive_bool: postgresql_crud::JsonStdPrimitiveBool,
    pub std_string_string: postgresql_crud::JsonStdStringString,

    pub std_option_option_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdPrimitiveI8,
    pub std_option_option_std_primitive_i16: postgresql_crud::JsonStdOptionOptionStdPrimitiveI16,
    pub std_option_option_std_primitive_i32: postgresql_crud::JsonStdOptionOptionStdPrimitiveI32,
    pub std_option_option_std_primitive_i64: postgresql_crud::JsonStdOptionOptionStdPrimitiveI64,
    pub std_option_option_std_primitive_i128: postgresql_crud::JsonStdOptionOptionStdPrimitiveI128,
    pub std_option_option_std_primitive_u8: postgresql_crud::JsonStdOptionOptionStdPrimitiveU8,
    pub std_option_option_std_primitive_u16: postgresql_crud::JsonStdOptionOptionStdPrimitiveU16,
    pub std_option_option_std_primitive_u32: postgresql_crud::JsonStdOptionOptionStdPrimitiveU32,
    pub std_option_option_std_primitive_u64: postgresql_crud::JsonStdOptionOptionStdPrimitiveU64,
    pub std_option_option_std_primitive_u128: postgresql_crud::JsonStdOptionOptionStdPrimitiveU128,
    pub std_option_option_std_primitive_f32: postgresql_crud::JsonStdOptionOptionStdPrimitiveF32,
    pub std_option_option_std_primitive_f64: postgresql_crud::JsonStdOptionOptionStdPrimitiveF64,
    pub std_option_option_std_primitive_bool: postgresql_crud::JsonStdOptionOptionStdPrimitiveBool,
    pub std_option_option_std_string_string: postgresql_crud::JsonStdOptionOptionStdStringString,

    pub std_vec_vec_std_primitive_i8: postgresql_crud::JsonStdVecVecStdPrimitiveI8,
    pub std_vec_vec_std_primitive_i16: postgresql_crud::JsonStdVecVecStdPrimitiveI16,
    pub std_vec_vec_std_primitive_i32: postgresql_crud::JsonStdVecVecStdPrimitiveI32,
    pub std_vec_vec_std_primitive_i64: postgresql_crud::JsonStdVecVecStdPrimitiveI64,
    pub std_vec_vec_std_primitive_i128: postgresql_crud::JsonStdVecVecStdPrimitiveI128,
    pub std_vec_vec_std_primitive_u8: postgresql_crud::JsonStdVecVecStdPrimitiveU8,
    pub std_vec_vec_std_primitive_u16: postgresql_crud::JsonStdVecVecStdPrimitiveU16,
    pub std_vec_vec_std_primitive_u32: postgresql_crud::JsonStdVecVecStdPrimitiveU32,
    pub std_vec_vec_std_primitive_u64: postgresql_crud::JsonStdVecVecStdPrimitiveU64,
    pub std_vec_vec_std_primitive_u128: postgresql_crud::JsonStdVecVecStdPrimitiveU128,
    pub std_vec_vec_std_primitive_f32: postgresql_crud::JsonStdVecVecStdPrimitiveF32,
    pub std_vec_vec_std_primitive_f64: postgresql_crud::JsonStdVecVecStdPrimitiveF64,
    pub std_vec_vec_std_primitive_bool: postgresql_crud::JsonStdVecVecStdPrimitiveBool,
    pub std_vec_vec_std_string_string: postgresql_crud::JsonStdVecVecStdStringString,

    pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8,
    pub std_option_option_std_vec_vec_std_primitive_i16: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI16,
    pub std_option_option_std_vec_vec_std_primitive_i32: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI32,
    pub std_option_option_std_vec_vec_std_primitive_i64: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI64,
    pub std_option_option_std_vec_vec_std_primitive_i128: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI128,
    pub std_option_option_std_vec_vec_std_primitive_u8: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveU8,
    pub std_option_option_std_vec_vec_std_primitive_u16: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveU16,
    pub std_option_option_std_vec_vec_std_primitive_u32: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveU32,
    pub std_option_option_std_vec_vec_std_primitive_u64: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveU64,
    pub std_option_option_std_vec_vec_std_primitive_u128: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveU128,
    pub std_option_option_std_vec_vec_std_primitive_f32: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveF32,
    pub std_option_option_std_vec_vec_std_primitive_f64: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveF64,
    pub std_option_option_std_vec_vec_std_primitive_bool: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveBool,
    pub std_option_option_std_vec_vec_std_string_string: postgresql_crud::JsonStdOptionOptionStdVecVecStdStringString,

    pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8,
    pub std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI16,
    pub std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI32,
    pub std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI64,
    pub std_vec_vec_std_option_option_std_primitive_i128: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI128,
    pub std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveU8,
    pub std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveU16,
    pub std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveU32,
    pub std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveU64,
    pub std_vec_vec_std_option_option_std_primitive_u128: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveU128,
    pub std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveF32,
    pub std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveF64,
    pub std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveBool,
    pub std_vec_vec_std_option_option_std_string_string: postgresql_crud::JsonStdVecVecStdOptionOptionStdStringString,

    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_i128: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_u128: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
    pub std_option_option_std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
    pub std_option_option_std_vec_vec_std_option_option_std_string_string: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString,

    pub generic: postgresql_crud::JsonGeneric<Doggie>,
    pub std_option_option_generic: postgresql_crud::JsonStdOptionOptionGeneric<Doggie>,
    pub std_vec_vec_generic: postgresql_crud::JsonStdVecVecGeneric<Doggie>,
    pub std_option_option_std_vec_vec_generic: postgresql_crud::JsonStdOptionOptionStdVecVecGeneric<Doggie>,
    pub std_vec_vec_std_option_option_generic: postgresql_crud::JsonStdVecVecStdOptionOptionGeneric<Doggie>,
    pub std_option_option_std_vec_vec_std_option_option_generic: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric<Doggie>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,
     postgresql_crud::GeneratePostgresqlQueryPart
)] //user type must implement utoipa::ToSchema trait
pub struct Doggie {
    pub std_string_string: postgresql_crud::JsonStdStringString,
}

#[test]
fn test_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() {
    let default = postgresql_crud::JsonGeneric(Something::default());
    println!("{default:#?}");
    let default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element: postgresql_crud::JsonGeneric::<Something> = postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element();
    println!("{default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element:#?}");
    let serialized = serde_json::to_string(&default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element).unwrap();
    println!("{serialized:#?}");
    let deserialized: postgresql_crud::JsonGeneric::<Something> = serde_json::from_str(&serialized).unwrap();
    println!("{deserialized:#?}");
}