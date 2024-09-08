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
     postgresql_crud::GeneratePostgresqlQueryPart
)] //user type must implement utoipa::ToSchema trait
pub struct Something {
    // pub id: postgresql_crud::JsonUuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    pub std_primitive_i8: postgresql_crud::JsonStdPrimitiveI8,
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

    // pub generic: postgresql_crud::JsonGeneric<Doggie>,
    // pub std_option_option_generic: postgresql_crud::JsonStdOptionOptionGeneric<Doggie>,
    pub std_vec_vec_generic: postgresql_crud::JsonStdVecVecGeneric<Doggie>,
    // pub std_option_option_std_vec_vec_generic: postgresql_crud::JsonStdOptionOptionStdVecVecGeneric<Doggie>,
    // pub std_vec_vec_std_option_option_generic: postgresql_crud::JsonStdVecVecStdOptionOptionGeneric<Doggie>,
    // pub std_option_option_std_vec_vec_std_option_option_generic: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric<Doggie>,
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
        // pub std_vec_vec_generic: postgresql_crud::JsonStdVecVecGeneric<Doggie>,
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
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> 
    {
        query = query.bind(sqlx::types::Json(self.std_primitive_i8.0));
        query = query.bind(sqlx::types::Json(self.std_vec_vec_generic.0));
        query
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,
     postgresql_crud::GeneratePostgresqlQueryPart
)] //user type must implement utoipa::ToSchema trait
pub struct Doggie {
    pub id: postgresql_crud::JsonUuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    pub std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16,
    // pub generic: postgresql_crud::JsonGeneric<Cat>,
}

// #[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema,
//     postgresql_crud::GeneratePostgresqlQueryPart
// )] //user type must implement utoipa::ToSchema trait
// pub struct Cat {
//     // pub id: postgresql_crud::JsonUuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

//     pub std_primitive_i32: postgresql_crud::JsonStdPrimitiveI32,
// }

