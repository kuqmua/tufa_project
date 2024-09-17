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
    pub sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::SqlxTypesJsonAsPostgresqlJsonBNotNull::<Something>,//todo

    // pub serde_json_value_as_postgresql_json: postgresql_crud::SerdeJsonValueAsPostgresqlJson,
    // pub serde_json_value_as_postgresql_json_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonNotNull,
    // pub serde_json_value_as_postgresql_json_b: postgresql_crud::SerdeJsonValueAsPostgresqlJsonB,
    // pub serde_json_value_as_postgresql_json_b_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonBNotNull,
}

//todo enum tree support
//todo generate wrapper type for all possible json type
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,
    //  postgresql_crud::GeneratePostgresqlQueryPart
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

    // pub generic: postgresql_crud::JsonGeneric<Rat>,
    // pub std_option_option_generic: postgresql_crud::JsonStdOptionOptionGeneric<Doggie>,

    pub std_vec_vec_generic_with_id: postgresql_crud::JsonStdVecVecGenericWithId<Doggie>,
    pub std_option_option_std_vec_vec_generic_with_id: postgresql_crud::JsonStdOptionOptionStdVecVecGenericWithId<Doggie>,
    // pub std_vec_vec_std_option_option_generic_with_id: postgresql_crud::JsonStdVecVecStdOptionOptionGenericWithId<Doggie>,
    // pub std_option_option_std_vec_vec_std_option_option_generic_with_id: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionGenericWithId<Doggie>,
}

//todo this need for old version of update_many. later need to refactor update many and remove this
impl<'a> postgresql_crud::BindQuery<'a> for Something {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        //implementation not necessary
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
            }
            None => {
                return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        }
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
            }
            None => {
                return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        }
        // Ok(())
        todo!()
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        let mut increments = std::string::String::default();//'id', to_jsonb(gen_random_uuid()),
        // // pub id: postgresql_crud::JsonUuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids
        // pub std_primitive_i8: postgresql_crud::JsonStdPrimitiveI8,
        // pub std_vec_vec_generic: postgresql_crud::JsonStdVecVecGenericWithId<Doggie>,
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                increments.push_str(&format!("'std_primitive_i8',${increment},"));
            }
            None => {
                return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        }
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                increments.push_str(&format!("'std_vec_vec_generic',${increment},"));
            }
            None => {
                return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        }
        let _ = increments.pop();
        Ok(format!("jsonb_build_object({increments})"))
    }
    fn bind_value_to_query(self, 
        // mut 
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> 
    {
        // query = query.bind(sqlx::types::Json(self.std_primitive_i8.0));
        // query = query.bind(sqlx::types::Json(self.std_vec_vec_generic_with_id.0));
        // query
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,
    //  postgresql_crud::GeneratePostgresqlQueryPart
)] //user type must implement utoipa::ToSchema trait
pub struct Doggie {
    pub id: postgresql_crud::JsonUuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    pub std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16,
    // pub generic: postgresql_crud::JsonGeneric<Cat>,
}

// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,
//      postgresql_crud::GeneratePostgresqlQueryPart
// )] //user type must implement utoipa::ToSchema trait
// pub struct Rat {
//     pub std_primitive_i16: postgresql_crud::JsonStdPrimitiveI64,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,
//     postgresql_crud::GeneratePostgresqlQueryPart
// )] //user type must implement utoipa::ToSchema trait
// pub struct Cat {
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
        SomethingOptionToUpdate::StdPrimitiveI8(postgresql_crud::Value {
            value: std::primitive::i8::default()
        }),
        SomethingOptionToUpdate::StdVecVecGeneric(postgresql_crud::Value {
            value: vec![
                DoggieOptionsToUpdate(postgresql_crud::JsonArrayElementChange::Create(DoggieToCreate {
                    std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16::default(),
                })),

                DoggieOptionsToUpdate(postgresql_crud::JsonArrayElementChange::Delete(postgresql_crud::JsonUuid(uuid::uuid!("d6a4aa72-b154-4699-889f-33ef34a8c7f2")))),
                //
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct DoggieOptionsToUpdateSSS {
//     id: uuid::Uuid,
//     fields: std::vec::Vec<DoggieOptionToUpdate>,
// }

// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct DoggieOptionsToUpdate(postgresql_crud::JsonArrayElementChange<DoggieToCreate, DoggieOptionsToUpdateSSS>);
                //
            ]
        }),
    ]);
    println!("{f:#?}");
    let serialized = serde_json::to_string(&f).unwrap();
    println!("{serialized:#?}");
}

/////////start block code for trying implement partial
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64,
    pub sqlx_types_json_t_as_postgresql_json_b_not_null:
        std::option::Option<Field<postgresql_crud::SqlxTypesJson<SomethingOptionsToUpdate>>>,//here 
}
#[derive(Debug)]
pub struct UpdateOneParameters {
    pub payload: UpdateOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateOneRouteLogicResponseVariants {
    Desirable(postgresql_crud::StdPrimitiveI64),
    CheckBodySize {
        check_body_size:
            postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
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
impl std::convert::From<TryUpdateOneRouteLogicErrorNamed>
    for TryUpdateOneRouteLogicResponseVariants
{
    fn from(value: TryUpdateOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            SerdeJson { serde_json, code_occurence } => Self :: SerdeJson
            { serde_json, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            NoPayloadFieldsPrimaryKey
            { no_payload_fields_primary_key, code_occurence } => Self ::
            NoPayloadFieldsPrimaryKey
            { no_payload_fields_primary_key, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            RowAndRollback { row, rollback, code_occurence } => Self ::
            RowAndRollback { row, rollback, code_occurence }
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
pub async fn try_update_one_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits >,
    request: axum::extract::Request,
) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(
        body,
        *app_state.get_maximum_size_of_http_body_in_bytes(),
    )
    .await
    {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2577,
                        column: 17,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateOneRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = UpdateOneParameters {
        payload: match serde_json::from_slice::<UpdateOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = UpdateOnePayload::from(value);
                if let None = &value.sqlx_types_json_t_as_postgresql_json_b_not_null {
                    let error_0 =
                        value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;
                    let error = TryUpdateOneRouteLogicErrorNamed::NoPayloadFieldsPrimaryKey {
                        no_payload_fields_primary_key: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 2059,
                                column: 13,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryUpdateOneRouteLogicResponseVariants::from(error),
                    ));
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
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2668,
                            column: 13,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateOneRouteLogicResponseVariants::from(error),
                ));
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
            //             Generic(
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
            //     "generic": {
            //       "std_string_string": "sdsd"
            //     },
            //     "std_primitive_i8": 0
            // }
            

            query.push_str("sqlx_types_json_t_as_postgresql_json_b_not_null = ");
            match postgresql_crud::GeneratePostgresqlQueryPartToUpdate::try_generate_bind_increments(
                &value.value.0.0,
                "sqlx_types_json_t_as_postgresql_json_b_not_null",
                "sqlx_types_json_t_as_postgresql_json_b_not_null",//sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_generic'->0->'generic'
                "",//{std_vec_vec_generic,0}
                &mut increment,
                postgresql_crud::ArrayObjectElementOrSimple::Simple,
            ) {
                Ok(value) => {
                    query.push_str(&value);
                },
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
            //             '{generic,std_string_string}', 
            //             '"something"'
            //         ), 
            //         '{std_option_option_generic}', 
            //         -- jsonb_build_object('std_option_option_generic', 'yeees')
            //         'null'
            //     )
            // WHERE std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14
            // RETURNING std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;


        }
        let _ = query.pop();
        match postgresql_crud::BindQuery::try_increment(
            &parameters
                .payload
                .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            &mut increment,
        ) {
            Ok(_) => {
                query.push_str(& format!
                (" where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = ${increment}"));
            }
            Err(error_0) => {
                let error = TryUpdateOneRouteLogicErrorNamed::BindQuery {
                    bind_query: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 4822,
                            column: 25,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
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
        if let Some(value) = parameters.payload.sqlx_types_json_t_as_postgresql_json_b_not_null {
            query = postgresql_crud::GeneratePostgresqlQueryPartToUpdate::bind_value_to_query(value.value.0.0, query);
            // for element in value.value.0.0.0 {
            //     match element {
            //         SomethingOptionToUpdate::StdPrimitiveI8(value) => {
            //             query = query.bind(sqlx::types::Json(value.value));//sqlx::types::Json
            //         }
            //         SomethingOptionToUpdate::Generic(value) => {
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
            // query = postgresql_crud::BindQuery::bind_value_to_query(sqlx::types::Json(postgresql_crud::JsonStdStringString(std::string::String::new("cat"))), query);
            // query = postgresql_crud::BindQuery::bind_value_to_query(sqlx::types::Json(postgresql_crud::JsonStdPrimitiveI8(42)), query);
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
        Err(error_0) => {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2609,
                        column: 17,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateOneRouteLogicResponseVariants::from(error),
            ));
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
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2609,
                        column: 17,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateOneRouteLogicResponseVariants::from(error),
            ));
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
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2289,
                            column: 17,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => {
                    match sqlx::Row::try_get::<std::primitive::i64, &std::primitive::str>(
                        &value,
                        "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                    ) {
                        Ok(value) => postgresql_crud::StdPrimitiveI64(value),
                        Err(error_0) => match executor.rollback().await {
                            Ok(_) => {
                                let error = TryUpdateOneRouteLogicErrorNamed :: Postgresql
                                    {
                                        postgresql : error_0, code_occurence : error_occurence_lib
                                        :: code_occurence :: CodeOccurence ::
                                        new(file! ().to_owned(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 3058, column : 17,
                                        }))
                                    };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() =
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            Err(error_1) => {
                                let error = TryUpdateOneRouteLogicErrorNamed ::
                                    RowAndRollback
                                    {
                                        row : error_0, rollback : error_1, code_occurence :
                                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                                        new(file! ().to_owned(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 3061, column : 17,
                                        }))
                                    };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() =
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        },
                    }
                }
                Err(error_0) => {
                    match executor.rollback().await {
                        Ok(_) => {
                            let error = TryUpdateOneRouteLogicErrorNamed :: Postgresql
                            {
                                postgresql : error_0, code_occurence : error_occurence_lib
                                :: code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 3068, column : 13,
                                }))
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryUpdateOneRouteLogicErrorNamed ::
                            RowAndRollback
                            {
                                row : error_0, rollback : error_1, code_occurence :
                                error_occurence_lib :: code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 3071, column : 13,
                                }))
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
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
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2307,
                        column: 17,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateOneRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryUpdateOneRouteLogicResponseVariants::Desirable(value),
    ));
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
        try_update_one_route_logic_error_named_with_serialize_deserialize:
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_one(
    server_location: &std::primitive::str,
    parameters: UpdateOneParameters,
) -> Result<postgresql_crud::StdPrimitiveI64, TryUpdateOneErrorNamed> {
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
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2711,
                            column: 17,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/jsongeneric/update_one", server_location,);
    let future = reqwest::Client::new()
        .patch(&url)
        .header(
            &postgresql_crud::CommitSnakeCase.to_string(),
            git_info::PROJECT_GIT_INFO.commit,
        )
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
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2774,
                        column: 17,
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
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2796,
                        column: 17,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryUpdateOneRouteLogicResponseVariants>(&error_2) {
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
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2814,
                            column: 17,
                        }),
                    ),
                });
            }
        };
    let try_update_one_route_logic_error_named_with_serialize_deserialize = match expected_response
    {
        TryUpdateOneRouteLogicResponseVariants::Desirable(value) => {
            let value = postgresql_crud::StdPrimitiveI64::from(value);
            return Ok(value);
        }
        TryUpdateOneRouteLogicResponseVariants::CheckBodySize {
            check_body_size,
            code_occurence,
        } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize {
            check_body_size,
            code_occurence,
        },
        TryUpdateOneRouteLogicResponseVariants::Postgresql {
            postgresql,
            code_occurence,
        } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql {
            postgresql,
            code_occurence,
        },
        TryUpdateOneRouteLogicResponseVariants::SerdeJson {
            serde_json,
            code_occurence,
        } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson {
            serde_json,
            code_occurence,
        },
        TryUpdateOneRouteLogicResponseVariants::CheckCommit {
            check_commit,
            code_occurence,
        } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit {
            check_commit,
            code_occurence,
        },
        TryUpdateOneRouteLogicResponseVariants::BindQuery {
            bind_query,
            code_occurence,
        } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::BindQuery {
            bind_query,
            code_occurence,
        },
        TryUpdateOneRouteLogicResponseVariants::NoPayloadFieldsPrimaryKey {
            no_payload_fields_primary_key,
            code_occurence,
        } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFieldsPrimaryKey {
            no_payload_fields_primary_key,
            code_occurence,
        },
        TryUpdateOneRouteLogicResponseVariants::RowAndRollback {
            row,
            rollback,
            code_occurence,
        } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback {
            row,
            rollback,
            code_occurence,
        },
    };
    Err(
        TryUpdateOneErrorNamed::TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize {
            try_update_one_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2857,
                    column: 17,
                }),
            ),
        },
    )
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

