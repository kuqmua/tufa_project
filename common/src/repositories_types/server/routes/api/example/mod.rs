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
pub struct Example {
    // pub std_primitive_i16_as_small_int2_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2Nullable,
    // pub std_primitive_i16_as_small_int2_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull,

    // pub std_primitive_i32_as_int4_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlInt4Nullable,
    // pub std_primitive_i32_as_int4_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlInt4NotNull,

    // pub std_primitive_i64_as_big_int8_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlInt8Nullable,
    // pub std_primitive_i64_as_big_int8_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlInt8NotNull,

    // pub std_primitive_f32_as_float4_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4Nullable,
    // pub std_primitive_f32_as_float4_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4NotNull,

    // pub std_primitive_f64_as_float8_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8Nullable,
    // pub std_primitive_f64_as_float8_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8NotNull,

    // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i16_as_small_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresqlNotNull,
    // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i32_as_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresqlNotNull,
    // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i64_as_big_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull,

    #[generate_postgresql_crud_primary_key]
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull,

    // pub sqlx_postgres_types_pg_money_as_money_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNullable,
    // pub sqlx_postgres_types_pg_money_as_money_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,

    // pub sqlx_types_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNullable,
    // pub sqlx_types_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_big_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNullable,
    // pub sqlx_types_big_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNotNull,

    // pub std_primitive_bool_as_bool_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNullable,
    // pub std_primitive_bool_as_bool_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    // pub std_string_string_as_char_n_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNNullable,
    // pub std_string_string_as_char_n_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNNotNull,

    // pub std_string_string_as_varchar_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarcharNullable,
    // pub std_string_string_as_varchar_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarcharNotNull,

    // pub std_string_string_as_text_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlTextNullable,
    // pub std_string_string_as_text_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlTextNotNull,

    // // todo need to install postgresql extension
    // // pub std_string_string_as_ci_text_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCiTextNullable,
    // // pub std_string_string_as_ci_text_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCiTextNotNull,

    // // todo need to install postgresql extension
    // // pub sqlx_postgres_types_pg_ci_text_as_ci_text_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNullable,
    // // pub sqlx_postgres_types_pg_ci_text_as_ci_text_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,

    // pub std_vec_vec_std_primitive_u8_as_bytea_nullable: postgresql_crud::postgresql_type::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNullable,
    // pub std_vec_vec_std_primitive_u8_as_bytea_not_null: postgresql_crud::postgresql_type::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,

    // pub sqlx_types_time_date_as_date_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNullable,
    // pub sqlx_types_time_date_as_date_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNotNull,

    // pub sqlx_types_chrono_naive_date_as_date_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNullable,
    // pub sqlx_types_chrono_naive_date_as_date_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,

    // pub sqlx_types_chrono_naive_time_as_time_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNullable,
    // pub sqlx_types_chrono_naive_time_as_time_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,

    // pub sqlx_types_time_time_as_time_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNullable,
    // pub sqlx_types_time_time_as_time_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

    // pub sqlx_postgres_types_pg_interval_as_interval_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNullable,
    // pub sqlx_postgres_types_pg_interval_as_interval_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,

    // pub pg_range_std_primitive_i32_as_int4_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNullable,
    // pub pg_range_std_primitive_i32_as_int4_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,

    // pub pg_range_std_primitive_i64_as_int8_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNullable,
    // pub pg_range_std_primitive_i64_as_int8_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,

    // pub pg_range_naive_date_time_as_ts_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNullable,
    // pub pg_range_naive_date_time_as_ts_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub pg_range_time_primitive_date_time_as_ts_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNullable,
    // pub pg_range_time_primitive_date_time_as_ts_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub pg_range_date_time_utc_as_ts_tz_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNullable,
    // pub pg_range_date_time_utc_as_ts_tz_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull,

    // pub pg_range_time_offset_date_time_as_ts_tz_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNullable,
    // pub pg_range_time_offset_date_time_as_ts_tz_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull,

    // pub pg_range_date_time_local_as_ts_tz_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNullable,
    // pub pg_range_date_time_local_as_ts_tz_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull,

    // pub pg_range_chrono_naive_date_as_date_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNullable,
    // pub pg_range_chrono_naive_date_as_date_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,

    // pub pg_range_sqlx_types_time_date_as_date_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNullable,
    // pub pg_range_sqlx_types_time_date_as_date_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,

    // pub pg_range_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNullable,
    // pub pg_range_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,

    // pub pg_range_sqlx_types_big_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNullable,
    // pub pg_range_sqlx_types_big_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,

    // pub chrono_naive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNullable,
    // pub chrono_naive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_time_primitive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNullable,
    // pub sqlx_types_time_primitive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,

    // pub time_offset_date_time_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNullable,
    // pub time_offset_date_time_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull,