#[test]
fn test_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() {
//     let default = postgresql_crud::JsonGeneric(Something::default());
//     println!("{default:#?}");
//     let default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element: postgresql_crud::JsonGeneric::<Something> = postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element();
//     println!("{default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element:#?}");
//     let serialized = serde_json::to_string(&default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element).unwrap();
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
        // 5. Append a New Element to an Array
        // Scenario: Add "Go" to Bob's skills array.

        // UPDATE users
        // SET details = jsonb_set(details, '{skills}', details->'skills' || '"Go"')
        // WHERE name = 'Bob';

        // 6. Update Multiple Keys at Once

        // UPDATE users
        // SET details = details
        //     || jsonb_build_object('age', 31)
        //     || jsonb_build_object('city', 'San Diego')
        // WHERE name = 'Bob';
        //

        //read

    //   select 
    //     case when jsonb_typeof(
    //       sqlx_types_json_t_as_postgresql_json_b_not_null
    //     ) = 'object' then jsonb_build_object(
    //       'Ok', 
    //       jsonb_build_object(
    //         'std_primitive_i8', 
    //         case when jsonb_typeof(
    //           sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_primitive_i8'
    //         ) = 'number' then jsonb_build_object(
    //           'Ok', sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_primitive_i8'
    //         ) else jsonb_build_object(
    //           jsonb_build_object(
    //             'Err', 'type of sqlx_types_json_t_as_postgresql_json_b_not_null.std_primitive_i8 is not number'
    //           )
    //         ) end
    //       )|| jsonb_build_object(
    //         'std_primitive_i16', 
    //         case when jsonb_typeof(
    //           sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_primitive_i16'
    //         ) = 'number' then jsonb_build_object(
    //           'Ok', sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_primitive_i16'
    //         ) else jsonb_build_object(
    //           jsonb_build_object(
    //             'Err', 'type of sqlx_types_json_t_as_postgresql_json_b_not_null.std_primitive_i16 is not number'
    //           )
    //         ) end
    //       )
    //     ) else jsonb_build_object(
    //       jsonb_build_object(
    //         'Err', 'type of sqlx_types_json_t_as_postgresql_json_b_not_null is not object'
    //       )
    //     ) end as sqlx_types_json_t_as_postgresql_json_b_not_null 
    //   from 
    //     jsongeneric 
    //   where 
    //     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = $1


        // update jsongeneric set sqlx_types_json_t_as_postgresql_json_b_not_null = $1 where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = $2 returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key

        //   update 
        //     jsongeneric 
        //   set 
        //     std_primitive_i32_as_postgresql_int = 42,
        //     sqlx_types_json_t_as_postgresql_json_b_not_null = jsonb_set(sqlx_types_json_t_as_postgresql_json_b_not_null, '{generic,std_string_string}', '"dark"', false)
        //   where 
        //     std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 1 returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key

// update 
//   jsongeneric 
// set 
//   sqlx_types_json_t_as_postgresql_json_b_not_null = sqlx_types_json_t_as_postgresql_json_b_not_null
//         || jsonb_build_object('std_primitive_i8', 2)
//         || jsonb_build_object('std_primitive_i16', 2)
// where 
//   std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14 returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key


// UPDATE jsongeneric 
// SET sqlx_types_json_t_as_postgresql_json_b_not_null = jsonb_set(
//                         jsonb_set(
//                             jsonb_set(
//                                 jsonb_set(
//                                     jsonb_set(
//                                         jsonb_set(
//                                             sqlx_types_json_t_as_postgresql_json_b_not_null, 
//                                             '{one,two}', 
//                                             '"twovaluenew1"'
//                                         ), 
//                                         '{one,three,four}', 
//                                         '"fourvaluenew1"'
//                                     ), 
//                                     '{one,five}', 
//                                     '"fivevaluenew1"'
//                                 ), 
//                                 '{six}', 
//                                 '"sixvaluenew1"'
//                             ), 
//                             '{seven,eight}', 
//                             '"eightvaluenew1"'
//                         ), 
//                         '{nine}', 
//                         '"ninevaluenew1"'
//                     )
// WHERE std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14
// RETURNING std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;


// now jsonb  is {"one": {"two": "twovalue", "three": { "four": "fourvalue" }, "five": "fivevalue"}, "six": "sixvalue", "seven": { "eight": "eightvalue" }, "nine": "ninevalue"}. 
// write query what updates 
// key "two" to "twovaluenew", key "four" to "fourvaluenew", key "five" to "fivevaluenew", key "six" to "sixvaluenew", key "eight" to "eightvaluenew", key "nine" to "ninevaluenew"


// {
//     "one": {
//       "two": "twovaluenew1",
//       "three": {
//         "four": "fourvaluenew1"
//       },
//       "five": "fivevaluenew1"
//     },
//     "six": "sixvaluenew1",
//     "seven": {
//       "eight": "eightvaluenew1"
//     },
//     "nine": "ninevaluenew1"
//   }

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
                    SomethingOptionToUpdate::StdPrimitiveI8(_) => {
                        let value = SomethingFieldToUpdate::StdPrimitiveI8;
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
                    SomethingOptionToUpdate::StdVecVecGeneric(_) => {
                        let value = SomethingFieldToUpdate::StdVecVecGeneric;
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
                SomethingOptionToUpdate::StdPrimitiveI8(_) => match increment.checked_add(1) {
                    Some(value) => {
                        *increment = value;
                        acc = format!("jsonb_set({acc},'{{{previous_jsonb_set_path}std_primitive_i8}}',${increment})");
                    }
                    None => {
                        return
                            Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed
                            :: CheckedAdd
                            {
                                code_occurence : error_occurence_lib :: code_occurence! (),
                            },);
                    }
                },
                SomethingOptionToUpdate::StdVecVecGeneric(value) => {
                    let current_jsonb_set_target = format!("{jsonb_set_target}->'std_vec_vec_generic'");

                    let mut create_query_part_acc = std::string::String::default();
                    let mut update_query_part_acc = std::string::String::default();
                    let mut delete_query_part_acc = std::string::String::default();
                    for (index, element) in &value.value.iter().enumerate().collect::<std::vec::Vec<(usize, &DoggieOptionsToUpdate)>>() {
                        match postgresql_crud::JsonArrayElementQueryPart::try_generate_create_query_part(*element, increment) {
                            Ok(value) => {
                                if let Some(value) = value {
                                    create_query_part_acc.push_str(&format!("{value},"));
                                }
                            },
                            Err(error) => {
                                todo!()
                            }
                        }
                        match postgresql_crud::JsonArrayElementQueryPart::try_generate_update_query_part(
                            *element,
                            &jsonb_set_accumulator,
                            &jsonb_set_target,
                            &jsonb_set_path,
                            increment,
                            is_array_object_element.clone(),
                        ) {
                            Ok(value) => {
                                if let Some(value) = value {
                                    update_query_part_acc.push_str(&value);
                                }
                            },
                            Err(error) => {
                                todo!()
                            }
                        }
                        match postgresql_crud::JsonArrayElementQueryPart::try_generate_delete_query_part(
                            *element,
                            &jsonb_set_accumulator,
                            &jsonb_set_target,
                            &jsonb_set_path,
                            increment,
                            is_array_object_element.clone(),
                        ) {
                            Ok(value) => {
                                if let Some(value) = value {
                                    let maybe_space_and_space = if delete_query_part_acc.is_empty() {
                                        ""
                                    }
                                    else {
                                        " and "
                                    };
                                    delete_query_part_acc.push_str(&format!("{value}{maybe_space_and_space}"));
                                }
                            },
                            Err(error) => {
                                todo!()
                            }
                        }
                    }
                    let _ = create_query_part_acc.pop();
                    //
                    // for (index, element) in &value.value.iter().enumerate().collect::<std::vec::Vec<(usize, &DoggieOptionsToUpdate)>>() {
                    //     match element.try_generate_bind_increments(
                    //         &acc,
                    //         &format!("{jsonb_set_target}->'std_vec_vec_generic'"),
                    //         &format!("{{std_vec_vec_generic,{index}}}"),
                    //         increment,
                    //         postgresql_crud::ArrayObjectElementOrSimple::ArrayObjectElement {
                    //             jsonb_set_path: format!("std_vec_vec_generic,{index}"),
                    //             index: index.clone(),
                    //         },//for arrays it must be true?
                    //     ) {
                    //         Ok(value) => {
                    //             acc = value;
                    //         }
                    //         Err(error) => {
                    //             return Err(SomethingOptionsToUpdateTryGenerateBindIncrementsErrorNamed::Doggie {
                    //                 doggie : error, code_occurence : error_occurence_lib ::
                    //                 code_occurence! (),
                    //             },);
                    //         }
                    //     }
                    // }
                    let maybe_jsonb_agg_case = if update_query_part_acc.is_empty() {
                        std::string::String::from("elem")
                    }
                    else {
                        format!("case {update_query_part_acc} else elem end")
                    };
                    let maybe_where = if delete_query_part_acc.is_empty() {
                        std::string::String::default()
                    }
                    else {
                        format!(" where {delete_query_part_acc}")
                    };
                    let maybe_jsonb_build_array = if create_query_part_acc.is_empty() {
                        std::string::String::default()
                    }
                    else {
                        format!(" || jsonb_build_array({create_query_part_acc})")
                    };
                    acc = format!(r#"jsonb_set({acc},'{{{previous_jsonb_set_path}std_vec_vec_generic}}',(select jsonb_agg({maybe_jsonb_agg_case}) from jsonb_array_elements({current_jsonb_set_target}) as elem {maybe_where}){maybe_jsonb_build_array})"#);
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
                SomethingOptionToUpdate::StdPrimitiveI8(value) => {
                    query = query.bind(sqlx::types::Json(value.value));
                }
                SomethingOptionToUpdate::StdVecVecGeneric(value) => {
                    for element in value.value {
                        //first update then delete then create. order matters!
                        query = postgresql_crud::JsonArrayElementQueryPart::bind_update_value_to_query(element.clone(), query);
                        query = postgresql_crud::JsonArrayElementQueryPart::bind_delete_value_to_query(element.clone(), query);
                        query = postgresql_crud::JsonArrayElementQueryPart::bind_create_value_to_query(element.clone(), query);
                    }
                }
            }
        }
        query
    }
}

