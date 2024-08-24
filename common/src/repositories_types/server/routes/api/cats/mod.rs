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



//todo "serde_json": "unknown variant `ddfd`, expected one of `something`, `omega`, `doggie` at line 6 column 45"
//if doggie is object - wrong message. fix it
//from_str :: FromStr,
pub async fn try_read_one_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, >,
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
            let error = TryReadOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2219,
                        column: 17,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryReadOneRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    // let parameters =
    //     ReadOneParameters {
    //         payload: match serde_json::from_slice::<ReadOnePayload>(&body_bytes) {
    //             Ok(value) => {
    //                 let value = ReadOnePayload::from(value);
    //                 let mut acc = std::vec::Vec::new();
    //                 for element in &value.select {
    //                     if acc.contains(&element) {
    //                         let error_0 = element.clone();//here
    //                         let error = TryReadOneRouteLogicErrorNamed :: NotUniqueColumn { 
    //                             not_unique_column: error_0, 
    //                             code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //                                 file!().to_owned(),
    //                                 line!(),
    //                                 column!(),
    //                         Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                                     file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
    //                                     line: 1647, 
    //                                     column: 13,
    //                                 })
    //                             )
    //                         };
    //                         eprintln!("{error}");
    //                         let mut response = axum::response::IntoResponse::into_response(
    //                             axum::Json(TryReadOneRouteLogicResponseVariants::from(error)),
    //                         );
    //                         *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
    //                         return response;
    //                     } else {
    //                         match &element {
    //                             JsongenericColumn::StdPrimitiveI32AsPostgresqlInt => (),
    //                             JsongenericColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => (),
    //                             JsongenericColumn::SqlxTypesJsonTAsPostgresqlJsonBNotNull { filter } => {
    //                                 if filter.is_empty() {
    //                                     let error_0 = element.clone();//here
    //                                     let error = TryReadOneRouteLogicErrorNamed::EmptyColumnJsonReader { 
    //                                         empty_column_json_reader: error_0, 
    //                                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //                                             file!().to_owned(),
    //                                             line!(),
    //                                             column!(),
    //                                             Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                                                 file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
    //                                                 line: 1647, 
    //                                                 column: 13,
    //                                             })
    //                                         )
    //                                     };
    //                                     eprintln!("{error}");
    //                                     let mut response = axum::response::IntoResponse::into_response(
    //                                         axum::Json(TryReadOneRouteLogicResponseVariants::from(error)),
    //                                     );
    //                                     *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
    //                                     return response;
    //                                 }
    //                                 {
    //                                     let mut acc = vec![];
    //                                     for element_handle in filter {
    //                                         match acc.contains(&element_handle) {
    //                                             true => {
    //                                                 let error_0 = element.clone();//here
    //                                                 let error = TryReadOneRouteLogicErrorNamed :: NotUniqueColumnJsonReader { 
    //                                                     not_unique_column_json_reader: error_0, 
    //                                                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //                                                         file!().to_owned(),
    //                                                         line!(),
    //                                                         column!(),
    //                                                         Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                                                             file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
    //                                                             line: 1647, 
    //                                                             column: 13,
    //                                                         })
    //                                                     )
    //                                                 };
    //                                                 eprintln!("{error}");
    //                                                 let mut response = axum::response::IntoResponse::into_response(
    //                                                     axum::Json(TryReadOneRouteLogicResponseVariants::from(error)),
    //                                                 );
    //                                                 *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
    //                                                 return response;
    //                                             },
    //                                             false => {
    //                                                 acc.push(element_handle);
    //                                             }
    //                                         }
    //                                     }
    //                                 }
    //                             },
    //                         }
    //                         acc.push(element);
    //                     }
    //                 }
    //                 value
    //             }
    //             Err(error_0) => {
    //                 let error = TryReadOneRouteLogicErrorNamed::SerdeJson {
    //                     serde_json: error_0,
    //                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
    //                         file!().to_owned(),
    //                         line!(),
    //                         column!(),
    //                         Some(error_occurence_lib::code_occurence::MacroOccurence {
    //                             file: std::string::String::from(
    //                                 "postgresql_crud/generate_postgresql_crud/src/lib.rs",
    //                             ),
    //                             line: 2321,
    //                             column: 13,
    //                         }),
    //                     ),
    //                 };
    //                 eprintln!("{error}");
    //                 let mut response = axum::response::IntoResponse::into_response(axum::Json(
    //                     TryReadOneRouteLogicResponseVariants::from(error),
    //                 ));
    //                 *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
    //                 return response;
    //             }
    //         },
    //     };
    //
    let parameters = ReadOneParameters {
        payload: match serde_json::from_slice::<ReadOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = ReadOnePayload::from(value);
                let mut acc = std::vec::Vec::new();
                for element in &value.select {
                    if acc.contains(&element) {
                        let error_0 = element.clone();
                        let error =
                TryReadOneRouteLogicErrorNamed :: NotUniqueColumn
                {
                    not_unique_column : error_0, code_occurence :
                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                    new(file! ().to_owned(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 1828, column : 13,
                    }))
                };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(
                            axum::Json(TryReadOneRouteLogicResponseVariants::from(error)),
                        );
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
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2491,
                            column: 13,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryReadOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    //
    println!("{:#?}", parameters);
    let query_string = format!
    ("select {} from jsongeneric where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = $1",
    {
        let mut value =
        parameters.payload.select.iter().fold(std::string::String::from(""), | mut acc, element |
        {
            acc.push_str(&match element {
                JsongenericColumn::StdPrimitiveI32AsPostgresqlInt => "std_primitive_i32_as_postgresql_int".to_string(), 
                JsongenericColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key".to_string(),
                JsongenericColumn::SqlxTypesJsonTAsPostgresqlJsonBNotNull{ filter } => format!(
                    "{} as sqlx_types_json_t_as_postgresql_json_b_not_null",//todo should support arrays or "key: array" is enough? 
                    match postgresql_crud::GeneratePostgresqlQueryPart::generate_postgresql_query_part_from_self_vec(
                        filter, 
                        "sqlx_types_json_t_as_postgresql_json_b_not_null",
                        "sqlx_types_json_t_as_postgresql_json_b_not_null",
                        false
                    ) {
                        Ok(value) => value,
                        Err(error) => {
                            //SomethingGeneratePostgresqlQueryPartFromSelfVecErrorNamed
                            let f: bool = error;
                            // let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                            //     postgresql: error_0,
                            //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            //         file!().to_owned(),
                            //         line!(),
                            //         column!(),
                            //         Some(error_occurence_lib::code_occurence::MacroOccurence {
                            //             file: std::string::String::from(
                            //                 "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            //             ),
                            //             line: 2252,
                            //             column: 17,
                            //         }),
                            //     ),
                            // };
                            // eprintln!("{error}");
                            // let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            //     TryReadOneRouteLogicResponseVariants::from(error),
                            // ));
                            // *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            // return response;
                            todo!()
                        }
                    }
                )
            }); 
            acc.push_str(","); 
            acc
        }); 
        let _ = value.pop();
        value
    },);
    println!("{}", query_string);
    let binded_query = {
        let query = sqlx::query::<sqlx::Postgres>(&query_string);
        let query = postgresql_crud::BindQuery::bind_value_to_query(
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
            let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2252,
                        column: 17,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryReadOneRouteLogicResponseVariants::from(error),
            ));
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
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2252,
                        column: 17,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryReadOneRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => {
                    let mut std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<postgresql_crud::Value<postgresql_crud::StdPrimitiveI64>> = None;
                    let mut std_primitive_i32_as_postgresql_int: std::option::Option<postgresql_crud::Value<postgresql_crud::StdOptionOptionStdPrimitiveI32>> = None;
                    //todo change type
                    let mut sqlx_types_json_t_as_postgresql_json_b_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::SqlxTypesJson<SomethingWrapper>>> = None;
                    for element in &parameters.payload.select {
                        match element
                        {
                            JsongenericColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => match sqlx::Row::try_get::<std::primitive::i64, &std::primitive::str> (& value,
                            "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key")
                            {
                                Ok(value) =>
                                {
                                    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                                    =
                                    Some(postgresql_crud :: Value
                                    { value : postgresql_crud::StdPrimitiveI64(value) });
                                }, 
                                Err(error_0) =>
                                {
                                    let error = TryReadOneRouteLogicErrorNamed :: Postgresql
                                    {
                                        postgresql : error_0, code_occurence : error_occurence_lib
                                        :: code_occurence :: CodeOccurence ::
                                        new(file! ().to_owned(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 988, column : 17,
                                        }))
                                    }; eprintln! ("{error}"); let mut response = axum ::
                                    response :: IntoResponse ::
                                    into_response(axum ::
                                    Json(TryReadOneRouteLogicResponseVariants :: from(error)));
                                    * response.status_mut() = axum :: http :: StatusCode ::
                                    INTERNAL_SERVER_ERROR; return response;
                                }
                            }, 
                            
                            JsongenericColumn :: StdPrimitiveI32AsPostgresqlInt =>
                            match sqlx :: Row :: try_get :: <
                            std::option::Option<std::primitive::i32> , & std ::
                            primitive :: str >
                            (& value, "std_primitive_i32_as_postgresql_int")
                            {
                                Ok(value) =>
                                {
                                    std_primitive_i32_as_postgresql_int =
                                    Some(postgresql_crud :: Value
                                    {
                                        value :
                                        postgresql_crud::StdOptionOptionStdPrimitiveI32(match value
                                        {
                                            Some(value) =>
                                            Some(postgresql_crud::StdPrimitiveI32(value)), None => None
                                        })
                                    });
                                }, Err(error_0) =>
                                {
                                    let error = TryReadOneRouteLogicErrorNamed :: Postgresql
                                    {
                                        postgresql : error_0, code_occurence : error_occurence_lib
                                        :: code_occurence :: CodeOccurence ::
                                        new(file! ().to_owned(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 1028, column : 17,
                                        }))
                                    }; eprintln! ("{error}"); let mut response = axum ::
                                    response :: IntoResponse ::
                                    into_response(axum ::
                                    Json(TryReadOneRouteLogicResponseVariants :: from(error)));
                                    * response.status_mut() = axum :: http :: StatusCode ::
                                    INTERNAL_SERVER_ERROR; return response;
                                }
                            }, 
                            
                            JsongenericColumn::SqlxTypesJsonTAsPostgresqlJsonBNotNull{ filter } => match sqlx::Row::try_get::<sqlx::types::Json::<SomethingWrapper>, &std::primitive::str>(
                                &value,
                                "sqlx_types_json_t_as_postgresql_json_b_not_null"
                            ) {
                                Ok(value) => {
                                    sqlx_types_json_t_as_postgresql_json_b_not_null = Some(postgresql_crud::Value { value: postgresql_crud::SqlxTypesJson(value) });
                                },
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1028,
                                                column: 17,
                                            })
                                        )
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        }
                    }
                    JsongenericOptions {
                        std_primitive_i32_as_postgresql_int,
                        std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                        sqlx_types_json_t_as_postgresql_json_b_not_null,
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
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 3779,
                                column: 29,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryReadOneRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        };
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryReadOneRouteLogicResponseVariants::Desirable(value),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
pub async fn try_read_one(
    server_location: &std::primitive::str,
    parameters: ReadOneParameters,
) -> Result<JsongenericOptions, TryReadOneErrorNamed> {
    let payload = {
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.select {
            if acc.contains(&element) {
                let error_0 = element.clone();//here
                return Err(TryReadOneErrorNamed::NotUniqueColumn {
                    not_unique_column: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1668,
                            column: 13,
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
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2364,
                            column: 17,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/jsongeneric/read_one", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
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
            return Err(TryReadOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2427,
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
            return Err(TryReadOneErrorNamed::FailedToGetResponseText {
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
                        line: 2449,
                        column: 17,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryReadOneRouteLogicResponseVariants>(&error_2) {
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
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2467,
                            column: 17,
                        }),
                    ),
                });
            }
        };
    let try_read_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryReadOneRouteLogicResponseVariants::Desirable(value) => {
            let value = JsongenericOptions::from(value);
            return Ok(value);
        }
        TryReadOneRouteLogicResponseVariants::CheckBodySize {
            check_body_size,
            code_occurence,
        } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize {
            check_body_size,
            code_occurence,
        },
        TryReadOneRouteLogicResponseVariants::Postgresql {
            postgresql,
            code_occurence,
        } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql {
            postgresql,
            code_occurence,
        },
        TryReadOneRouteLogicResponseVariants::SerdeJson {
            serde_json,
            code_occurence,
        } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson {
            serde_json,
            code_occurence,
        },
        TryReadOneRouteLogicResponseVariants::CheckCommit {
            check_commit,
            code_occurence,
        } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit {
            check_commit,
            code_occurence,
        },
        TryReadOneRouteLogicResponseVariants::NotUniqueColumn {
            not_unique_column,
            code_occurence,
        } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn {
            not_unique_column,
            code_occurence,
        },
        TryReadOneRouteLogicResponseVariants::EmptyColumnJsonReader {
            empty_column_json_reader,
            code_occurence,
        } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::EmptyColumnJsonReader {
            empty_column_json_reader,
            code_occurence,
        },
        TryReadOneRouteLogicResponseVariants::NotUniqueColumnJsonReader {
            not_unique_column_json_reader,
            code_occurence,
        } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumnJsonReader {
            not_unique_column_json_reader,
            code_occurence,
        },
    };
    Err(
        TryReadOneErrorNamed::TryReadOneRouteLogicErrorNamedWithSerializeDeserialize {
            try_read_one_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2510,
                    column: 17,
                }),
            ),
        },
    )
}


//////////////////////
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