    // pub chrono_date_time_chrono_utc_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNullable,
    // pub chrono_date_time_chrono_utc_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,

    // pub chrono_date_time_chrono_local_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNullable,
    // pub chrono_date_time_chrono_local_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,

    // pub uuid_uuid_as_uuid_v4_init_by_postgres_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresqlNotNull,
    // #[generate_postgresql_crud_primary_key] //todo maybe later support something else but now i think uuid v4 and v7 is enough
    // pub uuid_uuid_as_uuid_v4_init_by_postgres_not_null_prime: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresqlNotNull,

    // pub uuid_uuid_as_uuid_init_by_client_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNullable,
    // pub uuid_uuid_as_uuid_init_by_client_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNotNull,

    // pub sqlx_types_ipnetwork_ip_network_as_inet_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNullable,
    // pub sqlx_types_ipnetwork_ip_network_as_inet_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,

    // pub sqlx_types_ipnetwork_ip_network_as_cidr_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNullable,
    // pub sqlx_types_ipnetwork_ip_network_as_cidr_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

    // pub mac_address_mac_address_as_mac_addr_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNullable,
    // pub mac_address_mac_address_as_mac_addr_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,

    // pub sqlx_types_bit_vec_as_bit_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNullable,
    // pub sqlx_types_bit_vec_as_bit_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNotNull,

    // pub sqlx_types_bit_vec_as_varbit_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNullable,
    // pub sqlx_types_bit_vec_as_varbit_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNotNull,

    // pub object_animal_as_jsonb_nullable: ObjectAnimalAsPostgresqlJsonbNullable,
    pub object_animal_as_jsonb_not_null: ObjectAnimalAsPostgresqlJsonbNotNull,

    // pub option_object_animal_as_jsonb_nullable: StdOptionOptionObjectAnimalAsPostgresqlJsonbNullable,
    // pub option_object_animal_as_jsonb_not_null: StdOptionOptionObjectAnimalAsPostgresqlJsonbNotNull,

    // pub std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdVecVecObjectWithIdAnimalAsPostgresqlJsonbNullable,
    // pub std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,

    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonbNullable,
    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,
    
    //63max
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
    // postgresql_crud::GeneratePostgresqlJsonType
)] //user type must implement utoipa::ToSchema trait
pub struct Animal {
    // pub id: postgresql_crud::postgresql_json_type::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    //todo check field max length in postgresql
    pub std_primitive_i8: postgresql_crud::postgresql_json_type::StdPrimitiveI8,
    // pub std_primitive_i16: postgresql_crud::postgresql_json_type::StdPrimitiveI16,
    // pub std_primitive_i32: postgresql_crud::postgresql_json_type::StdPrimitiveI32,
    // pub std_primitive_i64: postgresql_crud::postgresql_json_type::StdPrimitiveI64,
    // pub std_primitive_u8: postgresql_crud::postgresql_json_type::StdPrimitiveU8,
    // pub std_primitive_u16: postgresql_crud::postgresql_json_type::StdPrimitiveU16,
    // pub std_primitive_u32: postgresql_crud::postgresql_json_type::StdPrimitiveU32,
    // pub std_primitive_u64: postgresql_crud::postgresql_json_type::StdPrimitiveU64,
    // pub std_primitive_f32: postgresql_crud::postgresql_json_type::StdPrimitiveF32,
    // pub std_primitive_f64: postgresql_crud::postgresql_json_type::StdPrimitiveF64,
    // pub std_primitive_bool: postgresql_crud::postgresql_json_type::StdPrimitiveBool,
    // pub std_string_string: postgresql_crud::postgresql_json_type::StdStringString,
    // pub std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI8,
    // pub std_option_option_std_primitive_i16: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_primitive_i32: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_primitive_i64: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_primitive_u8: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_primitive_u16: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_primitive_u32: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_primitive_u64: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_primitive_f32: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_primitive_f64: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_primitive_bool: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_string_string: postgresql_crud::postgresql_json_type::StdOptionOptionStdStringString,
    // pub std_vec_vec_std_primitive_i8: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI8,
    // pub std_vec_vec_std_primitive_i16: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI16,
    // pub std_vec_vec_std_primitive_i32: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI32,
    // pub std_vec_vec_std_primitive_i64: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI64,
    // pub std_vec_vec_std_primitive_u8: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU8,
    // pub std_vec_vec_std_primitive_u16: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU16,
    // pub std_vec_vec_std_primitive_u32: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU32,
    // pub std_vec_vec_std_primitive_u64: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU64,
    // pub std_vec_vec_std_primitive_f32: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveF32,
    // pub std_vec_vec_std_primitive_f64: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveF64,
    // pub std_vec_vec_std_primitive_bool: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveBool,
    // pub std_vec_vec_std_string_string: postgresql_crud::postgresql_json_type::StdVecVecStdStringString,
    // pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI8,
    // pub std_option_option_std_vec_vec_std_primitive_i16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_primitive_i32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_primitive_i64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_primitive_u8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_primitive_u16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_primitive_u32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_primitive_u64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_primitive_f32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_primitive_f64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_primitive_bool: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_string_string: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdStringString,
    // pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI8,
    // pub std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_vec_vec_std_option_option_std_string_string: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdStringString,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_option_option_std_string_string: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdStringString,