////
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
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8,
    #[serde(rename(serialize = "std_vec_vec_generic", deserialize = "std_vec_vec_generic"))]
    StdVecVecGeneric,
}
impl error_occurence_lib::ToStdStringString for SomethingFieldToUpdate {
    fn to_std_string_string(&self) -> std::string::String {
        match &self {
            Self::StdPrimitiveI8 => "std_primitive_i8".to_owned(),
            Self::StdVecVecGeneric => "std_vec_vec_generic".to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
enum SomethingOptionToUpdate {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::Value<std::primitive::i8>),
    #[serde(rename(serialize = "std_vec_vec_generic", deserialize = "std_vec_vec_generic"))]
    StdVecVecGeneric(postgresql_crud::Value<std::vec::Vec<DoggieOptionsToUpdate>>),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct SomethingOptionsToUpdate(std::vec::Vec<SomethingOptionToUpdate>);
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
    Doggie {
        #[eo_error_occurence]
        doggie: DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

// i have a jsonb in postgresql

// {
//     "std_primitive_i8": 8,
//     "std_vec_vec_generic": [
//         {
//             "id": "8cc5da73-1a7e-4ff4-9cfa-4f84998c62a4",
//             "std_primitive_i16": 5
//         },
//         {
//             "id": "951240e0-990e-4cb8-909d-5183ff7725a4",
//             "std_primitive_i16": 6
//         },
//         {
//             "id": "5d628632-13f0-409f-8288-42b356cc033c",
//             "std_primitive_i16": 2
//         },
//         {
//             "id": "63b83936-24c8-429b-ab67-ee2c76856f18",
//             "std_primitive_i16": 3
//         }
//     ]
// }

// i want to set key "std_primitive_i8" to 7.

// and i want to add element { "id": "1ff4db66-1395-4d58-bcf5-8bf69f1b90d3", "std_primitive_i16": 10 } to array in key "std_vec_vec_generic". 

// and i want to add element { "id": "847e5f32-d1a5-4d6a-9c55-040cbf60f229", "std_primitive_i16": 20 } to array in key "std_vec_vec_generic". 

// and i want to update key "std_primitive_i16" to value 44 where key "id" is "5d628632-13f0-409f-8288-42b356cc033c". 

// and i want to update key "std_primitive_i16" to value 55 where key "id" is "63b83936-24c8-429b-ab67-ee2c76856f18". 

// and i want to delete element in value of key "std_vec_vec_generic" where key "id" = "8cc5da73-1a7e-4ff4-9cfa-4f84998c62a4". 

// and i want to delete element in value of key "std_vec_vec_generic" where key "id" = "951240e0-990e-4cb8-909d-5183ff7725a4". 

// please write a query for it


// UPDATE jsongeneric
// SET sqlx_types_json_t_as_postgresql_json_b_not_null = 
//     jsonb_set(
//         jsonb_set(
//             sqlx_types_json_t_as_postgresql_json_b_not_null,
//             '{std_primitive_i8}',
//             '27'::jsonb
//         ),
//         '{std_vec_vec_generic}',
// 		(
//                 SELECT jsonb_agg(
//                     CASE
//                         WHEN elem->>'id' = '5d628632-13f0-409f-8288-42b356cc033c'
//                         THEN jsonb_set(elem, '{std_primitive_i16}', '44'::jsonb)
					
//                         WHEN elem->>'id' = '63b83936-24c8-429b-ab67-ee2c76856f18'
//                         THEN jsonb_set(elem, '{std_primitive_i16}', '55'::jsonb)
					
//                         ELSE elem
//                     END
//                 )
//                 FROM jsonb_array_elements(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_generic') AS elem
//                 WHERE elem->>'id' <> '8cc5da73-1a7e-4ff4-9cfa-4f84998c62a4' and elem->>'id' <> '951240e0-990e-4cb8-909d-5183ff7725a4'
//          ) 
// 		|| 
// 		jsonb_build_array(
//     		jsonb_build_object('id', '1ff4db66-1395-4d58-bcf5-8bf69f1b90d3', 'std_primitive_i16', 10),
//     		jsonb_build_object('id', '847e5f32-d1a5-4d6a-9c55-040cbf60f229', 'std_primitive_i16', 20)
// 		)
//     )
// where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14 returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;

#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DoggieOptionsToUpdateUodateErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl postgresql_crud::JsonArrayElementQueryPart<DoggieOptionsToUpdateUodateErrorNamed> for DoggieOptionsToUpdate {
    fn try_generate_create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::option::Option<std::string::String>, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        match &self.0 {
            postgresql_crud::JsonArrayElementChange::Create(value) => {
                match postgresql_crud::BindQuery::try_generate_bind_increments(value, increment) {
                    Ok(value) => Ok(Some(value)),
                    Err(error) => Err(error),
                }
            },
            _ => Ok(None)
        }
    }
    fn bind_create_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let postgresql_crud::JsonArrayElementChange::Create(value) = self.0 {
            query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
        }
        query
    }
    fn try_generate_update_query_part(
        &self,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
        is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple,
    ) -> Result<std::option::Option<std::string::String>, DoggieOptionsToUpdateUodateErrorNamed> {
        match &self.0 {
            postgresql_crud::JsonArrayElementChange::Update(value) => {
// pub struct DoggieOptionsToUpdateSSS {
//     id: uuid::Uuid,
//     update: std::vec::Vec<DoggieOptionToUpdate>,
// 
                let id = &value.id;
                //first checked_add for id
                match increment.checked_add(1) {
                    Some(new_increment_value) => {
                        *increment = new_increment_value;
                        ///////
                        // let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
                        //     true => std::string::String::default(),
                        //     false => format!("{jsonb_set_path}"),
                        // };
                        let mut acc = std::string::String::default();
                        for element in &value.update {
                            match &element {
                                DoggieOptionToUpdate::StdPrimitiveI16(_) => match increment.checked_add(1) {
                                    Some(value) => {
                                        *increment = value;
                                        acc.push_str(&format!("'std_primitive_i16',${increment}"));
                                    }
                                    None => {
                                        return Err(
                                            DoggieOptionsToUpdateUodateErrorNamed::CheckedAdd {
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            },
                                        );
                                    }
                                },
                                // DoggieOptionToUpdate::Generic(value) => {
                                //     match value.value.try_generate_bind_increments(
                                //         &format!("{jsonb_set_target}->{index}->'generic'"),
                                //         &format!("{jsonb_set_target}->{index}->'generic'"),
                                //         "",
                                //         increment,
                                //         is_array_object_element.clone()
                                //     ) {
                                //         Ok(value) => {
                                //             acc.push_str(&format!("'generic', {value}"));
                                //         }
                                //         Err(error) => {
                                //             return Err(
                                //                 DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed::Cat {
                                //                     cat: error,
                                //                     code_occurence: error_occurence_lib::code_occurence!(),
                                //                 },
                                //             );
                                //         }
                                //     }
                                // }
                            }
                            acc.push_str(",");
                        }
                        let _ = acc.pop();
                        // let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
                        //     true => std::string::String::default(),
                        //     false => format!("{jsonb_set_path}"),
                        // };
                        // Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{previous_jsonb_set_path}}}',jsonb_build_object({acc}))"))
                        ///////
                        Ok(Some(format!("when elem->>'id' = '${increment}' then jsonb_set({acc})")))
                        // Ok(Some(format!("when elem->>'id' = '${increment}' then jsonb_set(elem, '{{std_primitive_i16}}', '44'::jsonb)")))
                    }
                    None => Err(DoggieOptionsToUpdateUodateErrorNamed::CheckedAdd {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
            },
            _ => Ok(None)
        }
    }
    fn bind_update_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let postgresql_crud::JsonArrayElementChange::Update(value) = self.0 {
            for element in value.update {
                match element {
                    DoggieOptionToUpdate::StdPrimitiveI16(value) => {
                        query = query.bind(sqlx::types::Json(value.value));
                    }
                }
            }
        }
        query
    }
    fn try_generate_delete_query_part(
        &self,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
        is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple,
    ) -> Result<std::option::Option<std::string::String>, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        match &self.0 {
            postgresql_crud::JsonArrayElementChange::Delete(value) => {
                Ok(Some(format!("elem->>'id' <> '8cc5da73-1a7e-4ff4-9cfa-4f84998c62a4'")))
            },
            _ => Ok(None)
        }
    }
    fn bind_delete_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let postgresql_crud::JsonArrayElementChange::Delete(value) = self.0 {
            query = query.bind(sqlx::types::Json(value));
        }
        query
    }
}

// impl
//     postgresql_crud::GeneratePostgresqlQueryPartToUpdate<
//         DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed,
//     > for DoggieOptionsToUpdate
// {
//     fn try_generate_bind_increments(
//         &self,
//         jsonb_set_accumulator: &std::primitive::str,
//         jsonb_set_target: &std::primitive::str,
//         jsonb_set_path: &std::primitive::str,
//         increment: &mut std::primitive::u64,
//         is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple,
//     ) -> Result<std::string::String, DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed> {
//         //todo later do validation checks

// // UPDATE jsongeneric
// // SET sqlx_types_json_t_as_postgresql_json_b_not_null = 
// //     jsonb_set(
// //         jsonb_set(
// //             sqlx_types_json_t_as_postgresql_json_b_not_null,
// //             '{std_primitive_i8}',
// //             '27'::jsonb
// //         ),
// //         '{std_vec_vec_generic}',
// // 		(
// //                 SELECT jsonb_agg(
// //                     CASE
// //                         WHEN elem->>'id' = '5d628632-13f0-409f-8288-42b356cc033c'
// //                         THEN jsonb_set(elem, '{std_primitive_i16}', '44'::jsonb)
					
// //                         WHEN elem->>'id' = '63b83936-24c8-429b-ab67-ee2c76856f18'
// //                         THEN jsonb_set(elem, '{std_primitive_i16}', '55'::jsonb)
					
// //                         ELSE elem
// //                     END
// //                 )
// //                 FROM jsonb_array_elements(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_generic') AS elem
// //                 WHERE elem->>'id' <> '8cc5da73-1a7e-4ff4-9cfa-4f84998c62a4' and elem->>'id' <> '951240e0-990e-4cb8-909d-5183ff7725a4'
// //          ) 
// // 		|| '[{"id": "1ff4db66-1395-4d58-bcf5-8bf69f1b90d3", "std_primitive_i16": 10}]'::jsonb
// // 		|| '[{"id": "847e5f32-d1a5-4d6a-9c55-040cbf60f229", "std_primitive_i16": 20}]'::jsonb
// //     )
// // where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14 returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;



//     // jsonb_set(
//     //     sqlx_types_json_t_as_postgresql_json_b_not_null,
//     //     '{std_vec_vec_generic}',
// 	// 	(
//     //             SELECT jsonb_agg(
//     //                 CASE
//     //                     WHEN elem->>'id' = '5d628632-13f0-409f-8288-42b356cc033c'
//     //                     THEN jsonb_set(elem, '{std_primitive_i16}', '44'::jsonb)
					
//     //                     WHEN elem->>'id' = '63b83936-24c8-429b-ab67-ee2c76856f18'
//     //                     THEN jsonb_set(elem, '{std_primitive_i16}', '55'::jsonb)
					
//     //                     ELSE elem
//     //                 END
//     //             )
//     //             FROM jsonb_array_elements(sqlx_types_json_t_as_postgresql_json_b_not_null->'std_vec_vec_generic') AS elem
//     //             WHERE elem->>'id' <> '8cc5da73-1a7e-4ff4-9cfa-4f84998c62a4' and elem->>'id' <> '951240e0-990e-4cb8-909d-5183ff7725a4'
//     //      ) 
// 	// 	|| '[{"id": "1ff4db66-1395-4d58-bcf5-8bf69f1b90d3", "std_primitive_i16": 10}]'::jsonb
// 	// 	|| '[{"id": "847e5f32-d1a5-4d6a-9c55-040cbf60f229", "std_primitive_i16": 20}]'::jsonb
//     // )



//         //
//         let mut acc = std::string::String::from(jsonb_set_accumulator);
//         let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
//             true => std::string::String::default(),
//             false => format!("{jsonb_set_path},"),
//         };
//         //

//         // if self.update.is_empty() {
//         //     return Err(
//         //         DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed::FieldsIsEmpty {
//         //             code_occurence: error_occurence_lib::code_occurence!(),
//         //         },
//         //     );
//         // }
//         // {
//         //     let mut acc = vec![];
//         //     for element in &self.update {
//         //         match element {
//         //             DoggieOptionToUpdate::StdPrimitiveI16(_) => {
//         //                 let value = DoggieFieldToUpdate::StdPrimitiveI16;
//         //                 if acc.contains(&value) {
//         //                     return
//         //                     Err(DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed
//         //                     :: NotUniqueField
//         //                     {
//         //                         field : value, code_occurence : error_occurence_lib ::
//         //                         code_occurence! (),
//         //                     },);
//         //                 } else {
//         //                     acc.push(value);
//         //                 }
//         //             }
//         //             DoggieOptionToUpdate::Generic(_) => {
//         //                 let value = DoggieFieldToUpdate::Generic;
//         //                 if acc.contains(&value) {
//         //                     return
//         //                     Err(DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed
//         //                     :: NotUniqueField
//         //                     {
//         //                         field : value, code_occurence : error_occurence_lib ::
//         //                         code_occurence! (),
//         //                     },);
//         //                 } else {
//         //                     acc.push(value);
//         //                 }
//         //             }
//         //         }
//         //     }
//         // }


        
// //         match &is_array_object_element {
// //             postgresql_crud::ArrayObjectElementOrSimple::ArrayObjectElement {
// //                 jsonb_set_path,
// //                 index
// //             } => {
// //                 let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
// //                     true => std::string::String::default(),
// //                     false => format!("{jsonb_set_path}"),
// //                 };
// //                 let mut acc = std::string::String::default();
// //                 for element in &self.update {
// //                     match &element {
// //                         DoggieOptionToUpdate::StdPrimitiveI16(_) => match increment.checked_add(1) {
// //                             Some(value) => {
// //                                 *increment = value;
// //                                 acc.push_str(&format!("'std_primitive_i16',${increment}"));
// //                             }
// //                             None => {
// //                                 return Err(
// //                                     DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd {
// //                                         code_occurence: error_occurence_lib::code_occurence!(),
// //                                     },
// //                                 );
// //                             }
// //                         },
// //                         DoggieOptionToUpdate::Generic(value) => {
// //                             match value.value.try_generate_bind_increments(
// //                                 &format!("{jsonb_set_target}->{index}->'generic'"),
// //                                 &format!("{jsonb_set_target}->{index}->'generic'"),
// //                                 "",
// //                                 increment,
// //                                 is_array_object_element.clone()
// //                             ) {
// //                                 Ok(value) => {
// //                                     acc.push_str(&format!("'generic', {value}"));
// //                                 }
// //                                 Err(error) => {
// //                                     return Err(
// //                                         DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed::Cat {
// //                                             cat: error,
// //                                             code_occurence: error_occurence_lib::code_occurence!(),
// //                                         },
// //                                     );
// //                                 }
// //                             }
// //                         }
// //                     }
// //                     acc.push_str(",");
// //                 }
// //                 let _ = acc.pop();
// //                 let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
// //                     true => std::string::String::default(),
// //                     false => format!("{jsonb_set_path}"),
// //                 };
// //                 Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{previous_jsonb_set_path}}}',jsonb_build_object({acc}))"))
// //             },
// //             postgresql_crud::ArrayObjectElementOrSimple::Simple => {
// //                 let mut acc = std::string::String::from(jsonb_set_accumulator);
// //                 let previous_jsonb_set_path = match jsonb_set_path.is_empty() {
// //                     true => std::string::String::default(),
// //                     false => format!("{jsonb_set_path},"),
// //                 };
// //                 for element in &self.update {
// //                     match &element {
// //                         DoggieOptionToUpdate::StdPrimitiveI16(_) => match increment.checked_add(1) {
// //                             Some(value) => {
// //                                 *increment = value;
// //                                 acc = format!(
// //                                     "jsonb_set({acc},'{{{previous_jsonb_set_path}std_primitive_i16}}',${increment})"
// //                                 );
// //                             }
// //                             None => {
// //                                 return Err(
// //                                     DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd {
// //                                         code_occurence: error_occurence_lib::code_occurence!(),
// //                                     },
// //                                 );
// //                             }
// //                         },
// //                         DoggieOptionToUpdate::Generic(value) => {
// //                             match value.value.try_generate_bind_increments(
// //                                 &acc,
// //                                 &format!("{jsonb_set_target}->'generic'"),
// //                                 "generic",
// //                                 increment,
// //                                 is_array_object_element.clone(),
// //                             )
// //                             {
// //                                 Ok(value) => {
// //                                     acc = value;
// //                                 }
// //                                 Err(error) => {
// //                                     return Err(
// //                                         DoggieOptionsToUpdateTryGenerateBindIncrementsErrorNamed::Cat {
// //                                             cat: error,
// //                                             code_occurence: error_occurence_lib::code_occurence!(),
// //                                         },
// //                                     );
// //                                 }
// //                             }
// //                         }
// //                     }
// //                     //
// // // //         CASE
// // // //             WHEN jsonb_array_length(data->'one') > 2 THEN
// // // //                 jsonb_set(
// // // //                     data->'one',
// // // //                     '{2}',
// // // //                     '{"five": "fivevalueupdated"}'::jsonb
// // // //                 )
// // // //             ELSE
// // // //                 -- If the array is shorter, extend it and then set the element at the new index
// // // //                 (data->'one') || jsonb_build_array(
// // // //                     (jsonb_build_array(NULL, NULL, NULL) -- Extend the array to have the required index
// // // //                     || jsonb_build_array(NULL, NULL, '{"five": "fivevalueupdated"}'::jsonb))
// // // //                 )
// // // //         END AS new_array
// //                     //
// //                 }
// //                 Ok(acc)
// //             }
// //         }
//     }
//     fn bind_value_to_query<'a>(
//         self,
//         // mut 
//         query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
//     ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         // for element in self.update {
//         //     match element {
//         //         DoggieOptionToUpdate::StdPrimitiveI16(value) => {
//         //             query = query.bind(sqlx::types::Json(value.value));
//         //         }
//         //         DoggieOptionToUpdate::Generic(value) => {
//         //             query = value.value.bind_value_to_query(query);
//         //         }
//         //     }
//         // }
//         query
//     }
// }
////

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
    #[serde(rename(serialize = "generic", deserialize = "generic"))]
    Generic,
}
impl error_occurence_lib::ToStdStringString for DoggieFieldToUpdate {
    fn to_std_string_string(&self) -> std::string::String {
        match &self {
            Self::StdPrimitiveI16 => "std_primitive_i16".to_owned(),
            Self::Generic => "generic".to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
enum DoggieOptionToUpdate {
    #[serde(rename(serialize = "std_primitive_i16", deserialize = "std_primitive_i16"))]
    StdPrimitiveI16(postgresql_crud::Value<std::primitive::i16>),
    // #[serde(rename(serialize = "generic", deserialize = "generic"))]
    // Generic(postgresql_crud::Value<CatOptionsToUpdate>),
}



#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DoggieOptionsToUpdateSSS {
    id: uuid::Uuid,
    update: std::vec::Vec<DoggieOptionToUpdate>,
}

#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DoggieOptionsToUpdate(postgresql_crud::JsonArrayElementChange<DoggieToCreate, DoggieOptionsToUpdateSSS>);



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
    // Cat {
    //     #[eo_error_occurence]
    //     cat: CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
}



// impl
//     postgresql_crud::GeneratePostgresqlQueryPartToUpdate<
//         CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed,
//     > for CatOptionsToUpdate
// {
//     fn try_generate_bind_increments(
//         &self,
//         jsonb_set_accumulator: &std::primitive::str,
//         jsonb_set_target: &std::primitive::str,
//         jsonb_set_path: &std::primitive::str,
//         increment: &mut std::primitive::u64,
//         is_array_object_element: postgresql_crud::ArrayObjectElementOrSimple,
//     ) -> Result<std::string::String, CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed> {
//         if self.0.is_empty() {
//             return Err(
//                 CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed::FieldsIsEmpty {
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 },
//             );
//         }
//         {
//             let mut acc = vec![];
//             for element in &self.0 {
//                 match element {
//                     CatOptionToUpdate::StdPrimitiveI32(_) => {
//                         let value = CatFieldToUpdate::StdPrimitiveI32;
//                         if acc.contains(&value) {
//                             return
//                             Err(CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed ::
//                             NotUniqueField
//                             {
//                                 field : value, code_occurence : error_occurence_lib ::
//                                 code_occurence! (),
//                             },);
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
//                         return Err(
//                             CatOptionsToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd {
//                                 code_occurence: error_occurence_lib::code_occurence!(),
//                             },
//                         );
//                     }
//                 },
//             }
//         }
//         Ok(acc)
//     }
//     fn bind_value_to_query<'a>(
//         self,
//         mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
//     ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
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