//////////
// impl std::fmt::Display for Something {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde :: Serialize,
//     serde :: Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
// pub enum SomethingFieldToRead {
//     #[serde(rename(
//         serialize = "std_vec_vec_generic_with_id",
//         deserialize = "std_vec_vec_generic_with_id"
//     ))]
//     StdVecVecGenericWithId {
//         field_vec: std::vec::Vec<DoggieFieldToRead>,
//         limit: std::primitive::u64,
//         offset: std::primitive::u64,
//     },
//     #[serde(rename(
//         serialize = "std_option_option_std_vec_vec_generic_with_id",
//         deserialize = "std_option_option_std_vec_vec_generic_with_id"
//     ))]
//     StdOptionOptionStdVecVecGenericWithId {
//         field_vec: std::vec::Vec<DoggieFieldToRead>,
//         limit: std::primitive::u64,
//         offset: std::primitive::u64,
//     },
// }
// impl error_occurence_lib::ToStdStringString for SomethingFieldToRead {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// #[derive(
//     Debug,
//     serde :: Serialize,
//     serde :: Deserialize,
//     thiserror :: Error,
//     error_occurence_lib :: ErrorOccurence,
// )]
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
// impl error_occurence_lib::ToStdStringString
//     for SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
// {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// #[derive(
//     Debug,
//     serde :: Serialize,
//     serde :: Deserialize,
//     thiserror :: Error,
//     error_occurence_lib :: ErrorOccurence,
// )]
// pub enum SomethingGeneratePostgresqlQueryPartToReadErrorNamed {
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
//     DoggieGeneratePostgresqlQueryPartToReadFromSelfVec {
//         #[eo_error_occurence]
//         field: DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl
//     postgresql_crud::GeneratePostgresqlQueryPartToRead<
//         SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed,
//         SomethingGeneratePostgresqlQueryPartToReadErrorNamed,
//     > for SomethingFieldToRead
// {
//     fn generate_postgresql_query_part_to_read_from_self_vec(
//         value: &std::vec::Vec<Self>,
//         column_name_and_maybe_field_getter: &std::primitive::str,
//         column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
//         is_optional: std::primitive::bool,
//     ) -> Result<std::string::String, SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed>
//     {
//         if value.is_empty() {
//             return
//             Err(SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
//             :: FieldsFilterIsEmpty
//             { code_occurence : error_occurence_lib :: code_occurence! (), });
//         }
//         let mut unique = vec![];
//         for element in value {
//             if unique.contains(&element) {
//                 return
//                 Err(SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
//                 :: NotUniqueFieldFilter
//                 {
//                     field : element.clone(), code_occurence :
//                     error_occurence_lib :: code_occurence! (),
//                 });
//             } else {
//                 unique.push(&element);
//             }
//         }
//         let mut acc = std::string::String::default();
//         for element in value {
//             match element.generate_postgresql_query_part_to_read(
//                 column_name_and_maybe_field_getter,
//                 column_name_and_maybe_field_getter_for_error_message,
//             ) {
//                 Ok(value) => {
//                     acc.push_str(&format!("{value}||"));
//                 }
//                 Err(error) => {
//                     return
//                     Err(SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
//                     :: GeneratePostgresqlQueryPart
//                     {
//                         error, code_occurence : error_occurence_lib ::
//                         code_occurence! (),
//                     });
//                 }
//             }
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         let is_optional_query_part =
//         match is_optional
//         {
//             true => format!
//             ("when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null)"),
//             false => std :: string :: String :: default()
//         };
//         Ok({
//             let space_and_not_null = if is_optional { " and not null" } else { "" };
//             format!
//             ("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}){is_optional_query_part} else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object{space_and_not_null}')) end")
//         })
//     }
//     fn generate_postgresql_query_part_to_read(
//         &self,
//         column_name_and_maybe_field_getter: &std::primitive::str,
//         column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
//     ) -> Result<std::string::String, SomethingGeneratePostgresqlQueryPartToReadErrorNamed> {
//         match self
//         {
//             Self :: StdVecVecGenericWithId { field_vec, limit, offset } =>
//             match postgresql_crud :: GeneratePostgresqlQueryPartToRead ::
//             generate_postgresql_query_part_to_read_from_self_vec(field_vec, &
//             format! ("value"), & format!
//             ("{column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_generic_with_id[array element]"),
//             false)
//             {
//                 Ok(value) =>
//                 {
//                     let start = offset; let end = match
//                     offset.checked_add(* limit)
//                     {
//                         Some(value) => value, None =>
//                         {
//                             return
//                             Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed ::
//                             StdVecVecGenericWithIdOffsetPlusLimitIsIntOverflow
//                             {
//                                 limit : * limit, offset : * offset, code_occurence :
//                                 error_occurence_lib :: code_occurence! (),
//                             });
//                         }
//                     };
//                     Ok(format!
//                     ("jsonb_build_object('std_vec_vec_generic_with_id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_vec_vec_generic_with_id') = 'array' then jsonb_build_object('Ok',(select jsonb_agg({value}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_vec_vec_generic_with_id')) with ordinality where ordinality between {start} and {end})) else jsonb_build_object(jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}} is not array')) end)"))
//                 }, Err(error) =>
//                 {
//                     return
//                     Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed ::
//                     DoggieGeneratePostgresqlQueryPartToReadFromSelfVec
//                     {
//                         field : error, code_occurence : error_occurence_lib ::
//                         code_occurence! (),
//                     });
//                 }
//             }, Self :: StdOptionOptionStdVecVecGenericWithId
//             { field_vec, limit, offset } => match postgresql_crud ::
//             GeneratePostgresqlQueryPartToRead ::
//             generate_postgresql_query_part_to_read_from_self_vec(field_vec, &
//             format! ("value"), & format!
//             ("{column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_generic_with_id[array element]"),
//             false)
//             {
//                 Ok(value) =>
//                 {
//                     let start = offset; let end = match
//                     offset.checked_add(* limit)
//                     {
//                         Some(value) => value, None =>
//                         {
//                             return
//                             Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed ::
//                             StdOptionOptionStdVecVecGenericWithIdOffsetPlusLimitIsIntOverflow
//                             {
//                                 limit : * limit, offset : * offset, code_occurence :
//                                 error_occurence_lib :: code_occurence! (),
//                             });
//                         }
//                     };
//                     Ok(format!
//                     ("jsonb_build_object('std_option_option_std_vec_vec_generic_with_id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic_with_id') = 'array' then jsonb_build_object('Ok',(select jsonb_agg({value}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic_with_id')) with ordinality where ordinality between {start} and {end}))when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic_with_id') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object(jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}} is not array and not null')) end)"))
//                 }, Err(error) =>
//                 {
//                     return
//                     Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed ::
//                     DoggieGeneratePostgresqlQueryPartToReadFromSelfVec
//                     {
//                         field : error, code_occurence : error_occurence_lib ::
//                         code_occurence! (),
//                     });
//                 }
//             }
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
// pub struct SomethingOptionsToRead {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_vec_vec_generic_with_id:
//         std::option::Option<postgresql_crud::Value<std::vec::Vec<DoggieOptionsToRead>>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     std_option_option_std_vec_vec_generic_with_id: std::option::Option<
//         postgresql_crud::Value<std::option::Option<std::vec::Vec<DoggieOptionsToRead>>>,
//     >,
// }
// impl std::convert::From<Something> for SomethingOptionsToRead {
//     fn from(value: Something) -> Self {
//         Self {
//             std_vec_vec_generic_with_id: Some(postgresql_crud::Value {
//                 value: value
//                     .std_vec_vec_generic_with_id
//                     .0
//                     .into_iter()
//                     .map(|element| DoggieOptionsToRead::from(element))
//                     .collect::<std::vec::Vec<DoggieOptionsToRead>>(),
//             }),
//             std_option_option_std_vec_vec_generic_with_id: Some(postgresql_crud::Value {
//                 value: match value.std_option_option_std_vec_vec_generic_with_id.0 {
//                     Some(value) => Some(
//                         value
//                             .into_iter()
//                             .map(|element| DoggieOptionsToRead::from(element))
//                             .collect::<std::vec::Vec<DoggieOptionsToRead>>(),
//                     ),
//                     None => None,
//                 },
//             }),
//         }
//     }
// }
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
//             __ignore,
//         }
//         #[doc(hidden)]
//         struct __FieldVisitor;
//         impl serde::de::Visitor<'_> for __FieldVisitor {
//             type Value = __Field;
//             fn expecting(
//                 &self,
//                 __formatter: &mut serde::__private::Formatter<'_>,
//             ) -> serde::__private::fmt::Result {
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
//                     "std_vec_vec_generic_with_id" => serde::__private::Ok(__Field::__field0),
//                     "std_option_option_std_vec_vec_generic_with_id" => {
//                         serde::__private::Ok(__Field::__field1)
//                     }
//                     _ => serde::__private::Ok(__Field::__ignore),
//                 }
//             }
//             fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
//             where
//                 __E: serde::de::Error,
//             {
//                 match __value {
//                     b"std_vec_vec_generic_with_id" => serde::__private::Ok(__Field::__field0),
//                     b"std_option_option_std_vec_vec_generic_with_id" => {
//                         serde::__private::Ok(__Field::__field1)
//                     }
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
//             fn expecting(
//                 &self,
//                 __formatter: &mut serde::__private::Formatter<'_>,
//             ) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "struct SomethingOptionsToRead")
//             }
//             #[inline]
//             fn visit_seq<__A>(
//                 self,
//                 mut __seq: __A,
//             ) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<
//                     std::option::Option<
//                         std::result::Result<
//                             std::vec::Vec<
//                                 std::result::Result<DoggieOptionsToRead, std::string::String>,
//                             >,
//                             std::string::String,
//                         >,
//                     >,
//                 >(&mut __seq)?
//                 {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(
//                             0usize,
//                             &"struct SomethingOptionsToRead with 2 elements",
//                         ));
//                     }
//                 };
//                 let __field1 = match serde::de::SeqAccess::next_element::<
//                     std::option::Option<
//                         std::result::Result<
//                             std::option::Option<
//                                 std::vec::Vec<
//                                     std::result::Result<DoggieOptionsToRead, std::string::String>,
//                                 >,
//                             >,
//                             std::string::String,
//                         >,
//                     >,
//                 >(&mut __seq)?
//                 {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(
//                             1usize,
//                             &"struct SomethingOptionsToRead with 2 elements",
//                         ));
//                     }
//                 };
//                 serde::__private::Ok(SomethingOptionsToRead {
//                     std_vec_vec_generic_with_id: match __field0 {
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
//                     std_option_option_std_vec_vec_generic_with_id: match __field1 {
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
//             fn visit_map<__A>(
//                 self,
//                 mut __map: __A,
//             ) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::MapAccess<'de>,
//             {
//                 let mut __field0: serde::__private::Option<
//                     std::option::Option<
//                         std::result::Result<
//                             std::vec::Vec<
//                                 std::result::Result<DoggieOptionsToRead, std::string::String>,
//                             >,
//                             std::string::String,
//                         >,
//                     >,
//                 > = serde::__private::None;
//                 let mut __field1: serde::__private::Option<
//                     std::option::Option<
//                         std::result::Result<
//                             std::option::Option<
//                                 std::vec::Vec<
//                                     std::result::Result<DoggieOptionsToRead, std::string::String>,
//                                 >,
//                             >,
//                             std::string::String,
//                         >,
//                     >,
//                 > = serde::__private::None;
//                 while let serde::__private::Some(__key) =
//                     serde::de::MapAccess::next_key::<__Field>(&mut __map)?
//                 {
//                     match __key {
//                         __Field::__field0 => {
//                             if serde::__private::Option::is_some(&__field0) {
//                                 return serde::__private::Err(
//                                     <__A::Error as serde::de::Error>::duplicate_field(
//                                         "std_vec_vec_generic_with_id",
//                                     ),
//                                 );
//                             }
//                             __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<
//                                 std::option::Option<
//                                     std::result::Result<
//                                         std::vec::Vec<
//                                             std::result::Result<
//                                                 DoggieOptionsToRead,
//                                                 std::string::String,
//                                             >,
//                                         >,
//                                         std::string::String,
//                                     >,
//                                 >,
//                             >(
//                                 &mut __map
//                             )?);
//                         }
//                         __Field::__field1 => {
//                             if serde::__private::Option::is_some(&__field1) {
//                                 return serde::__private::Err(
//                                     <__A::Error as serde::de::Error>::duplicate_field(
//                                         "std_option_option_std_vec_vec_generic_with_id",
//                                     ),
//                                 );
//                             }
//                             __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<
//                                 std::option::Option<
//                                     std::result::Result<
//                                         std::option::Option<
//                                             std::vec::Vec<
//                                                 std::result::Result<
//                                                     DoggieOptionsToRead,
//                                                     std::string::String,
//                                                 >,
//                                             >,
//                                         >,
//                                         std::string::String,
//                                     >,
//                                 >,
//                             >(
//                                 &mut __map
//                             )?);
//                         }
//                         _ => {
//                             let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(
//                                 &mut __map,
//                             )?;
//                         }
//                     }
//                 }
//                 let __field0 = match __field0 {
//                     serde::__private::Some(__field0) => __field0,
//                     serde::__private::None => {
//                         serde::__private::de::missing_field("std_vec_vec_generic_with_id")?
//                     }
//                 };
//                 let __field1 = match __field1 {
//                     serde::__private::Some(__field1) => __field1,
//                     serde::__private::None => serde::__private::de::missing_field(
//                         "std_option_option_std_vec_vec_generic_with_id",
//                     )?,
//                 };
//                 serde::__private::Ok(SomethingOptionsToRead {
//                     std_vec_vec_generic_with_id: match __field0 {
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
//                     std_option_option_std_vec_vec_generic_with_id: match __field1 {
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
//         const FIELDS: &'static [&'static str] = &[
//             "std_vec_vec_generic_with_id",
//             "std_option_option_std_vec_vec_generic_with_id",
//         ];
//         serde::Deserializer::deserialize_struct(
//             __deserializer,
//             "SomethingOptionsToRead",
//             FIELDS,
//             __Visitor {
//                 marker: serde::__private::PhantomData::<SomethingOptionsToRead>,
//                 lifetime: serde::__private::PhantomData,
//             },
//         )
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
//             fn expecting(
//                 &self,
//                 __formatter: &mut serde::__private::Formatter<'_>,
//             ) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "tuple struct SomethingReader")
//             }
//             #[inline]
//             fn visit_newtype_struct<__E>(
//                 self,
//                 __e: __E,
//             ) -> serde::__private::Result<Self::Value, __E::Error>
//             where
//                 __E: serde::Deserializer<'de>,
//             {
//                 let __field0: Result<SomethingOptionsToRead, std::string::String> = <Result<
//                     SomethingOptionsToRead,
//                     std::string::String,
//                 > as serde::Deserialize>::deserialize(
//                     __e
//                 )?;
//                 serde::__private::Ok(SomethingReader(match __field0 {
//                     Ok(value) => value,
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(error));
//                     }
//                 }))
//             }
//             #[inline]
//             fn visit_seq<__A>(
//                 self,
//                 mut __seq: __A,
//             ) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<
//                     Result<SomethingOptionsToRead, std::string::String>,
//                 >(&mut __seq)?
//                 {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(
//                             0usize,
//                             &"tuple struct SomethingReader with 1 element",
//                         ));
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
//         serde::Deserializer::deserialize_newtype_struct(
//             __deserializer,
//             "SomethingReader",
//             __Visitor {
//                 marker: serde::__private::PhantomData::<SomethingReader>,
//                 lifetime: serde::__private::PhantomData,
//             },
//         )
//     }
// }
// impl postgresql_crud ::
// StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
// for Something
// {
//     #[inline] fn
//     default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
//     -> Self
//     {
//         Self
//         {
//             std_vec_vec_generic_with_id : postgresql_crud ::
//             StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
//             ::
//             default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             std_option_option_std_vec_vec_generic_with_id : postgresql_crud ::
//             StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
//             ::
//             default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
//         }
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde :: Serialize,
//     serde :: Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
// pub enum SomethingFieldToUpdate {
//     #[serde(rename(
//         serialize = "std_vec_vec_generic_with_id",
//         deserialize = "std_vec_vec_generic_with_id"
//     ))]
//     StdVecVecGenericWithId,
//     #[serde(rename(
//         serialize = "std_option_option_std_vec_vec_generic_with_id",
//         deserialize = "std_option_option_std_vec_vec_generic_with_id"
//     ))]
//     StdOptionOptionStdVecVecGenericWithId,
// }
// impl error_occurence_lib::ToStdStringString for SomethingFieldToUpdate {
//     fn to_std_string_string(&self) -> std::string::String {
//         match &self {
//             Self::StdVecVecGenericWithId => "std_vec_vec_generic_with_id".to_owned(),
//             Self::StdOptionOptionStdVecVecGenericWithId => {
//                 "std_option_option_std_vec_vec_generic_with_id".to_owned()
//             }
//         }
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub enum SomethingOptionToUpdate {
//     #[serde(rename(
//         serialize = "std_vec_vec_generic_with_id",
//         deserialize = "std_vec_vec_generic_with_id"
//     ))]
//     StdVecVecGenericWithId(postgresql_crud::Value<std::vec::Vec<DoggieJsonArrayElementChange>>),
//     #[serde(rename(
//         serialize = "std_option_option_std_vec_vec_generic_with_id",
//         deserialize = "std_option_option_std_vec_vec_generic_with_id"
//     ))]
//     StdOptionOptionStdVecVecGenericWithId(
//         postgresql_crud::Value<std::vec::Vec<std::option::Option<DoggieJsonArrayElementChange>>>,
//     ),
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct SomethingOptionsToUpdate(pub std::vec::Vec<SomethingOptionToUpdate>);
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
//     StdVecVecGenericWithIdDoggieNotUniqueId {
//         #[eo_to_std_string_string_serialize_deserialize]
//         std_vec_vec_generic_with_id_doggie_not_unique_id: postgresql_crud::JsonUuid,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements {
//         #[eo_error_occurence]
//         std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments:
//             DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements {
//         #[eo_error_occurence]
//         std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments:
//             postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements {
//         #[eo_error_occurence]
//         std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments:
//             postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdOptionOptionStdVecVecGenericWithIdDoggieNotUniqueId {
//         #[eo_to_std_string_string_serialize_deserialize]
//         std_option_option_std_vec_vec_generic_with_id_doggie_not_unique_id:
//             postgresql_crud::JsonUuid,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements {
//         #[eo_error_occurence]
//         std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments:
//             DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements {
//         #[eo_error_occurence]
//         std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments:
//             postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements {
//         #[eo_error_occurence]
//         std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments:
//             postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl
//     postgresql_crud::GeneratePostgresqlQueryPartToUpdate<
//         SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed,
//     > for SomethingOptionsToUpdate
// {
    // fn try_generate_bind_increments(
    //     &self,
    //     jsonb_set_accumulator: &std::primitive::str,
    //     jsonb_set_target: &std::primitive::str,
    //     jsonb_set_path: &std::primitive::str,
    //     increment: &mut std::primitive::u64,
    //     is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple,
    // ) -> Result<std::string::String, SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed>
    // {
    //     if self.0.is_empty() {
    //         return Err(
    //             SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::FieldsIsEmpty {
    //                 code_occurence: error_occurence_lib::code_occurence!(),
    //             },
    //         );
    //     }
    //     {
    //         let mut acc = vec![];
    //         for element in &self.0 {
    //             match element {
    //                 SomethingOptionToUpdate::StdVecVecGenericWithId(_) => {
    //                     let value = SomethingFieldToUpdate::StdVecVecGenericWithId;
    //                     if acc.contains(&value) {
    //                         return
    //                         Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
    //                         :: NotUniqueField
    //                         {
    //                             field : value, code_occurence : error_occurence_lib ::
    //                             code_occurence! (),
    //                         },);
    //                     } else {
    //                         acc.push(value);
    //                     }
    //                 }
    //                 SomethingOptionToUpdate::StdOptionOptionStdVecVecGenericWithId(_) => {
    //                     let value = SomethingFieldToUpdate::StdOptionOptionStdVecVecGenericWithId;
    //                     if acc.contains(&value) {
    //                         return
    //                         Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
    //                         :: NotUniqueField
    //                         {
    //                             field : value, code_occurence : error_occurence_lib ::
    //                             code_occurence! (),
    //                         },);
    //                     } else {
    //                         acc.push(value);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     let mut acc = std::string::String::from(jsonb_set_accumulator);
    //     let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
    //         true => std::string::String::default(),
    //         false => format!("{jsonb_set_path},"),
    //     };
    //     for element in &self.0 {
    //         match &element {
    //             SomethingOptionToUpdate::StdVecVecGenericWithId(value) => {
    //                 {
    //                     let mut ids: std::vec::Vec<&postgresql_crud::JsonUuid> = vec![];
    //                     for element in &value.value {
    //                         match &element.0 {
    //                             postgresql_crud::JsonArrayElementChange::Update(value) => {
    //                                 if ids.contains(&&value.id) {
    //                                     return
    //                                     Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
    //                                     :: StdVecVecGenericWithIdDoggieNotUniqueId
    //                                     {
    //                                         std_vec_vec_generic_with_id_doggie_not_unique_id : value.id,
    //                                         code_occurence : error_occurence_lib :: code_occurence! (),
    //                                     });
    //                                 } else {
    //                                     ids.push(&value.id);
    //                                 }
    //                             }
    //                             postgresql_crud::JsonArrayElementChange::Delete(value) => {
    //                                 if ids.contains(&value) {
    //                                     return
    //                                     Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
    //                                     :: StdVecVecGenericWithIdDoggieNotUniqueId
    //                                     {
    //                                         std_vec_vec_generic_with_id_doggie_not_unique_id : * value,
    //                                         code_occurence : error_occurence_lib :: code_occurence! (),
    //                                     });
    //                                 } else {
    //                                     ids.push(&value);
    //                                 }
    //                             }
    //                             _ => (),
    //                         }
    //                     }
    //                 }
    //                 let current_jsonb_set_target =
    //                     format!("{jsonb_set_target}->'std_vec_vec_generic_with_id'");
    //                 let mut update_query_part_acc = std::string::String::default();
    //                 for (index, element) in &value
    //                     .value
    //                     .iter()
    //                     .enumerate()
    //                     .collect::<std::vec::Vec<(usize, &DoggieJsonArrayElementChange)>>()
    //                 {
    //                     match postgresql_crud :: JsonArrayElementBindQuery ::
    //                     try_generate_update_bind_increments(* element, &
    //                     jsonb_set_accumulator, & jsonb_set_target, & jsonb_set_path,
    //                     increment, is_array_object_element.clone(),)
    //                     {
    //                         Ok(value) =>
    //                         {
    //                             if let Some(value) = value
    //                             { update_query_part_acc.push_str(& value); }
    //                         }, Err(error) =>
    //                         {
    //                             return
    //                             Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
    //                             ::
    //                             StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements
    //                             {
    //                                 std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments
    //                                 : error, code_occurence : error_occurence_lib ::
    //                                 code_occurence! (),
    //                             });
    //                         }
    //                     }
    //                 }
    //                 let mut delete_query_part_acc = std::string::String::default();
    //                 for (index, element) in &value
    //                     .value
    //                     .iter()
    //                     .enumerate()
    //                     .collect::<std::vec::Vec<(usize, &DoggieJsonArrayElementChange)>>()
    //                 {
    //                     match postgresql_crud :: JsonArrayElementBindQuery ::
    //                     try_generate_delete_bind_increments(* element, increment)
    //                     {
    //                         Ok(value) =>
    //                         {
    //                             if let Some(value) = value
    //                             {
    //                                 let maybe_space_and_space = if
    //                                 delete_query_part_acc.is_empty() { "" } else { " and " };
    //                                 delete_query_part_acc.push_str(& format!
    //                                 ("{value}{maybe_space_and_space}"));
    //                             }
    //                         }, Err(error) =>
    //                         {
    //                             return
    //                             Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
    //                             ::
    //                             StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements
    //                             {
    //                                 std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments
    //                                 : error, code_occurence : error_occurence_lib ::
    //                                 code_occurence! (),
    //                             });
    //                         }
    //                     }
    //                 }
    //                 let mut create_query_part_acc = std::string::String::default();
    //                 for (index, element) in &value
    //                     .value
    //                     .iter()
    //                     .enumerate()
    //                     .collect::<std::vec::Vec<(usize, &DoggieJsonArrayElementChange)>>()
    //                 {
    //                     match postgresql_crud :: JsonArrayElementBindQuery ::
    //                     try_generate_create_bind_increments(* element, increment)
    //                     {
    //                         Ok(value) =>
    //                         {
    //                             if let Some(value) = value
    //                             { create_query_part_acc.push_str(& format! ("{value},")); }
    //                         }, Err(error) =>
    //                         {
    //                             return
    //                             Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
    //                             ::
    //                             StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements
    //                             {
    //                                 std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments
    //                                 : error, code_occurence : error_occurence_lib ::
    //                                 code_occurence! (),
    //                             });
    //                         }
    //                     }
    //                 }
    //                 let _ = create_query_part_acc.pop();
    //                 let maybe_jsonb_agg_case = if update_query_part_acc.is_empty() {
    //                     std::string::String::from("elem")
    //                 } else {
    //                     format!("case {update_query_part_acc} else elem end")
    //                 };
    //                 let maybe_where = if delete_query_part_acc.is_empty() {
    //                     std::string::String::default()
    //                 } else {
    //                     format!(" where {delete_query_part_acc}")
    //                 };
    //                 let maybe_jsonb_build_array = if create_query_part_acc.is_empty() {
    //                     std::string::String::default()
    //                 } else {
    //                     format!(" || jsonb_build_array({create_query_part_acc})")
    //                 };
    //                 acc = format!
    //                 ("jsonb_set({acc},'{{{previous_jsonb_set_path}std_vec_vec_generic_with_id}}',(select jsonb_agg({maybe_jsonb_agg_case}) from jsonb_array_elements({current_jsonb_set_target}) as elem {maybe_where}){maybe_jsonb_build_array})");
    //             }
    //             SomethingOptionToUpdate::StdOptionOptionStdVecVecGenericWithId(value) => {
    //                 {
    //                     let mut ids: std::vec::Vec<&postgresql_crud::JsonUuid> = vec![];
    //                     for element in &value.value {
    //                         //here
    //                         if let Some(value) = element {
    //                             match &value.0 {
    //                                 postgresql_crud::JsonArrayElementChange::Update(value) => {
    //                                     if ids.contains(&&value.id) {
    //                                         return
    //                                         Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
    //                                         :: StdOptionOptionStdVecVecGenericWithIdDoggieNotUniqueId
    //                                         {
    //                                             std_option_option_std_vec_vec_generic_with_id_doggie_not_unique_id
    //                                             : value.id, code_occurence : error_occurence_lib ::
    //                                             code_occurence! (),
    //                                         });
    //                                     } else {
    //                                         ids.push(&value.id);
    //                                     }
    //                                 }
    //                                 postgresql_crud::JsonArrayElementChange::Delete(value) => {
    //                                     if ids.contains(&value) {
    //                                         return
    //                                         Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
    //                                         :: StdOptionOptionStdVecVecGenericWithIdDoggieNotUniqueId
    //                                         {
    //                                             std_option_option_std_vec_vec_generic_with_id_doggie_not_unique_id
    //                                             : * value, code_occurence : error_occurence_lib ::
    //                                             code_occurence! (),
    //                                         });
    //                                     } else {
    //                                         ids.push(&value);
    //                                     }
    //                                 }
    //                                 _ => (),
    //                             }
    //                         }
    //                     }
    //                 }
    //                 let current_jsonb_set_target = format!(
    //                     "{jsonb_set_target}->'std_option_option_std_vec_vec_generic_with_id'"
    //                 );
    //                 let mut update_query_part_acc = std::string::String::default();
    //                 for (index, element) in &value
    //                     .value
    //                     .iter()
    //                     .enumerate()
    //                     .collect::<std::vec::Vec<(usize, &std::option::Option<DoggieJsonArrayElementChange>)>>()
    //                 {
    //                     match &element {
    //                         Some(value) => {
    //                             match postgresql_crud::JsonArrayElementBindQuery::try_generate_update_bind_increments(
    //                                 value,
    //                                 &jsonb_set_accumulator,
    //                                 &jsonb_set_target,
    //                                 &jsonb_set_path,
    //                                 increment,
    //                                 is_array_object_element.clone()
    //                             ) {
    //                                 Ok(value) => {
    //                                     if let Some(value) = value { 
    //                                         update_query_part_acc.push_str(&value);
    //                                     }
    //                                 },
    //                                 Err(error) => {
    //                                     return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements {
    //                                         std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments: error,
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     });
    //                                 }
    //                             }
    //                         },
    //                         None => {
    //                             ////herereree
    //                             match increment.checked_add(1) {
    //                                 Some(value) => {
    //                                     * increment = value;
    //                                     // acc.push_str(& format!("'{{std_primitive_i16}}',${increment}"));
    //                                     // jsonb_set(elem,{acc})
    //                                     update_query_part_acc.push_str("@@@HERETODO@@@");
    //                                 }
    //                                 None => {
    //                                     return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd {
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     });
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //                 let mut delete_query_part_acc = std::string::String::default();
    //                 for (index, element) in &value
    //                     .value
    //                     .iter()
    //                     .enumerate()
    //                     .collect::<std::vec::Vec<(usize, &std::option::Option<DoggieJsonArrayElementChange>)>>()
    //                 {
    //                     match element {
    //                         Some(value) => {
    //                             match postgresql_crud::JsonArrayElementBindQuery::try_generate_delete_bind_increments(value, increment) {
    //                                 Ok(value) => {
    //                                     if let Some(value) = value {
    //                                         let maybe_space_and_space = if delete_query_part_acc.is_empty() {
    //                                             ""
    //                                         } else {
    //                                             " and "
    //                                         };
    //                                         delete_query_part_acc.push_str(&format!("{value}{maybe_space_and_space}"));
    //                                     }
    //                                 },
    //                                 Err(error) => {
    //                                     return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements {
    //                                         std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments: error,
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     });
    //                                 }
    //                             }
    //                         },
    //                         None => {
    //                             //todo
    //                             delete_query_part_acc.push_str("@@@HERETODO@@@");
    //                         }
    //                     }
    //                 }
    //                 let mut create_query_part_acc = std::string::String::default();
    //                 for (index, element) in &value
    //                     .value
    //                     .iter()
    //                     .enumerate()
    //                     .collect::<std::vec::Vec<(usize, &std::option::Option<DoggieJsonArrayElementChange>)>>()
    //                 {
    //                     match element {
    //                         Some(value) => {
    //                             match postgresql_crud::JsonArrayElementBindQuery::try_generate_create_bind_increments(value, increment) {
    //                                 Ok(value) => {
    //                                     if let Some(value) = value {
    //                                         create_query_part_acc.push_str(&format!("{value},"));
    //                                     }
    //                                 },
    //                                 Err(error) => {
    //                                     return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements {
    //                                         std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments: error,
    //                                         code_occurence: error_occurence_lib::code_occurence!(),
    //                                     });
    //                                 }
    //                             }
    //                         },
    //                         None => {
    //                             //todo
    //                             create_query_part_acc.push_str("@@@HERETODO@@@");
    //                         }
    //                     }
    //                 }
    //                 let _ = create_query_part_acc.pop();
    //                 let maybe_jsonb_agg_case = if update_query_part_acc.is_empty() {
    //                     std::string::String::from("elem")
    //                 } else {
    //                     format!("case {update_query_part_acc} else elem end")
    //                 };
    //                 let maybe_where = if delete_query_part_acc.is_empty() {
    //                     std::string::String::default()
    //                 } else {
    //                     format!(" where {delete_query_part_acc}")
    //                 };
    //                 let maybe_jsonb_build_array = if create_query_part_acc.is_empty() {
    //                     std::string::String::default()
    //                 } else {
    //                     format!(" || jsonb_build_array({create_query_part_acc})")
    //                 };
    //                 acc = format!
    //                 ("jsonb_set({acc},'{{{previous_jsonb_set_path}std_option_option_std_vec_vec_generic_with_id}}',(select jsonb_agg({maybe_jsonb_agg_case}) from jsonb_array_elements({current_jsonb_set_target}) as elem {maybe_where}){maybe_jsonb_build_array})");
    //             }
    //         }
    //     }
    //     Ok(acc)
    // }
