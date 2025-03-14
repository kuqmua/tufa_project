#[derive(
    Debug, 
    // postgresql_crud::GeneratePostgresqlCrud
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
    pub std_primitive_i16_as_small_int2_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull,
    // pub std_primitive_i16_as_small_int2_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2Nullable,

    // pub std_primitive_i32_as_int4_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlInt4NotNull,
    // pub std_primitive_i32_as_int4_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlInt4Nullable,

    // pub std_primitive_i64_as_big_int8_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlInt8NotNull,
    // pub std_primitive_i64_as_big_int8_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlInt8Nullable,

    // pub std_primitive_f32_as_float4_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4NotNull,
    // pub std_primitive_f32_as_float4_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4Nullable,

    // pub std_primitive_f64_as_float8_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8NotNull,
    // pub std_primitive_f64_as_float8_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8Nullable,

    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i16_as_small_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresqlNotNull,
    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i32_as_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresqlNotNull,
    // // // #[generate_postgresql_crud_primary_key]
    // pub std_primitive_i64_as_big_serial_init_by_postgres_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull,
    // #[generate_postgresql_crud_primary_key]
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull,

    // pub sqlx_postgres_types_pg_money_as_money_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,
    // pub sqlx_postgres_types_pg_money_as_money_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNullable,

    // pub sqlx_types_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNotNull,
    // pub sqlx_types_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNullable,

    // pub sqlx_types_big_decimal_as_numeric_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNotNull,
    // pub sqlx_types_big_decimal_as_numeric_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNullable,

    // pub std_primitive_bool_as_bool_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull,
    // pub std_primitive_bool_as_bool_nullable: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNullable,

    // pub std_string_string_as_char_n_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNNotNull,
    // pub std_string_string_as_char_n_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNNullable,

    // pub std_string_string_as_varchar_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarcharNotNull,
    // pub std_string_string_as_varchar_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarcharNullable,

    // pub std_string_string_as_text_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlTextNotNull,
    // pub std_string_string_as_text_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlTextNullable,

    // // todo need to install postgresql extension
    // // pub std_string_string_as_ci_text_not_null: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCiTextNotNull,
    // // pub std_string_string_as_ci_text_nullable: postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCiTextNullable,

    // // todo need to install postgresql extension
    // // pub sqlx_postgres_types_pg_ci_text_as_ci_text_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,
    // // pub sqlx_postgres_types_pg_ci_text_as_ci_text_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNullable,

    // pub std_vec_vec_std_primitive_u8_as_bytea_not_null: postgresql_crud::postgresql_type::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,
    // pub std_vec_vec_std_primitive_u8_as_bytea_nullable: postgresql_crud::postgresql_type::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNullable,

    // pub sqlx_types_chrono_naive_time_as_time_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,
    // pub sqlx_types_chrono_naive_time_as_time_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNullable,

    // pub sqlx_types_time_time_as_time_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,
    // pub sqlx_types_time_time_as_time_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNullable,

    // pub sqlx_postgres_types_pg_interval_as_interval_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,
    // pub sqlx_postgres_types_pg_interval_as_interval_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNullable,

    // pub sqlx_types_time_date_as_date_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNotNull,
    // pub sqlx_types_time_date_as_date_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNullable,

    // pub sqlx_types_chrono_naive_date_as_date_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,
    // pub sqlx_types_chrono_naive_date_as_date_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNullable,

    // pub chrono_naive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,
    // pub chrono_naive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNullable,

    // pub sqlx_types_time_primitive_date_time_as_timestamp_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,
    // pub sqlx_types_time_primitive_date_time_as_timestamp_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNullable,

    // pub chrono_date_time_chrono_utc_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,
    // pub chrono_date_time_chrono_utc_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNullable,

    // pub chrono_date_time_chrono_local_as_timestamp_tz_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,
    // pub chrono_date_time_chrono_local_as_timestamp_tz_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNullable,

    // pub uuid_uuid_as_uuid_v4_init_by_postgres_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresqlNotNull,
    // // #[generate_postgresql_crud_primary_key] //todo maybe later support something else but now i think uuid v4 and v7 is enough
    // // pub uuid_uuid_as_uuid_v4_init_by_postgres_not_null_prime: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresqlNotNull,

    // pub uuid_uuid_as_uuid_init_by_client_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNotNull,
    // pub uuid_uuid_as_uuid_init_by_client_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNullable,

    // pub sqlx_types_ipnetwork_ip_network_as_inet_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_inet_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNullable,

    // pub sqlx_types_ipnetwork_ip_network_as_cidr_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_cidr_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNullable,

    // pub mac_address_mac_address_as_mac_addr_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,
    // pub mac_address_mac_address_as_mac_addr_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNullable,

    // pub sqlx_types_bit_vec_as_bit_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNotNull,
    // pub sqlx_types_bit_vec_as_bit_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNullable,

    // pub sqlx_types_bit_vec_as_varbit_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNotNull,
    // pub sqlx_types_bit_vec_as_varbit_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNullable,

    // pub pg_range_std_primitive_i32_as_int4_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,
    // pub pg_range_std_primitive_i32_as_int4_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNullable,

    // pub pg_range_std_primitive_i64_as_int8_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,
    // pub pg_range_std_primitive_i64_as_int8_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNullable,

    // pub pg_range_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,
    // pub pg_range_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNullable,

    // pub pg_range_sqlx_types_big_decimal_as_num_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,
    // pub pg_range_sqlx_types_big_decimal_as_num_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNullable,

    // pub pg_range_sqlx_types_time_date_as_date_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,
    // pub pg_range_sqlx_types_time_date_as_date_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNullable,

    // pub pg_range_chrono_naive_date_as_date_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,
    // pub pg_range_chrono_naive_date_as_date_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNullable,

    // pub pg_range_naive_date_time_as_timestamp_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNotNull,
    // pub pg_range_naive_date_time_as_timestamp_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNullable,

    // pub pg_range_time_primitive_date_time_as_timestamp_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNotNull,
    // pub pg_range_time_primitive_date_time_as_timestamp_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNullable,

    // pub pg_range_date_time_utc_as_timestamp_tz_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNotNull,
    // pub pg_range_date_time_utc_as_timestamp_tz_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNullable,

    // pub pg_range_date_time_local_as_timestamp_tz_range_not_null: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNotNull,
    // pub pg_range_date_time_local_as_timestamp_tz_range_nullable: postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNullable,
    pub object_animal_as_jsonb_not_null: ObjectAnimalAsPostgresqlJsonbNotNull,
    // pub object_animal_as_jsonb_nullable: ObjectAnimalAsPostgresqlJsonbNullable,

    // pub option_object_animal_as_jsonb_not_null: StdOptionOptionObjectAnimalAsPostgresqlJsonbNotNull,
    // pub option_object_animal_as_jsonb_nullable: StdOptionOptionObjectAnimalAsPostgresqlJsonbNullable,

    // pub std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,
    // pub std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdVecVecObjectWithIdAnimalAsPostgresqlJsonbNullable,

    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonbNotNull,
    // pub option_std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdOptionOptionStdVecVecObjectWithIdAnimalAsPostgresqlJsonbNullable,

    ///////////////////////////
    // pub vec_std_primitive_i16_as_postgresql_int2_array_not_null: postgresql_crud::postgresql_type::postgresql_type::VecStdPrimitiveI16AsPostgresqlInt2ArrayNotNull,
    //63max
}
//todo enum tree support
//todo generate wrapper type for all possible json type
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, postgresql_crud::GeneratePostgresqlJsonType)] //user type must implement utoipa::ToSchema trait
pub struct Animal {
    // pub id: postgresql_crud::postgresql_json_type::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    // pub std_vec_vec_uuid_uuid: postgresql_crud::postgresql_json_type::StdVecVecUuidUuid,
    pub std_vec_vec_std_vec_vec_uuid_uuid: postgresql_crud::postgresql_json_type::StdVecVecStdVecVecUuidUuid,
    //todo check field max length in postgresql
    // pub std_primitive_i8: postgresql_crud::postgresql_json_type::StdPrimitiveI8,
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
        // "create table if not exists example ({},{},{},{})",
        "create table if not exists example ({},{},{})",
        postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i64_as_postgresql_big_serial_not_null_primary_key", true),
        postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull::create_table_column_query_part(&"std_primitive_i16_as_small_int2_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2Nullable::create_table_column_query_part(&"std_primitive_i16_as_small_int2_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlInt4NotNull::create_table_column_query_part(&"std_primitive_i32_as_int4_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlInt4Nullable::create_table_column_query_part(&"std_primitive_i32_as_int4_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlInt8NotNull::create_table_column_query_part(&"std_primitive_i64_as_big_int8_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlInt8Nullable::create_table_column_query_part(&"std_primitive_i64_as_big_int8_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4NotNull::create_table_column_query_part(&"std_primitive_f32_as_float4_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF32AsPostgresqlFloat4Nullable::create_table_column_query_part(&"std_primitive_f32_as_float4_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8NotNull::create_table_column_query_part(&"std_primitive_f64_as_float8_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveF64AsPostgresqlFloat8Nullable::create_table_column_query_part(&"std_primitive_f64_as_float8_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i16_as_small_serial_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i32_as_serial_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull::create_table_column_query_part(&"std_primitive_i64_as_big_serial_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull::create_table_column_query_part(&"sqlx_postgres_types_pg_money_as_money_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNullable::create_table_column_query_part(&"sqlx_postgres_types_pg_money_as_money_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNotNull::create_table_column_query_part(&"sqlx_types_decimal_as_numeric_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesDecimalAsPostgresqlNumericNullable::create_table_column_query_part(&"sqlx_types_decimal_as_numeric_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNotNull::create_table_column_query_part(&"sqlx_types_big_decimal_as_numeric_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBigDecimalAsPostgresqlNumericNullable::create_table_column_query_part(&"sqlx_types_big_decimal_as_numeric_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNotNull::create_table_column_query_part(&"std_primitive_bool_as_bool_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveBoolAsPostgresqlBoolNullable::create_table_column_query_part(&"std_primitive_bool_as_bool_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNNotNull::create_table_column_query_part(&"std_string_string_as_char_n_not_null", false, postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNNullable::create_table_column_query_part(&"std_string_string_as_char_n_nullable", false, postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlCharNLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarcharNotNull::create_table_column_query_part(&"std_string_string_as_varchar_not_null", false, postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarcharLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarcharNullable::create_table_column_query_part(&"std_string_string_as_varchar_nullable", false, postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlVarcharLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlTextNotNull::create_table_column_query_part(&"std_string_string_as_text_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdStringStringAsPostgresqlTextNullable::create_table_column_query_part(&"std_string_string_as_text_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull::create_table_column_query_part(&"std_vec_vec_std_primitive_u8_as_bytea_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::StdVecVecStdPrimitiveU8AsPostgresqlByteaNullable::create_table_column_query_part(&"std_vec_vec_std_primitive_u8_as_bytea_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNotNull::create_table_column_query_part(&"sqlx_types_time_date_as_date_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeDateAsPostgresqlDateNullable::create_table_column_query_part(&"sqlx_types_time_date_as_date_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull::create_table_column_query_part(&"sqlx_types_chrono_naive_date_as_date_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateAsPostgresqlDateNullable::create_table_column_query_part(&"sqlx_types_chrono_naive_date_as_date_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull::create_table_column_query_part(&"sqlx_types_chrono_naive_time_as_time_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNullable::create_table_column_query_part(&"sqlx_types_chrono_naive_time_as_time_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNotNull::create_table_column_query_part(&"sqlx_types_time_time_as_time_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimeTimeAsPostgresqlTimeNullable::create_table_column_query_part(&"sqlx_types_time_time_as_time_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull::create_table_column_query_part(&"sqlx_postgres_types_pg_interval_as_interval_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNullable::create_table_column_query_part(&"sqlx_postgres_types_pg_interval_as_interval_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull::create_table_column_query_part(&"chrono_naive_date_time_as_timestamp_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNullable::create_table_column_query_part(&"chrono_naive_date_time_as_timestamp_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull::create_table_column_query_part(&"sqlx_types_time_primitive_date_time_as_timestamp_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNullable::create_table_column_query_part(&"sqlx_types_time_primitive_date_time_as_timestamp_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull::create_table_column_query_part(&"chrono_date_time_chrono_utc_as_timestamp_tz_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNullable::create_table_column_query_part(&"chrono_date_time_chrono_utc_as_timestamp_tz_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull::create_table_column_query_part(&"chrono_date_time_chrono_local_as_timestamp_tz_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNullable::create_table_column_query_part(&"chrono_date_time_chrono_local_as_timestamp_tz_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresqlNotNull::create_table_column_query_part(&"uuid_uuid_as_uuid_v4_init_by_postgres_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNotNull::create_table_column_query_part(&"uuid_uuid_as_uuid_init_by_client_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClientNullable::create_table_column_query_part(&"uuid_uuid_as_uuid_init_by_client_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_inet_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNullable::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_inet_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_cidr_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNullable::create_table_column_query_part(&"sqlx_types_ipnetwork_ip_network_as_cidr_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull::create_table_column_query_part(&"mac_address_mac_address_as_mac_addr_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNullable::create_table_column_query_part(&"mac_address_mac_address_as_mac_addr_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNotNull::create_table_column_query_part(&"sqlx_types_bit_vec_as_bit_not_null", false, postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBitNullable::create_table_column_query_part(&"sqlx_types_bit_vec_as_bit_nullable", false, postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlBitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNotNull::create_table_column_query_part(&"sqlx_types_bit_vec_as_varbit_not_null", false, postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitNullable::create_table_column_query_part(&"sqlx_types_bit_vec_as_varbit_nullable", false, postgresql_crud::postgresql_type::postgresql_type::SqlxTypesBitVecAsPostgresqlVarbitLength::try_from(1).unwrap()),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull::create_table_column_query_part(&"pg_range_std_primitive_i32_as_int4_range_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNullable::create_table_column_query_part(&"pg_range_std_primitive_i32_as_int4_range_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull::create_table_column_query_part(&"pg_range_std_primitive_i64_as_int8_range_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNullable::create_table_column_query_part(&"pg_range_std_primitive_i64_as_int8_range_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNotNull::create_table_column_query_part(&"pg_range_naive_date_time_as_timestamp_range_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampRangeNullable::create_table_column_query_part(&"pg_range_naive_date_time_as_timestamp_range_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNotNull::create_table_column_query_part(&"pg_range_time_primitive_date_time_as_timestamp_range_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampRangeNullable::create_table_column_query_part(&"pg_range_time_primitive_date_time_as_timestamp_range_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNotNull::create_table_column_query_part(&"pg_range_date_time_utc_as_timestamp_tz_range_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzRangeNullable::create_table_column_query_part(&"pg_range_date_time_utc_as_timestamp_tz_range_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNotNull::create_table_column_query_part(&"pg_range_date_time_local_as_timestamp_tz_range_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzRangeNullable::create_table_column_query_part(&"pg_range_date_time_local_as_timestamp_tz_range_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull::create_table_column_query_part(&"pg_range_chrono_naive_date_as_date_range_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNullable::create_table_column_query_part(&"pg_range_chrono_naive_date_as_date_range_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull::create_table_column_query_part(&"pg_range_sqlx_types_time_date_as_date_range_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNullable::create_table_column_query_part(&"pg_range_sqlx_types_time_date_as_date_range_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull::create_table_column_query_part(&"pg_range_decimal_as_num_range_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNullable::create_table_column_query_part(&"pg_range_decimal_as_num_range_nullable", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull::create_table_column_query_part(&"pg_range_sqlx_types_big_decimal_as_num_range_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNullable::create_table_column_query_part(&"pg_range_sqlx_types_big_decimal_as_num_range_nullable", false),
        ObjectAnimalAsPostgresqlJsonbNotNull::create_table_column_query_part(&"object_animal_as_jsonb_not_null", false),
        // postgresql_crud::postgresql_type::postgresql_type::VecStdPrimitiveI16AsPostgresqlInt2ArrayNotNull::create_table_column_query_part(&"vec_std_primitive_i16_as_postgresql_int2_array_not_null", false),
    );
    println!("{create_table_if_not_exists_query_stringified}");
    let _ = sqlx::query(&create_table_if_not_exists_query_stringified).execute(pool).await.unwrap();
}

//////////////////////////////////
impl Example {
    pub fn table_name() -> &'static str {
        "example"
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct ExampleOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_primitive_i16_as_small_int2_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNullRead>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_animal_as_jsonb_not_null: std::option::Option<postgresql_crud::Value<ObjectAnimalAsPostgresqlJsonbNotNullRead>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, PartialEq, Clone)]
pub enum ExampleColumn<'a> {
    #[serde(rename(serialize = "std_primitive_i16_as_small_int2_not_null", deserialize = "std_primitive_i16_as_small_int2_not_null"))]
    StdPrimitiveI16AsSmallInt2NotNull(<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Column),
    #[serde(rename(serialize = "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key", deserialize = "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"))]
    StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Column),
    #[serde(rename(serialize = "object_animal_as_jsonb_not_null", deserialize = "object_animal_as_jsonb_not_null"))]
    ObjectAnimalAsJsonbNotNull(<ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Column),
}
impl std::fmt::Display for ExampleColumn<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", serde_json::to_string(&self).unwrap_or_else(|e| format!("cannot serialize into json: {e:?}")))
    }
}
impl error_occurence_lib::ToStdStringString for ExampleColumn<'_> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ExampleColumn<'_> {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            ExampleColumn::StdPrimitiveI16AsSmallInt2NotNull(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            ExampleColumn::ObjectAnimalAsJsonbNotNull(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}
impl ExampleColumn<'_> {
    fn pick_column(&self) -> std::string::String {
        match &self {
            Self::StdPrimitiveI16AsSmallInt2NotNull(_) => "std_primitive_i16_as_small_int2_not_null".to_string(),
            Self::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(_) => "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key".to_string(),
            Self::ObjectAnimalAsJsonbNotNull(_) => "object_animal_as_jsonb_not_null".to_string(),
        }
    }
}
pub const ALLOW_METHODS: [http::Method; 4] = [http::Method::GET, http::Method::POST, http::Method::PATCH, http::Method::DELETE];
#[derive(Debug, Clone, Copy)]
pub struct ExampleColumnReadPermission {
    std_primitive_i16_as_small_int2_not_null: std::primitive::bool,
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::primitive::bool,
    object_animal_as_jsonb_not_null: std::primitive::bool,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElement<'a> {
    pub std_primitive_i16_as_small_int2_not_null: <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Create,
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Create,
    pub object_animal_as_jsonb_not_null: <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Create,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayload<'a>(pub std::vec::Vec<CreateManyPayloadElement<'a>>);
#[derive(Debug)]
pub struct CreateManyParameters<'a> {
    pub payload: CreateManyPayload<'a>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead>),
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
        bind_query: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize,
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
        bind_query: postgresql_crud::QueryPartErrorNamed,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2554,
                        column: 264,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2626,
                            column: 249,
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
            match postgresql_crud::BindQuery::try_generate_bind_increments(&element.std_primitive_i16_as_small_int2_not_null, &mut increment) {
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
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3140,
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
            match postgresql_crud::BindQuery::try_generate_bind_increments(&element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, &mut increment) {
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
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3140,
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
            match postgresql_crud::BindQuery::try_generate_bind_increments(&element.object_animal_as_jsonb_not_null, &mut increment) {
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
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3140,
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
        format!("insert into example (std_primitive_i16_as_small_int2_not_null,std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,object_animal_as_jsonb_not_null) values {values} returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key")
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        for element in parameters.payload.0 {
            query = postgresql_crud::BindQuery::bind_value_to_query(element.std_primitive_i16_as_small_int2_not_null, query);
            query = postgresql_crud::BindQuery::bind_value_to_query(element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, query);
            query = postgresql_crud::BindQuery::bind_value_to_query(element.object_animal_as_jsonb_not_null, query);
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2576,
                        column: 253,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2576,
                        column: 253,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2301,
                            column: 245,
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
                    Some(value) => match sqlx::Row::try_get::<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
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
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2974,
                                                column: 128,
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
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2974,
                                                column: 157,
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
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2976,
                                        column: 124,
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
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2976,
                                        column: 153,
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
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 3194,
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
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 3201,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2313,
                        column: 245,
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
pub async fn try_create_many(server_location: &std::primitive::str, parameters: CreateManyParameters<'_>) -> Result<std::vec::Vec<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead>, TryCreateManyErrorNamed> {
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2682,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2734,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2751,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2764,
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
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2802,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CreateManyPayloadElement<'_> {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i16_as_small_int2_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            object_animal_as_jsonb_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CreateManyPayload<'_> {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(vec![
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ])
    }
}
pub async fn create_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <CreateManyPayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateOnePayload<'a> {
    pub std_primitive_i16_as_small_int2_not_null: <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Create,
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Create,
    pub object_animal_as_jsonb_not_null: <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Create,
}
#[derive(Debug)]
pub struct CreateOneParameters<'a> {
    pub payload: CreateOnePayload<'a>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateOneRouteLogicResponseVariants {
    Desirable(postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead),
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
        bind_query: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize,
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
        bind_query: postgresql_crud::QueryPartErrorNamed,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2554,
                        column: 264,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2626,
                            column: 249,
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
            "insert into example (std_primitive_i16_as_small_int2_not_null,std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,object_animal_as_jsonb_not_null) values ({},{},{}) returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
            match postgresql_crud::BindQuery::try_generate_bind_increments(&parameters.payload.std_primitive_i16_as_small_int2_not_null, &mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TryCreateOneRouteLogicErrorNamed::BindQuery {
                        bind_query: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3428,
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
            match postgresql_crud::BindQuery::try_generate_bind_increments(&parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, &mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TryCreateOneRouteLogicErrorNamed::BindQuery {
                        bind_query: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3428,
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
            match postgresql_crud::BindQuery::try_generate_bind_increments(&parameters.payload.object_animal_as_jsonb_not_null, &mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TryCreateOneRouteLogicErrorNamed::BindQuery {
                        bind_query: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3428,
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
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.std_primitive_i16_as_small_int2_not_null, query);
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, query);
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.object_animal_as_jsonb_not_null, query);
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2576,
                        column: 253,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2576,
                        column: 253,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2301,
                            column: 245,
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
                Ok(value) => match sqlx::Row::try_get::<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
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
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2990,
                                        column: 111,
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
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2990,
                                        column: 140,
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
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2992,
                                    column: 107,
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
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2992,
                                    column: 136,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2313,
                        column: 245,
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
pub async fn try_create_one(server_location: &std::primitive::str, parameters: CreateOneParameters<'_>) -> Result<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead, TryCreateOneErrorNamed> {
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2682,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2734,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2751,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2764,
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
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2802,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CreateOnePayload<'_> {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i16_as_small_int2_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            object_animal_as_jsonb_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn create_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <CreateOnePayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadManyPayload<'a> {
    pub std_primitive_i16_as_small_int2_not_null: std::option::Option<<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Where>,
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Where>,
    pub object_animal_as_jsonb_not_null: std::option::Option<<ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Where>,
    pub select: std::vec::Vec<ExampleColumn<'a>>,
    pub order_by: postgresql_crud::OrderBy<ExampleColumn<'a>>,
    pub pagination: postgresql_crud::Pagination,
}
#[derive(Debug)]
pub struct ReadManyParameters<'a> {
    pub payload: ReadManyPayload<'a>,
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
        bind_query: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize,
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
    NotUniqueStdPrimitiveI16AsSmallInt2NotNull {
        std_primitive_i16_as_small_int2_not_null: std::string::String,
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
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveI16AsSmallInt2NotNull {
                std_primitive_i16_as_small_int2_not_null,
                code_occurence,
            } => Self::NotUniqueStdPrimitiveI16AsSmallInt2NotNull {
                std_primitive_i16_as_small_int2_not_null,
                code_occurence,
            },
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
        bind_query: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: ExampleColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsSmallInt2NotNull {
        #[eo_to_std_string_string]
        std_primitive_i16_as_small_int2_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull,
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
                        line: 2554,
                        column: 264,
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
                            line: 2626,
                            column: 249,
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
                        ExampleColumn::StdPrimitiveI16AsSmallInt2NotNull(value) => {
                            <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::column_query_part(value, "std_primitive_i16_as_small_int2_not_null")
                        }
                        ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(value) => {
                            <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::column_query_part(
                                value,
                                "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                            )
                        }
                        ExampleColumn::ObjectAnimalAsJsonbNotNull(value) => <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::column_query_part(value, "object_animal_as_jsonb_not_null"),
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
                if let Some(value) = &parameters.payload.std_primitive_i16_as_small_int2_not_null {
                    match <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::where_query_part(
                        value,
                        &mut increment,
                        &"std_primitive_i16_as_small_int2_not_null",
                        is_first_push_to_additional_parameters_already_happend,
                    ) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            todo!()
                        }
                    }
                }
                if let Some(value) = &parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
                    match <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::where_query_part(
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
                            todo!()
                        }
                    }
                }
                if let Some(value) = &parameters.payload.object_animal_as_jsonb_not_null {
                    match <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::where_query_part(value, &mut increment, &"object_animal_as_jsonb_not_null", is_first_push_to_additional_parameters_already_happend) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            todo!()
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
                                        line: 3664,
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
        if let Some(value) = parameters.payload.std_primitive_i16_as_small_int2_not_null {
            query = <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::where_query_bind(value, query);
        }
        if let Some(value) = parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
            query = <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::where_query_bind(value, query);
        }
        if let Some(value) = parameters.payload.object_animal_as_jsonb_not_null {
            query = <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::where_query_bind(value, query);
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
                        line: 2576,
                        column: 253,
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
                        line: 2576,
                        column: 253,
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
                        let mut std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead>> = None;
                        let mut std_primitive_i16_as_small_int2_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNullRead>> = None;
                        let mut object_animal_as_jsonb_not_null: std::option::Option<postgresql_crud::Value<ObjectAnimalAsPostgresqlJsonbNotNullRead>> = None;
                        for element in &parameters.payload.select {
                            match element {
                                ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(_) => {
                                    match sqlx::Row::try_get::<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
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
                                                        line: 1234,
                                                        column: 245,
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
                                ExampleColumn::StdPrimitiveI16AsSmallInt2NotNull(_) => match sqlx::Row::try_get::<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNullRead, &std::primitive::str>(&value, "std_primitive_i16_as_small_int2_not_null") {
                                    Ok(value) => {
                                        std_primitive_i16_as_small_int2_not_null = Some(postgresql_crud::Value { value: value });
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
                                                    line: 1269,
                                                    column: 245,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                },
                                ExampleColumn::ObjectAnimalAsJsonbNotNull(_) => match sqlx::Row::try_get::<ObjectAnimalAsPostgresqlJsonbNotNullRead, &std::primitive::str>(&value, "object_animal_as_jsonb_not_null") {
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
                                                    line: 1269,
                                                    column: 245,
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
                            std_primitive_i16_as_small_int2_not_null,
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
                                line: 3754,
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
        not_unique_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsSmallInt2NotNull {
        #[eo_to_std_string_string]
        std_primitive_i16_as_small_int2_not_null: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueObjectAnimalAsJsonbNotNull {
        #[eo_to_std_string_string]
        object_animal_as_jsonb_not_null: ObjectAnimalAsPostgresqlJsonbNotNull,
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
pub async fn try_read_many(server_location: &std::primitive::str, parameters: ReadManyParameters<'_>) -> Result<std::vec::Vec<ExampleOptions>, TryReadManyErrorNamed> {
    let payload = {
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2682,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2734,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2751,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2764,
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
        TryReadManyRouteLogicResponseVariants::NotUniqueStdPrimitiveI16AsSmallInt2NotNull {
            std_primitive_i16_as_small_int2_not_null,
            code_occurence,
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveI16AsSmallInt2NotNull {
            std_primitive_i16_as_small_int2_not_null,
            code_occurence,
        },
        TryReadManyRouteLogicResponseVariants::NotUniqueObjectAnimalAsJsonbNotNull { object_animal_as_jsonb_not_null, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueObjectAnimalAsJsonbNotNull { object_animal_as_jsonb_not_null, code_occurence },
    };
    Err(TryReadManyErrorNamed::TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_read_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2802,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ReadManyPayload<'_> {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i16_as_small_int2_not_null: Some(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: Some(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            object_animal_as_jsonb_not_null: Some(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            select: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            order_by: postgresql_crud::OrderBy {
                column: ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
                order: Some(postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            },
            pagination: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn read_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <ReadManyPayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead,
    pub select: std::vec::Vec<ExampleColumn<'a>>,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2554,
                        column: 264,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2626,
                            column: 249,
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
    let query_string = format!("select {} from example where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = $1", {
        let mut value = std::string::String::default();
        for element in &parameters.payload.select {
            value.push_str(&match element {
                ExampleColumn::StdPrimitiveI16AsSmallInt2NotNull(value) => {
                    <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::column_query_part(value, "std_primitive_i16_as_small_int2_not_null")
                }
                ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(value) => {
                    <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::column_query_part(
                        value,
                        "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                    )
                }
                ExampleColumn::ObjectAnimalAsJsonbNotNull(value) => <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::column_query_part(value, "object_animal_as_jsonb_not_null"),
            });
            value.push_str(",");
        }
        let _ = value.pop();
        value
    },);
    println!("{}", query_string);
    let binded_query = {
        let query = sqlx::query::<sqlx::Postgres>(&query_string);
        let query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, query);
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2576,
                        column: 253,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2576,
                        column: 253,
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
                    let mut std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead>> = None;
                    let mut std_primitive_i16_as_small_int2_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNullRead>> = None;
                    let mut object_animal_as_jsonb_not_null: std::option::Option<postgresql_crud::Value<ObjectAnimalAsPostgresqlJsonbNotNullRead>> = None;
                    for element in &parameters.payload.select {
                        match element {
                            ExampleColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(_) => {
                                match sqlx::Row::try_get::<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullRead, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
                                    Ok(value) => {
                                        std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = Some(postgresql_crud::Value { value: value });
                                    }
                                    Err(error_0) => {
                                        let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                            postgresql: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                    line: 1234,
                                                    column: 245,
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
                            ExampleColumn::StdPrimitiveI16AsSmallInt2NotNull(_) => match sqlx::Row::try_get::<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNullRead, &std::primitive::str>(&value, "std_primitive_i16_as_small_int2_not_null") {
                                Ok(value) => {
                                    std_primitive_i16_as_small_int2_not_null = Some(postgresql_crud::Value { value: value });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1269,
                                                column: 245,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                            ExampleColumn::ObjectAnimalAsJsonbNotNull(_) => match sqlx::Row::try_get::<ObjectAnimalAsPostgresqlJsonbNotNullRead, &std::primitive::str>(&value, "object_animal_as_jsonb_not_null") {
                                Ok(value) => {
                                    object_animal_as_jsonb_not_null = Some(postgresql_crud::Value { value: value });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1269,
                                                column: 245,
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
                        std_primitive_i16_as_small_int2_not_null,
                        std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                        object_animal_as_jsonb_not_null,
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
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 4079,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2682,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2734,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2751,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2764,
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
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2802,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ReadOnePayload {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            select: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn read_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <ReadOnePayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadElement<'a> {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate,
    pub std_primitive_i16_as_small_int2_not_null: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Update>>,
    pub object_animal_as_jsonb_not_null: std::option::Option<postgresql_crud::Value<<ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Update>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayload<'a>(pub std::vec::Vec<UpdateManyPayloadElement<'a>>);
#[derive(Debug)]
pub struct UpdateManyParameters<'a> {
    pub payload: UpdateManyPayload<'a>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate>),
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
        bind_query: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize,
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
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndRollback {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate>,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        #[eo_to_std_string_string]
        no_payload_fields_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2554,
                        column: 264,
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
                        if !acc.contains(&&element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key) {
                            acc.push(&element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key);
                        } else {
                            let error_0 = element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone();
                            let error = TryUpdateManyRouteLogicErrorNamed::NotUniquePrimaryKey {
                                not_unique_primary_key: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 4292,
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
                    if let (None, None) = (&element.std_primitive_i16_as_small_int2_not_null, &element.object_animal_as_jsonb_not_null) {
                        let error_0 = element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone();
                        let error = TryUpdateManyRouteLogicErrorNamed::NoPayloadFieldsPrimaryKey {
                            no_payload_fields_primary_key: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2079,
                                    column: 287,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2626,
                            column: 249,
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
        .map(|element| element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone())
        .collect::<std::vec::Vec<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate>>();
    let query_string = {
        let mut query = std::string::String::from("update example set ");
        let mut increment: std::primitive::u64 = 0;
        {
            let mut is_std_primitive_i_16_as_small_int_2_not_null_update_exist = false;
            for element in &parameters.payload.0 {
                if element.std_primitive_i16_as_small_int2_not_null.is_some() {
                    is_std_primitive_i_16_as_small_int_2_not_null_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_i_16_as_small_int_2_not_null_update_exist {
                let mut acc = std::string::String::from("std_primitive_i16_as_small_int2_not_null = case ");
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_i16_as_small_int2_not_null {
                        acc.push_str(&format!(
                            "when std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = {} then {} ",
                            match <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_part(
                                &element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                                &"",
                                &"std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                                &"",
                                &mut increment,
                            ) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    todo!()
                                }
                            },
                            match <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_part(
                                &value.value,
                                &"",
                                &"std_primitive_i16_as_small_int2_not_null",
                                &"",
                                &mut increment,
                            ) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    todo!()
                                }
                            }
                        ));
                    }
                }
                query.push_str(&format!("{}{}", acc, "else std_primitive_i16_as_small_int2_not_null end,"));
            }
        }
        {
            let mut is_object_animal_as_jsonb_not_null_update_exist = false;
            for element in &parameters.payload.0 {
                if element.object_animal_as_jsonb_not_null.is_some() {
                    is_object_animal_as_jsonb_not_null_update_exist = true;
                    break;
                }
            }
            if is_object_animal_as_jsonb_not_null_update_exist {
                let mut acc = std::string::String::from("object_animal_as_jsonb_not_null = case ");
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.object_animal_as_jsonb_not_null {
                        acc.push_str(&format!(
                            "when std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = {} then {} ",
                            match <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_part(
                                &element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                                &"",
                                &"std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                                &"",
                                &mut increment,
                            ) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    todo!()
                                }
                            },
                            match <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_part(&value.value, &"", &"object_animal_as_jsonb_not_null", &"", &mut increment,) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    todo!()
                                }
                            }
                        ));
                    }
                }
                query.push_str(&format!("{}{}", acc, "else object_animal_as_jsonb_not_null end,"));
            }
        }
        let _ = query.pop();
        query.push_str(&format!(" where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key in ({}) returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;", {
            let mut acc = std::string::String::default();
            for element in &parameters.payload.0 {
                match <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_part(
                    &element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                    &"",
                    &"std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
                    &"",
                    &mut increment,
                ) {
                    Ok(value) => {
                        acc.push_str(&format!("{value},"));
                    }
                    Err(error_0) => {
                        todo!()
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
            let mut is_std_primitive_i_16_as_small_int_2_not_null_update_exist = false;
            for element in &parameters.payload.0 {
                if element.std_primitive_i16_as_small_int2_not_null.is_some() {
                    is_std_primitive_i_16_as_small_int_2_not_null_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_i_16_as_small_int_2_not_null_update_exist {
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_i16_as_small_int2_not_null {
                        query = query.bind(element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone());
                        query = <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_bind(value.value.clone(), query);
                    }
                }
            }
        }
        {
            let mut is_object_animal_as_jsonb_not_null_update_exist = false;
            for element in &parameters.payload.0 {
                if element.object_animal_as_jsonb_not_null.is_some() {
                    is_object_animal_as_jsonb_not_null_update_exist = true;
                    break;
                }
            }
            if is_object_animal_as_jsonb_not_null_update_exist {
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.object_animal_as_jsonb_not_null {
                        query = query.bind(element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone());
                        query = <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_bind(value.value.clone(), query);
                    }
                }
            }
        }
        {
            for element in &parameters.payload.0 {
                query = <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_bind(
                    element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone(),
                    query,
                );
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2576,
                        column: 253,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2576,
                        column: 253,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2301,
                            column: 245,
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
                    Some(value) => match sqlx::Row::try_get::<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
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
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2974,
                                                column: 128,
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
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2974,
                                                column: 157,
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
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2976,
                                        column: 124,
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
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2976,
                                        column: 153,
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
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2161,
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
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2172,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2313,
                        column: 245,
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
        not_unique_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_update_many_route_logic_error_named_with_serialize_deserialize: TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_many(server_location: &std::primitive::str, parameters: UpdateManyParameters<'_>) -> Result<std::vec::Vec<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate>, TryUpdateManyErrorNamed> {
    let payload = {
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.0 {
            if !acc.contains(&&element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key) {
                acc.push(&element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key);
            } else {
                let error_0 = element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone();
                return Err(TryUpdateManyErrorNamed::NotUniquePrimaryKey {
                    not_unique_primary_key: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 4587,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2682,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2734,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2751,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2764,
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
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2802,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for UpdateManyPayloadElement<'_> {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_primitive_i16_as_small_int2_not_null: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            object_animal_as_jsonb_not_null: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        }
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for UpdateManyPayload<'_> {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(vec![
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ])
    }
}
pub async fn update_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <UpdateManyPayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateOnePayload<'a> {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate,
    pub std_primitive_i16_as_small_int2_not_null: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Update>>,
    pub object_animal_as_jsonb_not_null: std::option::Option<postgresql_crud::Value<<ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType<'a>>::Update>>,
}
#[derive(Debug)]
pub struct UpdateOneParameters<'a> {
    pub payload: UpdateOnePayload<'a>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateOneRouteLogicResponseVariants {
    Desirable(postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate),
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
        bind_query: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize,
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
        bind_query: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        #[eo_to_std_string_string]
        no_payload_fields_primary_key: postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2554,
                        column: 264,
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
                if let (None, None) = (&value.std_primitive_i16_as_small_int2_not_null, &value.object_animal_as_jsonb_not_null) {
                    let error_0 = value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone();
                    let error = TryUpdateOneRouteLogicErrorNamed::NoPayloadFieldsPrimaryKey {
                        no_payload_fields_primary_key: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 2079,
                                column: 287,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2626,
                            column: 249,
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
        if let Some(value) = &parameters.payload.std_primitive_i16_as_small_int2_not_null {
            match <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_part(&value.value, &"", &"std_primitive_i16_as_small_int2_not_null", &"", &mut increment) {
                Ok(value) => {
                    query.push_str(&format!("std_primitive_i16_as_small_int2_not_null = {value},"));
                }
                Err(error_0) => {
                    todo!()
                }
            }
        }
        if let Some(value) = &parameters.payload.object_animal_as_jsonb_not_null {
            match <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_part(&value.value, &"", &"object_animal_as_jsonb_not_null", &"", &mut increment) {
                Ok(value) => {
                    query.push_str(&format!("object_animal_as_jsonb_not_null = {value},"));
                }
                Err(error_0) => {
                    todo!()
                }
            }
        }
        let _ = query.pop();
        match <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_part(
            &parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            &"",
            &"std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
            &"",
            &mut increment,
        ) {
            Ok(value) => {
                query.push_str(&format!(" where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = {value}"));
            }
            Err(error_0) => {
                todo!()
            }
        }
        query.push_str(&format!("returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"));
        query
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters.payload.std_primitive_i16_as_small_int2_not_null {
            query = <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI16AsPostgresqlInt2NotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_bind(value.value, query);
        }
        if let Some(value) = parameters.payload.object_animal_as_jsonb_not_null {
            query = <ObjectAnimalAsPostgresqlJsonbNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_bind(value.value, query);
        }
        query = <postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::update_query_bind(
            parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2576,
                        column: 253,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2576,
                        column: 253,
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2301,
                            column: 245,
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
                Ok(value) => match sqlx::Row::try_get::<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
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
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2990,
                                        column: 111,
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
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2990,
                                        column: 140,
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
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2992,
                                    column: 107,
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
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2992,
                                    column: 136,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2313,
                        column: 245,
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
pub async fn try_update_one(server_location: &std::primitive::str, parameters: UpdateOneParameters<'_>) -> Result<postgresql_crud::postgresql_type::postgresql_type::StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullUpdate, TryUpdateOneErrorNamed> {
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
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2682,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2734,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2751,
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
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2764,
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
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2802,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for UpdateOnePayload<'_> {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            std_primitive_i16_as_small_int2_not_null: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
            object_animal_as_jsonb_not_null: Some(postgresql_crud::Value {
                value: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            }),
        }
    }
}
pub async fn update_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <UpdateOnePayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