/////
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde :: Serialize,
//     serde :: Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
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
// enum CatOptionToUpdate {
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32(postgresql_crud::Value<std::primitive::i32>),
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
// pub struct CatOptionsToUpdate(std::vec::Vec<CatOptionToUpdate>);
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

/////

// //maybe should for to have an id field ?

// // UPDATE my_table
// // SET data = data || jsonb_build_object('id', gen_random_uuid()::text)
// // WHERE id = 1;

// // jsonb_set(
// //     sqlx_types_json_t_as_postgresql_json_b_not_null,
// //     '{std_vec_vec_generic, 20}',
// // 	jsonb_build_object(
// // 		'std_primitive_i16', 33,
// // 		'std_string_string', 'zzz33',
// // 		'std_option_option_std_primitive_i32', 233
// // 	)
// // )

// // // WITH updated_data AS (
// // //     SELECT
// // //         id,  -- Assuming you have an id or some unique identifier for your rows
// // //         data,
// // //         -- Construct the new array with the element at the specified index
// // //         CASE
// // //             WHEN jsonb_array_length(data->'one') > 2 THEN
// // //                 jsonb_set(
// // //                     data->'one',
// // //                     '{2}',
// // //                     '{"five": "fivevalueupdated"}'::jsonb
// // //                 )
// // //             ELSE
// // //                 -- If the array is shorter, extend it and then set the element at the new index
// // //                 (data->'one') || jsonb_build_array(
// // //                     (jsonb_build_array(NULL, NULL, NULL) -- Extend the array to have the required index
// // //                     || jsonb_build_array(NULL, NULL, '{"five": "fivevalueupdated"}'::jsonb))
// // //                 )
// // //         END AS new_array
// // //     FROM my_table
// // // )
// // // UPDATE my_table
// // // SET data = jsonb_set(data, '{one}', updated_data.new_array)
// // // FROM updated_data
// // // WHERE my_table.id = updated_data.id;

