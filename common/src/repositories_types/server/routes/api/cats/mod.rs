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
            check_commit: route_validators::check_commit::CheckCommitErrorNamed,
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
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        //todo use reserved work instead of TryCreateManyRouteLogicErrorNamed
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        eprintln!("{error}");
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TryCreateManyRouteLogicResponseVariants::from(error),
        ));
        *response.status_mut() = status_code;
        return response;
    }
}]
pub struct Dog {
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdPrimitiveBoolAsPostgresqlBool,
    // pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt,
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
    // pub sqlx_types_json_t_as_postgresql_json: postgresql_crud::SqlxTypesJsonTAsPostgresqlJson::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_not_null: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonNotNull::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_b: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonB::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonBNotNull::<Something>,//todo

    // pub serde_json_value_as_postgresql_json: postgresql_crud::SerdeJsonValueAsPostgresqlJson,
    // pub serde_json_value_as_postgresql_json_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonNotNull,
    // pub serde_json_value_as_postgresql_json_b: postgresql_crud::SerdeJsonValueAsPostgresqlJsonB,
    // pub serde_json_value_as_postgresql_json_b_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonBNotNull,
}

// #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
// pub struct Something {
//     something: std::string::String,
// }

/////////////
#[derive(Debug)]
pub struct UpdateManyPayloadElement {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64,
    pub std_primitive_bool_as_postgresql_bool:
        OptionField<postgresql_crud::StdOptionOptionStdPrimitiveBool>,
    pub std_primitive_i16_as_postgresql_small_int:
        OptionField<postgresql_crud::StdOptionOptionStdPrimitiveI16>,
    pub std_primitive_i32_as_postgresql_int:
        OptionField<postgresql_crud::StdOptionOptionStdPrimitiveI32>,
}
#[derive(Debug)]
pub struct UpdateManyPayload(pub std::vec::Vec<UpdateManyPayloadElement>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadElementWithSerializeDeserialize {
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
    std_primitive_bool_as_postgresql_bool: OptionFieldWithSerializeDeserialize<
        postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
    >,
    std_primitive_i16_as_postgresql_small_int: OptionFieldWithSerializeDeserialize<
        postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
    >,
    std_primitive_i32_as_postgresql_int: OptionFieldWithSerializeDeserialize<
        postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
    >,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadWithSerializeDeserialize(
    pub std::vec::Vec<UpdateManyPayloadElementWithSerializeDeserialize>,
);
impl std::convert::From<UpdateManyPayloadElementWithSerializeDeserialize>
    for UpdateManyPayloadElement
{
    fn from(value: UpdateManyPayloadElementWithSerializeDeserialize) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let std_primitive_bool_as_postgresql_bool = OptionField {
            value: match value.std_primitive_bool_as_postgresql_bool.value {
                Some(value) => Some(postgresql_crud::StdOptionOptionStdPrimitiveBool::from(
                    value,
                )),
                None => None,
            },
        };
        let std_primitive_i16_as_postgresql_small_int = OptionField {
            value: match value.std_primitive_i16_as_postgresql_small_int.value {
                Some(value) => Some(postgresql_crud::StdOptionOptionStdPrimitiveI16::from(value)),
                None => None,
            },
        };
        let std_primitive_i32_as_postgresql_int = OptionField {
            value: match value.std_primitive_i32_as_postgresql_int.value {
                Some(value) => Some(postgresql_crud::StdOptionOptionStdPrimitiveI32::from(value)),
                None => None,
            },
        };
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        }
    }
}
impl std::convert::From<UpdateManyPayloadWithSerializeDeserialize> for UpdateManyPayload {
    fn from(value: UpdateManyPayloadWithSerializeDeserialize) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|element| UpdateManyPayloadElement::from(element))
                .collect(),
        )
    }
}
impl std::convert::From<UpdateManyPayloadElement>
    for UpdateManyPayloadElementWithSerializeDeserialize
{
    fn from(value: UpdateManyPayloadElement) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let std_primitive_bool_as_postgresql_bool = OptionFieldWithSerializeDeserialize {
            value: match value.std_primitive_bool_as_postgresql_bool.value {
                Some(value) => Some(
                    postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                        value,
                    ),
                ),
                None => None,
            },
        };
        let std_primitive_i16_as_postgresql_small_int = OptionFieldWithSerializeDeserialize {
            value: match value.std_primitive_i16_as_postgresql_small_int.value {
                Some(value) => Some(
                    postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                        value,
                    ),
                ),
                None => None,
            },
        };
        let std_primitive_i32_as_postgresql_int = OptionFieldWithSerializeDeserialize {
            value: match value.std_primitive_i32_as_postgresql_int.value {
                Some(value) => Some(
                    postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                        value,
                    ),
                ),
                None => None,
            },
        };
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        }
    }
}
impl std::convert::From<UpdateManyPayload> for UpdateManyPayloadWithSerializeDeserialize {
    fn from(value: UpdateManyPayload) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|element| UpdateManyPayloadElementWithSerializeDeserialize::from(element))
                .collect(),
        )
    }
}
#[derive(Debug)]
pub struct UpdateManyParameters {
    pub payload: UpdateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>),
    CheckBodySize {
        check_body_size:
            route_validators::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: route_validators::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        query: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        primary_key_from_row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndFailedRollback {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        commit_failed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        not_unique_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyRouteLogicErrorNamed>
    for TryUpdateManyRouteLogicResponseVariants
{
    fn from(value: TryUpdateManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            QueryAndRollbackFailed { query, rollback, code_occurence } => Self
            :: QueryAndRollbackFailed { query, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback, code_occurence } => Self ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback, code_occurence } => Self ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CommitFailed { commit_failed, code_occurence } => Self ::
            CommitFailed { commit_failed, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniquePrimaryKey { not_unique_primary_key, code_occurence } =>
            Self :: NotUniquePrimaryKey
            { not_unique_primary_key, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        #[eo_to_std_string_string]
        json: axum::extract::rejection::JsonRejection,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: route_validators::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        #[eo_to_std_string_string]
        query: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        #[eo_to_std_string_string]
        primary_key_from_row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndFailedRollback {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        #[eo_to_std_string_string]
        commit_failed: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::StdPrimitiveI64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_many_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, >,
    request: axum::extract::Request,
) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match route_validators::check_body_size::check_body_size(
        body,
        *app_state.get_maximum_size_of_http_body_in_bytes(),
    )
    .await
    {
        Ok(value) => value,
        Err(error) => {
            let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
            let error = TryUpdateManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1720,
                        column: 13,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateManyRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = status_code;
            return response;
        }
    };
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        eprintln!("{error}");
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TryCreateManyRouteLogicResponseVariants::from(error),
        ));
        *response.status_mut() = status_code;
        return response;
    }
    let parameters = UpdateManyParameters {
        payload: match axum::Json::<UpdateManyPayloadWithSerializeDeserialize>::from_bytes(
            &body_bytes,
        ) {
            Ok(axum::Json(value)) => {
                let value = UpdateManyPayload::from(value);
                let mut acc = std::vec::Vec::new();
                for element in &value.0 {
                    if !acc.contains(
                        &element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                    ) {
                        acc.push(
                            element
                                .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                                .clone(),
                        );
                    } else {
                        let error = TryUpdateManyRouteLogicErrorNamed::NotUniquePrimaryKey {
                            not_unique_primary_key: element
                                .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                                .clone(),
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1901,
                                    column: 21,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryUpdateManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                value
            }
            Err(error) => {
                let error = TryUpdateManyRouteLogicErrorNamed::Json {
                    json: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1809,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let expected_updated_primary_keys = parameters
        .payload
        .0
        .iter()
        .map(|element| {
            element
                .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                .clone()
        })
        .collect::<std::vec::Vec<postgresql_crud::StdPrimitiveI64>>();
    //
//     let query_string = {
//         let mut increment = 0;
//         //
// // #[derive(Debug)]
// // pub struct UpdateManyPayloadElement {
// //     pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
// //         postgresql_crud::StdPrimitiveI64,
// //     pub std_primitive_bool_as_postgresql_bool:
// //         OptionField<postgresql_crud::StdOptionOptionStdPrimitiveBool>,
// //     pub std_primitive_i16_as_postgresql_small_int:
// //         OptionField<postgresql_crud::StdOptionOptionStdPrimitiveI16>,
// //     pub std_primitive_i32_as_postgresql_int:
// //         OptionField<postgresql_crud::StdOptionOptionStdPrimitiveI32>,
// // }
//         //
//         let mut fields_acc = std::string::String::from("update dogs set ");
//         // let std_primitive_bool_as_postgresql_bool_handle = {
//         let is_std_primitive_bool_as_postgresql_bool_update_exist = false;
//         for element in &parameters.payload.0 {
//             if element.std_primitive_bool_as_postgresql_bool.value.is_some() {
//                 is_std_primitive_bool_as_postgresql_bool_update_exist = true;
//                 break;
//             }
//         }
//         if is_std_primitive_bool_as_postgresql_bool_update_exist {
//             let mut acc = std::string::String::default();
//             for element in &parameters.payload.0 {
//                 if let Some(value) = &element.std_primitive_bool_as_postgresql_bool.value {

//                 }
//             }
//             // let mut acc = std::string::String::default();
//                 // let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_increment_handle = match postgresql_crud::BindQuery::try_generate_bind_increments(&element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, &mut increment) {
//                 //     Ok(value) => value,
//                 //     Err(error) => {
//                 //         todo!()
//                 //     }
//                 // };
//                 // let std_primitive_bool_as_postgresql_bool_increment_handle = match postgresql_crud::BindQuery::try_generate_bind_increments(&value, &mut increment) {
//                 //     Ok(value) => value,
//                 //     Err(error) => {
//                 //         todo!()
//                 //     }
//                 // };
//                 // acc.push_str(&format!("WHEN std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = {std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_increment_handle} THEN {std_primitive_bool_as_postgresql_bool_increment_handle} "));
//             // let _ = acc.pop();
//         }



//             format!(r#"
//                 std_primitive_bool_as_postgresql_bool = 
//                     CASE 
//                         WHEN std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 1 THEN false
//                         WHEN std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 2 THEN null
//                         WHEN std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 3 THEN false
//                         ELSE std_primitive_bool_as_postgresql_bool  -- Keep the current status if no condition matches
//                     END,
//             "#)
//         // };
//         //
//         let primary_keys = {
//             let mut acc = std::string::String::default();
//             for element in &parameters.payload.0 {
//                 match postgresql_crud::BindQuery::try_generate_bind_increments(&element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, &mut increment) {
//                     Ok(value) => {
//                         acc.push_str(&format!("{value},"));
//                     },
//                     Err(error) => {
//                         todo!()
//                     }
//                 }
//             }
//             let _ = acc.pop();
//             acc
//         };
//         //
//         format!("
//             UPDATE dogs
//             SET 
//                 std_primitive_bool_as_postgresql_bool = 
//                     CASE 
//                         WHEN std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 1 THEN false
//                         WHEN std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 2 THEN null
//                         WHEN std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 3 THEN false
//                         ELSE std_primitive_bool_as_postgresql_bool  -- Keep the current status if no condition matches
//                     END,
//                 std_primitive_i16_as_postgresql_small_int = 
//                     CASE 
//                         WHEN std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 4 THEN 10
//                         WHEN std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 5 THEN null
//                         WHEN std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 9 THEN 10  -- Reset ship_date to NULL for order_id = 3
//                         ELSE std_primitive_i16_as_postgresql_small_int  -- Keep the current ship_date if no condition matches
//                     END
//             WHERE std_primitive_i64_as_postgresql_big_serial_not_null_primary_key IN ({primary_keys});
//         ")
//     };
        let query_string = "";
    // r#"
    //     update dogs as t 
    //     set 
    //     std_primitive_bool_as_postgresql_bool = data.std_primitive_bool_as_postgresql_bool, 
    //     std_primitive_i16_as_postgresql_small_int = data.std_primitive_i16_as_postgresql_small_int, 
    //     std_primitive_i32_as_postgresql_int = data.std_primitive_i32_as_postgresql_int 
    //     from (select * from unnest($1, $2, $3, $4)) 
    //     as 
    //     data(
    //         std_primitive_bool_as_postgresql_bool, 
    //         std_primitive_i16_as_postgresql_small_int, 
    //         std_primitive_i32_as_postgresql_int, 
    //         sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
    //     ) 
    //     where 
    //     t.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key = data.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key 
    //     returning data.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
    // "#;
    //
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        let current_vec_len = parameters.payload.0.len();
        let (
            std_primitive_bool_as_postgresql_bool_vec,
            std_primitive_i16_as_postgresql_small_int_vec,
            std_primitive_i32_as_postgresql_int_vec,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
        ) = parameters.payload.0.into_iter().fold(
            (
                std::vec::Vec::with_capacity(current_vec_len),
                std::vec::Vec::with_capacity(current_vec_len),
                std::vec::Vec::with_capacity(current_vec_len),
                std::vec::Vec::with_capacity(current_vec_len),
            ),
            |mut acc, element| {
                acc.0.push(element.std_primitive_bool_as_postgresql_bool);
                acc.1
                    .push(element.std_primitive_i16_as_postgresql_small_int);
                acc.2.push(element.std_primitive_i32_as_postgresql_int);
                acc.3
                    .push(element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key);
                acc
            },
        );
        query = query.bind(
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec
                .into_iter()
                .map(|element| element.into_inner())
                .collect::<std::vec::Vec<std::primitive::i64>>(),
        );
        query = query.bind(
            std_primitive_bool_as_postgresql_bool_vec
                .into_iter()
                .map(|element| match element.value {
                    Some(value) => value.into_inner(),
                    None => None,
                })
                .collect::<std::vec::Vec<std::option::Option<std::primitive::bool>>>(),
        );
        query = query.bind(
            std_primitive_i16_as_postgresql_small_int_vec
                .into_iter()
                .map(|element| match element.value {
                    Some(value) => value.into_inner(),
                    None => None,
                })
                .collect::<std::vec::Vec<std::option::Option<std::primitive::i16>>>(),
        );
        query = query.bind(
            std_primitive_i32_as_postgresql_int_vec
                .into_iter()
                .map(|element| match element.value {
                    Some(value) => value.into_inner(),
                    None => None,
                })
                .collect::<std::vec::Vec<std::option::Option<std::primitive::i32>>>(),
        );
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error) => {
            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1768,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error) => {
            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1768,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let value = {
        let mut postgres_transaction = match {
            use sqlx::Acquire;
            pg_connection.begin()
        }
        .await
        {
            Ok(value) => value,
            Err(error) => {
                let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                    postgresql: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1768,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let results_vec =
            {
                let mut results_vec =
                    std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
                let mut option_error: Option<sqlx::Error> = None;
                {
                    let mut rows = binded_query.fetch(postgres_transaction.as_mut());
                    while let (Some(Some(row)), None) = (
                        match {
                            use futures::TryStreamExt;
                            rows.try_next()
                        }
                        .await
                        {
                            Ok(value) => Some(value),
                            Err(error) => {
                                option_error = Some(error);
                                None
                            }
                        },
                        &option_error,
                    ) {
                        results_vec.push(row);
                    }
                }
                if let Some(error) = option_error {
                    match postgres_transaction.rollback().await {
                        Ok(_) => {
                            let error = TryUpdateManyRouteLogicErrorNamed :: Postgresql
                        {
                            postgresql : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1768, column : 21,
                            })),
                        };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(rollback_error) => {
                            let error = TryUpdateManyRouteLogicErrorNamed ::
                        QueryAndRollbackFailed
                        {
                            query : error, rollback : rollback_error, code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 874, column : 21,
                            })),
                        };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
                results_vec
            };
        let primary_key_vec = {
            let mut primary_key_vec =
                std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
            for element in results_vec {
                match primary_key_try_from_sqlx_row(&element) {
                    Ok(primary_key) => {
                        primary_key_vec.push(primary_key);
                    }
                    Err(error) => match postgres_transaction.rollback().await {
                        Ok(_) => {
                            let error = TryUpdateManyRouteLogicErrorNamed :: Postgresql
                            {
                                postgresql : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1768, column : 21,
                                })),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(rollback_error) => {
                            let error = TryUpdateManyRouteLogicErrorNamed ::
                            PrimaryKeyFromRowAndFailedRollback
                            {
                                primary_key_from_row : error, rollback : rollback_error,
                                code_occurence : error_occurence_lib :: code_occurence ::
                                CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 920, column : 21,
                                })),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                }
            }
            primary_key_vec
        };
        {
            let non_existing_primary_keys = {
                let len = expected_updated_primary_keys.len();
                expected_updated_primary_keys.into_iter().fold(
                    std::vec::Vec::with_capacity(len),
                    |mut acc, element| {
                        if let false = primary_key_vec.contains(&element) {
                            acc.push(element);
                        }
                        acc
                    },
                )
            };
            if let false = non_existing_primary_keys.is_empty() {
                match postgres_transaction.rollback().await {
                    Ok(_) => {
                        let error = TryUpdateManyRouteLogicErrorNamed::NonExistingPrimaryKeys {
                            non_existing_primary_keys,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1128,
                                    column: 21,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryUpdateManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                    Err(error) => {
                        let error = TryUpdateManyRouteLogicErrorNamed ::
                        NonExistingPrimaryKeysAndFailedRollback
                        {
                            non_existing_primary_keys, rollback : error, code_occurence
                            : error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1171, column : 21,
                            })),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryUpdateManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
            }
        }
        match postgres_transaction.commit().await {
            Ok(_) => primary_key_vec
                .into_iter()
                .map(|element| {
                    postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(element)
                })
                .collect(),
            Err(error) => {
                let error = TryUpdateManyRouteLogicErrorNamed::CommitFailed {
                    commit_failed: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1208,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        }
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryUpdateManyRouteLogicResponseVariants::Desirable(value),
    ));
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
        not_unique_primary_key: postgresql_crud::StdPrimitiveI64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_update_many_route_logic_error_named_with_serialize_deserialize:
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_many(
    server_location: &std::primitive::str,
    parameters: UpdateManyParameters,
) -> Result<std::vec::Vec<postgresql_crud::StdPrimitiveI64>, TryUpdateManyErrorNamed> {
    let payload = {
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.0 {
            if !acc
                .contains(&&element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key)
            {
                acc.push(&element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key);
            } else {
                return Err(TryUpdateManyErrorNamed::NotUniquePrimaryKey {
                    not_unique_primary_key: element
                        .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                        .clone(),
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1901,
                            column: 21,
                        }),
                    ),
                });
            }
        }
        let value = UpdateManyPayloadWithSerializeDeserialize::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryUpdateManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1535,
                            column: 21,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/dogs/update_many", server_location,);
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
        Err(error) => {
            return Err(TryUpdateManyErrorNamed::Reqwest {
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1694,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryUpdateManyErrorNamed::FailedToGetResponseText {
                status_code,
                headers,
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1591,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryUpdateManyRouteLogicResponseVariants>(&response_text) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryUpdateManyErrorNamed::DeserializeResponse {
                    status_code,
                    headers,
                    response_text,
                    serde: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1654,
                            column: 21,
                        }),
                    ),
                });
            }
        };
    let try_update_many_route_logic_error_named_with_serialize_deserialize
    = match expected_response
    {
        TryUpdateManyRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value =
            value.into_iter().map(| element | postgresql_crud::StdPrimitiveI64
            :: from(element)).collect(); return Ok(value);
        }, TryUpdateManyRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        Postgresql { postgresql, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: Json
        { json, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryUpdateManyRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckCommit { check_commit, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: QueryAndRollbackFailed
        { query, rollback, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        QueryAndRollbackFailed { query, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants ::
        PrimaryKeyFromRowAndFailedRollback
        { primary_key_from_row, rollback, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        PrimaryKeyFromRowAndFailedRollback
        { primary_key_from_row, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: NonExistingPrimaryKeys
        { non_existing_primary_keys, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence },
        TryUpdateManyRouteLogicResponseVariants ::
        NonExistingPrimaryKeysAndFailedRollback
        { non_existing_primary_keys, rollback, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NonExistingPrimaryKeysAndFailedRollback
        { non_existing_primary_keys, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: CommitFailed
        { commit_failed, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CommitFailed { commit_failed, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: NotUniquePrimaryKey
        { not_unique_primary_key, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryUpdateManyRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }
    };
    Err(
        TryUpdateManyErrorNamed::TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize {
            try_update_many_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 6943,
                    column: 13,
                }),
            ),
        },
    )
}