//     fn bind_value_to_query<'a>(
//         self,
//         mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
//     ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         for element in self.0 {
//             match element {
//                 SomethingOptionToUpdate::StdVecVecGenericWithId(value) => {
//                     for element in &value.value {
//                         query =
//                             postgresql_crud::JsonArrayElementBindQuery::bind_update_value_to_query(
//                                 element.clone(),
//                                 query,
//                             );
//                     }
//                     for element in &value.value {
//                         query =
//                             postgresql_crud::JsonArrayElementBindQuery::bind_delete_value_to_query(
//                                 element.clone(),
//                                 query,
//                             );
//                     }
//                     for element in &value.value {
//                         query =
//                             postgresql_crud::JsonArrayElementBindQuery::bind_create_value_to_query(
//                                 element.clone(),
//                                 query,
//                             );
//                     }
//                 }
//                 SomethingOptionToUpdate::StdOptionOptionStdVecVecGenericWithId(value) => {
//                     for element in &value.value {
//                         match element {
//                             Some(value) => {
//                                 query = postgresql_crud::JsonArrayElementBindQuery::bind_update_value_to_query(
//                                     value.clone(),
//                                     query,
//                                 );
//                             },
//                             None => {
//                                 query = query.bind(sqlx::types::Json(None::<std::option::Option<DoggieJsonArrayElementChange>>));
//                             }
//                         }
//                     }
//                     for element in &value.value {
//                         match element {
//                             Some(value) => {
//                                 query = postgresql_crud::JsonArrayElementBindQuery::bind_delete_value_to_query(
//                                     value.clone(),
//                                     query,
//                                 );
//                             },
//                             None => {
//                                 query = query.bind(sqlx::types::Json(None::<std::option::Option<DoggieJsonArrayElementChange>>));
//                             }
//                         }
//                     }
//                     for element in &value.value {
//                         match element {
//                             Some(value) => {
//                                 query = postgresql_crud::JsonArrayElementBindQuery::bind_create_value_to_query(
//                                     value.clone(),
//                                     query,
//                                 );
//                             },
//                             None => {
//                                 query = query.bind(sqlx::types::Json(None::<std::option::Option<DoggieJsonArrayElementChange>>));
//                             }
//                         }
//                     }
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
// pub struct SomethingToCreate {
//     pub std_vec_vec_generic_with_id: postgresql_crud::JsonStdVecVecGenericWithId<DoggieToCreate>,
//     pub std_option_option_std_vec_vec_generic_with_id:
//         postgresql_crud::JsonStdOptionOptionStdVecVecGenericWithId<DoggieToCreate>,
// }
// impl postgresql_crud ::
// StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
// for SomethingToCreate
// {
//     #[inline] fn
//     default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
//     -> Self
//     {
//         Self
//         {
//             std_vec_vec_generic_with_id : postgresql_crud ::
//             StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
//             ::
//             default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             std_option_option_std_vec_vec_generic_with_id : postgresql_crud ::
//             StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
//             ::
//             default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
//         }
//     }
// }
// impl<'a> postgresql_crud::BindQuery<'a> for SomethingToCreate {
//     fn try_increment(
//         &self,
//         increment: &mut std::primitive::u64,
//     ) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         todo!()
//     }
//     fn try_generate_bind_increments(
//         &self,
//         increment: &mut std::primitive::u64,
//     ) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         let mut increments = std::string::String::from("");
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
//             increments.push_str(&format!(
//                 "'std_vec_vec_generic_with_id',jsonb_build_array({acc}),"
//             ));
//         }
//         {
//             let mut acc = std::string::String::default();
//             match &self.std_option_option_std_vec_vec_generic_with_id.0 {
//                 Some(value) => {
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
//                 }
//                 None => {
// //heeeeere
//                     // match increment.checked_add(1) {
//                     //     Some(value) => {
//                     //         *increment = value;
//                     //         acc.push_str(& format!("${increment}"));
//                     //     } 
//                     //     None => {
//                     //         return Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed::CheckedAdd {
//                     //             code_occurence: error_occurence_lib::code_occurence!(),
//                     //         });
//                     //     }
//                     // }
//                     todo!()
//                 }
//             }
//             let _ = acc.pop();
//             increments.push_str(&format!(
//                 "'std_option_option_std_vec_vec_generic_with_id',jsonb_build_array({acc}),"//here maybe jsonb_build_array
//             ));
//         }
//         let _ = increments.pop();
//         Ok(format!("jsonb_build_object({increments})"))
//     }
//     fn bind_value_to_query(
//         self,
//         mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
//     ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
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
//                 query = query.bind(sqlx::types::Json(None::<std::option::Option<DoggieToCreate>>));
//             }
//         }
//         query
//     }
// }
// impl postgresql_crud::CheckIdExistsInJsonGenericFields for Something {
//     fn check_id_exists_in_json_generic_fields(&self) {
//         let _ : () = postgresql_crud ::
//         CheckIdExistsInJsonStdVecVecGenericWithId ::
//         check_id_exists_in_json_std_vec_vec_generic_with_id(&
//         self.std_vec_vec_generic_with_id);
//         let _ : () = postgresql_crud ::
//         CheckIdExistsInJsonStdOptionOptionStdVecVecGenericWithId ::
//         check_id_exists_in_json_std_option_option_std_vec_vec_generic_with_id(&
//         self.std_option_option_std_vec_vec_generic_with_id);
//     }
// }
// //////////
// impl std::fmt::Display for Doggie {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde :: Serialize,
//     serde :: Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
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
// #[derive(
//     Debug,
//     serde :: Serialize,
//     serde :: Deserialize,
//     thiserror :: Error,
//     error_occurence_lib :: ErrorOccurence,
// )]
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
// impl error_occurence_lib::ToStdStringString
//     for DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
// {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:?}")
//     }
// }
// impl
//     postgresql_crud::GeneratePostgresqlQueryPartToRead<
//         DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed,
//         (),
//     > for DoggieFieldToRead
// {
//     fn generate_postgresql_query_part_to_read_from_self_vec(
//         value: &std::vec::Vec<Self>,
//         column_name_and_maybe_field_getter: &std::primitive::str,
//         column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
//         is_optional: std::primitive::bool,
//     ) -> Result<std::string::String, DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed>
//     {
//         if value.is_empty() {
//             return Err(
//                 DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed::FieldsFilterIsEmpty {
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 },
//             );
//         }
//         let mut unique = vec![];
//         for element in value {
//             if unique.contains(&element) {
//                 return
//                 Err(DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
//                 :: NotUniqueFieldFilter
//                 {
//                     field : element.clone(), code_occurence :
//                     error_occurence_lib :: code_occurence! (),
//                 });
//             } else {
//                 unique.push(&element);
//             }
//         }
//         let mut acc = std::string::String::default();
//         for element in value {
//             acc.push_str(& format!
//             ("{}||", match element
//             {
//                 Self :: Id => format!
//                 ("jsonb_build_object('id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'id') = 'string' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'id') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.id is not string')) end )"),
//                 Self :: StdPrimitiveI16 => format!
//                 ("jsonb_build_object('std_primitive_i16',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i16') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i16') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_primitive_i16 is not number')) end )")
//             }));
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         let is_optional_query_part =
//         match is_optional
//         {
//             true => format!
//             ("when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null)"),
//             false => std :: string :: String :: default()
//         };
//         Ok({
//             let space_and_not_null = if is_optional { " and not null" } else { "" };
//             format!
//             ("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}){is_optional_query_part} else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object{space_and_not_null}')) end")
//         })
//     }
//     fn generate_postgresql_query_part_to_read(
//         &self,
//         column_name_and_maybe_field_getter: &std::primitive::str,
//         column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
//     ) -> Result<std::string::String, ()> {
//         match self
//         {
//             Self :: Id =>
//             Ok(format!
//             ("jsonb_build_object('id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'id') = 'string' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'id') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.id is not string')) end )")),
//             Self :: StdPrimitiveI16 =>
//             Ok(format!
//             ("jsonb_build_object('std_primitive_i16',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i16') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i16') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_primitive_i16 is not number')) end )"))
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
//         Self {
//             id: Some(value.id.0),
//             std_primitive_i16: Some(postgresql_crud::Value {
//                 value: value.std_primitive_i16.0,
//             }),
//         }
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
//             fn expecting(
//                 &self,
//                 __formatter: &mut serde::__private::Formatter<'_>,
//             ) -> serde::__private::fmt::Result {
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
//             fn expecting(
//                 &self,
//                 __formatter: &mut serde::__private::Formatter<'_>,
//             ) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "struct DoggieOptionsToRead")
//             }
//             #[inline]
//             fn visit_seq<__A>(
//                 self,
//                 mut __seq: __A,
//             ) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<
//                     std::option::Option<
//                         std::result::Result<postgresql_crud::Uuid, std::string::String>,
//                     >,
//                 >(&mut __seq)?
//                 {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(
//                             0usize,
//                             &"struct DoggieOptionsToRead with 2 elements",
//                         ));
//                     }
//                 };
//                 let __field1 = match serde::de::SeqAccess::next_element::<
//                     std::option::Option<
//                         std::result::Result<std::primitive::i16, std::string::String>,
//                     >,
//                 >(&mut __seq)?
//                 {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(
//                             1usize,
//                             &"struct DoggieOptionsToRead with 2 elements",
//                         ));
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
//             fn visit_map<__A>(
//                 self,
//                 mut __map: __A,
//             ) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::MapAccess<'de>,
//             {
//                 let mut __field0: serde::__private::Option<
//                     std::option::Option<
//                         std::result::Result<postgresql_crud::Uuid, std::string::String>,
//                     >,
//                 > = serde::__private::None;
//                 let mut __field1: serde::__private::Option<
//                     std::option::Option<
//                         std::result::Result<std::primitive::i16, std::string::String>,
//                     >,
//                 > = serde::__private::None;
//                 while let serde::__private::Some(__key) =
//                     serde::de::MapAccess::next_key::<__Field>(&mut __map)?
//                 {
//                     match __key {
//                         __Field::__field0 => {
//                             if serde::__private::Option::is_some(&__field0) {
//                                 return serde::__private::Err(
//                                     <__A::Error as serde::de::Error>::duplicate_field("id"),
//                                 );
//                             }
//                             __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<
//                                 std::option::Option<
//                                     std::result::Result<postgresql_crud::Uuid, std::string::String>,
//                                 >,
//                             >(
//                                 &mut __map
//                             )?);
//                         }
//                         __Field::__field1 => {
//                             if serde::__private::Option::is_some(&__field1) {
//                                 return serde::__private::Err(
//                                     <__A::Error as serde::de::Error>::duplicate_field(
//                                         "std_primitive_i16",
//                                     ),
//                                 );
//                             }
//                             __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<
//                                 std::option::Option<
//                                     std::result::Result<std::primitive::i16, std::string::String>,
//                                 >,
//                             >(
//                                 &mut __map
//                             )?);
//                         }
//                         _ => {
//                             let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(
//                                 &mut __map,
//                             )?;
//                         }
//                     }
//                 }
//                 let __field0 = match __field0 {
//                     serde::__private::Some(__field0) => __field0,
//                     serde::__private::None => serde::__private::de::missing_field("id")?,
//                 };
//                 let __field1 = match __field1 {
//                     serde::__private::Some(__field1) => __field1,
//                     serde::__private::None => {
//                         serde::__private::de::missing_field("std_primitive_i16")?
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
//         }
//         #[doc(hidden)]
//         const FIELDS: &'static [&'static str] = &["id", "std_primitive_i16"];
//         serde::Deserializer::deserialize_struct(
//             __deserializer,
//             "DoggieOptionsToRead",
//             FIELDS,
//             __Visitor {
//                 marker: serde::__private::PhantomData::<DoggieOptionsToRead>,
//                 lifetime: serde::__private::PhantomData,
//             },
//         )
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
//             fn expecting(
//                 &self,
//                 __formatter: &mut serde::__private::Formatter<'_>,
//             ) -> serde::__private::fmt::Result {
//                 serde::__private::Formatter::write_str(__formatter, "tuple struct DoggieReader")
//             }
//             #[inline]
//             fn visit_newtype_struct<__E>(
//                 self,
//                 __e: __E,
//             ) -> serde::__private::Result<Self::Value, __E::Error>
//             where
//                 __E: serde::Deserializer<'de>,
//             {
//                 let __field0: Result<DoggieOptionsToRead, std::string::String> = <Result<
//                     DoggieOptionsToRead,
//                     std::string::String,
//                 > as serde::Deserialize>::deserialize(
//                     __e
//                 )?;
//                 serde::__private::Ok(DoggieReader(match __field0 {
//                     Ok(value) => value,
//                     Err(error) => {
//                         return Err(serde::de::Error::custom(error));
//                     }
//                 }))
//             }
//             #[inline]
//             fn visit_seq<__A>(
//                 self,
//                 mut __seq: __A,
//             ) -> serde::__private::Result<Self::Value, __A::Error>
//             where
//                 __A: serde::de::SeqAccess<'de>,
//             {
//                 let __field0 = match serde::de::SeqAccess::next_element::<
//                     Result<DoggieOptionsToRead, std::string::String>,
//                 >(&mut __seq)?
//                 {
//                     serde::__private::Some(__value) => __value,
//                     serde::__private::None => {
//                         return serde::__private::Err(serde::de::Error::invalid_length(
//                             0usize,
//                             &"tuple struct DoggieReader with 1 element",
//                         ));
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
//         serde::Deserializer::deserialize_newtype_struct(
//             __deserializer,
//             "DoggieReader",
//             __Visitor {
//                 marker: serde::__private::PhantomData::<DoggieReader>,
//                 lifetime: serde::__private::PhantomData,
//             },
//         )
//     }
// }
// impl postgresql_crud ::
// StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
// for Doggie
// {
//     #[inline] fn
//     default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
//     -> Self
//     {
//         Self
//         {
//             id : postgresql_crud ::
//             StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
//             ::
//             default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             std_primitive_i16 : postgresql_crud ::
//             StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
//             ::
//             default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
//         }
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde :: Serialize,
//     serde :: Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
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
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct DoggieOptionsToUpdate {
//     pub id: postgresql_crud::JsonUuid,
//     pub fields: std::vec::Vec<DoggieOptionToUpdate>,
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
// impl
//     postgresql_crud::JsonArrayElementBindQuery<
//         DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed,
//     > for DoggieJsonArrayElementChange
// {
//     fn try_generate_update_bind_increments(
//         &self,
//         jsonb_set_accumulator: &std::primitive::str,
//         jsonb_set_target: &std::primitive::str,
//         jsonb_set_path: &std::primitive::str,
//         increment: &mut std::primitive::u64,
//         is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple,
//     ) -> Result<
//         std::option::Option<std::string::String>,
//         DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed,
//     > {
//         match &self.0 {
//             postgresql_crud::JsonArrayElementChange::Update(value) => {
//                 match increment.checked_add(1) {
//                     Some(new_increment_value) =>
//                     {
//                         *increment = new_increment_value; 
//                         if value.fields.is_empty() {
//                             return Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed:: FieldsIsEmpty {
//                                 code_occurence : error_occurence_lib :: code_occurence! (),
//                             },);
//                         }
//                         {
//                             let mut acc = vec! []; for element in & value.fields
//                             {
//                                 match element
//                                 {
//                                     DoggieOptionToUpdate :: StdPrimitiveI16(_) =>
//                                     {
//                                         let value = DoggieFieldToUpdate :: StdPrimitiveI16; if
//                                         acc.contains(& value)
//                                         {
//                                             return
//                                             Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed
//                                             :: NotUniqueField
//                                             {
//                                                 field : value, code_occurence : error_occurence_lib ::
//                                                 code_occurence! (),
//                                             });
//                                         } else { acc.push(value); }
//                                     }
//                                 }
//                             }
//                         } 
//                         let id_increment = format! ("${increment}"); let mut acc =
//                         std :: string :: String :: default(); 
//                         for element in &value.fields {
//                             match & element
//                             {
//                                 DoggieOptionToUpdate :: StdPrimitiveI16(_) =>
//                                 {
//                                     match increment.checked_add(1)
//                                     {
//                                         Some(value) =>
//                                         {
//                                             * increment = value;
//                                             acc.push_str(& format!("'{{std_primitive_i16}}',${increment}"));
//                                         } None =>
//                                         {
//                                             return
//                                             Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed
//                                             :: CheckedAdd
//                                             {
//                                                 code_occurence : error_occurence_lib :: code_occurence! (),
//                                             },);
//                                         }
//                                     }
//                                 }
//                             } acc.push_str(",");
//                         } 
//                         let _ = acc.pop();
//                         Ok(Some(format!("when elem->>'id' = {id_increment} then jsonb_set(elem,{acc})")))
//                     } 
                    
//                     None =>
//                     Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed
//                     :: CheckedAdd
//                     {
//                         code_occurence : error_occurence_lib :: code_occurence! (),
//                     })
//                 }
//             }
//             _ => Ok(None),
//         }
//     }
//     fn bind_update_value_to_query<'a>(
//         self,
//         mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
//     ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         if let postgresql_crud::JsonArrayElementChange::Update(value) = self.0 {
//             query = query.bind(value.id.0.to_string());
//             for element in value.fields {
//                 match element {
//                     DoggieOptionToUpdate::StdPrimitiveI16(value) => {
//                         query = query.bind(sqlx::types::Json(value.value));
//                     }
//                 }
//             }
//         }
//         query
//     }
//     fn try_generate_delete_bind_increments(
//         &self,
//         increment: &mut std::primitive::u64,
//     ) -> Result<
//         std::option::Option<std::string::String>,
//         postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed,
//     > {
//         match &self.0 {
//             postgresql_crud::JsonArrayElementChange::Delete(value) => {
//                 match increment.checked_add(1)
//                 {
//                     Some(value) =>
//                     {
//                         * increment = value;
//                         Ok(Some(format! ("elem->>'id' <> ${increment}")))
//                     } None =>
//                     Err(postgresql_crud ::
//                     TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed ::
//                     CheckedAdd
//                     {
//                         code_occurence : error_occurence_lib :: code_occurence! (),
//                     })
//                 }
//             }
//             _ => Ok(None),
//         }
//     }
//     fn bind_delete_value_to_query<'a>(
//         self,
//         mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
//     ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         if let postgresql_crud::JsonArrayElementChange::Delete(value) = self.0 {
//             query = query.bind(value.0.to_string());
//         }
//         query
//     }
//     fn try_generate_create_bind_increments(
//         &self,
//         increment: &mut std::primitive::u64,
//     ) -> Result<
//         std::option::Option<std::string::String>,
//         postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed,
//     > {
//         match & self.0
//         {
//             postgresql_crud :: JsonArrayElementChange :: Create(value) =>
//             {
//                 match postgresql_crud :: BindQuery ::
//                 try_generate_bind_increments(value, increment)
//                 {
//                     Ok(value) => Ok(Some(value)), Err(error) =>
//                     Err(postgresql_crud ::
//                     TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed ::
//                     TryGenerateBindIncrements
//                     {
//                         error : error, code_occurence : error_occurence_lib ::
//                         code_occurence! (),
//                     }),
//                 }
//             }, _ => Ok(None)
//         }
//     }
//     fn bind_create_value_to_query<'a>(
//         self,
//         mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
//     ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         if let postgresql_crud::JsonArrayElementChange::Create(value) = self.0 {
//             query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
//         }
//         query
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct DoggieJsonArrayElementChange(
//     postgresql_crud::JsonArrayElementChange<DoggieToCreate, DoggieOptionsToUpdate>,
// );
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
// impl
// postgresql_crud ::
// StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
// for DoggieToCreate
// {
//     #[inline] fn
//     default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
//     -> Self
//     {
//         Self
//         {
//             std_primitive_i16 : postgresql_crud ::
//             StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
//             ::
//             default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
//         }
//     }
// }
// impl<'a> postgresql_crud::BindQuery<'a> for DoggieToCreate {
//     fn try_increment(
//         &self,
//         increment: &mut std::primitive::u64,
//     ) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         todo!()
//     }
//     fn try_generate_bind_increments(
//         &self,
//         increment: &mut std::primitive::u64,
//     ) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
//         let mut increments = std::string::String::from("'id', to_jsonb(gen_random_uuid()),");
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("'std_primitive_i16',${increment},"));
//             }
//             None => {
//                 return Err(
//                     postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
//                         code_occurence: error_occurence_lib::code_occurence!(),
//                     },
//                 );
//             }
//         }
//         let _ = increments.pop();
//         Ok(format!("jsonb_build_object({increments})"))
//     }
//     fn bind_value_to_query(
//         self,
//         mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
//     ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
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


////////////////////
impl std::fmt::Display for Something {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", &self)
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub enum SomethingFieldToRead {
    #[serde(rename(
        serialize = "std_vec_vec_generic_with_id",
        deserialize = "std_vec_vec_generic_with_id"
    ))]
    StdVecVecGenericWithId {
        field_vec: std::vec::Vec<DoggieFieldToRead>,
        limit: std::primitive::u64,
        offset: std::primitive::u64,
    },
    #[serde(rename(
        serialize = "std_option_option_std_vec_vec_generic_with_id",
        deserialize = "std_option_option_std_vec_vec_generic_with_id"
    ))]
    StdOptionOptionStdVecVecGenericWithId {
        field_vec: std::vec::Vec<DoggieFieldToRead>,
        limit: std::primitive::u64,
        offset: std::primitive::u64,
    },
}
impl error_occurence_lib::ToStdStringString for SomethingFieldToRead {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: SomethingFieldToRead,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    GeneratePostgresqlQueryPart {
        #[eo_error_occurence]
        error: SomethingGeneratePostgresqlQueryPartToReadErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl error_occurence_lib::ToStdStringString
    for SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
{
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum SomethingGeneratePostgresqlQueryPartToReadErrorNamed {
    StdVecVecGenericWithIdOffsetPlusLimitIsIntOverflow {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: std::primitive::u64,
        #[eo_to_std_string_string_serialize_deserialize]
        offset: std::primitive::u64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecGenericWithIdOffsetPlusLimitIsIntOverflow {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: std::primitive::u64,
        #[eo_to_std_string_string_serialize_deserialize]
        offset: std::primitive::u64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DoggieGeneratePostgresqlQueryPartToReadFromSelfVec {
        #[eo_error_occurence]
        field: DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl
    postgresql_crud::GeneratePostgresqlQueryPartToRead<
        SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed,
        SomethingGeneratePostgresqlQueryPartToReadErrorNamed,
    > for SomethingFieldToRead
{
    fn generate_postgresql_query_part_to_read_from_self_vec(
        value: &std::vec::Vec<Self>,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
        is_optional: std::primitive::bool,
    ) -> Result<std::string::String, SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed>
    {
        if value.is_empty() {
            return
            Err(SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
            :: FieldsFilterIsEmpty
            { code_occurence : error_occurence_lib :: code_occurence! (), });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return
                Err(SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
                :: NotUniqueFieldFilter
                {
                    field : element.clone(), code_occurence :
                    error_occurence_lib :: code_occurence! (),
                });
            } else {
                unique.push(&element);
            }
        }
        let mut acc = std::string::String::default();
        for element in value {
            match element.generate_postgresql_query_part_to_read(
                column_name_and_maybe_field_getter,
                column_name_and_maybe_field_getter_for_error_message,
            ) {
                Ok(value) => {
                    acc.push_str(&format!("{value}||"));
                }
                Err(error) => {
                    return
                    Err(SomethingGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
                    :: GeneratePostgresqlQueryPart
                    {
                        error, code_occurence : error_occurence_lib ::
                        code_occurence! (),
                    });
                }
            }
        }
        let _ = acc.pop();
        let _ = acc.pop();
        let is_optional_query_part =
        match is_optional
        {
            true => format!
            ("when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null)"),
            false => std :: string :: String :: default()
        };
        Ok({
            let space_and_not_null = if is_optional { " and not null" } else { "" };
            format!
            ("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}){is_optional_query_part} else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object{space_and_not_null}')) end")
        })
    }
    fn generate_postgresql_query_part_to_read(
        &self,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
    ) -> Result<std::string::String, SomethingGeneratePostgresqlQueryPartToReadErrorNamed> {
        match self
        {
            Self :: StdVecVecGenericWithId { field_vec, limit, offset } =>
            match postgresql_crud :: GeneratePostgresqlQueryPartToRead ::
            generate_postgresql_query_part_to_read_from_self_vec(field_vec, &
            format! ("value"), & format!
            ("{column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_generic_with_id[array element]"),
            false)
            {
                Ok(value) =>
                {
                    let start = offset; let end = match
                    offset.checked_add(* limit)
                    {
                        Some(value) => value, None =>
                        {
                            return
                            Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed ::
                            StdVecVecGenericWithIdOffsetPlusLimitIsIntOverflow
                            {
                                limit : * limit, offset : * offset, code_occurence :
                                error_occurence_lib :: code_occurence! (),
                            });
                        }
                    };
                    Ok(format!
                    ("jsonb_build_object('std_vec_vec_generic_with_id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_vec_vec_generic_with_id') = 'array' then jsonb_build_object('Ok',(select jsonb_agg({value}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_vec_vec_generic_with_id')) with ordinality where ordinality between {start} and {end})) else jsonb_build_object(jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}} is not array')) end)"))
                }, Err(error) =>
                {
                    return
                    Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed ::
                    DoggieGeneratePostgresqlQueryPartToReadFromSelfVec
                    {
                        field : error, code_occurence : error_occurence_lib ::
                        code_occurence! (),
                    });
                }
            }, Self :: StdOptionOptionStdVecVecGenericWithId
            { field_vec, limit, offset } => match postgresql_crud ::
            GeneratePostgresqlQueryPartToRead ::
            generate_postgresql_query_part_to_read_from_self_vec(field_vec, &
            format! ("value"), & format!
            ("{column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_generic_with_id[array element]"),
            false)
            {
                Ok(value) =>
                {
                    let start = offset; let end = match
                    offset.checked_add(* limit)
                    {
                        Some(value) => value, None =>
                        {
                            return
                            Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed ::
                            StdOptionOptionStdVecVecGenericWithIdOffsetPlusLimitIsIntOverflow
                            {
                                limit : * limit, offset : * offset, code_occurence :
                                error_occurence_lib :: code_occurence! (),
                            });
                        }
                    };
                    Ok(format!
                    ("jsonb_build_object('std_option_option_std_vec_vec_generic_with_id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic_with_id') = 'array' then jsonb_build_object('Ok',(select jsonb_agg({value}) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic_with_id')) with ordinality where ordinality between {start} and {end}))when jsonb_typeof({column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic_with_id') = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object(jsonb_build_object('Err','type of {{column_name_and_maybe_field_getter_for_error_message}} is not array and not null')) end)"))
                }, Err(error) =>
                {
                    return
                    Err(SomethingGeneratePostgresqlQueryPartToReadErrorNamed ::
                    DoggieGeneratePostgresqlQueryPartToReadFromSelfVec
                    {
                        field : error, code_occurence : error_occurence_lib ::
                        code_occurence! (),
                    });
                }
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
pub struct SomethingOptionsToRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    std_vec_vec_generic_with_id:
        std::option::Option<postgresql_crud::Value<std::vec::Vec<DoggieOptionsToRead>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_option_option_std_vec_vec_generic_with_id: std::option::Option<
        postgresql_crud::Value<std::option::Option<std::vec::Vec<DoggieOptionsToRead>>>,
    >,
}
impl std::convert::From<Something> for SomethingOptionsToRead {
    fn from(value: Something) -> Self {
        Self {
            std_vec_vec_generic_with_id: Some(postgresql_crud::Value {
                value: value
                    .std_vec_vec_generic_with_id
                    .0
                    .into_iter()
                    .map(|element| DoggieOptionsToRead::from(element))
                    .collect::<std::vec::Vec<DoggieOptionsToRead>>(),
            }),
            std_option_option_std_vec_vec_generic_with_id: Some(postgresql_crud::Value {
                value: match value.std_option_option_std_vec_vec_generic_with_id.0 {
                    Some(value) => Some(
                        value
                            .into_iter()
                            .map(|element| DoggieOptionsToRead::from(element))
                            .collect::<std::vec::Vec<DoggieOptionsToRead>>(),
                    ),
                    None => None,
                },
            }),
        }
    }
}
impl<'de> serde::Deserialize<'de> for SomethingOptionsToRead {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
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
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "std_vec_vec_generic_with_id" => serde::__private::Ok(__Field::__field0),
                    "std_option_option_std_vec_vec_generic_with_id" => {
                        serde::__private::Ok(__Field::__field1)
                    }
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"std_vec_vec_generic_with_id" => serde::__private::Ok(__Field::__field0),
                    b"std_option_option_std_vec_vec_generic_with_id" => {
                        serde::__private::Ok(__Field::__field1)
                    }
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SomethingOptionsToRead>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SomethingOptionsToRead;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter<'_>,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct SomethingOptionsToRead")
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
                    std::option::Option<
                        std::result::Result<
                            std::vec::Vec<
                                std::result::Result<DoggieOptionsToRead, std::string::String>,
                            >,
                            std::string::String,
                        >,
                    >,
                >(&mut __seq)?
                {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(
                            0usize,
                            &"struct SomethingOptionsToRead with 2 elements",
                        ));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<
                    std::option::Option<
                        std::result::Result<
                            std::option::Option<
                                std::vec::Vec<
                                    std::result::Result<DoggieOptionsToRead, std::string::String>,
                                >,
                            >,
                            std::string::String,
                        >,
                    >,
                >(&mut __seq)?
                {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(
                            1usize,
                            &"struct SomethingOptionsToRead with 2 elements",
                        ));
                    }
                };
                serde::__private::Ok(SomethingOptionsToRead {
                    std_vec_vec_generic_with_id: match __field0 {
                        Some(value) => match value {
                            Ok(value) => Some(postgresql_crud::Value {
                                value: {
                                    let mut acc = vec![];
                                    for element in value {
                                        match element {
                                            Ok(value) => {
                                                acc.push(value);
                                            }
                                            Err(error) => {
                                                return Err(serde::de::Error::custom(error));
                                            }
                                        }
                                    }
                                    acc
                                },
                            }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None,
                    },
                    std_option_option_std_vec_vec_generic_with_id: match __field1 {
                        Some(value) => match value {
                            Ok(value) => Some(postgresql_crud::Value {
                                value: match value {
                                    Some(value) => {
                                        let mut acc = vec![];
                                        for element in value {
                                            match element {
                                                Ok(value) => {
                                                    acc.push(value);
                                                }
                                                Err(error) => {
                                                    return Err(serde::de::Error::custom(error));
                                                }
                                            }
                                        }
                                        Some(acc)
                                    }
                                    None => None,
                                },
                            }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None,
                    },
                })
            }
            #[inline]
            fn visit_map<__A>(
                self,
                mut __map: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<
                            std::vec::Vec<
                                std::result::Result<DoggieOptionsToRead, std::string::String>,
                            >,
                            std::string::String,
                        >,
                    >,
                > = serde::__private::None;
                let mut __field1: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<
                            std::option::Option<
                                std::vec::Vec<
                                    std::result::Result<DoggieOptionsToRead, std::string::String>,
                                >,
                            >,
                            std::string::String,
                        >,
                    >,
                > = serde::__private::None;
                while let serde::__private::Some(__key) =
                    serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "std_vec_vec_generic_with_id",
                                    ),
                                );
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                std::option::Option<
                                    std::result::Result<
                                        std::vec::Vec<
                                            std::result::Result<
                                                DoggieOptionsToRead,
                                                std::string::String,
                                            >,
                                        >,
                                        std::string::String,
                                    >,
                                >,
                            >(
                                &mut __map
                            )?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "std_option_option_std_vec_vec_generic_with_id",
                                    ),
                                );
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                std::option::Option<
                                    std::result::Result<
                                        std::option::Option<
                                            std::vec::Vec<
                                                std::result::Result<
                                                    DoggieOptionsToRead,
                                                    std::string::String,
                                                >,
                                            >,
                                        >,
                                        std::string::String,
                                    >,
                                >,
                            >(
                                &mut __map
                            )?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(
                                &mut __map,
                            )?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => {
                        serde::__private::de::missing_field("std_vec_vec_generic_with_id")?
                    }
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field(
                        "std_option_option_std_vec_vec_generic_with_id",
                    )?,
                };
                serde::__private::Ok(SomethingOptionsToRead {
                    std_vec_vec_generic_with_id: match __field0 {
                        Some(value) => match value {
                            Ok(value) => Some(postgresql_crud::Value {
                                value: {
                                    let mut acc = vec![];
                                    for element in value {
                                        match element {
                                            Ok(value) => {
                                                acc.push(value);
                                            }
                                            Err(error) => {
                                                return Err(serde::de::Error::custom(error));
                                            }
                                        }
                                    }
                                    acc
                                },
                            }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None,
                    },
                    std_option_option_std_vec_vec_generic_with_id: match __field1 {
                        Some(value) => match value {
                            Ok(value) => Some(postgresql_crud::Value {
                                value: match value {
                                    Some(value) => {
                                        let mut acc = vec![];
                                        for element in value {
                                            match element {
                                                Ok(value) => {
                                                    acc.push(value);
                                                }
                                                Err(error) => {
                                                    return Err(serde::de::Error::custom(error));
                                                }
                                            }
                                        }
                                        Some(acc)
                                    }
                                    None => None,
                                },
                            }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None,
                    },
                })
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &[
            "std_vec_vec_generic_with_id",
            "std_option_option_std_vec_vec_generic_with_id",
        ];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "SomethingOptionsToRead",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<SomethingOptionsToRead>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
pub struct SomethingReader(pub SomethingOptionsToRead);
impl<'de> serde::Deserialize<'de> for SomethingReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<SomethingReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = SomethingReader;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter<'_>,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct SomethingReader")
            }
            #[inline]
            fn visit_newtype_struct<__E>(
                self,
                __e: __E,
            ) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: Result<SomethingOptionsToRead, std::string::String> = <Result<
                    SomethingOptionsToRead,
                    std::string::String,
                > as serde::Deserialize>::deserialize(
                    __e
                )?;
                serde::__private::Ok(SomethingReader(match __field0 {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
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
                    Result<SomethingOptionsToRead, std::string::String>,
                >(&mut __seq)?
                {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct SomethingReader with 1 element",
                        ));
                    }
                };
                serde::__private::Ok(SomethingReader(match __field0 {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "SomethingReader",
            __Visitor {
                marker: serde::__private::PhantomData::<SomethingReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud ::
StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
for Something
{
    #[inline] fn
    default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    -> Self
    {
        Self
        {
            std_vec_vec_generic_with_id : postgresql_crud ::
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
            ::
            default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_vec_vec_generic_with_id : postgresql_crud ::
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
            ::
            default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        }
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub enum SomethingFieldToUpdate {
    #[serde(rename(
        serialize = "std_vec_vec_generic_with_id",
        deserialize = "std_vec_vec_generic_with_id"
    ))]
    StdVecVecGenericWithId,
    #[serde(rename(
        serialize = "std_option_option_std_vec_vec_generic_with_id",
        deserialize = "std_option_option_std_vec_vec_generic_with_id"
    ))]
    StdOptionOptionStdVecVecGenericWithId,
}
impl error_occurence_lib::ToStdStringString for SomethingFieldToUpdate {
    fn to_std_string_string(&self) -> std::string::String {
        match &self {
            Self::StdVecVecGenericWithId => "std_vec_vec_generic_with_id".to_owned(),
            Self::StdOptionOptionStdVecVecGenericWithId => {
                "std_option_option_std_vec_vec_generic_with_id".to_owned()
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum SomethingOptionToUpdate {
    #[serde(rename(
        serialize = "std_vec_vec_generic_with_id",
        deserialize = "std_vec_vec_generic_with_id"
    ))]
    StdVecVecGenericWithId(postgresql_crud::Value<std::vec::Vec<DoggieJsonArrayElementChange>>),
    #[serde(rename(
        serialize = "std_option_option_std_vec_vec_generic_with_id",
        deserialize = "std_option_option_std_vec_vec_generic_with_id"
    ))]
    StdOptionOptionStdVecVecGenericWithId(
        postgresql_crud::Value<std::vec::Vec<std::option::Option<DoggieJsonArrayElementChange>>>,
    ),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct SomethingOptionsToUpdate(pub std::vec::Vec<SomethingOptionToUpdate>);
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed {
    FieldsIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        field: SomethingFieldToUpdate,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecGenericWithIdDoggieNotUniqueId {
        #[eo_to_std_string_string_serialize_deserialize]
        std_vec_vec_generic_with_id_doggie_not_unique_id: postgresql_crud::JsonUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements {
        #[eo_error_occurence]
        std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments:
            DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements {
        #[eo_error_occurence]
        std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments:
            postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements {
        #[eo_error_occurence]
        std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments:
            postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecGenericWithIdDoggieNotUniqueId {
        #[eo_to_std_string_string_serialize_deserialize]
        std_option_option_std_vec_vec_generic_with_id_doggie_not_unique_id:
            postgresql_crud::JsonUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements {
        #[eo_error_occurence]
        std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments:
            DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements {
        #[eo_error_occurence]
        std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments:
            postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements {
        #[eo_error_occurence]
        std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments:
            postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl
    postgresql_crud::GeneratePostgresqlQueryPartToUpdate<
        SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed,
    > for SomethingOptionsToUpdate
{
    fn try_generate_bind_increments(
        &self,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
        is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple,
    ) -> Result<std::string::String, SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed>
    {
        if self.0.is_empty() {
            return Err(
                SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::FieldsIsEmpty {
                    code_occurence: error_occurence_lib::code_occurence!(),
                },
            );
        }
        {
            let mut acc = vec![];
            for element in &self.0 {
                match element {
                    SomethingOptionToUpdate::StdVecVecGenericWithId(_) => {
                        let value = SomethingFieldToUpdate::StdVecVecGenericWithId;
                        if acc.contains(&value) {
                            return
                            Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                            :: NotUniqueField
                            {
                                field : value, code_occurence : error_occurence_lib ::
                                code_occurence! (),
                            },);
                        } else {
                            acc.push(value);
                        }
                    }
                    SomethingOptionToUpdate::StdOptionOptionStdVecVecGenericWithId(_) => {
                        let value = SomethingFieldToUpdate::StdOptionOptionStdVecVecGenericWithId;
                        if acc.contains(&value) {
                            return
                            Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                            :: NotUniqueField
                            {
                                field : value, code_occurence : error_occurence_lib ::
                                code_occurence! (),
                            },);
                        } else {
                            acc.push(value);
                        }
                    }
                }
            }
        }
        let mut acc = std::string::String::from(jsonb_set_accumulator);
        let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
            true => std::string::String::default(),
            false => format!("{jsonb_set_path},"),
        };
        for element in &self.0 {
            match &element {
                SomethingOptionToUpdate::StdVecVecGenericWithId(value) => {
                    {
                        let mut ids: std::vec::Vec<&postgresql_crud::JsonUuid> = vec![];
                        for element in &value.value {
                            match &element.0 {
                                postgresql_crud::JsonArrayElementChange::Update(value) => {
                                    if ids.contains(&&value.id) {
                                        return
                                        Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                                        :: StdVecVecGenericWithIdDoggieNotUniqueId
                                        {
                                            std_vec_vec_generic_with_id_doggie_not_unique_id : value.id,
                                            code_occurence : error_occurence_lib :: code_occurence! (),
                                        });
                                    } else {
                                        ids.push(&value.id);
                                    }
                                }
                                postgresql_crud::JsonArrayElementChange::Delete(value) => {
                                    if ids.contains(&value) {
                                        return
                                        Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                                        :: StdVecVecGenericWithIdDoggieNotUniqueId
                                        {
                                            std_vec_vec_generic_with_id_doggie_not_unique_id : * value,
                                            code_occurence : error_occurence_lib :: code_occurence! (),
                                        });
                                    } else {
                                        ids.push(&value);
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    let current_jsonb_set_target =
                        format!("{jsonb_set_target}->'std_vec_vec_generic_with_id'");
                    let mut update_query_part_acc = std::string::String::default();
                    for (index, element) in &value
                        .value
                        .iter()
                        .enumerate()
                        .collect::<std::vec::Vec<(usize, &DoggieJsonArrayElementChange)>>()
                    {
                        match postgresql_crud :: JsonArrayElementBindQuery ::
                        try_generate_update_bind_increments(* element, &
                        jsonb_set_accumulator, & jsonb_set_target, & jsonb_set_path,
                        increment, is_array_object_element.clone(),)
                        {
                            Ok(value) =>
                            {
                                if let Some(value) = value
                                { update_query_part_acc.push_str(& value); }
                            }, Err(error) =>
                            {
                                return
                                Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                                ::
                                StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements
                                {
                                    std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments
                                    : error, code_occurence : error_occurence_lib ::
                                    code_occurence! (),
                                });
                            }
                        }
                    }
                    let mut delete_query_part_acc = std::string::String::default();
                    for (index, element) in &value
                        .value
                        .iter()
                        .enumerate()
                        .collect::<std::vec::Vec<(usize, &DoggieJsonArrayElementChange)>>()
                    {
                        match postgresql_crud :: JsonArrayElementBindQuery ::
                        try_generate_delete_bind_increments(* element, increment)
                        {
                            Ok(value) =>
                            {
                                if let Some(value) = value
                                {
                                    let maybe_space_and_space = if
                                    delete_query_part_acc.is_empty() { "" } else { " and " };
                                    delete_query_part_acc.push_str(& format!
                                    ("{value}{maybe_space_and_space}"));
                                }
                            }, Err(error) =>
                            {
                                return
                                Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                                ::
                                StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements
                                {
                                    std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments
                                    : error, code_occurence : error_occurence_lib ::
                                    code_occurence! (),
                                });
                            }
                        }
                    }
                    let mut create_query_part_acc = std::string::String::default();
                    for (index, element) in &value
                        .value
                        .iter()
                        .enumerate()
                        .collect::<std::vec::Vec<(usize, &DoggieJsonArrayElementChange)>>()
                    {
                        match postgresql_crud :: JsonArrayElementBindQuery ::
                        try_generate_create_bind_increments(* element, increment)
                        {
                            Ok(value) =>
                            {
                                if let Some(value) = value
                                { create_query_part_acc.push_str(& format! ("{value},")); }
                            }, Err(error) =>
                            {
                                return
                                Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                                ::
                                StdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements
                                {
                                    std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments
                                    : error, code_occurence : error_occurence_lib ::
                                    code_occurence! (),
                                });
                            }
                        }
                    }
                    let _ = create_query_part_acc.pop();
                    let maybe_jsonb_agg_case = if update_query_part_acc.is_empty() {
                        std::string::String::from("elem")
                    } else {
                        format!("case {update_query_part_acc} else elem end")
                    };
                    let maybe_where = if delete_query_part_acc.is_empty() {
                        std::string::String::default()
                    } else {
                        format!(" where {delete_query_part_acc}")
                    };
                    let maybe_jsonb_build_array = if create_query_part_acc.is_empty() {
                        std::string::String::default()
                    } else {
                        format!(" || jsonb_build_array({create_query_part_acc})")
                    };
                    acc = format!
                    ("jsonb_set({acc},'{{{previous_jsonb_set_path}std_vec_vec_generic_with_id}}',(select jsonb_agg({maybe_jsonb_agg_case}) from jsonb_array_elements({current_jsonb_set_target}) as elem {maybe_where}){maybe_jsonb_build_array})");
                }
                SomethingOptionToUpdate::StdOptionOptionStdVecVecGenericWithId(value) => {
                    {
                        let mut ids: std::vec::Vec<&postgresql_crud::JsonUuid> = vec![];
                        for element in &value.value {
                            if let Some(value) = element {
                                match &value.0 {
                                    postgresql_crud::JsonArrayElementChange::Update(value) => {
                                        if ids.contains(&&value.id) {
                                            return
                                            Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                                            :: StdOptionOptionStdVecVecGenericWithIdDoggieNotUniqueId
                                            {
                                                std_option_option_std_vec_vec_generic_with_id_doggie_not_unique_id
                                                : value.id, code_occurence : error_occurence_lib ::
                                                code_occurence! (),
                                            });
                                        } else {
                                            ids.push(&value.id);
                                        }
                                    }
                                    postgresql_crud::JsonArrayElementChange::Delete(value) => {
                                        if ids.contains(&value) {
                                            return
                                            Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                                            :: StdOptionOptionStdVecVecGenericWithIdDoggieNotUniqueId
                                            {
                                                std_option_option_std_vec_vec_generic_with_id_doggie_not_unique_id
                                                : * value, code_occurence : error_occurence_lib ::
                                                code_occurence! (),
                                            });
                                        } else {
                                            ids.push(&value);
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    let current_jsonb_set_target = format!(
                        "{jsonb_set_target}->'std_option_option_std_vec_vec_generic_with_id'"
                    );
                    let mut update_query_part_acc = std::string::String::default();
                    for (index, element) in
                        &value.value.iter().enumerate().collect::<std::vec::Vec<(
                            usize,
                            &std::option::Option<DoggieJsonArrayElementChange>,
                        )>>()
                    {
                        match & element
                        {
                            Some(value) =>
                            {
                                match postgresql_crud :: JsonArrayElementBindQuery ::
                                try_generate_update_bind_increments(value, &
                                jsonb_set_accumulator, & jsonb_set_target, & jsonb_set_path,
                                increment, is_array_object_element.clone())
                                {
                                    Ok(value) =>
                                    {
                                        if let Some(value) = value
                                        { update_query_part_acc.push_str(& value); }
                                    }, Err(error) =>
                                    {
                                        return
                                        Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                                        ::
                                        StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementUpdateBindIncrements
                                        {
                                            std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_update_bind_increments
                                            : error, code_occurence : error_occurence_lib ::
                                            code_occurence! (),
                                        });
                                    }
                                }
                            }, None =>
                            {
                                match increment.checked_add(1)
                                {
                                    Some(value) =>
                                    {
                                        * increment = value;
                                        update_query_part_acc.push_str("@@@HERETODO@@@");
                                    } None =>
                                    {
                                        return
                                        Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                                        :: CheckedAdd
                                        {
                                            code_occurence : error_occurence_lib :: code_occurence! (),
                                        });
                                    }
                                }
                            }
                        }
                    }
                    let mut delete_query_part_acc = std::string::String::default();
                    for (index, element) in
                        &value.value.iter().enumerate().collect::<std::vec::Vec<(
                            usize,
                            &std::option::Option<DoggieJsonArrayElementChange>,
                        )>>()
                    {
                        match element
                        {
                            Some(value) =>
                            {
                                match postgresql_crud :: JsonArrayElementBindQuery ::
                                try_generate_delete_bind_increments(value, increment)
                                {
                                    Ok(value) =>
                                    {
                                        if let Some(value) = value
                                        {
                                            let maybe_space_and_space = if
                                            delete_query_part_acc.is_empty() { "" } else { " and " };
                                            delete_query_part_acc.push_str(& format!
                                            ("{value}{maybe_space_and_space}"));
                                        }
                                    }, Err(error) =>
                                    {
                                        return
                                        Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                                        ::
                                        StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementDeleteBindIncrements
                                        {
                                            std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_delete_bind_increments
                                            : error, code_occurence : error_occurence_lib ::
                                            code_occurence! (),
                                        });
                                    }
                                }
                            }, None =>
                            { delete_query_part_acc.push_str("@@@HERETODO@@@"); }
                        }
                    }
                    let mut create_query_part_acc = std::string::String::default();
                    for (index, element) in
                        &value.value.iter().enumerate().collect::<std::vec::Vec<(
                            usize,
                            &std::option::Option<DoggieJsonArrayElementChange>,
                        )>>()
                    {
                        match element
                        {
                            Some(value) =>
                            {
                                match postgresql_crud :: JsonArrayElementBindQuery ::
                                try_generate_create_bind_increments(value, increment)
                                {
                                    Ok(value) =>
                                    {
                                        if let Some(value) = value
                                        { create_query_part_acc.push_str(& format! ("{value},")); }
                                    }, Err(error) =>
                                    {
                                        return
                                        Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                                        ::
                                        StdOptionOptionStdVecVecGenericWithIdDoggieTryGenerateJsonArrayElementCreateBindIncrements
                                        {
                                            std_option_option_std_vec_vec_generic_with_id_doggie_try_generate_json_array_element_create_bind_increments
                                            : error, code_occurence : error_occurence_lib ::
                                            code_occurence! (),
                                        });
                                    }
                                }
                            }, None =>
                            { create_query_part_acc.push_str("@@@HERETODO@@@"); }
                        }
                    }
                    let _ = create_query_part_acc.pop();
                    let maybe_jsonb_agg_case = if update_query_part_acc.is_empty() {
                        std::string::String::from("elem")
                    } else {
                        format!("case {update_query_part_acc} else elem end")
                    };
                    let maybe_where = if delete_query_part_acc.is_empty() {
                        std::string::String::default()
                    } else {
                        format!(" where {delete_query_part_acc}")
                    };
                    let maybe_jsonb_build_array = if create_query_part_acc.is_empty() {
                        std::string::String::default()
                    } else {
                        format!(" || jsonb_build_array({create_query_part_acc})")
                    };
                    acc = format!
                    ("jsonb_set({acc},'{{{previous_jsonb_set_path}std_option_option_std_vec_vec_generic_with_id}}',(select jsonb_agg({maybe_jsonb_agg_case}) from jsonb_array_elements({current_jsonb_set_target}) as elem {maybe_where}){maybe_jsonb_build_array})");
                }
            }
        }
        Ok(acc)
    }
    fn bind_value_to_query<'a>(
        self,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0 {
            match element {
                SomethingOptionToUpdate::StdVecVecGenericWithId(value) => {
                    for element in &value.value {
                        query =
                            postgresql_crud::JsonArrayElementBindQuery::bind_update_value_to_query(
                                element.clone(),
                                query,
                            );
                    }
                    for element in &value.value {
                        query =
                            postgresql_crud::JsonArrayElementBindQuery::bind_delete_value_to_query(
                                element.clone(),
                                query,
                            );
                    }
                    for element in &value.value {
                        query =
                            postgresql_crud::JsonArrayElementBindQuery::bind_create_value_to_query(
                                element.clone(),
                                query,
                            );
                    }
                }
                SomethingOptionToUpdate::StdOptionOptionStdVecVecGenericWithId(value) => {
                    for element in &value.value {
                        query =
                            postgresql_crud::JsonArrayElementBindQuery::bind_update_value_to_query(
                                element.clone(),
                                query,
                            );
                    }
                    for element in &value.value {
                        query =
                            postgresql_crud::JsonArrayElementBindQuery::bind_delete_value_to_query(
                                element.clone(),
                                query,
                            );
                    }
                    for element in &value.value {
                        query =
                            postgresql_crud::JsonArrayElementBindQuery::bind_create_value_to_query(
                                element.clone(),
                                query,
                            );
                    }
                }
            }
        }
        query
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct SomethingToCreate {
    pub std_vec_vec_generic_with_id: postgresql_crud::JsonStdVecVecGenericWithId<DoggieToCreate>,
    pub std_option_option_std_vec_vec_generic_with_id:
        postgresql_crud::JsonStdOptionOptionStdVecVecGenericWithId<DoggieToCreate>,
}
impl postgresql_crud ::
StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
for SomethingToCreate
{
    #[inline] fn
    default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    -> Self
    {
        Self
        {
            std_vec_vec_generic_with_id : postgresql_crud ::
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
            ::
            default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_option_option_std_vec_vec_generic_with_id : postgresql_crud ::
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
            ::
            default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        }
    }
}
impl<'a> postgresql_crud::BindQuery<'a> for SomethingToCreate {
    fn try_increment(
        &self,
        increment: &mut std::primitive::u64,
    ) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn try_generate_bind_increments(
        &self,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        let mut increments = std::string::String::from("");
        {
            let mut acc = std::string::String::default();
            for element in &self.std_vec_vec_generic_with_id.0 {
                match element.try_generate_bind_increments(increment) {
                    Ok(value) => {
                        acc.push_str(&format!("{value},"));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            let _ = acc.pop();
            increments.push_str(&format!(
                "'std_vec_vec_generic_with_id',jsonb_build_array({acc}),"
            ));
        }
        {
            let mut acc = std::string::String::default();
            for element in &self.std_option_option_std_vec_vec_generic_with_id.0 {
                match element.try_generate_bind_increments(increment) {
                    Ok(value) => {
                        acc.push_str(&format!("{value},"));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            let _ = acc.pop();
            increments.push_str(&format!(
                "'std_option_option_std_vec_vec_generic_with_id',jsonb_build_array({acc}),"
            ));
        }
        let _ = increments.pop();
        Ok(format!("jsonb_build_object({increments})"))
    }
    fn bind_value_to_query(
        self,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.std_vec_vec_generic_with_id.0 {
            query = element.bind_value_to_query(query);
        }
        for element in self.std_option_option_std_vec_vec_generic_with_id.0 {
            query = element.bind_value_to_query(query);
        }
        query
    }
}
impl postgresql_crud::CheckIdExistsInJsonGenericFields for Something {
    fn check_id_exists_in_json_generic_fields(&self) {
        let _ : () = postgresql_crud ::
        CheckIdExistsInJsonStdVecVecGenericWithId ::
        check_id_exists_in_json_std_vec_vec_generic_with_id(&
        self.std_vec_vec_generic_with_id);
        let _ : () = postgresql_crud ::
        CheckIdExistsInJsonStdOptionOptionStdVecVecGenericWithId ::
        check_id_exists_in_json_std_option_option_std_vec_vec_generic_with_id(&
        self.std_option_option_std_vec_vec_generic_with_id);
    }
}
/////////
impl std::fmt::Display for Doggie {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", &self)
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub enum DoggieFieldToRead {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id,
    #[serde(rename(serialize = "std_primitive_i16", deserialize = "std_primitive_i16"))]
    StdPrimitiveI16,
}
impl error_occurence_lib::ToStdStringString for DoggieFieldToRead {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: DoggieFieldToRead,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl error_occurence_lib::ToStdStringString
    for DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
{
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl
    postgresql_crud::GeneratePostgresqlQueryPartToRead<
        DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed,
        (),
    > for DoggieFieldToRead
{
    fn generate_postgresql_query_part_to_read_from_self_vec(
        value: &std::vec::Vec<Self>,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
        is_optional: std::primitive::bool,
    ) -> Result<std::string::String, DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed>
    {
        if value.is_empty() {
            return Err(
                DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed::FieldsFilterIsEmpty {
                    code_occurence: error_occurence_lib::code_occurence!(),
                },
            );
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return
                Err(DoggieGeneratePostgresqlQueryPartToReadFromSelfVecErrorNamed
                :: NotUniqueFieldFilter
                {
                    field : element.clone(), code_occurence :
                    error_occurence_lib :: code_occurence! (),
                });
            } else {
                unique.push(&element);
            }
        }
        let mut acc = std::string::String::default();
        for element in value {
            acc.push_str(& format!
            ("{}||", match element
            {
                Self :: Id => format!
                ("jsonb_build_object('id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'id') = 'string' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'id') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.id is not string')) end )"),
                Self :: StdPrimitiveI16 => format!
                ("jsonb_build_object('std_primitive_i16',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i16') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i16') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_primitive_i16 is not number')) end )")
            }));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        let is_optional_query_part =
        match is_optional
        {
            true => format!
            ("when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null)"),
            false => std :: string :: String :: default()
        };
        Ok({
            let space_and_not_null = if is_optional { " and not null" } else { "" };
            format!
            ("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}){is_optional_query_part} else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object{space_and_not_null}')) end")
        })
    }
    fn generate_postgresql_query_part_to_read(
        &self,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
    ) -> Result<std::string::String, ()> {
        match self
        {
            Self :: Id =>
            Ok(format!
            ("jsonb_build_object('id',case when jsonb_typeof({column_name_and_maybe_field_getter}->'id') = 'string' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'id') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.id is not string')) end )")),
            Self :: StdPrimitiveI16 =>
            Ok(format!
            ("jsonb_build_object('std_primitive_i16',case when jsonb_typeof({column_name_and_maybe_field_getter}->'std_primitive_i16') = 'number' then jsonb_build_object('Ok',{column_name_and_maybe_field_getter}->'std_primitive_i16') else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message}.std_primitive_i16 is not number')) end )"))
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
pub struct DoggieOptionsToRead {
    id: std::option::Option<postgresql_crud::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_primitive_i16: std::option::Option<postgresql_crud::Value<std::primitive::i16>>,
}
impl std::convert::From<Doggie> for DoggieOptionsToRead {
    fn from(value: Doggie) -> Self {
        Self {
            id: Some(value.id.0),
            std_primitive_i16: Some(postgresql_crud::Value {
                value: value.std_primitive_i16.0,
            }),
        }
    }
}
impl<'de> serde::Deserialize<'de> for DoggieOptionsToRead {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
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
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "id" => serde::__private::Ok(__Field::__field0),
                    "std_primitive_i16" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"id" => serde::__private::Ok(__Field::__field0),
                    b"std_primitive_i16" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<DoggieOptionsToRead>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = DoggieOptionsToRead;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter<'_>,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct DoggieOptionsToRead")
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
                    std::option::Option<
                        std::result::Result<postgresql_crud::Uuid, std::string::String>,
                    >,
                >(&mut __seq)?
                {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(
                            0usize,
                            &"struct DoggieOptionsToRead with 2 elements",
                        ));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<
                    std::option::Option<
                        std::result::Result<std::primitive::i16, std::string::String>,
                    >,
                >(&mut __seq)?
                {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(
                            1usize,
                            &"struct DoggieOptionsToRead with 2 elements",
                        ));
                    }
                };
                serde::__private::Ok(DoggieOptionsToRead {
                    id: match __field0 {
                        Some(value) => match value {
                            Ok(value) => Some(value),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None,
                    },
                    std_primitive_i16: match __field1 {
                        Some(value) => match value {
                            Ok(value) => Some(postgresql_crud::Value { value: value }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None,
                    },
                })
            }
            #[inline]
            fn visit_map<__A>(
                self,
                mut __map: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<postgresql_crud::Uuid, std::string::String>,
                    >,
                > = serde::__private::None;
                let mut __field1: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<std::primitive::i16, std::string::String>,
                    >,
                > = serde::__private::None;
                while let serde::__private::Some(__key) =
                    serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field("id"),
                                );
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                std::option::Option<
                                    std::result::Result<postgresql_crud::Uuid, std::string::String>,
                                >,
                            >(
                                &mut __map
                            )?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "std_primitive_i16",
                                    ),
                                );
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                std::option::Option<
                                    std::result::Result<std::primitive::i16, std::string::String>,
                                >,
                            >(
                                &mut __map
                            )?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(
                                &mut __map,
                            )?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("id")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => {
                        serde::__private::de::missing_field("std_primitive_i16")?
                    }
                };
                serde::__private::Ok(DoggieOptionsToRead {
                    id: match __field0 {
                        Some(value) => match value {
                            Ok(value) => Some(value),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None,
                    },
                    std_primitive_i16: match __field1 {
                        Some(value) => match value {
                            Ok(value) => Some(postgresql_crud::Value { value: value }),
                            Err(error) => {
                                return Err(serde::de::Error::custom(error));
                            }
                        },
                        None => None,
                    },
                })
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["id", "std_primitive_i16"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "DoggieOptionsToRead",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<DoggieOptionsToRead>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema)]
pub struct DoggieReader(pub DoggieOptionsToRead);
impl<'de> serde::Deserialize<'de> for DoggieReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<DoggieReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = DoggieReader;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter<'_>,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct DoggieReader")
            }
            #[inline]
            fn visit_newtype_struct<__E>(
                self,
                __e: __E,
            ) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: Result<DoggieOptionsToRead, std::string::String> = <Result<
                    DoggieOptionsToRead,
                    std::string::String,
                > as serde::Deserialize>::deserialize(
                    __e
                )?;
                serde::__private::Ok(DoggieReader(match __field0 {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
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
                    Result<DoggieOptionsToRead, std::string::String>,
                >(&mut __seq)?
                {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct DoggieReader with 1 element",
                        ));
                    }
                };
                serde::__private::Ok(DoggieReader(match __field0 {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(serde::de::Error::custom(error));
                    }
                }))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "DoggieReader",
            __Visitor {
                marker: serde::__private::PhantomData::<DoggieReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud ::
StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
for Doggie
{
    #[inline] fn
    default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    -> Self
    {
        Self
        {
            id : postgresql_crud ::
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
            ::
            default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_primitive_i16 : postgresql_crud ::
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
            ::
            default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        }
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub enum DoggieFieldToUpdate {
    #[serde(rename(serialize = "std_primitive_i16", deserialize = "std_primitive_i16"))]
    StdPrimitiveI16,
}
impl error_occurence_lib::ToStdStringString for DoggieFieldToUpdate {
    fn to_std_string_string(&self) -> std::string::String {
        match &self {
            Self::StdPrimitiveI16 => "std_primitive_i16".to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum DoggieOptionToUpdate {
    #[serde(rename(serialize = "std_primitive_i16", deserialize = "std_primitive_i16"))]
    StdPrimitiveI16(postgresql_crud::Value<std::primitive::i16>),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DoggieOptionsToUpdate {
    pub id: postgresql_crud::JsonUuid,
    pub fields: std::vec::Vec<DoggieOptionToUpdate>,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed {
    FieldsIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        field: DoggieFieldToUpdate,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FieldsIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        field: DoggieFieldToUpdate,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl
    postgresql_crud::JsonArrayElementBindQuery<
        DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed,
    > for DoggieJsonArrayElementChange
{
    fn try_generate_update_bind_increments(
        &self,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
        is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple,
    ) -> Result<
        std::option::Option<std::string::String>,
        DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed,
    > {
        match &self.0 {
            postgresql_crud::JsonArrayElementChange::Update(value) => {
                match increment.checked_add(1)
                {
                    Some(new_increment_value) =>
                    {
                        * increment = new_increment_value; if
                        value.fields.is_empty()
                        {
                            return
                            Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed
                            :: FieldsIsEmpty
                            {
                                code_occurence : error_occurence_lib :: code_occurence! (),
                            },);
                        }
                        {
                            let mut acc = vec! []; for element in & value.fields
                            {
                                match element
                                {
                                    DoggieOptionToUpdate :: StdPrimitiveI16(_) =>
                                    {
                                        let value = DoggieFieldToUpdate :: StdPrimitiveI16; if
                                        acc.contains(& value)
                                        {
                                            return
                                            Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed
                                            :: NotUniqueField
                                            {
                                                field : value, code_occurence : error_occurence_lib ::
                                                code_occurence! (),
                                            });
                                        } else { acc.push(value); }
                                    }
                                }
                            }
                        } let id_increment = format! ("${increment}"); let mut acc =
                        std :: string :: String :: default(); for element in &
                        value.fields
                        {
                            match & element
                            {
                                DoggieOptionToUpdate :: StdPrimitiveI16(_) =>
                                {
                                    match increment.checked_add(1)
                                    {
                                        Some(value) =>
                                        {
                                            * increment = value;
                                            acc.push_str(& format!
                                            ("'{{std_primitive_i16}}',${increment}"));
                                        } None =>
                                        {
                                            return
                                            Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed
                                            :: CheckedAdd
                                            {
                                                code_occurence : error_occurence_lib :: code_occurence! (),
                                            },);
                                        }
                                    }
                                }
                            } acc.push_str(",");
                        } let _ = acc.pop();
                        Ok(Some(format!
                        ("when elem->>'id' = {id_increment} then jsonb_set(elem,{acc})")))
                    } None =>
                    Err(DoggieTryGenerateJsonArrayElementUpdateBindIncrementsErrorNamed
                    :: CheckedAdd
                    {
                        code_occurence : error_occurence_lib :: code_occurence! (),
                    })
                }
            }
            _ => Ok(None),
        }
    }
    fn bind_update_value_to_query<'a>(
        self,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let postgresql_crud::JsonArrayElementChange::Update(value) = self.0 {
            query = query.bind(value.id.0.to_string());
            for element in value.fields {
                match element {
                    DoggieOptionToUpdate::StdPrimitiveI16(value) => {
                        query = query.bind(sqlx::types::Json(value.value));
                    }
                }
            }
        }
        query
    }
    fn try_generate_delete_bind_increments(
        &self,
        increment: &mut std::primitive::u64,
    ) -> Result<
        std::option::Option<std::string::String>,
        postgresql_crud::TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed,
    > {
        match &self.0 {
            postgresql_crud::JsonArrayElementChange::Delete(value) => {
                match increment.checked_add(1)
                {
                    Some(value) =>
                    {
                        * increment = value;
                        Ok(Some(format! ("elem->>'id' <> ${increment}")))
                    } None =>
                    Err(postgresql_crud ::
                    TryGenerateJsonArrayElementDeleteBindIncrementsErrorNamed ::
                    CheckedAdd
                    {
                        code_occurence : error_occurence_lib :: code_occurence! (),
                    })
                }
            }
            _ => Ok(None),
        }
    }
    fn bind_delete_value_to_query<'a>(
        self,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let postgresql_crud::JsonArrayElementChange::Delete(value) = self.0 {
            query = query.bind(value.0.to_string());
        }
        query
    }
    fn try_generate_create_bind_increments(
        &self,
        increment: &mut std::primitive::u64,
    ) -> Result<
        std::option::Option<std::string::String>,
        postgresql_crud::TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed,
    > {
        match & self.0
        {
            postgresql_crud :: JsonArrayElementChange :: Create(value) =>
            {
                match postgresql_crud :: BindQuery ::
                try_generate_bind_increments(value, increment)
                {
                    Ok(value) => Ok(Some(value)), Err(error) =>
                    Err(postgresql_crud ::
                    TryGenerateJsonArrayElementCreateBindIncrementsErrorNamed ::
                    TryGenerateBindIncrements
                    {
                        error : error, code_occurence : error_occurence_lib ::
                        code_occurence! (),
                    }),
                }
            }, _ => Ok(None)
        }
    }
    fn bind_create_value_to_query<'a>(
        self,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let postgresql_crud::JsonArrayElementChange::Create(value) = self.0 {
            query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
        }
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DoggieJsonArrayElementChange(
    postgresql_crud::JsonArrayElementChange<DoggieToCreate, DoggieOptionsToUpdate>,
);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct DoggieToCreate {
    pub std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16,
}
impl
postgresql_crud ::
StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
for DoggieToCreate
{
    #[inline] fn
    default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    -> Self
    {
        Self
        {
            std_primitive_i16 : postgresql_crud ::
            StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
            ::
            default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        }
    }
}
impl<'a> postgresql_crud::BindQuery<'a> for DoggieToCreate {
    fn try_increment(
        &self,
        increment: &mut std::primitive::u64,
    ) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn try_generate_bind_increments(
        &self,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        let mut increments = std::string::String::from("'id', to_jsonb(gen_random_uuid()),");
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                increments.push_str(&format!("'std_primitive_i16',${increment},"));
            }
            None => {
                return Err(
                    postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    },
                );
            }
        }
        let _ = increments.pop();
        Ok(format!("jsonb_build_object({increments})"))
    }
    fn bind_value_to_query(
        self,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.std_primitive_i16.0));
        query
    }
}
impl postgresql_crud::GetJsonId for Doggie {
    fn get_json_id(&self) -> &postgresql_crud::JsonUuid {
        &self.id
    }
}
impl postgresql_crud::CheckIdExistsInJsonGenericFields for Doggie {
    fn check_id_exists_in_json_generic_fields(&self) {}
}