// // // update 
// // //   test 
// // // set 
// // //   jsoncolumn = jsonb_set(
// // //     jsonb_set(
// // //       jsoncolumn, 
// // //       '{std_vec_vec_generic,0}', 
// // //       jsonb_build_object(
// // //         'std_primitive_i16', '0',
// // // 		'std_option_option_std_primitive_i32', '0',
// // // 		'std_string_string', '0'
// // //       )
// // //     ), 
// // //     '{std_vec_vec_generic,1}', 
// // //     jsonb_build_object(
// // //       'std_primitive_i16', '1',
// // // 	  'std_option_option_std_primitive_i32', '1',
// // // 	  'std_string_string', jsonb_set(
// // //       	(jsoncolumn -> 'std_vec_vec_generic' -> 0), 
// // //       	'{std_vec_vec_generic,0,std_string_string}', 
// // //       	'"anotherbulshit"'
// // //       )
// // //     )
// // //   ) 
// // // where 
// // //   id = 1 returning id


// // UPDATE my_table
// // SET data = (
// //     SELECT
// //         CASE
// //             -- Check if the JSONB array contains an element with id = '90'
// //             WHEN data::jsonb @> '[{"id": "90"}]' THEN
// //                 -- If id = '90' exists, just update id = '74' and leave the rest
// //                 jsonb_agg(
// //                     CASE
// //                         WHEN elem->>'id' = '74' THEN jsonb_set(elem, '{cat}', '"b"')
// //                         ELSE elem
// //                     END
// //                 )
// //             ELSE
// //                 -- If id = '90' does not exist, update id = '74' and add the new element
// //                 jsonb_agg(
// //                     CASE
// //                         WHEN elem->>'id' = '74' THEN jsonb_set(elem, '{cat}', '"b"')
// //                         ELSE elem
// //                     END
// //                 ) || jsonb_build_object('id', '90', 'cat', 'w')
// //         END
// //     FROM (
// //         -- Expanding the JSONB array into individual elements
// //         SELECT jsonb_array_elements(data) AS elem
// //         FROM my_table
// //         WHERE id = my_table.id
// //     ) AS elements
// // )
// // WHERE id = 1;



