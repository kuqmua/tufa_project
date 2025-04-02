#[derive(Debug, postgresql_crud::GeneratePostgresqlCrud)]
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
    pub std_primitive_i16_as_small_int2_not_null: postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull,
    pub std_primitive_i16_as_small_int2_nullable: postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2Nullable,

    // pub std_primitive_i32_as_int4_not_null: postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlInt4NotNull,
    // pub std_primitive_i32_as_int4_nullable: postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlInt4Nullable,

    // pub std_primitive_i64_as_big_int8_not_null: postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlInt8NotNull,
    // pub std_primitive_i64_as_big_int8_nullable: postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlInt8Nullable,

    // pub std_primitive_f32_as_float4_not_null: postgresql_crud::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4NotNull,
    // pub std_primitive_f32_as_float4_nullable: postgresql_crud::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4Nullable,

    // pub std_primitive_f64_as_float8_not_null: postgresql_crud::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8NotNull,
    // pub std_primitive_f64_as_float8_nullable: postgresql_crud::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8Nullable,

    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i16_as_small_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresqlNotNull,
    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i32_as_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresqlNotNull,
    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i64_as_big_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull,
    #[generate_postgresql_crud_primary_key]
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull,

    // pub sqlx_postgres_types_pg_money_as_money_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,
    // pub sqlx_postgres_types_pg_money_as_money_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNullable,

    // pub sqlx_types_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNotNull,
    // pub sqlx_types_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNullable,

    // pub sqlx_types_big_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNotNull,
    // pub sqlx_types_big_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNullable,

    // pub std_primitive_bool_as_bool_not_null: postgresql_crud::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull,
    // pub std_primitive_bool_as_bool_nullable: postgresql_crud::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNullable,

    // pub std_string_string_as_char_n_not_null: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNNotNull,
    // pub std_string_string_as_char_n_nullable: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNNullable,

    // pub std_string_string_as_varchar_not_null: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharNotNull,
    // pub std_string_string_as_varchar_nullable: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharNullable,

    // pub std_string_string_as_text_not_null: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlTextNotNull,
    // pub std_string_string_as_text_nullable: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlTextNullable,

    // // todo need to install postgresql extension
    // // pub std_string_string_as_ci_text_not_null: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCiTextNotNull,
    // // pub std_string_string_as_ci_text_nullable: postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCiTextNullable,

    // // todo need to install postgresql extension
    // // pub sqlx_postgres_types_pg_ci_text_as_ci_text_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,
    // // pub sqlx_postgres_types_pg_ci_text_as_ci_text_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNullable,

    // pub std_vec_vec_std_primitive_u8_as_bytea_not_null: postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,
    // pub std_vec_vec_std_primitive_u8_as_bytea_nullable: postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNullable,

    // pub sqlx_types_chrono_naive_time_as_time_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,
    // pub sqlx_types_chrono_naive_time_as_time_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNullable,

    // pub sqlx_types_time_time_as_time_not_null: postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,
    // pub sqlx_types_time_time_as_time_nullable: postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNullable,

    // pub sqlx_postgres_types_pg_interval_as_interval_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,
    // pub sqlx_postgres_types_pg_interval_as_interval_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNullable,

    // pub sqlx_types_time_date_as_date_not_null: postgresql_crud::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNotNull,
    // pub sqlx_types_time_date_as_date_nullable: postgresql_crud::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNullable,

    // pub sqlx_types_chrono_naive_date_as_date_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,
    // pub sqlx_types_chrono_naive_date_as_date_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNullable,

    // pub chrono_naive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,
    // pub chrono_naive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNullable,

    // pub sqlx_types_time_primitive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,
    // pub sqlx_types_time_primitive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNullable,

    // pub chrono_date_time_chrono_utc_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,
    // pub chrono_date_time_chrono_utc_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNullable,

    // pub chrono_date_time_chrono_local_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,
    // pub chrono_date_time_chrono_local_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNullable,

    // pub uuid_uuid_as_uuid_v4_init_by_postgres_not_null: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresqlNotNull,
    // // #[generate_postgresql_crud_primary_key] //todo maybe later support something else but now i think uuid v4 and v7 is enough
    // // pub uuid_uuid_as_uuid_v4_init_by_postgres_not_null_prime: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresqlNotNull,

    // pub uuid_uuid_as_uuid_init_by_client_not_null: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNotNull,
    // pub uuid_uuid_as_uuid_init_by_client_nullable: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNullable,

    // pub sqlx_types_ipnetwork_ip_network_as_inet_not_null: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_inet_nullable: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNullable,

    // pub sqlx_types_ipnetwork_ip_network_as_cidr_not_null: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_cidr_nullable: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNullable,

    // pub mac_address_mac_address_as_mac_addr_not_null: postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,
    // pub mac_address_mac_address_as_mac_addr_nullable: postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNullable,

    // pub sqlx_types_bit_vec_as_bit_not_null: postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNotNull,
    // pub sqlx_types_bit_vec_as_bit_nullable: postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNullable,

    // pub sqlx_types_bit_vec_as_varbit_not_null: postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNotNull,
    // pub sqlx_types_bit_vec_as_varbit_nullable: postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNullable,

    // pub pg_range_std_primitive_i32_as_int4_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,
    // pub pg_range_std_primitive_i32_as_int4_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNullable,

    // pub pg_range_std_primitive_i64_as_int8_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,
    // pub pg_range_std_primitive_i64_as_int8_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNullable,

    // pub pg_range_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,
    // pub pg_range_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNullable,

    // pub pg_range_sqlx_types_big_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,
    // pub pg_range_sqlx_types_big_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNullable,

    // pub pg_range_sqlx_types_time_date_as_date_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,
    // pub pg_range_sqlx_types_time_date_as_date_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNullable,

    // pub pg_range_chrono_naive_date_as_date_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,
    // pub pg_range_chrono_naive_date_as_date_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNullable,

    // pub pg_range_naive_date_time_as_timestamp_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNotNull,
    // pub pg_range_naive_date_time_as_timestamp_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNullable,

    // pub pg_range_time_primitive_date_time_as_timestamp_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNotNull,
    // pub pg_range_time_primitive_date_time_as_timestamp_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNullable,

    // pub pg_range_date_time_utc_as_timestamp_tz_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNotNull,
    // pub pg_range_date_time_utc_as_timestamp_tz_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNullable,

    // pub pg_range_date_time_local_as_timestamp_tz_range_not_null: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNotNull,
    // pub pg_range_date_time_local_as_timestamp_tz_range_nullable: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNullable,
    pub object_animal_as_jsonb_not_null: ObjectAnimalAsPostgresqlJsonbNotNull,
    // pub object_animal_as_jsonb_nullable: ObjectAnimalAsPostgresqlJsonbNullable,

    // pub option_object_animal_as_jsonb_not_null: StdOptionOptionObjectAnimalAsPostgresqlJsonbNotNull,
    // pub option_object_animal_as_jsonb_nullable: StdOptionOptionObjectAnimalAsPostgresqlJsonbNullable,

    // pub std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,
    // pub std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdVecVecObjectWithIdAnimalAsPostgresqlJsonbNullable,

    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,
    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonbNullable,

    ///////////////////////////
    // pub vec_std_primitive_i16_as_postgresql_int2_array_not_null: postgresql_crud::postgresql_type::VecStdPrimitiveI16AsPostgresqlInt2ArrayNotNull,
    //63max
}
//todo enum tree support
//todo generate wrapper type for all possible json type
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema
    // , postgresql_crud::GeneratePostgresqlJsonType
)] //user type must implement utoipa::ToSchema trait
pub struct Animal {
    // pub id: postgresql_crud::postgresql_json_type::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    // pub std_vec_vec_uuid_uuid: postgresql_crud::postgresql_json_type::StdVecVecUuidUuid,
    // pub std_vec_vec_std_vec_vec_uuid_uuid: postgresql_crud::postgresql_json_type::StdVecVecStdVecVecUuidUuid,
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
///////////////////////////////////////////////////
pub async fn create_table_if_not_exists(pool: &sqlx::Pool<sqlx::Postgres>) {
    let create_extension_if_not_exists_pg_jsonschema_query_stringified = "create extension if not exists pg_jsonschema";
    println!("{create_extension_if_not_exists_pg_jsonschema_query_stringified}");
    let _ = sqlx::query(create_extension_if_not_exists_pg_jsonschema_query_stringified).execute(pool).await.unwrap();
    let create_extension_if_not_exists_uuid_ossp_query_stringified = "create extension if not exists \"uuid-ossp\"";
    println!("{create_extension_if_not_exists_uuid_ossp_query_stringified}");
    let _ = sqlx::query(create_extension_if_not_exists_uuid_ossp_query_stringified).execute(pool).await.unwrap();
    let create_table_if_not_exists_query_stringified = format!(
        // "create table if not exists example ({},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        "create table if not exists example ({},{},{},{})",
        // "create table if not exists example ({},{},{})",
        postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i64_as_postgresql_big_serial_not_null_primary_key", true),
        postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull::create_table_column_query_part(&"std_primitive_i16_as_small_int2_not_null", false),
        postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlInt2Nullable::create_table_column_query_part(&"std_primitive_i16_as_small_int2_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlInt4NotNull::create_table_column_query_part(&"std_primitive_i32_as_int4_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlInt4Nullable::create_table_column_query_part(&"std_primitive_i32_as_int4_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlInt8NotNull::create_table_column_query_part(&"std_primitive_i64_as_big_int8_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlInt8Nullable::create_table_column_query_part(&"std_primitive_i64_as_big_int8_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4NotNull::create_table_column_query_part(&"std_primitive_f32_as_float4_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4Nullable::create_table_column_query_part(&"std_primitive_f32_as_float4_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8NotNull::create_table_column_query_part(&"std_primitive_f64_as_float8_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8Nullable::create_table_column_query_part(&"std_primitive_f64_as_float8_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i16_as_small_serial_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i32_as_serial_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i64_as_big_serial_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull::create_table_column_query_part(&"sqlx_postgres_types_pg_money_as_money_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNullable::create_table_column_query_part(&"sqlx_postgres_types_pg_money_as_money_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNotNull::create_table_column_query_part(&"sqlx_types_decimal_as_numeric_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNullable::create_table_column_query_part(&"sqlx_types_decimal_as_numeric_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNotNull::create_table_column_query_part(&"sqlx_types_big_decimal_as_numeric_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNullable::create_table_column_query_part(&"sqlx_types_big_decimal_as_numeric_nullable", false),
        // postgresql_crud::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull::create_table_column_query_part(&"std_primitive_bool_as_bool_not_null", false),
        // postgresql_crud::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNullable::create_table_column_query_part(&"std_primitive_bool_as_bool_nullable", false),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNNotNull::create_table_column_query_part(&"std_string_string_as_char_n_not_null", false, postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNNullable::create_table_column_query_part(&"std_string_string_as_char_n_nullable", false, postgresql_crud::postgresql_type::StdStringStringAsPostgresqlCharNLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharNotNull::create_table_column_query_part(&"std_string_string_as_varchar_not_null", false, postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharNullable::create_table_column_query_part(&"std_string_string_as_varchar_nullable", false, postgresql_crud::postgresql_type::StdStringStringAsPostgresqlVarcharLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlTextNotNull::create_table_column_query_part(&"std_string_string_as_text_not_null", false),
        // postgresql_crud::postgresql_type::StdStringStringAsPostgresqlTextNullable::create_table_column_query_part(&"std_string_string_as_text_nullable", false),
        // postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull::create_table_column_query_part(&"std_vec_vec_std_primitive_u8_as_bytea_not_null", false),
        // postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNullable::create_table_column_query_part(&"std_vec_vec_std_primitive_u8_as_bytea_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNotNull::create_table_column_query_part(&"sqlx_types_time_date_as_date_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNullable::create_table_column_query_part(&"sqlx_types_time_date_as_date_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull::create_table_column_query_part(&"sqlx_types_chrono_naive_date_as_date_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNullable::create_table_column_query_part(&"sqlx_types_chrono_naive_date_as_date_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull::create_table_column_query_part(&"sqlx_types_chrono_naive_time_as_time_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNullable::create_table_column_query_part(&"sqlx_types_chrono_naive_time_as_time_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNotNull::create_table_column_query_part(&"sqlx_types_time_time_as_time_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNullable::create_table_column_query_part(&"sqlx_types_time_time_as_time_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull::create_table_column_query_part(&"sqlx_postgres_types_pg_interval_as_interval_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNullable::create_table_column_query_part(&"sqlx_postgres_types_pg_interval_as_interval_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull::create_table_column_query_part(&"chrono_naive_date_time_as_timestamp_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNullable::create_table_column_query_part(&"chrono_naive_date_time_as_timestamp_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull::create_table_column_query_part(&"sqlx_types_time_primitive_date_time_as_timestamp_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNullable::create_table_column_query_part(&"sqlx_types_time_primitive_date_time_as_timestamp_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull::create_table_column_query_part(&"chrono_date_time_chrono_utc_as_timestamp_tz_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNullable::create_table_column_query_part(&"chrono_date_time_chrono_utc_as_timestamp_tz_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull::create_table_column_query_part(&"chrono_date_time_chrono_local_as_timestamp_tz_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNullable::create_table_column_query_part(&"chrono_date_time_chrono_local_as_timestamp_tz_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresqlNotNull::create_table_column_query_part(&"uuid_uuid_as_uuid_v4_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNotNull::create_table_column_query_part(&"uuid_uuid_as_uuid_init_by_client_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNullable::create_table_column_query_part(&"uuid_uuid_as_uuid_init_by_client_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_inet_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNullable::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_inet_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_cidr_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNullable::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_cidr_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull::create_table_column_query_part(&"mac_address_mac_address_as_mac_addr_not_null", false),
        // postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNullable::create_table_column_query_part(&"mac_address_mac_address_as_mac_addr_nullable", false),
        // postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNotNull::create_table_column_query_part(&"sqlx_types_bit_vec_as_bit_not_null", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNullable::create_table_column_query_part(&"sqlx_types_bit_vec_as_bit_nullable", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlBitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNotNull::create_table_column_query_part(&"sqlx_types_bit_vec_as_varbit_not_null", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNullable::create_table_column_query_part(&"sqlx_types_bit_vec_as_varbit_nullable", false, postgresql_crud::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull::create_table_column_query_part(&"pg_range_std_primitive_i32_as_int4_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNullable::create_table_column_query_part(&"pg_range_std_primitive_i32_as_int4_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull::create_table_column_query_part(&"pg_range_std_primitive_i64_as_int8_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNullable::create_table_column_query_part(&"pg_range_std_primitive_i64_as_int8_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNotNull::create_table_column_query_part(&"pg_range_naive_date_time_as_timestamp_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNullable::create_table_column_query_part(&"pg_range_naive_date_time_as_timestamp_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNotNull::create_table_column_query_part(&"pg_range_time_primitive_date_time_as_timestamp_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNullable::create_table_column_query_part(&"pg_range_time_primitive_date_time_as_timestamp_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNotNull::create_table_column_query_part(&"pg_range_date_time_utc_as_timestamp_tz_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNullable::create_table_column_query_part(&"pg_range_date_time_utc_as_timestamp_tz_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNotNull::create_table_column_query_part(&"pg_range_date_time_local_as_timestamp_tz_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNullable::create_table_column_query_part(&"pg_range_date_time_local_as_timestamp_tz_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull::create_table_column_query_part(&"pg_range_chrono_naive_date_as_date_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNullable::create_table_column_query_part(&"pg_range_chrono_naive_date_as_date_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull::create_table_column_query_part(&"pg_range_sqlx_types_time_date_as_date_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNullable::create_table_column_query_part(&"pg_range_sqlx_types_time_date_as_date_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull::create_table_column_query_part(&"pg_range_decimal_as_num_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNullable::create_table_column_query_part(&"pg_range_decimal_as_num_range_nullable", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull::create_table_column_query_part(&"pg_range_sqlx_types_big_decimal_as_num_range_not_null", false),
        // postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNullable::create_table_column_query_part(&"pg_range_sqlx_types_big_decimal_as_num_range_nullable", false),
        ObjectAnimalAsPostgresqlJsonbNotNull::create_table_column_query_part(&"object_animal_as_jsonb_not_null", false),
        // postgresql_crud::postgresql_type::VecStdPrimitiveI16AsPostgresqlInt2ArrayNotNull::create_table_column_query_part(&"vec_std_primitive_i16_as_postgresql_int2_array_not_null", false),
    );
    println!("{create_table_if_not_exists_query_stringified}");
    let _ = sqlx::query(&create_table_if_not_exists_query_stringified).execute(pool).await.unwrap();
}

//////////////////////////////////
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
    std_primitive_i8: postgresql_crud::postgresql_json_type::StdPrimitiveI8Create,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalToCreateOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i8: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl AnimalToCreateOrigin {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.std_primitive_i8, increment) {
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
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.std_primitive_i8, query);
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
    pub fn new(std_primitive_i8: postgresql_crud::postgresql_json_type::StdPrimitiveI8Create) -> Self {
        Self(AnimalToCreateOrigin { std_primitive_i8 })
    }
}
impl AnimalToCreateWithoutGeneratedId {
    pub fn new(std_primitive_i8: postgresql_crud::postgresql_json_type::StdPrimitiveI8Create) -> Self {
        Self(AnimalToCreateOrigin { std_primitive_i8 })
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalToCreateWithGeneratedId {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalToCreateWithoutGeneratedId {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl AnimalToCreateWithGeneratedId {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match self.0.create_query_part(increment) {
            Ok(value) => Ok(format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{value}")),
            Err(error) => Err(error),
        }
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.0.create_query_bind(query);
        query
    }
}
impl AnimalToCreateWithoutGeneratedId {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match self.0.create_query_part(increment) {
            Ok(value) => Ok(value),
            Err(error) => Err(error),
        }
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.0.create_query_bind(query);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalFieldToReadWithoutId {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::postgresql_json_type::StdPrimitiveI8Select),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalFieldToReadWithId {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id(postgresql_crud::postgresql_json_type::UuidUuidSelect),
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::postgresql_json_type::StdPrimitiveI8Select),
}
impl error_occurence_lib::ToStdStringString for AnimalFieldToReadWithoutId {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalFieldToReadWithoutId {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![AnimalFieldToReadWithoutId::StdPrimitiveI8(
            postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        )]
    }
}
impl error_occurence_lib::ToStdStringString for AnimalFieldToReadWithId {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalFieldToReadWithId {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalFieldToReadWithId::Id(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            AnimalFieldToReadWithId::StdPrimitiveI8(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
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
pub struct AnimalReadWithoutId {
    #[serde(skip_serializing_if = "Option::is_none")]
    std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::StdPrimitiveI8Read>>,
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
pub struct AnimalReadWithId {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::UuidUuidRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::StdPrimitiveI8Read>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalReadWithOrWithoutIdTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl AnimalReadWithoutId {
    pub fn try_new(std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::StdPrimitiveI8Read>>) -> Result<Self, AnimalReadWithOrWithoutIdTryFromErrorNamed> {
        if let None = &std_primitive_i8 {
            return Err(AnimalReadWithOrWithoutIdTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { std_primitive_i8 })
    }
}
impl AnimalReadWithId {
    pub fn try_new(
        id: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::UuidUuidRead>>,
        std_primitive_i8: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::StdPrimitiveI8Read>>,
    ) -> Result<Self, AnimalReadWithOrWithoutIdTryFromErrorNamed> {
        if let (None, None) = (&id, &std_primitive_i8) {
            return Err(AnimalReadWithOrWithoutIdTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { id, std_primitive_i8 })
    }
}
impl<'de> serde::Deserialize<'de> for AnimalReadWithoutId {
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
            marker: serde::__private::PhantomData<AnimalReadWithoutId>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = AnimalReadWithoutId;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct AnimalRead")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::StdPrimitiveI8Read>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalRead with 1 elements"));
                    }
                };
                match AnimalReadWithoutId::try_new(__field0) {
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
                let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::StdPrimitiveI8Read>>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i8"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::StdPrimitiveI8Read>>>(&mut __map)?);
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
                match AnimalReadWithoutId::try_new(__field0) {
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
            "AnimalReadWithoutId",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<AnimalReadWithoutId>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl<'de> serde::Deserialize<'de> for AnimalReadWithId {
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
            marker: serde::__private::PhantomData<AnimalReadWithId>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = AnimalReadWithId;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "struct AnimalRead")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::UuidUuidRead>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalRead with 2 elements"));
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::StdPrimitiveI8Read>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalRead with 2 elements"));
                    }
                };
                match AnimalReadWithId::try_new(__field0, __field1) {
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
                let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::UuidUuidRead>>> = serde::__private::None;
                let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::StdPrimitiveI8Read>>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("id"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::UuidUuidRead>>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("std_primitive_i8"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_json_type::StdPrimitiveI8Read>>>(&mut __map)?);
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
                match AnimalReadWithId::try_new(__field0, __field1) {
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
            "AnimalReadWithId",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<AnimalReadWithId>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalReadWithoutId {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalReadWithId {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            std_primitive_i8: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
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
pub enum AnimalUpdateOrigin {
    #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
    StdPrimitiveI8(postgresql_crud::Value<postgresql_crud::postgresql_json_type::StdPrimitiveI8Update>),
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalJsonArrayChangeTryGenerateErrorNamed {
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Create {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalUpdateOrigin {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![AnimalUpdateOrigin::StdPrimitiveI8(postgresql_crud::Value {
            value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalOptionsToUpdate {
    pub id: postgresql_crud::postgresql_json_type::UuidUuidSelect,
    pub fields: AnimalUpdate,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalOptionsToUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            fields: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl std::fmt::Display for Animal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", &self)
    }
}
pub type AnimalCreate = AnimalToCreateWithoutGeneratedId;
pub type AnimalFieldToRead = AnimalFieldToReadWithoutId;
pub type AnimalRead = AnimalReadWithoutId;
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
pub struct AnimalSelect(std::vec::Vec<AnimalFieldToReadWithoutId>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalSelectTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithoutId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl AnimalSelect {
    pub fn try_new(value: std::vec::Vec<AnimalFieldToReadWithoutId>) -> Result<Self, AnimalSelectTryNewErrorNamed> {
        if value.is_empty() {
            return Err(AnimalSelectTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(AnimalSelectTryNewErrorNamed::NotUniqueFieldFilter {
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
impl<'de> serde::Deserialize<'de> for AnimalSelect {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<AnimalSelect>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = AnimalSelect;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct AnimalSelect")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalFieldToReadWithoutId> = <std::vec::Vec<AnimalFieldToReadWithoutId> as serde::Deserialize>::deserialize(__e)?;
                match AnimalSelect::try_new(__field0) {
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
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct AnimalSelect with 1 element"));
                    }
                };
                match AnimalSelect::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "AnimalSelect",
            __Visitor {
                marker: serde::__private::PhantomData::<AnimalSelect>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
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
                        <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::select_query_part(value, "std_primitive_i8", column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message, false,),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        format!("{acc}")
    }
}
pub type AnimalReader = AnimalRead;
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
pub struct AnimalUpdate(std::vec::Vec<AnimalUpdateOrigin>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalUpdateTryNewErrorNamed {
    FieldsAreEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldStdPrimitiveI8 {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl AnimalUpdate {
    pub fn try_new(value: std::vec::Vec<AnimalUpdateOrigin>) -> Result<Self, AnimalUpdateTryNewErrorNamed> {
        if value.is_empty() {
            return Err(AnimalUpdateTryNewErrorNamed::FieldsAreEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            let generate_not_unique_field = |value: &std::primitive::str| format!("not unique {value} field");
            for element in &value {
                match element {
                    AnimalUpdateOrigin::StdPrimitiveI8(_) => {
                        let value = AnimalFieldToUpdate::StdPrimitiveI8;
                        if acc.contains(&value) {
                            return Err(AnimalUpdateTryNewErrorNamed::NotUniqueFieldStdPrimitiveI8 {
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
impl<'de> serde::Deserialize<'de> for AnimalUpdate {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<AnimalUpdate>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = AnimalUpdate;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct AnimalUpdate")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalUpdateOrigin> = <std::vec::Vec<AnimalUpdateOrigin> as serde::Deserialize>::deserialize(__e)?;
                match AnimalUpdate::try_new(__field0) {
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
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<AnimalUpdateOrigin>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct AnimalUpdate with 1 element"));
                    }
                };
                match AnimalUpdate::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "AnimalUpdate",
            __Visitor {
                marker: serde::__private::PhantomData::<AnimalUpdate>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl AnimalUpdate {
    fn update_query_part(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
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
                AnimalUpdateOrigin::StdPrimitiveI8(value) => {
                    match <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &local_acc, &generate_jsonb_set_target("std_primitive_i8"), &generate_jsonb_set_path("std_primitive_i8"), increment) {
                        Ok(value) => {
                            local_acc = value;
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
            }
        }
        Ok(local_acc)
    }
    fn update_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0 {
            match element {
                AnimalUpdateOrigin::StdPrimitiveI8(value) => {
                    query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
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
pub struct ObjectAnimal {
    pub std_primitive_i8: postgresql_crud::postgresql_json_type::StdPrimitiveI8,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectAnimal {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i8: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
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
pub struct ObjectAnimalCreate(pub AnimalToCreateWithoutGeneratedId);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectAnimalCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
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
pub struct ObjectAnimalRead(pub AnimalReadWithoutId);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectAnimalRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
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
pub struct ObjectAnimalSelect(std::vec::Vec<AnimalFieldToReadWithoutId>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ObjectAnimalSelectTryNewErrorNamed {
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: AnimalFieldToReadWithoutId,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl ObjectAnimalSelect {
    pub fn try_new(value: std::vec::Vec<AnimalFieldToReadWithoutId>) -> Result<Self, ObjectAnimalSelectTryNewErrorNamed> {
        if value.is_empty() {
            return Err(ObjectAnimalSelectTryNewErrorNamed::FieldsFilterIsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        let mut unique = vec![];
        for element in value {
            if unique.contains(&element) {
                return Err(ObjectAnimalSelectTryNewErrorNamed::NotUniqueFieldFilter {
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
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectAnimalSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl<'de> serde::Deserialize<'de> for ObjectAnimalSelect {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<ObjectAnimalSelect>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = ObjectAnimalSelect;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct ObjectAnimalSelect")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: std::vec::Vec<AnimalFieldToReadWithoutId> = <std::vec::Vec<AnimalFieldToReadWithoutId> as serde::Deserialize>::deserialize(__e)?;
                match ObjectAnimalSelect::try_new(__field0) {
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
                        return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"tuple struct ObjectAnimalSelect with 1 element"));
                    }
                };
                match ObjectAnimalSelect::try_new(__field0) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            __deserializer,
            "ObjectAnimalSelect",
            __Visitor {
                marker: serde::__private::PhantomData::<ObjectAnimalSelect>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
pub type ObjectAnimalReader = ObjectAnimalRead;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum ObjectAnimalWhereElement {
    StdPrimitiveI8(postgresql_crud::PostgresqlTypeWhere<postgresql_crud::postgresql_json_type::StdPrimitiveI8WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for ObjectAnimalWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::StdPrimitiveI8(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'std_primitive_i8'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, 
        // mut 
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::StdPrimitiveI8(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for ObjectAnimalWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectAnimalWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::StdPrimitiveI8(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
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
pub struct ObjectAnimalUpdate(pub AnimalUpdate);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectAnimalUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl postgresql_crud::PostgresqlJsonType for ObjectAnimal {
    type Create<'a> = ObjectAnimalCreate;
    fn create_query_part(value: &Self::Create<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.0.create_query_part(increment)
    }
    fn create_query_bind<'a>(value: Self::Create<'a>, 
    // mut 
    query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.0.create_query_bind(query)
    }
    type Select<'a> = ObjectAnimalSelect;
    fn select_query_part(value: &Self::Select<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = if is_postgresql_type { column_name_and_maybe_field_getter.to_string() } else { format!("{column_name_and_maybe_field_getter}->'{field_ident}'") };
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in &value.0 {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalFieldToReadWithoutId::StdPrimitiveI8(value) =>
                        <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::select_query_part(value, "std_primitive_i8", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
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
    type WhereElement<'a> = ObjectAnimalWhereElement;
    type Read<'a> = ObjectAnimalRead;
    type Update<'a> = ObjectAnimalUpdate;
    fn update_query_part(value: &Self::Update<'_>, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        for element in &value.0 .0 {
            match element {
                AnimalUpdateOrigin::StdPrimitiveI8(value) => {
                    match <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("std_primitive_i8"), "std_primitive_i8", increment) {
                        Ok(value) => {
                            std_option_option_object_acc = value;
                        }
                        Err(error) => {
                            return Err(error);
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
    fn update_query_bind<'a>(value: Self::Update<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in value.0 .0 {
            match element {
                AnimalUpdateOrigin::StdPrimitiveI8(value) => {
                    query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8 as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
            }
        }
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct ObjectAnimalAsPostgresqlJsonbNotNull(ObjectAnimalCreate);
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
impl ObjectAnimalAsPostgresqlJsonbNotNull {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(ObjectAnimalAsPostgresqlJsonbNotNull)).unwrap())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct ObjectAnimalAsPostgresqlJsonbNotNullCreate(ObjectAnimalAsPostgresqlJsonbNotNull);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectAnimalAsPostgresqlJsonbNotNullCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(ObjectAnimalAsPostgresqlJsonbNotNull(
            postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        ))
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct ObjectAnimalAsPostgresqlJsonbNotNullSelect(ObjectAnimalSelect);
impl sqlx::Type<sqlx::Postgres> for ObjectAnimalAsPostgresqlJsonbNotNullSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<ObjectAnimalSelect> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<ObjectAnimalSelect> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for ObjectAnimalAsPostgresqlJsonbNotNullSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<ObjectAnimalSelect> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value.0)),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectAnimalAsPostgresqlJsonbNotNullSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum ObjectAnimalAsPostgresqlJsonbNotNullWhereElement {
    StdPrimitiveI8(postgresql_crud::PostgresqlTypeWhere<postgresql_crud::postgresql_json_type::StdPrimitiveI8WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for ObjectAnimalAsPostgresqlJsonbNotNullWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::StdPrimitiveI8(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'std_primitive_i8'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, 
        // mut 
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::StdPrimitiveI8(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for ObjectAnimalAsPostgresqlJsonbNotNullWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectAnimalAsPostgresqlJsonbNotNullWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::StdPrimitiveI8(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct ObjectAnimalAsPostgresqlJsonbNotNullRead(ObjectAnimalRead);
impl sqlx::Type<sqlx::Postgres> for ObjectAnimalAsPostgresqlJsonbNotNullRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<ObjectAnimalRead> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<ObjectAnimalRead> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for ObjectAnimalAsPostgresqlJsonbNotNullRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<ObjectAnimalRead> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value.0)),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ObjectAnimalAsPostgresqlJsonbNotNullUpdateQueryPartErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Todo,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct ObjectAnimalAsPostgresqlJsonbNotNullUpdate(ObjectAnimalUpdate);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ObjectAnimalAsPostgresqlJsonbNotNullUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl postgresql_crud::PostgresqlType for ObjectAnimalAsPostgresqlJsonbNotNull {
    type PostgresqlTypeSelf = Self;
    type Create = ObjectAnimalAsPostgresqlJsonbNotNullCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        Ok(<ObjectAnimal as postgresql_crud::PostgresqlJsonType>::create_query_part(&value.0 .0, increment).unwrap())
    }
    fn create_query_bind(value: Self::Create, 
        // mut 
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        <ObjectAnimal as postgresql_crud::PostgresqlJsonType>::create_query_bind(value.0 .0, query)
    }
    type Select = ObjectAnimalAsPostgresqlJsonbNotNullSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", <ObjectAnimal as postgresql_crud::PostgresqlJsonType>::select_query_part(&value.0, &column, &column, &column, true))
    }
    type WhereElement = ObjectAnimalAsPostgresqlJsonbNotNullWhereElement;
    type Read = ObjectAnimalAsPostgresqlJsonbNotNullRead;
    type Update = ObjectAnimalAsPostgresqlJsonbNotNullUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        Ok(<ObjectAnimal as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.0, jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment).unwrap())
    }
    fn update_query_bind<'a>(value: Self::Update, 
        // mut 
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        <ObjectAnimal as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.0, query)
    }
}