    // pub object_doggie: ObjectDoggie,
    // pub std_option_option_object: StdOptionOptionObjectDoggie,
    // pub std_vec_vec_object: StdVecVecObjectWithIdDoggie,
    // pub std_option_option_std_vec_vec_object: StdOptionOptionStdVecVecObjectWithIdDoggie,

    // pub std_vec_vec_object_with_id: StdVecVecObjectWithIdDoggie,
    // pub std_option_option_std_vec_vec_object_with_id: StdOptionOptionStdVecVecObjectWithIdDoggie
}
// /////////////////////////////////////////
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud::GeneratePostgresqlJsonType
// )] //user type must implement utoipa::ToSchema trait
// pub struct Doggie {
//     pub std_primitive_i8: postgresql_crud::postgresql_json_type::StdPrimitiveI8,
//     pub std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI8,
//     pub std_vec_vec_std_primitive_i8: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI8,
//     pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
// }
//////////////////////////////////////////////
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadManyPayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<postgresql_crud::postgresql_type::postgresql_type::PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere>,
    pub object_animal_as_jsonb_not_null: std::option::Option<PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullWhere>,
    pub select: std::vec::Vec<PostgresqlTypeExampleColumn>,
    pub order_by: postgresql_crud::OrderBy<PostgresqlTypeExampleColumn>,
    pub pagination: postgresql_crud::Pagination,
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
        not_unique_column: PostgresqlTypeExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueObjectAnimalAsJsonbNotNull {
        object_animal_as_jsonb_not_null: std::string::String,
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
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueObjectAnimalAsJsonbNotNull { object_animal_as_jsonb_not_null, code_occurence } => Self::NotUniqueObjectAnimalAsJsonbNotNull { object_animal_as_jsonb_not_null, code_occurence },
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
        not_unique_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: PostgresqlTypeExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueObjectAnimalAsJsonbNotNull {
        #[eo_to_std_string_string]
        object_animal_as_jsonb_not_null: ObjectAnimalAsPostgresqlJsonbNotNull,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2560,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2632,
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
                        PostgresqlTypeExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(value) => {
                            <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::postgresql_type_self_column_query_part(
                                value,
                                "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                            )
                        }
                        PostgresqlTypeExampleColumn::ObjectAnimalAsJsonbNotNull(value) => <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::postgresql_type_self_column_query_part(value, "object_animal_as_jsonb_not_null"),
                    });
                    value.push_str(",");
                }
                let _ = value.pop();
                value
            },
            {
                let mut increment: std::primitive::u64 = 0;
                let mut additional_parameters = std::string::String::from("where");
                let mut is_first_push_to_additional_parameters_already_happend = false;
                if let Some(value) = &parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
                    match <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::postgresql_type_self_where_try_generate_bind_increments(
                        value,
                        &mut increment,
                        &"std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                        is_first_push_to_additional_parameters_already_happend,
                    ) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            panic!()
                        }
                    }
                }
                if let Some(value) = &parameters.payload.object_animal_as_jsonb_not_null {
                    match <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::postgresql_type_self_where_try_generate_bind_increments(
                        value,
                        &mut increment,
                        &"object_animal_as_jsonb_not_null",
                        is_first_push_to_additional_parameters_already_happend,
                    ) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            panic!()
                        }
                    }
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
                    let value = match postgresql_crud::BindQuery::try_generate_bind_increments(&parameters.payload.pagination, &mut increment) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                bind_query: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 3677,
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
                    additional_parameters.push_str(&format!("{}{}", prefix, value));
                }
                additional_parameters
            }
        )
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
            query = <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::postgresql_type_self_where_bind_value_to_query(value, query);
        }
        if let Some(value) = parameters.payload.object_animal_as_jsonb_not_null {
            query = <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::postgresql_type_self_where_bind_value_to_query(value, query);
        }
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.pagination, query);
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2582,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2582,
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
                        let mut std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_type::postgresql_type::PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToRead>> = None;
                        let mut object_animal_as_jsonb_not_null: std::option::Option<postgresql_crud::Value<PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToRead>> = None;
                        for element in &parameters.payload.select {
                            // println!("11111");
                            match element {
                                PostgresqlTypeExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(_) => {
                                    match sqlx::Row::try_get::<postgresql_crud::postgresql_type::postgresql_type::PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToRead, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
                                        Ok(value) => {
                                            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = Some(postgresql_crud::Value { value: value });
                                        }
                                        Err(error_0) => {
                                            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                                postgresql: error_0,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!(),
                                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                        line: 1248,
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
                                PostgresqlTypeExampleColumn::ObjectAnimalAsJsonbNotNull(_) => match sqlx::Row::try_get::<PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToRead, &std::primitive::str>(&value, "object_animal_as_jsonb_not_null") {
                                    Ok(value) => {
                                        object_animal_as_jsonb_not_null = Some(postgresql_crud::Value { value: value });
                                    }
                                    Err(error_0) => {
                                        let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                            postgresql: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                    line: 1283,
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
                            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                            object_animal_as_jsonb_not_null,
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
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3767,
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
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ReadManyPayload {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: Some(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            object_animal_as_jsonb_not_null: Some(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            select: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            order_by: postgresql_crud::OrderBy {
                column: PostgresqlTypeExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(
                    postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                ),
                order: Some(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            },
            pagination: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn read_many_payload_example_route_logic() -> axum::response::Response {
    println!("@@");
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <ReadManyPayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    println!("@@1111");
    //here
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
///////////////////////
///////////////////////
///////////////////////
///////////////////////
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
struct AnimalToCreateOrigin {
    std_primitive_i8: postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8ToCreate,
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalToCreateOrigin {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl AnimalToCreateOrigin {
    fn try_generate_postgresql_json_type_to_create(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_json_type_to_create(&self.std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.std_primitive_i8, query);
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
pub struct AnimalToCreateWithGeneratedId(AnimalToCreateOrigin);
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
pub struct AnimalToCreateWithoutGeneratedId(AnimalToCreateOrigin);
impl AnimalToCreateWithGeneratedId {
    pub fn new(std_primitive_i8: postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8ToCreate) -> Self {
        Self(AnimalToCreateOrigin { std_primitive_i8 })
    }
}
impl AnimalToCreateWithoutGeneratedId {
    pub fn new(std_primitive_i8: postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8ToCreate) -> Self {
        Self(AnimalToCreateOrigin { std_primitive_i8 })
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalToCreateWithGeneratedId {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalToCreateWithoutGeneratedId {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl AnimalToCreateWithGeneratedId {
    fn try_generate_postgresql_json_type_to_create(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed> {
        match self.0.try_generate_postgresql_json_type_to_create(increment) {
            Ok(value) => Ok(format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{value}")),
            Err(error) => Err(error),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.0.bind_value_to_postgresql_query_part_to_create(query);
        query
    }
}
impl AnimalToCreateWithoutGeneratedId {
    fn try_generate_postgresql_json_type_to_create(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed> {
        match self.0.try_generate_postgresql_json_type_to_create(increment) {
            Ok(value) => Ok(value),
            Err(error) => Err(error),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.0.bind_value_to_postgresql_query_part_to_create(query);
        query
    }
}
impl<'a> postgresql_crud::BindQuery<'a> for AnimalToCreateWithoutGeneratedId {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_json_type_to_create(&self.0.std_primitive_i8, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("std_primitive_i8", &value));
            }
            Err(error) => {
                return Err(error.into());
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0.std_primitive_i8, query);
        query
    }
}
impl postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToCreateTraits<'_> for AnimalToCreateWithoutGeneratedId {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalFieldToReadWithoutId {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8FieldReader),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalFieldToReadWithId {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id(postgresql_crud::postgresql_json_type::PostgresqlJsonTypeUuidFieldReader),
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8FieldReader),
}
impl error_occurence_lib::ToStdStringString for AnimalFieldToReadWithoutId {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalFieldToReadWithoutId {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![AnimalFieldToReadWithoutId::StdPrimitiveI8(
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        )]
    }
}
impl error_occurence_lib::ToStdStringString for AnimalFieldToReadWithId {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalFieldToReadWithId {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalFieldToReadWithId::Id(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdPrimitiveI8(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeAnimalOptionsToReadWithoutId {
    #[serde(skip_serializing_if = "Option::is_none")]
    std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeAnimalOptionsToReadWithId {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeUuidOptionsToRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeAnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl PostgresqlJsonTypeAnimalOptionsToReadWithoutId {
    pub fn try_new(std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>) -> Result<Self, PostgresqlJsonTypeAnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed> {
        if let (None) = (&std_primitive_i8) {
            return Err(PostgresqlJsonTypeAnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { std_primitive_i8 })
    }
}
impl PostgresqlJsonTypeAnimalOptionsToReadWithId {
    pub fn try_new(
        id: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeUuidOptionsToRead>>,
        std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>,
    ) -> Result<Self, PostgresqlJsonTypeAnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed> {
        if let (None, None) = (&id, &std_primitive_i8) {
            return Err(PostgresqlJsonTypeAnimalOptionsToReadWithOrWithoutIdTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { id, std_primitive_i8 })
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlJsonTypeAnimalOptionsToReadWithoutId {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "std_primitive_i8" => serde::__private::Ok(__Field::__field0),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"std_primitive_i8" => serde::__private::Ok(__Field::__field0),
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
            marker: serde::__private::PhantomData<PostgresqlJsonTypeAnimalOptionsToReadWithoutId>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PostgresqlJsonTypeAnimalOptionsToReadWithoutId;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeAnimalOptionsToRead")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 1 elements"));
                    }
                };
                match PostgresqlJsonTypeAnimalOptionsToReadWithoutId::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i8"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("std_primitive_i8")?,
                };
                match PostgresqlJsonTypeAnimalOptionsToReadWithoutId::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["std_primitive_i8"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "PostgresqlJsonTypeAnimalOptionsToReadWithoutId",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<PostgresqlJsonTypeAnimalOptionsToReadWithoutId>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlJsonTypeAnimalOptionsToReadWithId {
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
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
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
                    "std_primitive_i8" => serde::__private::Ok(__Field::__field1),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"id" => serde::__private::Ok(__Field::__field0),
                    b"std_primitive_i8" => serde::__private::Ok(__Field::__field1),
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
            marker: serde::__private::PhantomData<PostgresqlJsonTypeAnimalOptionsToReadWithId>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PostgresqlJsonTypeAnimalOptionsToReadWithId;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeAnimalOptionsToRead")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeUuidOptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeAnimalOptionsToRead with 2 elements"));
                    }
                };
                match PostgresqlJsonTypeAnimalOptionsToReadWithId::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeUuidOptionsToRead>>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("id"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeUuidOptionsToRead>>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i8"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionsToRead>>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => serde::__private::de::missing_field("id")?,
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::de::missing_field("std_primitive_i8")?,
                };
                match PostgresqlJsonTypeAnimalOptionsToReadWithId::try_new(__field0, __field1) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["id", "std_primitive_i8"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "PostgresqlJsonTypeAnimalOptionsToReadWithId",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<PostgresqlJsonTypeAnimalOptionsToReadWithId>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeAnimalOptionsToReadWithoutId {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        }
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeAnimalOptionsToReadWithId {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            id: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalFieldToUpdate {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8,
}
impl error_occurence_lib::ToStdStringString for AnimalFieldToUpdate {
    fn to_std_string_string(&self) -> std::string::String {
        match &self {
            Self::StdPrimitiveI8 => "std_primitive_i8".to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum PostgresqlJsonTypeAnimalOptionToUpdateOrigin {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::Value<postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionToUpdate>),
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalJsonArrayChangeTryGeneratePostgresqlJsonTypeErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Create {
        #[eo_error_occurence]
        error: postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeAnimalOptionToUpdateOrigin {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(postgresql_crud::Value {
            value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        })]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalOptionsToUpdate {
    pub id: postgresql_crud::postgresql_json_type::PostgresqlJsonTypeUuidOptionToUpdate,
    pub fields: PostgresqlJsonTypeAnimalOptionToUpdate,
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for AnimalOptionsToUpdate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            fields: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl std::fmt::Display for Animal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", &self)
    }
}
pub type PostgresqlJsonTypeAnimalToCreate = AnimalToCreateWithoutGeneratedId;
pub type AnimalFieldToRead = AnimalFieldToReadWithoutId;
pub type PostgresqlJsonTypeAnimalOptionsToRead = PostgresqlJsonTypeAnimalOptionsToReadWithoutId;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeAnimalFieldReader(std::vec::Vec<AnimalFieldToReadWithoutId>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeAnimalFieldReaderTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithoutId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeAnimalFieldReader {
    pub fn try_new(value: std::vec::Vec<AnimalFieldToReadWithoutId>) -> Result<Self, PostgresqlJsonTypeAnimalFieldReaderTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlJsonTypeAnimalFieldReaderTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(PostgresqlJsonTypeAnimalFieldReaderTryNewErrorNamed::NotUniqueFieldFilter {
                    field: element,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            } else {
                unique.push(element);
            }
        }
        Ok(Self(unique))
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlJsonTypeAnimalFieldReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<PostgresqlJsonTypeAnimalFieldReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PostgresqlJsonTypeAnimalFieldReader;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct PostgresqlJsonTypeAnimalFieldReader")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalFieldToReadWithoutId> = <std::vec::Vec<AnimalFieldToReadWithoutId> as serde::Deserialize>::deserialize(__e)?;
                match PostgresqlJsonTypeAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalFieldToReadWithoutId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct PostgresqlJsonTypeAnimalFieldReader with 1 element"));
                    }
                };
                match PostgresqlJsonTypeAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "PostgresqlJsonTypeAnimalFieldReader",
            __Visitor {
                marker: serde::__private::PhantomData::<PostgresqlJsonTypeAnimalFieldReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeAnimalFieldReader {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl postgresql_crud::GeneratePostgresqlJsonTypeToRead for AnimalFieldToReadWithoutId {
    fn generate_postgresql_json_type_to_read_from_vec(value: &std::vec::Vec<Self>, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
        let mut acc = std::string::String::default();
        for element in value {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithoutId::StdPrimitiveI8(value) =>
                        <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_json_type_to_read(value, "std_primitive_i8", column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message, false,),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        format!("{acc}")
    }
}
pub type AnimalReader = PostgresqlJsonTypeAnimalOptionsToRead;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeAnimalOptionToUpdate(std::vec::Vec<PostgresqlJsonTypeAnimalOptionToUpdateOrigin>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed {
    FieldsAreEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeAnimalOptionToUpdate {
    pub fn try_new(value: std::vec::Vec<PostgresqlJsonTypeAnimalOptionToUpdateOrigin>) -> Result<Self, PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed::FieldsAreEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            let generate_not_unique_field = |value: &std::primitive::str| format!("not unique {value} field");
            for element in &value {
                match element {
                    PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryNewErrorNamed::NotUniqueFieldStdPrimitiveI8 {
                                error: generate_not_unique_field("std_primitive_i8"),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        } else {
                            acc.push(value);
                        }
                    }
                }
            }
        }
        Ok(Self(value))
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlJsonTypeAnimalOptionToUpdate {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<PostgresqlJsonTypeAnimalOptionToUpdate>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PostgresqlJsonTypeAnimalOptionToUpdate;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct PostgresqlJsonTypeAnimalOptionToUpdate")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<PostgresqlJsonTypeAnimalOptionToUpdateOrigin> = <std::vec::Vec<PostgresqlJsonTypeAnimalOptionToUpdateOrigin> as serde::Deserialize>::deserialize(__e)?;
                match PostgresqlJsonTypeAnimalOptionToUpdate::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<PostgresqlJsonTypeAnimalOptionToUpdateOrigin>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct PostgresqlJsonTypeAnimalOptionToUpdate with 1 element"));
                    }
                };
                match PostgresqlJsonTypeAnimalOptionToUpdate::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "PostgresqlJsonTypeAnimalOptionToUpdate",
            __Visitor {
                marker: serde::__private::PhantomData::<PostgresqlJsonTypeAnimalOptionToUpdate>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeAnimalOptionToUpdate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeAnimalOptionToUpdate {
    fn try_generate_postgresql_json_type_to_update(
        &self,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed> {
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        let generate_jsonb_set_path = |value: &std::primitive::str| {
            let previous = match jsonb_set_path.is_empty() {
                true => std::string::String::default(),
                false => format!("{jsonb_set_path},"),
            };
            format!("{previous}{value}")
        };
        let mut local_acc = format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end)");
        for element in &self.0 {
            match &element {
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                    match <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_json_type_to_update(&value.value, &local_acc, &generate_jsonb_set_target("std_primitive_i8"), &generate_jsonb_set_path("std_primitive_i8"), increment) {
                        Ok(value) => {
                            local_acc = value;
                        }
                        Err(error) => {
                            return Err(PostgresqlJsonTypeAnimalOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed::StdPrimitiveI8 {
                                error,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                }
            }
        }
        Ok(local_acc)
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0 {
            match element {
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                    query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
            }
        }
        query
    }
}
impl<'a> postgresql_crud::BindQuery<'a> for PostgresqlJsonTypeAnimalOptionToUpdate {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        panic!()
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        panic!()
    }
}
impl postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToUpdateTraits<'_> for PostgresqlJsonTypeAnimalOptionToUpdate {}
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
pub struct ObjectAnimal {
    pub std_primitive_i8: postgresql_crud::postgresql_json_type::StdPrimitiveI8,
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ObjectAnimal {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
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
pub struct PostgresqlJsonTypeObjectAnimalToCreate(pub AnimalToCreateWithoutGeneratedId);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeObjectAnimalToCreate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
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
pub struct PostgresqlJsonTypeObjectAnimalOptionsToRead(pub PostgresqlJsonTypeAnimalOptionsToReadWithoutId);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeObjectAnimalOptionsToRead {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    utoipa ::
ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeObjectAnimalFieldReader(std::vec::Vec<AnimalFieldToReadWithoutId>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeObjectAnimalFieldReaderTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithoutId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeObjectAnimalFieldReader {
    pub fn try_new(value: std::vec::Vec<AnimalFieldToReadWithoutId>) -> Result<Self, PostgresqlJsonTypeObjectAnimalFieldReaderTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlJsonTypeObjectAnimalFieldReaderTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(PostgresqlJsonTypeObjectAnimalFieldReaderTryNewErrorNamed::NotUniqueFieldFilter {
                    field: element,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            } else {
                unique.push(element);
            }
        }
        Ok(Self(unique))
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeObjectAnimalFieldReader {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl<'de> serde::Deserialize<'de> for PostgresqlJsonTypeObjectAnimalFieldReader {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<PostgresqlJsonTypeObjectAnimalFieldReader>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PostgresqlJsonTypeObjectAnimalFieldReader;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct PostgresqlJsonTypeObjectAnimalFieldReader")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalFieldToReadWithoutId> = <std::vec::Vec<AnimalFieldToReadWithoutId> as serde::Deserialize>::deserialize(__e)?;
                match PostgresqlJsonTypeObjectAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalFieldToReadWithoutId>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct PostgresqlJsonTypeObjectAnimalFieldReader with 1 element"));
                    }
                };
                match PostgresqlJsonTypeObjectAnimalFieldReader::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "PostgresqlJsonTypeObjectAnimalFieldReader",
            __Visitor {
                marker: serde::__private::PhantomData::<PostgresqlJsonTypeObjectAnimalFieldReader>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
pub type PostgresqlJsonTypeObjectAnimalReader = PostgresqlJsonTypeObjectAnimalOptionsToRead;
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
pub struct PostgresqlJsonTypeObjectAnimalOptionToUpdate(pub PostgresqlJsonTypeAnimalOptionToUpdate);
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeObjectAnimalOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed {
    StdPrimitiveI8 {
        #[eo_error_occurence]
        error: postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8OptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeObjectAnimalOptionToUpdate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl postgresql_crud::PostgresqlJsonType for ObjectAnimal {
    type PostgresqlJsonTypeSelfToCreate<'a> = PostgresqlJsonTypeObjectAnimalToCreate;
    fn try_generate_postgresql_json_type_to_create(postgresql_json_type_self_to_create: &Self::PostgresqlJsonTypeSelfToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed> {
        postgresql_json_type_self_to_create.0.try_generate_postgresql_json_type_to_create(increment)
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(postgresql_json_type_self_to_create: Self::PostgresqlJsonTypeSelfToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        postgresql_json_type_self_to_create.0.bind_value_to_postgresql_query_part_to_create(query)
    }
    type PostgresqlJsonTypeSelfFieldReader<'a> = PostgresqlJsonTypeObjectAnimalFieldReader;
    type PostgresqlJsonTypeSelfOptionsToRead<'a> = PostgresqlJsonTypeObjectAnimalOptionsToRead;
    fn generate_postgresql_json_type_to_read(
        postgresql_json_type_self_field_reader: &Self::PostgresqlJsonTypeSelfFieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
        is_postgresql_type: std::primitive::bool,
    ) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = if is_postgresql_type { column_name_and_maybe_field_getter.to_string() } else { format!("{column_name_and_maybe_field_getter}->'{field_ident}'") };
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in &postgresql_json_type_self_field_reader.0 {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithoutId::StdPrimitiveI8(value) => <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::generate_postgresql_json_type_to_read(
                        value,
                        "std_primitive_i8",
                        &column_name_and_maybe_field_getter_field_ident,
                        &column_name_and_maybe_field_getter_for_error_message_field_ident,
                        false,
                    ),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type {
            format!("{acc}")
        } else {
            format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))")
        }
    }
    type PostgresqlJsonTypeSelfOptionToUpdate<'a> = PostgresqlJsonTypeObjectAnimalOptionToUpdate;
    type PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed = PostgresqlJsonTypeObjectAnimalOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed;
    fn try_generate_postgresql_json_type_to_update(
        postgresql_json_type_self_option_to_update: &Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed> {
        let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        for element in &postgresql_json_type_self_option_to_update.0 .0 {
            match element {
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                    match <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_json_type_to_update(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("std_primitive_i8"), "std_primitive_i8", increment) {
                        Ok(value) => {
                            std_option_option_object_acc = value;
                        }
                        Err(error) => {
                            return Err(PostgresqlJsonTypeObjectAnimalOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed::StdPrimitiveI8 {
                                error,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                }
            }
        }
        if jsonb_set_accumulator.is_empty() && jsonb_set_path.is_empty() {
            Ok(std_option_option_object_acc)
        } else {
            Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',{std_option_option_object_acc})"))
        }
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(
        postgresql_json_type_self_option_to_update: Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in postgresql_json_type_self_option_to_update.0 .0 {
            match element {
                PostgresqlJsonTypeAnimalOptionToUpdateOrigin::StdPrimitiveI8(value) => {
                    query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(value.value, query);
                }
            }
        }
        query
    }
}
#[derive(Debug, Clone, PartialEq, schemars :: JsonSchema)]
pub struct ObjectAnimalAsPostgresqlJsonbNotNull(PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToCreate);
impl std::fmt::Display for ObjectAnimalAsPostgresqlJsonbNotNull {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for ObjectAnimalAsPostgresqlJsonbNotNull {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl postgresql_crud::CreateTableColumnQueryPart for ObjectAnimalAsPostgresqlJsonbNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} JSONB NOT NULL check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(ObjectAnimalAsPostgresqlJsonbNotNull)).unwrap())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullColumn(PostgresqlJsonTypeObjectAnimalFieldReader);
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullColumn {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<PostgresqlJsonTypeObjectAnimalFieldReader> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullColumn {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<PostgresqlJsonTypeObjectAnimalFieldReader> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value.0)),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullColumn {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullColumn {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToCreate(PostgresqlJsonTypeObjectAnimalToCreate);
impl<'a> postgresql_crud::BindQuery<'a> for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToCreate {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        Ok(<ObjectAnimal as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_json_type_to_create(&self.0, increment).unwrap())
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        <ObjectAnimal as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_create(self.0, query)
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToCreate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToCreateTraits<'_> for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToCreate {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToRead(PostgresqlJsonTypeObjectAnimalOptionsToRead);
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<PostgresqlJsonTypeObjectAnimalOptionsToRead> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<PostgresqlJsonTypeObjectAnimalOptionsToRead> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value.0)),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToRead {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToUpdate(PostgresqlJsonTypeObjectAnimalOptionToUpdate);
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToUpdate {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToUpdateTraits<'_> for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToUpdate {}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToUpdateQueryPartErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Todo,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullWhereElement {
    logical_operator: postgresql_crud::LogicalOperator,
    std_primitive_i8: postgresql_crud::postgresql_json_type::PostgresqlJsonTypeStdPrimitiveI8FieldReader,
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullWhereElement {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_primitive_i8: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullWhereElement {}
impl postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        // todo!()
        Ok(String::from(" object_animal_as_jsonb_not_null->>'std_primitive_i8' = '0'"))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        // todo!()
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullWhere {
    logical_operator: postgresql_crud::LogicalOperator,
    value: std::vec::Vec<PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullWhereElement>,
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullWhere {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: vec![postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()],
        }
    }
}
impl postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'_> for ObjectAnimalAsPostgresqlJsonbNotNull {
    type PostgresqlTypeSelf = ObjectAnimalAsPostgresqlJsonbNotNull;
    type PostgresqlTypeSelfColumn = PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullColumn;
    fn postgresql_type_self_column_query_part(postgresql_type_self_column: &Self::PostgresqlTypeSelfColumn, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", <ObjectAnimal as postgresql_crud::PostgresqlJsonType>::generate_postgresql_json_type_to_read(&postgresql_type_self_column.0, &column, &column, &column, true))
    }
    type PostgresqlTypeSelfToCreate = PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToCreate;
    type PostgresqlTypeSelfToRead = PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToRead;
    type PostgresqlTypeSelfToUpdate = PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToUpdate;
    type PostgresqlTypeSelfToUpdateQueryPartErrorNamed = PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullToUpdateQueryPartErrorNamed;
    fn postgresql_type_self_to_update_query_part(
        postgresql_type_self_to_update: &Self::PostgresqlTypeSelfToUpdate,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlTypeSelfToUpdateQueryPartErrorNamed> {
        Ok(<ObjectAnimal as postgresql_crud::PostgresqlJsonType>::try_generate_postgresql_json_type_to_update(&postgresql_type_self_to_update.0, jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment).unwrap())
    }
    fn postgresql_type_self_to_update_bind_query_part<'a>(postgresql_type_self_to_update: Self::PostgresqlTypeSelfToUpdate, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        <ObjectAnimal as postgresql_crud::PostgresqlJsonType>::bind_value_to_postgresql_query_part_to_update(postgresql_type_self_to_update.0, query)
    }
    type PostgresqlTypeSelfWhereElement = PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullWhereElement;
    type PostgresqlTypeSelfWhere = PostgresqlTypeObjectAnimalAsPostgresqlJsonbNotNullWhere;
    fn postgresql_type_self_where_try_generate_bind_increments(
        postgresql_type_self_where: &Self::PostgresqlTypeSelfWhere,
        increment: &mut std::primitive::u64,
        column: &dyn std::fmt::Display,
        is_need_to_add_logical_operator: std::primitive::bool,
    ) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        for element in &postgresql_type_self_where.value {
            match postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
                Ok(value) => {
                    acc.push_str(&format!("{value} "));
                    is_need_to_add_logical_operator_inner_handle = true;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let _ = acc.pop();
        Ok(format!("{}({acc})", &postgresql_type_self_where.logical_operator.to_query_part(is_need_to_add_logical_operator)))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(postgresql_type_self_where: Self::PostgresqlTypeSelfWhere, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in postgresql_type_self_where.value {
            query = postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
        }
        query
    }
}