// // UPDATE jsongeneric 
// // SET sqlx_types_json_t_as_postgresql_json_b_not_null = jsonb_set(
// //     jsonb_set(
// //       sqlx_types_json_t_as_postgresql_json_b_not_null, 
// //       '{std_primitive_i8}', 
// //       '9'::jsonb
// //     ), 
// //     '{std_vec_vec_generic,0}', 
// //     jsonb_set(
// //       jsonb_build_object(
// //         'std_primitive_i16', 
// //         '30'::jsonb, 
// //         'generic', 
// //         jsonb_set(
// //           sqlx_types_json_t_as_postgresql_json_b_not_null -> 'std_vec_vec_generic' -> 0 -> 'generic', 
// //           '{std_primitive_i32}', 
// //           '50'::jsonb
// //         )
// //       ),
// //       '{}', '{}', false
// //     )
// //   ) 
// // WHERE 
// //   std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 14 
// // RETURNING 
// //   std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;



// // UPDATE my_table
// // SET my_jsonb_column = jsonb_set(
// //     my_jsonb_column,
// //     '{items}', -- path to the array
// //     (
// //         SELECT jsonb_agg(elem)
// //         FROM jsonb_array_elements(my_jsonb_column->'items') elem
// //         WHERE elem != '3'
// //     )
// // )
// // WHERE id = 1; -- or any condition that matches your row


// // UPDATE my_table
// // SET my_jsonb_column = jsonb_set(
// //     my_jsonb_column,
// //     '{f}',  -- Path to the key where UUID will be inserted
// //     to_jsonb(gen_random_uuid())  -- Convert UUID to JSONB format
// // )
// // WHERE id = 1;  -- Condition to specify which row to update

// ////
//todo must be id: "8769696599965", not id: { "value": "6858658658758" }
//  {
//     "Desirable": {
//       "sqlx_types_json_t_as_postgresql_json_b_not_null": {
//         "value": {
//           "std_primitive_i8": {
//             "value": 8
//           },
//           "std_vec_vec_generic": {
//             "value": [
//               {
//                 "id": {
//                   "value": "7cdbb3e6-8d89-44f8-85d9-4b1ef98aaea5"
//                 },
//                 "std_primitive_i16": {
//                   "value": 0
//                 }
//               },
//               {
//                 "id": {
//                   "value": "71532648-95dc-456b-9158-3ff430768e85"
//                 },
//                 "std_primitive_i16": {
//                   "value": 0
//                 }
//               }
//             ]
//           }
//         }
//       }
//     }
//   }